//! Final paper experiments and report generation.
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Write,
    path::Path,
    time::{Duration, Instant},
};

use anyhow::{ensure, Context, Result};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[path = "paper_fiat_shamir.rs"]
pub mod fiat_shamir;

use crate::{
    canopy::CanopyVectorCommitment,
    executable_whir::{
        commit_matrix_with_whir, encode_interleaved_with_whir, encode_many_interleaved_with_whir,
        execute_benchmark_with_security, ExecutableWhirRun,
    },
    merkle::frontier_hash_count,
    whir::{
        estimate_proof, estimate_schwartz_zippel_batched_initial_proofs, expand_seed,
        sample_initial_oracle_queries, WhirDecodingRegime, WhirParameters, WhirProofSize,
        DEFAULT_SECURITY_BITS,
    },
    Fr, FIELD_BITS, FIELD_BYTES, HASH_BYTES,
};

pub const PAPER_LOG_LENGTHS: &[usize] = &[18, 20, 22, 24, 26, 28, 30];
pub const PAPER_INPUT_ORACLE_COUNTS: &[usize] = &[1, 2, 4, 8];
pub const CANOPY_HYBRID_CODEWORD_REDUCTION: usize = 0;
pub const PAPER_FOLDING_CHOICES: &[usize] = &[3, 4];
pub const PAPER_TERMINAL_MESSAGE_VARIABLES: usize = 10;
/// The selected initial fold at the executable `2^18` timing point.
pub const PAPER_TIMING_FOLDING_LOG: usize = 3;
pub const CANOPY_HYBRID_HEIGHT: usize = PAPER_TIMING_FOLDING_LOG + 1;
pub const OUTER_STARTING_LOG_INV_RATE: usize = 1;
pub const QUARTER_RATE_STARTING_LOG_INV_RATE: usize = 2;

/// Eight bits of local margin for the union of queries and algebraic events.
pub const PAPER_LOCAL_SECURITY_BITS: usize = 136;

fn paper_parameters(
    n: usize,
    rate: usize,
    regime: WhirDecodingRegime,
    terminal: usize,
    schedule: &[usize],
    reduction: usize,
) -> Result<WhirParameters> {
    WhirParameters::with_regime_and_terminal_and_schedule(
        n, rate, regime, terminal, schedule, reduction,
    )?
    .with_algebraic_budget(PAPER_LOCAL_SECURITY_BITS)
}

fn execute_whir(
    vectors: &[Vec<Fr>],
    rate: usize,
    regime: WhirDecodingRegime,
    folding: usize,
) -> Result<ExecutableWhirRun> {
    execute_benchmark_with_security(vectors, rate, regime, folding, PAPER_LOCAL_SECURITY_BITS)
}

fn aligned_source_oracles(input_oracles: usize) -> usize {
    2 * input_oracles
}

fn aligned_canopy_height(initial_folding_log: usize) -> usize {
    initial_folding_log + 1
}

fn folding_schedules_up_to(message_variables: usize) -> Vec<Vec<usize>> {
    fn visit(remaining: usize, prefix: &mut Vec<usize>, schedules: &mut Vec<Vec<usize>>) {
        if remaining <= PAPER_TERMINAL_MESSAGE_VARIABLES {
            schedules.push(prefix.clone());
            return;
        }
        for &fold in PAPER_FOLDING_CHOICES {
            if fold <= remaining {
                prefix.push(fold);
                visit(remaining - fold, prefix, schedules);
                prefix.pop();
            }
        }
    }

    let mut schedules = Vec::new();
    visit(message_variables, &mut Vec::new(), &mut schedules);
    schedules
}

fn terminal_message_variables(message_variables: usize, schedule: &[usize]) -> usize {
    message_variables - schedule.iter().copied().sum::<usize>()
}

fn schedule_label(schedule: &[usize]) -> String {
    if schedule.is_empty() {
        "terminal".to_owned()
    } else {
        schedule
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(";")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HybridProofSizeRow {
    pub experiment: usize,
    pub scheme: String,
    pub input_oracles: usize,
    pub batched_source_oracles: usize,
    pub log_message_length: usize,
    pub message_length: u64,
    pub outer_starting_rate: String,
    pub outer_initial_folding_log: usize,
    pub outer_folding_schedule: String,
    pub outer_terminal_message_variables: usize,
    pub canopy_height: Option<usize>,
    pub canopy_folding_schedule: Option<String>,
    pub canopy_terminal_message_variables: Option<usize>,
    pub canopy_codeword_log_reduction: Option<usize>,
    pub aligned_message_log: Option<usize>,
    pub aligned_codeword_log: Option<usize>,
    pub aligned_log_inverse_rate: Option<usize>,
    pub samples: usize,
    pub recursive_field_bits: usize,
    pub local_algebraic_budget_bits: usize,
    pub minimum_rbr_security_bits: f64,
    pub algebraic_rbr_union_bound_bits: f64,
    pub meets_128_bit_rbr: bool,
    pub verifier_leaf_hashes: f64,
    pub verifier_parent_hashes: f64,
    pub verifier_auth_hashes: f64,
    pub verifier_hash_to_field: f64,
    pub mean_root_hashes: f64,
    pub mean_auth_hashes: f64,
    pub mean_hash_count: f64,
    pub mean_hash_bytes: f64,
    pub mean_ood_fields: f64,
    pub mean_ood_bytes: f64,
    pub mean_sumcheck_fields: f64,
    pub mean_sumcheck_bytes: f64,
    pub mean_batching_fields: f64,
    pub mean_batching_bytes: f64,
    pub mean_query_answer_fields: f64,
    pub mean_query_answer_bytes: f64,
    pub mean_final_coefficient_fields: f64,
    pub mean_final_coefficient_bytes: f64,
    pub mean_field_count: f64,
    pub mean_field_bytes: f64,
    pub mean_total_bytes: f64,
    pub mean_total_kib: f64,
    pub relative_to_merkle_whir: f64,
    pub relative_to_batched_merkle_whir: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct OracleQueryBreakdownRow {
    pub experiment: usize,
    pub scheme: String,
    pub input_oracles: usize,
    pub log_message_length: usize,
    pub oracle_role: String,
    pub commitment_kind: String,
    pub oracle_count: usize,
    pub shared_query_positions: bool,
    pub shared_authentication: bool,
    pub message_log: usize,
    pub codeword_log: usize,
    pub log_inverse_rate: usize,
    pub field_bits: usize,
    pub leaf_width: usize,
    pub leaf_count: u64,
    pub requested_queries: usize,
    pub samples: usize,
    pub mean_unique_queries: f64,
    pub verifier_leaf_hashes: f64,
    pub verifier_parent_hashes: f64,
    pub verifier_auth_hashes: f64,
    pub verifier_hash_to_field: f64,
    pub mean_auth_hashes: f64,
    pub mean_answer_fields: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct RbrSecurityRow {
    pub experiment: usize,
    pub scheme: String,
    pub input_oracles: usize,
    pub log_message_length: usize,
    pub phase: String,
    pub event: String,
    pub security_bits: f64,
}

#[derive(Clone, Default)]
struct Totals {
    hashes: usize,
    fields: usize,
    field_bytes: usize,
    root_hashes: usize,
    auth_hashes: usize,
    ood_fields: usize,
    ood_bytes: usize,
    sumcheck_fields: usize,
    sumcheck_bytes: usize,
    batching_fields: usize,
    batching_bytes: usize,
    query_answer_fields: usize,
    query_answer_bytes: usize,
    final_coefficient_fields: usize,
    final_coefficient_bytes: usize,
}

impl Totals {
    fn add_whir_with_existing_root(&mut self, proof: &WhirProofSize) {
        let field_bytes = proof.field_bytes_per_element;
        self.hashes += proof.hash_count;
        self.fields += proof.field_count;
        self.field_bytes += proof.field_count * field_bytes;
        self.root_hashes += proof.intermediate_roots;
        self.auth_hashes += proof.merkle_auth_hashes;
        self.ood_fields += proof.ood_fields;
        self.ood_bytes += proof.ood_fields * field_bytes;
        self.sumcheck_fields += proof.sumcheck_fields;
        self.sumcheck_bytes += proof.sumcheck_fields * field_bytes;
        self.batching_fields += proof.batching_fields;
        self.batching_bytes += proof.batching_fields * field_bytes;
        self.query_answer_fields += proof.query_answer_fields;
        self.query_answer_bytes += proof.query_answer_fields * field_bytes;
        self.final_coefficient_fields += proof.final_coefficient_fields;
        self.final_coefficient_bytes += proof.final_coefficient_fields * field_bytes;
    }

    fn add_whir_with_initial_root(&mut self, proof: &WhirProofSize) {
        self.hashes += 1;
        self.root_hashes += 1;
        self.add_whir_with_existing_root(proof);
    }

    fn add_hybrid_front(
        &mut self,
        outer: &WhirParameters,
        bottom_hashes: usize,
        first_fields: usize,
    ) {
        // The Canopy top root is also the Canopy-claim WHIR root. The other
        // root commits outer WHIR's aligned second oracle.
        self.hashes += 2 + bottom_hashes;
        self.root_hashes += 2;
        self.auth_hashes += bottom_hashes;
        self.fields += initial_stage_fields(outer) + first_fields;
        self.field_bytes += (initial_stage_fields(outer) + first_fields) * FIELD_BYTES;
        self.ood_fields += outer.commitment_ood_samples;
        self.ood_bytes += outer.commitment_ood_samples * FIELD_BYTES;
        self.sumcheck_fields += 3 * outer.folding_log * outer.initial_sumcheck_repetitions;
        self.sumcheck_bytes +=
            3 * outer.folding_log * outer.initial_sumcheck_repetitions * FIELD_BYTES;
        self.query_answer_fields += first_fields;
        self.query_answer_bytes += first_fields * FIELD_BYTES;
    }

    fn add_merkle_front(
        &mut self,
        outer: &WhirParameters,
        first_auth_hashes: usize,
        first_fields: usize,
    ) {
        // One root for the original oracle and one for its aligned second
        // oracle. The latter is consumed by the separately parameterized tail.
        self.hashes += 2 + first_auth_hashes;
        self.root_hashes += 2;
        self.auth_hashes += first_auth_hashes;
        self.fields += initial_stage_fields(outer) + first_fields;
        self.field_bytes += (initial_stage_fields(outer) + first_fields) * FIELD_BYTES;
        self.ood_fields += outer.commitment_ood_samples;
        self.ood_bytes += outer.commitment_ood_samples * FIELD_BYTES;
        self.sumcheck_fields += 3 * outer.folding_log * outer.initial_sumcheck_repetitions;
        self.sumcheck_bytes +=
            3 * outer.folding_log * outer.initial_sumcheck_repetitions * FIELD_BYTES;
        self.query_answer_fields += first_fields;
        self.query_answer_bytes += first_fields * FIELD_BYTES;
    }

    fn bytes(&self) -> usize {
        self.hashes * HASH_BYTES + self.field_bytes
    }
}

#[derive(Clone)]
struct QueryDescriptor {
    experiment: usize,
    scheme: &'static str,
    input_oracles: usize,
    log_message_length: usize,
    role: String,
    commitment: &'static str,
    oracle_count: usize,
    shared_positions: bool,
    shared_authentication: bool,
    message_log: usize,
    codeword_log: usize,
    log_inverse_rate: usize,
    field_bits: usize,
    leaf_width: usize,
    leaf_count: usize,
    requested_queries: usize,
}

#[derive(Clone, Default)]
struct QueryAccumulator {
    descriptor: Option<QueryDescriptor>,
    unique_queries: usize,
    auth_hashes: usize,
    answer_fields: usize,
    samples: usize,
}

#[derive(Clone)]
struct ProofCandidate {
    input_oracles: usize,
    batched_source_oracles: usize,
    starting_log_inverse_rate: usize,
    initial_folding_log: usize,
    canopy_height: Option<usize>,
    outer_folding_schedule: Vec<usize>,
    canopy_folding_schedule: Option<Vec<usize>>,
    recursive_field_bits: usize,
    minimum_rbr_security_bits: f64,
    algebraic_rbr_union_bound_bits: f64,
    rbr_events: Vec<TaggedRbrEvent>,
    totals: Totals,
    queries: BTreeMap<String, QueryAccumulator>,
}

#[derive(Clone)]
struct TaggedRbrEvent {
    phase: String,
    event: String,
    security_bits: f64,
}

fn tagged_rbr_events(
    phase: &str,
    params: &WhirParameters,
    initial_only: bool,
) -> Vec<TaggedRbrEvent> {
    let events = if initial_only {
        params.initial_rbr_security_events()
    } else {
        params.rbr_security_events()
    };
    events
        .into_iter()
        .map(|event| TaggedRbrEvent {
            phase: phase.to_owned(),
            event: event.event,
            security_bits: event.security_bits,
        })
        .collect()
}

fn minimum_event_security(events: &[TaggedRbrEvent]) -> f64 {
    events
        .iter()
        .map(|event| event.security_bits)
        .reduce(f64::min)
        .unwrap_or(f64::INFINITY)
}

fn union_bound_security(events: &[TaggedRbrEvent]) -> f64 {
    let minimum = minimum_event_security(events);
    minimum
        - events
            .iter()
            .map(|event| 2.0_f64.powf(minimum - event.security_bits))
            .sum::<f64>()
            .log2()
}

fn best_rbr_candidate(candidates: &[ProofCandidate]) -> &ProofCandidate {
    if let Some(candidate) = candidates
        .iter()
        .filter(|candidate| {
            candidate.minimum_rbr_security_bits >= DEFAULT_SECURITY_BITS as f64
                && candidate.algebraic_rbr_union_bound_bits >= DEFAULT_SECURITY_BITS as f64
        })
        .min_by_key(|candidate| {
            (
                candidate.totals.bytes(),
                candidate.initial_folding_log,
                candidate.canopy_height,
                &candidate.outer_folding_schedule,
                &candidate.canopy_folding_schedule,
            )
        })
    {
        return candidate;
    }

    candidates
        .iter()
        .max_by(|a, b| {
            a.minimum_rbr_security_bits
                .total_cmp(&b.minimum_rbr_security_bits)
                .then_with(|| b.totals.bytes().cmp(&a.totals.bytes()))
        })
        .expect("nonempty folding grid")
}

impl QueryAccumulator {
    fn push(
        &mut self,
        descriptor: QueryDescriptor,
        unique_queries: usize,
        auth_hashes: usize,
        answer_fields: usize,
    ) {
        if let Some(existing) = &self.descriptor {
            debug_assert_eq!(existing.role, descriptor.role);
        } else {
            self.descriptor = Some(descriptor);
        }
        self.unique_queries += unique_queries;
        self.auth_hashes += auth_hashes;
        self.answer_fields += answer_fields;
        self.samples += 1;
    }

    fn verifier_work(&self) -> (f64, f64, f64) {
        let d = self.descriptor.as_ref().expect("query descriptor");
        let leaves = (self.unique_queries * d.oracle_count) as f64;
        if d.commitment == "independent Canopy bottom forests" {
            // Exp3 aligns h=initial_fold+1, hence one level above row leaves.
            // reconstruct_roots hashes once per child, including paired siblings.
            (leaves, leaves, (leaves + self.auth_hashes as f64) / 2.0)
        } else {
            let roots = (d.oracle_count * self.samples) as f64;
            (leaves, leaves + self.auth_hashes as f64 - roots, 0.0)
        }
    }

    fn row(self) -> OracleQueryBreakdownRow {
        let (leaf, parent, h2f) = self.verifier_work();
        let descriptor = self.descriptor.expect("query descriptor");
        let samples = self.samples as f64;
        OracleQueryBreakdownRow {
            experiment: descriptor.experiment,
            scheme: descriptor.scheme.to_owned(),
            input_oracles: descriptor.input_oracles,
            log_message_length: descriptor.log_message_length,
            oracle_role: descriptor.role,
            commitment_kind: descriptor.commitment.to_owned(),
            oracle_count: descriptor.oracle_count,
            shared_query_positions: descriptor.shared_positions,
            shared_authentication: descriptor.shared_authentication,
            message_log: descriptor.message_log,
            codeword_log: descriptor.codeword_log,
            log_inverse_rate: descriptor.log_inverse_rate,
            field_bits: descriptor.field_bits,
            leaf_width: descriptor.leaf_width,
            leaf_count: descriptor.leaf_count as u64,
            requested_queries: descriptor.requested_queries,
            samples: self.samples,
            mean_unique_queries: self.unique_queries as f64 / samples,
            verifier_leaf_hashes: leaf / samples,
            verifier_parent_hashes: parent / samples,
            verifier_auth_hashes: (leaf + parent) / samples,
            verifier_hash_to_field: h2f / samples,
            mean_auth_hashes: self.auth_hashes as f64 / samples,
            mean_answer_fields: self.answer_fields as f64 / samples,
        }
    }
}

fn derive_u64(seed: u64, words: &[usize]) -> u64 {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"canopy-system/paper-experiment-seed/v1");
    hasher.update(&seed.to_le_bytes());
    for &word in words {
        hasher.update(&(word as u64).to_le_bytes());
    }
    u64::from_le_bytes(hasher.finalize().as_bytes()[..8].try_into().unwrap())
}

fn write_csv<T: Serialize>(path: impl AsRef<Path>, rows: &[T]) -> Result<()> {
    let path = path.as_ref();
    let mut writer =
        csv::Writer::from_path(path).with_context(|| format!("creating {}", path.display()))?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

fn initial_stage_fields(params: &WhirParameters) -> usize {
    params.commitment_ood_samples + 3 * params.folding_log * params.initial_sumcheck_repetitions
}

fn query_geometry(params: &WhirParameters) -> Vec<(usize, usize, usize, usize, usize, usize)> {
    let mut result = Vec::new();
    let mut message_variables = params.message_variables;
    let mut log_inverse_rate = params.log_inverse_rate;
    for round in &params.rounds {
        result.push((
            message_variables,
            message_variables + log_inverse_rate,
            log_inverse_rate,
            round.query_leaf_width,
            round.oracle_leaf_count,
            round.requested_queries,
        ));
        message_variables = round.message_variables;
        log_inverse_rate = round.log_inverse_rate_after;
    }
    result.push((
        message_variables,
        message_variables + log_inverse_rate,
        log_inverse_rate,
        params.final_oracle_leaf_width,
        params.final_oracle_leaf_count,
        params.final_requested_queries,
    ));
    result
}

#[allow(clippy::too_many_arguments)]
fn accumulate_whir_queries(
    accumulators: &mut BTreeMap<String, QueryAccumulator>,
    experiment: usize,
    scheme: &'static str,
    input_oracles: usize,
    log_length: usize,
    role_prefix: &str,
    commitment: &'static str,
    params: &WhirParameters,
    proof: &WhirProofSize,
    initial_oracle_count: usize,
    recursive_oracle_count: usize,
    initial_auth_multiplier: usize,
    recursive_auth_multiplier: usize,
    shared_query_positions: bool,
    separate_initial_roots: bool,
) {
    for (oracle, geometry) in query_geometry(params).into_iter().enumerate() {
        let (message_log, codeword_log, rate, width, leaves, requested) = geometry;
        let role = if oracle == 0 {
            format!("{role_prefix} initial oracle")
        } else {
            format!("{role_prefix} recursive oracle {oracle}")
        };
        let oracle_count = if oracle == 0 {
            initial_oracle_count
        } else {
            recursive_oracle_count
        };
        let key = format!("{experiment}/{scheme}/{input_oracles}/{log_length}/{role}");
        accumulators.entry(key).or_default().push(
            QueryDescriptor {
                experiment,
                scheme,
                input_oracles,
                log_message_length: log_length,
                role,
                commitment,
                oracle_count,
                shared_positions: oracle_count > 1 && shared_query_positions,
                shared_authentication: oracle_count > 1
                    && shared_query_positions
                    && !(separate_initial_roots && oracle == 0),
                message_log,
                codeword_log,
                log_inverse_rate: rate,
                field_bits: params.field_bits,
                leaf_width: width,
                leaf_count: leaves,
                requested_queries: requested,
            },
            proof.unique_queries_by_oracle[oracle],
            proof.auth_hashes_by_oracle[oracle]
                * if oracle == 0 {
                    initial_auth_multiplier
                } else {
                    recursive_auth_multiplier
                },
            proof.unique_queries_by_oracle[oracle] * width * oracle_count,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn proof_row(
    experiment: usize,
    scheme: &str,
    input_oracles: usize,
    batched_source_oracles: usize,
    log_length: usize,
    samples: usize,
    totals: &Totals,
    independent_baseline_bytes: f64,
    batched_baseline_bytes: f64,
    starting_log_inverse_rate: usize,
    initial_folding_log: usize,
    canopy_height: Option<usize>,
    outer_folding_schedule: &[usize],
    canopy_folding_schedule: Option<&[usize]>,
    recursive_field_bits: usize,
    minimum_rbr_security_bits: f64,
    algebraic_rbr_union_bound_bits: f64,
) -> HybridProofSizeRow {
    debug_assert_eq!(totals.hashes, totals.root_hashes + totals.auth_hashes);
    debug_assert_eq!(
        totals.fields,
        totals.ood_fields
            + totals.sumcheck_fields
            + totals.batching_fields
            + totals.query_answer_fields
            + totals.final_coefficient_fields
    );
    debug_assert_eq!(
        totals.field_bytes,
        totals.ood_bytes
            + totals.sumcheck_bytes
            + totals.batching_bytes
            + totals.query_answer_bytes
            + totals.final_coefficient_bytes
    );
    let mean_hashes = totals.hashes as f64 / samples as f64;
    let mean_fields = totals.fields as f64 / samples as f64;
    let mean_field_bytes = totals.field_bytes as f64 / samples as f64;
    let mean_bytes = mean_hashes * HASH_BYTES as f64 + mean_field_bytes;
    HybridProofSizeRow {
        experiment,
        scheme: scheme.to_owned(),
        input_oracles,
        batched_source_oracles,
        log_message_length: log_length,
        message_length: 1u64 << log_length,
        outer_starting_rate: format!("1/{}", 1usize << starting_log_inverse_rate),
        outer_initial_folding_log: initial_folding_log,
        outer_folding_schedule: schedule_label(outer_folding_schedule),
        outer_terminal_message_variables: terminal_message_variables(
            log_length,
            outer_folding_schedule,
        ),
        canopy_height,
        canopy_folding_schedule: canopy_folding_schedule.map(schedule_label),
        canopy_terminal_message_variables: canopy_height.zip(canopy_folding_schedule).map(
            |(height, schedule)| terminal_message_variables(log_length + 1 - height, schedule),
        ),
        canopy_codeword_log_reduction: canopy_height.map(|_| CANOPY_HYBRID_CODEWORD_REDUCTION),
        aligned_message_log: canopy_height.map(|_| log_length - initial_folding_log),
        aligned_codeword_log: canopy_height
            .map(|_| log_length + 1 - CANOPY_HYBRID_CODEWORD_REDUCTION),
        aligned_log_inverse_rate: canopy_height
            .map(|_| initial_folding_log + 1 - CANOPY_HYBRID_CODEWORD_REDUCTION),
        samples,
        recursive_field_bits,
        local_algebraic_budget_bits: PAPER_LOCAL_SECURITY_BITS,
        minimum_rbr_security_bits,
        algebraic_rbr_union_bound_bits,
        meets_128_bit_rbr: minimum_rbr_security_bits >= DEFAULT_SECURITY_BITS as f64
            && algebraic_rbr_union_bound_bits >= DEFAULT_SECURITY_BITS as f64,
        verifier_leaf_hashes: 0.0,
        verifier_parent_hashes: 0.0,
        verifier_auth_hashes: 0.0,
        verifier_hash_to_field: 0.0,
        mean_root_hashes: totals.root_hashes as f64 / samples as f64,
        mean_auth_hashes: totals.auth_hashes as f64 / samples as f64,
        mean_hash_count: mean_hashes,
        mean_hash_bytes: mean_hashes * HASH_BYTES as f64,
        mean_ood_fields: totals.ood_fields as f64 / samples as f64,
        mean_ood_bytes: totals.ood_bytes as f64 / samples as f64,
        mean_sumcheck_fields: totals.sumcheck_fields as f64 / samples as f64,
        mean_sumcheck_bytes: totals.sumcheck_bytes as f64 / samples as f64,
        mean_batching_fields: totals.batching_fields as f64 / samples as f64,
        mean_batching_bytes: totals.batching_bytes as f64 / samples as f64,
        mean_query_answer_fields: totals.query_answer_fields as f64 / samples as f64,
        mean_query_answer_bytes: totals.query_answer_bytes as f64 / samples as f64,
        mean_final_coefficient_fields: totals.final_coefficient_fields as f64 / samples as f64,
        mean_final_coefficient_bytes: totals.final_coefficient_bytes as f64 / samples as f64,
        mean_field_count: mean_fields,
        mean_field_bytes,
        mean_total_bytes: mean_bytes,
        mean_total_kib: mean_bytes / 1024.0,
        relative_to_merkle_whir: mean_bytes / independent_baseline_bytes,
        relative_to_batched_merkle_whir: mean_bytes / batched_baseline_bytes,
    }
}

fn repeated_rbr_events(
    phase: &str,
    params: &WhirParameters,
    initial_only: bool,
    copies: usize,
) -> Vec<TaggedRbrEvent> {
    (0..copies)
        .flat_map(|copy| {
            tagged_rbr_events(&format!("{phase} input {}", copy + 1), params, initial_only)
        })
        .collect()
}

fn sz_batching_security_bits(params: &WhirParameters, source_claims: usize) -> f64 {
    debug_assert!(source_claims >= 2);
    params.field_log2_size
        - params.initial_list_size_bits()
        - (2.0 * (source_claims - 1) as f64).log2()
}

#[allow(clippy::too_many_arguments)]
fn accumulate_front_queries(
    accumulators: &mut BTreeMap<String, QueryAccumulator>,
    experiment: usize,
    scheme: &'static str,
    input_oracles: usize,
    log_length: usize,
    role: &str,
    commitment: &'static str,
    geometry: (usize, usize, usize, usize, usize, usize),
    unique_queries: usize,
    auth_hashes: usize,
    answer_fields: usize,
) {
    let (message_log, codeword_log, rate, width, leaves, requested) = geometry;
    let key = format!("{experiment}/{scheme}/{input_oracles}/{log_length}/{role}");
    accumulators.entry(key).or_default().push(
        QueryDescriptor {
            experiment,
            scheme,
            input_oracles,
            log_message_length: log_length,
            role: role.to_owned(),
            commitment,
            oracle_count: input_oracles,
            shared_positions: false,
            shared_authentication: false,
            message_log,
            codeword_log,
            log_inverse_rate: rate,
            field_bits: FIELD_BITS,
            leaf_width: width,
            leaf_count: leaves,
            requested_queries: requested,
        },
        unique_queries,
        auth_hashes,
        answer_fields,
    );
}

#[allow(clippy::too_many_arguments)]
fn candidate_row(
    experiment: usize,
    scheme: &str,
    log_length: usize,
    samples: usize,
    candidate: &ProofCandidate,
    independent_baseline_bytes: f64,
    batched_baseline_bytes: f64,
) -> HybridProofSizeRow {
    let mut row = proof_row(
        experiment,
        scheme,
        candidate.input_oracles,
        candidate.batched_source_oracles,
        log_length,
        samples,
        &candidate.totals,
        independent_baseline_bytes,
        batched_baseline_bytes,
        candidate.starting_log_inverse_rate,
        candidate.initial_folding_log,
        candidate.canopy_height,
        &candidate.outer_folding_schedule,
        candidate.canopy_folding_schedule.as_deref(),
        candidate.recursive_field_bits,
        candidate.minimum_rbr_security_bits,
        candidate.algebraic_rbr_union_bound_bits,
    );
    for query in candidate.queries.values() {
        let (leaf, parent, h2f) = query.verifier_work();
        row.verifier_leaf_hashes += leaf / samples as f64;
        row.verifier_parent_hashes += parent / samples as f64;
        row.verifier_hash_to_field += h2f / samples as f64;
    }
    row.verifier_auth_hashes = row.verifier_leaf_hashes + row.verifier_parent_hashes;
    row
}

/// Generate the batched WHIR baselines and Canopy Exp3 proof-size/query grids.
pub fn run_proof_estimates(output_dir: &Path, samples: usize, seed: u64) -> Result<()> {
    run_proof_estimates_with_regime(output_dir, samples, seed, WhirDecodingRegime::Johnson)
}

pub fn run_proof_estimates_with_regime(
    output_dir: &Path,
    samples: usize,
    seed: u64,
    decoding_regime: WhirDecodingRegime,
) -> Result<()> {
    run_proof_estimates_with_regime_and_terminal(
        output_dir,
        samples,
        seed,
        decoding_regime,
        PAPER_TERMINAL_MESSAGE_VARIABLES,
    )
}

pub fn run_proof_estimates_with_regime_and_terminal(
    output_dir: &Path,
    samples: usize,
    seed: u64,
    decoding_regime: WhirDecodingRegime,
    max_terminal_message_variables: usize,
) -> Result<()> {
    ensure!(samples > 0, "samples must be positive");
    ensure!(
        max_terminal_message_variables == PAPER_TERMINAL_MESSAGE_VARIABLES,
        "paper schedule search requires terminal size at most 2^{PAPER_TERMINAL_MESSAGE_VARIABLES}"
    );
    ensure!(
        FIELD_BITS == 256,
        "paper experiments require the 256-bit field"
    );
    fs::create_dir_all(output_dir)?;

    let mut proof_rows = Vec::new();
    let mut folding_grid_rows = Vec::new();
    let mut rbr_security_rows = Vec::new();
    let mut query_accumulators = BTreeMap::<String, QueryAccumulator>::new();

    for &n in PAPER_LOG_LENGTHS {
        for &input_oracles in PAPER_INPUT_ORACLE_COUNTS {
            let mut independent_candidates = Vec::new();
            let mut batched_baseline_candidates = Vec::new();
            let mut quarter_rate_batched_baseline_candidates = Vec::new();
            let mut experiment3_candidates = Vec::new();

            for outer_schedule in folding_schedules_up_to(n) {
                let initial_folding_log = outer_schedule[0];
                let outer_tail_schedule = &outer_schedule[1..];
                let outer_terminal = terminal_message_variables(n, &outer_schedule);

                let ordinary_outer = paper_parameters(
                    n,
                    OUTER_STARTING_LOG_INV_RATE,
                    decoding_regime,
                    outer_terminal,
                    &outer_schedule,
                    1,
                )?;
                let ordinary_tail = paper_parameters(
                    n - initial_folding_log,
                    initial_folding_log,
                    decoding_regime,
                    outer_terminal,
                    outer_tail_schedule,
                    1,
                )?;
                debug_assert_eq!(
                    query_geometry(&ordinary_outer)[1],
                    query_geometry(&ordinary_tail)[0]
                );

                let ordinary_first_geometry = query_geometry(&ordinary_outer)[0];
                let ordinary_first_queries = (0..samples)
                    .map(|sample| {
                        sample_initial_oracle_queries(
                            &ordinary_outer,
                            expand_seed(derive_u64(
                                seed,
                                &[0, input_oracles, n, sample, 0x0046_4952_5354],
                            )),
                        )
                    })
                    .collect::<Vec<_>>();

                if input_oracles == 1 {
                    let mut independent = Totals::default();
                    let mut independent_queries = BTreeMap::new();
                    for sample in 0..samples {
                        let proof = estimate_proof(
                            &ordinary_outer,
                            expand_seed(derive_u64(
                                seed,
                                &[0, input_oracles, n, sample, 0x0049_4e44_4550],
                            )),
                        )?;
                        for _ in 0..input_oracles {
                            independent.add_whir_with_initial_root(&proof);
                        }
                        accumulate_whir_queries(
                            &mut independent_queries,
                            0,
                            "merkle_whir_batched_after_second",
                            input_oracles,
                            n,
                            "independent WHIR",
                            "independent Merkle roots",
                            &ordinary_outer,
                            &proof,
                            input_oracles,
                            input_oracles,
                            input_oracles,
                            input_oracles,
                            false,
                            true,
                        );
                    }
                    let independent_events = repeated_rbr_events(
                        "independent WHIR",
                        &ordinary_outer,
                        false,
                        input_oracles,
                    );
                    independent_candidates.push(ProofCandidate {
                        input_oracles,
                        batched_source_oracles: 0,
                        starting_log_inverse_rate: OUTER_STARTING_LOG_INV_RATE,
                        initial_folding_log,
                        canopy_height: None,
                        outer_folding_schedule: outer_schedule.clone(),
                        canopy_folding_schedule: None,
                        recursive_field_bits: FIELD_BITS,
                        minimum_rbr_security_bits: minimum_event_security(&independent_events),
                        algebraic_rbr_union_bound_bits: union_bound_security(&independent_events),
                        rbr_events: independent_events,
                        totals: independent,
                        queries: independent_queries,
                    });
                }
                let mut batched_baseline = Totals::default();
                let mut batched_baseline_queries = BTreeMap::new();
                for (sample, first_queries) in ordinary_first_queries.iter().enumerate() {
                    let first_auth = frontier_hash_count(
                        first_queries,
                        ordinary_first_geometry.4,
                        ordinary_first_geometry.4.ilog2() as usize,
                    )?;
                    let first_fields = first_queries.len() * ordinary_first_geometry.3;
                    for _ in 0..input_oracles {
                        batched_baseline.add_merkle_front(
                            &ordinary_outer,
                            first_auth,
                            first_fields,
                        );
                    }
                    let tail = estimate_schwartz_zippel_batched_initial_proofs(
                        &ordinary_tail,
                        input_oracles,
                        expand_seed(derive_u64(
                            seed,
                            &[0, input_oracles, n, sample, 0x0042_4154_4348],
                        )),
                    )?;
                    batched_baseline.add_whir_with_existing_root(&tail);
                    accumulate_front_queries(
                        &mut batched_baseline_queries,
                        0,
                        "merkle_whir_batched_after_second",
                        input_oracles,
                        n,
                        "independent first Merkle oracles",
                        "independent Merkle roots",
                        ordinary_first_geometry,
                        first_queries.len(),
                        input_oracles * first_auth,
                        input_oracles * first_fields,
                    );
                    accumulate_whir_queries(
                        &mut batched_baseline_queries,
                        0,
                        "merkle_whir_batched_after_second",
                        input_oracles,
                        n,
                        "SZ-batched second-oracle claim",
                        "separate source roots; one recursive root",
                        &ordinary_tail,
                        &tail,
                        input_oracles,
                        1,
                        1,
                        1,
                        true,
                        true,
                    );
                }
                let mut batched_baseline_events = repeated_rbr_events(
                    "independent first transition",
                    &ordinary_outer,
                    true,
                    input_oracles,
                );
                if input_oracles > 1 {
                    batched_baseline_events.push(TaggedRbrEvent {
                        phase: "second-oracle batching".to_owned(),
                        event: format!(
                            "degree-{} generalized Schwartz-Zippel identity",
                            2 * (input_oracles - 1)
                        ),
                        security_bits: sz_batching_security_bits(&ordinary_tail, input_oracles),
                    });
                }
                batched_baseline_events.extend(tagged_rbr_events(
                    "combined recursive WHIR",
                    &ordinary_tail,
                    false,
                ));
                batched_baseline_candidates.push(ProofCandidate {
                    input_oracles,
                    batched_source_oracles: input_oracles,
                    starting_log_inverse_rate: OUTER_STARTING_LOG_INV_RATE,
                    initial_folding_log,
                    canopy_height: None,
                    outer_folding_schedule: outer_schedule.clone(),
                    canopy_folding_schedule: None,
                    recursive_field_bits: FIELD_BITS,
                    minimum_rbr_security_bits: minimum_event_security(&batched_baseline_events),
                    algebraic_rbr_union_bound_bits: union_bound_security(&batched_baseline_events),
                    rbr_events: batched_baseline_events,
                    totals: batched_baseline,
                    queries: batched_baseline_queries,
                });

                let quarter_outer = paper_parameters(
                    n,
                    QUARTER_RATE_STARTING_LOG_INV_RATE,
                    decoding_regime,
                    outer_terminal,
                    &outer_schedule,
                    1,
                )?;
                let quarter_tail_log_inverse_rate =
                    QUARTER_RATE_STARTING_LOG_INV_RATE + initial_folding_log - 1;
                let quarter_tail = paper_parameters(
                    n - initial_folding_log,
                    quarter_tail_log_inverse_rate,
                    decoding_regime,
                    outer_terminal,
                    outer_tail_schedule,
                    1,
                )?;
                debug_assert_eq!(
                    query_geometry(&quarter_outer)[1],
                    query_geometry(&quarter_tail)[0]
                );
                let quarter_first_geometry = query_geometry(&quarter_outer)[0];
                let quarter_first_queries = (0..samples)
                    .map(|sample| {
                        sample_initial_oracle_queries(
                            &quarter_outer,
                            expand_seed(derive_u64(
                                seed,
                                &[0, input_oracles, n, sample, 0x0051_5541_5254],
                            )),
                        )
                    })
                    .collect::<Vec<_>>();
                let mut quarter_rate_batched_baseline = Totals::default();
                let mut quarter_rate_direct = Totals::default();
                let mut quarter_direct_queries = BTreeMap::new();
                let mut quarter_rate_batched_baseline_queries = BTreeMap::new();
                for (sample, first_queries) in quarter_first_queries.iter().enumerate() {
                    let first_auth = frontier_hash_count(
                        first_queries,
                        quarter_first_geometry.4,
                        quarter_first_geometry.4.ilog2() as usize,
                    )?;
                    let first_fields = first_queries.len() * quarter_first_geometry.3;
                    for _ in 0..input_oracles {
                        quarter_rate_batched_baseline.add_merkle_front(
                            &quarter_outer,
                            first_auth,
                            first_fields,
                        );
                    }
                    let tail = estimate_schwartz_zippel_batched_initial_proofs(
                        &quarter_tail,
                        input_oracles,
                        expand_seed(derive_u64(
                            seed,
                            &[0, input_oracles, n, sample, 0x0051_4241_5443],
                        )),
                    )?;
                    quarter_rate_batched_baseline.add_whir_with_existing_root(&tail);
                    if input_oracles == 1 {
                        let direct = estimate_proof(
                            &quarter_outer,
                            expand_seed(derive_u64(
                                seed,
                                &[0, input_oracles, n, sample, 0x0051_4449_5245],
                            )),
                        )?;
                        quarter_rate_direct.add_whir_with_initial_root(&direct);
                        accumulate_whir_queries(
                            &mut quarter_direct_queries,
                            0,
                            "merkle_whir_batched_rate_quarter_after_second",
                            input_oracles,
                            n,
                            "single-input WHIR",
                            "independent Merkle roots",
                            &quarter_outer,
                            &direct,
                            1,
                            1,
                            1,
                            1,
                            false,
                            true,
                        );
                    }
                    accumulate_front_queries(
                        &mut quarter_rate_batched_baseline_queries,
                        0,
                        "merkle_whir_batched_rate_quarter_after_second",
                        input_oracles,
                        n,
                        "independent first Merkle oracles",
                        "independent Merkle roots",
                        quarter_first_geometry,
                        first_queries.len(),
                        input_oracles * first_auth,
                        input_oracles * first_fields,
                    );
                    accumulate_whir_queries(
                        &mut quarter_rate_batched_baseline_queries,
                        0,
                        "merkle_whir_batched_rate_quarter_after_second",
                        input_oracles,
                        n,
                        "SZ-batched second-oracle claim",
                        "separate source roots; one recursive root",
                        &quarter_tail,
                        &tail,
                        input_oracles,
                        1,
                        1,
                        1,
                        true,
                        true,
                    );
                }
                let mut quarter_rate_events = repeated_rbr_events(
                    "independent first transition",
                    &quarter_outer,
                    true,
                    input_oracles,
                );
                if input_oracles > 1 {
                    quarter_rate_events.push(TaggedRbrEvent {
                        phase: "second-oracle batching".to_owned(),
                        event: format!(
                            "degree-{} generalized Schwartz-Zippel identity",
                            2 * (input_oracles - 1)
                        ),
                        security_bits: sz_batching_security_bits(&quarter_tail, input_oracles),
                    });
                }
                quarter_rate_events.extend(tagged_rbr_events(
                    "combined recursive WHIR",
                    &quarter_tail,
                    false,
                ));
                if input_oracles == 1 {
                    quarter_rate_batched_baseline = quarter_rate_direct;
                    quarter_rate_batched_baseline_queries = quarter_direct_queries;
                    quarter_rate_events = tagged_rbr_events("rate-1/4 WHIR", &quarter_outer, false);
                }
                quarter_rate_batched_baseline_candidates.push(ProofCandidate {
                    input_oracles,
                    batched_source_oracles: input_oracles,
                    starting_log_inverse_rate: QUARTER_RATE_STARTING_LOG_INV_RATE,
                    initial_folding_log,
                    canopy_height: None,
                    outer_folding_schedule: outer_schedule.clone(),
                    canopy_folding_schedule: None,
                    recursive_field_bits: FIELD_BITS,
                    minimum_rbr_security_bits: minimum_event_security(&quarter_rate_events),
                    algebraic_rbr_union_bound_bits: union_bound_security(&quarter_rate_events),
                    rbr_events: quarter_rate_events,
                    totals: quarter_rate_batched_baseline,
                    queries: quarter_rate_batched_baseline_queries,
                });

                let hybrid_outer = paper_parameters(
                    n,
                    OUTER_STARTING_LOG_INV_RATE,
                    decoding_regime,
                    outer_terminal,
                    &outer_schedule,
                    CANOPY_HYBRID_CODEWORD_REDUCTION,
                )?;
                let hybrid_tail = paper_parameters(
                    n - initial_folding_log,
                    initial_folding_log + 1,
                    decoding_regime,
                    outer_terminal,
                    outer_tail_schedule,
                    1,
                )?;
                debug_assert_eq!(
                    query_geometry(&hybrid_outer)[1],
                    query_geometry(&hybrid_tail)[0]
                );
                let hybrid_first_geometry = query_geometry(&hybrid_outer)[0];
                let first_width = hybrid_first_geometry.3;
                let hybrid_first_queries = (0..samples)
                    .map(|sample| {
                        sample_initial_oracle_queries(
                            &hybrid_outer,
                            expand_seed(derive_u64(
                                seed,
                                &[2, input_oracles, n, sample, 0x0046_4952_5354],
                            )),
                        )
                    })
                    .collect::<Vec<_>>();
                let bottom_hashes_by_height = (1..=n)
                    .map(|height| {
                        hybrid_first_queries
                            .iter()
                            .map(|queries| {
                                if height >= initial_folding_log {
                                    frontier_hash_count(
                                        queries,
                                        hybrid_first_geometry.4,
                                        height - initial_folding_log,
                                    )
                                } else {
                                    Ok(0)
                                }
                            })
                            .collect::<Result<Vec<_>>>()
                    })
                    .collect::<Result<Vec<_>>>()?;

                let canopy_height = aligned_canopy_height(initial_folding_log);
                let source_oracles = aligned_source_oracles(input_oracles);
                let mut experiment3 = Totals::default();
                let mut experiment3_queries = BTreeMap::new();
                for sample in 0..samples {
                    let first_queries = &hybrid_first_queries[sample];
                    let bottom_hashes = bottom_hashes_by_height[canopy_height - 1][sample];
                    let first_fields = first_queries.len() * first_width;
                    for _ in 0..input_oracles {
                        experiment3.add_hybrid_front(&hybrid_outer, bottom_hashes, first_fields);
                    }
                    let tail = estimate_schwartz_zippel_batched_initial_proofs(
                        &hybrid_tail,
                        source_oracles,
                        expand_seed(derive_u64(
                            seed,
                            &[3, input_oracles, n, sample, 0x0053_5a42_4154],
                        )),
                    )?;
                    experiment3.add_whir_with_existing_root(&tail);
                    accumulate_front_queries(
                        &mut experiment3_queries,
                        3,
                        "canopy_schwartz_zippel",
                        input_oracles,
                        n,
                        "outer first oracle via Canopy",
                        "independent Canopy bottom forests",
                        hybrid_first_geometry,
                        first_queries.len(),
                        input_oracles * bottom_hashes,
                        input_oracles * first_fields,
                    );
                    accumulate_whir_queries(
                        &mut experiment3_queries,
                        3,
                        "canopy_schwartz_zippel",
                        input_oracles,
                        n,
                        "SZ-batched aligned claims",
                        "separate source roots; one recursive root",
                        &hybrid_tail,
                        &tail,
                        source_oracles,
                        1,
                        1,
                        1,
                        true,
                        true,
                    );
                }
                let mut experiment3_events = repeated_rbr_events(
                    "outer initial transition",
                    &hybrid_outer,
                    true,
                    input_oracles,
                );
                experiment3_events.push(TaggedRbrEvent {
                    phase: "aligned-claim batching".to_owned(),
                    event: format!(
                        "degree-{} generalized Schwartz-Zippel identity",
                        2 * (source_oracles - 1)
                    ),
                    security_bits: sz_batching_security_bits(&hybrid_tail, source_oracles),
                });
                experiment3_events.extend(tagged_rbr_events(
                    "combined recursive WHIR",
                    &hybrid_tail,
                    false,
                ));
                experiment3_candidates.push(ProofCandidate {
                    input_oracles,
                    batched_source_oracles: source_oracles,
                    starting_log_inverse_rate: OUTER_STARTING_LOG_INV_RATE,
                    initial_folding_log,
                    canopy_height: Some(canopy_height),
                    outer_folding_schedule: outer_schedule.clone(),
                    canopy_folding_schedule: Some(outer_tail_schedule.to_vec()),
                    recursive_field_bits: FIELD_BITS,
                    minimum_rbr_security_bits: minimum_event_security(&experiment3_events),
                    algebraic_rbr_union_bound_bits: union_bound_security(&experiment3_events),
                    rbr_events: experiment3_events,
                    totals: experiment3,
                    queries: experiment3_queries,
                });
            }

            // With one input there is no batching challenge: the second
            // baseline is exactly the ordinary WHIR proof, not a separately
            // sampled decomposition of it.
            if input_oracles == 1 {
                for (independent, batched) in independent_candidates
                    .iter()
                    .zip(&mut batched_baseline_candidates)
                {
                    batched.totals = independent.totals.clone();
                    batched.minimum_rbr_security_bits = independent.minimum_rbr_security_bits;
                    batched.algebraic_rbr_union_bound_bits =
                        independent.algebraic_rbr_union_bound_bits;
                    batched.rbr_events = independent.rbr_events.clone();
                    batched.queries = independent.queries.clone();
                }
            }

            let batched_baseline = best_rbr_candidate(&batched_baseline_candidates);
            let quarter_rate_batched_baseline =
                best_rbr_candidate(&quarter_rate_batched_baseline_candidates);
            let experiment3 = best_rbr_candidate(&experiment3_candidates);
            let baseline_bytes = batched_baseline.totals.bytes() as f64 / samples as f64;
            let batched_baseline_bytes = batched_baseline.totals.bytes() as f64 / samples as f64;

            for (experiment, scheme, candidates) in [
                (
                    0,
                    "merkle_whir_batched_after_second",
                    &batched_baseline_candidates,
                ),
                (
                    0,
                    "merkle_whir_batched_rate_quarter_after_second",
                    &quarter_rate_batched_baseline_candidates,
                ),
                (3, "canopy_schwartz_zippel", &experiment3_candidates),
            ] {
                folding_grid_rows.extend(candidates.iter().map(|candidate| {
                    candidate_row(
                        experiment,
                        scheme,
                        n,
                        samples,
                        candidate,
                        baseline_bytes,
                        batched_baseline_bytes,
                    )
                }));
            }

            for (experiment, scheme, candidate) in [
                (0, "merkle_whir_batched_after_second", batched_baseline),
                (
                    0,
                    "merkle_whir_batched_rate_quarter_after_second",
                    quarter_rate_batched_baseline,
                ),
                (3, "canopy_schwartz_zippel", experiment3),
            ] {
                proof_rows.push(candidate_row(
                    experiment,
                    scheme,
                    n,
                    samples,
                    candidate,
                    baseline_bytes,
                    batched_baseline_bytes,
                ));
                rbr_security_rows.extend(candidate.rbr_events.iter().map(|event| RbrSecurityRow {
                    experiment,
                    scheme: scheme.to_owned(),
                    input_oracles,
                    log_message_length: n,
                    phase: event.phase.clone(),
                    event: event.event.clone(),
                    security_bits: event.security_bits,
                }));
                query_accumulators.extend(candidate.queries.clone());
            }
        }
    }

    let query_rows = query_accumulators
        .into_values()
        .map(QueryAccumulator::row)
        .collect::<Vec<_>>();
    write_csv(output_dir.join("hybrid_proof_sizes.csv"), &proof_rows)?;
    write_csv(
        output_dir.join("hybrid_parameter_grid.csv"),
        &folding_grid_rows,
    )?;
    write_csv(output_dir.join("oracle_query_breakdown.csv"), &query_rows)?;
    write_csv(
        output_dir.join("rbr_security_breakdown.csv"),
        &rbr_security_rows,
    )?;

    for experiment in [3usize] {
        let experiment_proofs = proof_rows
            .iter()
            .filter(|row| row.experiment == 0 || row.experiment == experiment)
            .cloned()
            .collect::<Vec<_>>();
        let experiment_queries = query_rows
            .iter()
            .filter(|row| row.experiment == 0 || row.experiment == experiment)
            .cloned()
            .collect::<Vec<_>>();
        write_csv(
            output_dir.join(format!("experiment{experiment}_vs_merkle_proof_sizes.csv")),
            &experiment_proofs,
        )?;
        write_csv(
            output_dir.join(format!(
                "experiment{experiment}_vs_merkle_oracle_queries.csv"
            )),
            &experiment_queries,
        )?;
    }
    write_multi_input_summary(
        output_dir.join("summary.md"),
        samples,
        seed,
        decoding_regime,
        &proof_rows,
        &query_rows,
    )?;
    Ok(())
}

fn write_multi_input_summary(
    path: impl AsRef<Path>,
    samples: usize,
    seed: u64,
    decoding_regime: WhirDecodingRegime,
    proof_rows: &[HybridProofSizeRow],
    query_rows: &[OracleQueryBreakdownRow],
) -> Result<()> {
    let mut file = File::create(path)?;
    writeln!(
        file,
        "# Batched WHIR and Canopy Exp3 ({})\n",
        decoding_regime.label()
    )?;
    writeln!(file, "Batched WHIR rates 1/2 and 1/4, and Canopy Exp3; GF(2^256), BLAKE3, 128-bit target with a 136-bit local algebraic/query budget. t=1,2,4,8; n=18,20,...,30. Fold logs 3/4 searched with terminal cap 10. {} deterministic query samples, seed {:#x}; {:?} decoding. Authentication counts are verifier leaf and parent calls; H2F is separate. WHIR proof models deduplicate query rows.\n", samples, seed, decoding_regime)?;
    writeln!(file, "## Selected proof sizes\n")?;
    writeln!(
        file,
        "Both relative columns use the corresponding Merkle-WHIR baseline at the same `t,n`.\n"
    )?;
    writeln!(file, "| t | log2 n | scheme | rate | batched sources | folds | h | terminal | min event bits | union bits | 128-bit RBR? | KiB | verifier auth hashes | H2F | vs batched | vs batched |")?;
    writeln!(
        file,
        "|---:|---:|---|---|---:|---|---:|---:|---:|---:|:---:|---:|---:|---:|---:|---:|"
    )?;
    for row in proof_rows {
        writeln!(
            file,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {:.2} | {:.2} | {} | {:.2} | {:.2} | {:.2} | {:.3}x | {:.3}x |",
            row.input_oracles,
            row.log_message_length,
            row.scheme,
            row.outer_starting_rate,
            row.batched_source_oracles,
            row.outer_folding_schedule.replace(';', ","),
            row.canopy_height
                .map_or_else(|| "-".to_owned(), |h| h.to_string()),
            row.outer_terminal_message_variables,
            row.minimum_rbr_security_bits,
            row.algebraic_rbr_union_bound_bits,
            if row.meets_128_bit_rbr { "yes" } else { "no" },
            row.mean_total_kib,
            row.verifier_auth_hashes,
            row.verifier_hash_to_field,
            row.relative_to_merkle_whir,
            row.relative_to_batched_merkle_whir,
        )?;
    }

    writeln!(file, "\n## Component breakdown\n")?;
    writeln!(
        file,
        "Size entries are KiB; authentication and H2F columns count hash calls.\n"
    )?;
    writeln!(file, "| t | log2 n | scheme | roots | auth | OOD | sumcheck | batching | answers | coefficients | total | verifier auth hashes | H2F |")?;
    writeln!(
        file,
        "|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"
    )?;
    for row in proof_rows {
        writeln!(
            file,
            "| {} | {} | {} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} |",
            row.input_oracles,
            row.log_message_length,
            row.scheme,
            row.mean_root_hashes * HASH_BYTES as f64 / 1024.0,
            row.mean_auth_hashes * HASH_BYTES as f64 / 1024.0,
            row.mean_ood_bytes / 1024.0,
            row.mean_sumcheck_bytes / 1024.0,
            row.mean_batching_bytes / 1024.0,
            row.mean_query_answer_bytes / 1024.0,
            row.mean_final_coefficient_bytes / 1024.0,
            row.mean_total_kib,
            row.verifier_auth_hashes,
            row.verifier_hash_to_field,
        )?;
    }

    writeln!(file, "\n## Oracle queries at n=30\n")?;
    writeln!(file, "`unique` is the mean number of distinct positions per queried oracle group; authentication and answer columns are total transmitted values across the reported oracle count.\n")?;
    writeln!(file, "| exp | t | scheme | role | oracles | code | field | requested | unique | auth | answer fields |")?;
    writeln!(
        file,
        "|---:|---:|---|---|---:|---|---:|---:|---:|---:|---:|"
    )?;
    for row in query_rows.iter().filter(|row| row.log_message_length == 30) {
        writeln!(
            file,
            "| {} | {} | {} | {} | {} | [2^{},2^{}] | {} | {} | {:.2} | {:.2} | {:.2} |",
            row.experiment,
            row.input_oracles,
            row.scheme,
            row.oracle_role,
            row.oracle_count,
            row.codeword_log,
            row.message_log,
            row.field_bits,
            row.requested_queries,
            row.mean_unique_queries,
            row.mean_auth_hashes,
            row.mean_answer_fields,
        )?;
    }

    Ok(())
}

#[derive(Clone, Debug, Serialize)]
pub struct PaperTimingRow {
    pub experiment: usize,
    pub scheme: String,
    pub input_oracles: usize,
    pub log_input_length: usize,
    pub log_initial_codeword_length: usize,
    pub executable_folding_log: usize,
    pub canopy_height: Option<usize>,
    pub warmups: usize,
    pub repetitions: usize,
    pub mean_commitment_seconds: f64,
    pub median_commitment_seconds: f64,
    pub mean_opening_seconds: f64,
    pub median_opening_seconds: f64,
    pub mean_prover_seconds: f64,
    pub median_prover_seconds: f64,
    pub min_prover_seconds: f64,
    pub max_prover_seconds: f64,
    pub relative_to_independent_baseline: f64,
    pub relative_to_batched_baseline: f64,
    pub note: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct PaperTimingStageRow {
    pub experiment: usize,
    pub scheme: String,
    pub input_oracles: usize,
    pub log_input_length: usize,
    pub stage: String,
    pub repetitions: usize,
    pub mean_seconds: f64,
    pub median_seconds: f64,
    pub percent_of_mean_total: f64,
}

fn duration_stats(values: &[Duration]) -> (f64, f64, f64, f64) {
    let mut seconds: Vec<_> = values.iter().map(Duration::as_secs_f64).collect();
    seconds.sort_by(f64::total_cmp);
    let mean = seconds.iter().sum::<f64>() / seconds.len() as f64;
    let median = if seconds.len().is_multiple_of(2) {
        (seconds[seconds.len() / 2 - 1] + seconds[seconds.len() / 2]) / 2.0
    } else {
        seconds[seconds.len() / 2]
    };
    (mean, median, seconds[0], *seconds.last().unwrap())
}

#[allow(clippy::too_many_arguments)]
fn timing_row(
    experiment: usize,
    scheme: &str,
    input_oracles: usize,
    log_input_length: usize,
    starting_log_inverse_rate: usize,
    height: Option<usize>,
    warmups: usize,
    values: &[Duration],
    commitment_values: &[Duration],
    independent_baseline_mean: f64,
    batched_baseline_mean: f64,
    note: &str,
) -> PaperTimingRow {
    debug_assert_eq!(values.len(), commitment_values.len());
    let (mean, median, min, max) = duration_stats(values);
    let (commitment_mean, commitment_median, _, _) = duration_stats(commitment_values);
    let opening_values = values
        .iter()
        .zip(commitment_values)
        .map(|(&total, &commitment)| total.saturating_sub(commitment))
        .collect::<Vec<_>>();
    let (opening_mean, opening_median, _, _) = duration_stats(&opening_values);
    PaperTimingRow {
        experiment,
        scheme: scheme.to_owned(),
        input_oracles,
        log_input_length,
        log_initial_codeword_length: log_input_length + starting_log_inverse_rate,
        executable_folding_log: PAPER_TIMING_FOLDING_LOG,
        canopy_height: height,
        warmups,
        repetitions: values.len(),
        mean_commitment_seconds: commitment_mean,
        median_commitment_seconds: commitment_median,
        mean_opening_seconds: opening_mean,
        median_opening_seconds: opening_median,
        mean_prover_seconds: mean,
        median_prover_seconds: median,
        min_prover_seconds: min,
        max_prover_seconds: max,
        relative_to_independent_baseline: mean / independent_baseline_mean,
        relative_to_batched_baseline: mean / batched_baseline_mean,
        note: note.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn stage_rows(
    experiment: usize,
    scheme: &str,
    input_oracles: usize,
    log_input_length: usize,
    total_values: &[Duration],
    stages: BTreeMap<&'static str, Vec<Duration>>,
) -> Vec<PaperTimingStageRow> {
    let total_mean = duration_stats(total_values).0;
    let mut adjustment: Vec<f64> = total_values
        .iter()
        .enumerate()
        .map(|(i, total)| {
            total.as_secs_f64() - stages.values().map(|v| v[i].as_secs_f64()).sum::<f64>()
        })
        .collect();
    let mut rows: Vec<_> = stages
        .into_iter()
        .map(|(stage, values)| {
            let (mean, median, _, _) = duration_stats(&values);
            PaperTimingStageRow {
                experiment,
                scheme: scheme.to_owned(),
                input_oracles,
                log_input_length,
                stage: stage.to_owned(),
                repetitions: values.len(),
                mean_seconds: mean,
                median_seconds: median,
                percent_of_mean_total: 100.0 * mean / total_mean,
            }
        })
        .collect();
    // Independently timed tail subtraction can overlap measured substages.
    // Report a signed adjustment; do not alter any measured duration.
    if adjustment.iter().any(|x| x.abs() > 1e-9) {
        let mean = adjustment.iter().sum::<f64>() / adjustment.len() as f64;
        adjustment.sort_by(f64::total_cmp);
        let middle = adjustment.len() / 2;
        let median = if adjustment.len() % 2 == 0 {
            (adjustment[middle - 1] + adjustment[middle]) / 2.0
        } else {
            adjustment[middle]
        };
        rows.push(PaperTimingStageRow {
            experiment,
            scheme: scheme.into(),
            input_oracles,
            log_input_length,
            stage: "separately_measured_component_adjustment".into(),
            repetitions: adjustment.len(),
            mean_seconds: mean,
            median_seconds: median,
            percent_of_mean_total: 100.0 * mean / total_mean,
        });
    }
    rows
}

/// Remove the reference implementation's native sub-`2^f` suffix so the
/// measured recursive prover stops at the paper's at-most-`2^10` terminal.
/// The full proof and the suffix proof are both verifier-checked executions.
fn proving_time_until_paper_terminal(
    full_opening_time: Duration,
    source: &[Fr],
    starting_log_inverse_rate: usize,
    decoding_regime: WhirDecodingRegime,
) -> Result<Duration> {
    let mut message_variables = source.len().ilog2() as usize;
    let mut log_inverse_rate = starting_log_inverse_rate;
    let mut folds = 0;
    while message_variables > PAPER_TERMINAL_MESSAGE_VARIABLES {
        message_variables -= PAPER_TIMING_FOLDING_LOG;
        log_inverse_rate += PAPER_TIMING_FOLDING_LOG - 1;
        folds += 1;
    }
    if folds == 0 {
        return Ok(Duration::ZERO);
    }
    let terminal = source[..1usize << message_variables].to_vec();
    let suffix = execute_whir(
        &[terminal],
        log_inverse_rate,
        decoding_regime,
        PAPER_TIMING_FOLDING_LOG,
    )?;
    Ok(full_opening_time.saturating_sub(suffix.ood_time + suffix.proving_time))
}

// Optimize the actual executable (uniform fold-3) geometry, independently of
// the {3,4} schedule search used for headline proof sizes. Cache outside timers.
struct HybridIteration {
    initial_commitment: Duration,
    ordinary_first_transition: Duration,
    ordinary_first_transition_stages: BTreeMap<&'static str, Duration>,
    ordinary_tail: Duration,
    ordinary_tail_ood: Duration,
    quarter_initial_commitment: Duration,
    quarter_first_transition: Duration,
    quarter_tail: Duration,
    quarter_tail_ood: Duration,
    quarter_first_transition_stages: BTreeMap<&'static str, Duration>,
    outer_ood: Duration,
    hybrid_outer_front_after_ood: Duration,
    outer_second: Vec<Fr>,
}

fn run_hybrid_iteration(
    message: &[Fr],
    decoding_regime: WhirDecodingRegime,
) -> Result<HybridIteration> {
    let outer = execute_whir(
        &[message.to_vec()],
        OUTER_STARTING_LOG_INV_RATE,
        decoding_regime,
        PAPER_TIMING_FOLDING_LOG,
    )?;
    let outer_proving_time = proving_time_until_paper_terminal(
        outer.ood_time + outer.proving_time,
        message,
        OUTER_STARTING_LOG_INV_RATE,
        decoding_regime,
    )?;

    let start = Instant::now();
    let first_oracle = encode_interleaved_with_whir(
        message,
        OUTER_STARTING_LOG_INV_RATE,
        PAPER_TIMING_FOLDING_LOG,
    )?;
    let outer_rs = start.elapsed();
    let (base_commitment, outer_merkle) =
        commit_matrix_with_whir(&first_oracle, first_oracle.row_width)?;
    std::hint::black_box(base_commitment.root);
    let initial_commitment = outer_rs + outer_merkle;
    let quarter = execute_whir(
        &[message.to_vec()],
        QUARTER_RATE_STARTING_LOG_INV_RATE,
        decoding_regime,
        PAPER_TIMING_FOLDING_LOG,
    )?;
    let quarter_proving_time = proving_time_until_paper_terminal(
        quarter.ood_time + quarter.proving_time,
        message,
        QUARTER_RATE_STARTING_LOG_INV_RATE,
        decoding_regime,
    )?;
    let start = Instant::now();
    let quarter_first_oracle = encode_interleaved_with_whir(
        message,
        QUARTER_RATE_STARTING_LOG_INV_RATE,
        PAPER_TIMING_FOLDING_LOG,
    )?;
    let quarter_rs = start.elapsed();
    let (quarter_commitment, quarter_merkle) =
        commit_matrix_with_whir(&quarter_first_oracle, quarter_first_oracle.row_width)?;
    std::hint::black_box(quarter_commitment.root);
    let quarter_initial_commitment = quarter_rs + quarter_merkle;
    let aligned_len = message.len() >> PAPER_TIMING_FOLDING_LOG;
    let outer_second = message[..aligned_len].to_vec();

    let aligned_log_inverse_rate = PAPER_TIMING_FOLDING_LOG + 1 - CANOPY_HYBRID_CODEWORD_REDUCTION;
    let ordinary_tail = execute_whir(
        std::slice::from_ref(&outer_second),
        PAPER_TIMING_FOLDING_LOG,
        decoding_regime,
        PAPER_TIMING_FOLDING_LOG,
    )?;
    let ordinary_tail_proving_time = proving_time_until_paper_terminal(
        ordinary_tail.ood_time + ordinary_tail.proving_time,
        &outer_second,
        PAPER_TIMING_FOLDING_LOG,
        decoding_regime,
    )?;
    let quarter_tail_log_inverse_rate =
        QUARTER_RATE_STARTING_LOG_INV_RATE + PAPER_TIMING_FOLDING_LOG - 1;
    let quarter_tail = execute_whir(
        std::slice::from_ref(&outer_second),
        quarter_tail_log_inverse_rate,
        decoding_regime,
        PAPER_TIMING_FOLDING_LOG,
    )?;
    let quarter_tail_proving_time = proving_time_until_paper_terminal(
        quarter_tail.ood_time + quarter_tail.proving_time,
        &outer_second,
        quarter_tail_log_inverse_rate,
        decoding_regime,
    )?;
    let hybrid_outer_tail = execute_whir(
        std::slice::from_ref(&outer_second),
        aligned_log_inverse_rate,
        decoding_regime,
        PAPER_TIMING_FOLDING_LOG,
    )?;
    let hybrid_outer_tail_proving_time = proving_time_until_paper_terminal(
        hybrid_outer_tail.ood_time + hybrid_outer_tail.proving_time,
        &outer_second,
        aligned_log_inverse_rate,
        decoding_regime,
    )?;
    let ordinary_first_transition =
        initial_commitment + outer_proving_time.saturating_sub(ordinary_tail_proving_time);
    let ordinary_first_opening = outer_proving_time.saturating_sub(ordinary_tail_proving_time);
    let ordinary_first_transition_stages = BTreeMap::from([
        ("initial_rs_encoding", outer_rs),
        ("initial_merkle_tree", outer_merkle),
        ("initial_ood_evaluations", outer.ood_time),
        (
            "second_oracle_rs_and_merkle_commitment",
            ordinary_tail.commitment_time,
        ),
        (
            "first_transition_residual_after_subtracting_tail",
            ordinary_first_opening
                .saturating_sub(outer.ood_time)
                .saturating_sub(ordinary_tail.commitment_time),
        ),
    ]);
    let quarter_first_opening = quarter_proving_time.saturating_sub(quarter_tail_proving_time);
    let quarter_first_transition = quarter_initial_commitment + quarter_first_opening;
    let quarter_first_transition_stages = BTreeMap::from([
        ("initial_rs_encoding", quarter_rs),
        ("initial_merkle_tree", quarter_merkle),
        ("initial_ood_evaluations", quarter.ood_time),
        (
            "second_oracle_rs_and_merkle_commitment",
            quarter_tail.commitment_time,
        ),
        (
            "first_transition_residual_after_subtracting_tail",
            quarter_first_opening
                .saturating_sub(quarter.ood_time)
                .saturating_sub(quarter_tail.commitment_time),
        ),
    ]);
    let hybrid_outer_front = outer_proving_time.saturating_sub(hybrid_outer_tail_proving_time);
    Ok(HybridIteration {
        initial_commitment,
        ordinary_first_transition,
        ordinary_first_transition_stages,
        ordinary_tail: ordinary_tail_proving_time,
        ordinary_tail_ood: ordinary_tail.ood_time,
        quarter_initial_commitment,
        quarter_first_transition,
        quarter_tail: quarter_tail_proving_time,
        quarter_tail_ood: quarter_tail.ood_time,
        quarter_first_transition_stages,
        outer_ood: outer.ood_time,
        hybrid_outer_front_after_ood: hybrid_outer_front.saturating_sub(outer.ood_time),
        outer_second,
    })
}

fn scale_duration(duration: Duration, copies: usize) -> Duration {
    Duration::from_secs_f64(duration.as_secs_f64() * copies as f64)
}

fn evaluate_multilinear_with_scratch(values: &[Fr], point: &[Fr], scratch: &mut Vec<Fr>) -> Fr {
    debug_assert_eq!(values.len(), 1usize << point.len());
    scratch.resize(values.len(), Fr::from(0u64));
    scratch.copy_from_slice(values);
    let mut active_len = values.len();
    for coordinate in point {
        let half = active_len / 2;
        let one_minus = Fr::from(1u64) - coordinate;
        for index in 0..half {
            scratch[index] = scratch[2 * index] * one_minus + scratch[2 * index + 1] * coordinate;
        }
        active_len = half;
    }
    scratch[0]
}

/// Measure the off-diagonal evaluations required by the generalized
/// Schwartz--Zippel identity for claims with distinct linear functionals.
fn generalized_sz_cross_term_work(vectors: &[Vec<Fr>]) -> Duration {
    if vectors.len() <= 1 {
        return Duration::ZERO;
    }
    let variables = vectors[0].len().ilog2() as usize;
    let points = (0..vectors.len())
        .map(|claim| {
            (0..variables)
                .map(|variable| {
                    crate::benchmark_field_element(
                        0x43414e4f5059,
                        claim as u64 + 1,
                        variable as u64,
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let start = Instant::now();
    let diagonal = vectors
        .par_iter()
        .zip(points.par_iter())
        .map_init(Vec::new, |scratch, (vector, point)| {
            evaluate_multilinear_with_scratch(vector, point, scratch)
        })
        .collect::<Vec<_>>();
    let coefficient_count = 2 * vectors.len() - 3;
    let pair_count = vectors.len() * vectors.len();
    let coefficients = (0..pair_count)
        .into_par_iter()
        .filter(|pair| pair / vectors.len() != pair % vectors.len())
        .map_init(Vec::new, |scratch, pair| {
            let vector_index = pair / vectors.len();
            let claim_index = pair % vectors.len();
            let cross = evaluate_multilinear_with_scratch(
                &vectors[vector_index],
                &points[claim_index],
                scratch,
            );
            (
                vector_index + claim_index - 1,
                cross * diagonal[claim_index],
            )
        })
        .fold(
            || vec![Fr::from(0u64); coefficient_count],
            |mut local, (index, value)| {
                local[index] += value;
                local
            },
        )
        .reduce(
            || vec![Fr::from(0u64); coefficient_count],
            |mut left, right| {
                for (left, right) in left.iter_mut().zip(right) {
                    *left += right;
                }
                left
            },
        );
    std::hint::black_box(coefficients);
    start.elapsed()
}

type TimingKey = (usize, &'static str);

fn record_timing(
    totals: &mut BTreeMap<TimingKey, Vec<Duration>>,
    commitments: &mut BTreeMap<TimingKey, Vec<Duration>>,
    stages: &mut BTreeMap<TimingKey, BTreeMap<&'static str, Vec<Duration>>>,
    key: TimingKey,
    total: Duration,
    commitment: Duration,
    stage_values: BTreeMap<&'static str, Duration>,
) {
    totals.entry(key).or_default().push(total);
    commitments.entry(key).or_default().push(commitment);
    let destination = stages.entry(key).or_default();
    for (stage, duration) in stage_values {
        destination.entry(stage).or_default().push(duration);
    }
}

/// Run executable prover benchmarks at `2^log_length`. Every WHIR component
/// is generated and verifier-checked. The executable reference adapter uses
/// the selected three-variable fold, so the aligned Canopy height is four. Batched timings
/// additionally charge the explicitly measured distinct-functional cross terms.
pub fn run_prover_timings(
    output_dir: &Path,
    log_length: usize,
    warmups: usize,
    repetitions: usize,
    _samples_for_height: usize,
    seed: u64,
) -> Result<()> {
    run_prover_timings_with_regime(
        output_dir,
        log_length,
        warmups,
        repetitions,
        0,
        seed,
        WhirDecodingRegime::Johnson,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn run_prover_timings_with_regime(
    output_dir: &Path,
    log_length: usize,
    warmups: usize,
    repetitions: usize,
    _samples_for_height: usize,
    seed: u64,
    decoding_regime: WhirDecodingRegime,
) -> Result<()> {
    ensure!(repetitions > 0, "timing repetitions must be positive");
    ensure!(log_length > 10, "timing instance must require a WHIR fold");
    fs::create_dir_all(output_dir)?;
    let length = 1usize << log_length;
    let values: Vec<_> = (0..length as u64)
        .map(|index| crate::benchmark_field_element(seed, 0, index))
        .collect();

    let mut timing_schedule = Vec::new();
    let mut remaining = log_length;
    while remaining > PAPER_TERMINAL_MESSAGE_VARIABLES {
        timing_schedule.push(PAPER_TIMING_FOLDING_LOG);
        remaining -= PAPER_TIMING_FOLDING_LOG;
    }
    let outer_params = paper_parameters(
        log_length,
        OUTER_STARTING_LOG_INV_RATE,
        decoding_regime,
        remaining,
        &timing_schedule,
        CANOPY_HYBRID_CODEWORD_REDUCTION,
    )?;
    let outer_seed = expand_seed(derive_u64(seed, &[2, log_length, 0]));
    let first_query_leaves = sample_initial_oracle_queries(&outer_params, outer_seed);
    let mut totals = BTreeMap::<TimingKey, Vec<Duration>>::new();
    let mut commitments = BTreeMap::<TimingKey, Vec<Duration>>::new();
    let mut stages = BTreeMap::<TimingKey, BTreeMap<&'static str, Vec<Duration>>>::new();
    for repetition in 0..warmups + repetitions {
        eprintln!(
            "paper timing iteration {}/{}: common stages",
            repetition + 1,
            warmups + repetitions
        );
        let iteration = run_hybrid_iteration(&values, decoding_regime)?;
        for &input_oracles in PAPER_INPUT_ORACLE_COUNTS {
            eprintln!(
                "paper timing iteration {}: t={input_oracles}",
                repetition + 1
            );
            // Experiment 3's independent input codewords have identical
            // geometry, so schedule their NTTs concurrently on the shared
            // worker pool. The Canopy layer follows the same staged batch path.
            let input_refs = vec![values.as_slice(); input_oracles];
            let start = Instant::now();
            let experiment3_first_oracles = encode_many_interleaved_with_whir(
                &input_refs,
                OUTER_STARTING_LOG_INV_RATE,
                PAPER_TIMING_FOLDING_LOG,
            )?;
            let experiment3_outer_rs = start.elapsed();
            let (experiment3_commitments, experiment3_canopy_data, experiment3_profile) =
                CanopyVectorCommitment::commit_whir_profiled_batch(
                    &experiment3_first_oracles,
                    CANOPY_HYBRID_HEIGHT,
                    CANOPY_HYBRID_CODEWORD_REDUCTION,
                    PAPER_TIMING_FOLDING_LOG,
                )?;
            std::hint::black_box(
                experiment3_commitments
                    .iter()
                    .map(|commitment| commitment.root)
                    .collect::<Vec<_>>(),
            );
            let start = Instant::now();
            experiment3_canopy_data
                .par_iter()
                .zip(experiment3_first_oracles.par_iter())
                .try_for_each(|(canopy, first_oracle)| -> Result<()> {
                    let opening = CanopyVectorCommitment::open_whir_bottom(
                        canopy,
                        first_oracle,
                        &first_query_leaves,
                    )?;
                    std::hint::black_box(opening.auth_hashes.len());
                    Ok(())
                })?;
            let experiment3_canopy_open = start.elapsed();

            let ordinary_sources = vec![iteration.outer_second.clone(); input_oracles];
            let (ordinary_batch_tail, ordinary_cross_terms) = if input_oracles == 1 {
                (iteration.ordinary_tail, Duration::ZERO)
            } else {
                let cross_terms = generalized_sz_cross_term_work(&ordinary_sources);
                let tail = execute_whir(
                    &ordinary_sources,
                    PAPER_TIMING_FOLDING_LOG,
                    decoding_regime,
                    PAPER_TIMING_FOLDING_LOG,
                )?;
                let tail_proving_time = proving_time_until_paper_terminal(
                    tail.ood_time + tail.proving_time,
                    &ordinary_sources[0],
                    PAPER_TIMING_FOLDING_LOG,
                    decoding_regime,
                )?;
                (tail_proving_time, cross_terms)
            };
            let quarter_tail_log_inverse_rate =
                QUARTER_RATE_STARTING_LOG_INV_RATE + PAPER_TIMING_FOLDING_LOG - 1;
            let quarter_batch_tail = if input_oracles == 1 {
                iteration.quarter_tail
            } else {
                let tail = execute_whir(
                    &ordinary_sources,
                    quarter_tail_log_inverse_rate,
                    decoding_regime,
                    PAPER_TIMING_FOLDING_LOG,
                )?;
                proving_time_until_paper_terminal(
                    tail.ood_time + tail.proving_time,
                    &ordinary_sources[0],
                    quarter_tail_log_inverse_rate,
                    decoding_regime,
                )?
            };

            let mut hybrid_sources = Vec::with_capacity(2 * input_oracles);
            for canopy in &experiment3_canopy_data {
                hybrid_sources.push(iteration.outer_second.clone());
                hybrid_sources.push(canopy.canopy_message.clone());
            }
            let hybrid_cross_terms = generalized_sz_cross_term_work(&hybrid_sources);
            let hybrid_batch_tail = execute_whir(
                &hybrid_sources,
                PAPER_TIMING_FOLDING_LOG + 1 - CANOPY_HYBRID_CODEWORD_REDUCTION,
                decoding_regime,
                PAPER_TIMING_FOLDING_LOG,
            )?;
            let hybrid_batch_tail_proving_time = proving_time_until_paper_terminal(
                hybrid_batch_tail.ood_time + hybrid_batch_tail.proving_time,
                &hybrid_sources[0],
                PAPER_TIMING_FOLDING_LOG + 1 - CANOPY_HYBRID_CODEWORD_REDUCTION,
                decoding_regime,
            )?;

            if repetition < warmups {
                continue;
            }

            let ordinary_front = scale_duration(iteration.ordinary_first_transition, input_oracles);
            let batched_total = ordinary_front + ordinary_cross_terms + ordinary_batch_tail;
            let mut ordinary_stages = iteration
                .ordinary_first_transition_stages
                .iter()
                .map(|(&stage, &duration)| (stage, scale_duration(duration, input_oracles)))
                .collect::<BTreeMap<_, _>>();
            ordinary_stages.insert("generalized_sz_cross_terms", ordinary_cross_terms);
            if input_oracles == 1 {
                ordinary_stages.insert(
                    "recursive_tail_ood_evaluations",
                    iteration.ordinary_tail_ood,
                );
                ordinary_stages.insert(
                    "recursive_tail_after_ood",
                    ordinary_batch_tail.saturating_sub(iteration.ordinary_tail_ood),
                );
            } else {
                ordinary_stages.insert("batched_recursive_whir", ordinary_batch_tail);
            }
            record_timing(
                &mut totals,
                &mut commitments,
                &mut stages,
                (input_oracles, "merkle_whir_batched_after_second"),
                batched_total,
                scale_duration(iteration.initial_commitment, input_oracles),
                ordinary_stages,
            );

            let quarter_front = scale_duration(iteration.quarter_first_transition, input_oracles);
            let quarter_total = quarter_front + ordinary_cross_terms + quarter_batch_tail;
            let mut quarter_stages = iteration
                .quarter_first_transition_stages
                .iter()
                .map(|(&stage, &duration)| (stage, scale_duration(duration, input_oracles)))
                .collect::<BTreeMap<_, _>>();
            quarter_stages.insert("generalized_sz_cross_terms", ordinary_cross_terms);
            if input_oracles == 1 {
                quarter_stages.insert("recursive_tail_ood_evaluations", iteration.quarter_tail_ood);
                quarter_stages.insert(
                    "recursive_tail_after_ood",
                    quarter_batch_tail.saturating_sub(iteration.quarter_tail_ood),
                );
            } else {
                quarter_stages.insert("batched_recursive_whir", quarter_batch_tail);
            }
            record_timing(
                &mut totals,
                &mut commitments,
                &mut stages,
                (
                    input_oracles,
                    "merkle_whir_batched_rate_quarter_after_second",
                ),
                quarter_total,
                scale_duration(iteration.quarter_initial_commitment, input_oracles),
                quarter_stages,
            );

            let experiment3_outer_front =
                iteration.outer_ood + iteration.hybrid_outer_front_after_ood;
            let experiment3_commitment = experiment3_outer_rs + experiment3_profile.total;
            let experiment3_front = experiment3_commitment
                + experiment3_canopy_open
                + scale_duration(experiment3_outer_front, input_oracles);
            let experiment3_total =
                experiment3_front + hybrid_cross_terms + hybrid_batch_tail_proving_time;
            let mut experiment3_stages = BTreeMap::from([
                ("outer_rs_encoding", experiment3_outer_rs),
                (
                    "outer_ood_evaluations",
                    scale_duration(iteration.outer_ood, input_oracles),
                ),
                ("canopy_bottom_forest", experiment3_profile.bottom_forest),
                ("canopy_hash_to_field", experiment3_profile.hash_to_field),
                (
                    "canopy_rs_encoding",
                    experiment3_profile.reed_solomon_encoding,
                ),
                (
                    "canopy_codeword_merkle_tree",
                    experiment3_profile.codeword_merkle_tree,
                ),
                ("canopy_commitment_other", experiment3_profile.other()),
                ("canopy_bottom_opening", experiment3_canopy_open),
                (
                    "outer_initial_whir_stage_after_ood",
                    scale_duration(iteration.hybrid_outer_front_after_ood, input_oracles),
                ),
            ]);
            experiment3_stages.insert("generalized_sz_cross_terms", hybrid_cross_terms);
            experiment3_stages.insert("batched_recursive_whir", hybrid_batch_tail_proving_time);
            record_timing(
                &mut totals,
                &mut commitments,
                &mut stages,
                (input_oracles, "canopy_schwartz_zippel"),
                experiment3_total,
                experiment3_commitment,
                experiment3_stages,
            );
        }
    }

    let mut timing_rows = Vec::new();
    let mut all_stage_rows = Vec::new();
    for &input_oracles in PAPER_INPUT_ORACLE_COUNTS {
        let batched_mean = duration_stats(
            totals
                .get(&(input_oracles, "merkle_whir_batched_after_second"))
                .context("missing batched timing samples")?,
        )
        .0;
        for (experiment, scheme, starting_log_inverse_rate, height, note) in [
            (
                0,
                "merkle_whir_batched_after_second",
                OUTER_STARTING_LOG_INV_RATE,
                None,
                "independent first transitions; distinct-functional SZ batching from the second oracle onward",
            ),
            (
                0,
                "merkle_whir_batched_rate_quarter_after_second",
                QUARTER_RATE_STARTING_LOG_INV_RATE,
                None,
                "rate-1/4 independent first transitions; distinct-functional SZ batching from the second oracle onward",
            ),
            (
                3,
                "canopy_schwartz_zippel",
                OUTER_STARTING_LOG_INV_RATE,
                Some(CANOPY_HYBRID_HEIGHT),
                "t Canopy fronts; 2t aligned claims combined by one distinct-functional SZ identity",
            ),
        ] {
            let key = (input_oracles, scheme);
            let values = totals.get(&key).context("missing timing samples")?;
            timing_rows.push(timing_row(
                experiment,
                scheme,
                input_oracles,
                log_length,
                starting_log_inverse_rate,
                height,
                warmups,
                values,
                commitments
                    .get(&key)
                    .context("missing commitment timing samples")?,
                batched_mean,
                batched_mean,
                note,
            ));
            all_stage_rows.extend(stage_rows(
                experiment,
                scheme,
                input_oracles,
                log_length,
                values,
                stages.remove(&key).context("missing timing stages")?,
            ));
        }
    }

    let mut samples = csv::Writer::from_path(output_dir.join("prover_timing_samples.csv"))?;
    samples.write_record([
        "field",
        "input_oracles",
        "scheme",
        "repetition",
        "commitment_seconds",
        "opening_seconds",
        "prover_seconds",
    ])?;
    for (&(input_oracles, scheme), values) in &totals {
        for (repetition, &total) in values.iter().enumerate() {
            let commitment = commitments[&(input_oracles, scheme)][repetition];
            samples.write_record([
                crate::BENCHMARK_FIELD_LABEL.to_owned(),
                input_oracles.to_string(),
                scheme.to_owned(),
                (repetition + 1).to_string(),
                commitment.as_secs_f64().to_string(),
                total.saturating_sub(commitment).as_secs_f64().to_string(),
                total.as_secs_f64().to_string(),
            ])?;
        }
    }
    samples.flush()?;
    write_csv(output_dir.join("prover_timings.csv"), &timing_rows)?;
    write_csv(output_dir.join("prover_timing_stages.csv"), &all_stage_rows)?;
    write_timing_summary(
        output_dir.join("prover_timings.md"),
        &timing_rows,
        &all_stage_rows,
        decoding_regime,
    )?;
    Ok(())
}

fn write_timing_summary(
    path: impl AsRef<Path>,
    rows: &[PaperTimingRow],
    stages: &[PaperTimingStageRow],
    decoding_regime: WhirDecodingRegime,
) -> Result<()> {
    let mut file = File::create(path)?;
    writeln!(
        file,
        "# {} paper prover timings\n",
        if decoding_regime.is_conjectural() {
            "Conjectural capacity-bound"
        } else {
            "Final Johnson-bound"
        }
    )?;
    writeln!(
        file,
        "Field: **{}**. Inputs and evaluation points are deterministic full-width values.\n",
        crate::BENCHMARK_FIELD_LABEL
    )?;
    writeln!(file, "Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.\n")?;
    writeln!(
        file,
        "The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.\n"
    )?;
    writeln!(
        file,
        "| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |"
    )?;
    writeln!(
        file,
        "|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|"
    )?;
    for row in rows {
        writeln!(
            file,
            "| {} | {} | {} | 1/{} | {} | {} | {} | {:.4} | {:.4} | {:.4} | {:.4} | {:.3}x | {:.3}x |",
            row.experiment,
            row.input_oracles,
            row.scheme,
            1usize << (row.log_initial_codeword_length - row.log_input_length),
            row.log_initial_codeword_length,
            row.canopy_height
                .map_or_else(|| "-".to_owned(), |x| x.to_string()),
            row.repetitions,
            row.mean_commitment_seconds,
            row.mean_opening_seconds,
            row.mean_prover_seconds,
            row.median_prover_seconds,
            row.relative_to_independent_baseline,
            row.relative_to_batched_baseline,
        )?;
    }
    writeln!(file, "\n## Mean stage breakdown\n")?;
    writeln!(
        file,
        "| experiment | t | scheme | stage | mean s | median s | share |"
    )?;
    writeln!(file, "|---:|---:|---|---|---:|---:|---:|")?;
    for row in stages {
        writeln!(
            file,
            "| {} | {} | {} | {} | {:.4} | {:.4} | {:.1}% |",
            row.experiment,
            row.input_oracles,
            row.scheme,
            row.stage,
            row.mean_seconds,
            row.median_seconds,
            row.percent_of_mean_total,
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn local_budget_covers_the_composed_batched_grid() {
        for &n in PAPER_LOG_LENGTHS {
            for schedule in folding_schedules_up_to(n) {
                let fold = schedule[0];
                let terminal = terminal_message_variables(n, &schedule);
                for (rate, reduction, sources) in [(1, 1, 8), (2, 1, 8), (1, 0, 16)] {
                    let outer = paper_parameters(
                        n,
                        rate,
                        WhirDecodingRegime::Johnson,
                        terminal,
                        &schedule,
                        reduction,
                    )
                    .unwrap();
                    let tail = paper_parameters(
                        n - fold,
                        rate + fold - reduction,
                        WhirDecodingRegime::Johnson,
                        terminal,
                        &schedule[1..],
                        1,
                    )
                    .unwrap();
                    let mut events = repeated_rbr_events("outer", &outer, true, 8);
                    events.extend(tagged_rbr_events("tail", &tail, false));
                    events.push(TaggedRbrEvent {
                        phase: "batch".into(),
                        event: "SZ".into(),
                        security_bits: sz_batching_security_bits(&tail, sources),
                    });
                    assert!(
                        union_bound_security(&events) >= DEFAULT_SECURITY_BITS as f64,
                        "n={n}, rate={rate}, reduction={reduction}, schedule={schedule:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn aligned_hybrid_codes_match() {
        for n in PAPER_LOG_LENGTHS {
            for initial_folding_log in [1usize, 4, 7] {
                let height = aligned_canopy_height(initial_folding_log);
                let canopy_message_log = n + 1 - height;
                let canopy_codeword_log = n + 1 - CANOPY_HYBRID_CODEWORD_REDUCTION;
                assert_eq!(canopy_message_log, n - initial_folding_log);
                assert_eq!(canopy_codeword_log, n + 1);
                assert_eq!(
                    canopy_codeword_log - canopy_message_log,
                    initial_folding_log + 1
                );

                let outer = WhirParameters::with_regime_and_terminal_and_initial_transition(
                    *n,
                    OUTER_STARTING_LOG_INV_RATE,
                    WhirDecodingRegime::Capacity,
                    8,
                    initial_folding_log,
                    CANOPY_HYBRID_CODEWORD_REDUCTION,
                )
                .unwrap();
                let aligned = WhirParameters::with_regime_and_terminal(
                    n - initial_folding_log,
                    initial_folding_log + 1,
                    WhirDecodingRegime::Capacity,
                    8,
                )
                .unwrap();
                assert_eq!(query_geometry(&outer)[1], query_geometry(&aligned)[0]);
            }
        }
    }

    #[test]
    fn quarter_rate_second_oracle_matches_batched_tail() {
        for n in PAPER_LOG_LENGTHS {
            for initial_folding_log in PAPER_FOLDING_CHOICES {
                let outer = WhirParameters::with_regime_and_terminal_and_initial_transition(
                    *n,
                    QUARTER_RATE_STARTING_LOG_INV_RATE,
                    WhirDecodingRegime::Johnson,
                    PAPER_TERMINAL_MESSAGE_VARIABLES,
                    *initial_folding_log,
                    1,
                )
                .unwrap();
                let tail = WhirParameters::with_regime_and_terminal(
                    n - initial_folding_log,
                    QUARTER_RATE_STARTING_LOG_INV_RATE + initial_folding_log - 1,
                    WhirDecodingRegime::Johnson,
                    PAPER_TERMINAL_MESSAGE_VARIABLES,
                )
                .unwrap();
                assert_eq!(query_geometry(&outer)[1], query_geometry(&tail)[0]);
            }
        }
    }

    #[test]
    fn rbr_selection_falls_back_without_panicking() {
        let candidate = |security_bits: f64, bytes: usize| ProofCandidate {
            input_oracles: 4,
            batched_source_oracles: 8,
            starting_log_inverse_rate: OUTER_STARTING_LOG_INV_RATE,
            initial_folding_log: 3,
            canopy_height: Some(4),
            outer_folding_schedule: vec![3],
            canopy_folding_schedule: Some(Vec::new()),
            recursive_field_bits: 256,
            minimum_rbr_security_bits: security_bits,
            algebraic_rbr_union_bound_bits: security_bits,
            rbr_events: Vec::new(),
            totals: Totals {
                field_bytes: bytes,
                ..Totals::default()
            },
            queries: BTreeMap::new(),
        };
        let candidates = [candidate(120.0, 10), candidate(125.0, 20)];
        assert_eq!(
            best_rbr_candidate(&candidates).minimum_rbr_security_bits,
            125.0
        );
    }

    #[test]
    fn small_proof_report_writes_expected_files() {
        // Exercise the report helpers without running the paper-size grid.
        let temp = tempdir().unwrap();
        write_csv::<HybridProofSizeRow>(temp.path().join("empty.csv"), &[]).unwrap();
        assert!(temp.path().join("empty.csv").exists());
    }
}
