//! Final paper experiments and report generation.

use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Write,
    path::Path,
    time::{Duration, Instant},
};

use anyhow::{ensure, Context, Result};
use serde::Serialize;

use crate::{
    benchmark::{
        run_size_matrix_with_regime, run_size_matrix_with_regime_and_terminal,
        sample_unique_indices, BestHeightRow, SizeMatrixConfig,
    },
    canopy::CanopyVectorCommitment,
    executable_whir::execute_with_regime as execute_whir,
    merkle::{frontier_hash_count, MerkleVectorCommitment},
    reed_solomon,
    whir::{
        estimate_proof, estimate_schwartz_zippel_batched_initial_proofs, expand_seed,
        sample_initial_oracle_queries, WhirDecodingRegime, WhirParameters, WhirProofSize,
        DEFAULT_FOLDING_LOG, DEFAULT_SECURITY_BITS, MAX_TERMINAL_MESSAGE_VARIABLES,
    },
    Fr, FIELD_BITS, FIELD_BYTES, HASH_BYTES,
};

pub const PAPER_LOG_LENGTHS: &[usize] = &[22, 24, 26, 28, 30];
pub const PAPER_BATCH_SIZES: &[usize] = &[128, 256, 512, 1024];
pub const CANOPY_HYBRID_CODEWORD_REDUCTION: usize = 0;
pub const PAPER_FIXED_FOLDING_LOG: usize = 3;
/// Legacy four-variable-fold alignment used by the executable timing path.
pub const CANOPY_HYBRID_HEIGHT: usize = DEFAULT_FOLDING_LOG + 1;
pub const OUTER_STARTING_LOG_INV_RATE: usize = 1;

fn aligned_canopy_height(initial_folding_log: usize) -> usize {
    initial_folding_log + 1
}

fn full_height_grid(max_log_length: usize, folding_log: usize) -> Vec<usize> {
    (1..=max_log_length - folding_log).collect()
}

#[derive(Clone, Debug, Serialize)]
pub struct HybridProofSizeRow {
    pub experiment: usize,
    pub scheme: String,
    pub log_message_length: usize,
    pub message_length: u64,
    pub outer_starting_rate: String,
    pub outer_initial_folding_log: usize,
    pub canopy_height: Option<usize>,
    pub canopy_codeword_log_reduction: Option<usize>,
    pub aligned_message_log: Option<usize>,
    pub aligned_codeword_log: Option<usize>,
    pub aligned_log_inverse_rate: Option<usize>,
    pub samples: usize,
    pub mean_root_hashes: f64,
    pub mean_auth_hashes: f64,
    pub mean_hash_count: f64,
    pub mean_hash_bytes: f64,
    pub mean_ood_fields: f64,
    pub mean_sumcheck_fields: f64,
    pub mean_batching_fields: f64,
    pub mean_query_answer_fields: f64,
    pub mean_final_coefficient_fields: f64,
    pub mean_field_count: f64,
    pub mean_field_bytes: f64,
    pub mean_total_bytes: f64,
    pub mean_total_kib: f64,
    pub relative_to_merkle_whir: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct OracleQueryBreakdownRow {
    pub experiment: usize,
    pub scheme: String,
    pub log_message_length: usize,
    pub oracle_role: String,
    pub commitment_kind: String,
    pub oracle_count: usize,
    pub shared_query_positions: bool,
    pub shared_authentication: bool,
    pub message_log: usize,
    pub codeword_log: usize,
    pub log_inverse_rate: usize,
    pub leaf_width: usize,
    pub leaf_count: u64,
    pub requested_queries: usize,
    pub samples: usize,
    pub mean_unique_queries: f64,
    pub mean_auth_hashes: f64,
    pub mean_answer_fields: f64,
}

#[derive(Clone, Default)]
struct Totals {
    hashes: usize,
    fields: usize,
    root_hashes: usize,
    auth_hashes: usize,
    ood_fields: usize,
    sumcheck_fields: usize,
    batching_fields: usize,
    query_answer_fields: usize,
    final_coefficient_fields: usize,
}

impl Totals {
    fn add_whir_with_existing_root(&mut self, proof: &WhirProofSize) {
        self.hashes += proof.hash_count;
        self.fields += proof.field_count;
        self.root_hashes += proof.intermediate_roots;
        self.auth_hashes += proof.merkle_auth_hashes;
        self.ood_fields += proof.ood_fields;
        self.sumcheck_fields += proof.sumcheck_fields;
        self.batching_fields += proof.batching_fields;
        self.query_answer_fields += proof.query_answer_fields;
        self.final_coefficient_fields += proof.final_coefficient_fields;
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
        self.ood_fields += outer.commitment_ood_samples;
        self.sumcheck_fields += 3 * outer.folding_log * outer.initial_sumcheck_repetitions;
        self.query_answer_fields += first_fields;
    }

    fn bytes(&self) -> usize {
        self.hashes * HASH_BYTES + self.fields * FIELD_BYTES
    }
}

#[derive(Clone)]
struct QueryDescriptor {
    experiment: usize,
    scheme: &'static str,
    log_message_length: usize,
    role: String,
    commitment: &'static str,
    oracle_count: usize,
    shared_positions: bool,
    shared_authentication: bool,
    message_log: usize,
    codeword_log: usize,
    log_inverse_rate: usize,
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
    initial_folding_log: usize,
    canopy_height: Option<usize>,
    totals: Totals,
    queries: BTreeMap<String, QueryAccumulator>,
}

fn best_candidate(candidates: &[ProofCandidate]) -> &ProofCandidate {
    candidates
        .iter()
        .min_by_key(|candidate| {
            (
                candidate.totals.bytes(),
                candidate.initial_folding_log,
                candidate.canopy_height,
            )
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

    fn row(self) -> OracleQueryBreakdownRow {
        let descriptor = self.descriptor.expect("query descriptor");
        let samples = self.samples as f64;
        OracleQueryBreakdownRow {
            experiment: descriptor.experiment,
            scheme: descriptor.scheme.to_owned(),
            log_message_length: descriptor.log_message_length,
            oracle_role: descriptor.role,
            commitment_kind: descriptor.commitment.to_owned(),
            oracle_count: descriptor.oracle_count,
            shared_query_positions: descriptor.shared_positions,
            shared_authentication: descriptor.shared_authentication,
            message_log: descriptor.message_log,
            codeword_log: descriptor.codeword_log,
            log_inverse_rate: descriptor.log_inverse_rate,
            leaf_width: descriptor.leaf_width,
            leaf_count: descriptor.leaf_count as u64,
            requested_queries: descriptor.requested_queries,
            samples: self.samples,
            mean_unique_queries: self.unique_queries as f64 / samples,
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
    log_length: usize,
    role_prefix: &str,
    commitment: &'static str,
    params: &WhirParameters,
    proof: &WhirProofSize,
    initial_oracle_count: usize,
    recursive_oracle_count: usize,
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
        let key = format!("{experiment}/{scheme}/{log_length}/{role}");
        accumulators.entry(key).or_default().push(
            QueryDescriptor {
                experiment,
                scheme,
                log_message_length: log_length,
                role,
                commitment,
                oracle_count,
                shared_positions: oracle_count > 1,
                shared_authentication: oracle_count > 1 && !(separate_initial_roots && oracle == 0),
                message_log,
                codeword_log,
                log_inverse_rate: rate,
                leaf_width: width,
                leaf_count: leaves,
                requested_queries: requested,
            },
            proof.unique_queries_by_oracle[oracle],
            proof.auth_hashes_by_oracle[oracle],
            proof.unique_queries_by_oracle[oracle] * width * oracle_count,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn proof_row(
    experiment: usize,
    scheme: &str,
    log_length: usize,
    samples: usize,
    totals: &Totals,
    baseline_bytes: f64,
    initial_folding_log: usize,
    canopy_height: Option<usize>,
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
    let mean_hashes = totals.hashes as f64 / samples as f64;
    let mean_fields = totals.fields as f64 / samples as f64;
    let mean_bytes = mean_hashes * HASH_BYTES as f64 + mean_fields * FIELD_BYTES as f64;
    HybridProofSizeRow {
        experiment,
        scheme: scheme.to_owned(),
        log_message_length: log_length,
        message_length: 1u64 << log_length,
        outer_starting_rate: "1/2".to_owned(),
        outer_initial_folding_log: initial_folding_log,
        canopy_height,
        canopy_codeword_log_reduction: canopy_height.map(|_| CANOPY_HYBRID_CODEWORD_REDUCTION),
        aligned_message_log: canopy_height.map(|_| log_length - initial_folding_log),
        aligned_codeword_log: canopy_height
            .map(|_| log_length + 1 - CANOPY_HYBRID_CODEWORD_REDUCTION),
        aligned_log_inverse_rate: canopy_height
            .map(|_| initial_folding_log + 1 - CANOPY_HYBRID_CODEWORD_REDUCTION),
        samples,
        mean_root_hashes: totals.root_hashes as f64 / samples as f64,
        mean_auth_hashes: totals.auth_hashes as f64 / samples as f64,
        mean_hash_count: mean_hashes,
        mean_hash_bytes: mean_hashes * HASH_BYTES as f64,
        mean_ood_fields: totals.ood_fields as f64 / samples as f64,
        mean_sumcheck_fields: totals.sumcheck_fields as f64 / samples as f64,
        mean_batching_fields: totals.batching_fields as f64 / samples as f64,
        mean_query_answer_fields: totals.query_answer_fields as f64 / samples as f64,
        mean_final_coefficient_fields: totals.final_coefficient_fields as f64 / samples as f64,
        mean_field_count: mean_fields,
        mean_field_bytes: mean_fields * FIELD_BYTES as f64,
        mean_total_bytes: mean_bytes,
        mean_total_kib: mean_bytes / 1024.0,
        relative_to_merkle_whir: mean_bytes / baseline_bytes,
    }
}

/// Generate experiment 1 and the proof-size/query-breakdown grids for
/// experiments 2 and 3.
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
        MAX_TERMINAL_MESSAGE_VARIABLES,
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
        FIELD_BITS == 256,
        "paper experiments require the 256-bit field"
    );
    fs::create_dir_all(output_dir)?;

    let experiment1_dir = output_dir.join("experiment1_canopy_vs_merkle");
    let experiment1_rows = run_size_matrix_with_regime_and_terminal(
        &SizeMatrixConfig {
            log_lengths: PAPER_LOG_LENGTHS.to_vec(),
            batch_sizes: PAPER_BATCH_SIZES.to_vec(),
            heights: full_height_grid(
                *PAPER_LOG_LENGTHS.iter().max().unwrap(),
                PAPER_FIXED_FOLDING_LOG,
            ),
            layer_field_bits: vec![256],
            codeword_log_reductions: vec![0],
            folding_log: PAPER_FIXED_FOLDING_LOG,
            samples,
            seed,
        },
        &experiment1_dir,
        decoding_regime,
        max_terminal_message_variables,
    )?;

    let mut proof_rows = Vec::new();
    let mut folding_grid_rows = Vec::new();
    let mut query_accumulators = BTreeMap::<String, QueryAccumulator>::new();
    for &n in PAPER_LOG_LENGTHS {
        let initial_folding_log = PAPER_FIXED_FOLDING_LOG;
        ensure!(
            n - initial_folding_log > max_terminal_message_variables,
            "fixed first fold must leave a committed second oracle"
        );

        let mut baseline_candidates = Vec::with_capacity(1);
        let mut separate_candidates = Vec::new();
        let mut schwartz_zippel_candidates = Vec::with_capacity(1);

        let baseline_outer = WhirParameters::with_regime_and_terminal_and_folding(
            n,
            OUTER_STARTING_LOG_INV_RATE,
            decoding_regime,
            max_terminal_message_variables,
            PAPER_FIXED_FOLDING_LOG,
        )?;
        let mut baseline = Totals::default();
        let mut baseline_queries = BTreeMap::new();
        for sample in 0..samples {
            let outer_seed = expand_seed(derive_u64(seed, &[2, n, sample, 0]));
            let outer_proof = estimate_proof(&baseline_outer, outer_seed)?;
            baseline.add_whir_with_initial_root(&outer_proof);
            accumulate_whir_queries(
                &mut baseline_queries,
                2,
                "merkle_whir",
                n,
                "outer WHIR",
                "Merkle",
                &baseline_outer,
                &outer_proof,
                1,
                1,
                false,
            );
        }
        baseline_candidates.push(ProofCandidate {
            initial_folding_log,
            canopy_height: None,
            totals: baseline,
            queries: baseline_queries,
        });

        // The hybrid retains the initial codeword length after its first fold.
        let outer =
            WhirParameters::with_regime_and_terminal_and_initial_transition_and_recursive_folding(
                n,
                OUTER_STARTING_LOG_INV_RATE,
                decoding_regime,
                max_terminal_message_variables,
                initial_folding_log,
                CANOPY_HYBRID_CODEWORD_REDUCTION,
                PAPER_FIXED_FOLDING_LOG,
            )?;
        let outer_tail_params = WhirParameters::with_regime_and_terminal_and_folding(
            n - initial_folding_log,
            initial_folding_log + 1,
            decoding_regime,
            max_terminal_message_variables,
            PAPER_FIXED_FOLDING_LOG,
        )?;
        debug_assert_eq!(
            query_geometry(&outer)[1],
            query_geometry(&outer_tail_params)[0]
        );
        let first_geometry = query_geometry(&outer)[0];
        let first_width = 1usize << initial_folding_log;
        debug_assert_eq!(first_geometry.3, first_width);

        // Experiment 2 proves the two claims independently, so h is free.
        for canopy_height in 1..=n + 1 - PAPER_FIXED_FOLDING_LOG {
            let canopy_tail_params = WhirParameters::with_regime_and_terminal_and_folding(
                n + 1 - canopy_height,
                canopy_height,
                decoding_regime,
                max_terminal_message_variables,
                PAPER_FIXED_FOLDING_LOG,
            )?;
            let mut separate = Totals::default();
            let mut separate_queries = BTreeMap::<String, QueryAccumulator>::new();
            for sample in 0..samples {
                let outer_seed = expand_seed(derive_u64(seed, &[2, n, sample, 0]));
                let first_oracle_queries = sample_initial_oracle_queries(&outer, outer_seed);
                let bottom_hashes = if canopy_height >= initial_folding_log {
                    frontier_hash_count(
                        &first_oracle_queries,
                        first_geometry.4,
                        canopy_height - initial_folding_log,
                    )?
                } else {
                    // A width-2^f0 query contains complete height-h subtrees.
                    0
                };
                let first_fields = first_oracle_queries.len() * first_width;
                let canopy_tail_seed = expand_seed(derive_u64(
                    seed,
                    &[2, n, canopy_height, sample, 0x4341_4e4f_5059],
                ));
                let outer_tail_seed =
                    expand_seed(derive_u64(seed, &[2, n, sample, 0x004f_5554_4552]));
                let canopy_tail = estimate_proof(&canopy_tail_params, canopy_tail_seed)?;
                let outer_tail = estimate_proof(&outer_tail_params, outer_tail_seed)?;

                separate.add_hybrid_front(&outer, bottom_hashes, first_fields);
                separate.add_whir_with_existing_root(&canopy_tail);
                separate.add_whir_with_existing_root(&outer_tail);

                let first_key = format!("2/canopy_separate/{n}/outer first oracle via Canopy");
                separate_queries.entry(first_key).or_default().push(
                    QueryDescriptor {
                        experiment: 2,
                        scheme: "canopy_separate",
                        log_message_length: n,
                        role: "outer first oracle via Canopy".to_owned(),
                        commitment: "Canopy bottom forest",
                        oracle_count: 1,
                        shared_positions: false,
                        shared_authentication: false,
                        message_log: n,
                        codeword_log: n + 1,
                        log_inverse_rate: 1,
                        leaf_width: first_width,
                        leaf_count: first_geometry.4,
                        requested_queries: first_geometry.5,
                    },
                    first_oracle_queries.len(),
                    bottom_hashes,
                    first_fields,
                );
                accumulate_whir_queries(
                    &mut separate_queries,
                    2,
                    "canopy_separate",
                    n,
                    "Canopy claim WHIR",
                    "Merkle",
                    &canopy_tail_params,
                    &canopy_tail,
                    1,
                    1,
                    false,
                );
                accumulate_whir_queries(
                    &mut separate_queries,
                    2,
                    "canopy_separate",
                    n,
                    "outer second-oracle WHIR",
                    "Merkle",
                    &outer_tail_params,
                    &outer_tail,
                    1,
                    1,
                    false,
                );
            }
            separate_candidates.push(ProofCandidate {
                initial_folding_log,
                canopy_height: Some(canopy_height),
                totals: separate,
                queries: separate_queries,
            });
        }

        // Experiment 3 needs identical RS codes, fixing h=f0+1.
        let canopy_height = aligned_canopy_height(initial_folding_log);
        let mut schwartz_zippel = Totals::default();
        let mut schwartz_zippel_queries = BTreeMap::<String, QueryAccumulator>::new();
        for sample in 0..samples {
            let outer_seed = expand_seed(derive_u64(seed, &[2, n, sample, 0]));
            let first_oracle_queries = sample_initial_oracle_queries(&outer, outer_seed);
            let bottom_hashes = frontier_hash_count(
                &first_oracle_queries,
                first_geometry.4,
                canopy_height - initial_folding_log,
            )?;
            let first_fields = first_oracle_queries.len() * first_width;
            let schwartz_zippel_seed =
                expand_seed(derive_u64(seed, &[3, n, sample, 0x0053_5a42_4154]));
            let schwartz_zippel_tail = estimate_schwartz_zippel_batched_initial_proofs(
                &outer_tail_params,
                schwartz_zippel_seed,
            )?;
            schwartz_zippel.add_hybrid_front(&outer, bottom_hashes, first_fields);
            schwartz_zippel.add_whir_with_existing_root(&schwartz_zippel_tail);

            let first_key = format!("3/canopy_schwartz_zippel/{n}/outer first oracle via Canopy");
            schwartz_zippel_queries.entry(first_key).or_default().push(
                QueryDescriptor {
                    experiment: 3,
                    scheme: "canopy_schwartz_zippel",
                    log_message_length: n,
                    role: "outer first oracle via Canopy".to_owned(),
                    commitment: "Canopy bottom forest",
                    oracle_count: 1,
                    shared_positions: false,
                    shared_authentication: false,
                    message_log: n,
                    codeword_log: n + 1,
                    log_inverse_rate: 1,
                    leaf_width: first_width,
                    leaf_count: first_geometry.4,
                    requested_queries: first_geometry.5,
                },
                first_oracle_queries.len(),
                bottom_hashes,
                first_fields,
            );
            accumulate_whir_queries(
                &mut schwartz_zippel_queries,
                3,
                "canopy_schwartz_zippel",
                n,
                "Schwartz-Zippel combined aligned claim",
                "two initial Merkle roots; one combined recursive oracle",
                &outer_tail_params,
                &schwartz_zippel_tail,
                2,
                1,
                true,
            );
        }
        schwartz_zippel_candidates.push(ProofCandidate {
            initial_folding_log,
            canopy_height: Some(canopy_height),
            totals: schwartz_zippel,
            queries: schwartz_zippel_queries,
        });

        let baseline = best_candidate(&baseline_candidates);
        let separate = best_candidate(&separate_candidates);
        let schwartz_zippel = best_candidate(&schwartz_zippel_candidates);
        let baseline_bytes = baseline.totals.bytes() as f64 / samples as f64;

        for candidate in &baseline_candidates {
            folding_grid_rows.push(proof_row(
                2,
                "merkle_whir",
                n,
                samples,
                &candidate.totals,
                baseline_bytes,
                candidate.initial_folding_log,
                candidate.canopy_height,
            ));
        }
        for (experiment, scheme, candidates) in [
            (2, "canopy_separate", &separate_candidates),
            (3, "canopy_schwartz_zippel", &schwartz_zippel_candidates),
        ] {
            for candidate in candidates {
                folding_grid_rows.push(proof_row(
                    experiment,
                    scheme,
                    n,
                    samples,
                    &candidate.totals,
                    baseline_bytes,
                    candidate.initial_folding_log,
                    candidate.canopy_height,
                ));
            }
        }

        proof_rows.push(proof_row(
            2,
            "merkle_whir",
            n,
            samples,
            &baseline.totals,
            baseline_bytes,
            baseline.initial_folding_log,
            baseline.canopy_height,
        ));
        for (experiment, scheme, candidate) in [
            (2, "canopy_separate", separate),
            (3, "canopy_schwartz_zippel", schwartz_zippel),
        ] {
            proof_rows.push(proof_row(
                experiment,
                scheme,
                n,
                samples,
                &candidate.totals,
                baseline_bytes,
                candidate.initial_folding_log,
                candidate.canopy_height,
            ));
        }

        for candidate in [baseline, separate, schwartz_zippel] {
            query_accumulators.extend(candidate.queries.clone());
        }
    }

    let query_rows: Vec<_> = query_accumulators
        .into_values()
        .map(QueryAccumulator::row)
        .collect();
    let experiment3_proof_rows: Vec<_> = proof_rows
        .iter()
        .filter(|row| row.scheme == "merkle_whir" || row.scheme == "canopy_schwartz_zippel")
        .cloned()
        .collect();
    let experiment3_query_rows: Vec<_> = query_rows
        .iter()
        .filter(|row| row.scheme == "merkle_whir" || row.scheme == "canopy_schwartz_zippel")
        .cloned()
        .collect();
    write_csv(output_dir.join("hybrid_proof_sizes.csv"), &proof_rows)?;
    write_csv(
        output_dir.join("hybrid_parameter_grid.csv"),
        &folding_grid_rows,
    )?;
    write_csv(output_dir.join("oracle_query_breakdown.csv"), &query_rows)?;
    write_csv(
        output_dir.join("experiment3_vs_merkle_proof_sizes.csv"),
        &experiment3_proof_rows,
    )?;
    write_csv(
        output_dir.join("experiment3_vs_merkle_oracle_queries.csv"),
        &experiment3_query_rows,
    )?;
    write_summary(
        output_dir.join("summary.md"),
        samples,
        seed,
        decoding_regime,
        max_terminal_message_variables,
        &experiment1_rows,
        &proof_rows,
        &query_rows,
    )?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn write_summary(
    path: impl AsRef<Path>,
    samples: usize,
    seed: u64,
    decoding_regime: WhirDecodingRegime,
    max_terminal_message_variables: usize,
    experiment1_rows: &[BestHeightRow],
    proof_rows: &[HybridProofSizeRow],
    query_rows: &[OracleQueryBreakdownRow],
) -> Result<()> {
    let mut file = File::create(path)?;
    writeln!(
        file,
        "# {} paper proof-size experiments\n",
        if decoding_regime.is_conjectural() {
            "Conjectural capacity-bound"
        } else {
            "Final Johnson-bound"
        }
    )?;
    writeln!(
        file,
        "All experiments target {}-bit soundness over the {}-bit prime field, use BLAKE3-256, use no proof of work, use terminal polynomials of at most `2^{}`, use three-variable folds everywhere, and use the {} RS proximity regime.{} Means use {} deterministic query samples from seed `{:#x}`.\n",
        DEFAULT_SECURITY_BITS,
        FIELD_BITS,
        max_terminal_message_variables,
        decoding_regime.label(),
        if decoding_regime.is_conjectural() {
            " Capacity-bound soundness relies on STIR Conjecture 5.6 and is not proved."
        } else {
            ""
        },
        samples,
        seed
    )?;
    writeln!(file, "## Experiment 1: one Canopy commitment vs Merkle\n")?;
    writeln!(
        file,
        "The table reports the proof-size-optimal Canopy height over the full feasible grid with three-variable WHIR folds. Detailed rows and proof components are in `experiment1_canopy_vs_merkle/`.\n"
    )?;
    writeln!(
        file,
        "| log2 vector | batch | best h | Merkle KiB | Canopy KiB | Canopy / Merkle |"
    )?;
    writeln!(file, "|---:|---:|---:|---:|---:|---:|")?;
    for row in experiment1_rows {
        writeln!(
            file,
            "| {} | {} | {} | {:.2} | {:.2} | {:.3}x |",
            row.log_length,
            row.batch_size,
            row.best_h,
            row.merkle_mean_bytes / 1024.0,
            row.canopy_mean_bytes / 1024.0,
            row.canopy_over_merkle,
        )?;
    }
    writeln!(file, "\n## Experiments 2 and 3: WHIR hybrids\n")?;
    writeln!(
        file,
        "Every initial and recursive WHIR fold uses three variables, so `f0=3`. The hybrids use no Canopy codeword shortening. Experiment 2 proves the claims separately and searches Canopy height `h` independently over its full feasible range. Experiment 3 needs identical RS codes and therefore fixes `h=f0+1=4`; both aligned claims encode `2^(n-3)` coefficients into `2^(n+1)` symbols at inverse rate `2^4`. It sends one pre-challenge cross-term sum, samples `alpha`, and proves the single degree-two identity for `f+alpha*g` under `L_f+alpha*L_g`. Its additional batching error is at most `2/|F|`, approximately `2^-255`. The complete fixed-fold height search is in `hybrid_parameter_grid.csv`.\n"
    )?;
    writeln!(file, "## WHIR and hybrid proof sizes\n")?;
    writeln!(
        file,
        "| log2 message | scheme | f0 | h | KiB | vs Merkle WHIR | hashes KiB | fields KiB |"
    )?;
    writeln!(file, "|---:|---|---:|---:|---:|---:|---:|---:|")?;
    for row in proof_rows {
        writeln!(
            file,
            "| {} | {} | {} | {} | {:.2} | {:.3}x | {:.2} | {:.2} |",
            row.log_message_length,
            row.scheme,
            row.outer_initial_folding_log,
            row.canopy_height
                .map_or_else(|| "-".to_owned(), |height| height.to_string()),
            row.mean_total_kib,
            row.relative_to_merkle_whir,
            row.mean_hash_bytes / 1024.0,
            row.mean_field_bytes / 1024.0,
        )?;
    }
    writeln!(file, "\n## Experiment 3 vs Merkle WHIR components\n")?;
    writeln!(file, "All entries below are KiB. Roots and authentication paths are hashes; the remaining columns are 256-bit field elements.\n")?;
    writeln!(file, "| log2 message | scheme | roots | authentication | OOD | sumcheck | batching | query answers | terminal coefficients | total |")?;
    writeln!(file, "|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|")?;
    for row in proof_rows
        .iter()
        .filter(|row| row.scheme == "merkle_whir" || row.scheme == "canopy_schwartz_zippel")
    {
        writeln!(
            file,
            "| {} | {} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} |",
            row.log_message_length,
            row.scheme,
            row.mean_root_hashes * HASH_BYTES as f64 / 1024.0,
            row.mean_auth_hashes * HASH_BYTES as f64 / 1024.0,
            row.mean_ood_fields * FIELD_BYTES as f64 / 1024.0,
            row.mean_sumcheck_fields * FIELD_BYTES as f64 / 1024.0,
            row.mean_batching_fields * FIELD_BYTES as f64 / 1024.0,
            row.mean_query_answer_fields * FIELD_BYTES as f64 / 1024.0,
            row.mean_final_coefficient_fields * FIELD_BYTES as f64 / 1024.0,
            row.mean_total_kib,
        )?;
    }
    writeln!(file, "\n## Oracle-query breakdown\n")?;
    writeln!(file, "The machine-readable table is `oracle_query_breakdown.csv`. Values below are the largest proof-size point (`n=30`).\n")?;
    writeln!(file, "| experiment | scheme | oracle | code | requested | unique | auth hashes | answer fields |")?;
    writeln!(file, "|---:|---|---|---|---:|---:|---:|---:|")?;
    for row in query_rows.iter().filter(|row| row.log_message_length == 30) {
        writeln!(
            file,
            "| {} | {} | {} | [2^{}, 2^{}] | {} | {:.2} | {:.2} | {:.2} |",
            row.experiment,
            row.scheme,
            row.oracle_role,
            row.codeword_log,
            row.message_log,
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
    pub log_length: usize,
    pub batch_size: Option<usize>,
    pub canopy_height: Option<usize>,
    pub warmups: usize,
    pub repetitions: usize,
    pub mean_prover_seconds: f64,
    pub median_prover_seconds: f64,
    pub min_prover_seconds: f64,
    pub max_prover_seconds: f64,
    pub relative_to_baseline: f64,
    pub note: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct PaperTimingStageRow {
    pub experiment: usize,
    pub scheme: String,
    pub log_length: usize,
    pub batch_size: Option<usize>,
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
    log_length: usize,
    batch_size: Option<usize>,
    height: Option<usize>,
    warmups: usize,
    values: &[Duration],
    baseline_mean: f64,
    note: &str,
) -> PaperTimingRow {
    let (mean, median, min, max) = duration_stats(values);
    PaperTimingRow {
        experiment,
        scheme: scheme.to_owned(),
        log_length,
        batch_size,
        canopy_height: height,
        warmups,
        repetitions: values.len(),
        mean_prover_seconds: mean,
        median_prover_seconds: median,
        min_prover_seconds: min,
        max_prover_seconds: max,
        relative_to_baseline: mean / baseline_mean,
        note: note.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn stage_rows(
    experiment: usize,
    scheme: &str,
    log_length: usize,
    batch_size: Option<usize>,
    total_mean: f64,
    stages: BTreeMap<&'static str, Vec<Duration>>,
) -> Vec<PaperTimingStageRow> {
    stages
        .into_iter()
        .map(|(stage, values)| {
            let (mean, median, _, _) = duration_stats(&values);
            PaperTimingStageRow {
                experiment,
                scheme: scheme.to_owned(),
                log_length,
                batch_size,
                stage: stage.to_owned(),
                repetitions: values.len(),
                mean_seconds: mean,
                median_seconds: median,
                percent_of_mean_total: 100.0 * mean / total_mean,
            }
        })
        .collect()
}

fn evaluate_ood_work(coefficients: &[Fr], count: usize) {
    let point = Fr::from(11u64);
    for offset in 0..count {
        let shifted = point + Fr::from(offset as u64);
        let value = coefficients
            .iter()
            .rev()
            .fold(Fr::from(0u64), |acc, coefficient| {
                acc * shifted + coefficient
            });
        std::hint::black_box(value);
    }
}

struct HybridIteration {
    baseline: Duration,
    separate: Duration,
    baseline_stages: BTreeMap<&'static str, Duration>,
    separate_stages: BTreeMap<&'static str, Duration>,
}

fn run_hybrid_iteration(
    message: &[Fr],
    first_oracle_value_indices: &[usize],
    outer_ood_samples: usize,
    decoding_regime: WhirDecodingRegime,
) -> Result<HybridIteration> {
    let outer = execute_whir(
        &[message.to_vec()],
        OUTER_STARTING_LOG_INV_RATE,
        decoding_regime,
    )?;
    let baseline = outer.commitment_time + outer.proving_time;
    let baseline_stages = BTreeMap::from([
        ("initial_commitment", outer.commitment_time),
        ("whir_opening_proof", outer.proving_time),
    ]);

    let start = Instant::now();
    let first_oracle = reed_solomon::encode(message, message.len() * 2)?;
    let outer_rs = start.elapsed();

    let start = Instant::now();
    evaluate_ood_work(message, outer_ood_samples);
    let outer_ood = start.elapsed();

    let (_, canopy_data, canopy_profile) = CanopyVectorCommitment::commit_profiled(
        &first_oracle,
        CANOPY_HYBRID_HEIGHT,
        CANOPY_HYBRID_CODEWORD_REDUCTION,
    )?;
    let start = Instant::now();
    let bottom_opening = CanopyVectorCommitment::open_bottom(
        &canopy_data,
        &first_oracle,
        first_oracle_value_indices,
    )?;
    std::hint::black_box(bottom_opening.auth_hashes.len());
    let canopy_open = start.elapsed();

    let aligned_len = message.len() >> DEFAULT_FOLDING_LOG;
    let outer_second = message[..aligned_len].to_vec();
    let canopy_claim = canopy_data.canopy_message.clone();
    ensure!(
        canopy_claim.len() == aligned_len,
        "Canopy and outer-second claims did not align"
    );

    let aligned_log_inverse_rate = DEFAULT_FOLDING_LOG + 1 - CANOPY_HYBRID_CODEWORD_REDUCTION;
    let outer_tail = execute_whir(
        std::slice::from_ref(&outer_second),
        aligned_log_inverse_rate,
        decoding_regime,
    )?;
    let canopy_tail = execute_whir(
        std::slice::from_ref(&canopy_claim),
        aligned_log_inverse_rate,
        decoding_regime,
    )?;
    let outer_front = outer.proving_time.saturating_sub(outer_tail.proving_time);
    let front = outer_rs + outer_ood + canopy_profile.total + canopy_open + outer_front;
    let separate = front + outer_tail.proving_time + canopy_tail.proving_time;

    let separate_stages = BTreeMap::from([
        ("outer_rs_encoding", outer_rs),
        ("outer_ood_evaluations", outer_ood),
        ("canopy_commitment", canopy_profile.total),
        ("canopy_bottom_opening", canopy_open),
        ("outer_initial_whir_stage", outer_front),
        ("outer_second_oracle_whir", outer_tail.proving_time),
        ("canopy_claim_whir", canopy_tail.proving_time),
    ]);
    Ok(HybridIteration {
        baseline,
        separate,
        baseline_stages,
        separate_stages,
    })
}

fn push_stages(
    destination: &mut BTreeMap<&'static str, Vec<Duration>>,
    stages: BTreeMap<&'static str, Duration>,
) {
    for (stage, duration) in stages {
        destination.entry(stage).or_default().push(duration);
    }
}

/// Run executable prover benchmarks at `2^log_length`. Every WHIR component
/// is generated and verifier-checked. Hybrid totals compose measured stages:
/// the stock initial Merkle commit is replaced by explicit RS generation plus
/// Canopy. Experiment 3 remains estimate-only until its combined recursive
/// oracle is implemented in the executable WHIR adapter.
pub fn run_prover_timings(
    output_dir: &Path,
    log_length: usize,
    warmups: usize,
    repetitions: usize,
    samples_for_height: usize,
    seed: u64,
) -> Result<()> {
    run_prover_timings_with_regime(
        output_dir,
        log_length,
        warmups,
        repetitions,
        samples_for_height,
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
    samples_for_height: usize,
    seed: u64,
    decoding_regime: WhirDecodingRegime,
) -> Result<()> {
    ensure!(repetitions > 0, "timing repetitions must be positive");
    ensure!(log_length >= 8, "timing instance is too small for WHIR");
    fs::create_dir_all(output_dir)?;
    let length = 1usize << log_length;
    let values: Vec<_> = (0..length as u64).map(Fr::from).collect();

    let height_dir = output_dir.join("experiment1_height_search");
    let height_rows = run_size_matrix_with_regime(
        &SizeMatrixConfig {
            log_lengths: vec![log_length],
            batch_sizes: PAPER_BATCH_SIZES.to_vec(),
            heights: full_height_grid(log_length, DEFAULT_FOLDING_LOG),
            layer_field_bits: vec![256],
            codeword_log_reductions: vec![0],
            folding_log: DEFAULT_FOLDING_LOG,
            samples: samples_for_height,
            seed,
        },
        &height_dir,
        decoding_regime,
    )?;
    let heights: BTreeMap<_, _> = height_rows
        .iter()
        .map(|row| (row.batch_size, row.best_h))
        .collect();

    let mut timing_rows = Vec::new();
    let mut all_stage_rows = Vec::new();
    for &batch in PAPER_BATCH_SIZES {
        let height = heights[&batch];
        let indices =
            sample_unique_indices(length, batch, derive_u64(seed, &[1, log_length, batch]))?;
        let mut merkle_totals = Vec::new();
        let mut canopy_totals = Vec::new();
        let mut merkle_stages = BTreeMap::<&'static str, Vec<Duration>>::new();
        let mut canopy_stages = BTreeMap::<&'static str, Vec<Duration>>::new();

        for repetition in 0..warmups + repetitions {
            let start = Instant::now();
            let (_, merkle_data) = MerkleVectorCommitment::commit(&values)?;
            let merkle_commit = start.elapsed();
            let start = Instant::now();
            let opening = MerkleVectorCommitment::open(&merkle_data, &values, &indices)?;
            std::hint::black_box(opening.auth_hashes.len());
            let merkle_open = start.elapsed();

            let (_, canopy_data, profile) =
                CanopyVectorCommitment::commit_profiled(&values, height, 0)?;
            let start = Instant::now();
            let opening = CanopyVectorCommitment::open_bottom(&canopy_data, &values, &indices)?;
            std::hint::black_box(opening.auth_hashes.len());
            let canopy_open = start.elapsed();
            let nested = execute_whir(
                std::slice::from_ref(&canopy_data.canopy_message),
                height,
                decoding_regime,
            )?;
            let canopy_total = profile.total + canopy_open + nested.proving_time;

            if repetition >= warmups {
                merkle_totals.push(merkle_commit + merkle_open);
                canopy_totals.push(canopy_total);
                merkle_stages
                    .entry("commitment")
                    .or_default()
                    .push(merkle_commit);
                merkle_stages
                    .entry("batch_opening")
                    .or_default()
                    .push(merkle_open);
                canopy_stages
                    .entry("canopy_commitment")
                    .or_default()
                    .push(profile.total);
                canopy_stages
                    .entry("bottom_batch_opening")
                    .or_default()
                    .push(canopy_open);
                canopy_stages
                    .entry("whir_claim_prover")
                    .or_default()
                    .push(nested.proving_time);
            }
        }
        let merkle_mean = duration_stats(&merkle_totals).0;
        let canopy_mean = duration_stats(&canopy_totals).0;
        timing_rows.push(timing_row(
            1,
            "merkle",
            log_length,
            Some(batch),
            None,
            warmups,
            &merkle_totals,
            merkle_mean,
            "commitment plus minimal batch multiproof generation",
        ));
        timing_rows.push(timing_row(
            1,
            "canopy",
            log_length,
            Some(batch),
            Some(height),
            warmups,
            &canopy_totals,
            merkle_mean,
            "Canopy commitment and bottom opening plus executable WHIR claim prover; WHIR's duplicate initial commit is excluded",
        ));
        all_stage_rows.extend(stage_rows(
            1,
            "merkle",
            log_length,
            Some(batch),
            merkle_mean,
            merkle_stages,
        ));
        all_stage_rows.extend(stage_rows(
            1,
            "canopy",
            log_length,
            Some(batch),
            canopy_mean,
            canopy_stages,
        ));
    }

    let outer_params = WhirParameters::with_regime(log_length, 1, decoding_regime)?;
    let outer_seed = expand_seed(derive_u64(seed, &[2, log_length, 0]));
    let first_query_leaves = sample_initial_oracle_queries(&outer_params, outer_seed);
    let mut first_query_values = Vec::with_capacity(first_query_leaves.len() * 16);
    for leaf in first_query_leaves {
        first_query_values.extend(leaf * 16..(leaf + 1) * 16);
    }
    let mut baseline_totals = Vec::new();
    let mut separate_totals = Vec::new();
    let mut baseline_stages = BTreeMap::<&'static str, Vec<Duration>>::new();
    let mut separate_stages = BTreeMap::<&'static str, Vec<Duration>>::new();
    for repetition in 0..warmups + repetitions {
        let iteration = run_hybrid_iteration(
            &values,
            &first_query_values,
            outer_params.commitment_ood_samples,
            decoding_regime,
        )?;
        if repetition >= warmups {
            baseline_totals.push(iteration.baseline);
            separate_totals.push(iteration.separate);
            push_stages(&mut baseline_stages, iteration.baseline_stages);
            push_stages(&mut separate_stages, iteration.separate_stages);
        }
    }
    let baseline_mean = duration_stats(&baseline_totals).0;
    let separate_mean = duration_stats(&separate_totals).0;
    timing_rows.push(timing_row(
        2,
        "merkle_whir",
        log_length,
        None,
        None,
        warmups,
        &baseline_totals,
        baseline_mean,
        "verifier-checked official WHIR commit plus prove",
    ));
    timing_rows.push(timing_row(
        2,
        "canopy_separate",
        log_length,
        None,
        Some(CANOPY_HYBRID_HEIGHT),
        warmups,
        &separate_totals,
        baseline_mean,
        "measured component composition with separate WHIR proofs for the aligned claims",
    ));
    all_stage_rows.extend(stage_rows(
        2,
        "merkle_whir",
        log_length,
        None,
        baseline_mean,
        baseline_stages,
    ));
    all_stage_rows.extend(stage_rows(
        2,
        "canopy_separate",
        log_length,
        None,
        separate_mean,
        separate_stages,
    ));

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
        "All WHIR proofs are generated and verifier-checked at 128-bit security over the 256-bit FFT field, in {} mode, with no proof of work.{} Input generation, verification, and deallocation are excluded. Hybrid totals are sums of the measured stages listed below.\n",
        decoding_regime.label(),
        if decoding_regime.is_conjectural() {
            " Capacity-bound soundness relies on STIR Conjecture 5.6 and is not proved."
        } else {
            ""
        },
    )?;
    writeln!(
        file,
        "| experiment | scheme | batch | h | repetitions | mean s | median s | vs baseline |"
    )?;
    writeln!(file, "|---:|---|---:|---:|---:|---:|---:|---:|")?;
    for row in rows {
        writeln!(
            file,
            "| {} | {} | {} | {} | {} | {:.4} | {:.4} | {:.3}x |",
            row.experiment,
            row.scheme,
            row.batch_size
                .map_or_else(|| "-".to_owned(), |x| x.to_string()),
            row.canopy_height
                .map_or_else(|| "-".to_owned(), |x| x.to_string()),
            row.repetitions,
            row.mean_prover_seconds,
            row.median_prover_seconds,
            row.relative_to_baseline,
        )?;
    }
    writeln!(file, "\n## Mean stage breakdown\n")?;
    writeln!(
        file,
        "| experiment | scheme | batch | stage | mean s | median s | share |"
    )?;
    writeln!(file, "|---:|---|---:|---|---:|---:|---:|")?;
    for row in stages {
        writeln!(
            file,
            "| {} | {} | {} | {} | {:.4} | {:.4} | {:.1}% |",
            row.experiment,
            row.scheme,
            row.batch_size
                .map_or_else(|| "-".to_owned(), |x| x.to_string()),
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
    fn small_proof_report_writes_expected_files() {
        // Exercise the report helpers without running the paper-size grid.
        let temp = tempdir().unwrap();
        write_csv::<HybridProofSizeRow>(temp.path().join("empty.csv"), &[]).unwrap();
        assert!(temp.path().join("empty.csv").exists());
    }
}
