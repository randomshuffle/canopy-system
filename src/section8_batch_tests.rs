use super::*;

struct BatchFixture {
    cfg: CommitmentConfig,
    sources: Vec<Prepared>,
    output: Prepared,
    proof: Proof,
}

impl BatchFixture {
    fn new(log_inv_rate: usize) -> Self {
        let cfg = commitment_config(10, log_inv_rate, 3, 136.0).unwrap();
        let sources = (0..3)
            .map(|j| {
                let form = if j == 2 {
                    prior_form(&cfg, 91, 1200)
                } else {
                    Form::Mle(point(10, 91, 1000 + j as u64))
                };
                prepare(&cfg, message(10, 91, 100 + j as u64), form)
            })
            .collect::<Vec<_>>();
        let (measurement, output, proof) =
            run_batch(&cfg, &sources, "batch-test", 0.0, false).unwrap();
        assert!(measurement.verified);
        let fixture = Self {
            cfg,
            sources,
            output,
            proof,
        };
        fixture.verify(&fixture.proof).unwrap();
        fixture
    }

    fn inputs(&self) -> Vec<BatchPublicInput> {
        self.sources.iter().map(BatchPublicInput::from).collect()
    }

    fn public_output(&self) -> BatchPublicOutput {
        BatchPublicOutput::from(&self.output)
    }

    fn verify(&self, proof: &Proof) -> Result<usize> {
        verify_batch_proof(&self.cfg, &self.inputs(), &self.public_output(), proof)
    }

    fn reject(&self, label: &str, proof: &Proof) {
        assert_rejected(label, || self.verify(proof));
    }

    /// Native IRS writes one vector and one authentication frontier per source.
    /// Common query indices give equal-sized frontiers for all source trees.
    fn hint_blocks(&self) -> Vec<(usize, usize, usize)> {
        let source_count = self.sources.len();
        assert_eq!(self.proof.hints.len() % source_count, 0);
        let block_bytes = self.proof.hints.len() / source_count;
        let row_fields = self.cfg.in_domain_samples() * self.cfg.interleaving_depth();
        let rows_bytes = row_fields * FIELD_BYTES;
        assert!(
            block_bytes > 8 + rows_bytes,
            "fixture needs authentication digests"
        );
        assert_eq!((block_bytes - 8 - rows_bytes) % HASH_BYTES, 0);
        (0..source_count)
            .map(|j| {
                let start = j * block_bytes;
                let declared =
                    u64::from_le_bytes(self.proof.hints[start..start + 8].try_into().unwrap());
                assert_eq!(declared as usize, row_fields);
                (start, start + 8, start + 8 + rows_bytes)
            })
            .collect()
    }
}

#[test]
fn batch_binds_every_public_source_and_output() {
    for rate in [1, 2] {
        let fixture = BatchFixture::new(rate);
        let original = fixture.inputs();
        let output = fixture.public_output();
        for j in 0..original.len() {
            let mut inputs = original.clone();
            inputs[j].root.0[0] ^= 1;
            assert_rejected(&format!("rate {rate}, source {j} root"), || {
                verify_batch_proof(&fixture.cfg, &inputs, &output, &fixture.proof)
            });

            let mut inputs = original.clone();
            inputs[j].sigma += Fr::ONE;
            assert_rejected(&format!("rate {rate}, source {j} sigma"), || {
                verify_batch_proof(&fixture.cfg, &inputs, &output, &fixture.proof)
            });

            let mut inputs = original.clone();
            match &mut inputs[j].form {
                Form::Mle(point) | Form::Batch { point, .. } => point[0] += Fr::ONE,
                Form::Canopy { .. } => unreachable!(),
            }
            assert_rejected(&format!("rate {rate}, source {j} form"), || {
                verify_batch_proof(&fixture.cfg, &inputs, &output, &fixture.proof)
            });
        }
        // Exercise the non-MLE covector's additional public components too.
        for component in 0..3 {
            let mut inputs = original.clone();
            if let Form::Batch {
                alpha,
                lambda,
                indices,
                rows,
                ..
            } = &mut inputs[2].form
            {
                match component {
                    0 => *alpha += Fr::ONE,
                    1 => *lambda += Fr::ONE,
                    _ => indices[0] = (indices[0] + 1) % *rows,
                }
            } else {
                unreachable!();
            }
            assert_rejected(
                &format!("rate {rate}, prior form component {component}"),
                || verify_batch_proof(&fixture.cfg, &inputs, &output, &fixture.proof),
            );
        }

        let mut swapped = original.clone();
        swapped.swap(0, 1);
        assert_rejected("source-order replay", || {
            verify_batch_proof(&fixture.cfg, &swapped, &output, &fixture.proof)
        });
        let mut missing = original.clone();
        missing.pop();
        assert_rejected("missing-source replay", || {
            verify_batch_proof(&fixture.cfg, &missing, &output, &fixture.proof)
        });
        let mut extra = original.clone();
        extra.push(original[0].clone());
        assert_rejected("extra-source replay", || {
            verify_batch_proof(&fixture.cfg, &extra, &output, &fixture.proof)
        });
        assert_rejected("zero-source replay", || {
            verify_batch_proof(&fixture.cfg, &[], &output, &fixture.proof)
        });
        let mut duplicated = original.clone();
        duplicated[2] = original[0].clone();
        assert_rejected("different-source replay", || {
            verify_batch_proof(&fixture.cfg, &duplicated, &output, &fixture.proof)
        });

        let mut changed = output.clone();
        changed.root.0[0] ^= 1;
        assert_rejected("output public root", || {
            verify_batch_proof(&fixture.cfg, &original, &changed, &fixture.proof)
        });
        let mut changed = output.clone();
        changed.sigma += Fr::ONE;
        assert_rejected("output public sigma", || {
            verify_batch_proof(&fixture.cfg, &original, &changed, &fixture.proof)
        });
        for component in 0..4 {
            let mut changed = output.clone();
            if let Form::Batch {
                point,
                alpha,
                lambda,
                indices,
                rows,
                ..
            } = &mut changed.form
            {
                match component {
                    0 => point[0] += Fr::ONE,
                    1 => *alpha += Fr::ONE,
                    2 => *lambda += Fr::ONE,
                    _ => indices[0] = (indices[0] + 1) % *rows,
                }
            } else {
                unreachable!();
            }
            assert_rejected(&format!("output public form component {component}"), || {
                verify_batch_proof(&fixture.cfg, &original, &changed, &fixture.proof)
            });
        }

        let other_rate = commitment_config(10, if rate == 1 { 2 } else { 1 }, 3, 136.0).unwrap();
        assert_rejected("RS-parameter replay", || {
            verify_batch_proof(&other_rate, &original, &output, &fixture.proof)
        });
    }
}

#[test]
fn batch_rejects_each_proof_field_and_each_source_opening() {
    for rate in [1, 2] {
        let fixture = BatchFixture::new(rate);
        let log_n = fixture.cfg.vector_size().ilog2() as usize;
        let before_root = (2 * log_n + fixture.sources.len()) * FIELD_BYTES;
        assert_eq!(
            fixture.proof.narg_string.len(),
            before_root + HASH_BYTES + FIELD_BYTES
        );
        for field in 0..2 * log_n {
            let mut changed = fixture.proof.clone();
            changed.narg_string[field * FIELD_BYTES] ^= 1;
            fixture.reject(&format!("sumcheck coefficient {field}"), &changed);
        }
        for source in 0..fixture.sources.len() {
            let mut changed = fixture.proof.clone();
            changed.narg_string[(2 * log_n + source) * FIELD_BYTES] ^= 1;
            fixture.reject(&format!("final evaluation {source}"), &changed);
        }
        let mut changed = fixture.proof.clone();
        changed.narg_string[before_root] ^= 1;
        fixture.reject("proof output root", &changed);
        let mut changed = fixture.proof.clone();
        changed.narg_string[before_root + HASH_BYTES] ^= 1;
        fixture.reject("proof OOD evaluation", &changed);

        for (source, (_, rows, authentication)) in fixture.hint_blocks().into_iter().enumerate() {
            let mut changed = fixture.proof.clone();
            changed.hints[rows] ^= 1;
            fixture.reject(&format!("source {source} opened row"), &changed);
            let mut changed = fixture.proof.clone();
            changed.hints[authentication] ^= 1;
            fixture.reject(&format!("source {source} authentication digest"), &changed);
        }
    }
}

#[test]
fn batch_rejects_malformed_truncated_and_appended_streams() {
    let fixture = BatchFixture::new(1);
    let log_n = fixture.cfg.vector_size().ilog2() as usize;
    let before_root = (2 * log_n + fixture.sources.len()) * FIELD_BYTES;
    for length in [
        0,
        1,
        FIELD_BYTES - 1,
        2 * log_n * FIELD_BYTES,
        before_root,
        before_root + HASH_BYTES - 1,
        fixture.proof.narg_string.len() - 1,
    ] {
        let mut changed = fixture.proof.clone();
        changed.narg_string.truncate(length);
        fixture.reject(&format!("truncated transcript at {length}"), &changed);
    }
    let mut changed = fixture.proof.clone();
    changed.narg_string.push(0);
    fixture.reject("appended transcript byte", &changed);
    for length in [0, 1, 7, 8, fixture.proof.hints.len() - 1] {
        let mut changed = fixture.proof.clone();
        changed.hints.truncate(length);
        fixture.reject(&format!("truncated hint stream at {length}"), &changed);
    }
    let mut changed = fixture.proof.clone();
    changed.hints.push(0);
    fixture.reject("appended hint byte", &changed);

    let row_fields = fixture.cfg.in_domain_samples() * fixture.cfg.interleaving_depth();
    for (source, (header, rows, authentication)) in fixture.hint_blocks().into_iter().enumerate() {
        for fields in [0, row_fields - 1, row_fields + 1] {
            let mut changed = fixture.proof.clone();
            changed.hints[header..header + 8].copy_from_slice(&(fields as u64).to_le_bytes());
            fixture.reject(
                &format!("source {source} malformed row-vector length {fields}"),
                &changed,
            );
        }
        for length in [
            rows + FIELD_BYTES - 1,
            authentication,
            authentication + HASH_BYTES - 1,
        ] {
            let mut changed = fixture.proof.clone();
            changed.hints.truncate(length);
            fixture.reject(
                &format!("source {source} truncated row/path at {length}"),
                &changed,
            );
        }
    }
}

#[test]
fn batch_prover_rejects_declared_root_and_witness_inconsistency() {
    let fixture = BatchFixture::new(1);
    assert_rejected("prover empty batch", || {
        run_batch(&fixture.cfg, &[], "empty-batch", 0.0, false).map(|_| 0)
    });
    for source in 0..fixture.sources.len() {
        let mut sources = fixture.sources.clone();
        sources[source].root.0[0] ^= 1;
        assert_rejected(
            &format!("prover source {source} declared root mismatch"),
            || run_batch(&fixture.cfg, &sources, "inconsistent-root", 0.0, false).map(|_| 0),
        );
    }
    let mut sources = fixture.sources.clone();
    let mut changed_message = sources[0].message.to_slice().to_vec();
    changed_message[0] += Fr::ONE;
    sources[0].message = Buffer::from(changed_message);
    sources[0].sigma = sources[0].message.dot(&sources[0].form.materialize());
    assert_rejected("prover message differs from committed witness", || {
        run_batch(&fixture.cfg, &sources, "inconsistent-witness", 0.0, false).map(|_| 0)
    });
}

#[test]
fn batch_zero_constant_and_repeated_sources_produce_fresh_valid_outputs() {
    for rate in [1, 2] {
        let cfg = commitment_config(10, rate, 3, 136.0).unwrap();
        for case in 0..3 {
            let base = match case {
                0 => Buffer::from(vec![Fr::ZERO; cfg.vector_size()]),
                1 => Buffer::from(vec![elem(91, 777, 0); cfg.vector_size()]),
                _ => message(10, 91, 888),
            };
            let form = Form::Mle(point(10, 91, 900));
            let identical = prepare(&cfg, base, form);
            let sources = vec![identical.clone(), identical.clone(), identical];
            assert!(sources.iter().all(|s| s.root == sources[0].root));
            let (measurement, output, proof) =
                run_batch(&cfg, &sources, "honest-edge-case", 0.0, false).unwrap();
            assert!(measurement.verified);
            let public = sources
                .iter()
                .map(BatchPublicInput::from)
                .collect::<Vec<_>>();
            verify_batch_proof(&cfg, &public, &BatchPublicOutput::from(&output), &proof).unwrap();
            assert_eq!(output.message.dot(&output.form.materialize()), output.sigma);
            assert!(encode(&cfg, &output.message) == output.witness.matrix);
            let freshly_committed = prepare(&cfg, output.message.clone(), output.form.clone());
            assert_eq!(freshly_committed.root, output.root);
            check_batch_output(&cfg, &output, true).unwrap();
        }
    }
}
