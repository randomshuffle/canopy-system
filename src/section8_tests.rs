//! Protocol regression tests. Rejection is checked against explicit public
//! statements, and outgoing relations are checked by an independent full witness
//! decider. The decider work is intentionally outside benchmark measurements.
use super::*;

#[path = "section8_batch_tests.rs"]
mod batch;
#[path = "section8_prefix_tests.rs"]
mod prefix;
#[path = "section8_profile_tests.rs"]
mod profile;

fn assert_rejected(label: &str, verify: impl FnOnce() -> Result<usize>) {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(verify)) {
        Ok(result) => assert!(result.is_err(), "accepted {label}"),
        Err(payload) => {
            let text = payload
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| payload.downcast_ref::<&str>().copied())
                .unwrap_or("");
            // whir's debug-only transcript pattern records the honest proof's
            // interaction types. A changed challenge can change that pattern.
            // Bounds, arithmetic, allocation, and unrelated assertion panics
            // must never count as cryptographic rejection. Release tests require Err.
            if !cfg!(debug_assertions) || !text.contains("Transcript error: Expected interaction") {
                std::panic::resume_unwind(payload);
            }
        }
    }
}

fn check_ifold_decider(
    cfg: &CommitmentConfig,
    layer_cfg: &CommitmentConfig,
    height: usize,
    execution: &IfoldExecution,
) {
    let out = &execution.public_output;
    assert_eq!(
        execution.output_message.dot(&out.large_form.materialize()),
        out.large_sigma
    );
    // Re-encode and re-commit from the actual full output polynomial. This
    // independently checks the Canopy codeword, layer root, and OOD value.
    let checked = prepare_canopy(&execution.output_message, cfg, layer_cfg, height);
    assert_eq!(checked.base, execution.output.base);
    assert_eq!(checked.small.message, execution.output.small.message);
    assert_eq!(checked.small.root, out.root);
    assert_eq!(
        checked.small.witness.matrix,
        execution.output.small.witness.matrix
    );
    assert_eq!(
        multilinear_extend(checked.small.message.to_slice(), &out.layer_point),
        out.layer_value
    );
    assert_eq!(execution.residual.len(), execution.public_inputs.len());
    for ((source, input), (form, sigma)) in execution
        .residual
        .iter()
        .zip(&execution.public_inputs)
        .zip(&out.residual_claims)
    {
        // The residual must use the *same* incoming layer commitment, with no
        // independently prepared replacement oracle at the ifold -> iacc handoff.
        assert_eq!(source.root, input.root);
        assert_eq!(&source.form, form);
        assert_eq!(source.sigma, *sigma);
        check_batch_output(layer_cfg, source, true).unwrap();
    }
}

#[test]
fn end_to_end_canopy_and_controls_across_layouts_arities_and_heights() {
    // Includes scalar/packed leaves, zero-bottom-depth, sparse paths, the
    // smallest legal layer, and zero/one/multiple previous accumulators.
    for (log_n, width, height, casts, priors, seed) in [
        (8, 0, 3, 1, 0, 11),
        (10, 1, 4, 2, 1, 19),
        (10, 3, 3, 4, 3, 23),
        (10, 3, 5, 2, 0, 29),
        (10, 3, 8, 1, 1, 31),
        (10, 3, 4, 64, 1, 37),
        (16, 3, 4, 8, 1, 41),
    ] {
        let cfg = commitment_config(log_n, 1, width, 136.).unwrap();
        let layer_cfg = commitment_config(log_n + 1 - height, height - 1, width, 136.).unwrap();
        let execution = run_ifold(log_n, casts, height, width, 136., seed, true).unwrap();
        assert_eq!(
            verify_ifold_proof(
                &cfg,
                &layer_cfg,
                height,
                &execution.public_inputs,
                &execution.public_output,
                &execution.proof
            )
            .unwrap(),
            execution.measurement.auth_hashes
        );
        check_ifold_decider(&cfg, &layer_cfg, height, &execution);
        let run = run_canopy(log_n, casts, priors, height, width, 136., seed, true).unwrap();
        assert!(run.ifold.verified && run.iacc.verified);
        assert_eq!(run.ifold.sources, casts + 1);
        assert_eq!(run.iacc.sources, casts + priors + 1);
        assert_eq!(run.ifold.queries, 317);
        assert_eq!(run.iacc.queries, layer_cfg.in_domain_samples());
        assert_eq!(run.ifold.output_instance_bytes, HASH_BYTES + FIELD_BYTES);
        assert_eq!(run.iacc.output_instance_bytes, HASH_BYTES);
        for measurement in [&run.ifold, &run.iacc] {
            assert_eq!(
                measurement.proof_field_bytes
                    + measurement.proof_hash_bytes
                    + measurement.proof_framing_bytes,
                measurement.proof_bytes
            );
            assert_eq!(
                measurement.proof_bytes + measurement.output_instance_bytes,
                measurement.serialized_total_bytes
            );
        }
        for rate in [1, 2] {
            let control = run_merkle(log_n, casts, priors, rate, width, 136., seed, true).unwrap();
            assert!(control.verified);
            assert_eq!(control.sources, casts + priors);
            assert_eq!(control.queries, if rate == 1 { 317 } else { 147 });
        }
    }
}

fn retain_canopy_output(execution: IfoldExecution) -> CanopySource {
    CanopySource {
        message: execution.output_message,
        data: execution.output,
        form: execution.public_output.large_form,
        sigma: execution.public_output.large_sigma,
    }
}

fn public_canopy_source(source: &CanopySource) -> CanopyPublicInput {
    CanopyPublicInput {
        root: source.data.small.root,
        layer_point: source.data.layer_point.clone(),
        layer_value: source.data.layer_value,
        form: source.form.clone(),
        sigma: source.sigma,
    }
}

#[test]
fn actual_canopy_and_merkle_accumulators_survive_three_steps_and_tampering() {
    let (log_n, width, height) = (10, 3, 5);
    let cfg = commitment_config(log_n, 1, width, 136.).unwrap();
    let layer_cfg = commitment_config(log_n + 1 - height, height - 1, width, 136.).unwrap();

    // Bootstrap both relations from real protocol outputs. Every subsequent
    // step must carry the full Canopy Batch claim as well as its Merkle peer.
    let bootstrap = run_ifold(log_n, 2, height, width, 136., 37, true).unwrap();
    check_ifold_decider(&cfg, &layer_cfg, height, &bootstrap);
    let (_, mut merkle_accumulator, _) =
        run_batch(&layer_cfg, &bootstrap.residual, "bootstrap", 0., true).unwrap();
    check_batch_output(&layer_cfg, &merkle_accumulator, true).unwrap();
    let mut canopy_accumulator = retain_canopy_output(bootstrap);
    let mut stale_canopy: Option<CanopyPublicInput> = None;
    let mut stale_merkle: Option<BatchPublicInput> = None;

    for (round, fresh_count) in [1, 2, 4].into_iter().enumerate() {
        let seed = 41 + round as u64;
        let previous_canopy = public_canopy_source(&canopy_accumulator);
        let previous_merkle = BatchPublicInput::from(&merkle_accumulator);
        assert!(matches!(&previous_canopy.form, Form::Batch { .. }));
        assert!(matches!(&previous_merkle.form, Form::Batch { .. }));
        let mut canopy_sources = (0..fresh_count)
            .map(|j| {
                let m = message(log_n, seed, 1000 + j as u64);
                let form = Form::Mle(point(log_n, seed, 2000 + j as u64));
                let sigma = m.dot(&form.materialize());
                let data = prepare_canopy(&m, &cfg, &layer_cfg, height);
                CanopySource {
                    message: m,
                    data,
                    form,
                    sigma,
                }
            })
            .collect::<Vec<_>>();
        canopy_sources.push(canopy_accumulator);
        let mut execution = execute_ifold(
            &cfg,
            &layer_cfg,
            height,
            canopy_sources,
            0.,
            CommitmentProfile::default(),
            true,
        )
        .unwrap();
        assert_eq!(execution.measurement.sources, fresh_count + 1);
        assert_eq!(execution.residual.len(), fresh_count + 1);
        for input in &execution.public_inputs[..fresh_count] {
            assert!(matches!(&input.form, Form::Mle(_)));
        }
        let carried = &execution.public_inputs[fresh_count];
        assert_eq!(carried.root, previous_canopy.root);
        assert_eq!(carried.layer_point, previous_canopy.layer_point);
        assert_eq!(carried.layer_value, previous_canopy.layer_value);
        assert_eq!(carried.form, previous_canopy.form);
        assert_eq!(carried.sigma, previous_canopy.sigma);
        assert!(matches!(&carried.form, Form::Batch { .. }));
        check_ifold_decider(&cfg, &layer_cfg, height, &execution);

        let verify_ifold = |inputs: &[CanopyPublicInput], proof: &Proof| {
            verify_ifold_proof(
                &cfg,
                &layer_cfg,
                height,
                inputs,
                &execution.public_output,
                proof,
            )
        };
        verify_ifold(&execution.public_inputs, &execution.proof).unwrap();
        // The real previous Canopy output has more public structure than an
        // MLE claim. Bind every Batch term and both Canopy commitment OOD data.
        for component in 0..9 {
            let mut changed = execution.public_inputs.clone();
            let prior = &mut changed[fresh_count];
            match component {
                0 => prior.root.0[0] ^= 1,
                1 => prior.sigma += Fr::ONE,
                2..=5 => {
                    let Form::Batch {
                        point,
                        alpha,
                        lambda,
                        indices,
                        rows,
                        ..
                    } = &mut prior.form
                    else {
                        unreachable!()
                    };
                    match component {
                        2 => point[0] += Fr::ONE,
                        3 => *alpha += Fr::ONE,
                        4 => *lambda += Fr::ONE,
                        _ => indices[0] = (indices[0] + 1) % *rows,
                    }
                }
                6 => prior.layer_point[0] += Fr::ONE,
                7 => prior.layer_value += Fr::ONE,
                8 => {
                    let Form::Batch { point, .. } = &prior.form else {
                        unreachable!()
                    };
                    prior.form = Form::Mle(point.clone());
                }
                _ => unreachable!(),
            }
            assert_rejected(
                &format!("round {round} carried Canopy component {component}"),
                || verify_ifold(&changed, &execution.proof),
            );
        }
        let mut changed = execution.public_inputs.clone();
        assert_ne!(previous_canopy.root, previous_merkle.root);
        changed[fresh_count].root = previous_merkle.root;
        assert_rejected("cross-wired previous Canopy/Merkle roots", || {
            verify_ifold(&changed, &execution.proof)
        });
        if let Some(stale) = &stale_canopy {
            let mut changed = execution.public_inputs.clone();
            changed[fresh_count] = stale.clone();
            assert_rejected("stale but individually valid Canopy accumulator", || {
                verify_ifold(&changed, &execution.proof)
            });
        }
        let block_bytes = execution.proof.hints.len() / (fresh_count + 1);
        assert_eq!(execution.proof.hints.len() % (fresh_count + 1), 0);
        let mut changed = execution.proof.clone();
        changed.hints[fresh_count * block_bytes + 8] ^= 1;
        assert_rejected("carried Canopy input bottom opening", || {
            verify_ifold(&execution.public_inputs, &changed)
        });

        let mut sources = std::mem::take(&mut execution.residual);
        // Transfer the actual old Merkle witness, without preparing a synthetic
        // replacement or forgetting the claim accumulated by the previous step.
        sources.push(merkle_accumulator);
        let public = sources
            .iter()
            .map(BatchPublicInput::from)
            .collect::<Vec<_>>();
        let carried = public.last().unwrap();
        assert_eq!(carried.root, previous_merkle.root);
        assert_eq!(carried.form, previous_merkle.form);
        assert_eq!(carried.sigma, previous_merkle.sigma);
        let (measurement, output, proof) =
            run_batch(&layer_cfg, &sources, "iacc_test", 0., true).unwrap();
        assert_eq!(measurement.sources, fresh_count + 2);
        let expected = BatchPublicOutput::from(&output);
        let verify_merkle = |inputs: &[BatchPublicInput], proof: &Proof| {
            verify_batch_proof(&layer_cfg, inputs, &expected, proof)
        };
        assert_eq!(
            verify_merkle(&public, &proof).unwrap(),
            measurement.auth_hashes
        );
        check_batch_output(&layer_cfg, &output, true).unwrap();
        for component in 0..6 {
            let mut changed = public.clone();
            let prior = changed.last_mut().unwrap();
            match component {
                0 => prior.root.0[0] ^= 1,
                1 => prior.sigma += Fr::ONE,
                2..=5 => {
                    let Form::Batch {
                        point,
                        alpha,
                        lambda,
                        indices,
                        rows,
                        ..
                    } = &mut prior.form
                    else {
                        unreachable!()
                    };
                    match component {
                        2 => point[0] += Fr::ONE,
                        3 => *alpha += Fr::ONE,
                        4 => *lambda += Fr::ONE,
                        _ => indices[0] = (indices[0] + 1) % *rows,
                    }
                }
                _ => unreachable!(),
            }
            assert_rejected(
                &format!("round {round} carried Merkle component {component}"),
                || verify_merkle(&changed, &proof),
            );
        }
        if let Some(stale) = &stale_merkle {
            let mut changed = public.clone();
            *changed.last_mut().unwrap() = stale.clone();
            assert_rejected("stale but individually valid Merkle accumulator", || {
                verify_merkle(&changed, &proof)
            });
        }
        let block_bytes = proof.hints.len() / sources.len();
        assert_eq!(proof.hints.len() % sources.len(), 0);
        let mut changed = proof.clone();
        changed.hints[(sources.len() - 1) * block_bytes + 8] ^= 1;
        assert_rejected("carried Merkle input queried field", || {
            verify_merkle(&public, &changed)
        });
        // The public verifier still works after all incoming secret witnesses
        // have been discarded.
        drop(sources);
        verify_merkle(&public, &proof).unwrap();
        verify_ifold(&execution.public_inputs, &execution.proof).unwrap();
        stale_canopy = Some(previous_canopy);
        stale_merkle = Some(previous_merkle);
        canopy_accumulator = retain_canopy_output(execution);
        merkle_accumulator = output;
    }
}

#[test]
fn ifold_rejects_public_input_changes_and_replays() {
    let (log_n, width, height) = (10, 3, 5);
    let cfg = commitment_config(log_n, 1, width, 136.).unwrap();
    let layer_cfg = commitment_config(6, 4, width, 136.).unwrap();
    let execution = run_ifold(log_n, 2, height, width, 136., 47, false).unwrap();
    for j in 0..2 {
        for kind in 0..5 {
            let mut input = execution.public_inputs.clone();
            match kind {
                0 => input[j].root.0[0] ^= 1,
                1 => input[j].sigma += Fr::ONE,
                2 => {
                    let Form::Mle(point) = &mut input[j].form else {
                        unreachable!()
                    };
                    point[0] += Fr::ONE;
                }
                3 => input[j].layer_point[0] += Fr::ONE,
                4 => input[j].layer_value += Fr::ONE,
                _ => unreachable!(),
            }
            assert_rejected(&format!("ifold input {j} component {kind}"), || {
                verify_ifold_proof(
                    &cfg,
                    &layer_cfg,
                    height,
                    &input,
                    &execution.public_output,
                    &execution.proof,
                )
            });
        }
    }
    let mut reordered = execution.public_inputs.clone();
    reordered.swap(0, 1);
    assert_rejected("reordered ifold sources", || {
        verify_ifold_proof(
            &cfg,
            &layer_cfg,
            height,
            &reordered,
            &execution.public_output,
            &execution.proof,
        )
    });
    let other = run_ifold(log_n, 2, height, width, 136., 53, false).unwrap();
    assert_rejected("replay under another input seed", || {
        verify_ifold_proof(
            &cfg,
            &layer_cfg,
            height,
            &other.public_inputs,
            &execution.public_output,
            &execution.proof,
        )
    });
    let other_layer = commitment_config(7, 3, width, 136.).unwrap();
    assert_rejected("replay under another height", || {
        verify_ifold_proof(
            &cfg,
            &other_layer,
            4,
            &execution.public_inputs,
            &execution.public_output,
            &execution.proof,
        )
    });
}

#[test]
fn ifold_rejects_each_outgoing_claim_change() {
    let cfg = commitment_config(10, 1, 3, 136.).unwrap();
    let layer_cfg = commitment_config(6, 4, 3, 136.).unwrap();
    let execution = run_ifold(10, 2, 5, 3, 136., 59, false).unwrap();
    for kind in 0..8 {
        let mut out = execution.public_output.clone();
        match kind {
            0 => out.root.0[0] ^= 1,
            1 => out.layer_point[0] += Fr::ONE,
            2 => out.layer_value += Fr::ONE,
            3 => out.large_sigma += Fr::ONE,
            4 => {
                let Form::Batch { lambda, .. } = &mut out.large_form else {
                    unreachable!()
                };
                *lambda += Fr::ONE;
            }
            5 => {
                out.residual_claims.pop();
            }
            6 => out.residual_claims.swap(0, 1),
            7 => out.residual_claims[1].1 += Fr::ONE,
            _ => unreachable!(),
        }
        assert_rejected(&format!("ifold output component {kind}"), || {
            verify_ifold_proof(
                &cfg,
                &layer_cfg,
                5,
                &execution.public_inputs,
                &out,
                &execution.proof,
            )
        });
    }
    for j in 0..2 {
        let mut out = execution.public_output.clone();
        let Form::Canopy { gamma, .. } = &mut out.residual_claims[j].0 else {
            unreachable!()
        };
        *gamma += Fr::ONE;
        assert_rejected(&format!("residual {j} covector"), || {
            verify_ifold_proof(
                &cfg,
                &layer_cfg,
                5,
                &execution.public_inputs,
                &out,
                &execution.proof,
            )
        });
    }
}

#[test]
fn ifold_rejects_each_proof_component_and_each_sources_bottom_path() {
    let cfg = commitment_config(10, 1, 3, 136.).unwrap();
    let layer_cfg = commitment_config(6, 4, 3, 136.).unwrap();
    let execution = run_ifold(10, 2, 5, 3, 136., 61, false).unwrap();
    let verify = |proof: &Proof| {
        verify_ifold_proof(
            &cfg,
            &layer_cfg,
            5,
            &execution.public_inputs,
            &execution.public_output,
            proof,
        )
    };
    let output_offset = (2 * 10 + 2) * FIELD_BYTES;
    // Every sumcheck round's two coefficients and every final source evaluation.
    for field in 0..22 {
        let mut proof = execution.proof.clone();
        proof.narg_string[field * FIELD_BYTES] ^= 1;
        assert_rejected(&format!("ifold transcript field {field}"), || {
            verify(&proof)
        });
    }
    for (name, offset) in [
        ("output root", output_offset),
        ("layer OOD", output_offset + HASH_BYTES),
        ("code-switch OOD", output_offset + HASH_BYTES + FIELD_BYTES),
    ] {
        let mut proof = execution.proof.clone();
        proof.narg_string[offset] ^= 1;
        assert_rejected(name, || verify(&proof));
    }
    let mut cursor = 0;
    for j in 0..2 {
        let count = u64::from_le_bytes(
            execution.proof.hints[cursor..cursor + 8]
                .try_into()
                .unwrap(),
        ) as usize;
        assert!(count > 0);
        let values_end = cursor + 8 + count * FIELD_BYTES;
        let auth_bytes = execution.measurement.proof_hash_bytes / 2;
        assert!(
            auth_bytes > 0,
            "fixture must include actual authentication hashes"
        );
        for (name, offset) in [("row", cursor + 8), ("path", values_end + auth_bytes / 2)] {
            let mut proof = execution.proof.clone();
            proof.hints[offset] ^= 1;
            assert_rejected(&format!("source {j} bottom {name}"), || verify(&proof));
        }
        cursor = values_end + auth_bytes;
    }
    assert_eq!(cursor, execution.proof.hints.len());
}

#[test]
fn ifold_rejects_truncation_trailing_bytes_and_wrong_vector_lengths() {
    let cfg = commitment_config(10, 1, 3, 136.).unwrap();
    let layer_cfg = commitment_config(6, 4, 3, 136.).unwrap();
    let execution = run_ifold(10, 2, 5, 3, 136., 67, false).unwrap();
    let verify = |proof: &Proof| {
        verify_ifold_proof(
            &cfg,
            &layer_cfg,
            5,
            &execution.public_inputs,
            &execution.public_output,
            proof,
        )
    };
    let root = 22 * FIELD_BYTES;
    for length in [
        0,
        FIELD_BYTES - 1,
        root - 1,
        root + HASH_BYTES - 1,
        root + HASH_BYTES + FIELD_BYTES - 1,
        execution.proof.narg_string.len() - 1,
    ] {
        let mut proof = execution.proof.clone();
        proof.narg_string.truncate(length);
        assert_rejected(&format!("narg truncated to {length}"), || verify(&proof));
    }
    for length in [0, 7, 8 + FIELD_BYTES - 1, execution.proof.hints.len() - 1] {
        let mut proof = execution.proof.clone();
        proof.hints.truncate(length);
        assert_rejected(&format!("hints truncated to {length}"), || verify(&proof));
    }
    for hints in [false, true] {
        let mut proof = execution.proof.clone();
        if hints {
            proof.hints.push(0);
        } else {
            proof.narg_string.push(0);
        }
        assert_rejected("trailing proof data", || verify(&proof));
    }
    let honest_len = u64::from_le_bytes(execution.proof.hints[..8].try_into().unwrap());
    for length in [0, honest_len - 1, honest_len + 1] {
        let mut proof = execution.proof.clone();
        proof.hints[..8].copy_from_slice(&length.to_le_bytes());
        assert_rejected("wrong bottom vector length", || verify(&proof));
    }
}

#[test]
fn modified_bottom_hash_becomes_a_false_deferred_claim() {
    let cfg = commitment_config(10, 1, 3, 136.).unwrap();
    let layer_cfg = commitment_config(6, 4, 3, 136.).unwrap();
    let data = prepare_canopy(&message(10, 71, 1), &cfg, &layer_cfg, 5);
    // Deliberately sparse indices ensure actual siblings, merged paths, and
    // multiple touched Canopy blocks. ifold only derives these roots; iacc and
    // the outgoing membership relation must retain the obligation to check them.
    let indices = [0, 1, 8, 31];
    let (values, auth) = forest_open(&data, &indices, 8);
    assert!(!auth.is_empty());
    let (honest_roots, _) = forest_verify(&indices, &values, &auth, 8, 2).unwrap();
    for (&i, &root) in &honest_roots {
        assert_eq!(root, data.small.message.to_slice()[i]);
    }
    let mut bad_auth = auth.clone();
    bad_auth[0].0[0] ^= 1;
    let (bad_roots, _) = forest_verify(&indices, &values, &bad_auth, 8, 2).unwrap();
    assert_ne!(bad_roots, honest_roots);
    let (form, sigma) = canopy_claim(&data, &bad_roots, elem(71, 2, 0));
    assert_ne!(data.small.message.dot(&form.materialize()), sigma);
    let mut false_source = data.small;
    false_source.form = form;
    false_source.sigma = sigma;
    assert!(run_batch(&layer_cfg, &[false_source], "invalid_residual", 0., false).is_err());
}

#[test]
fn rejects_invalid_section8_parameters_before_work() {
    for (n, width, h, casts) in [
        (10, 3, 2, 1),
        (10, 3, 9, 1),
        (10, 3, 11, 1),
        (10, 3, 5, 0),
        (10, 11, 5, 1),
    ] {
        assert!(run_canopy(n, casts, 1, h, width, 136., 73, false).is_err());
    }
    assert!(run_merkle(10, 0, 0, 1, 3, 136., 73, false).is_err());
    assert!(run_merkle(10, usize::MAX, 1, 1, 3, 136., 73, false).is_err());
    for rate in [0, 3] {
        assert!(run_merkle(10, 1, 1, rate, 3, 136., 73, false).is_err());
    }
    for budget in [0., -1., f64::NAN, f64::INFINITY] {
        assert!(commitment_config(10, 1, 3, budget).is_err());
    }
}

#[test]
fn public_verifiers_reject_malformed_covectors_and_geometry_without_panicking() {
    let cfg = commitment_config(10, 1, 3, 136.).unwrap();
    let layer_cfg = commitment_config(6, 4, 3, 136.).unwrap();
    let execution = run_ifold(10, 2, 5, 3, 136., 79, false).unwrap();
    let mut inputs = execution.public_inputs.clone();
    let Form::Mle(point) = &mut inputs[0].form else {
        unreachable!()
    };
    point.pop();
    assert!(verify_ifold_proof(
        &cfg,
        &layer_cfg,
        5,
        &inputs,
        &execution.public_output,
        &execution.proof
    )
    .is_err());
    let mut inputs = execution.public_inputs.clone();
    inputs[0].layer_point.pop();
    assert!(verify_ifold_proof(
        &cfg,
        &layer_cfg,
        5,
        &inputs,
        &execution.public_output,
        &execution.proof
    )
    .is_err());
    for height in [0, 2, 11, usize::MAX] {
        assert!(verify_ifold_proof(
            &cfg,
            &layer_cfg,
            height,
            &execution.public_inputs,
            &execution.public_output,
            &execution.proof
        )
        .is_err());
    }
    let incompatible = commitment_config(6, 3, 3, 136.).unwrap();
    assert!(verify_ifold_proof(
        &cfg,
        &incompatible,
        5,
        &execution.public_inputs,
        &execution.public_output,
        &execution.proof
    )
    .is_err());
    let public = execution
        .residual
        .iter()
        .map(BatchPublicInput::from)
        .collect::<Vec<_>>();
    let (_, output, proof) =
        run_batch(&layer_cfg, &execution.residual, "shape-test", 0., false).unwrap();
    let expected = BatchPublicOutput::from(&output);
    for kind in 0..3 {
        let mut malformed = public.clone();
        let Form::Canopy { point, indices, .. } = &mut malformed[0].form else {
            unreachable!()
        };
        match kind {
            0 => {
                point.pop();
            }
            1 => indices.push(layer_cfg.vector_size()),
            2 => indices.push(usize::MAX),
            _ => unreachable!(),
        }
        // No catch_unwind: all malformed public shapes must return Err directly.
        assert!(verify_batch_proof(&layer_cfg, &malformed, &expected, &proof).is_err());
    }
    for kind in 0..5 {
        let mut malformed = expected.clone();
        let Form::Batch {
            point,
            rows,
            width,
            indices,
            ..
        } = &mut malformed.form
        else {
            unreachable!()
        };
        match kind {
            0 => {
                point.pop();
            }
            1 => *rows = 0,
            2 => *width = 0,
            3 => *width *= 2,
            4 => indices.push(*rows),
            _ => unreachable!(),
        }
        assert!(verify_batch_proof(&layer_cfg, &public, &malformed, &proof).is_err());
    }
}
