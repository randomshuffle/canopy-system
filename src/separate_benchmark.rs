//! Experiment for batching openings of `t` independently committed vectors.
//!
//! Merkle uses `t` separate trees and authentication paths. Canopy also uses
//! `t` separate bottom forests and codeword trees, but batches their aligned
//! SEP claims with independent random linear combinations. The legacy model
//! commits the combined codewords and checks them against the old oracles. The
//! virtual-initial-oracle model instead derives WHIR's first queried values
//! directly from the old commitments and starts committing only at WHIR's
//! first recursive oracle.

use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{ensure, Context, Result};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rayon::prelude::*;
use serde::Serialize;

use crate::benchmark::{
    default_heights_for_batch, sample_unique_indices, DEFAULT_BATCH_SIZES, DEFAULT_LOG_LENGTHS,
};
use crate::canopy::{CanopyVectorCommitment, WHIR_LEAF_WIDTH};
use crate::merkle::{frontier_hash_count, MerkleLevels, MerkleVectorCommitment};
use crate::whir::{
    estimate_parallel_interleaved_proofs, estimate_parallel_virtual_initial_proofs, expand_seed,
    field_profile, WhirFieldProfile, WhirParameters, DEFAULT_FIELD_BITS, DEFAULT_FOLDING_FACTOR,
    DEFAULT_FOLDING_LOG, DEFAULT_SECURITY_BITS, MAX_TERMINAL_MESSAGE_VARIABLES,
};
use crate::{Fr, FIELD_BITS, FIELD_BYTES, HASH_BYTES};

pub const DEFAULT_COMMITMENT_COUNTS: &[usize] = &[8, 16, 32];

#[derive(Clone, Debug)]
pub struct SeparateSizeMatrixConfig {
    pub log_lengths: Vec<usize>,
    pub batch_sizes: Vec<usize>,
    pub heights: Vec<usize>,
    pub commitment_counts: Vec<usize>,
    pub layer_field_bits: Vec<usize>,
    pub codeword_log_reductions: Vec<usize>,
    pub rs_interleaving_factors: Vec<usize>,
    pub virtual_initial_oracle: bool,
    pub samples: usize,
    pub seed: u64,
}

impl Default for SeparateSizeMatrixConfig {
    fn default() -> Self {
        Self {
            log_lengths: DEFAULT_LOG_LENGTHS.to_vec(),
            batch_sizes: DEFAULT_BATCH_SIZES.to_vec(),
            heights: Vec::new(),
            commitment_counts: DEFAULT_COMMITMENT_COUNTS.to_vec(),
            layer_field_bits: vec![256],
            codeword_log_reductions: vec![0],
            rs_interleaving_factors: vec![1],
            virtual_initial_oracle: false,
            samples: 100,
            seed: 0x5345_5041_5241_5445,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SeparateProofSizeRow {
    pub log_length: usize,
    pub vector_length: u64,
    pub commitment_count: usize,
    pub batch_size_per_commitment: usize,
    pub total_opened_values: usize,
    pub scheme: String,
    pub virtual_initial_oracle: bool,
    pub h: Option<usize>,
    pub codeword_log_reduction: Option<usize>,
    pub rs_length_log: Option<usize>,
    pub rs_interleaving_factor: Option<usize>,
    pub rs_block_length_log: Option<usize>,
    pub input_field_bits: usize,
    pub layer_field_bits: Option<usize>,
    pub layer_field_bytes: Option<usize>,
    pub parallel_linear_combinations: usize,
    pub parallel_whir_instances: usize,
    pub batching_soundness_bits: Option<f64>,
    pub consistency_queries: Option<usize>,
    pub minimum_relative_distance: Option<f64>,
    pub virtual_initial_queries: Option<usize>,
    pub samples: usize,
    pub mean_opening_auth_hashes: f64,
    pub mean_virtual_initial_auth_hashes: f64,
    pub mean_old_oracle_consistency_auth_hashes: f64,
    pub mean_combined_and_whir_hashes: f64,
    pub mean_hash_count: f64,
    pub mean_hash_bytes: f64,
    pub mean_input_field_count: f64,
    pub mean_input_field_bytes: f64,
    pub mean_virtual_initial_field_count: f64,
    pub mean_virtual_initial_field_bytes: f64,
    pub mean_old_oracle_consistency_field_count: f64,
    pub mean_old_oracle_consistency_field_bytes: f64,
    pub mean_new_oracle_consistency_field_count: f64,
    pub mean_new_oracle_consistency_field_bytes: f64,
    pub mean_consistency_field_count: f64,
    pub mean_consistency_field_bytes: f64,
    pub mean_whir_field_count: f64,
    pub mean_whir_field_bytes: f64,
    pub mean_field_bytes: f64,
    pub mean_total_bytes: f64,
    pub min_total_bytes: usize,
    pub max_total_bytes: usize,
    pub mean_total_kib: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct SeparateBestHeightRow {
    pub log_length: usize,
    pub vector_length: u64,
    pub commitment_count: usize,
    pub batch_size_per_commitment: usize,
    pub total_opened_values: usize,
    pub h_candidates: String,
    pub merkle_mean_bytes: f64,
    pub best_layer_field_bits: usize,
    pub codeword_log_reduction: usize,
    pub rs_length_log: usize,
    pub rs_interleaving_factor: usize,
    pub rs_block_length_log: usize,
    pub parallel_linear_combinations: usize,
    pub parallel_whir_instances: usize,
    pub batching_soundness_bits: f64,
    pub consistency_queries: usize,
    pub virtual_initial_queries: usize,
    pub best_h: usize,
    pub canopy_mean_bytes: f64,
    pub canopy_hash_bytes: f64,
    pub canopy_field_bytes: f64,
    pub canopy_over_merkle: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct SeparateBestByFieldRow {
    pub log_length: usize,
    pub vector_length: u64,
    pub commitment_count: usize,
    pub batch_size_per_commitment: usize,
    pub total_opened_values: usize,
    pub layer_field_bits: usize,
    pub layer_field_log2_size: f64,
    pub codeword_log_reduction: usize,
    pub rs_length_log: usize,
    pub rs_interleaving_factor: usize,
    pub rs_block_length_log: usize,
    pub parallel_linear_combinations: usize,
    pub parallel_whir_instances: usize,
    pub batching_soundness_bits: f64,
    pub h_candidates: String,
    pub merkle_mean_bytes: f64,
    pub consistency_queries: usize,
    pub virtual_initial_queries: usize,
    pub best_h: usize,
    pub canopy_mean_bytes: f64,
    pub canopy_hash_bytes: f64,
    pub canopy_field_bytes: f64,
    pub canopy_over_merkle: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct SeparateFieldProfileRow {
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

#[derive(Clone, Debug, Serialize)]
pub struct SeparateProtocolParameterRow {
    pub log_length: usize,
    pub h: usize,
    pub codeword_log_reduction: usize,
    pub rs_length_log: usize,
    pub rs_interleaving_factor: usize,
    pub rs_block_length_log: usize,
    pub layer_field_bits: usize,
    pub layer_field_log2_size: f64,
    pub parallel_linear_combinations: usize,
    pub parallel_whir_instances: usize,
    pub batching_soundness_bits: f64,
    pub virtual_initial_oracle: bool,
    pub minimum_relative_distance: Option<f64>,
    pub consistency_queries: usize,
    pub virtual_initial_queries: usize,
    pub effective_initial_folding_factor: usize,
    pub additional_initial_whir_folding_factor: usize,
    pub recursive_whir_folding_factor: usize,
    pub whir_message_variables: usize,
    pub whir_log_inverse_rate: usize,
    pub whir_requested_queries_by_oracle: String,
    pub whir_ood_samples_by_commitment: String,
    pub whir_sumcheck_repetitions: String,
}

#[derive(Default)]
struct ProofSamples {
    opening_auth_hashes: Vec<usize>,
    virtual_initial_auth_hashes: Vec<usize>,
    old_consistency_auth_hashes: Vec<usize>,
    combined_and_whir_hashes: Vec<usize>,
    input_fields: Vec<usize>,
    virtual_initial_fields: Vec<usize>,
    old_consistency_fields: Vec<usize>,
    new_consistency_fields: Vec<usize>,
    whir_fields: Vec<usize>,
    total_bytes: Vec<usize>,
}

struct ProofSample {
    opening_auth_hashes: usize,
    virtual_initial_auth_hashes: usize,
    old_consistency_auth_hashes: usize,
    combined_and_whir_hashes: usize,
    input_fields: usize,
    virtual_initial_fields: usize,
    old_consistency_fields: usize,
    new_consistency_fields: usize,
    whir_fields: usize,
    layer_field_bytes: usize,
}

impl ProofSamples {
    fn push(&mut self, sample: ProofSample) {
        let hash_count = sample.opening_auth_hashes
            + sample.virtual_initial_auth_hashes
            + sample.old_consistency_auth_hashes
            + sample.combined_and_whir_hashes;
        let field_bytes = sample.input_fields * FIELD_BYTES
            + (sample.virtual_initial_fields
                + sample.old_consistency_fields
                + sample.new_consistency_fields
                + sample.whir_fields)
                * sample.layer_field_bytes;
        self.opening_auth_hashes.push(sample.opening_auth_hashes);
        self.virtual_initial_auth_hashes
            .push(sample.virtual_initial_auth_hashes);
        self.old_consistency_auth_hashes
            .push(sample.old_consistency_auth_hashes);
        self.combined_and_whir_hashes
            .push(sample.combined_and_whir_hashes);
        self.input_fields.push(sample.input_fields);
        self.virtual_initial_fields
            .push(sample.virtual_initial_fields);
        self.old_consistency_fields
            .push(sample.old_consistency_fields);
        self.new_consistency_fields
            .push(sample.new_consistency_fields);
        self.whir_fields.push(sample.whir_fields);
        self.total_bytes.push(hash_count * HASH_BYTES + field_bytes);
    }

    fn row(
        &self,
        log_length: usize,
        commitment_count: usize,
        batch_size: usize,
        scheme: &str,
        canopy: Option<(usize, usize, usize, bool)>,
        field: Option<WhirFieldProfile>,
    ) -> SeparateProofSizeRow {
        let mean = |xs: &[usize]| xs.iter().sum::<usize>() as f64 / xs.len() as f64;
        let opening_hashes = mean(&self.opening_auth_hashes);
        let virtual_initial_hashes = mean(&self.virtual_initial_auth_hashes);
        let consistency_hashes = mean(&self.old_consistency_auth_hashes);
        let protocol_hashes = mean(&self.combined_and_whir_hashes);
        let hash_count =
            opening_hashes + virtual_initial_hashes + consistency_hashes + protocol_hashes;
        let input_fields = mean(&self.input_fields);
        let virtual_initial_fields = mean(&self.virtual_initial_fields);
        let old_consistency_fields = mean(&self.old_consistency_fields);
        let new_consistency_fields = mean(&self.new_consistency_fields);
        let consistency_fields = old_consistency_fields + new_consistency_fields;
        let whir_fields = mean(&self.whir_fields);
        let layer_bytes = field.map_or(0, |profile| profile.element_bytes);
        let field_bytes = input_fields * FIELD_BYTES as f64
            + (virtual_initial_fields + consistency_fields + whir_fields) * layer_bytes as f64;
        let total_bytes = mean(&self.total_bytes);
        let parallel_combinations = field.map_or(0, linear_combination_repetitions);
        let h = canopy.map(|(height, _, _, _)| height);
        let codeword_log_reduction = canopy.map(|(_, reduction, _, _)| reduction);
        let rs_interleaving_factor = canopy.map(|(_, _, factor, _)| factor);
        let virtual_initial_oracle =
            canopy.is_some_and(|(_, _, _, virtual_initial)| virtual_initial);
        let virtual_initial_queries = if virtual_initial_oracle {
            let height = h.unwrap();
            let reduction = codeword_log_reduction.unwrap();
            let factor = rs_interleaving_factor.unwrap();
            let profile = field.unwrap();
            let params = WhirParameters::johnson_bound_with_field(
                log_length - height - factor.ilog2() as usize,
                height - reduction,
                profile,
            )
            .unwrap();
            Some(params.all_query_counts()[0])
        } else {
            None
        };
        SeparateProofSizeRow {
            log_length,
            vector_length: 1u64 << log_length,
            commitment_count,
            batch_size_per_commitment: batch_size,
            total_opened_values: commitment_count * batch_size,
            scheme: scheme.to_owned(),
            virtual_initial_oracle,
            h,
            codeword_log_reduction,
            rs_length_log: codeword_log_reduction.map(|reduction| log_length - reduction),
            rs_interleaving_factor,
            rs_block_length_log: codeword_log_reduction
                .zip(rs_interleaving_factor)
                .map(|(reduction, factor)| log_length - reduction - factor.ilog2() as usize),
            input_field_bits: FIELD_BITS,
            layer_field_bits: field.map(|profile| profile.bits),
            layer_field_bytes: field.map(|profile| profile.element_bytes),
            parallel_linear_combinations: parallel_combinations,
            parallel_whir_instances: parallel_combinations,
            batching_soundness_bits: field
                .map(|profile| parallel_combinations as f64 * profile.log2_size),
            consistency_queries: if virtual_initial_oracle {
                None
            } else {
                h.zip(codeword_log_reduction)
                    .map(|(height, reduction)| consistency_query_count(height - reduction))
            },
            minimum_relative_distance: if virtual_initial_oracle {
                None
            } else {
                h.zip(codeword_log_reduction)
                    .map(|(height, reduction)| minimum_relative_distance(height - reduction))
            },
            virtual_initial_queries,
            samples: self.total_bytes.len(),
            mean_opening_auth_hashes: opening_hashes,
            mean_virtual_initial_auth_hashes: virtual_initial_hashes,
            mean_old_oracle_consistency_auth_hashes: consistency_hashes,
            mean_combined_and_whir_hashes: protocol_hashes,
            mean_hash_count: hash_count,
            mean_hash_bytes: hash_count * HASH_BYTES as f64,
            mean_input_field_count: input_fields,
            mean_input_field_bytes: input_fields * FIELD_BYTES as f64,
            mean_virtual_initial_field_count: virtual_initial_fields,
            mean_virtual_initial_field_bytes: virtual_initial_fields * layer_bytes as f64,
            mean_old_oracle_consistency_field_count: old_consistency_fields,
            mean_old_oracle_consistency_field_bytes: old_consistency_fields * layer_bytes as f64,
            mean_new_oracle_consistency_field_count: new_consistency_fields,
            mean_new_oracle_consistency_field_bytes: new_consistency_fields * layer_bytes as f64,
            mean_consistency_field_count: consistency_fields,
            mean_consistency_field_bytes: consistency_fields * layer_bytes as f64,
            mean_whir_field_count: whir_fields,
            mean_whir_field_bytes: whir_fields * layer_bytes as f64,
            mean_field_bytes: field_bytes,
            mean_total_bytes: total_bytes,
            min_total_bytes: *self.total_bytes.iter().min().unwrap(),
            max_total_bytes: *self.total_bytes.iter().max().unwrap(),
            mean_total_kib: total_bytes / 1024.0,
        }
    }
}

fn derive_u64(seed: u64, words: &[usize]) -> u64 {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"canopy-system/separate-commitment-benchmark-seed/v1");
    hasher.update(&seed.to_le_bytes());
    for &word in words {
        hasher.update(&(word as u64).to_le_bytes());
    }
    let digest = hasher.finalize();
    u64::from_le_bytes(digest.as_bytes()[..8].try_into().unwrap())
}

fn linear_combination_repetitions(field: WhirFieldProfile) -> usize {
    (DEFAULT_SECURITY_BITS as f64 / field.log2_size).ceil() as usize
}

fn canopy_scheme(
    codeword_log_reduction: usize,
    rs_interleaving_factor: usize,
    virtual_initial_oracle: bool,
) -> &'static str {
    if virtual_initial_oracle {
        return "canopy_virtual_initial_whir";
    }
    match (codeword_log_reduction, rs_interleaving_factor) {
        (0, 1) => "canopy_batched_whir",
        (1, 1) => "canopy_half_batched_whir",
        (2, 1) => "canopy_quarter_batched_whir",
        (2, 8) => "canopy_quarter_rs8_batched_whir",
        (2, 16) => "canopy_quarter_rs16_batched_whir",
        (_, 1) => "canopy_shortened_batched_whir",
        _ => "canopy_interleaved_rs_batched_whir",
    }
}

fn minimum_relative_distance(h: usize) -> f64 {
    1.0 - 2.0_f64.powi(-(h as i32))
}

fn consistency_query_count(h: usize) -> usize {
    // A wrong codeword differs from the intended linear combination on at
    // least 1-rho positions. Missing all disagreements has probability at
    // most rho^q = 2^(-h*q).
    DEFAULT_SECURITY_BITS.div_ceil(h)
}

fn height_candidates(
    requested: &[usize],
    log_length: usize,
    batch_size: usize,
) -> Result<Vec<usize>> {
    let heights = if requested.is_empty() {
        default_heights_for_batch(batch_size)?
    } else {
        requested.to_vec()
    };
    let heights: Vec<_> = heights
        .into_iter()
        .filter(|&h| h < log_length && log_length - h >= DEFAULT_FOLDING_LOG)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    ensure!(
        !heights.is_empty(),
        "no valid h candidate for log length {log_length} and batch size {batch_size}"
    );
    Ok(heights)
}

#[derive(Clone, Copy)]
struct CanopySampleContext {
    log_length: usize,
    commitment_count: usize,
    batch_size: usize,
    h: usize,
    codeword_log_reduction: usize,
    rs_interleaving_factor: usize,
    field: WhirFieldProfile,
    virtual_initial_oracle: bool,
}

fn estimate_canopy_sample(
    context: CanopySampleContext,
    bottom_indices: &[usize],
    consistency_queries: &[usize],
    whir_seed: u64,
) -> Result<ProofSample> {
    let vector_len = 1usize << context.log_length;
    let codeword_len = vector_len >> context.codeword_log_reduction;
    let interleaving_log = context.rs_interleaving_factor.ilog2() as usize;
    let block_len = codeword_len / context.rs_interleaving_factor;
    let bottom_hashes = frontier_hash_count(bottom_indices, vector_len, context.h)?;
    let params = WhirParameters::johnson_bound_with_field(
        context.log_length - context.h - interleaving_log,
        context.h - context.codeword_log_reduction,
        context.field,
    )?;
    let combinations = linear_combination_repetitions(context.field);

    if context.virtual_initial_oracle {
        ensure!(
            context.rs_interleaving_factor == DEFAULT_FOLDING_FACTOR,
            "the virtual initial oracle requires RS interleaving to equal WHIR's initial folding factor"
        );
        ensure!(
            consistency_queries.is_empty(),
            "the virtual initial oracle does not use a standalone consistency check"
        );
        let virtual_whir = estimate_parallel_virtual_initial_proofs(
            &params,
            combinations,
            expand_seed(whir_seed),
        )?;
        let virtual_auth_hashes = frontier_hash_count(
            &virtual_whir.initial_query_indices,
            block_len,
            block_len.ilog2() as usize,
        )?;

        return Ok(ProofSample {
            opening_auth_hashes: context.commitment_count * bottom_hashes,
            virtual_initial_auth_hashes: context.commitment_count * virtual_auth_hashes,
            old_consistency_auth_hashes: 0,
            // The initial RLC oracle is virtual. Only recursive WHIR roots and
            // their authentication frontiers are serialized.
            combined_and_whir_hashes: virtual_whir.proof.hash_count,
            input_fields: context.commitment_count * context.batch_size,
            // One queried coordinate opens the complete interleaving column
            // in every old commitment; all parallel RLC values are derived.
            virtual_initial_fields: context.commitment_count
                * context.rs_interleaving_factor
                * virtual_whir.initial_query_indices.len(),
            old_consistency_fields: 0,
            new_consistency_fields: 0,
            whir_fields: virtual_whir.proof.field_count,
            layer_field_bytes: context.field.element_bytes,
        });
    }

    let (
        old_consistency_hashes,
        combined_leaf_indices,
        old_consistency_fields,
        new_consistency_fields,
    ) = if context.rs_interleaving_factor == 1 {
        let leaf_count = block_len / WHIR_LEAF_WIDTH;
        (
            frontier_hash_count(consistency_queries, leaf_count, leaf_count.ilog2() as usize)?,
            consistency_queries.to_vec(),
            context.commitment_count * consistency_queries.len() * WHIR_LEAF_WIDTH,
            0,
        )
    } else {
        let combined_leaf_indices: Vec<_> = consistency_queries
            .iter()
            .map(|&coordinate| coordinate / WHIR_LEAF_WIDTH)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let old_fields =
            context.commitment_count * context.rs_interleaving_factor * consistency_queries.len();
        let new_fields = combinations
            * (combined_leaf_indices.len() * WHIR_LEAF_WIDTH - consistency_queries.len());
        (
            frontier_hash_count(consistency_queries, block_len, block_len.ilog2() as usize)?,
            combined_leaf_indices,
            old_fields,
            new_fields,
        )
    };
    let whir = estimate_parallel_interleaved_proofs(
        &params,
        combinations,
        &combined_leaf_indices,
        expand_seed(whir_seed),
    )?;

    Ok(ProofSample {
        opening_auth_hashes: context.commitment_count * bottom_hashes,
        virtual_initial_auth_hashes: 0,
        old_consistency_auth_hashes: context.commitment_count * old_consistency_hashes,
        // The challenge-dependent combined-oracle root is a prover message.
        // WHIR counts its recursive roots and merged Merkle frontiers.
        combined_and_whir_hashes: 1 + whir.hash_count,
        input_fields: context.commitment_count * context.batch_size,
        virtual_initial_fields: 0,
        // One coordinate opens all stripes in every old commitment. At the
        // corresponding packed new-oracle leaf, the checked values are
        // verifier-derived and only the unchecked offsets are serialized.
        old_consistency_fields,
        new_consistency_fields,
        whir_fields: whir.field_count,
        layer_field_bytes: context.field.element_bytes,
    })
}

pub fn run_separate_size_matrix(
    config: &SeparateSizeMatrixConfig,
    output_dir: &Path,
) -> Result<Vec<SeparateBestHeightRow>> {
    ensure!(config.samples > 0, "samples must be positive");
    ensure!(
        !config.commitment_counts.is_empty()
            && config.commitment_counts.iter().all(|&count| count > 0),
        "commitment counts must be positive"
    );
    ensure!(
        !config.layer_field_bits.is_empty(),
        "at least one Canopy-layer field size is required"
    );
    ensure!(
        !config.codeword_log_reductions.is_empty(),
        "at least one codeword length is required"
    );
    ensure!(
        !config.rs_interleaving_factors.is_empty()
            && config
                .rs_interleaving_factors
                .iter()
                .all(|factor| factor.is_power_of_two()),
        "RS interleaving factors must be nonzero powers of two"
    );
    if config.virtual_initial_oracle {
        ensure!(
            config
                .rs_interleaving_factors
                .iter()
                .all(|&factor| factor == DEFAULT_FOLDING_FACTOR),
            "the virtual initial oracle currently requires RS interleaving factor {}",
            DEFAULT_FOLDING_FACTOR,
        );
    }
    fs::create_dir_all(output_dir).with_context(|| format!("creating {}", output_dir.display()))?;

    let mut proof_rows = Vec::new();
    let mut best_rows = Vec::new();
    let mut best_by_field_rows = Vec::new();
    let mut field_rows = Vec::new();
    let mut protocol_rows = Vec::new();

    for &n in config.log_lengths.iter().collect::<BTreeSet<_>>() {
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
        let interleavings: Vec<_> = config
            .rs_interleaving_factors
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        ensure!(
            reductions.iter().all(|&reduction| interleavings
                .iter()
                .all(|&factor| factor.ilog2() as usize + reduction < n)),
            "RS interleaving must leave every block nonempty"
        );
        let required_codeword_log = reductions
            .iter()
            .flat_map(|&reduction| {
                interleavings
                    .iter()
                    .map(move |&factor| n - reduction - factor.ilog2() as usize)
            })
            .max()
            .unwrap();
        let mut fields = Vec::new();
        for bits in config
            .layer_field_bits
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
        {
            match field_profile(bits, required_codeword_log) {
                Ok(field) => {
                    field_rows.push(SeparateFieldProfileRow {
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
                            "supports the largest requested 2^{required_codeword_log} {} codeword domain",
                            field.domain_kind,
                        ),
                    });
                    fields.push(field);
                }
                Err(error) => field_rows.push(SeparateFieldProfileRow {
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
            !fields.is_empty(),
            "none of the requested Canopy-layer fields is feasible for 2^{n}"
        );

        let all_heights: BTreeSet<_> = config
            .batch_sizes
            .iter()
            .map(|&batch| height_candidates(&config.heights, n, batch))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();
        for &field in &fields {
            for &reduction in &reductions {
                for &interleaving in &interleavings {
                    let interleaving_log = interleaving.ilog2() as usize;
                    for &h in all_heights.iter().filter(|&&h| {
                        h > reduction && n - h >= interleaving_log + DEFAULT_FOLDING_LOG
                    }) {
                        let rate_log = h - reduction;
                        let message_variables = n - h - interleaving_log;
                        let params = WhirParameters::johnson_bound_with_field(
                            message_variables,
                            rate_log,
                            field,
                        )?;
                        let combinations = linear_combination_repetitions(field);
                        protocol_rows.push(SeparateProtocolParameterRow {
                            log_length: n,
                            h,
                            codeword_log_reduction: reduction,
                            rs_length_log: n - reduction,
                            rs_interleaving_factor: interleaving,
                            rs_block_length_log: n - reduction - interleaving_log,
                            layer_field_bits: field.bits,
                            layer_field_log2_size: field.log2_size,
                            parallel_linear_combinations: combinations,
                            parallel_whir_instances: combinations,
                            batching_soundness_bits: combinations as f64 * field.log2_size,
                            virtual_initial_oracle: config.virtual_initial_oracle,
                            minimum_relative_distance: (!config.virtual_initial_oracle)
                                .then(|| minimum_relative_distance(rate_log)),
                            consistency_queries: if config.virtual_initial_oracle {
                                0
                            } else {
                                consistency_query_count(rate_log)
                            },
                            virtual_initial_queries: if config.virtual_initial_oracle {
                                params.all_query_counts()[0]
                            } else {
                                0
                            },
                            effective_initial_folding_factor: if config.virtual_initial_oracle {
                                interleaving
                            } else {
                                DEFAULT_FOLDING_FACTOR
                            },
                            additional_initial_whir_folding_factor: if config.virtual_initial_oracle
                            {
                                1
                            } else {
                                DEFAULT_FOLDING_FACTOR
                            },
                            recursive_whir_folding_factor: DEFAULT_FOLDING_FACTOR,
                            whir_message_variables: message_variables,
                            whir_log_inverse_rate: rate_log,
                            whir_requested_queries_by_oracle: params
                                .all_query_counts()
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(";"),
                            whir_ood_samples_by_commitment: params
                                .all_ood_counts()
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(";"),
                            whir_sumcheck_repetitions: params
                                .all_sumcheck_repetitions()
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(";"),
                        });
                    }
                }
            }
        }

        for &batch_size in &config.batch_sizes {
            ensure!(batch_size <= vector_len, "batch exceeds vector length");
            let heights = height_candidates(&config.heights, n, batch_size)?;

            for &commitment_count in config.commitment_counts.iter().collect::<BTreeSet<_>>() {
                let mut merkle_samples = ProofSamples::default();
                let mut canopy_samples = Vec::new();
                for &field in &fields {
                    for &reduction in &reductions {
                        for &interleaving in &interleavings {
                            let interleaving_log = interleaving.ilog2() as usize;
                            for &h in heights.iter().filter(|&&h| {
                                h > reduction && n - h >= interleaving_log + DEFAULT_FOLDING_LOG
                            }) {
                                canopy_samples.push((
                                    field,
                                    reduction,
                                    interleaving,
                                    h,
                                    ProofSamples::default(),
                                ));
                            }
                        }
                    }
                }

                for sample in 0..config.samples {
                    // All t commitments use the same logical batch so their
                    // SEP functionals align, but each tree sends its own path.
                    let bottom_indices = sample_unique_indices(
                        vector_len,
                        batch_size,
                        derive_u64(config.seed, &[n, batch_size, sample]),
                    )?;
                    let merkle_hashes = frontier_hash_count(&bottom_indices, vector_len, n)?;
                    merkle_samples.push(ProofSample {
                        opening_auth_hashes: commitment_count * merkle_hashes,
                        virtual_initial_auth_hashes: 0,
                        old_consistency_auth_hashes: 0,
                        combined_and_whir_hashes: 0,
                        input_fields: commitment_count * batch_size,
                        virtual_initial_fields: 0,
                        old_consistency_fields: 0,
                        new_consistency_fields: 0,
                        whir_fields: 0,
                        layer_field_bytes: 0,
                    });

                    for (field, reduction, interleaving, h, samples) in &mut canopy_samples {
                        let consistency_queries = if config.virtual_initial_oracle {
                            Vec::new()
                        } else {
                            let q = consistency_query_count(*h - *reduction);
                            let block_len = (vector_len >> *reduction) / *interleaving;
                            let consistency_universe = if *interleaving == 1 {
                                block_len / WHIR_LEAF_WIDTH
                            } else {
                                block_len
                            };
                            sample_unique_indices(
                                consistency_universe,
                                q,
                                derive_u64(
                                    config.seed,
                                    &[
                                        n,
                                        batch_size,
                                        *reduction,
                                        *interleaving,
                                        *h,
                                        sample,
                                        0x434f_4e53,
                                    ],
                                ),
                            )?
                        };
                        samples.push(estimate_canopy_sample(
                            CanopySampleContext {
                                log_length: n,
                                commitment_count,
                                batch_size,
                                h: *h,
                                codeword_log_reduction: *reduction,
                                rs_interleaving_factor: *interleaving,
                                field: *field,
                                virtual_initial_oracle: config.virtual_initial_oracle,
                            },
                            &bottom_indices,
                            &consistency_queries,
                            derive_u64(
                                config.seed,
                                &[
                                    n,
                                    batch_size,
                                    *reduction,
                                    *interleaving,
                                    *h,
                                    sample,
                                    0x5748_4952,
                                ],
                            ),
                        )?);
                    }
                }

                let merkle_row = merkle_samples.row(
                    n,
                    commitment_count,
                    batch_size,
                    "merkle_separate",
                    None,
                    None,
                );
                proof_rows.push(merkle_row.clone());
                let mut rows_for_batch = Vec::new();
                for (field, reduction, interleaving, h, samples) in &canopy_samples {
                    let scheme =
                        canopy_scheme(*reduction, *interleaving, config.virtual_initial_oracle);
                    let row = samples.row(
                        n,
                        commitment_count,
                        batch_size,
                        scheme,
                        Some((*h, *reduction, *interleaving, config.virtual_initial_oracle)),
                        Some(*field),
                    );
                    proof_rows.push(row.clone());
                    rows_for_batch.push(row);
                }

                for &field in &fields {
                    for &reduction in &reductions {
                        for &interleaving in &interleavings {
                            let interleaving_log = interleaving.ilog2() as usize;
                            let best = rows_for_batch
                                .iter()
                                .filter(|row| {
                                    row.layer_field_bits == Some(field.bits)
                                        && row.codeword_log_reduction == Some(reduction)
                                        && row.rs_interleaving_factor == Some(interleaving)
                                })
                                .min_by(|a, b| a.mean_total_bytes.total_cmp(&b.mean_total_bytes))
                                .unwrap();
                            best_by_field_rows.push(SeparateBestByFieldRow {
                                log_length: n,
                                vector_length: 1u64 << n,
                                commitment_count,
                                batch_size_per_commitment: batch_size,
                                total_opened_values: commitment_count * batch_size,
                                layer_field_bits: field.bits,
                                layer_field_log2_size: field.log2_size,
                                codeword_log_reduction: reduction,
                                rs_length_log: n - reduction,
                                rs_interleaving_factor: interleaving,
                                rs_block_length_log: n - reduction - interleaving_log,
                                parallel_linear_combinations: best.parallel_linear_combinations,
                                parallel_whir_instances: best.parallel_whir_instances,
                                batching_soundness_bits: best.batching_soundness_bits.unwrap(),
                                h_candidates: heights
                                    .iter()
                                    .filter(|&&h| {
                                        h > reduction
                                            && n - h >= interleaving_log + DEFAULT_FOLDING_LOG
                                    })
                                    .map(ToString::to_string)
                                    .collect::<Vec<_>>()
                                    .join(";"),
                                merkle_mean_bytes: merkle_row.mean_total_bytes,
                                consistency_queries: best.consistency_queries.unwrap_or(0),
                                virtual_initial_queries: best.virtual_initial_queries.unwrap_or(0),
                                best_h: best.h.unwrap(),
                                canopy_mean_bytes: best.mean_total_bytes,
                                canopy_hash_bytes: best.mean_hash_bytes,
                                canopy_field_bytes: best.mean_field_bytes,
                                canopy_over_merkle: best.mean_total_bytes
                                    / merkle_row.mean_total_bytes,
                            });
                        }
                    }
                }

                for &reduction in &reductions {
                    for &interleaving in &interleavings {
                        let interleaving_log = interleaving.ilog2() as usize;
                        let best = rows_for_batch
                            .iter()
                            .filter(|row| {
                                row.codeword_log_reduction == Some(reduction)
                                    && row.rs_interleaving_factor == Some(interleaving)
                            })
                            .min_by(|a, b| a.mean_total_bytes.total_cmp(&b.mean_total_bytes))
                            .unwrap();
                        best_rows.push(SeparateBestHeightRow {
                            log_length: n,
                            vector_length: 1u64 << n,
                            commitment_count,
                            batch_size_per_commitment: batch_size,
                            total_opened_values: commitment_count * batch_size,
                            h_candidates: heights
                                .iter()
                                .filter(|&&h| {
                                    h > reduction && n - h >= interleaving_log + DEFAULT_FOLDING_LOG
                                })
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(";"),
                            merkle_mean_bytes: merkle_row.mean_total_bytes,
                            best_layer_field_bits: best.layer_field_bits.unwrap(),
                            codeword_log_reduction: reduction,
                            rs_length_log: n - reduction,
                            rs_interleaving_factor: interleaving,
                            rs_block_length_log: n - reduction - interleaving_log,
                            parallel_linear_combinations: best.parallel_linear_combinations,
                            parallel_whir_instances: best.parallel_whir_instances,
                            batching_soundness_bits: best.batching_soundness_bits.unwrap(),
                            consistency_queries: best.consistency_queries.unwrap_or(0),
                            virtual_initial_queries: best.virtual_initial_queries.unwrap_or(0),
                            best_h: best.h.unwrap(),
                            canopy_mean_bytes: best.mean_total_bytes,
                            canopy_hash_bytes: best.mean_hash_bytes,
                            canopy_field_bytes: best.mean_field_bytes,
                            canopy_over_merkle: best.mean_total_bytes / merkle_row.mean_total_bytes,
                        });
                    }
                }
            }
        }
    }

    write_csv(output_dir.join("proof_sizes.csv"), &proof_rows)?;
    write_csv(output_dir.join("best_h.csv"), &best_rows)?;
    write_csv(output_dir.join("best_h_by_field.csv"), &best_by_field_rows)?;
    write_csv(output_dir.join("field_profiles.csv"), &field_rows)?;
    write_csv(output_dir.join("protocol_parameters.csv"), &protocol_rows)?;
    write_summary(output_dir.join("summary.md"), config, &best_rows)?;
    Ok(best_rows)
}

#[derive(Clone, Debug)]
pub struct SeparateHeightSearchConfig<'a> {
    pub requested_heights: &'a [usize],
    pub codeword_log_reduction: usize,
    pub layer_field_bits: usize,
    pub samples: usize,
    pub seed: u64,
}

pub fn select_separate_height(
    log_length: usize,
    commitment_count: usize,
    batch_size: usize,
    config: &SeparateHeightSearchConfig<'_>,
) -> Result<usize> {
    ensure!(config.samples > 0, "samples must be positive");
    ensure!(commitment_count > 0, "commitment count must be positive");
    let vector_len = 1usize << log_length;
    ensure!(batch_size <= vector_len, "batch exceeds vector length");
    ensure!(
        config.codeword_log_reduction < log_length,
        "codeword log reduction must be smaller than log length"
    );
    let heights: Vec<_> = height_candidates(config.requested_heights, log_length, batch_size)?
        .into_iter()
        .filter(|&h| h > config.codeword_log_reduction)
        .collect();
    ensure!(
        !heights.is_empty(),
        "no height is larger than the codeword log reduction"
    );
    let field = field_profile(
        config.layer_field_bits,
        log_length - config.codeword_log_reduction,
    )?;
    let mut totals = vec![0f64; heights.len()];

    for sample in 0..config.samples {
        let bottom_indices = sample_unique_indices(
            vector_len,
            batch_size,
            derive_u64(config.seed, &[log_length, batch_size, sample]),
        )?;
        for (offset, &h) in heights.iter().enumerate() {
            let rate_log = h - config.codeword_log_reduction;
            let consistency_queries = sample_unique_indices(
                (vector_len >> config.codeword_log_reduction) / WHIR_LEAF_WIDTH,
                consistency_query_count(rate_log),
                derive_u64(
                    config.seed,
                    &[
                        log_length,
                        batch_size,
                        config.codeword_log_reduction,
                        h,
                        sample,
                        0x434f_4e53,
                    ],
                ),
            )?;
            let estimate = estimate_canopy_sample(
                CanopySampleContext {
                    log_length,
                    commitment_count,
                    batch_size,
                    h,
                    codeword_log_reduction: config.codeword_log_reduction,
                    rs_interleaving_factor: 1,
                    field,
                    virtual_initial_oracle: false,
                },
                &bottom_indices,
                &consistency_queries,
                derive_u64(
                    config.seed,
                    &[
                        log_length,
                        batch_size,
                        config.codeword_log_reduction,
                        h,
                        sample,
                        0x5748_4952,
                    ],
                ),
            )?;
            let hashes = estimate.opening_auth_hashes
                + estimate.old_consistency_auth_hashes
                + estimate.combined_and_whir_hashes;
            totals[offset] += (hashes * HASH_BYTES
                + estimate.input_fields * FIELD_BYTES
                + (estimate.old_consistency_fields
                    + estimate.new_consistency_fields
                    + estimate.whir_fields)
                    * field.element_bytes) as f64;
        }
    }

    Ok(heights
        .into_iter()
        .zip(totals)
        .min_by(|(h_a, total_a), (h_b, total_b)| {
            total_a.total_cmp(total_b).then_with(|| h_a.cmp(h_b))
        })
        .unwrap()
        .0)
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

fn write_summary(
    path: impl AsRef<Path>,
    config: &SeparateSizeMatrixConfig,
    rows: &[SeparateBestHeightRow],
) -> Result<()> {
    let mut file = File::create(path)?;
    writeln!(
        file,
        "# Separate-commitment Canopy vs. Merkle proof sizes\n"
    )?;
    if config.virtual_initial_oracle {
        writeln!(
            file,
            "Each of the `t` vectors has a separate commitment and separate Merkle path. All commitments open the same `B` logical indices and use the same SEP-reduction challenge so their functionals align. The old codeword roots jointly bind a virtual initial WHIR oracle: at each initial WHIR query, the prover opens one width-16 interleaving column in every old commitment and the verifier derives all random-linear-combination values locally. There is no separate combined-oracle commitment or standalone consistency check.\n"
        )?;
        writeln!(
            file,
            "The number of independent linear combinations is `ceil(128 / log2(|F|))`, providing at least 128 batching-soundness bits for each requested field. RS interleaving factor 16 supplies the effective first width-16 fold, so the additional initial WHIR folding factor is one; recursive folds remain width 16. The initial old-oracle openings use WHIR's proved Johnson-bound query count, all parallel instances share those openings, and only recursive WHIR roots and paths are committed.\n"
        )?;
    } else {
        writeln!(
            file,
            "Each of the `t` vectors has a separate commitment and separate Merkle path. All commitments open the same `B` logical indices and use the same SEP-reduction challenge so their functionals align. Canopy replaces `t` independent WHIR executions with independently randomized linear combinations, interleaves the resulting codewords in one commitment, checks that commitment against all old codeword commitments, and runs parallel WHIR instances with shared oracle paths.\n"
        )?;
        writeln!(
            file,
            "The number of linear combinations is `ceil(128 / log2(|F|))`. If the RS interleaving factor is `s`, each total codeword is split into `s` equal blocks. The random batching challenge combines all `s*t` stripe claims into that same number of scalar WHIR instances; their committed oracles and paths are interleaved. For codeword log reduction `r`, consistency uses the RS minimum distance `1-2^-(h-r)` and `ceil(128/(h-r))` shared queries. The factor-1 baseline derives complete packed leaves from old openings. For interleaved RS, consistency instead samples one coordinate: all old stripes are opened, the checked combined values are verifier-derived, and only the unchecked offsets in the packed WHIR leaf are serialized. The combined-oracle authentication frontier is merged with its first WHIR opening. WHIR uses the proved Johnson-bound estimator.\n"
        )?;
    }
    writeln!(
        file,
        "Mean of {} deterministic samples; seed `{:#x}`. WHIR targets {} bits, terminates at at most `2^{}` variables, and folds `2^{}` values per column.\n",
        config.samples,
        config.seed,
        DEFAULT_SECURITY_BITS,
        MAX_TERMINAL_MESSAGE_VARIABLES,
        DEFAULT_FOLDING_LOG,
    )?;
    writeln!(
        file,
        "| log2 N | t | B each | total opened | Merkle KiB | field | codeword | RS blocks | combinations | WHIR instances | best h | {} | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |",
        if config.virtual_initial_oracle {
            "initial WHIR q"
        } else {
            "consistency q"
        }
    )?;
    writeln!(
        file,
        "|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"
    )?;
    for row in rows {
        writeln!(
            file,
            "| {} | {} | {} | {} | {:.2} | {}-bit | N/{} | {} x 2^{} | {} | {} | {} | {} | {:.2} | {:.2} | {:.2} | {:.3} |",
            row.log_length,
            row.commitment_count,
            row.batch_size_per_commitment,
            row.total_opened_values,
            row.merkle_mean_bytes / 1024.0,
            row.best_layer_field_bits,
            1usize << row.codeword_log_reduction,
            row.rs_interleaving_factor,
            row.rs_block_length_log,
            row.parallel_linear_combinations,
            row.parallel_whir_instances,
            row.best_h,
            if config.virtual_initial_oracle {
                row.virtual_initial_queries
            } else {
                row.consistency_queries
            },
            row.canopy_mean_bytes / 1024.0,
            row.canopy_hash_bytes / 1024.0,
            row.canopy_field_bytes / 1024.0,
            row.canopy_over_merkle,
        )?;
    }
    Ok(())
}

#[derive(Clone, Debug, Serialize)]
pub struct SeparateCommitmentTimingRow {
    pub log_length: usize,
    pub vector_length: usize,
    pub commitment_count: usize,
    pub batch_size_per_commitment: usize,
    pub scheme: String,
    pub h: Option<usize>,
    pub codeword_log_reduction: Option<usize>,
    pub rs_length: Option<usize>,
    pub parallel_linear_combinations: usize,
    pub repetitions: usize,
    pub mean_seconds: f64,
    pub median_seconds: f64,
    pub min_seconds: f64,
    pub max_seconds: f64,
    pub relative_to_merkle_mean: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct SeparateCommitmentStageTimingRow {
    pub log_length: usize,
    pub commitment_count: usize,
    pub batch_size_per_commitment: usize,
    pub scheme: String,
    pub h: Option<usize>,
    pub codeword_log_reduction: Option<usize>,
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

struct StageTimingContext<'a> {
    log_length: usize,
    commitment_count: usize,
    batch_size: usize,
    scheme: &'a str,
    h: Option<usize>,
    codeword_log_reduction: Option<usize>,
    mean_total: f64,
}

fn stage_timing_row(
    context: &StageTimingContext<'_>,
    stage: &str,
    work_items: Option<usize>,
    work_unit: &str,
    durations: &[Duration],
) -> SeparateCommitmentStageTimingRow {
    let (mean, median, min, max) = duration_statistics(durations);
    SeparateCommitmentStageTimingRow {
        log_length: context.log_length,
        commitment_count: context.commitment_count,
        batch_size_per_commitment: context.batch_size,
        scheme: context.scheme.to_owned(),
        h: context.h,
        codeword_log_reduction: context.codeword_log_reduction,
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

fn commitment_profile_path(output: &Path) -> PathBuf {
    let stem = output
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("separate_commitment_timings");
    output.with_file_name(format!("{stem}_profile.csv"))
}

fn linear_combination_coefficients(
    combinations: usize,
    commitment_count: usize,
    seed: u64,
) -> Vec<Vec<Fr>> {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    (0..combinations)
        .map(|_| {
            (0..commitment_count)
                .map(|_| Fr::from_bits(rng.gen::<[u64; 4]>()))
                .collect()
        })
        .collect()
}

/// Arrange one folding column from every combined codeword in each Merkle
/// leaf. For five combinations, each leaf contains `5 * 16 = 80` fields.
fn combine_codewords_interleaved(
    codewords: &[Vec<Fr>],
    coefficients: &[Vec<Fr>],
) -> Result<Vec<Fr>> {
    ensure!(!codewords.is_empty(), "at least one codeword is required");
    ensure!(
        !coefficients.is_empty() && coefficients.iter().all(|row| row.len() == codewords.len()),
        "coefficient matrix has the wrong shape"
    );
    let codeword_len = codewords[0].len();
    ensure!(
        codeword_len.is_multiple_of(WHIR_LEAF_WIDTH)
            && codewords.iter().all(|word| word.len() == codeword_len),
        "codewords must have one common folding-aligned length"
    );
    let combinations = coefficients.len();
    let leaf_width = combinations * WHIR_LEAF_WIDTH;
    let mut result = vec![Fr::from(0u64); combinations * codeword_len];
    result
        .par_chunks_mut(leaf_width)
        .enumerate()
        .for_each(|(leaf, output)| {
            let start = leaf * WHIR_LEAF_WIDTH;
            for combination in 0..combinations {
                for offset in 0..WHIR_LEAF_WIDTH {
                    let mut value = Fr::from(0u64);
                    for (word, coefficient) in
                        codewords.iter().zip(coefficients[combination].iter())
                    {
                        value += *coefficient * word[start + offset];
                    }
                    output[combination * WHIR_LEAF_WIDTH + offset] = value;
                }
            }
        });
    Ok(result)
}

struct CanopyPipelineMeasurement {
    total: Duration,
    bottom_forest: Duration,
    hash_to_field: Duration,
    reed_solomon_encoding: Duration,
    old_codeword_trees: Duration,
    linear_combination: Duration,
    combined_oracle_tree: Duration,
    other: Duration,
    combined_root: [u8; HASH_BYTES],
}

fn measure_canopy_pipeline(
    vectors: &[Vec<Fr>],
    h: usize,
    codeword_log_reduction: usize,
    coefficients: &[Vec<Fr>],
) -> Result<CanopyPipelineMeasurement> {
    let mut bottom_forest = Duration::ZERO;
    let mut hash_to_field = Duration::ZERO;
    let mut reed_solomon_encoding = Duration::ZERO;
    let mut old_codeword_trees = Duration::ZERO;
    let mut other = Duration::ZERO;
    let mut initial_totals = Duration::ZERO;
    let mut codewords = Vec::with_capacity(vectors.len());

    for values in vectors {
        let (commitment, data, profile) =
            CanopyVectorCommitment::commit_profiled(values, h, codeword_log_reduction)?;
        std::hint::black_box(commitment.root);
        bottom_forest += profile.bottom_forest;
        hash_to_field += profile.hash_to_field;
        reed_solomon_encoding += profile.reed_solomon_encoding;
        old_codeword_trees += profile.codeword_merkle_tree;
        other += profile.other();
        initial_totals += profile.total;
        // The codeword already exists in prover data. Move it into the list;
        // deallocation of the remaining prover data is outside stage timers.
        codewords.push(data.codeword);
    }

    let start = Instant::now();
    let combined = combine_codewords_interleaved(&codewords, coefficients)?;
    let linear_combination = start.elapsed();

    let start = Instant::now();
    let combined_tree = MerkleLevels::commit(&combined, coefficients.len() * WHIR_LEAF_WIDTH)?;
    let combined_oracle_tree = start.elapsed();
    let combined_root = combined_tree.root()?;

    Ok(CanopyPipelineMeasurement {
        total: initial_totals + linear_combination + combined_oracle_tree,
        bottom_forest,
        hash_to_field,
        reed_solomon_encoding,
        old_codeword_trees,
        linear_combination,
        combined_oracle_tree,
        other,
        combined_root,
    })
}

#[derive(Clone, Copy, Debug)]
pub struct SeparateCommitmentTimingConfig<'a> {
    pub log_length: usize,
    pub commitment_count: usize,
    pub batch_size: usize,
    pub h: usize,
    pub codeword_log_reduction: usize,
    pub repetitions: usize,
    pub seed: u64,
    pub output: &'a Path,
}

pub fn run_separate_commitment_timings(
    config: &SeparateCommitmentTimingConfig<'_>,
) -> Result<Vec<SeparateCommitmentTimingRow>> {
    let SeparateCommitmentTimingConfig {
        log_length,
        commitment_count,
        batch_size,
        h,
        codeword_log_reduction,
        repetitions,
        seed,
        output,
    } = *config;
    ensure!(repetitions > 0, "repetitions must be positive");
    ensure!(commitment_count > 0, "commitment count must be positive");
    let vector_len = 1usize << log_length;
    ensure!(batch_size <= vector_len, "batch exceeds vector length");
    ensure!(h > 0 && h < log_length, "invalid Canopy height");
    ensure!(
        codeword_log_reduction < h,
        "Canopy reduction must be smaller than h"
    );
    let codeword_len = vector_len >> codeword_log_reduction;
    let field = field_profile(DEFAULT_FIELD_BITS, log_length - codeword_log_reduction)?;
    let combinations = linear_combination_repetitions(field);

    let vectors: Vec<Vec<Fr>> = (0..commitment_count)
        .map(|vector| {
            (0..vector_len)
                .map(|index| {
                    crate::benchmark_field_element(0x43414e4f5059, vector as u64, index as u64)
                })
                .collect()
        })
        .collect();
    let coefficients = linear_combination_coefficients(combinations, commitment_count, seed);
    let mut rows = Vec::new();
    let mut stage_rows = Vec::new();

    for values in &vectors {
        let _ = MerkleVectorCommitment::commit(values)?;
    }
    let mut merkle_durations = Vec::with_capacity(repetitions);
    for _ in 0..repetitions {
        let mut total = Duration::ZERO;
        for values in &vectors {
            let start = Instant::now();
            let (commitment, data) = MerkleVectorCommitment::commit(values)?;
            total += start.elapsed();
            std::hint::black_box(commitment.root);
            drop(data);
        }
        merkle_durations.push(total);
    }
    let (merkle_mean, median, min, max) = duration_statistics(&merkle_durations);
    let merkle_context = StageTimingContext {
        log_length,
        commitment_count,
        batch_size,
        scheme: "merkle_separate",
        h: None,
        codeword_log_reduction: None,
        mean_total: merkle_mean,
    };
    stage_rows.push(stage_timing_row(
        &merkle_context,
        "separate_merkle_trees",
        Some(commitment_count * (2 * vector_len - 1)),
        "leaf/internal hashes",
        &merkle_durations,
    ));
    rows.push(SeparateCommitmentTimingRow {
        log_length,
        vector_length: vector_len,
        commitment_count,
        batch_size_per_commitment: batch_size,
        scheme: "merkle_separate".to_owned(),
        h: None,
        codeword_log_reduction: None,
        rs_length: None,
        parallel_linear_combinations: 0,
        repetitions,
        mean_seconds: merkle_mean,
        median_seconds: median,
        min_seconds: min,
        max_seconds: max,
        relative_to_merkle_mean: 1.0,
    });

    let _ = measure_canopy_pipeline(&vectors, h, codeword_log_reduction, &coefficients)?;
    let mut totals = Vec::with_capacity(repetitions);
    let mut bottom = Vec::with_capacity(repetitions);
    let mut hash_to_field = Vec::with_capacity(repetitions);
    let mut encoding = Vec::with_capacity(repetitions);
    let mut old_trees = Vec::with_capacity(repetitions);
    let mut combinations_time = Vec::with_capacity(repetitions);
    let mut combined_trees = Vec::with_capacity(repetitions);
    let mut other = Vec::with_capacity(repetitions);
    for _ in 0..repetitions {
        let profile = measure_canopy_pipeline(&vectors, h, codeword_log_reduction, &coefficients)?;
        std::hint::black_box(profile.combined_root);
        totals.push(profile.total);
        bottom.push(profile.bottom_forest);
        hash_to_field.push(profile.hash_to_field);
        encoding.push(profile.reed_solomon_encoding);
        old_trees.push(profile.old_codeword_trees);
        combinations_time.push(profile.linear_combination);
        combined_trees.push(profile.combined_oracle_tree);
        other.push(profile.other);
    }
    let (mean, median, min, max) = duration_statistics(&totals);
    let scheme = canopy_scheme(codeword_log_reduction, 1, false);
    let canopy_context = StageTimingContext {
        log_length,
        commitment_count,
        batch_size,
        scheme,
        h: Some(h),
        codeword_log_reduction: Some(codeword_log_reduction),
        mean_total: mean,
    };
    for (stage, work_items, work_unit, durations) in [
        (
            "separate_bottom_forests",
            Some(commitment_count * (2 * vector_len - (vector_len >> h))),
            "leaf/internal hashes",
            &bottom,
        ),
        (
            "hash_to_field",
            Some(commitment_count * (vector_len >> h)),
            "forest roots",
            &hash_to_field,
        ),
        (
            "reed_solomon_encoding",
            Some(commitment_count * codeword_len),
            "codeword elements",
            &encoding,
        ),
        (
            "old_codeword_merkle_trees",
            Some(commitment_count * (2 * (codeword_len / WHIR_LEAF_WIDTH) - 1)),
            "leaf/internal hashes",
            &old_trees,
        ),
        (
            "linear_combinations",
            Some(combinations * commitment_count * codeword_len),
            "field multiply-adds",
            &combinations_time,
        ),
        (
            "combined_oracle_merkle_tree",
            Some(2 * (codeword_len / WHIR_LEAF_WIDTH) - 1),
            "leaf/internal hashes",
            &combined_trees,
        ),
        ("other", None, "-", &other),
    ] {
        stage_rows.push(stage_timing_row(
            &canopy_context,
            stage,
            work_items,
            work_unit,
            durations,
        ));
    }
    rows.push(SeparateCommitmentTimingRow {
        log_length,
        vector_length: vector_len,
        commitment_count,
        batch_size_per_commitment: batch_size,
        scheme: scheme.to_owned(),
        h: Some(h),
        codeword_log_reduction: Some(codeword_log_reduction),
        rs_length: Some(codeword_len),
        parallel_linear_combinations: combinations,
        repetitions,
        mean_seconds: mean,
        median_seconds: median,
        min_seconds: min,
        max_seconds: max,
        relative_to_merkle_mean: mean / merkle_mean,
    });

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    write_csv(output, &rows)?;
    write_csv(commitment_profile_path(output), &stage_rows)?;
    write_timing_summary(output.with_extension("md"), &rows, &stage_rows)?;
    Ok(rows)
}

fn write_timing_summary(
    path: impl AsRef<Path>,
    rows: &[SeparateCommitmentTimingRow],
    stage_rows: &[SeparateCommitmentStageTimingRow],
) -> Result<()> {
    let mut file = File::create(path)?;
    let first = rows.first().expect("at least one timing row");
    let canopy = rows.last().expect("at least one Canopy timing row");
    let codeword_divisor = 1usize << canopy.codeword_log_reduction.unwrap();
    writeln!(file, "# Separate-commitment prover timings\n")?;
    writeln!(
        file,
        "`N = 2^{}`, `t = {}`, and `B = {}` per commitment. Canopy uses an `N/{}` codeword, and the batch selects its proof-size-optimal height. Commitments are executed sequentially; their internal Rayon parallelism is unchanged. Witness generation, verifier-challenge generation, and deallocation are excluded. Canopy includes construction and commitment of the {} challenge-dependent linear combinations, but not WHIR execution.\n",
        first.log_length,
        first.commitment_count,
        first.batch_size_per_commitment,
        codeword_divisor,
        canopy.parallel_linear_combinations,
    )?;
    writeln!(
        file,
        "| scheme | codeword | h | combinations | repetitions | mean s | median s | min s | max s | vs Merkle mean |"
    )?;
    writeln!(file, "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|")?;
    for row in rows {
        let codeword = row.codeword_log_reduction.map_or_else(
            || "-".to_owned(),
            |reduction| format!("N/{}", 1usize << reduction),
        );
        writeln!(
            file,
            "| {} | {} | {} | {} | {} | {:.4} | {:.4} | {:.4} | {:.4} | {:.3}x |",
            row.scheme,
            codeword,
            row.h.map_or_else(|| "-".to_owned(), |h| h.to_string()),
            row.parallel_linear_combinations,
            row.repetitions,
            row.mean_seconds,
            row.median_seconds,
            row.min_seconds,
            row.max_seconds,
            row.relative_to_merkle_mean,
        )?;
    }

    writeln!(file, "\n## Mean stage profile\n")?;
    writeln!(
        file,
        "| scheme | codeword | h | stage | work | mean s | median s | share |"
    )?;
    writeln!(file, "|---|---:|---:|---|---:|---:|---:|---:|")?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_combinations_are_required_for_the_32_bit_field() {
        let field = field_profile(32, 12).unwrap();
        let repetitions = linear_combination_repetitions(field);
        assert_eq!(repetitions, 5);
        assert!(repetitions as f64 * field.log2_size >= DEFAULT_SECURITY_BITS as f64);
        assert!((repetitions - 1) as f64 * field.log2_size < DEFAULT_SECURITY_BITS as f64);
    }

    #[test]
    fn six_combinations_are_required_for_the_24_bit_field() {
        let field = field_profile(24, 23).unwrap();
        let repetitions = linear_combination_repetitions(field);
        assert_eq!(repetitions, 6);
        assert_eq!(repetitions as f64 * field.log2_size, 144.0);
        assert!((repetitions - 1) as f64 * field.log2_size < DEFAULT_SECURITY_BITS as f64);
    }

    #[test]
    fn consistency_queries_follow_minimum_distance() {
        assert_eq!(consistency_query_count(10), 13);
        assert_eq!(consistency_query_count(4), 32);
        // Canopy-quarter at h=6 has log inverse rate 6 - 2 = 4.
        assert_eq!(consistency_query_count(6 - 2), 32);
    }

    #[test]
    fn pointwise_consistency_opens_old_stripes_and_packed_new_leaf_remainders() {
        let sample = estimate_canopy_sample(
            CanopySampleContext {
                log_length: 18,
                commitment_count: 2,
                batch_size: 8,
                h: 5,
                codeword_log_reduction: 2,
                rs_interleaving_factor: 8,
                field: field_profile(32, 13).unwrap(),
                virtual_initial_oracle: false,
            },
            &(0..8).collect::<Vec<_>>(),
            &[0, 1, 17],
            7,
        )
        .unwrap();
        assert_eq!(sample.old_consistency_fields, 2 * 8 * 3);
        // Coordinates 0 and 1 share one width-16 leaf; coordinate 17 uses a
        // second. Five checked values per coordinate are derived.
        assert_eq!(sample.new_consistency_fields, 5 * (2 * 16 - 3));
    }

    #[test]
    fn combined_codewords_use_interleaved_folding_columns() {
        let words = vec![
            (0..32).map(Fr::from).collect::<Vec<_>>(),
            (100..132).map(Fr::from).collect::<Vec<_>>(),
        ];
        let coefficients = vec![vec![Fr::from(1u64), Fr::from(2u64)]; 5];
        let combined = combine_codewords_interleaved(&words, &coefficients).unwrap();
        assert_eq!(combined.len(), 5 * 32);
        assert_eq!(combined[0], Fr::from(200u64));
        assert_eq!(combined[WHIR_LEAF_WIDTH], Fr::from(200u64));
        let tree = MerkleLevels::commit(&combined, 5 * WHIR_LEAF_WIDTH).unwrap();
        assert_eq!(tree.leaf_count(), 2);
    }

    #[test]
    fn small_matrix_writes_all_outputs() {
        let temp = tempfile::tempdir().unwrap();
        let config = SeparateSizeMatrixConfig {
            log_lengths: vec![18],
            batch_sizes: vec![8],
            heights: vec![4, 5],
            commitment_counts: vec![2, 4],
            layer_field_bits: vec![32],
            codeword_log_reductions: vec![0, 2],
            rs_interleaving_factors: vec![1, 8, 16],
            virtual_initial_oracle: false,
            samples: 3,
            seed: 1,
        };
        let rows = run_separate_size_matrix(&config, temp.path()).unwrap();
        assert_eq!(rows.len(), 12);
        assert!(rows.iter().any(|row| row.codeword_log_reduction == 2));
        assert!(rows.iter().any(|row| {
            row.codeword_log_reduction == 2
                && row.rs_interleaving_factor == 8
                && row.rs_block_length_log == 13
        }));
        assert!(rows.iter().any(|row| {
            row.codeword_log_reduction == 2
                && row.rs_interleaving_factor == 16
                && row.rs_block_length_log == 12
        }));
        assert!(temp.path().join("proof_sizes.csv").exists());
        assert!(temp.path().join("best_h.csv").exists());
        assert!(temp.path().join("best_h_by_field.csv").exists());
        assert!(temp.path().join("field_profiles.csv").exists());
        assert!(temp.path().join("protocol_parameters.csv").exists());
        assert!(temp.path().join("summary.md").exists());
    }

    #[test]
    fn virtual_initial_oracle_replaces_standalone_consistency() {
        let sample = estimate_canopy_sample(
            CanopySampleContext {
                log_length: 22,
                commitment_count: 2,
                batch_size: 8,
                h: 5,
                codeword_log_reduction: 2,
                rs_interleaving_factor: 16,
                field: field_profile(32, 16).unwrap(),
                virtual_initial_oracle: true,
            },
            &(0..8).collect::<Vec<_>>(),
            &[],
            7,
        )
        .unwrap();

        assert!(sample.virtual_initial_auth_hashes > 0);
        assert!(sample.virtual_initial_fields > 0);
        assert_eq!(sample.virtual_initial_fields % (2 * 16), 0);
        assert_eq!(sample.old_consistency_auth_hashes, 0);
        assert_eq!(sample.old_consistency_fields, 0);
        assert_eq!(sample.new_consistency_fields, 0);
    }
}
