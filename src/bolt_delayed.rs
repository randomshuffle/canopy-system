//! Half-rate systematic Canopy followed by zero or more independent WHIR folds.
//! The parity side is frozen to the selected k/2 reference configuration.
use super::*;

#[derive(Serialize)]
struct Candidate {
    log_input: usize,
    input_oracles: usize,
    systematic_h: usize,
    systematic_rs_length: usize,
    systematic_message_log: usize,
    canopy_folds_before_join: Option<usize>,
    systematic_target_log: usize,
    systematic_join_log_inverse_rate: usize,
    total_bytes: f64,
    verifier_auth_hashes: f64,
    verifier_hash_to_field: f64,
    fs_finalizations: usize,
    verifier_hashes: f64,
    selected: bool,
}

#[derive(Serialize)]
struct MerkleBoundaryCheck {
    log_input: usize,
    input_oracles: usize,
    previous_split_bytes: f64,
    optimized_split_bytes: f64,
    systematic_target_log: usize,
    parity_target_log: usize,
}

/// Optimize height and join round at initial Canopy rate 1/2. The r=0 candidates
/// measure whether delaying the join itself helps, separately from changing h.
pub fn estimate(
    logs: &[usize],
    inputs: &[usize],
    samples: usize,
    reference: &Path,
    output: &Path,
) -> Result<()> {
    ensure!(samples > 0, "positive samples required");
    let prior = csv::Reader::from_path(reference.join("proof_sizes/proof_sizes.csv"))?
        .deserialize::<Row>()
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let sides =
        csv::Reader::from_path(reference.join("proof_sizes/independent_height_search.csv"))?
            .deserialize::<SideRow>()
            .collect::<std::result::Result<Vec<_>, _>>()?;
    std::fs::create_dir_all(output)?;
    let mut selected = Vec::new();
    let mut strictly_delayed = Vec::new();
    let mut grid = Vec::new();
    let mut merkle_boundary_checks = Vec::new();
    for &eta in inputs {
        ensure!([1, 2, 4, 8].contains(&eta), "unsupported input count");
        for &n in logs {
            eprintln!("Delayed systematic search n={n}, eta={eta}");
            let old = prior
                .iter()
                .find(|r| r.log_input == n && r.input_oracles == eta && r.systematic_h.is_some())
                .context("missing Canopy reference")?;
            ensure!(
                old.interleaving == 4 && old.samples == samples,
                "reference width/sample count must match"
            );
            let old_x = sides
                .iter()
                .find(|r| {
                    r.log_input == n
                        && r.input_oracles == eta
                        && r.canopy
                        && r.side == "systematic"
                        && r.selected
                })
                .context("missing systematic reference")?;
            let log_k = n - 2;
            let k = 1usize << log_k;
            ensure!(
                old.parity_rs_length == k / 2,
                "parity reference must use k/2"
            );
            let mut bottom = vec![[0.0; 4]; log_k + 1];
            for sample in 0..samples {
                let indices = sample_unique_indices(
                    k,
                    old.systematic_queries,
                    DEFAULT_SEED.wrapping_add(sample as u64),
                )?;
                for (h, work) in crate::merkle::authentication_work_by_height(&indices, k)?
                    .iter()
                    .enumerate()
                {
                    for (dest, value) in bottom[h].iter_mut().zip([
                        work.auth,
                        work.leaves,
                        work.forest_parents,
                        work.roots,
                    ]) {
                        *dest += value as f64 * eta as f64 / samples as f64;
                    }
                }
            }
            let first = grid.len();
            let mut candidates = Vec::new();
            for h in 0..=log_k {
                let message_log = log_k - h;
                let canopy = if message_log > 0 {
                    Some(whir_config(message_log, 1, true)?)
                } else {
                    None
                };
                let joins = if message_log > 0 {
                    (0..=(message_log - 1) / FOLD)
                        .map(|r| (message_log - FOLD * r, Some(r)))
                        .collect::<Vec<_>>()
                } else {
                    (1..=log_k).map(|target| (target, None)).collect()
                };
                for (target, folds) in joins {
                    let mut row = old.clone();
                    row.scheme = "bolt_canopy_delayed_systematic".into();
                    row.delayed_systematic = true;
                    row.systematic_h = Some(h);
                    row.systematic_target_log = target;
                    row.systematic_rs_length = 1usize << (message_log + 1);
                    let p = row.parameters();
                    let common = p.configs(k, eta)?;
                    let mut size = Size::default();
                    for sample in 0..samples {
                        size.add(side_size_with_canopy(
                            log_k,
                            &common[0],
                            canopy.as_ref(),
                            eta,
                            &mut ChaCha20Rng::seed_from_u64(
                                DEFAULT_SEED ^ (n as u64) << 32 ^ sample as u64,
                            ),
                        )?);
                    }
                    let b = bottom[h];
                    row.external_auth_bytes += b[0] * HASH_BYTES as f64 - old_x.external_auth_bytes;
                    let constant_bytes = if canopy.is_none() {
                        ((HASH_BYTES + FIELD_BYTES) * eta) as f64
                    } else {
                        0.0
                    };
                    row.whir_and_claim_bytes +=
                        size.bytes() / samples as f64 + constant_bytes - old_x.whir_and_claim_bytes;
                    row.total_bytes =
                        row.external_row_bytes + row.external_auth_bytes + row.whir_and_claim_bytes;
                    row.verifier_leaf_hashes += b[1] + size.verifier_leaf_hashes / samples as f64
                        - old_x.verifier_leaf_hashes;
                    row.verifier_parent_hashes += b[2]
                        + size.verifier_parent_hashes / samples as f64
                        - old_x.verifier_parent_hashes;
                    if canopy.is_none() {
                        row.verifier_leaf_hashes += eta as f64;
                        row.verifier_parent_hashes += eta as f64; // constant codeword length two
                    }
                    row.verifier_auth_hashes =
                        row.verifier_leaf_hashes + row.verifier_parent_hashes;
                    // Both old and new systematic roots use a direct field conversion.
                    row.verifier_hash_to_field = 0.0;
                    account(&mut row)?;
                    grid.push(Candidate {
                        log_input: n,
                        input_oracles: eta,
                        systematic_h: h,
                        systematic_rs_length: row.systematic_rs_length,
                        systematic_message_log: message_log,
                        canopy_folds_before_join: folds,
                        systematic_target_log: target,
                        systematic_join_log_inverse_rate: 1 + folds.unwrap_or(0) * (FOLD - 1),
                        total_bytes: row.total_bytes,
                        verifier_auth_hashes: row.verifier_auth_hashes,
                        verifier_hash_to_field: row.verifier_hash_to_field,
                        fs_finalizations: row.fs_finalizations,
                        verifier_hashes: row.verifier_hashes,
                        selected: false,
                    });
                    candidates.push(row);
                }
            }
            let best = (0..candidates.len())
                .min_by(|&a, &b| {
                    candidates[a]
                        .total_bytes
                        .total_cmp(&candidates[b].total_bytes)
                })
                .context("empty delayed search")?;
            let best_delayed = (0..candidates.len())
                .filter(|&i| {
                    grid[first + i]
                        .canopy_folds_before_join
                        .is_some_and(|r| r > 0)
                })
                .min_by(|&a, &b| {
                    candidates[a]
                        .total_bytes
                        .total_cmp(&candidates[b].total_bytes)
                })
                .context("empty strictly delayed search")?;
            grid[first + best].selected = true;
            eprintln!(
                "Selected h={}, independent folds={:?}, join log={}, proof={:.2} KiB",
                grid[first + best].systematic_h,
                grid[first + best].canopy_folds_before_join,
                grid[first + best].systematic_target_log,
                candidates[best].total_bytes / 1024.0
            );
            // Keep the split Merkle candidate so the existing timing runner can
            // select the better of this and the retained shared Merkle baseline.
            let mut merkle = prior
                .iter()
                .find(|r| r.log_input == n && r.input_oracles == eta && r.systematic_h.is_none())
                .context("missing Merkle reference")?
                .clone();
            // The retained split search required a prefix transition. Check
            // its omitted immediate-join boundary before reusing the baseline.
            let previous_split_bytes = merkle.total_bytes;
            for (side, source_log) in [("systematic", log_k), ("parity", log_k - 3)] {
                let old_side = sides
                    .iter()
                    .find(|r| {
                        r.log_input == n
                            && r.input_oracles == eta
                            && !r.canopy
                            && r.side == side
                            && r.selected
                    })
                    .context("missing Merkle side reference")?;
                let common = whir_config(source_log, 1, true)?;
                let mut immediate = Size::default();
                for sample in 0..samples {
                    immediate.add(side_size(
                        source_log,
                        &common,
                        false,
                        eta,
                        &mut ChaCha20Rng::seed_from_u64(
                            DEFAULT_SEED ^ (n as u64) << 32 ^ sample as u64,
                        ),
                    )?);
                }
                let difference = immediate.bytes() / samples as f64 - old_side.whir_and_claim_bytes;
                if difference < 0.0 {
                    let prior_common = whir_config(old_side.target_log, 1, true)?;
                    let mut prior_size = Size::default();
                    for sample in 0..samples {
                        prior_size.add(side_size(
                            source_log,
                            &prior_common,
                            false,
                            eta,
                            &mut ChaCha20Rng::seed_from_u64(
                                DEFAULT_SEED ^ (n as u64) << 32 ^ sample as u64,
                            ),
                        )?);
                    }
                    merkle.whir_and_claim_bytes += difference;
                    merkle.total_bytes += difference;
                    merkle.verifier_leaf_hashes += (immediate.verifier_leaf_hashes
                        - prior_size.verifier_leaf_hashes)
                        / samples as f64;
                    merkle.verifier_parent_hashes += (immediate.verifier_parent_hashes
                        - prior_size.verifier_parent_hashes)
                        / samples as f64;
                    if side == "systematic" {
                        merkle.systematic_target_log = source_log;
                    } else {
                        merkle.parity_target_log = source_log;
                    }
                }
            }
            merkle.verifier_auth_hashes =
                merkle.verifier_leaf_hashes + merkle.verifier_parent_hashes;
            account(&mut merkle)?;
            merkle_boundary_checks.push(MerkleBoundaryCheck {
                log_input: n,
                input_oracles: eta,
                previous_split_bytes,
                optimized_split_bytes: merkle.total_bytes,
                systematic_target_log: merkle.systematic_target_log,
                parity_target_log: merkle.parity_target_log,
            });
            strictly_delayed.push(merkle.clone());
            strictly_delayed.push(candidates[best_delayed].clone());
            selected.push(merkle);
            selected.push(candidates.swap_remove(best));
            write_csv(output.join("proof_sizes.csv"), &selected)?;
            let strict_output = output.join("strictly_delayed");
            std::fs::create_dir_all(&strict_output)?;
            write_csv(strict_output.join("proof_sizes.csv"), &strictly_delayed)?;
            write_csv(output.join("systematic_join_search.csv"), &grid)?;
            write_csv(
                output.join("merkle_immediate_join_check.csv"),
                &merkle_boundary_checks,
            )?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn merkle_boundary_without_prefix_verifies_and_binds_claims() {
        let encoder = BoltMaxEncoder::new(512, 4, 19).unwrap();
        let p = Parameters {
            canopy_log_divisor: 0,
            delayed_systematic: false,
            systematic_h: None,
            parity_h: None,
            systematic_target_log: 9,
            parity_target_log: 6,
        };
        let run = prove(&encoder, &input_data(11, 1, 19), p, 19).unwrap();
        verify(&encoder, 11, p, 19, &run.evaluations, &run.proof).unwrap();
        let mut bad = run.evaluations.clone();
        bad[0] += Fr::ONE;
        assert!(rejected(|| verify(&encoder, 11, p, 19, &bad, &run.proof)));
    }
    #[test]
    fn delayed_systematic_proof_binds_both_layers_and_join() {
        let seed = 51;
        let k = 512;
        let n = 11;
        let encoder = BoltMaxEncoder::new(k, 4, seed).unwrap();
        let inputs = input_data(n, 1, seed);
        for (height, target) in [(0, 9), (2, 7), (2, 4), (1, 2), (9, 3), (9, 9)] {
            let p = Parameters {
                canopy_log_divisor: 1,
                delayed_systematic: true,
                systematic_h: Some(height),
                parity_h: Some(3),
                systematic_target_log: target,
                parity_target_log: 4,
            };
            let run = prove(&encoder, &inputs, p, seed).unwrap();
            verify(&encoder, n, p, seed, &run.evaluations, &run.proof).unwrap();
            let changed = Parameters {
                systematic_target_log: if target > 3 { target - 3 } else { target + 3 },
                ..p
            };
            assert!(rejected(|| verify(
                &encoder,
                n,
                changed,
                seed,
                &run.evaluations,
                &run.proof
            )));
            let mut values = run.evaluations.clone();
            values[0] += Fr::ONE;
            assert!(rejected(|| verify(
                &encoder, n, p, seed, &values, &run.proof
            )));
            for at in [0, 32, run.proof.narg_string.len() - 1] {
                let mut bad = run.proof.clone();
                bad.narg_string[at] ^= 1;
                assert!(rejected(|| verify(
                    &encoder,
                    n,
                    p,
                    seed,
                    &run.evaluations,
                    &bad
                )));
            }
        }
    }
    fn rejected(f: impl FnOnce() -> Result<()>) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
            Ok(r) => r.is_err(),
            Err(_) => true,
        }
    }
    #[test]
    fn delayed_canopy_prefix_size_matches_serialized_proof() {
        // Every query is a full read at these sizes, so sampled and actual
        // authentication sizes agree exactly, including the separate Canopy fold.
        let source_log = 6;
        let canopy = whir_config(4, 1, true).unwrap();
        let common = whir_config(1, 3, true).unwrap();
        let source = batching::initial_config(source_log, &common).unwrap();
        let configs = [source, canopy.clone()];
        let domain = DomainSeparator::protocol(&"delayed-canopy-size-test").instance(&Empty);
        let mut state = ProverState::new(&domain, Blake3Transcript::default());
        let mut claims = Vec::new();
        for (i, cfg) in configs.iter().enumerate() {
            let message = Buffer::from(
                (0..cfg.initial_size())
                    .map(|j| crate::benchmark_field_element(71, i as u64, j as u64))
                    .collect::<Vec<_>>(),
            );
            let witness = cfg.commit(&mut state, &[&message]);
            let form = vec![Fr::ONE; message.len()];
            let value = dot(&form, message.to_slice());
            claims.push(batching::ProverClaim {
                config: cfg.clone(),
                message,
                witness,
                forms: vec![Box::new(Covector::new(form))],
                values: vec![value],
            });
        }
        batching::prove_batch(claims, &common, &mut state, &mut Vec::new()).unwrap();
        let proof = state.proof();
        let modeled = side_size_with_canopy(
            source_log,
            &common,
            Some(&canopy),
            1,
            &mut ChaCha20Rng::seed_from_u64(91),
        )
        .unwrap()
        .bytes() as usize;
        // One u64 framing per OOD vector: source 3 roots, Canopy 2 roots,
        // The fixed-size native terminal message has no length prefix.
        let prefix_commits = 3 + 2;
        let framing = 8 * (prefix_commits + common.round_configs.len());
        assert_eq!(
            proof.narg_string.len() + proof.hints.len(),
            modeled + framing
        );
    }
}
