//! Exact transcript equivalence for the structured fresh-MLE prover path.
use super::*;

fn assert_prefix_equal(left: &Prefix, right: &Prefix) {
    assert_eq!(left.point, right.point);
    assert_eq!(left.values, right.values);
    assert_eq!(left.weights, right.weights);
    assert_eq!(left.message, right.message);
    assert_eq!(left.matrix, right.matrix);
}

// Freeze the old sumcheck algorithm independently of the optimized state:
// copy full messages, materialize every form, and use native separate folds.
fn full_clone_reference_prefix(
    cfg: &CommitmentConfig,
    p: &mut P,
    messages: &[&Buffer<Fr>],
    matrices: &[&Buffer<Fr>],
    forms: &[Buffer<Fr>],
    sigmas: &[Fr],
) -> Prefix {
    let beta: Fr = p.verifier_message();
    let bs = powers(beta, Fr::ONE, messages.len());
    let mut sum = sigmas.iter().zip(&bs).map(|(&s, &b)| s * b).sum::<Fr>();
    let mut pairs = messages
        .iter()
        .zip(forms)
        .map(|(m, f)| ((*m).clone(), f.clone()))
        .collect::<Vec<_>>();
    let mut point = Vec::new();
    for _ in 0..cfg.vector_size().ilog2() {
        let (a, c) = pairs
            .iter()
            .zip(&bs)
            .map(|((m, f), &b)| {
                let (a, c) = m.sumcheck_polynomial(f);
                (a * b, c * b)
            })
            .fold((Fr::ZERO, Fr::ZERO), |(a, b), (c, d)| (a + c, b + d));
        p.prover_message(&a);
        p.prover_message(&c);
        let r: Fr = p.verifier_message();
        sum = (c * r + sum - c) * r + a;
        for (m, f) in &mut pairs {
            m.fold_pair(f, r);
        }
        point.push(r);
    }
    let values = pairs
        .iter()
        .map(|(m, _)| m.to_slice()[0])
        .collect::<Vec<_>>();
    p.prover_messages(&values);
    let gamma: Fr = p.verifier_message();
    let weights = powers(gamma, Fr::ONE, messages.len());
    let combine = |buffers: &[&Buffer<Fr>], len: usize| {
        Buffer::from(
            (0..len)
                .map(|j| {
                    buffers
                        .iter()
                        .zip(&weights)
                        .map(|(b, &w)| b.to_slice()[j] * w)
                        .sum::<Fr>()
                })
                .collect::<Vec<_>>(),
        )
    };
    Prefix {
        point,
        values,
        weights: weights.clone(),
        message: combine(messages, cfg.vector_size()),
        matrix: combine(matrices, cfg.size()),
        leaves: None,
    }
}

#[test]
fn implicit_mle_prefix_preserves_dense_transcript_and_invalid_sigma_rejection() {
    // Different RS layouts and arities, including a mixture of all supported
    // covectors. General Batch/Canopy claims always use their dense prover form.
    for (log_n, width, rate, arity) in [
        (0, 0, 1, 3),
        (1, 0, 1, 1),
        (4, 0, 1, 1),
        (6, 1, 2, 2),
        (8, 3, 1, 5),
        (7, 3, 2, 17),
        (16, 3, 1, 8),
    ] {
        let cfg = commitment_config(log_n, rate, width, 136.).unwrap();
        for case in 0..3 {
            let seed = 181 + case;
            let messages = (0..arity)
                .map(|j| match (case, j % 3) {
                    (1, 0) => Buffer::from(vec![Fr::ZERO; cfg.vector_size()]),
                    (1, 1) => Buffer::from(vec![elem(seed, 100, 0); cfg.vector_size()]),
                    _ => message(log_n, seed, 200 + j as u64),
                })
                .collect::<Vec<_>>();
            let matrices = messages.iter().map(|m| encode(&cfg, m)).collect::<Vec<_>>();
            let forms = (0..arity)
                .map(|j| {
                    if case == 2 && j % 3 == 1 {
                        prior_form(&cfg, seed, 300 + j as u64)
                    } else if case == 2 && j % 3 == 2 {
                        Form::Canopy {
                            point: point(log_n, seed, 400 + j as u64),
                            gamma: elem(seed, 500, j as u64),
                            indices: vec![0, cfg.vector_size() / 2, cfg.vector_size() - 1, 0],
                        }
                    } else {
                        let mut z = point(log_n, seed, 600 + j as u64);
                        if case == 1 {
                            for (i, x) in z.iter_mut().enumerate() {
                                *x = if (i + j) % 2 == 0 { Fr::ZERO } else { Fr::ONE };
                            }
                        }
                        Form::Mle(z)
                    }
                })
                .collect::<Vec<_>>();
            let dense_forms = forms.iter().map(Form::materialize).collect::<Vec<_>>();
            let honest_sigmas = messages
                .iter()
                .zip(&dense_forms)
                .map(|(m, f)| m.dot(f))
                .collect::<Vec<_>>();
            let message_refs = messages.iter().collect::<Vec<_>>();
            let matrix_refs = matrices.iter().collect::<Vec<_>>();
            for corrupt_sigma in [false, true] {
                let mut sigmas = honest_sigmas.clone();
                if corrupt_sigma {
                    sigmas[0] += Fr::ONE;
                }
                let mut input = PublicInput::new();
                input.integer(arity);
                for (form, &sigma) in forms.iter().zip(&sigmas) {
                    form.bind(&mut input);
                    input.field(sigma);
                }
                let ds = DomainSeparator::protocol(&cfg)
                    .session(&"section8/prefix-equivalence/v1")
                    .instance(&input);
                let mut dense_p = P::new(&ds, Sponge::default());
                let dense = full_clone_reference_prefix(
                    &cfg,
                    &mut dense_p,
                    &message_refs,
                    &matrix_refs,
                    &dense_forms,
                    &sigmas,
                );
                let mut implicit_p = P::new(&ds, Sponge::default());
                let implicit = prefix_prove(
                    &cfg,
                    &mut implicit_p,
                    &message_refs,
                    &matrix_refs,
                    forms.iter().map(ProverForm::from_form).collect(),
                    &sigmas,
                    &mut ReductionMeasurement::default(),
                );
                assert_prefix_equal(&dense, &implicit);
                // Equality of the following challenge additionally checks that
                // the same sponge state survives the whole prefix, including
                // the final gamma when arity=1 makes its geometric weight ONE.
                let dense_next: Fr = dense_p.verifier_message();
                let implicit_next: Fr = implicit_p.verifier_message();
                assert_eq!(dense_next, implicit_next);
                dense_p.prover_message(&dense_next);
                implicit_p.prover_message(&implicit_next);
                let dense_proof = dense_p.proof();
                let implicit_proof = implicit_p.proof();
                assert_eq!(dense_proof, implicit_proof);
                // This is the unchanged dense public verifier, including for
                // an inconsistent input sigma. No inferred claim is used by
                // the implicit prover's coefficient computation.
                let mut v = V::new(&ds, &implicit_proof, Sponge::default());
                let verified = prefix_verify(&cfg, &mut v, &dense_forms, &sigmas);
                if corrupt_sigma {
                    assert!(verified.is_err(), "invalid sigma accepted");
                } else {
                    let (point, values, weights) = verified.unwrap();
                    assert_eq!(point, implicit.point);
                    assert_eq!(values, implicit.values);
                    assert_eq!(weights, implicit.weights);
                    let next: Fr = v.verifier_message();
                    assert_eq!(next, implicit_next);
                    assert_eq!(read::<Fr>(&mut v).unwrap(), next);
                    v.check_eof().unwrap();
                }
            }
        }
    }
}

#[test]
fn implicit_mle_handles_zero_equality_factor_without_division() {
    for log_n in [1, 3, 7] {
        let z = point(log_n, 191, 700);
        let form = Form::Mle(z.clone());
        let mut dense = ProverForm::Dense(form.materialize());
        let mut implicit = ProverForm::from_form(&form);
        let mut dense_message = message(log_n, 191, 800);
        let mut implicit_message = dense_message.clone();
        for round in 0..log_n {
            assert_eq!(
                dense.polynomial(&dense_message),
                implicit.polynomial(&implicit_message)
            );
            let challenge = if round == 0 {
                Fr::ONE - z[0]
            } else {
                elem(191, 900, round as u64)
            };
            dense.fold_pair(&mut dense_message, challenge);
            implicit.fold_pair(&mut implicit_message, challenge);
            assert_eq!(dense_message, implicit_message);
            if let ProverForm::Dense(values) = &dense {
                assert!(values.to_slice().iter().all(|&x| x == Fr::ZERO));
            } else {
                unreachable!();
            }
            if let ProverForm::Mle {
                scale, round: done, ..
            } = &implicit
            {
                assert_eq!(*scale, Fr::ZERO);
                assert_eq!(*done, round + 1);
            } else {
                unreachable!();
            }
        }
    }
}

#[test]
fn borrowed_first_round_and_fused_mle_match_native_folds() {
    for threads in [1, 8] {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap()
            .install(|| {
                // The last size crosses the native parallel workload threshold.
                for log_n in [1, 2, 7, 14] {
                    for mode in 0..4 {
                        let mut z = point(log_n, 293, 1100);
                        if mode == 1 {
                            for (i, x) in z.iter_mut().enumerate() {
                                *x = if i % 2 == 0 { Fr::ZERO } else { Fr::ONE };
                            }
                        }
                        let original = match mode {
                            1 => Buffer::from(vec![Fr::ONE; 1 << log_n]),
                            2 => Buffer::from(vec![Fr::ZERO; 1 << log_n]),
                            _ => message(log_n, 293, 1200),
                        };
                        let snapshot = original.clone();
                        let form = Form::Mle(z.clone());
                        let mut dense_form = ProverForm::Dense(form.materialize());
                        let mut implicit_form = ProverForm::from_form(&form);
                        let mut dense_message = original.clone();
                        let mut state = SumcheckMessage::Borrowed(&original);
                        assert!(std::ptr::eq(state.buffer(), &original));
                        for round in 0..log_n {
                            assert_eq!(
                                dense_form.polynomial(&dense_message),
                                implicit_form.polynomial(state.buffer()),
                                "threads={threads} log_n={log_n} mode={mode} round={round}"
                            );
                            let r = if mode == 3 && round == log_n / 2 {
                                Fr::ONE - z[round]
                            } else if mode == 1 {
                                if round % 2 == 0 {
                                    Fr::ZERO
                                } else {
                                    Fr::ONE
                                }
                            } else {
                                elem(293, 1300, round as u64)
                            };
                            dense_form.fold_pair(&mut dense_message, r);
                            state.fold_pair(&mut implicit_form, r);
                            assert!(matches!(state, SumcheckMessage::Folded(_)));
                            assert_eq!(&dense_message, state.buffer());
                            assert_eq!(state.buffer().len(), 1 << (log_n - round - 1));
                        }
                        assert_eq!(original, snapshot, "borrowed source was modified");
                    }
                }
            });
    }
}

// Diagnostic only: fixed challenges isolate witness folding from the rest of
// the reduction. Run explicitly in release mode; this is not a timing assertion.
#[test]
#[ignore]
fn sumcheck_borrowed_fusion_release_diagnostic() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap()
        .install(|| {
            let log_n = 20;
            let messages = (0..9).map(|j| message(log_n, 391, j)).collect::<Vec<_>>();
            let points = (0..9).map(|j| point(log_n, 391, 100 + j)).collect::<Vec<_>>();
            let challenges = point(log_n, 391, 200);
            let run = |fused: bool| {
                let started = Instant::now();
                let mut trace = Vec::new();
                for (message, z) in messages.iter().zip(&points) {
                    if fused {
                        let mut state = SumcheckMessage::Borrowed(message);
                        let mut form = ProverForm::from_form(&Form::Mle(z.clone()));
                        for &r in &challenges {
                            trace.push(form.polynomial(state.buffer()));
                            state.fold_pair(&mut form, r);
                        }
                        trace.push((state.buffer().to_slice()[0], Fr::ZERO));
                        drop(state);
                    } else {
                        let mut state = message.clone();
                        let mut scale = Fr::ONE;
                        for (round, &r) in challenges.iter().enumerate() {
                            let (low, high) = state.to_slice().split_at(state.len() / 2);
                            let l0 = multilinear_extend(low, &z[round + 1..]);
                            let l1 = multilinear_extend(high, &z[round + 1..]);
                            trace.push((scale * (Fr::ONE - z[round]) * l0, scale * (l1 - l0)));
                            state.fold(r);
                            scale *= Fr::ONE - z[round] - r;
                        }
                        trace.push((state.to_slice()[0], Fr::ZERO));
                        drop(state);
                    }
                }
                (started.elapsed().as_secs_f64(), trace)
            };
            let (_, reference) = run(false);
            let (_, optimized) = run(true);
            assert_eq!(reference, optimized);
            for repetition in 0..2 {
                let order = if repetition == 0 { [false, true] } else { [true, false] };
                for fused in order {
                    let (seconds, trace) = run(fused);
                    assert_eq!(trace, reference);
                    eprintln!("sumcheck_kernel,N=20,sources=9,threads=8,rep={repetition},fused={fused},seconds={seconds:.9}");
                }
            }
        });
}
