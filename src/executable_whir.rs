//! Executable, verifier-checked WHIR prover used by the timing experiments.
//!
//! The implementation is the official arkworks WHIR prover pinned in
//! `Cargo.toml`. This wrapper fixes the paper parameters (128-bit security,
//! selectable Johnson/capacity decoding, rate supplied by the caller,
//! four-variable folds, BLAKE3, and zero proof of work) and exposes stable
//! timing/accounting data. Capacity mode is conjectural.

use std::time::{Duration, Instant};

use anyhow::{ensure, Result};
use rayon::prelude::*;
use whir_ref::{
    algebra::{
        embedding::Identity,
        linear_form::{Evaluate, LinearForm, MultilinearExtension},
        ntt::{self, PolynomialSegment, Polynomials},
    },
    bits::Bits,
    buffer::{Buffer, BufferMath, BufferOps},
    hash,
    parameters::ProtocolParameters,
    protocols::{
        irs_commit, matrix_commit,
        params::DecodingRegime,
        whir::{Config, Witness},
    },
    transcript::{codecs::Empty, DomainSeparator, ProverState, VerifierMessage, VerifierState},
};

use crate::{whir::WhirDecodingRegime, Fr, FIELD_BITS, HASH_BYTES};

type BenchmarkTranscript = spongefish::instantiations::XOF<blake3::Hasher>;

pub const SECURITY_BITS: usize = 128;
pub const FOLDING_LOG: usize = 4;
pub const POW_BITS: usize = 0;

pub(crate) fn register_field_ntt() {
    crate::register_benchmark_ntt();
}

/// A row-major RS codeword produced by the same interleaved NTT path used by
/// WHIR's initial IRS commitment. `row_width` is the number of logical
/// polynomials interleaved in each Merkle leaf.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WhirEncodedMatrix {
    pub matrix: Buffer<Fr>,
    pub row_count: usize,
    pub row_width: usize,
}

/// A matrix commitment produced by WHIR's own row encoder and Merkle-tree
/// implementation.
#[derive(Clone, Debug)]
pub struct WhirMatrixCommitment {
    pub root: hash::Hash,
    pub config: matrix_commit::Config<Fr>,
    pub witness: matrix_commit::Witness,
}

impl WhirEncodedMatrix {
    pub fn values(&self) -> &[Fr] {
        self.matrix.to_slice()
    }

    pub fn len(&self) -> usize {
        self.matrix.len()
    }

    pub fn is_empty(&self) -> bool {
        self.matrix.is_empty()
    }
}

/// Encode one vector with WHIR's exact interleaved RS machinery. This is the
/// shared base-code implementation used by both the Merkle-WHIR baseline and
/// the WHIR-compatible Canopy commitment.
pub fn encode_interleaved_with_whir(
    values: &[Fr],
    log_inverse_rate: usize,
    folding_log: usize,
) -> Result<WhirEncodedMatrix> {
    register_field_ntt();
    ensure!(
        values.len().is_power_of_two(),
        "WHIR RS input length must be a power of two"
    );
    ensure!(log_inverse_rate > 0, "WHIR RS rate must be below one");
    ensure!(folding_log > 0, "WHIR interleaving must be positive");

    let requested_width = 1usize << folding_log;
    // A terminal vector shorter than one requested fold is encoded as one
    // scalar polynomial. There is no recursive fold to interleave in that
    // case; its evaluations may still be packed into wider Merkle leaves.
    let row_width = if values.len() < requested_width {
        1
    } else {
        requested_width
    };
    ensure!(
        values.len().is_multiple_of(row_width),
        "WHIR RS input is not divisible by its interleaving width"
    );
    let source = Buffer::from(values);
    let source_refs = [&source];
    let segments = [PolynomialSegment::from_rows(&source_refs, row_width)];
    let polynomials = Polynomials::from_segments(&segments);
    let row_count = (values.len() / row_width) << log_inverse_rate;
    let matrix = ntt::interleaved_rs_encode(polynomials, row_count);
    ensure!(
        matrix.len() == row_count * row_width,
        "WHIR RS encoder returned an unexpected matrix shape"
    );
    Ok(WhirEncodedMatrix {
        matrix,
        row_count,
        row_width,
    })
}

/// Encode independently committable vectors concurrently on WHIR's shared NTT
/// worker pool.
///
/// A single multi-polynomial transform produces a column-interleaved matrix and
/// requires a full-size deinterleaving copy before the inputs can receive
/// separate commitments. Running the independent transforms concurrently was
/// faster for this workload and preserves the native matrix layout directly.
pub fn encode_many_interleaved_with_whir(
    vectors: &[&[Fr]],
    log_inverse_rate: usize,
    folding_log: usize,
) -> Result<Vec<WhirEncodedMatrix>> {
    ensure!(!vectors.is_empty(), "at least one WHIR vector is required");
    let vector_len = vectors[0].len();
    ensure!(
        vectors.iter().all(|vector| vector.len() == vector_len),
        "batched WHIR RS inputs must have equal lengths"
    );
    vectors
        .par_iter()
        .map(|vector| encode_interleaved_with_whir(vector, log_inverse_rate, folding_log))
        .collect()
}

/// Commit a row-major codeword with WHIR's exact matrix-commitment code. The
/// optional row width lets a terminal one-polynomial codeword use the same
/// folding-column leaf width as the surrounding experiment.
pub fn commit_matrix_with_whir(
    encoded: &WhirEncodedMatrix,
    row_width: usize,
) -> Result<(WhirMatrixCommitment, Duration)> {
    ensure!(
        row_width.is_power_of_two(),
        "WHIR matrix row width must be a power of two"
    );
    ensure!(
        encoded.len().is_multiple_of(row_width),
        "WHIR matrix length is not divisible by its commitment row width"
    );
    let row_count = encoded.len() / row_width;
    ensure!(
        row_count.is_power_of_two(),
        "WHIR matrix row count must be a power of two"
    );
    let config = matrix_commit::Config::<Fr>::with_hash(hash::BLAKE3, row_count, row_width);
    let domain = DomainSeparator::protocol(&config)
        .session(&"canopy-system/whir-matrix-commitment/v1")
        .instance(&Empty);
    let mut prover_state = ProverState::new(&domain, BenchmarkTranscript::default());
    let start = Instant::now();
    let witness = config.commit(&mut prover_state, &encoded.matrix);
    let elapsed = start.elapsed();
    let proof = prover_state.proof();
    ensure!(
        proof.narg_string.len() >= HASH_BYTES,
        "WHIR matrix commitment did not emit a root"
    );
    let mut root = [0u8; HASH_BYTES];
    root.copy_from_slice(&proof.narg_string[..HASH_BYTES]);
    Ok((
        WhirMatrixCommitment {
            root: hash::Hash(root),
            config,
            witness,
        },
        elapsed,
    ))
}

#[derive(Clone, Debug)]
pub struct OracleGeometry {
    pub oracle: usize,
    pub message_length: usize,
    pub codeword_length: usize,
    pub leaf_width: usize,
    pub leaf_count: usize,
    pub requested_queries: usize,
}

#[derive(Clone, Debug)]
pub struct ExecutableWhirRun {
    pub vectors: usize,
    pub message_length: usize,
    pub log_inverse_rate: usize,
    pub commitment_time: Duration,
    /// Initial out-of-domain evaluations and transcript messages. This is
    /// opening work, not RS encoding or Merkle commitment work.
    pub ood_time: Duration,
    pub claim_evaluation_time: Duration,
    pub protocol_proving_time: Duration,
    pub proving_time: Duration,
    pub verification_time: Duration,
    pub proof_bytes: usize,
    pub oracle_geometry: Vec<OracleGeometry>,
}

pub(crate) fn config(
    message_length: usize,
    log_inverse_rate: usize,
    decoding_regime: WhirDecodingRegime,
    folding_log: usize,
) -> Result<Config<Identity<Fr>>> {
    config_with_security(
        message_length,
        log_inverse_rate,
        decoding_regime,
        folding_log,
        SECURITY_BITS,
    )
}

fn config_with_security(
    message_length: usize,
    log_inverse_rate: usize,
    decoding_regime: WhirDecodingRegime,
    folding_log: usize,
    security_bits: usize,
) -> Result<Config<Identity<Fr>>> {
    register_field_ntt();
    ensure!(
        message_length.is_power_of_two(),
        "WHIR message length must be a power of two"
    );
    ensure!(log_inverse_rate > 0, "WHIR rate must be below one");
    ensure!(folding_log > 0, "WHIR folding factor must be positive");
    let parameters = ProtocolParameters {
        decoding_regime: match decoding_regime {
            WhirDecodingRegime::Johnson => DecodingRegime::Johnson,
            WhirDecodingRegime::Capacity => DecodingRegime::Capacity,
        },
        starting_log_inv_rate: log_inverse_rate,
        initial_folding_factor: folding_log,
        folding_factor: folding_log,
        security_level: security_bits,
        pow_bits: POW_BITS,
        // Each call to `commit` below binds one vector. Multiple independently
        // committed vectors are batched by `prove`, which is experiment 3's
        // two-root composition.
        batch_size: 1,
        hash_id: hash::BLAKE3,
    };
    let config = Config::<Identity<Fr>>::new(message_length, &parameters);
    ensure!(
        config.check_max_pow_bits(Bits::new(0.0)),
        "the selected WHIR parameters require proof of work"
    );
    ensure!(
        config.security_level(1, 1) >= security_bits as f64,
        "WHIR configuration supplies only {:.2} bits",
        config.security_level(1, 1)
    );
    Ok(config)
}

fn oracle_geometry(config: &Config<Identity<Fr>>) -> Vec<OracleGeometry> {
    let initial = &config.initial_committer;
    let mut rows = vec![OracleGeometry {
        oracle: 0,
        message_length: initial.vector_size(),
        codeword_length: initial.codeword_length() * initial.interleaving_depth(),
        leaf_width: initial.interleaving_depth(),
        leaf_count: initial.codeword_length(),
        requested_queries: initial.in_domain_samples(),
    }];
    rows.extend(
        config
            .round_configs
            .iter()
            .enumerate()
            .map(|(round, config)| {
                let oracle = &config.irs_committer;
                OracleGeometry {
                    oracle: round + 1,
                    message_length: oracle.vector_size(),
                    codeword_length: oracle.codeword_length() * oracle.interleaving_depth(),
                    leaf_width: oracle.interleaving_depth(),
                    leaf_count: oracle.codeword_length(),
                    requested_queries: oracle.in_domain_samples(),
                }
            }),
    );
    rows
}

/// Execute and verify one WHIR proof. Passing multiple equal-length vectors
/// creates independent initial commitments and batches their claims into one
/// recursive WHIR execution, matching experiment 3.
pub fn execute(vectors: &[Vec<Fr>], log_inverse_rate: usize) -> Result<ExecutableWhirRun> {
    execute_with_regime(vectors, log_inverse_rate, WhirDecodingRegime::Johnson)
}

/// Execute and verify one WHIR proof in the requested decoding regime.
pub fn execute_with_regime(
    vectors: &[Vec<Fr>],
    log_inverse_rate: usize,
    decoding_regime: WhirDecodingRegime,
) -> Result<ExecutableWhirRun> {
    execute_with_regime_and_folding(vectors, log_inverse_rate, decoding_regime, FOLDING_LOG)
}

/// Execute and verify one WHIR proof with an explicit uniform folding factor.
pub fn execute_with_regime_and_folding(
    vectors: &[Vec<Fr>],
    log_inverse_rate: usize,
    decoding_regime: WhirDecodingRegime,
    folding_log: usize,
) -> Result<ExecutableWhirRun> {
    execute_inner(
        vectors,
        log_inverse_rate,
        decoding_regime,
        folding_log,
        false,
        SECURITY_BITS,
    )
}

/// Same prover with full-width deterministic evaluation points for comparing fields.
pub fn execute_benchmark_with_regime_and_folding(
    vectors: &[Vec<Fr>],
    log_inverse_rate: usize,
    decoding_regime: WhirDecodingRegime,
    folding_log: usize,
) -> Result<ExecutableWhirRun> {
    execute_inner(
        vectors,
        log_inverse_rate,
        decoding_regime,
        folding_log,
        true,
        SECURITY_BITS,
    )
}

/// Full-width benchmark with an explicit local security budget for composition.
pub fn execute_benchmark_with_security(
    vectors: &[Vec<Fr>],
    log_inverse_rate: usize,
    decoding_regime: WhirDecodingRegime,
    folding_log: usize,
    security_bits: usize,
) -> Result<ExecutableWhirRun> {
    execute_inner(
        vectors,
        log_inverse_rate,
        decoding_regime,
        folding_log,
        true,
        security_bits,
    )
}

fn execute_inner(
    vectors: &[Vec<Fr>],
    log_inverse_rate: usize,
    decoding_regime: WhirDecodingRegime,
    folding_log: usize,
    full_width_point: bool,
    security_bits: usize,
) -> Result<ExecutableWhirRun> {
    ensure!(!vectors.is_empty(), "at least one WHIR vector is required");
    let message_length = vectors[0].len();
    ensure!(
        vectors.iter().all(|vector| vector.len() == message_length),
        "batched WHIR vectors must have equal lengths"
    );
    ensure!(
        FIELD_BITS == 256,
        "paper WHIR execution requires the 256-bit field"
    );

    let config = if security_bits == SECURITY_BITS {
        config(
            message_length,
            log_inverse_rate,
            decoding_regime,
            folding_log,
        )?
    } else {
        config_with_security(
            message_length,
            log_inverse_rate,
            decoding_regime,
            folding_log,
            security_bits,
        )?
    };
    let geometry = oracle_geometry(&config);
    let domain = DomainSeparator::protocol(&config)
        .session(&"canopy-system/paper-experiments/v1")
        .instance(&Empty);
    let mut prover_state = ProverState::new(&domain, BenchmarkTranscript::default());
    let buffers: Vec<_> = vectors
        .iter()
        .map(|vector| Buffer::from(vector.as_slice()))
        .collect();

    let mut commitment_time = Duration::ZERO;
    let mut ood_time = Duration::ZERO;
    let mut witnesses = Vec::with_capacity(buffers.len());
    for buffer in &buffers {
        let commit_start = Instant::now();
        let irs = config
            .initial_committer
            .commit(&mut prover_state, &[buffer]);
        commitment_time += commit_start.elapsed();

        let ood_start = Instant::now();
        let points: Vec<Fr> = prover_state.verifier_message_vec(config.initial_out_domain_samples);
        let matrix = points
            .par_iter()
            .map(|&point| {
                buffer.mixed_univariate_evaluate(config.initial_committer.embedding(), point)
            })
            .collect::<Vec<_>>();
        // Transcript writes retain their canonical order after the independent
        // evaluations have completed in parallel.
        for value in &matrix {
            prover_state.prover_message(value);
        }
        let out_of_domain = irs_commit::Evaluations {
            points,
            matrix: Buffer::from(matrix.as_slice()),
            domain: None,
        };
        ood_time += ood_start.elapsed();
        witnesses.push(Witness { irs, out_of_domain });
    }

    // Computing the claimed evaluations is prover work, so include it in the
    // opening-proof timer rather than treating it as untimed setup.
    let claim_evaluation_start = Instant::now();
    let num_variables = message_length.ilog2() as usize;
    let point = if full_width_point || <Fr as ark_ff::Field>::characteristic() == [2] {
        (0..num_variables)
            .map(|i| crate::benchmark_field_element(0x43414e4f5059, 99, i as u64))
            .collect()
    } else {
        vec![Fr::from(7u64); num_variables]
    };
    let evaluation_form = MultilinearExtension::new(point.clone());
    let evaluations: Vec<_> = vectors
        .par_iter()
        .map(|vector| evaluation_form.evaluate(config.embedding(), vector))
        .collect();
    let claim_evaluation_time = claim_evaluation_start.elapsed();
    let prove_forms: Vec<Box<dyn LinearForm<Fr>>> =
        vec![Box::new(MultilinearExtension::new(point.clone()))];
    let buffer_refs: Vec<_> = buffers.iter().collect();
    let witness_refs: Vec<_> = witnesses.iter().collect();

    let protocol_proving_start = Instant::now();
    let _final_claim = config.prove(
        &mut prover_state,
        &buffer_refs,
        witness_refs,
        prove_forms,
        Buffer::from(evaluations.as_slice()),
    );
    let protocol_proving_time = protocol_proving_start.elapsed();
    let proving_time = claim_evaluation_time + protocol_proving_time;
    let proof = prover_state.proof();
    let proof_bytes = proof.narg_string.len() + proof.hints.len();

    let verify_start = Instant::now();
    let mut verifier_state = VerifierState::new(&domain, &proof, BenchmarkTranscript::default());
    let commitments: Vec<_> = (0..vectors.len())
        .map(|_| config.receive_commitment(&mut verifier_state))
        .collect::<std::result::Result<_, _>>()
        .map_err(|_| anyhow::anyhow!("failed to receive WHIR commitment"))?;
    let commitment_refs: Vec<_> = commitments.iter().collect();
    let final_claim = config
        .verify(&mut verifier_state, &commitment_refs, &evaluations)
        .map_err(|_| anyhow::anyhow!("WHIR verification failed"))?;
    let verify_form = MultilinearExtension::new(point);
    final_claim
        .verify([&verify_form as &dyn LinearForm<Fr>])
        .map_err(|_| anyhow::anyhow!("WHIR final claim failed"))?;
    let verification_time = verify_start.elapsed();

    Ok(ExecutableWhirRun {
        vectors: vectors.len(),
        message_length,
        log_inverse_rate,
        commitment_time,
        ood_time,
        claim_evaluation_time,
        protocol_proving_time,
        proving_time,
        verification_time,
        proof_bytes,
        oracle_geometry: geometry,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composed_budget_executes_and_verifies_with_more_queries() {
        let vectors: Vec<_> = (0..2)
            .map(|j| {
                (0..256)
                    .map(|i| crate::benchmark_field_element(123, j, i))
                    .collect()
            })
            .collect();
        let run = execute_benchmark_with_security(&vectors, 1, WhirDecodingRegime::Johnson, 3, 136)
            .unwrap();
        let old = config(256, 1, WhirDecodingRegime::Johnson, 3).unwrap();
        assert!(
            run.oracle_geometry[0].requested_queries > old.initial_committer.in_domain_samples()
        );
        assert!(run.proof_bytes > 0);
    }

    #[test]
    fn reference_whir_round_trip_without_pow() {
        let vector = (0..256u64).map(Fr::from).collect::<Vec<_>>();
        let run = execute(&[vector], 1).unwrap();
        assert_eq!(run.message_length, 256);
        assert!(run.proof_bytes > 0);
        assert_eq!(run.oracle_geometry[0].codeword_length, 512);
        let estimated = crate::whir::WhirParameters::johnson_bound(8, 1).unwrap();
        assert_eq!(
            run.oracle_geometry
                .iter()
                .map(|oracle| oracle.requested_queries)
                .collect::<Vec<_>>(),
            estimated.all_query_counts()
        );
    }

    #[test]
    fn two_independent_roots_can_be_batched() {
        let first = (0..256u64).map(Fr::from).collect::<Vec<_>>();
        let second = (1..=256u64).map(Fr::from).collect::<Vec<_>>();
        let run = execute(&[first, second], 4).unwrap();
        assert_eq!(run.vectors, 2);
    }

    #[test]
    fn batch_rs_encoding_matches_individual_encodings() {
        let first = (0..256u64).map(Fr::from).collect::<Vec<_>>();
        let second = (0..256u64)
            .map(|value| Fr::from(3 * value + 1))
            .collect::<Vec<_>>();
        let expected = [&first, &second]
            .into_iter()
            .map(|vector| encode_interleaved_with_whir(vector, 1, 3).unwrap())
            .collect::<Vec<_>>();
        let actual =
            encode_many_interleaved_with_whir(&[first.as_slice(), second.as_slice()], 1, 3)
                .unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn capacity_reference_whir_round_trip_without_pow() {
        let vector = (0..256u64).map(Fr::from).collect::<Vec<_>>();
        let run = execute_with_regime(&[vector], 1, WhirDecodingRegime::Capacity).unwrap();
        let estimated = crate::whir::WhirParameters::capacity_bound(8, 1).unwrap();
        assert_eq!(
            run.oracle_geometry
                .iter()
                .map(|oracle| oracle.requested_queries)
                .collect::<Vec<_>>(),
            estimated.all_query_counts()
        );
    }

    #[test]
    fn shared_encoder_and_matrix_commit_match_whir_initial_commitment() {
        for (length, log_inverse_rate) in [(256usize, 1usize), (32, 4)] {
            let vector = (0..length as u64).map(Fr::from).collect::<Vec<_>>();
            let config = config(length, log_inverse_rate, WhirDecodingRegime::Johnson, 3).unwrap();
            let domain = DomainSeparator::protocol(&config)
                .session(&"canopy-system/shared-code-test/v1")
                .instance(&Empty);
            let mut prover_state = ProverState::new(&domain, BenchmarkTranscript::default());
            let buffer = Buffer::from(vector.as_slice());
            let witness = config
                .initial_committer
                .commit(&mut prover_state, &[&buffer]);

            let encoded = encode_interleaved_with_whir(&vector, log_inverse_rate, 3).unwrap();
            assert_eq!(encoded.row_width, 8);
            assert_eq!(encoded.row_count, 64);
            assert_eq!(encoded.values(), witness.matrix.to_slice());

            let official_proof = prover_state.proof();
            let (shared_commitment, _) =
                commit_matrix_with_whir(&encoded, encoded.row_width).unwrap();
            assert_eq!(
                official_proof.narg_string[..HASH_BYTES],
                shared_commitment.root.0
            );
        }
    }
}
