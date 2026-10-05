use crate::{benchmark_field_element, executable_whir, whir::WhirDecodingRegime, Fr};
use whir_ref::{
    algebra::linear_form::{Evaluate, LinearForm, MultilinearExtension},
    buffer::{Buffer, BufferOps},
    protocols::matrix_commit::Encodable,
    transcript::{codecs::Empty, DomainSeparator, ProverState, VerifierState},
};

#[test]
fn full_width_whir_proof_and_tampering() {
    crate::register_benchmark_ntt();
    assert_eq!(<Fr as Encodable>::encoded_size(), 32);
    let values: Vec<_> = (0..4096)
        .map(|i| benchmark_field_element(19, 0, i))
        .collect();
    let config = executable_whir::config(values.len(), 1, WhirDecodingRegime::Johnson, 3).unwrap();
    let domain = DomainSeparator::protocol(&config)
        .session(&"binary-whir/test")
        .instance(&Empty);
    let point: Vec<_> = (0..12).map(|i| benchmark_field_element(19, 1, i)).collect();
    let form = MultilinearExtension::new(point.clone());
    let evaluation = form.evaluate(config.embedding(), &values);
    let buffer = Buffer::from(values);
    let mut prover = ProverState::new_std(&domain);
    let witness = config.commit(&mut prover, &[&buffer]);
    let _final_claim = config.prove(
        &mut prover,
        &[&buffer],
        vec![&witness],
        vec![Box::new(form)],
        Buffer::from(vec![evaluation]),
    );
    let proof = prover.proof();
    let verify = |proof: &_| -> bool {
        let mut verifier = VerifierState::new_std(&domain, proof);
        let Ok(commitment) = config.receive_commitment(&mut verifier) else {
            return false;
        };
        let Ok(claim) = config.verify(&mut verifier, &[&commitment], &[evaluation]) else {
            return false;
        };
        claim
            .verify([&MultilinearExtension::new(point.clone()) as &dyn LinearForm<Fr>])
            .is_ok()
    };
    assert!(verify(&proof));
    let mut tampered = proof.clone();
    tampered.narg_string[0] ^= 1;
    assert!(!verify(&tampered));
    let mut tampered = proof.clone();
    let last = tampered.narg_string.len() - 1;
    tampered.narg_string[last] ^= 1;
    assert!(!verify(&tampered));
    let mut tampered = proof.clone();
    let last = tampered.hints.len() - 1;
    tampered.hints[last] ^= 1;
    assert!(!verify(&tampered));
    let second: Vec<_> = (0..4096)
        .map(|i| benchmark_field_element(23, 0, i))
        .collect();
    executable_whir::execute_benchmark_with_regime_and_folding(
        &[buffer.to_slice().to_vec(), second],
        1,
        WhirDecodingRegime::Johnson,
        3,
    )
    .unwrap();
}

#[test]
fn zero_polynomial_whir_roundtrip() {
    crate::register_benchmark_ntt();
    let run = executable_whir::execute_benchmark_with_regime_and_folding(
        &[vec![Fr::ZERO; 4096]],
        1,
        WhirDecodingRegime::Johnson,
        3,
    )
    .unwrap();
    assert!(run.proof_bytes > 0);
}
