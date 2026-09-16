//! Optional tracing-based profiler for the executable WHIR prover.

use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fs,
    path::Path,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

use anyhow::{ensure, Context, Result};
use serde::Serialize;
use tracing::{
    field::{Field, Visit},
    metadata::LevelFilter,
    span::{Attributes, Id, Record},
    Event, Metadata, Subscriber,
};

use crate::{
    executable_whir::{config, execute_with_regime_and_folding},
    whir::WhirDecodingRegime,
    Fr,
};
use whir_ref::{
    algebra::embedding::Identity,
    buffer::{Buffer, BufferMath, BufferOps},
    protocols::irs_commit::Evaluations,
};

#[derive(Clone)]
struct TimingSubscriber {
    inner: Arc<TimingSubscriberInner>,
}

struct TimingSubscriberInner {
    next_id: AtomicU64,
    epoch: Mutex<Instant>,
    active: Mutex<HashMap<u64, ActiveSpan>>,
    completed: Mutex<Vec<CompletedSpan>>,
}

struct ActiveSpan {
    references: usize,
    name: &'static str,
    target: &'static str,
    fields: String,
    first_entered_at: Option<Instant>,
    entered_at: Vec<Instant>,
    elapsed: Duration,
}

#[derive(Clone)]
struct CompletedSpan {
    name: &'static str,
    target: &'static str,
    fields: String,
    start: Duration,
    elapsed: Duration,
}

#[derive(Default)]
struct FieldVisitor(Vec<String>);

impl Visit for FieldVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.0.push(format!("{}={value:?}", field.name()));
    }
}

impl TimingSubscriber {
    fn new() -> Self {
        Self {
            inner: Arc::new(TimingSubscriberInner {
                next_id: AtomicU64::new(1),
                epoch: Mutex::new(Instant::now()),
                active: Mutex::new(HashMap::new()),
                completed: Mutex::new(Vec::new()),
            }),
        }
    }

    fn begin_run(&self) {
        assert!(self.inner.active.lock().unwrap().is_empty());
        self.inner.completed.lock().unwrap().clear();
        *self.inner.epoch.lock().unwrap() = Instant::now();
    }

    fn take_completed(&self) -> Vec<CompletedSpan> {
        let mut completed = std::mem::take(&mut *self.inner.completed.lock().unwrap());
        completed.sort_by_key(|span| span.start);
        completed
    }
}

impl Subscriber for TimingSubscriber {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.target().starts_with("whir")
    }

    fn max_level_hint(&self) -> Option<LevelFilter> {
        Some(LevelFilter::TRACE)
    }

    fn new_span(&self, attributes: &Attributes<'_>) -> Id {
        let id = self.inner.next_id.fetch_add(1, Ordering::Relaxed);
        let mut visitor = FieldVisitor::default();
        attributes.record(&mut visitor);
        let metadata = attributes.metadata();
        self.inner.active.lock().unwrap().insert(
            id,
            ActiveSpan {
                references: 1,
                name: metadata.name(),
                target: metadata.target(),
                fields: visitor.0.join(";"),
                first_entered_at: None,
                entered_at: Vec::new(),
                elapsed: Duration::ZERO,
            },
        );
        Id::from_u64(id)
    }

    fn record(&self, id: &Id, values: &Record<'_>) {
        let mut visitor = FieldVisitor::default();
        values.record(&mut visitor);
        if let Some(span) = self.inner.active.lock().unwrap().get_mut(&id.into_u64()) {
            if !span.fields.is_empty() && !visitor.0.is_empty() {
                span.fields.push(';');
            }
            span.fields.push_str(&visitor.0.join(";"));
        }
    }

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {}

    fn event(&self, _event: &Event<'_>) {}

    fn enter(&self, id: &Id) {
        let now = Instant::now();
        if let Some(span) = self.inner.active.lock().unwrap().get_mut(&id.into_u64()) {
            span.first_entered_at.get_or_insert(now);
            span.entered_at.push(now);
        }
    }

    fn exit(&self, id: &Id) {
        let now = Instant::now();
        if let Some(span) = self.inner.active.lock().unwrap().get_mut(&id.into_u64()) {
            if let Some(entered_at) = span.entered_at.pop() {
                span.elapsed += now.saturating_duration_since(entered_at);
            }
        }
    }

    fn clone_span(&self, id: &Id) -> Id {
        if let Some(span) = self.inner.active.lock().unwrap().get_mut(&id.into_u64()) {
            span.references += 1;
        }
        id.clone()
    }

    fn try_close(&self, id: Id) -> bool {
        let mut active = self.inner.active.lock().unwrap();
        let Some(span) = active.get_mut(&id.into_u64()) else {
            return true;
        };
        span.references -= 1;
        if span.references != 0 {
            return false;
        }
        let span = active.remove(&id.into_u64()).unwrap();
        drop(active);
        let epoch = *self.inner.epoch.lock().unwrap();
        self.inner.completed.lock().unwrap().push(CompletedSpan {
            name: span.name,
            target: span.target,
            fields: span.fields,
            start: span.first_entered_at.map_or(Duration::ZERO, |start| {
                start.saturating_duration_since(epoch)
            }),
            elapsed: span.elapsed,
        });
        true
    }
}

#[derive(Serialize)]
struct TraceRow {
    starting_rate: String,
    repetition: usize,
    ordinal: usize,
    occurrence: usize,
    span: String,
    target: String,
    fields: String,
    start_microseconds: f64,
    duration_microseconds: f64,
}

#[derive(Serialize)]
struct TraceRunRow {
    starting_rate: String,
    repetition: usize,
    log_message_length: usize,
    log_codeword_length: usize,
    commitment_seconds: f64,
    ood_seconds: f64,
    claim_evaluation_seconds: f64,
    protocol_proving_seconds: f64,
    opening_seconds: f64,
    verification_seconds: f64,
    proof_bytes: usize,
}

#[derive(Serialize)]
struct TraceGeometryRow {
    starting_rate: String,
    oracle: usize,
    message_length: usize,
    codeword_length: usize,
    leaf_width: usize,
    leaf_count: usize,
    requested_queries: usize,
}

#[derive(Serialize)]
struct FirstTransitionTraceRow {
    starting_rate: String,
    repetition: usize,
    phase: &'static str,
    duration_microseconds: f64,
}

#[derive(Serialize)]
struct ConstraintMicroprofileRow {
    starting_rate: String,
    repetition: usize,
    current_vector_length: usize,
    in_domain_claims: usize,
    out_of_domain_claims: usize,
    phase: &'static str,
    duration_microseconds: f64,
}

fn write_constraint_microprofile(
    output_dir: &Path,
    log_length: usize,
    folding_log: usize,
    warmups: usize,
    repetitions: usize,
) -> Result<()> {
    let mut writer = csv::Writer::from_path(output_dir.join("whir_constraint_microprofile.csv"))?;
    for log_inverse_rate in [1usize, 2] {
        let config = config(
            1usize << log_length,
            log_inverse_rate,
            WhirDecodingRegime::Johnson,
            folding_log,
        )?;
        let round = &config.round_configs[0];
        let current_vector_length = round.initial_size();
        let in_domain_claims = config.initial_committer.in_domain_samples();
        let out_of_domain_claims = round.out_domain_samples;
        let leaf_width = config.initial_committer.interleaving_depth();
        let out_points = (0..out_of_domain_claims)
            .map(|index| crate::benchmark_field_element(0x43414e4f5059, 2, index as u64))
            .collect::<Vec<_>>();
        let in_points = (0..in_domain_claims)
            .map(|index| crate::benchmark_field_element(0x43414e4f5059, 101, index as u64))
            .collect::<Vec<_>>();
        let out_of_domain = Evaluations {
            points: out_points.clone(),
            matrix: Buffer::from(
                (0..out_of_domain_claims)
                    .map(|index| crate::benchmark_field_element(0x43414e4f5059, 211, index as u64))
                    .collect::<Vec<_>>(),
            ),
            domain: None,
        };
        let in_domain = Evaluations {
            points: in_points.clone(),
            matrix: Buffer::from(
                (0..in_domain_claims * leaf_width)
                    .map(|index| crate::benchmark_field_element(0x43414e4f5059, 307, index as u64))
                    .collect::<Vec<_>>(),
            ),
            domain: None,
        };
        let folding_randomness = (0..folding_log)
            .map(|index| crate::benchmark_field_element(0x43414e4f5059, 401, index as u64))
            .collect::<Vec<_>>();
        let vector_rlc_coeffs = Buffer::<Fr>::ones(1);
        let eq_weights = Buffer::<Fr>::eq_weights(&folding_randomness);
        let stir_weights = vector_rlc_coeffs.tensor_product(&eq_weights);
        let out_weights = Buffer::<Fr>::ones(1);
        let stir_points = out_points
            .iter()
            .chain(&in_points)
            .copied()
            .collect::<Vec<_>>();
        let stir_evaluations = out_of_domain
            .values_buffer(&Identity::new(), &out_weights)
            .concat(&in_domain.values_buffer(&Identity::new(), &stir_weights));
        let stir_rlc_coeffs = Buffer::<Fr>::geometric_challenge(
            Fr::from(1u64),
            crate::benchmark_field_element(0x43414e4f5059, 503, 0),
            stir_points.len(),
        );
        let starting_rate = format!("1/{}", 1usize << log_inverse_rate);

        for iteration in 0..warmups + repetitions {
            let mut measurements = Vec::new();

            let start = Instant::now();
            let points = out_points
                .iter()
                .chain(&in_points)
                .copied()
                .collect::<Vec<_>>();
            std::hint::black_box(&points);
            measurements.push(("collect_evaluation_points", start.elapsed()));

            let start = Instant::now();
            let eq = Buffer::<Fr>::eq_weights(&folding_randomness);
            let weights = vector_rlc_coeffs.tensor_product(&eq);
            std::hint::black_box(&weights);
            measurements.push(("construct_fold_weights", start.elapsed()));

            let start = Instant::now();
            let evaluations = out_of_domain
                .values_buffer(&Identity::new(), &out_weights)
                .concat(&in_domain.values_buffer(&Identity::new(), &stir_weights));
            std::hint::black_box(&evaluations);
            measurements.push(("pack_weighted_evaluations", start.elapsed()));

            let start = Instant::now();
            let challenges = Buffer::<Fr>::geometric_challenge(
                Fr::from(1u64),
                crate::benchmark_field_element(0x43414e4f5059, 503, 0),
                stir_points.len(),
            );
            std::hint::black_box(&challenges);
            measurements.push(("generate_batching_challenge_sequence", start.elapsed()));

            let start = Instant::now();
            let mut covector = Buffer::<Fr>::zeros(current_vector_length);
            std::hint::black_box(&covector);
            measurements.push(("allocate_zero_covector", start.elapsed()));

            let start = Instant::now();
            covector.accumulate_geometric(&stir_points, &stir_rlc_coeffs, current_vector_length);
            std::hint::black_box(&covector);
            measurements.push(("accumulate_geometric_covector", start.elapsed()));

            let start = Instant::now();
            let claimed_sum = stir_rlc_coeffs.dot(&stir_evaluations);
            std::hint::black_box(claimed_sum);
            measurements.push(("dot_combined_claimed_value", start.elapsed()));

            if iteration < warmups {
                continue;
            }
            for (phase, duration) in measurements {
                writer.serialize(ConstraintMicroprofileRow {
                    starting_rate: starting_rate.clone(),
                    repetition: iteration - warmups,
                    current_vector_length,
                    in_domain_claims,
                    out_of_domain_claims,
                    phase,
                    duration_microseconds: duration.as_secs_f64() * 1e6,
                })?;
            }
        }
    }
    writer.flush().context("flushing constraint microprofile")?;
    Ok(())
}

/// Record every tracing span emitted by the official WHIR prover for isolated
/// rate-1/2 and rate-1/4, one-oracle Johnson executions.
pub fn run_whir_trace_profile(
    output_dir: &Path,
    log_length: usize,
    folding_log: usize,
    warmups: usize,
    repetitions: usize,
) -> Result<()> {
    ensure!(repetitions > 0, "trace repetitions must be positive");
    fs::create_dir_all(output_dir)?;
    let subscriber = TimingSubscriber::new();
    tracing::subscriber::set_global_default(subscriber.clone())
        .map_err(|error| anyhow::anyhow!("installing WHIR timing subscriber: {error}"))?;

    let values = (0..1u64 << log_length)
        .map(|i| crate::benchmark_field_element(0x43414e4f5059, 0, i))
        .collect::<Vec<_>>();
    let mut trace_writer = csv::Writer::from_path(output_dir.join("whir_trace_spans.csv"))?;
    let mut run_writer = csv::Writer::from_path(output_dir.join("whir_trace_runs.csv"))?;
    let mut geometry_writer = csv::Writer::from_path(output_dir.join("whir_trace_geometry.csv"))?;
    let mut first_transition_writer =
        csv::Writer::from_path(output_dir.join("whir_trace_first_transition.csv"))?;
    let mut wrote_geometry = BTreeSet::new();

    for iteration in 0..warmups + repetitions {
        let mut rates = [1usize, 2usize];
        if iteration.is_multiple_of(2) {
            rates.reverse();
        }
        for log_inverse_rate in rates {
            subscriber.begin_run();
            let run = execute_with_regime_and_folding(
                std::slice::from_ref(&values),
                log_inverse_rate,
                WhirDecodingRegime::Johnson,
                folding_log,
            )?;
            let spans = subscriber.take_completed();
            if iteration < warmups {
                continue;
            }
            let repetition = iteration - warmups;
            let starting_rate = format!("1/{}", 1usize << log_inverse_rate);
            let mut occurrences = BTreeMap::<(&str, &str), usize>::new();
            for (ordinal, span) in spans.iter().enumerate() {
                let occurrence = occurrences
                    .entry((span.target, span.name))
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                trace_writer.serialize(TraceRow {
                    starting_rate: starting_rate.clone(),
                    repetition,
                    ordinal,
                    occurrence: *occurrence,
                    span: span.name.to_owned(),
                    target: span.target.to_owned(),
                    fields: span.fields.clone(),
                    start_microseconds: span.start.as_secs_f64() * 1e6,
                    duration_microseconds: span.elapsed.as_secs_f64() * 1e6,
                })?;
            }
            let whir_prove = spans
                .iter()
                .find(|span| span.target == "whir::protocols::whir::prover" && span.name == "prove")
                .context("missing WHIR prove span")?;
            let sumchecks = spans
                .iter()
                .filter(|span| span.target == "whir::protocols::sumcheck" && span.name == "prove")
                .collect::<Vec<_>>();
            let recursive_commits = spans
                .iter()
                .filter(|span| {
                    span.target == "whir::protocols::irs_commit" && span.name == "commit_with_ood"
                })
                .collect::<Vec<_>>();
            let oracle_openings = spans
                .iter()
                .filter(|span| span.target == "whir::protocols::irs_commit" && span.name == "open")
                .collect::<Vec<_>>();
            ensure!(
                sumchecks.len() >= 2
                    && !recursive_commits.is_empty()
                    && !oracle_openings.is_empty(),
                "WHIR trace did not contain a complete first transition"
            );
            let initial_sumcheck = sumchecks[0];
            let next_sumcheck = sumchecks[1];
            let second_oracle_commit = recursive_commits[0];
            let first_oracle_open = oracle_openings[0];
            let span_end = |span: &CompletedSpan| span.start + span.elapsed;
            for (phase, duration) in [
                ("claimed_mle_evaluation", run.claim_evaluation_time),
                (
                    "pre_initial_sumcheck_setup",
                    initial_sumcheck.start.saturating_sub(whir_prove.start),
                ),
                ("initial_sumcheck_and_fold", initial_sumcheck.elapsed),
                (
                    "post_sumcheck_transcript_gap",
                    second_oracle_commit
                        .start
                        .saturating_sub(span_end(initial_sumcheck)),
                ),
                ("second_oracle_commit_and_ood", second_oracle_commit.elapsed),
                (
                    "pre_opening_gap",
                    first_oracle_open
                        .start
                        .saturating_sub(span_end(second_oracle_commit)),
                ),
                ("first_oracle_irs_merkle_open", first_oracle_open.elapsed),
                (
                    "stir_constraint_covector_assembly",
                    next_sumcheck
                        .start
                        .saturating_sub(span_end(first_oracle_open)),
                ),
                ("next_sumcheck_and_fold", next_sumcheck.elapsed),
            ] {
                first_transition_writer.serialize(FirstTransitionTraceRow {
                    starting_rate: starting_rate.clone(),
                    repetition,
                    phase,
                    duration_microseconds: duration.as_secs_f64() * 1e6,
                })?;
            }
            run_writer.serialize(TraceRunRow {
                starting_rate: starting_rate.clone(),
                repetition,
                log_message_length: log_length,
                log_codeword_length: log_length + log_inverse_rate,
                commitment_seconds: run.commitment_time.as_secs_f64(),
                ood_seconds: run.ood_time.as_secs_f64(),
                claim_evaluation_seconds: run.claim_evaluation_time.as_secs_f64(),
                protocol_proving_seconds: run.protocol_proving_time.as_secs_f64(),
                opening_seconds: (run.ood_time + run.proving_time).as_secs_f64(),
                verification_seconds: run.verification_time.as_secs_f64(),
                proof_bytes: run.proof_bytes,
            })?;
            if wrote_geometry.insert(log_inverse_rate) {
                for geometry in &run.oracle_geometry {
                    geometry_writer.serialize(TraceGeometryRow {
                        starting_rate: starting_rate.clone(),
                        oracle: geometry.oracle,
                        message_length: geometry.message_length,
                        codeword_length: geometry.codeword_length,
                        leaf_width: geometry.leaf_width,
                        leaf_count: geometry.leaf_count,
                        requested_queries: geometry.requested_queries,
                    })?;
                }
            }
        }
    }
    trace_writer.flush().context("flushing WHIR trace spans")?;
    run_writer.flush().context("flushing WHIR trace runs")?;
    geometry_writer
        .flush()
        .context("flushing WHIR trace geometry")?;
    first_transition_writer
        .flush()
        .context("flushing WHIR first-transition trace")?;
    write_constraint_microprofile(output_dir, log_length, folding_log, warmups, repetitions)?;
    Ok(())
}
