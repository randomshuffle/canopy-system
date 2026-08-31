//! Executable, verifier-checked WHIR prover used by the timing experiments.
//!
//! The implementation is the official arkworks WHIR prover pinned in
//! `Cargo.toml`. This wrapper fixes the paper parameters (128-bit security,
//! selectable Johnson/capacity decoding, rate supplied by the caller,
//! four-variable folds, BLAKE3, and zero proof of work) and exposes stable
//! timing/accounting data. Capacity mode is conjectural.

use std::{
    sync::{Arc, Once},
    time::{Duration, Instant},
};

use anyhow::{ensure, Result};
use whir_ref::{
    algebra::{
        embedding::Identity,
        linear_form::{Evaluate, LinearForm, MultilinearExtension},
        ntt::{ReedSolomon, NTT},
    },
    bits::Bits,
    buffer::{Buffer, DefaultRs},
    hash,
    parameters::ProtocolParameters,
    protocols::{params::DecodingRegime, whir::Config},
    transcript::{codecs::Empty, DomainSeparator, ProverState, VerifierState},
};

use crate::{whir::WhirDecodingRegime, Fr, FIELD_BITS};

pub const SECURITY_BITS: usize = 128;
pub const FOLDING_LOG: usize = 4;
pub const POW_BITS: usize = 0;

fn register_field_ntt() {
    static REGISTER: Once = Once::new();
    REGISTER.call_once(|| {
        NTT.insert(Arc::new(DefaultRs::<Fr>::new_from_fftfield()) as Arc<dyn ReedSolomon<Fr>>);
    });
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
    pub proving_time: Duration,
    pub verification_time: Duration,
    pub proof_bytes: usize,
    pub oracle_geometry: Vec<OracleGeometry>,
}

fn config(
    message_length: usize,
    log_inverse_rate: usize,
    decoding_regime: WhirDecodingRegime,
) -> Result<Config<Identity<Fr>>> {
    register_field_ntt();
    ensure!(
        message_length.is_power_of_two(),
        "WHIR message length must be a power of two"
    );
    ensure!(log_inverse_rate > 0, "WHIR rate must be below one");
    let parameters = ProtocolParameters {
        decoding_regime: match decoding_regime {
            WhirDecodingRegime::Johnson => DecodingRegime::Johnson,
            WhirDecodingRegime::Capacity => DecodingRegime::Capacity,
        },
        starting_log_inv_rate: log_inverse_rate,
        initial_folding_factor: FOLDING_LOG,
        folding_factor: FOLDING_LOG,
        security_level: SECURITY_BITS,
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
        config.security_level(1, 1) >= SECURITY_BITS as f64,
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

    let config = config(message_length, log_inverse_rate, decoding_regime)?;
    let geometry = oracle_geometry(&config);
    let domain = DomainSeparator::protocol(&config)
        .session(&"canopy-system/paper-experiments/v1")
        .instance(&Empty);
    let mut prover_state = ProverState::new_std(&domain);
    let buffers: Vec<_> = vectors
        .iter()
        .map(|vector| Buffer::from(vector.as_slice()))
        .collect();

    let commit_start = Instant::now();
    let witnesses: Vec<_> = buffers
        .iter()
        .map(|buffer| config.commit(&mut prover_state, &[buffer]))
        .collect();
    let commitment_time = commit_start.elapsed();

    let num_variables = message_length.ilog2() as usize;
    let point = vec![Fr::from(7u64); num_variables];
    let evaluation_form = MultilinearExtension::new(point.clone());
    let evaluations: Vec<_> = vectors
        .iter()
        .map(|vector| evaluation_form.evaluate(config.embedding(), vector))
        .collect();
    let prove_forms: Vec<Box<dyn LinearForm<Fr>>> =
        vec![Box::new(MultilinearExtension::new(point.clone()))];
    let buffer_refs: Vec<_> = buffers.iter().collect();
    let witness_refs: Vec<_> = witnesses.iter().collect();

    let prove_start = Instant::now();
    let _final_claim = config.prove(
        &mut prover_state,
        &buffer_refs,
        witness_refs,
        prove_forms,
        Buffer::from(evaluations.as_slice()),
    );
    let proving_time = prove_start.elapsed();
    let proof = prover_state.proof();
    let proof_bytes = proof.narg_string.len() + proof.hints.len();

    let verify_start = Instant::now();
    let mut verifier_state = VerifierState::new_std(&domain, &proof);
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
}
