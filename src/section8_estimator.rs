//! Exact expected authentication costs for the two Section 8 reductions.
//!
//! Queries are sampled with replacement. Canopy bottom forests deduplicate
//! rows and parent computations; native IRS openings retain repeated row
//! answers and leaf hashes, then merge parent paths. Returned commitments are
//! output-instance data and are kept separate from the reduction proof.
//! Each recurring step casts `ell_cast` fresh Canopy commitments together with
//! one prior Canopy accumulator. Its Merkle reduction also consumes `ell_acc`
//! prior Merkle accumulators. Ordinary Merkle controls batch `ell_cast` fresh
//! full-length messages and `ell_acc` prior full-length accumulators.
//!
//! This module records the repository's Johnson-with-slack query budget and
//! finite-field diagnostics. It does not certify a composed accumulation
//! theorem for batching by powers of one challenge.

use anyhow::{ensure, Context, Result};
use serde::Serialize;

use crate::{section8::commitment_config, FIELD_BITS, FIELD_BYTES, HASH_BYTES};

pub const SECURITY_SCOPE: &str = "local Johnson-with-slack query budget and degree-scaled affine MCA diagnostics; composed powers-batching soundness is not certified";

#[derive(Clone, Debug, Serialize)]
pub struct Section8EstimateParameters {
    pub log_message: usize,
    /// Fresh Canopy inputs, excluding the one prior Canopy accumulator.
    pub ell_cast: usize,
    /// Prior Merkle accumulators; also the prior full-length control inputs.
    pub ell_acc: usize,
    pub log_leaf_width: usize,
    pub local_security_bits: usize,
}

/// Authenticated source counts for one recurring accumulation step.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Section8SourceCounts {
    pub fresh_canopy_sources: usize,
    pub prior_canopy_sources: usize,
    pub prior_merkle_sources: usize,
    pub ifold_sources: usize,
    pub iacc_sources: usize,
    pub merkle_control_sources: usize,
}

impl Section8EstimateParameters {
    pub fn source_counts(&self) -> Result<Section8SourceCounts> {
        let prior_canopy_sources = 1;
        let ifold_sources = self
            .ell_cast
            .checked_add(prior_canopy_sources)
            .context("ifold source count overflow")?;
        let iacc_sources = ifold_sources
            .checked_add(self.ell_acc)
            .context("iacc source count overflow")?;
        let merkle_control_sources = self
            .ell_cast
            .checked_add(self.ell_acc)
            .context("Merkle control source count overflow")?;
        Ok(Section8SourceCounts {
            fresh_canopy_sources: self.ell_cast,
            prior_canopy_sources,
            prior_merkle_sources: self.ell_acc,
            ifold_sources,
            iacc_sources,
            merkle_control_sources,
        })
    }
}

impl Default for Section8EstimateParameters {
    fn default() -> Self {
        Self {
            log_message: 22,
            ell_cast: 2,
            ell_acc: 1,
            log_leaf_width: 3,
            local_security_bits: 128,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct ExpectedAuthentication {
    pub distinct_rows: f64,
    pub authentication_digests: f64,
    pub leaf_hashes: f64,
    pub parent_hashes: f64,
    pub roots: f64,
}

impl ExpectedAuthentication {
    pub fn verifier_hashes(&self) -> f64 {
        self.leaf_hashes + self.parent_hashes
    }
}

/// Expected occupied bins after `query_draws` independent uniform samples.
/// The log1p/expm1 form remains accurate when a large tree has few queries.
pub fn expected_occupied_nodes(nodes: usize, query_draws: usize) -> f64 {
    assert!(nodes > 0, "an authentication level cannot be empty");
    if query_draws == 0 {
        return 0.0;
    }
    if nodes == 1 {
        return 1.0;
    }
    let m = nodes as f64;
    -m * ((query_draws as f64) * (-1.0 / m).ln_1p()).exp_m1()
}

/// Exact expectation for a binary forest opening through `depth` levels.
/// Native openings hash all repeated rows; Canopy forests hash distinct rows.
pub fn expected_authentication(
    rows: usize,
    query_draws: usize,
    depth: usize,
    repeated_leaf_hashes: bool,
) -> Result<ExpectedAuthentication> {
    ensure!(rows.is_power_of_two(), "row count must be a power of two");
    ensure!(
        depth <= rows.ilog2() as usize,
        "authentication depth exceeds tree"
    );
    let distinct_rows = expected_occupied_nodes(rows, query_draws);
    let mut out = ExpectedAuthentication {
        distinct_rows,
        leaf_hashes: if repeated_leaf_hashes {
            query_draws as f64
        } else {
            distinct_rows
        },
        roots: distinct_rows,
        ..Default::default()
    };
    let mut children = distinct_rows;
    for level in 1..=depth {
        let parents = expected_occupied_nodes(rows >> level, query_draws);
        // Each touched parent has two children, minus the children supplied by
        // other queried paths. This counts transmitted digests, not hash calls.
        out.authentication_digests += 2.0 * parents - children;
        out.parent_hashes += parents;
        out.roots = parents;
        children = parents;
    }
    Ok(out)
}

#[derive(Clone, Debug, Serialize)]
pub struct ReductionEstimate {
    pub scheme: String,
    pub log_message: usize,
    pub message_field_elements: usize,
    pub codeword_field_elements: usize,
    pub merkle_rows: usize,
    pub leaf_width: usize,
    /// Actual authenticated input commitments, including recurring state.
    pub sources: usize,
    pub query_draws_per_source: usize,
    /// Per source; paths under distinct commitments cannot be merged.
    pub expected_distinct_query_rows: f64,
    pub expected_opened_field_elements: f64,
    pub sumcheck_field_elements: usize,
    pub final_evaluation_field_elements: usize,
    pub ood_field_elements: usize,
    pub transcript_field_elements: usize,
    pub expected_authentication_digests: f64,
    pub expected_authentication_proof_bytes: f64,
    pub expected_field_proof_bytes: f64,
    pub reduction_proof_bytes: f64,
    pub output_instance_bytes: usize,
    pub proof_and_output_bytes: f64,
    pub expected_authentication_hashes: f64,
    pub fiat_shamir_finalizations: usize,
    pub verifier_hashes: f64,
    /// Alias used by callers choosing a height-search objective.
    pub total_verifier_hashes: f64,
    pub query_security_bits: f64,
    /// Native affine (degree-one) MCA bound before batching degree scaling.
    pub affine_mca_security_bits: f64,
    pub batching_powers_degree: usize,
    /// Diagnostic: multiply the affine error by the challenge-polynomial degree.
    /// This is not an extraction or composition proof for the powers compiler.
    pub degree_scaled_affine_mca_security_bits: f64,
    pub one_ood_security_bits: f64,
    pub sumcheck_security_bits: f64,
    pub finite_field_floor_bits: f64,
    pub eligible: bool,
    pub security_scope: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Section8HeightEstimate {
    pub height: usize,
    pub bottom_depth: usize,
    pub layer_log_message: usize,
    pub layer_log_inverse_rate: usize,
    pub ifold: ReductionEstimate,
    pub iacc: ReductionEstimate,
    pub combined_verifier_hashes: f64,
    pub total_verifier_hashes: f64,
    pub combined_reduction_proof_bytes: f64,
    pub combined_output_instance_bytes: usize,
    pub combined_proof_and_output_bytes: f64,
    pub eligible: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct Section8HeightSearch {
    pub parameters: Section8EstimateParameters,
    pub candidates: Vec<Section8HeightEstimate>,
    pub selected: Section8HeightEstimate,
    pub merkle_rate_half: ReductionEstimate,
    pub merkle_rate_quarter: ReductionEstimate,
}

/// Flat, CSV-compatible rows; every candidate and both controls are exported.
#[derive(Clone, Debug, Serialize)]
pub struct Section8EstimateCsvRow {
    /// Distinguish recurring-state estimates from historical fresh-only runs.
    pub workload: String,
    pub log_n: usize,
    pub ell_cast: usize,
    pub ell_acc: usize,
    pub log_leaf_width: usize,
    pub local_security_bits: usize,
    pub height: usize,
    pub selected_height: usize,
    pub is_selected: bool,
    pub eligible: bool,
    pub scheme: String,
    pub log_message: usize,
    pub message_field_elements: usize,
    pub codeword_field_elements: usize,
    pub merkle_rows: usize,
    pub leaf_width: usize,
    pub sources: usize,
    pub query_draws_per_source: usize,
    pub expected_distinct_query_rows: f64,
    pub expected_opened_field_elements: f64,
    pub sumcheck_field_elements: usize,
    pub final_evaluation_field_elements: usize,
    pub ood_field_elements: usize,
    pub transcript_field_elements: usize,
    pub expected_authentication_digests: f64,
    pub expected_authentication_proof_bytes: f64,
    pub expected_field_proof_bytes: f64,
    pub reduction_proof_bytes: f64,
    pub output_instance_bytes: usize,
    pub proof_and_output_bytes: f64,
    pub expected_authentication_hashes: f64,
    pub fiat_shamir_finalizations: usize,
    pub verifier_hashes: f64,
    pub query_security_bits: f64,
    pub affine_mca_security_bits: f64,
    pub batching_powers_degree: usize,
    pub degree_scaled_affine_mca_security_bits: f64,
    pub one_ood_security_bits: f64,
    pub sumcheck_security_bits: f64,
    pub finite_field_floor_bits: f64,
    pub security_scope: String,
}

impl Section8HeightSearch {
    pub fn csv_rows(&self) -> Vec<Section8EstimateCsvRow> {
        let mut out = Vec::with_capacity(self.candidates.len() * 2 + 2);
        for candidate in &self.candidates {
            for reduction in [&candidate.ifold, &candidate.iacc] {
                out.push(self.csv_row(reduction, candidate.height, candidate.eligible));
            }
        }
        for reduction in [&self.merkle_rate_half, &self.merkle_rate_quarter] {
            out.push(self.csv_row(reduction, 0, reduction.eligible));
        }
        out
    }

    fn csv_row(
        &self,
        r: &ReductionEstimate,
        height: usize,
        eligible: bool,
    ) -> Section8EstimateCsvRow {
        Section8EstimateCsvRow {
            workload: "steady_state_v2".to_owned(),
            log_n: self.parameters.log_message,
            ell_cast: self.parameters.ell_cast,
            ell_acc: self.parameters.ell_acc,
            log_leaf_width: self.parameters.log_leaf_width,
            local_security_bits: self.parameters.local_security_bits,
            height,
            selected_height: self.selected.height,
            is_selected: height == 0 || height == self.selected.height,
            eligible,
            scheme: r.scheme.clone(),
            log_message: r.log_message,
            message_field_elements: r.message_field_elements,
            codeword_field_elements: r.codeword_field_elements,
            merkle_rows: r.merkle_rows,
            leaf_width: r.leaf_width,
            sources: r.sources,
            query_draws_per_source: r.query_draws_per_source,
            expected_distinct_query_rows: r.expected_distinct_query_rows,
            expected_opened_field_elements: r.expected_opened_field_elements,
            sumcheck_field_elements: r.sumcheck_field_elements,
            final_evaluation_field_elements: r.final_evaluation_field_elements,
            ood_field_elements: r.ood_field_elements,
            transcript_field_elements: r.transcript_field_elements,
            expected_authentication_digests: r.expected_authentication_digests,
            expected_authentication_proof_bytes: r.expected_authentication_proof_bytes,
            expected_field_proof_bytes: r.expected_field_proof_bytes,
            reduction_proof_bytes: r.reduction_proof_bytes,
            output_instance_bytes: r.output_instance_bytes,
            proof_and_output_bytes: r.proof_and_output_bytes,
            expected_authentication_hashes: r.expected_authentication_hashes,
            fiat_shamir_finalizations: r.fiat_shamir_finalizations,
            verifier_hashes: r.verifier_hashes,
            query_security_bits: r.query_security_bits,
            affine_mca_security_bits: r.affine_mca_security_bits,
            batching_powers_degree: r.batching_powers_degree,
            degree_scaled_affine_mca_security_bits: r.degree_scaled_affine_mca_security_bits,
            one_ood_security_bits: r.one_ood_security_bits,
            sumcheck_security_bits: r.sumcheck_security_bits,
            finite_field_floor_bits: r.finite_field_floor_bits,
            security_scope: r.security_scope.clone(),
        }
    }
}

fn reduction_estimate(
    parameters: &Section8EstimateParameters,
    scheme: &str,
    log_message: usize,
    log_inverse_rate: usize,
    sources: usize,
    bottom_depth: Option<usize>,
) -> Result<ReductionEstimate> {
    let cfg = commitment_config(
        log_message,
        log_inverse_rate,
        parameters.log_leaf_width,
        parameters.local_security_bits as f64,
    )?;
    let is_ifold = bottom_depth.is_some();
    let depth = bottom_depth.unwrap_or(cfg.codeword_length().ilog2() as usize);
    let q = cfg.in_domain_samples();
    let auth = expected_authentication(cfg.codeword_length(), q, depth, !is_ifold)?;
    let opened_rows = if is_ifold {
        auth.distinct_rows
    } else {
        q as f64
    };
    let opened_fields = opened_rows * cfg.interleaving_depth() as f64 * sources as f64;
    let sumcheck_fields = 2 * log_message;
    let final_evaluation_fields = sources;
    let ood_fields = 1;
    let transcript_fields = sumcheck_fields
        .checked_add(final_evaluation_fields)
        .and_then(|fields| fields.checked_add(ood_fields))
        .context("transcript field count overflow")?;
    let authentication_digests = auth.authentication_digests * sources as f64;
    let authentication_proof_bytes = authentication_digests * HASH_BYTES as f64;
    let field_proof_bytes = (opened_fields + transcript_fields as f64) * FIELD_BYTES as f64;
    let reduction_proof_bytes = authentication_proof_bytes + field_proof_bytes;
    // The complete SEP instance also contains its functional and sigma. Those
    // values are verifier-derived and not newly transmitted. Only the returned
    // commitment's root and (for Canopy) layer OOD value are charged here.
    let output_instance_bytes = HASH_BYTES + if is_ifold { FIELD_BYTES } else { 0 };
    let authentication_hashes = auth.verifier_hashes() * sources as f64;
    let fs = log_message + if is_ifold { 5 } else { 4 };
    let affine_mca_bits = cfg.rbr_soundness_fold_prox_gaps();
    let powers_degree = sources.saturating_sub(1).max(1);
    let degree_scaled_mca_bits = affine_mca_bits - (powers_degree as f64).log2();
    // A single original-polynomial OOD answer. This is the exact choose(L,2)
    // union factor used by the native IRS one-sample OOD parameter solver.
    let list_size = cfg.list_size();
    let ood_degree = (1usize << log_message).saturating_sub(1).max(1);
    let ood_bits = FIELD_BITS as f64
        - (ood_degree as f64).log2()
        - (0.5 * list_size * (list_size - 1.0)).log2();
    let sumcheck_bits = FIELD_BITS as f64 - (2.0 * log_message.max(1) as f64).log2();
    let batching_bits = FIELD_BITS as f64 - (powers_degree as f64).log2();
    let finite_field_floor = degree_scaled_mca_bits
        .min(ood_bits)
        .min(sumcheck_bits)
        .min(batching_bits);
    let query_bits = cfg.rbr_queries();
    let budget = parameters.local_security_bits as f64;
    let eligible = query_bits + 1e-9 >= budget && finite_field_floor + 1e-9 >= budget;
    Ok(ReductionEstimate {
        scheme: scheme.to_owned(),
        log_message,
        message_field_elements: 1 << log_message,
        codeword_field_elements: cfg.size(),
        merkle_rows: cfg.codeword_length(),
        leaf_width: cfg.interleaving_depth(),
        sources,
        query_draws_per_source: q,
        expected_distinct_query_rows: auth.distinct_rows,
        expected_opened_field_elements: opened_fields,
        sumcheck_field_elements: sumcheck_fields,
        final_evaluation_field_elements: final_evaluation_fields,
        ood_field_elements: ood_fields,
        transcript_field_elements: transcript_fields,
        expected_authentication_digests: authentication_digests,
        expected_authentication_proof_bytes: authentication_proof_bytes,
        expected_field_proof_bytes: field_proof_bytes,
        reduction_proof_bytes,
        output_instance_bytes,
        proof_and_output_bytes: reduction_proof_bytes + output_instance_bytes as f64,
        expected_authentication_hashes: authentication_hashes,
        fiat_shamir_finalizations: fs,
        verifier_hashes: authentication_hashes + fs as f64,
        total_verifier_hashes: authentication_hashes + fs as f64,
        query_security_bits: query_bits,
        affine_mca_security_bits: affine_mca_bits,
        batching_powers_degree: powers_degree,
        degree_scaled_affine_mca_security_bits: degree_scaled_mca_bits,
        one_ood_security_bits: ood_bits,
        sumcheck_security_bits: sumcheck_bits,
        finite_field_floor_bits: finite_field_floor,
        eligible,
        security_scope: SECURITY_SCOPE.to_owned(),
    })
}

pub fn estimate_height(
    parameters: &Section8EstimateParameters,
    height: usize,
) -> Result<Section8HeightEstimate> {
    validate_parameters(parameters)?;
    let n = parameters.log_message;
    ensure!(
        height >= parameters.log_leaf_width.max(2),
        "height must contain a full leaf and leave rate below one"
    );
    ensure!(height <= n, "constant Canopy layers are excluded");
    let layer_log_message = n + 1 - height;
    ensure!(
        layer_log_message >= parameters.log_leaf_width,
        "Canopy message must contain at least one full leaf"
    );
    let bottom_depth = height - parameters.log_leaf_width;
    let counts = parameters.source_counts()?;
    let ifold = reduction_estimate(
        parameters,
        "ifold",
        n,
        1,
        counts.ifold_sources,
        Some(bottom_depth),
    )?;
    let iacc = reduction_estimate(
        parameters,
        "iacc",
        layer_log_message,
        height - 1,
        counts.iacc_sources,
        None,
    )?;
    Ok(Section8HeightEstimate {
        height,
        bottom_depth,
        layer_log_message,
        layer_log_inverse_rate: height - 1,
        combined_verifier_hashes: ifold.verifier_hashes + iacc.verifier_hashes,
        total_verifier_hashes: ifold.verifier_hashes + iacc.verifier_hashes,
        combined_reduction_proof_bytes: ifold.reduction_proof_bytes + iacc.reduction_proof_bytes,
        combined_output_instance_bytes: ifold.output_instance_bytes + iacc.output_instance_bytes,
        combined_proof_and_output_bytes: ifold.proof_and_output_bytes + iacc.proof_and_output_bytes,
        eligible: ifold.eligible && iacc.eligible,
        ifold,
        iacc,
    })
}

fn validate_parameters(parameters: &Section8EstimateParameters) -> Result<()> {
    ensure!(
        (2..=30).contains(&parameters.log_message),
        "estimate message log must be between 2 and 30 (rate-1/4 domain fits 2^32)"
    );
    ensure!(
        parameters.log_leaf_width <= parameters.log_message,
        "leaf exceeds input message"
    );
    ensure!(
        parameters.ell_cast > 0,
        "ifold needs at least one fresh Canopy source"
    );
    parameters.source_counts()?;
    ensure!(
        (1..=256).contains(&parameters.local_security_bits),
        "local security budget must be 1..=256 bits"
    );
    Ok(())
}

/// Search all geometrically valid nonconstant Canopy heights and choose the
/// eligible minimum of total ifold+iacc verifier hashes. Equal objectives use
/// the smaller height for deterministic selection.
pub fn estimate_height_search(
    parameters: &Section8EstimateParameters,
) -> Result<Section8HeightSearch> {
    validate_parameters(parameters)?;
    let maximum_height = parameters
        .log_message
        .min(parameters.log_message + 1 - parameters.log_leaf_width);
    let minimum_height = parameters.log_leaf_width.max(2);
    ensure!(
        minimum_height <= maximum_height,
        "no nonconstant Canopy layer fits the requested leaf width"
    );
    let candidates = (minimum_height..=maximum_height)
        .map(|height| estimate_height(parameters, height))
        .collect::<Result<Vec<_>>>()?;
    let selected = candidates
        .iter()
        .filter(|candidate| candidate.eligible)
        .min_by(|a, b| {
            a.combined_verifier_hashes
                .total_cmp(&b.combined_verifier_hashes)
                .then(a.height.cmp(&b.height))
        })
        .context("no height meets the local query and finite-field diagnostic budgets")?
        .clone();
    let sources = parameters.source_counts()?.merkle_control_sources;
    let merkle_rate_half = reduction_estimate(
        parameters,
        "merkle_rate_half",
        parameters.log_message,
        1,
        sources,
        None,
    )?;
    let merkle_rate_quarter = reduction_estimate(
        parameters,
        "merkle_rate_quarter",
        parameters.log_message,
        2,
        sources,
        None,
    )?;
    Ok(Section8HeightSearch {
        parameters: parameters.clone(),
        candidates,
        selected,
        merkle_rate_half,
        merkle_rate_quarter,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::{
        merkle::{authentication_work_by_height, reconstruct_roots, MerkleLevels},
        Fr,
    };

    fn close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 1e-10, "{actual} != {expected}");
    }

    #[test]
    fn forest_expectation_matches_exhaustive_repeated_queries() {
        let rows: usize = 4;
        let draws: usize = 3;
        let samples = rows.pow(draws as u32);
        for depth in 0..=2 {
            let mut digests = 0.0;
            let mut distinct_rows = 0.0;
            let mut parents = 0.0;
            let mut roots = 0.0;
            for encoded in 0..samples {
                let indices = [
                    encoded % rows,
                    (encoded / rows) % rows,
                    encoded / (rows * rows),
                ];
                let work = authentication_work_by_height(&indices, rows).unwrap()[depth];
                digests += work.auth as f64;
                distinct_rows += indices.into_iter().collect::<BTreeSet<_>>().len() as f64;
                parents += work.parents as f64;
                roots += work.roots as f64;
            }
            for native in [false, true] {
                let expected = expected_authentication(rows, draws, depth, native).unwrap();
                close(expected.authentication_digests, digests / samples as f64);
                close(expected.distinct_rows, distinct_rows / samples as f64);
                close(expected.parent_hashes, parents / samples as f64);
                close(expected.roots, roots / samples as f64);
                close(
                    expected.leaf_hashes,
                    if native {
                        draws as f64
                    } else {
                        distinct_rows / samples as f64
                    },
                );
            }
        }
    }

    #[test]
    fn occupancy_is_stable_for_sparse_huge_trees_and_saturated_small_trees() {
        close(expected_occupied_nodes(1, 0), 0.0);
        close(expected_occupied_nodes(1, 317), 1.0);
        close(expected_occupied_nodes(4, 1), 1.0);
        close(expected_occupied_nodes(4, 100_000), 4.0);
        let sparse = expected_occupied_nodes(1usize << 32, 317);
        assert!(sparse < 317.0 && sparse > 316.9999);
    }

    #[test]
    fn geometry_and_output_boundaries_match_section8() {
        let params = Section8EstimateParameters::default();
        let search = estimate_height_search(&params).unwrap();
        assert_eq!(search.merkle_rate_half.query_draws_per_source, 298);
        assert_eq!(search.merkle_rate_quarter.query_draws_per_source, 138);
        assert_eq!(search.merkle_rate_half.sources, 3);
        assert_eq!(search.merkle_rate_quarter.sources, 3);
        for candidate in &search.candidates {
            assert_eq!(candidate.ifold.sources, 3);
            assert_eq!(candidate.iacc.sources, 4);
            assert_eq!(
                candidate.ifold.codeword_field_elements,
                1 << (params.log_message + 1)
            );
            assert_eq!(
                candidate.iacc.codeword_field_elements,
                1 << params.log_message
            );
            assert_eq!(
                candidate.iacc.message_field_elements,
                1 << (params.log_message + 1 - candidate.height)
            );
            assert_eq!(candidate.ifold.output_instance_bytes, 64);
            assert_eq!(candidate.iacc.output_instance_bytes, 32);
            close(
                candidate.ifold.reduction_proof_bytes,
                candidate.ifold.expected_authentication_proof_bytes
                    + candidate.ifold.expected_field_proof_bytes,
            );
            close(
                candidate.combined_verifier_hashes,
                candidate.ifold.verifier_hashes + candidate.iacc.verifier_hashes,
            );
        }
        let selected = &search.selected;
        assert!(selected.eligible);
        assert!(search
            .candidates
            .iter()
            .filter(|c| c.eligible)
            .all(|c| c.combined_verifier_hashes >= selected.combined_verifier_hashes));
        let rows = search.csv_rows();
        assert_eq!(rows.len(), 2 * search.candidates.len() + 2);
        assert!(rows.iter().all(|row| row.workload == "steady_state_v2"));
    }

    #[test]
    fn query_budgets_follow_the_code_rate_and_not_the_source_count() {
        // These are the native Johnson-with-slack budgets at the current
        // 128-bit per-round target. Changing the row width or the number of source
        // roots must not accidentally change the RS rate's query budget.
        for log_message in [18, 20, 22] {
            for log_leaf_width in [0, 3] {
                for ell_cast in [1, 8] {
                    let params = Section8EstimateParameters {
                        log_message,
                        ell_cast,
                        ell_acc: 1,
                        log_leaf_width,
                        local_security_bits: 128,
                    };
                    let search = estimate_height_search(&params).unwrap();
                    assert_eq!(search.merkle_rate_half.query_draws_per_source, 298);
                    assert_eq!(search.merkle_rate_quarter.query_draws_per_source, 138);
                    for (height, expected_queries) in [(4, 90), (5, 67), (6, 53)] {
                        let candidate = search
                            .candidates
                            .iter()
                            .find(|c| c.height == height)
                            .unwrap();
                        assert_eq!(candidate.ifold.query_draws_per_source, 298);
                        assert_eq!(candidate.iacc.query_draws_per_source, expected_queries);
                        assert_eq!(candidate.ifold.sources, ell_cast + 1);
                        assert_eq!(candidate.iacc.sources, ell_cast + 2);
                        assert_eq!(search.merkle_rate_half.sources, ell_cast + 1);
                        assert_eq!(search.merkle_rate_quarter.sources, ell_cast + 1);
                        assert_eq!(
                            candidate.iacc.expected_opened_field_elements,
                            (expected_queries * (1 << log_leaf_width) * (ell_cast + 2)) as f64
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn accepted_grid_uses_the_global_hash_minimum_at_the_expected_heights() {
        // Recurring steps include one prior Canopy source. These optima were
        // independently obtained by rescaling the retained per-source costs
        // at every height, rather than reusing the implementation's search.
        for (log_message, expected_heights) in
            [(18, [4, 4, 4, 4]), (20, [5, 5, 4, 4]), (22, [5, 5, 5, 5])]
        {
            for (ell_cast, expected_height) in [1, 2, 4, 8].into_iter().zip(expected_heights) {
                let params = Section8EstimateParameters {
                    log_message,
                    ell_cast,
                    ell_acc: 1,
                    log_leaf_width: 3,
                    local_security_bits: 136,
                };
                let search = estimate_height_search(&params).unwrap();
                assert_eq!(search.selected.height, expected_height);
                // With row width eight, all nonconstant aligned heights are
                // exactly 3..=log(N)-2. Require the search to retain the full
                // range, rather than only checking its winning neighbors.
                assert_eq!(
                    search
                        .candidates
                        .iter()
                        .map(|c| c.height)
                        .collect::<Vec<_>>(),
                    (3..=log_message - 2).collect::<Vec<_>>()
                );
                for candidate in search.candidates.iter().filter(|c| c.eligible) {
                    assert!(
                        candidate.total_verifier_hashes >= search.selected.total_verifier_hashes,
                        "N=2^{log_message}, cast={ell_cast}: height {} beats selected height {}",
                        candidate.height,
                        search.selected.height
                    );
                    assert_eq!(candidate.ifold.codeword_field_elements, 2 << log_message);
                    assert_eq!(candidate.iacc.codeword_field_elements, 1 << log_message);
                    assert_eq!(
                        candidate.iacc.message_field_elements * (1 << (candidate.height - 1)),
                        candidate.iacc.codeword_field_elements
                    );
                    assert_eq!(
                        candidate.ifold.merkle_rows * candidate.ifold.leaf_width,
                        candidate.ifold.codeword_field_elements
                    );
                    assert_eq!(
                        candidate.iacc.merkle_rows * candidate.iacc.leaf_width,
                        candidate.iacc.codeword_field_elements
                    );
                }
                assert_eq!(
                    search.merkle_rate_half.message_field_elements,
                    1 << log_message
                );
                assert_eq!(
                    search.merkle_rate_half.codeword_field_elements,
                    2 << log_message
                );
                assert_eq!(
                    search.merkle_rate_quarter.message_field_elements,
                    1 << log_message
                );
                assert_eq!(
                    search.merkle_rate_quarter.codeword_field_elements,
                    4 << log_message
                );
            }
        }
    }

    #[test]
    fn recurring_accumulators_change_cast_counts_but_not_full_length_controls() {
        let params = Section8EstimateParameters {
            log_message: 10,
            ell_cast: 1,
            ell_acc: 1,
            log_leaf_width: 3,
            local_security_bits: 136,
        };
        assert_eq!(
            params.source_counts().unwrap(),
            Section8SourceCounts {
                fresh_canopy_sources: 1,
                prior_canopy_sources: 1,
                prior_merkle_sources: 1,
                ifold_sources: 2,
                iacc_sources: 3,
                merkle_control_sources: 2,
            }
        );
        let search = estimate_height_search(&params).unwrap();
        let candidate = search.candidates.iter().find(|c| c.height == 5).unwrap();
        // Fresh and prior Canopy roots both contribute bottom openings and an
        // evaluation. Their two residuals join the prior Merkle accumulator.
        assert_eq!(candidate.ifold.final_evaluation_field_elements, 2);
        assert_eq!(candidate.ifold.transcript_field_elements, 23);
        assert_eq!(candidate.iacc.final_evaluation_field_elements, 3);
        assert_eq!(candidate.iacc.transcript_field_elements, 16);
        assert_eq!(candidate.ifold.batching_powers_degree, 1);
        assert_eq!(candidate.iacc.batching_powers_degree, 2);
        for control in [&search.merkle_rate_half, &search.merkle_rate_quarter] {
            assert_eq!(control.sources, 2);
            assert_eq!(control.message_field_elements, 1024);
            assert_eq!(control.transcript_field_elements, 23);
            assert_eq!(control.batching_powers_degree, 1);
        }

        // Prior Merkle states only affect iacc and the full-length controls.
        let more_priors = Section8EstimateParameters {
            ell_acc: 4,
            ..params.clone()
        };
        let more = estimate_height(&more_priors, 5).unwrap();
        assert_eq!(more.ifold.sources, 2);
        assert_eq!(more.iacc.sources, 6);
        assert_eq!(more.iacc.batching_powers_degree, 5);
        close(
            more.ifold.reduction_proof_bytes,
            candidate.ifold.reduction_proof_bytes,
        );
        close(
            more.iacc.expected_authentication_hashes,
            2.0 * candidate.iacc.expected_authentication_hashes,
        );
        close(
            more.iacc.degree_scaled_affine_mca_security_bits,
            candidate.iacc.affine_mca_security_bits - 5f64.log2(),
        );
    }

    #[test]
    fn recurring_source_counts_reject_overflow_before_estimation() {
        for (ell_cast, ell_acc) in [(usize::MAX, 0), (usize::MAX - 1, 1)] {
            let params = Section8EstimateParameters {
                ell_cast,
                ell_acc,
                ..Default::default()
            };
            assert!(params.source_counts().is_err());
            assert!(estimate_height(&params, 5).is_err());
            assert!(estimate_height_search(&params).is_err());
        }
        // A representable source count can still exceed transcript capacity.
        // Report an error instead of overflowing while adding sumcheck fields.
        let params = Section8EstimateParameters {
            ell_cast: usize::MAX - 2,
            ell_acc: 1,
            ..Default::default()
        };
        assert!(params.source_counts().is_ok());
        assert!(estimate_height(&params, 5).is_err());
    }

    #[test]
    fn expected_counts_match_real_packed_forest_openings_exhaustively() {
        let rows: usize = 8;
        let width: usize = 8;
        let draws: usize = 3;
        let samples = rows.pow(draws as u32);
        let values = (0..rows * width)
            .map(|i| Fr::from(i as u64 + 1))
            .collect::<Vec<_>>();
        for depth in 0..=3 {
            let forest = MerkleLevels::commit_forest(&values, width, depth).unwrap();
            let mut auth_digests = 0;
            let mut distinct_rows = 0;
            let mut touched_roots = 0;
            let mut parent_hashes = 0;
            let mut opened_fields = 0;
            for encoded in 0..samples {
                let indices = [
                    encoded % rows,
                    (encoded / rows) % rows,
                    encoded / (rows * rows),
                ]
                .into_iter()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
                let proof = forest.open_hashes(&indices).unwrap();
                let leaves = indices
                    .iter()
                    .map(|&i| values[i * width..(i + 1) * width].to_vec())
                    .collect::<Vec<_>>();
                let roots =
                    reconstruct_roots(rows, width, depth, &indices, &leaves, &proof).unwrap();
                for (&i, root) in &roots {
                    assert_eq!(root, &forest.roots()[i]);
                }
                auth_digests += proof.len();
                distinct_rows += indices.len();
                touched_roots += roots.len();
                opened_fields += leaves.iter().map(Vec::len).sum::<usize>();
                // In the actual opening graph, distinct binary parents equal
                // supplied digests + opened leaves - returned roots.
                // Derive this from serialized proofs and verified roots,
                // independently of the estimator's occupancy recurrence.
                parent_hashes += proof.len() + indices.len() - roots.len();
            }
            for native in [false, true] {
                let estimate = expected_authentication(rows, draws, depth, native).unwrap();
                close(
                    estimate.authentication_digests,
                    auth_digests as f64 / samples as f64,
                );
                close(
                    estimate.distinct_rows,
                    distinct_rows as f64 / samples as f64,
                );
                close(
                    estimate.parent_hashes,
                    parent_hashes as f64 / samples as f64,
                );
                close(estimate.roots, touched_roots as f64 / samples as f64);
                close(
                    estimate.leaf_hashes,
                    if native {
                        draws as f64
                    } else {
                        distinct_rows as f64 / samples as f64
                    },
                );
                if !native {
                    close(
                        estimate.distinct_rows * width as f64,
                        opened_fields as f64 / samples as f64,
                    );
                }
            }
        }
    }

    #[test]
    fn oversized_scalar_height_is_retained_but_excluded_by_field_diagnostic() {
        let params = Section8EstimateParameters {
            log_message: 30,
            log_leaf_width: 0,
            // This field diagnostic lies between the 128- and 136-bit targets.
            local_security_bits: 136,
            ..Default::default()
        };
        let search = estimate_height_search(&params).unwrap();
        let tall = search.candidates.iter().find(|c| c.height == 30).unwrap();
        assert!(!tall.eligible);
        assert!(
            tall.iacc.degree_scaled_affine_mca_security_bits < params.local_security_bits as f64
        );
        assert!(search.selected.eligible);
    }
}
