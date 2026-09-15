use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{ensure, Context, Result};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::Serialize;

use crate::canopy::{CanopyVectorCommitment, WHIR_LEAF_WIDTH};
use crate::merkle::{frontier_hash_count, MerkleVectorCommitment};
use crate::whir::{
    estimate_proof, expand_seed, field_profile, WhirDecodingRegime, WhirFieldProfile,
    WhirParameters, DEFAULT_FOLDING_LOG, DEFAULT_SECURITY_BITS, MAX_TERMINAL_MESSAGE_VARIABLES,
};
use crate::{Fr, FIELD_BITS, FIELD_BYTES, HASH_BYTES};

pub const DEFAULT_LOG_LENGTHS: &[usize] = &[22, 24, 26, 28, 30];
pub const DEFAULT_BATCH_SIZES: &[usize] = &[64, 128, 256, 512, 1024];

#[derive(Clone, Debug)]
pub struct SizeMatrixConfig {
    pub log_lengths: Vec<usize>,
    pub batch_sizes: Vec<usize>,
    pub heights: Vec<usize>,
    pub layer_field_bits: Vec<usize>,
    pub codeword_log_reductions: Vec<usize>,
    /// Folding width used by the initial and all recursive WHIR folds.
    pub folding_log: usize,
    /// Candidate widths for an optimized ordered folding schedule.
    pub folding_choices: Vec<usize>,
    /// Search schedules that stop as soon as the message reaches the terminal cap.
    pub optimize_folding_schedule: bool,
    pub samples: usize,
    pub seed: u64,
}

impl Default for SizeMatrixConfig {
    fn default() -> Self {
        Self {
            log_lengths: DEFAULT_LOG_LENGTHS.to_vec(),
            batch_sizes: DEFAULT_BATCH_SIZES.to_vec(),
            // Empty means use the full default grid in
            // `default_heights_for_batch`.
            heights: Vec::new(),
            layer_field_bits: vec![256],
            codeword_log_reductions: vec![0],
            folding_log: DEFAULT_FOLDING_LOG,
            folding_choices: Vec::new(),
            optimize_folding_schedule: false,
            samples: 100,
            seed: 0x4341_4e4f_5059,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ProofSizeRow {
    pub log_length: usize,
    pub vector_length: u64,
    pub batch_size: usize,
    pub scheme: String,
    pub h: Option<usize>,
    pub folding_schedule: Option<String>,
    pub terminal_message_variables: Option<usize>,
    pub codeword_log_reduction: Option<usize>,
    pub rs_length_log: Option<usize>,
    pub input_field_bits: usize,
    pub layer_field_bits: Option<usize>,
    pub layer_field_bytes: Option<usize>,
    pub samples: usize,
    pub mean_hash_count: f64,
    pub mean_hash_bytes: f64,
    pub mean_field_count: f64,
    pub mean_field_bytes: f64,
    pub mean_input_field_count: f64,
    pub mean_input_field_bytes: f64,
    pub mean_layer_field_count: f64,
    pub mean_layer_field_bytes: f64,
    pub opened_value_bytes: usize,
    pub mean_total_bytes: f64,
    pub min_total_bytes: usize,
    pub max_total_bytes: usize,
    pub mean_total_kib: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct BestHeightRow {
    pub log_length: usize,
    pub vector_length: u64,
    pub batch_size: usize,
    pub h_candidates: String,
    pub merkle_mean_bytes: f64,
    pub best_layer_field_bits: usize,
    pub best_codeword_log_reduction: usize,
    pub best_h: usize,
    pub best_folding_schedule: String,
    pub best_terminal_message_variables: usize,
    pub canopy_mean_bytes: f64,
    pub canopy_hash_bytes: f64,
    pub canopy_field_bytes: f64,
    pub canopy_over_merkle: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct BestByFieldRow {
    pub log_length: usize,
    pub vector_length: u64,
    pub batch_size: usize,
    pub input_field_bits: usize,
    pub layer_field_bits: usize,
    pub layer_field_log2_size: f64,
    pub codeword_log_reduction: usize,
    pub rs_length_log: usize,
    pub h_candidates: String,
    pub merkle_mean_bytes: f64,
    pub best_h: usize,
    pub best_folding_schedule: String,
    pub best_terminal_message_variables: usize,
    pub canopy_mean_bytes: f64,
    pub canopy_hash_bytes: f64,
    pub canopy_field_bytes: f64,
    pub canopy_over_merkle: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct FieldProfileRow {
    pub log_length: usize,
    pub requested_bits: usize,
    pub feasible: bool,
    pub modulus: Option<&'static str>,
    pub log2_size: Option<f64>,
    pub element_bytes: usize,
    pub two_adicity: Option<usize>,
    pub domain_kind: Option<&'static str>,
    pub max_domain_log: Option<usize>,
    pub required_domain_log: usize,
    pub reason: String,
}

/// Default full height grid requested for the batch-size sweep.
pub fn default_heights_for_batch(batch_size: usize) -> Result<Vec<usize>> {
    let heights = match batch_size {
        64 | 128 | 256 | 512 | 1024 => (1..=15).collect(),
        _ => anyhow::bail!("no default h grid configured for batch size {batch_size}"),
    };
    Ok(heights)
}

fn folding_schedules_up_to(
    message_variables: usize,
    max_terminal_message_variables: usize,
    choices: &[usize],
) -> Vec<Vec<usize>> {
    fn visit(
        remaining: usize,
        terminal_cap: usize,
        choices: &[usize],
        prefix: &mut Vec<usize>,
        schedules: &mut Vec<Vec<usize>>,
    ) {
        if remaining <= terminal_cap {
            schedules.push(prefix.clone());
            return;
        }
        for &choice in choices {
            if choice <= remaining {
                prefix.push(choice);
                visit(remaining - choice, terminal_cap, choices, prefix, schedules);
                prefix.pop();
            }
        }
    }

    let mut choices = choices.to_vec();
    choices.sort_unstable();
    choices.dedup();
    let mut schedules = Vec::new();
    visit(
        message_variables,
        max_terminal_message_variables,
        &choices,
        &mut Vec::new(),
        &mut schedules,
    );
    schedules
}

fn folding_schedules_for_message(
    config: &SizeMatrixConfig,
    message_variables: usize,
    terminal_message_variables: usize,
) -> Vec<Vec<usize>> {
    if config.optimize_folding_schedule {
        folding_schedules_up_to(
            message_variables,
            terminal_message_variables,
            &config.folding_choices,
        )
    } else {
        vec![Vec::new()]
    }
}

fn height_candidates(
    config: &SizeMatrixConfig,
    n: usize,
    batch_size: usize,
    terminal_message_variables: usize,
) -> Result<Vec<usize>> {
    let requested = if config.heights.is_empty() {
        default_heights_for_batch(batch_size)?
    } else {
        config.heights.clone()
    };
    let candidates: Vec<_> = requested
        .into_iter()
        .filter(|&h| {
            h < n
                && if config.optimize_folding_schedule {
                    !folding_schedules_for_message(config, n - h, terminal_message_variables)
                        .is_empty()
                } else {
                    n - h >= config.folding_log
                }
        })
        .collect();
    ensure!(
        !candidates.is_empty(),
        "no valid h candidate for log length {n} and batch size {batch_size}"
    );
    Ok(candidates)
}

#[allow(clippy::too_many_arguments)]
fn whir_params_for_schedule(
    config: &SizeMatrixConfig,
    message_variables: usize,
    log_inverse_rate: usize,
    field: WhirFieldProfile,
    decoding_regime: WhirDecodingRegime,
    terminal_message_variables: usize,
    folding_schedule: &[usize],
) -> Result<WhirParameters> {
    if config.optimize_folding_schedule {
        let terminal_message_variables =
            message_variables - folding_schedule.iter().copied().sum::<usize>();
        WhirParameters::with_regime_and_field_and_terminal_and_schedule(
            message_variables,
            log_inverse_rate,
            field,
            decoding_regime,
            terminal_message_variables,
            folding_schedule,
            1,
        )
    } else {
        WhirParameters::with_regime_and_field_and_terminal_and_folding(
            message_variables,
            log_inverse_rate,
            field,
            decoding_regime,
            terminal_message_variables,
            config.folding_log,
        )
    }
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

#[derive(Clone, Debug, Serialize)]
pub struct WhirParameterRow {
    pub decoding_regime: &'static str,
    pub max_terminal_message_variables: usize,
    pub log_length: usize,
    pub h: usize,
    pub codeword_log_reduction: usize,
    pub message_variables: usize,
    pub rs_dimension_log: usize,
    pub rs_length_log: usize,
    pub log_inverse_rate: usize,
    pub folding_log: usize,
    pub folding_factor: usize,
    pub terminal_message_variables: usize,
    pub security_bits: usize,
    pub field_bits: usize,
    pub field_modulus: &'static str,
    pub field_log2_size: f64,
    pub field_bytes: usize,
    pub field_two_adicity: usize,
    pub hash_bits: usize,
    pub hash_collision_security_bits: usize,
    pub effective_security_bits: usize,
    pub pow_enabled: bool,
    pub minimum_single_challenge_security_bits: f64,
    pub commitment_ood_samples: usize,
    pub rounds: usize,
    pub folding_logs_initial_and_rounds: String,
    pub query_leaf_widths_by_oracle: String,
    pub requested_queries_by_oracle: String,
    pub query_distances_by_oracle: String,
    pub query_security_bits_per_query_by_oracle: String,
    pub ood_samples_by_commitment: String,
    pub folding_security_bits_per_challenge_initial_and_rounds: String,
    pub combination_security_bits_per_challenge_by_round: String,
    pub sumcheck_repetitions_initial_and_rounds: String,
    pub sumcheck_challenges_initial_and_rounds: String,
    pub final_sumcheck_rounds: usize,
    pub final_sumcheck_security_bits_per_challenge: f64,
    pub final_sumcheck_repetitions: usize,
    pub final_sumcheck_challenges: usize,
    pub final_log_inverse_rate: usize,
    pub final_oracle_leaf_width: usize,
}

#[derive(Default)]
struct Samples {
    hashes: Vec<usize>,
    input_fields: Vec<usize>,
    layer_fields: Vec<usize>,
    field_bytes: Vec<usize>,
}

impl Samples {
    fn push(
        &mut self,
        hashes: usize,
        input_fields: usize,
        layer_fields: usize,
        layer_field_bytes: usize,
    ) {
        self.hashes.push(hashes);
        self.input_fields.push(input_fields);
        self.layer_fields.push(layer_fields);
        self.field_bytes
            .push(input_fields * FIELD_BYTES + layer_fields * layer_field_bytes);
    }

    #[allow(clippy::too_many_arguments)]
    fn row(
        &self,
        log_length: usize,
        batch_size: usize,
        scheme: &str,
        h: Option<usize>,
        folding_schedule: Option<String>,
        terminal_message_variables: Option<usize>,
        layer_field: Option<WhirFieldProfile>,
        codeword_log_reduction: Option<usize>,
    ) -> ProofSizeRow {
        let totals: Vec<usize> = self
            .hashes
            .iter()
            .zip(&self.field_bytes)
            .map(|(&hashes, &field_bytes)| hashes * HASH_BYTES + field_bytes)
            .collect();
        let mean = |xs: &[usize]| xs.iter().sum::<usize>() as f64 / xs.len() as f64;
        let mean_hash_count = mean(&self.hashes);
        let mean_input_field_count = mean(&self.input_fields);
        let mean_layer_field_count = mean(&self.layer_fields);
        let mean_field_count = mean_input_field_count + mean_layer_field_count;
        let mean_field_bytes = mean(&self.field_bytes);
        let mean_total_bytes = mean(&totals);
        ProofSizeRow {
            log_length,
            vector_length: 1u64 << log_length,
            batch_size,
            scheme: scheme.to_owned(),
            h,
            folding_schedule,
            terminal_message_variables,
            codeword_log_reduction,
            rs_length_log: codeword_log_reduction.map(|reduction| log_length - reduction),
            input_field_bits: FIELD_BITS,
            layer_field_bits: layer_field.map(|field| field.bits),
            layer_field_bytes: layer_field.map(|field| field.element_bytes),
            samples: totals.len(),
            mean_hash_count,
            mean_hash_bytes: mean_hash_count * HASH_BYTES as f64,
            mean_field_count,
            mean_field_bytes,
            mean_input_field_count,
            mean_input_field_bytes: mean_input_field_count * FIELD_BYTES as f64,
            mean_layer_field_count,
            mean_layer_field_bytes: mean_field_bytes - mean_input_field_count * FIELD_BYTES as f64,
            opened_value_bytes: batch_size * FIELD_BYTES,
            mean_total_bytes,
            min_total_bytes: *totals.iter().min().unwrap(),
            max_total_bytes: *totals.iter().max().unwrap(),
            mean_total_kib: mean_total_bytes / 1024.0,
        }
    }
}

fn derive_u64(seed: u64, words: &[usize]) -> u64 {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"canopy-system/benchmark-seed/v1");
    hasher.update(&seed.to_le_bytes());
    for &word in words {
        hasher.update(&(word as u64).to_le_bytes());
    }
    let digest = hasher.finalize();
    u64::from_le_bytes(digest.as_bytes()[..8].try_into().unwrap())
}

pub fn sample_unique_indices(universe: usize, count: usize, seed: u64) -> Result<Vec<usize>> {
    ensure!(
        count <= universe,
        "cannot sample more indices than the universe"
    );
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let mut set = BTreeSet::new();
    while set.len() < count {
        set.insert(rng.gen_range(0..universe));
    }
    Ok(set.into_iter().collect())
}

pub fn run_size_matrix(config: &SizeMatrixConfig, output_dir: &Path) -> Result<Vec<BestHeightRow>> {
    run_size_matrix_with_regime(config, output_dir, WhirDecodingRegime::Johnson)
}

pub fn run_size_matrix_with_regime(
    config: &SizeMatrixConfig,
    output_dir: &Path,
    decoding_regime: WhirDecodingRegime,
) -> Result<Vec<BestHeightRow>> {
    run_size_matrix_with_regime_and_terminal(
        config,
        output_dir,
        decoding_regime,
        MAX_TERMINAL_MESSAGE_VARIABLES,
    )
}

pub fn run_size_matrix_with_regime_and_terminal(
    config: &SizeMatrixConfig,
    output_dir: &Path,
    decoding_regime: WhirDecodingRegime,
    max_terminal_message_variables: usize,
) -> Result<Vec<BestHeightRow>> {
    ensure!(config.samples > 0, "samples must be positive");
    ensure!(
        !config.layer_field_bits.is_empty(),
        "at least one Canopy-layer field size is required"
    );
    ensure!(
        !config.codeword_log_reductions.is_empty(),
        "at least one codeword length is required"
    );
    if config.optimize_folding_schedule {
        ensure!(
            !config.folding_choices.is_empty(),
            "folding-schedule search requires folding choices"
        );
        ensure!(
            config.folding_choices.iter().all(|&fold| fold > 0),
            "folding choices must be positive"
        );
    }
    fs::create_dir_all(output_dir).with_context(|| format!("creating {}", output_dir.display()))?;

    let mut proof_rows = Vec::new();
    let mut best_rows = Vec::new();
    let mut best_by_field_rows = Vec::new();
    let mut parameter_rows = Vec::new();
    let mut field_profile_rows = Vec::new();

    for &n in &config.log_lengths {
        ensure!(n < usize::BITS as usize, "log length does not fit usize");
        let vector_len = 1usize << n;
        let reductions: Vec<_> = config
            .codeword_log_reductions
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        ensure!(
            reductions.iter().all(|&reduction| reduction < n),
            "codeword log reduction must be smaller than log length {n}"
        );
        let required_codeword_log = n - reductions.iter().copied().min().unwrap();
        let mut layer_fields = Vec::new();
        for bits in config
            .layer_field_bits
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
        {
            match field_profile(bits, required_codeword_log) {
                Ok(field) => {
                    field_profile_rows.push(FieldProfileRow {
                        log_length: n,
                        requested_bits: bits,
                        feasible: true,
                        modulus: Some(field.modulus),
                        log2_size: Some(field.log2_size),
                        element_bytes: field.element_bytes,
                        two_adicity: Some(field.two_adicity),
                        domain_kind: Some(field.domain_kind),
                        max_domain_log: Some(field.max_domain_log),
                        required_domain_log: required_codeword_log,
                        reason: format!(
                            "supports the largest requested 2^{required_codeword_log} {} domain",
                            field.domain_kind,
                        ),
                    });
                    layer_fields.push(field);
                }
                Err(error) => field_profile_rows.push(FieldProfileRow {
                    log_length: n,
                    requested_bits: bits,
                    feasible: false,
                    modulus: None,
                    log2_size: None,
                    element_bytes: bits.div_ceil(8),
                    two_adicity: None,
                    domain_kind: None,
                    max_domain_log: None,
                    required_domain_log: required_codeword_log,
                    reason: error.to_string(),
                }),
            }
        }
        ensure!(
            !layer_fields.is_empty(),
            "none of the requested Canopy-layer fields is feasible for 2^{n}"
        );
        let parameter_heights: BTreeSet<usize> = config
            .batch_sizes
            .iter()
            .map(|&batch_size| {
                height_candidates(config, n, batch_size, max_terminal_message_variables)
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();
        for &field in &layer_fields {
            for &reduction in &reductions {
                for &h in parameter_heights.iter().filter(|&&h| h > reduction) {
                    ensure!(h < n, "h must be smaller than n");
                    for folding_schedule in
                        folding_schedules_for_message(config, n - h, max_terminal_message_variables)
                    {
                        let params = whir_params_for_schedule(
                            config,
                            n - h,
                            h - reduction,
                            field,
                            decoding_regime,
                            max_terminal_message_variables,
                            &folding_schedule,
                        )?;
                        let combination_security = params
                            .rounds
                            .iter()
                            .map(|r| format!("{:.2}", r.combination_security_bits_per_challenge))
                            .collect::<Vec<_>>()
                            .join(";");
                        let folding_security = params
                            .all_folding_security_bits_per_challenge()
                            .iter()
                            .map(|x| format!("{x:.2}"))
                            .collect::<Vec<_>>()
                            .join(";");
                        let sumcheck_repetitions = params
                            .all_sumcheck_repetitions()
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(";");
                        let sumcheck_challenges = params
                            .all_sumcheck_repetitions()
                            .iter()
                            .zip(params.all_folding_logs())
                            .map(|(repetitions, folding_log)| repetitions * folding_log)
                            .map(|challenges| challenges.to_string())
                            .collect::<Vec<_>>()
                            .join(";");
                        parameter_rows.push(WhirParameterRow {
                            decoding_regime: decoding_regime.label(),
                            max_terminal_message_variables,
                            log_length: n,
                            h,
                            codeword_log_reduction: reduction,
                            message_variables: n - h,
                            rs_dimension_log: n - h,
                            rs_length_log: n - reduction,
                            log_inverse_rate: h - reduction,
                            folding_log: params.folding_log,
                            folding_factor: params.folding_factor,
                            terminal_message_variables: params.terminal_message_variables,
                            security_bits: params.security_bits,
                            field_bits: params.field_bits,
                            field_modulus: params.field_modulus,
                            field_log2_size: params.field_log2_size,
                            field_bytes: params.field_bytes,
                            field_two_adicity: params.field_two_adicity,
                            hash_bits: params.hash_bits,
                            hash_collision_security_bits: params.hash_collision_security_bits,
                            effective_security_bits: params.effective_security_bits,
                            pow_enabled: false,
                            minimum_single_challenge_security_bits: params
                                .minimum_single_challenge_security_bits(),
                            commitment_ood_samples: params.commitment_ood_samples,
                            rounds: params.rounds.len(),
                            folding_logs_initial_and_rounds: params
                                .all_folding_logs()
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(";"),
                            query_leaf_widths_by_oracle: params
                                .all_query_leaf_widths()
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(";"),
                            requested_queries_by_oracle: params
                                .all_query_counts()
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(";"),
                            query_distances_by_oracle: params
                                .all_query_distances()
                                .iter()
                                .map(|distance| format!("{distance:.12}"))
                                .collect::<Vec<_>>()
                                .join(";"),
                            query_security_bits_per_query_by_oracle: params
                                .all_query_security_bits_per_query()
                                .iter()
                                .map(|bits| format!("{bits:.6}"))
                                .collect::<Vec<_>>()
                                .join(";"),
                            ood_samples_by_commitment: params
                                .all_ood_counts()
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(";"),
                            folding_security_bits_per_challenge_initial_and_rounds:
                                folding_security,
                            combination_security_bits_per_challenge_by_round: combination_security,
                            sumcheck_repetitions_initial_and_rounds: sumcheck_repetitions,
                            sumcheck_challenges_initial_and_rounds: sumcheck_challenges,
                            final_sumcheck_rounds: params.final_sumcheck_rounds,
                            final_sumcheck_security_bits_per_challenge: params
                                .final_sumcheck_security_bits_per_challenge,
                            final_sumcheck_repetitions: params.final_sumcheck_repetitions,
                            final_sumcheck_challenges: params.final_sumcheck_rounds
                                * params.final_sumcheck_repetitions,
                            final_log_inverse_rate: params.final_log_inverse_rate,
                            final_oracle_leaf_width: params.final_oracle_leaf_width,
                        });
                    }
                }
            }
        }

        for &batch_size in &config.batch_sizes {
            ensure!(batch_size <= vector_len, "batch exceeds vector length");
            let heights = height_candidates(config, n, batch_size, max_terminal_message_variables)?;
            let mut merkle_samples = Samples::default();
            let mut canopy_samples =
                Vec::<(WhirFieldProfile, usize, usize, Vec<usize>, Samples)>::new();
            for &field in &layer_fields {
                for &reduction in &reductions {
                    for &h in heights.iter().filter(|&&h| h > reduction) {
                        for folding_schedule in folding_schedules_for_message(
                            config,
                            n - h,
                            max_terminal_message_variables,
                        ) {
                            canopy_samples.push((
                                field,
                                reduction,
                                h,
                                folding_schedule,
                                Samples::default(),
                            ));
                        }
                    }
                }
            }

            for sample in 0..config.samples {
                let indices = sample_unique_indices(
                    vector_len,
                    batch_size,
                    derive_u64(config.seed, &[n, batch_size, sample]),
                )?;
                let merkle_hashes = frontier_hash_count(&indices, vector_len, n)?;
                merkle_samples.push(merkle_hashes, batch_size, 0, 0);

                for (field, reduction, h, folding_schedule, samples) in &mut canopy_samples {
                    let bottom_hashes = frontier_hash_count(&indices, vector_len, *h)?;
                    let params = whir_params_for_schedule(
                        config,
                        n - *h,
                        *h - *reduction,
                        *field,
                        decoding_regime,
                        max_terminal_message_variables,
                        folding_schedule,
                    )?;
                    let whir_seed = expand_seed(derive_u64(
                        config.seed,
                        &[
                            n,
                            batch_size,
                            *h,
                            sample,
                            folding_schedule
                                .iter()
                                .fold(0usize, |acc, &fold| acc.wrapping_mul(8).wrapping_add(fold)),
                            0x5748_4952,
                        ],
                    ));
                    let whir = estimate_proof(&params, whir_seed)?;
                    samples.push(
                        bottom_hashes + whir.hash_count,
                        batch_size,
                        whir.field_count,
                        field.element_bytes,
                    );
                }
            }

            let merkle_row =
                merkle_samples.row(n, batch_size, "merkle", None, None, None, None, None);
            proof_rows.push(merkle_row.clone());
            let mut rows_for_batch = Vec::new();
            for (field, reduction, h, folding_schedule, samples) in &canopy_samples {
                let row = samples.row(
                    n,
                    batch_size,
                    "canopy",
                    Some(*h),
                    Some(schedule_label(folding_schedule)),
                    Some(n - *h - folding_schedule.iter().copied().sum::<usize>()),
                    Some(*field),
                    Some(*reduction),
                );
                proof_rows.push(row.clone());
                rows_for_batch.push(row);
            }
            for &field in &layer_fields {
                for &reduction in &reductions {
                    let best = rows_for_batch
                        .iter()
                        .filter(|row| {
                            row.layer_field_bits == Some(field.bits)
                                && row.codeword_log_reduction == Some(reduction)
                        })
                        .min_by(|a, b| a.mean_total_bytes.total_cmp(&b.mean_total_bytes))
                        .unwrap();
                    best_by_field_rows.push(BestByFieldRow {
                        log_length: n,
                        vector_length: 1u64 << n,
                        batch_size,
                        input_field_bits: FIELD_BITS,
                        layer_field_bits: field.bits,
                        layer_field_log2_size: field.log2_size,
                        codeword_log_reduction: reduction,
                        rs_length_log: n - reduction,
                        h_candidates: heights
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(";"),
                        merkle_mean_bytes: merkle_row.mean_total_bytes,
                        best_h: best.h.unwrap(),
                        best_folding_schedule: best.folding_schedule.clone().unwrap(),
                        best_terminal_message_variables: best.terminal_message_variables.unwrap(),
                        canopy_mean_bytes: best.mean_total_bytes,
                        canopy_hash_bytes: best.mean_hash_bytes,
                        canopy_field_bytes: best.mean_field_bytes,
                        canopy_over_merkle: best.mean_total_bytes / merkle_row.mean_total_bytes,
                    });
                }
            }
            let best = rows_for_batch
                .iter()
                .min_by(|a, b| a.mean_total_bytes.total_cmp(&b.mean_total_bytes))
                .unwrap();
            best_rows.push(BestHeightRow {
                log_length: n,
                vector_length: 1u64 << n,
                batch_size,
                h_candidates: heights
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(";"),
                merkle_mean_bytes: merkle_row.mean_total_bytes,
                best_layer_field_bits: best.layer_field_bits.unwrap(),
                best_codeword_log_reduction: best.codeword_log_reduction.unwrap(),
                best_h: best.h.unwrap(),
                best_folding_schedule: best.folding_schedule.clone().unwrap(),
                best_terminal_message_variables: best.terminal_message_variables.unwrap(),
                canopy_mean_bytes: best.mean_total_bytes,
                canopy_hash_bytes: best.mean_hash_bytes,
                canopy_field_bytes: best.mean_field_bytes,
                canopy_over_merkle: best.mean_total_bytes / merkle_row.mean_total_bytes,
            });
        }
    }

    write_csv(output_dir.join("proof_sizes.csv"), &proof_rows)?;
    write_csv(output_dir.join("best_h.csv"), &best_rows)?;
    write_csv(output_dir.join("best_h_by_field.csv"), &best_by_field_rows)?;
    write_csv(output_dir.join("field_profiles.csv"), &field_profile_rows)?;
    write_csv(output_dir.join("whir_parameters.csv"), &parameter_rows)?;
    write_summary_markdown(
        output_dir.join("summary.md"),
        config,
        &best_rows,
        &best_by_field_rows,
        &field_profile_rows,
        decoding_regime,
        max_terminal_message_variables,
    )?;
    Ok(best_rows)
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

fn write_summary_markdown(
    path: impl AsRef<Path>,
    config: &SizeMatrixConfig,
    rows: &[BestHeightRow],
    by_field: &[BestByFieldRow],
    field_profiles: &[FieldProfileRow],
    decoding_regime: WhirDecodingRegime,
    max_terminal_message_variables: usize,
) -> Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "# Canopy vs. Merkle batch-opening proof sizes\n")?;
    writeln!(
        file,
        "Mean of {} deterministic random index/query samples; seed `{:#x}`. Sizes include opened values, exclude verifier-generated indices and randomness.\n",
        config.samples, config.seed
    )?;
    if config.optimize_folding_schedule {
        writeln!(
            file,
            "WHIR security target: {} bits. Folding schedules are searched over `{{{}}}` and stop as soon as the message contains at most `2^{}` coefficients.\n",
            DEFAULT_SECURITY_BITS,
            config
                .folding_choices
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(","),
            max_terminal_message_variables,
        )?;
    } else {
        writeln!(
            file,
            "WHIR security target: {} bits. Recursive folding terminates once message length is at most `2^{}`; all folds use `2^{}`.\n",
            DEFAULT_SECURITY_BITS, max_terminal_message_variables, config.folding_log
        )?;
    }
    writeln!(
        file,
        "Input-vector elements use the fixed {}-bit field and occupy {} bytes. The Canopy/WHIR layer field is searched independently. Merkle hashes occupy {} bytes.\n",
        FIELD_BITS,
        FIELD_BYTES,
        HASH_BYTES
    )?;
    writeln!(
        file,
        "No proof of work is used. Oracle queries, OOD samples, and every fold target {} bits in the {} regime over the 256-bit field.{}\n",
        DEFAULT_SECURITY_BITS,
        decoding_regime.label(),
        if decoding_regime.is_conjectural() {
            " These estimates rely on STIR Conjecture 5.6 and are not proved."
        } else {
            ""
        },
    )?;
    writeln!(file, "## Field feasibility\n")?;
    writeln!(
        file,
        "| log2 N | requested bits | feasible | log2 field size | bytes | two-adicity | reason |"
    )?;
    writeln!(file, "|---:|---:|:---:|---:|---:|---:|---|")?;
    for row in field_profiles {
        writeln!(
            file,
            "| {} | {} | {} | {} | {} | {} | {} |",
            row.log_length,
            row.requested_bits,
            if row.feasible { "yes" } else { "no" },
            row.log2_size
                .map_or_else(|| "-".to_owned(), |value| format!("{value:.6}")),
            row.element_bytes,
            row.two_adicity
                .map_or_else(|| "-".to_owned(), |value| value.to_string()),
            row.reason,
        )?;
    }

    writeln!(
        file,
        "\n## Best height for each feasible layer field and codeword length\n"
    )?;
    writeln!(
        file,
        "| log2 N | batch | layer bits | codeword | best h | folds | terminal log | Merkle KiB | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |"
    )?;
    writeln!(
        file,
        "|---:|---:|---:|---|---:|---|---:|---:|---:|---:|---:|---:|"
    )?;
    for row in by_field {
        writeln!(
            file,
            "| {} | {} | {} | N/{} | {} | {} | {} | {:.2} | {:.2} | {:.2} | {:.2} | {:.3} |",
            row.log_length,
            row.batch_size,
            row.layer_field_bits,
            1usize << row.codeword_log_reduction,
            row.best_h,
            row.best_folding_schedule.replace(';', ","),
            row.best_terminal_message_variables,
            row.merkle_mean_bytes / 1024.0,
            row.canopy_mean_bytes / 1024.0,
            row.canopy_hash_bytes / 1024.0,
            row.canopy_field_bytes / 1024.0,
            row.canopy_over_merkle,
        )?;
    }

    writeln!(file, "\n## Overall optimum\n")?;
    writeln!(
        file,
        "| log2 N | batch | h candidates | Merkle KiB | layer bits | codeword | best h | folds | terminal log | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |"
    )?;
    writeln!(
        file,
        "|---:|---:|---|---:|---:|---|---:|---|---:|---:|---:|---:|---:|"
    )?;
    for row in rows {
        writeln!(
            file,
            "| {} | {} | {} | {:.2} | {} | N/{} | {} | {} | {} | {:.2} | {:.2} | {:.2} | {:.3} |",
            row.log_length,
            row.batch_size,
            row.h_candidates.replace(';', ","),
            row.merkle_mean_bytes / 1024.0,
            row.best_layer_field_bits,
            1usize << row.best_codeword_log_reduction,
            row.best_h,
            row.best_folding_schedule.replace(';', ","),
            row.best_terminal_message_variables,
            row.canopy_mean_bytes / 1024.0,
            row.canopy_hash_bytes / 1024.0,
            row.canopy_field_bytes / 1024.0,
            row.canopy_over_merkle,
        )?;
    }
    Ok(())
}

#[derive(Clone, Debug, Serialize)]
pub struct CommitmentTimingRow {
    pub log_length: usize,
    pub vector_length: usize,
    pub scheme: String,
    pub codeword_log_reduction: Option<usize>,
    pub h: Option<usize>,
    pub rs_length: Option<usize>,
    pub repetitions: usize,
    pub mean_seconds: f64,
    pub median_seconds: f64,
    pub min_seconds: f64,
    pub max_seconds: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct CommitmentStageTimingRow {
    pub log_length: usize,
    pub scheme: String,
    pub codeword_log_reduction: Option<usize>,
    pub h: Option<usize>,
    pub stage: String,
    pub work_items: Option<usize>,
    pub work_unit: String,
    pub repetitions: usize,
    pub mean_seconds: f64,
    pub median_seconds: f64,
    pub min_seconds: f64,
    pub max_seconds: f64,
    pub percent_of_mean_total: f64,
}

fn duration_statistics(durations: &[Duration]) -> (f64, f64, f64, f64) {
    let mut seconds: Vec<_> = durations.iter().map(Duration::as_secs_f64).collect();
    seconds.sort_by(f64::total_cmp);
    let mean = seconds.iter().sum::<f64>() / seconds.len() as f64;
    let median = if seconds.len() % 2 == 0 {
        (seconds[seconds.len() / 2 - 1] + seconds[seconds.len() / 2]) / 2.0
    } else {
        seconds[seconds.len() / 2]
    };
    (mean, median, seconds[0], *seconds.last().unwrap())
}

fn commitment_profile_path(output: &Path) -> PathBuf {
    let stem = output
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("commitment_timings");
    output.with_file_name(format!("{stem}_profile.csv"))
}

struct StageTimingContext<'a> {
    log_length: usize,
    scheme: &'a str,
    reduction: Option<usize>,
    h: Option<usize>,
    mean_total: f64,
}

fn stage_timing_row(
    context: &StageTimingContext<'_>,
    stage: &str,
    work_items: Option<usize>,
    work_unit: &str,
    durations: &[Duration],
) -> CommitmentStageTimingRow {
    let (mean, median, min, max) = duration_statistics(durations);
    CommitmentStageTimingRow {
        log_length: context.log_length,
        scheme: context.scheme.to_owned(),
        codeword_log_reduction: context.reduction,
        h: context.h,
        stage: stage.to_owned(),
        work_items,
        work_unit: work_unit.to_owned(),
        repetitions: durations.len(),
        mean_seconds: mean,
        median_seconds: median,
        min_seconds: min,
        max_seconds: max,
        percent_of_mean_total: 100.0 * mean / context.mean_total,
    }
}

/// Compare prover commitment time for Merkle and selected Canopy codeword
/// lengths. Input-vector construction and deallocation are outside the timer.
pub fn run_commitment_timings(
    log_length: usize,
    configurations: &[(usize, usize)],
    repetitions: usize,
    output: &Path,
) -> Result<Vec<CommitmentTimingRow>> {
    ensure!(repetitions > 0, "repetitions must be positive");
    ensure!(
        !configurations.is_empty(),
        "at least one Canopy timing configuration is required"
    );
    let vector_len = 1usize << log_length;
    let values: Vec<Fr> = (0..vector_len as u64).map(Fr::from).collect();
    let mut rows = Vec::new();
    let mut stage_rows = Vec::new();

    let _ = MerkleVectorCommitment::commit(&values)?;
    let mut durations = Vec::with_capacity(repetitions);
    for _ in 0..repetitions {
        let start = Instant::now();
        let (commitment, data) = MerkleVectorCommitment::commit(&values)?;
        durations.push(start.elapsed());
        std::hint::black_box(commitment.root);
        drop(data);
    }
    let (mean, median, min, max) = duration_statistics(&durations);
    let merkle_stage_context = StageTimingContext {
        log_length,
        scheme: "merkle",
        reduction: None,
        h: None,
        mean_total: mean,
    };
    stage_rows.push(stage_timing_row(
        &merkle_stage_context,
        "full_merkle_tree",
        Some(2 * vector_len - 1),
        "hashes",
        &durations,
    ));
    rows.push(CommitmentTimingRow {
        log_length,
        vector_length: vector_len,
        scheme: "merkle".to_owned(),
        codeword_log_reduction: None,
        h: None,
        rs_length: None,
        repetitions,
        mean_seconds: mean,
        median_seconds: median,
        min_seconds: min,
        max_seconds: max,
    });

    for &(reduction, h) in configurations {
        ensure!(reduction < h, "Canopy reduction must be smaller than h");
        let _ = CanopyVectorCommitment::commit_profiled(&values, h, reduction)?;
        let mut durations = Vec::with_capacity(repetitions);
        let mut bottom_durations = Vec::with_capacity(repetitions);
        let mut hash_to_field_durations = Vec::with_capacity(repetitions);
        let mut encoding_durations = Vec::with_capacity(repetitions);
        let mut top_tree_durations = Vec::with_capacity(repetitions);
        let mut other_durations = Vec::with_capacity(repetitions);
        for _ in 0..repetitions {
            let (commitment, data, profile) =
                CanopyVectorCommitment::commit_profiled(&values, h, reduction)?;
            durations.push(profile.total);
            bottom_durations.push(profile.bottom_forest);
            hash_to_field_durations.push(profile.hash_to_field);
            encoding_durations.push(profile.reed_solomon_encoding);
            top_tree_durations.push(profile.codeword_merkle_tree);
            other_durations.push(profile.other());
            std::hint::black_box(commitment.root);
            drop(data);
        }
        let (mean, median, min, max) = duration_statistics(&durations);
        let scheme = match reduction {
            0 => "canopy_original",
            1 => "canopy_half",
            2 => "canopy_quarter",
            _ => "canopy_shortened",
        };
        let stage_context = StageTimingContext {
            log_length,
            scheme,
            reduction: Some(reduction),
            h: Some(h),
            mean_total: mean,
        };
        let rs_length = vector_len >> reduction;
        for (stage, work_items, work_unit, stage_durations) in [
            (
                "bottom_forest",
                Some(2 * vector_len - (vector_len >> h)),
                "hashes",
                &bottom_durations,
            ),
            (
                "hash_to_field",
                Some(vector_len >> h),
                "roots",
                &hash_to_field_durations,
            ),
            (
                "reed_solomon_encoding",
                Some(rs_length),
                "codeword elements",
                &encoding_durations,
            ),
            (
                "codeword_merkle_tree",
                Some(2 * (rs_length / WHIR_LEAF_WIDTH) - 1),
                "hashes",
                &top_tree_durations,
            ),
            ("other", None, "-", &other_durations),
        ] {
            stage_rows.push(stage_timing_row(
                &stage_context,
                stage,
                work_items,
                work_unit,
                stage_durations,
            ));
        }
        rows.push(CommitmentTimingRow {
            log_length,
            vector_length: vector_len,
            scheme: scheme.to_owned(),
            codeword_log_reduction: Some(reduction),
            h: Some(h),
            rs_length: Some(vector_len >> reduction),
            repetitions,
            mean_seconds: mean,
            median_seconds: median,
            min_seconds: min,
            max_seconds: max,
        });
    }

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    write_csv(output, &rows)?;
    write_csv(commitment_profile_path(output), &stage_rows)?;
    write_commitment_timing_markdown(output.with_extension("md"), &rows, &stage_rows)?;
    Ok(rows)
}

fn write_commitment_timing_markdown(
    path: impl AsRef<Path>,
    rows: &[CommitmentTimingRow],
    stage_rows: &[CommitmentStageTimingRow],
) -> Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "# Prover commitment timings\n")?;
    writeln!(
        file,
        "Input-vector generation and deallocation are excluded. Canopy heights are the batch-128 proof-size optima for their respective codeword lengths.\n"
    )?;
    writeln!(
        file,
        "| scheme | codeword | h | repetitions | mean s | median s | min s | max s |"
    )?;
    writeln!(file, "|---|---|---:|---:|---:|---:|---:|---:|")?;
    for row in rows {
        let codeword = row.codeword_log_reduction.map_or_else(
            || "-".to_owned(),
            |reduction| format!("N/{}", 1usize << reduction),
        );
        writeln!(
            file,
            "| {} | {} | {} | {} | {:.4} | {:.4} | {:.4} | {:.4} |",
            row.scheme,
            codeword,
            row.h.map_or_else(|| "-".to_owned(), |h| h.to_string()),
            row.repetitions,
            row.mean_seconds,
            row.median_seconds,
            row.min_seconds,
            row.max_seconds,
        )?;
    }
    writeln!(file, "\n## Mean stage breakdown\n")?;
    writeln!(
        file,
        "Stage percentages use mean stage time divided by mean total time, so they add to 100% within rounding.\n"
    )?;
    writeln!(
        file,
        "| scheme | codeword | h | stage | work | mean s | median s | share |"
    )?;
    writeln!(file, "|---|---|---:|---|---:|---:|---:|---:|")?;
    for row in stage_rows {
        let codeword = row.codeword_log_reduction.map_or_else(
            || "-".to_owned(),
            |reduction| format!("N/{}", 1usize << reduction),
        );
        writeln!(
            file,
            "| {} | {} | {} | {} | {} | {:.4} | {:.4} | {:.1}% |",
            row.scheme,
            codeword,
            row.h.map_or_else(|| "-".to_owned(), |h| h.to_string()),
            row.stage,
            row.work_items.map_or_else(
                || "-".to_owned(),
                |items| format!("{items} {}", row.work_unit),
            ),
            row.mean_seconds,
            row.median_seconds,
            row.percent_of_mean_total,
        )?;
    }
    Ok(())
}

#[derive(Clone, Debug, Serialize)]
pub struct TimingRow {
    pub log_length: usize,
    pub vector_length: usize,
    pub batch_size: usize,
    pub scheme: String,
    pub h: Option<usize>,
    pub commit_seconds: f64,
    pub prover_open_milliseconds: f64,
    pub verifier_milliseconds: f64,
    pub proof_bytes: usize,
    pub note: String,
}

fn mean_duration(durations: &[Duration]) -> Duration {
    Duration::from_secs_f64(
        durations.iter().map(Duration::as_secs_f64).sum::<f64>() / durations.len() as f64,
    )
}

/// Run real commit/open/verify timings. Canopy timings cover the implemented
/// bottom-tree interaction and SEP reduction; WHIR execution is excluded.
pub fn run_timings(
    log_length: usize,
    batch_sizes: &[usize],
    heights: &[usize],
    repetitions: usize,
    seed: u64,
    output: &Path,
) -> Result<Vec<TimingRow>> {
    ensure!(repetitions > 0, "repetitions must be positive");
    let vector_len = 1usize << log_length;
    let values: Vec<Fr> = (0..vector_len as u64).map(Fr::from).collect();
    let mut rows = Vec::new();

    let start = Instant::now();
    let (merkle_commitment, merkle_data) = MerkleVectorCommitment::commit(&values)?;
    let merkle_commit_time = start.elapsed();
    for &batch_size in batch_sizes {
        let indices = sample_unique_indices(
            vector_len,
            batch_size,
            derive_u64(seed, &[log_length, batch_size, 0x4d45_524b]),
        )?;
        let warmup = MerkleVectorCommitment::open(&merkle_data, &values, &indices)?;
        ensure!(
            MerkleVectorCommitment::verify(&merkle_commitment, &indices, &warmup)?,
            "Merkle warmup proof failed verification"
        );
        let mut prover_times = Vec::with_capacity(repetitions);
        let mut verifier_times = Vec::with_capacity(repetitions);
        let mut last_proof = None;
        for _ in 0..repetitions {
            let start = Instant::now();
            let proof = MerkleVectorCommitment::open(&merkle_data, &values, &indices)?;
            prover_times.push(start.elapsed());
            let start = Instant::now();
            ensure!(
                MerkleVectorCommitment::verify(&merkle_commitment, &indices, &proof)?,
                "Merkle timing proof failed verification"
            );
            verifier_times.push(start.elapsed());
            last_proof = Some(proof);
        }
        let proof = last_proof.unwrap();
        rows.push(TimingRow {
            log_length,
            vector_length: vector_len,
            batch_size,
            scheme: "merkle".to_owned(),
            h: None,
            commit_seconds: merkle_commit_time.as_secs_f64(),
            prover_open_milliseconds: mean_duration(&prover_times).as_secs_f64() * 1_000.0,
            verifier_milliseconds: mean_duration(&verifier_times).as_secs_f64() * 1_000.0,
            proof_bytes: proof.proof_bytes(),
            note: "complete implementation".to_owned(),
        });
    }
    drop(merkle_data);

    for &h in heights {
        let start = Instant::now();
        let (commitment, data) = CanopyVectorCommitment::commit(&values, h)?;
        let commit_time = start.elapsed();
        for &batch_size in batch_sizes {
            let indices = sample_unique_indices(
                vector_len,
                batch_size,
                derive_u64(seed, &[log_length, batch_size, h, 0x4341_4e4f]),
            )?;
            let warmup = CanopyVectorCommitment::open_bottom(&data, &values, &indices)?;
            let _warmup_instance = CanopyVectorCommitment::verify_bottom_and_reduce(
                &commitment,
                &indices,
                &warmup,
                Fr::from(1u64),
            )?;
            let mut prover_times = Vec::with_capacity(repetitions);
            let mut verifier_times = Vec::with_capacity(repetitions);
            let mut last_bottom = None;
            for repetition in 0..repetitions {
                let start = Instant::now();
                let bottom = CanopyVectorCommitment::open_bottom(&data, &values, &indices)?;
                prover_times.push(start.elapsed());
                let start = Instant::now();
                let gamma = Fr::from((repetition + 2) as u64);
                let _instance = CanopyVectorCommitment::verify_bottom_and_reduce(
                    &commitment,
                    &indices,
                    &bottom,
                    gamma,
                )?;
                verifier_times.push(start.elapsed());
                last_bottom = Some(bottom);
            }
            let bottom = last_bottom.unwrap();
            let params = WhirParameters::johnson_bound(log_length - h, h)?;
            let whir = estimate_proof(&params, expand_seed(derive_u64(seed, &[log_length, h])))?;
            let proof_bytes = bottom.hash_bytes() + bottom.field_bytes() + whir.total_bytes();
            rows.push(TimingRow {
                log_length,
                vector_length: vector_len,
                batch_size,
                scheme: "canopy_frontend".to_owned(),
                h: Some(h),
                commit_seconds: commit_time.as_secs_f64(),
                prover_open_milliseconds: mean_duration(&prover_times).as_secs_f64() * 1_000.0,
                verifier_milliseconds: mean_duration(&verifier_times).as_secs_f64() * 1_000.0,
                proof_bytes,
                note: format!(
                    "commit is complete; open/verify timings exclude WHIR; code leaves contain {WHIR_LEAF_WIDTH} fields"
                ),
            });
        }
        drop(data);
    }

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    write_csv(output, &rows)?;
    write_timing_markdown(output.with_extension("md"), &rows, log_length, repetitions)?;
    Ok(rows)
}

fn write_timing_markdown(
    path: impl AsRef<Path>,
    rows: &[TimingRow],
    log_length: usize,
    repetitions: usize,
) -> Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "# Measured timings at 2^{log_length}\n")?;
    writeln!(
        file,
        "Open and verify values are means of {repetitions} repetitions. Merkle rows are end-to-end. Canopy commitment is complete, but Canopy open/verify rows measure only the bottom-tree opening and committed-SEP reduction; WHIR execution is excluded.\n"
    )?;
    writeln!(
        file,
        "Canopy proof estimates use {}-bit WHIR security and terminate recursive folding once message length is at most `2^{}`; all folds use `2^{}`.\n",
        DEFAULT_SECURITY_BITS, MAX_TERMINAL_MESSAGE_VARIABLES, DEFAULT_FOLDING_LOG
    )?;
    writeln!(
        file,
        "Original-vector and WHIR field elements use a {}-bit field and occupy {} bytes.\n",
        crate::whir::DEFAULT_FIELD_BITS,
        FIELD_BYTES
    )?;
    writeln!(
        file,
        "No proof of work is used. WHIR uses the proved Johnson-bound regime over the 256-bit field.\n"
    )?;
    writeln!(
        file,
        "| scheme | h | batch | commit s | prover/open ms | verifier ms | estimated proof KiB |"
    )?;
    writeln!(file, "|---|---:|---:|---:|---:|---:|---:|")?;
    for row in rows {
        writeln!(
            file,
            "| {} | {} | {} | {:.3} | {:.3} | {:.3} | {:.2} |",
            row.scheme,
            row.h.map_or_else(|| "-".to_owned(), |h| h.to_string()),
            row.batch_size,
            row.commit_seconds,
            row.prover_open_milliseconds,
            row.verifier_milliseconds,
            row.proof_bytes as f64 / 1024.0,
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimized_schedules_stop_at_the_terminal_cap() {
        let schedules = folding_schedules_up_to(22, 10, &[3, 4]);
        assert!(!schedules.is_empty());
        for schedule in schedules {
            let folded = schedule.iter().sum::<usize>();
            assert!(22 - folded <= 10);
            assert!(22 - (folded - schedule.last().unwrap()) > 10);
        }
        assert_eq!(folding_schedules_up_to(10, 10, &[3, 4]), vec![vec![]]);
    }

    #[test]
    fn small_matrix_writes_all_outputs() {
        let temp = tempfile::tempdir().unwrap();
        let config = SizeMatrixConfig {
            log_lengths: vec![20],
            batch_sizes: vec![8],
            heights: vec![4, 5],
            layer_field_bits: vec![128],
            codeword_log_reductions: vec![0, 1, 2],
            folding_log: DEFAULT_FOLDING_LOG,
            folding_choices: Vec::new(),
            optimize_folding_schedule: false,
            samples: 3,
            seed: 1,
        };
        let rows = run_size_matrix(&config, temp.path()).unwrap();
        assert_eq!(rows.len(), 1);
        assert!(temp.path().join("proof_sizes.csv").exists());
        assert!(temp.path().join("best_h.csv").exists());
        assert!(temp.path().join("best_h_by_field.csv").exists());
        assert!(temp.path().join("field_profiles.csv").exists());
        assert!(temp.path().join("whir_parameters.csv").exists());
        assert!(temp.path().join("summary.md").exists());
    }

    #[test]
    fn default_height_grid_covers_all_requested_heights() {
        assert_eq!(
            default_heights_for_batch(128).unwrap(),
            (1..=15).collect::<Vec<_>>()
        );

        let config = SizeMatrixConfig::default();
        // The initial four-variable fold is still valid when the message is
        // already at or below the terminal threshold.
        assert_eq!(
            height_candidates(&config, 22, 64, MAX_TERMINAL_MESSAGE_VARIABLES).unwrap(),
            (1..=15).collect::<Vec<_>>()
        );
    }
}
