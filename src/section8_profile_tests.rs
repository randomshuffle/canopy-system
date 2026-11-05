//! Timing-profile regressions. Durations are checked by conservation and by
//! which operations the reduction performs, without snapshots of wall time.
use super::*;
use whir_ref::protocols::matrix_commit::Merklize;

fn close_time(label: &str, total: f64, parts: f64) {
    assert!(
        total.is_finite() && total >= 0.0,
        "{label}: invalid total {total}"
    );
    assert!(
        parts.is_finite() && parts >= 0.0,
        "{label}: invalid parts {parts}"
    );
    let tolerance = 1e-9 + 1e-8 * total.abs().max(parts.abs());
    assert!(
        (total - parts).abs() <= tolerance,
        "{label}: total {total} differs from disjoint phases {parts}"
    );
}

fn fresh_commitment_phases(row: &ReductionMeasurement) -> [f64; 6] {
    [
        row.fresh_encoding_seconds,
        row.fresh_merkle_leaf_seconds,
        row.fresh_merkle_parent_seconds,
        row.fresh_h2f_seconds,
        row.fresh_ood_seconds,
        row.fresh_other_seconds,
    ]
}

fn incoming_commitment_phases(row: &ReductionMeasurement) -> [f64; 6] {
    [
        row.incoming_encoding_seconds,
        row.incoming_merkle_leaf_seconds,
        row.incoming_merkle_parent_seconds,
        row.incoming_h2f_seconds,
        row.incoming_ood_seconds,
        row.incoming_other_seconds,
    ]
}

fn assert_profile_conservation(row: &ReductionMeasurement) {
    assert!(row.verified);
    let output_commit = [
        row.encoding_seconds,
        row.merkle_leaf_seconds,
        row.merkle_parent_seconds,
        row.h2f_seconds,
        row.canopy_ood_seconds,
        row.output_ood_seconds,
        row.output_commit_other_seconds,
    ];
    let sumcheck = [
        row.sumcheck_setup_seconds,
        row.sumcheck_polynomial_seconds,
        row.sumcheck_fold_seconds,
        row.sumcheck_transcript_seconds,
    ];
    let combination = [
        row.message_combination_seconds,
        row.codeword_combination_seconds,
    ];
    let opening = [row.opening_auth_seconds, row.opening_claim_seconds];
    let incoming = incoming_commitment_phases(row);
    let fresh = fresh_commitment_phases(row);
    for &seconds in output_commit
        .iter()
        .chain(&sumcheck)
        .chain(&combination)
        .chain(&opening)
        .chain(&incoming)
        .chain(&fresh)
        .chain(&[
            row.form_seconds,
            row.serialization_seconds,
            row.prover_other_seconds,
            row.fresh_commit_seconds,
            row.prior_commit_seconds,
        ])
    {
        assert!(
            seconds.is_finite() && seconds >= 0.0,
            "{} contains an invalid phase duration {seconds}",
            row.scheme
        );
    }
    close_time(
        "output commitment",
        row.output_commit_seconds,
        output_commit.iter().sum(),
    );
    close_time("sumcheck", row.sumcheck_seconds, sumcheck.iter().sum());
    close_time(
        "combination",
        row.combination_seconds,
        combination.iter().sum(),
    );
    close_time("opening", row.opening_seconds, opening.iter().sum());
    close_time(
        "incoming commitments",
        row.incoming_commit_seconds,
        incoming.iter().sum(),
    );
    close_time(
        "fresh commitments",
        row.fresh_commit_seconds,
        fresh.iter().sum(),
    );
    assert_eq!(
        row.incoming_commit_seconds,
        row.fresh_commit_seconds + row.prior_commit_seconds,
        "{} must partition input commitment setup exactly",
        row.scheme
    );
    for (fresh_phase, incoming_phase) in fresh.into_iter().zip(incoming) {
        assert!(
            fresh_phase <= incoming_phase + 1e-9,
            "{} fresh phase includes more than all incoming work",
            row.scheme
        );
    }

    // Parent totals provide a view of the same events. Sum the exclusive
    // children once, so accidentally including both parent and child timers
    // in prover time cannot pass this check.
    let exclusive_prover = row.form_seconds
        + sumcheck.iter().sum::<f64>()
        + combination.iter().sum::<f64>()
        + output_commit.iter().sum::<f64>()
        + opening.iter().sum::<f64>()
        + row.serialization_seconds
        + row.prover_other_seconds;
    close_time("prover step", row.step_seconds, exclusive_prover);
    assert!(row.step_seconds > 0.0);
    assert!(row.incoming_commit_seconds >= 0.0);
}

#[test]
fn reduction_profiles_partition_setup_and_prover_operations() {
    let run = run_canopy(10, 2, 1, 5, 3, 136., 91, true).unwrap();
    let half = run_merkle(10, 2, 1, 1, 3, 136., 91, true).unwrap();
    let quarter = run_merkle(10, 2, 1, 2, 3, 136., 91, true).unwrap();
    for row in [&run.ifold, &run.iacc, &half, &quarter] {
        assert_profile_conservation(row);
        assert!(row.incoming_commit_seconds > 0.0);
        assert!(row.incoming_encoding_seconds > 0.0);
        assert!(row.incoming_merkle_leaf_seconds > 0.0);
        assert!(row.incoming_merkle_parent_seconds > 0.0);
        assert!(row.merkle_leaf_seconds > 0.0);
        assert!(row.merkle_parent_seconds > 0.0);
        assert!(row.output_ood_seconds > 0.0);
        assert!(row.serialization_seconds > 0.0);
        assert!(row.sumcheck_polynomial_seconds > 0.0);
        assert!(row.sumcheck_fold_seconds > 0.0);
        assert!(row.message_combination_seconds > 0.0);
        assert!(row.codeword_combination_seconds > 0.0);
    }

    // ifold encodes its fresh Canopy layer, converts bottom subtree digests,
    // and evaluates its layer OOD point. Those phases also occur in preparation
    // of its incoming Canopy commitments.
    assert!(run.ifold.encoding_seconds > 0.0);
    assert!(run.ifold.h2f_seconds > 0.0);
    assert!(run.ifold.canopy_ood_seconds > 0.0);
    assert!(run.ifold.incoming_h2f_seconds > 0.0);
    assert!(run.ifold.incoming_ood_seconds > 0.0);

    // The Merkle reductions combine cached codewords; they perform no output
    // RS encoding or digest conversion. Their incoming encoding is setup and
    // must remain separate from reduction time.
    for row in [&run.iacc, &half, &quarter] {
        assert_eq!(row.encoding_seconds, 0.0);
        assert_eq!(row.h2f_seconds, 0.0);
        assert_eq!(row.canopy_ood_seconds, 0.0);
        assert_eq!(row.incoming_h2f_seconds, 0.0);
        assert_eq!(row.incoming_ood_seconds, 0.0);
    }
}

fn assert_no_fresh_commitment(row: &ReductionMeasurement) {
    assert_eq!(row.fresh_commit_sources, 0);
    assert_eq!(row.fresh_commit_seconds, 0.0);
    assert_eq!(fresh_commitment_phases(row), [0.0; 6]);
}

#[test]
fn fresh_commitment_profiles_exclude_prior_and_reused_layer_inputs() {
    for fresh_count in [1, 8] {
        let run = run_canopy(10, fresh_count, 1, 5, 3, 136., 137, false).unwrap();
        assert_profile_conservation(&run.ifold);
        assert_profile_conservation(&run.iacc);
        assert_eq!(run.ifold.sources, fresh_count + 1);
        assert_eq!(run.ifold.fresh_commit_sources, fresh_count);
        assert!(run.ifold.fresh_commit_seconds > 0.0);
        assert!(run.ifold.prior_commit_seconds > 0.0);
        assert!(run.ifold.fresh_encoding_seconds > 0.0);
        assert!(run.ifold.fresh_merkle_leaf_seconds > 0.0);
        assert!(run.ifold.fresh_merkle_parent_seconds > 0.0);
        assert!(run.ifold.fresh_h2f_seconds > 0.0);
        assert!(run.ifold.fresh_ood_seconds > 0.0);

        // All F+1 layer witnesses are transferred from ifold. Their generation
        // belongs to the original Canopy commitments, so iacc must not charge
        // them again as freshly supplied oracles.
        assert_eq!(run.iacc.sources, fresh_count + 2);
        assert_no_fresh_commitment(&run.iacc);
        assert!(run.iacc.prior_commit_seconds > 0.0);
        assert_eq!(
            run.iacc.incoming_commit_seconds,
            run.iacc.prior_commit_seconds
        );
        assert!(run.iacc.merkle_leaf_seconds > 0.0);
        assert!(run.iacc.merkle_parent_seconds > 0.0);

        for rate in [1, 2] {
            let control = run_merkle(10, fresh_count, 1, rate, 3, 136., 137, false).unwrap();
            assert_profile_conservation(&control);
            assert_eq!(control.sources, fresh_count + 1);
            assert_eq!(control.fresh_commit_sources, fresh_count);
            assert!(control.fresh_commit_seconds > 0.0);
            assert!(control.prior_commit_seconds > 0.0);
            assert!(control.fresh_encoding_seconds > 0.0);
            assert!(control.fresh_merkle_leaf_seconds > 0.0);
            assert!(control.fresh_merkle_parent_seconds > 0.0);
            assert_eq!(control.fresh_h2f_seconds, 0.0);
            assert_eq!(control.fresh_ood_seconds, 0.0);
        }
    }
}

#[test]
fn empty_commitment_groups_have_exactly_zero_cost() {
    let cached_layers = run_canopy(10, 1, 0, 5, 3, 136., 139, false).unwrap();
    assert_profile_conservation(&cached_layers.ifold);
    assert_profile_conservation(&cached_layers.iacc);
    assert_eq!(cached_layers.ifold.fresh_commit_sources, 1);
    assert!(cached_layers.ifold.prior_commit_seconds > 0.0);
    assert_eq!(cached_layers.iacc.sources, 2);
    assert_no_fresh_commitment(&cached_layers.iacc);
    assert_eq!(cached_layers.iacc.prior_commit_seconds, 0.0);
    assert_eq!(cached_layers.iacc.incoming_commit_seconds, 0.0);
    assert_eq!(incoming_commitment_phases(&cached_layers.iacc), [0.0; 6]);
    // Zero incoming commitment work does not remove the actual reduction or
    // the newly produced output commitment from its measured prover time.
    assert!(cached_layers.iacc.step_seconds > 0.0);
    assert!(cached_layers.iacc.output_commit_seconds > 0.0);
    assert!(cached_layers.iacc.merkle_leaf_seconds > 0.0);

    for rate in [1, 2] {
        for (fresh_count, prior_count) in [(1, 0), (0, 1)] {
            let row = run_merkle(10, fresh_count, prior_count, rate, 3, 136., 139, false).unwrap();
            assert_profile_conservation(&row);
            assert_eq!(row.sources, fresh_count + prior_count);
            assert_eq!(row.fresh_commit_sources, fresh_count);
            if fresh_count == 0 {
                assert_no_fresh_commitment(&row);
                assert!(row.prior_commit_seconds > 0.0);
                assert_eq!(row.incoming_commit_seconds, row.prior_commit_seconds);
            } else {
                assert!(row.fresh_commit_seconds > 0.0);
                assert_eq!(row.prior_commit_seconds, 0.0);
                assert_eq!(row.incoming_commit_seconds, row.fresh_commit_seconds);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ProofReceipt {
    queries: usize,
    authentication_hashes: usize,
    fs_finalizations: usize,
    proof_bytes: usize,
    hint_bytes: usize,
    field_bytes: usize,
    hash_bytes: usize,
    framing_bytes: usize,
    output_bytes: usize,
    serialized_bytes: usize,
}

impl From<&ReductionMeasurement> for ProofReceipt {
    fn from(row: &ReductionMeasurement) -> Self {
        Self {
            queries: row.queries,
            authentication_hashes: row.auth_hashes,
            fs_finalizations: row.fs_finalizations,
            proof_bytes: row.proof_bytes,
            hint_bytes: row.proof_hint_bytes,
            field_bytes: row.proof_field_bytes,
            hash_bytes: row.proof_hash_bytes,
            framing_bytes: row.proof_framing_bytes,
            output_bytes: row.output_instance_bytes,
            serialized_bytes: row.serialized_total_bytes,
        }
    }
}

#[test]
fn profiling_pins_steady_state_small_proof_receipts() {
    // N=2^10, one fresh cast plus one prior Canopy, one prior Merkle, h=8,
    // width=8, local budget=136, seed=91. Pin proof accounting independently
    // of wall time. The controls retain their pre-correction receipts.
    let run = run_canopy(10, 1, 1, 8, 3, 136., 91, true).unwrap();
    let half = run_merkle(10, 1, 1, 1, 3, 136., 91, true).unwrap();
    let quarter = run_merkle(10, 1, 1, 2, 3, 136., 91, true).unwrap();
    let expected = [
        ProofReceipt {
            queries: 317,
            authentication_hashes: 854,
            fs_finalizations: 15,
            proof_bytes: 100_400,
            hint_bytes: 99_664,
            field_bytes: 96_480,
            hash_bytes: 3_904,
            framing_bytes: 16,
            output_bytes: 64,
            serialized_bytes: 100_464,
        },
        ProofReceipt {
            queries: 40,
            authentication_hashes: 339,
            fs_finalizations: 7,
            proof_bytes: 35_096,
            hint_bytes: 34_776,
            field_bytes: 31_040,
            hash_bytes: 4_032,
            framing_bytes: 24,
            output_bytes: 32,
            serialized_bytes: 35_128,
        },
        ProofReceipt {
            queries: 317,
            authentication_hashes: 1_130,
            fs_finalizations: 14,
            proof_bytes: 167_152,
            hint_bytes: 166_416,
            field_bytes: 163_040,
            hash_bytes: 4_096,
            framing_bytes: 16,
            output_bytes: 32,
            serialized_bytes: 167_184,
        },
        ProofReceipt {
            queries: 147,
            authentication_hashes: 936,
            fs_finalizations: 14,
            proof_bytes: 87_984,
            hint_bytes: 87_248,
            field_bytes: 76_000,
            hash_bytes: 11_968,
            framing_bytes: 16,
            output_bytes: 32,
            serialized_bytes: 88_016,
        },
    ];
    for (row, receipt) in [&run.ifold, &run.iacc, &half, &quarter]
        .into_iter()
        .zip(expected)
    {
        assert_profile_conservation(row);
        assert_eq!(
            ProofReceipt::from(row),
            receipt,
            "{} changed its proof",
            row.scheme
        );
    }
}

#[test]
fn profiled_merklize_preserves_native_nodes_and_root() {
    for log_width in [0, 3] {
        let cfg = commitment_config(8, 1, log_width, 136.).unwrap();
        let input = message(8, 91, 1717);
        let matrix = encode(&cfg, &input);
        let (native_nodes, native_root) = matrix.merklize(
            cfg.interleaving_depth(),
            hash::BLAKE3,
            &cfg.matrix_commit().merkle_tree,
        );
        let mut profile = CommitmentProfile::default();
        let (profiled_nodes, profiled_root) = merklize_profiled(&matrix, &cfg, &mut profile);
        assert_eq!(
            profiled_nodes,
            native_nodes,
            "width {} changed tree nodes",
            1 << log_width
        );
        assert_eq!(
            profiled_root,
            native_root,
            "width {} changed root",
            1 << log_width
        );
        assert!(profile.leaf > 0.0);
        assert!(profile.parent > 0.0);
        assert_eq!(profile.encoding, 0.0);
        assert_eq!(profile.h2f, 0.0);
        assert_eq!(profile.ood, 0.0);
    }
}

#[test]
fn fused_output_leaf_profile_is_explicit_and_conserved() {
    let run = run_canopy(16, 8, 1, 4, 3, 136., 177, false).unwrap();
    for row in [&run.ifold, &run.iacc] {
        assert_profile_conservation(row);
        assert!(row.codeword_leaf_fused);
        if row.scheme == "ifold_canopy" {
            assert!(row.merkle_leaf_seconds > 0.0); // Encoded Canopy layer leaves.
        } else {
            assert_eq!(row.merkle_leaf_seconds, 0.0);
        }
        assert!(row.codeword_combination_seconds > 0.0);
    }
    for rate in [1, 2] {
        let row = run_merkle(16, 8, 1, rate, 3, 136., 177, false).unwrap();
        assert_profile_conservation(&row);
        assert!(row.codeword_leaf_fused);
        assert_eq!(row.merkle_leaf_seconds, 0.0);
    }
}

#[test]
fn timer_remainders_normalize_roundoff_but_expose_real_overlap() {
    assert_eq!(phase_remainder(0.00001, 0.00001 + 1e-21), 0.0);
    assert!(phase_remainder(0.00001, 0.000010001) < 0.0);
    assert_eq!(phase_remainder(0.0, 0.0), 0.0);
}
