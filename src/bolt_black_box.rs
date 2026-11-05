//! Bolt's unchanged shared Merkle IOPP, with separate Canopy SEP proofs.
//! Only the input X/Y commitments use Canopy. Canopy claims may batch across
//! inputs on the same side, but never with any claim from the Bolt IOPP.
use super::*;

pub(super) fn partition_front<T>(
    front: Vec<T>,
    heights: (usize, usize),
    k: usize,
) -> Result<[Vec<T>; 3]> {
    let mut iter = front.into_iter();
    let mut groups = [Vec::new(), Vec::new(), Vec::new()];
    for _ in 0..2 {
        groups[0].push(iter.next().context("missing Bolt claim")?);
    }
    for (side, rows, height) in [(1, k, heights.0), (2, k / 4, heights.1)] {
        if height < rows.ilog2() as usize {
            groups[side].push(iter.next().context("missing Canopy claim")?);
        }
    }
    ensure!(iter.next().is_none(), "extra front claim");
    Ok(groups)
}

/// Includes the already-transmitted Canopy roots and OOD values, and one
/// shared native WHIR proof for this family of eta separately committed inputs.
fn canopy_batch_size(cfg: &WhirConfig, eta: usize, rng: &mut ChaCha20Rng) -> Result<Size> {
    ensure!(
        cfg.security_level(eta, eta) + 1e-9 >= WHIR_SECURITY as f64,
        "insufficient standalone Canopy batch security"
    );
    let mut first_queries = rng.clone();
    let mut size = whir_size(cfg, rng)?;
    let opening = oracle_size(&cfg.initial_committer, &mut first_queries)?;
    for _ in 1..eta {
        size.add(opening);
        size.hashes += 1.;
        size.fields += cfg.initial_out_domain_samples as f64;
    }
    size.fields += (eta * (eta - 1) * (1 + cfg.initial_out_domain_samples)) as f64;
    Ok(size)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EstimateRow {
    pub log_input: usize,
    pub input_oracles: usize,
    pub scheme: String,
    pub systematic_h: Option<usize>,
    pub parity_h: Option<usize>,
    pub shared_bolt_target_log: usize,
    pub systematic_queries: usize,
    pub parity_queries: usize,
    pub external_row_bytes: f64,
    pub external_auth_bytes: f64,
    pub common_bolt_iopp_bytes: f64,
    pub canopy_sep_bytes: f64,
    pub total_bytes: f64,
    pub samples: usize,
    #[serde(default)]
    pub verifier_auth_hashes: f64,
    #[serde(default)]
    pub fs_finalizations: usize,
    #[serde(default)]
    pub verifier_hashes: f64,
}

#[derive(Clone, Debug, Serialize)]
struct HeightCandidate {
    log_input: usize,
    input_oracles: usize,
    side: &'static str,
    height: usize,
    canopy_message_length: usize,
    canopy_rs_length: usize,
    bottom_auth_bytes: f64,
    canopy_sep_bytes: f64,
    total_authentication_bytes: f64,
    verifier_auth_hashes: f64,
    selected: bool,
}

pub fn estimate(logs: &[usize], inputs: &[usize], samples: usize, output: &Path) -> Result<()> {
    ensure!(
        samples > 0 && !logs.is_empty() && !inputs.is_empty(),
        "empty estimate grid"
    );
    ensure!(
        !output.join("proof_sizes.csv").exists(),
        "estimate output already exists"
    );
    std::fs::create_dir_all(output)?;
    let mut rows = Vec::new();
    let mut search = Vec::new();
    for &n in logs {
        ensure!((10..=30).contains(&n), "supported log sizes are 10..=30");
        let log_k = n - 2;
        let k = 1 << log_k;
        let target = log_k - 3; // Unchanged paper Merkle baseline: join at k/8.
        let common = batching::common_config(k, None, CanopyLayout::EqualBlocks, target)?;
        for &eta in inputs {
            ensure!([1, 2, 4, 8].contains(&eta), "unsupported input count");
            eprintln!("Black-box Bolt proof search n={n} eta={eta}");
            let (qx, qy) = multi::query_counts_for_inputs(eta);
            let mut common_size = Size::default();
            for sample in 0..samples {
                common_size.add(batching::batch_size_inputs(
                    log_k,
                    &common,
                    false,
                    eta,
                    &mut ChaCha20Rng::seed_from_u64(
                        DEFAULT_SEED ^ (n as u64) << 32 ^ sample as u64,
                    ),
                )?);
            }
            let common_bytes =
                common_size.bytes() / samples as f64 + ((4 + 1) * FIELD_BYTES * eta) as f64;
            let common_hashes = (common_size.verifier_leaf_hashes
                + common_size.verifier_parent_hashes)
                / samples as f64;
            let external_rows = ((qx.min(k) + qy.min(k / 4)) * 4 * FIELD_BYTES * eta) as f64;
            let mut baseline_auth = (2 * HASH_BYTES * eta) as f64;
            let mut baseline_hashes = common_hashes;
            let mut best = Vec::new();
            for (side, count, length) in [("X", qx, k), ("Y", qy, k / 4)] {
                let mut bottom = vec![0.; length.ilog2() as usize + 1];
                let mut bottom_hashes = bottom.clone();
                for sample in 0..samples {
                    let indices = sample_unique_indices(
                        length,
                        count.min(length),
                        DEFAULT_SEED.wrapping_add(sample as u64),
                    )?;
                    for (h, work) in crate::merkle::authentication_work_by_height(&indices, length)?
                        .iter()
                        .enumerate()
                    {
                        bottom[h] += (work.auth * HASH_BYTES * eta) as f64 / samples as f64;
                        bottom_hashes[h] +=
                            ((work.leaves + work.forest_parents) * eta) as f64 / samples as f64;
                    }
                }
                baseline_auth += bottom[length.ilog2() as usize];
                baseline_hashes += bottom_hashes[length.ilog2() as usize];
                let mut candidates = Vec::new();
                for h in minimum_canopy_height(length, k / 4)..=length.ilog2() as usize {
                    let dimension = length >> h;
                    let mut sep = 0.;
                    let mut sep_hashes = 0.;
                    if dimension == 1 {
                        sep = ((HASH_BYTES + FIELD_BYTES) * eta) as f64;
                        sep_hashes = ((1 + (k / 4).ilog2() as usize) * eta) as f64;
                    } else {
                        let cfg = whir_config(
                            dimension.ilog2() as usize,
                            canopy_rate_log(length, k / 4, h),
                            true,
                        )?;
                        for sample in 0..samples {
                            let size = canopy_batch_size(
                                &cfg,
                                eta,
                                &mut ChaCha20Rng::seed_from_u64(
                                    DEFAULT_SEED
                                        ^ (n as u64) << 32
                                        ^ (h as u64) << 16
                                        ^ sample as u64,
                                ),
                            )?;
                            sep += size.bytes();
                            sep_hashes += size.verifier_leaf_hashes + size.verifier_parent_hashes;
                        }
                        sep /= samples as f64;
                        sep_hashes /= samples as f64;
                    }
                    candidates.push(HeightCandidate {
                        log_input: n,
                        input_oracles: eta,
                        side,
                        height: h,
                        canopy_message_length: dimension,
                        canopy_rs_length: k / 4,
                        bottom_auth_bytes: bottom[h],
                        canopy_sep_bytes: sep,
                        total_authentication_bytes: bottom[h] + sep,
                        verifier_auth_hashes: bottom_hashes[h] + sep_hashes,
                        selected: false,
                    });
                }
                let index = (0..candidates.len())
                    .min_by(|&a, &b| {
                        candidates[a]
                            .total_authentication_bytes
                            .total_cmp(&candidates[b].total_authentication_bytes)
                    })
                    .unwrap();
                candidates[index].selected = true;
                best.push(candidates[index].clone());
                search.extend(candidates);
            }
            let mut baseline = EstimateRow {
                log_input: n,
                input_oracles: eta,
                scheme: "bolt_merkle_shared".into(),
                systematic_h: None,
                parity_h: None,
                shared_bolt_target_log: target,
                systematic_queries: qx.min(k),
                parity_queries: qy.min(k / 4),
                external_row_bytes: external_rows,
                external_auth_bytes: baseline_auth,
                common_bolt_iopp_bytes: common_bytes,
                canopy_sep_bytes: 0.,
                total_bytes: external_rows + baseline_auth + common_bytes,
                samples,
                verifier_auth_hashes: baseline_hashes,
                fs_finalizations: 0,
                verifier_hashes: 0.,
            };
            let mut separate = baseline.clone();
            separate.scheme = "bolt_canopy_black_box".into();
            separate.systematic_h = Some(best[0].height);
            separate.parity_h = Some(best[1].height);
            separate.external_auth_bytes = best.iter().map(|r| r.bottom_auth_bytes).sum();
            separate.canopy_sep_bytes = best.iter().map(|r| r.canopy_sep_bytes).sum();
            separate.total_bytes = external_rows
                + separate.external_auth_bytes
                + common_bytes
                + separate.canopy_sep_bytes;
            separate.verifier_auth_hashes =
                common_hashes + best.iter().map(|r| r.verifier_auth_hashes).sum::<f64>();
            for row in [&mut baseline, &mut separate] {
                // Finalization count depends on message/challenge transitions,
                // not the number of words in a nonempty opening segment.
                let query_segment = usize::from(qx < k || qy < k / 4);
                row.fs_finalizations = fs_model(row, &vec![1; eta], &vec![query_segment; eta])?
                    .counts()
                    .finalizations;
                row.verifier_hashes = row.verifier_auth_hashes + row.fs_finalizations as f64;
            }
            rows.extend([baseline, separate]);
        }
    }
    write_csv(output.join("proof_sizes.csv"), &rows)?;
    write_csv(output.join("height_search.csv"), &search)?;
    Ok(())
}

fn fs_model(
    row: &EstimateRow,
    outer_words: &[usize],
    query_bytes: &[usize],
) -> Result<crate::fiat_shamir::Ledger> {
    use crate::fiat_shamir::{native_commit, native_verify, Ledger};
    let eta = row.input_oracles;
    ensure!(
        outer_words.len() == eta && query_bytes.len() == eta,
        "wrong transcript shape"
    );
    let log_k = row.log_input - 2;
    let k = 1 << log_k;
    let common = batching::common_config(
        k,
        None,
        CanopyLayout::EqualBlocks,
        row.shared_bolt_target_log,
    )?;
    let x = batching::initial_config(log_k, &common)?;
    let u = batching::initial_config(log_k - 3, &common)?;
    let mut canopy = Vec::new();
    for (length, height) in [(k, row.systematic_h), (k / 4, row.parity_h)] {
        canopy.push(match height {
            Some(h) if h < length.ilog2() as usize => Some(whir_config(
                length.ilog2() as usize - h,
                canopy_rate_log(length, k / 4, h),
                true,
            )?),
            _ => None,
        });
    }
    let mut l = Ledger::domain();
    l.fields(2 * eta);
    for i in 0..eta {
        l.fields(4);
        l.challenges(4);
        native_commit(&mut l, &x);
        native_commit(&mut l, &u);
        l.challenges(log_k - 3);
        l.fields(1);
        l.push('s', 1, query_bytes[i]);
        l.fields(outer_words[i]);
        l.challenges(2);
        for (cfg, height) in canopy.iter().zip([row.systematic_h, row.parity_h]) {
            if let Some(cfg) = cfg {
                l.challenges(1);
                l.challenges(cfg.initial_out_domain_samples);
                l.fields(cfg.initial_out_domain_samples);
            } else if height.is_some() {
                l.fields(1);
            }
        }
    }
    for _ in 0..eta {
        fiat_shamir::reduce(&mut l, x.clone(), &common, 3)?;
        fiat_shamir::reduce(&mut l, u.clone(), &common, 2)?;
    }
    let count = 2 * eta;
    l.fields(count * (count - 1));
    native_verify(&mut l, &common, count, count);
    for cfg in canopy.into_iter().flatten() {
        for _ in 0..eta {
            fiat_shamir::reduce(&mut l, cfg.clone(), &cfg, 1)?;
        }
        l.fields(eta * (eta - 1));
        native_verify(&mut l, &cfg, eta, eta);
    }
    Ok(l)
}

/// Matched complete serialized proofs; the methods rotate order each
/// repetition. Setup and input generation are outside all prover timers.
pub fn time(
    n: usize,
    eta: usize,
    warmups: usize,
    repetitions: usize,
    estimates: &Path,
    output: &Path,
) -> Result<()> {
    ensure!(repetitions > 0, "positive repetitions required");
    ensure!(
        !output.join("prover_timings.csv").exists(),
        "timing output already exists"
    );
    let black = csv::Reader::from_path(estimates.join("proof_sizes.csv"))?
        .deserialize::<EstimateRow>()
        .collect::<std::result::Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|r| {
            r.log_input == n && r.input_oracles == eta && r.scheme == "bolt_canopy_black_box"
        })
        .collect::<Vec<_>>();
    ensure!(
        black.len() == 1,
        "expected one selected black-box configuration"
    );
    let black = &black[0];
    let heights = black.systematic_h.zip(black.parity_h);
    let seed = DEFAULT_SEED;
    let encoder = BoltMaxEncoder::new(1 << (n - 2), 4, seed)?;
    let inputs = (0..eta)
        .map(|j| {
            (0..1usize << n)
                .map(|i| crate::benchmark_field_element(seed, 0x424f4c54 + j as u64, i as u64))
                .collect()
        })
        .collect::<Vec<Vec<Fr>>>();
    std::fs::create_dir_all(output)?;
    let mut rows = Vec::new();
    let mut stages = Vec::new();
    for iteration in 0..warmups + repetitions {
        let methods: &[usize] = &[0, 1];
        for offset in 0..methods.len() {
            let method = methods[(iteration + offset) % methods.len()];
            eprintln!("Black-box comparison n={n} eta={eta} iteration={iteration} method={method}");
            let mut run = match method {
                0 => multi::prove(
                    &encoder,
                    &inputs,
                    None,
                    seed,
                    CanopyLayout::EqualBlocks,
                    black.shared_bolt_target_log,
                )?,
                _ => multi::prove_mode(
                    &encoder,
                    &inputs,
                    heights,
                    seed,
                    CanopyLayout::EqualBlocks,
                    black.shared_bolt_target_log,
                    true,
                )?,
            };
            let start = Instant::now();
            match method {
                0 => multi::verify(
                    &encoder,
                    n,
                    None,
                    seed,
                    &run.evaluations,
                    &run.proof,
                    CanopyLayout::EqualBlocks,
                    black.shared_bolt_target_log,
                )?,
                _ => multi::verify_mode(
                    &encoder,
                    n,
                    heights,
                    seed,
                    &run.evaluations,
                    &run.proof,
                    CanopyLayout::EqualBlocks,
                    black.shared_bolt_target_log,
                    true,
                )?,
            }
            run.row.verification_seconds = start.elapsed().as_secs_f64();
            run.row.proof_verified = true;
            run.row.scheme = ["bolt_merkle_shared", "bolt_canopy_black_box"][method].into();
            if iteration < warmups {
                continue;
            }
            run.row.iteration = iteration - warmups;
            for event in run.profile {
                stages.push(StageRow {
                    input_oracles: eta,
                    canopy_layout: run.row.canopy_layout,
                    log_input: n,
                    interleaving: 4,
                    scheme: run.row.scheme.clone(),
                    systematic_h: run.row.systematic_h,
                    parity_h: run.row.parity_h,
                    setup_seed: seed,
                    rayon_threads: rayon::current_num_threads(),
                    iteration: run.row.iteration,
                    phase: event.phase,
                    part: event.part,
                    stage: event.stage,
                    seconds: event.seconds,
                });
            }
            rows.push(run.row);
            write_csv(output.join("prover_timings.csv"), &rows)?;
            write_csv(output.join("prover_timing_stages.csv"), &stages)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rejected(action: impl FnOnce() -> Result<()>) -> bool {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(action)).map_or(true, |r| r.is_err())
    }

    #[test]
    #[cfg(feature = "fs-accounting")]
    fn black_box_fs_model_matches_verifier() {
        use crate::fiat_shamir::capture;
        let dir = tempfile::tempdir().unwrap();
        let n = 18;
        estimate(&[n], &[1, 2, 4, 8], 1, dir.path()).unwrap();
        let rows = csv::Reader::from_path(dir.path().join("proof_sizes.csv"))
            .unwrap()
            .deserialize::<EstimateRow>()
            .collect::<std::result::Result<Vec<_>, _>>()
            .unwrap();
        for row in rows {
            let eta = row.input_oracles;
            let encoder = BoltMaxEncoder::new(1 << (n - 2), 4, DEFAULT_SEED).unwrap();
            let inputs = (0..eta)
                .map(|j| {
                    (0..1usize << n)
                        .map(|i| crate::benchmark_field_element(DEFAULT_SEED, j as u64, i as u64))
                        .collect()
                })
                .collect::<Vec<Vec<Fr>>>();
            let heights = row.systematic_h.zip(row.parity_h);
            let run = multi::prove_mode(
                &encoder,
                &inputs,
                heights,
                DEFAULT_SEED,
                CanopyLayout::EqualBlocks,
                row.shared_bolt_target_log,
                heights.is_some(),
            )
            .unwrap();
            let (verified, traces) = capture(|| {
                multi::verify_mode(
                    &encoder,
                    n,
                    heights,
                    DEFAULT_SEED,
                    &run.evaluations,
                    &run.proof,
                    CanopyLayout::EqualBlocks,
                    row.shared_bolt_target_log,
                    heights.is_some(),
                )
            });
            verified.unwrap();
            assert_eq!(traces.len(), 1);
            let trace = &traces[0];
            let mut openings = trace
                .events
                .iter()
                .enumerate()
                .filter(|(_, e)| e.operation == 'a' && e.bytes == 32)
                .collect::<Vec<_>>();
            openings.sort_by_key(|(_, e)| std::cmp::Reverse(e.count));
            openings.truncate(eta);
            openings.sort_by_key(|(i, _)| *i);
            let mut words = Vec::new();
            let mut queries = Vec::new();
            for (i, e) in openings {
                assert_eq!(
                    (trace.events[i - 1].operation, trace.events[i - 1].bytes),
                    ('s', 1)
                );
                words.push(e.count);
                queries.push(trace.events[i - 1].count);
            }
            let modeled = fs_model(&row, &words, &queries).unwrap();
            assert_eq!(
                modeled.normalized(),
                trace.normalized(),
                "{} eta={eta}",
                row.scheme
            );
            assert_eq!(row.fs_finalizations, trace.counts().finalizations);
            assert_eq!(
                trace.counts().absorb_bytes,
                96 + run.proof.narg_string.len()
            );
        }
    }

    #[test]
    fn separate_canopy_round_trip_and_tamper() {
        let n = 10;
        let k = 1 << (n - 2);
        let seed = 73;
        let encoder = BoltMaxEncoder::new(k, 4, seed).unwrap();
        for eta in [1, 2, 8] {
            let inputs = (0..eta)
                .map(|j| {
                    (0..1usize << n)
                        .map(|i| crate::benchmark_field_element(seed, j as u64, i as u64))
                        .collect()
                })
                .collect::<Vec<Vec<Fr>>>();
            // Nonconstant, mixed, and constant layers; independent heights do
            // not have to line up with either the Bolt tail or one another.
            for heights in [(3, 3), (4, 2), (8, 2), (3, 6), (8, 6)] {
                let run = multi::prove_mode(
                    &encoder,
                    &inputs,
                    Some(heights),
                    seed,
                    CanopyLayout::EqualBlocks,
                    n - 5,
                    true,
                )
                .unwrap();
                let verify = |evaluations: &[Fr], proof: &Proof| {
                    multi::verify_mode(
                        &encoder,
                        n,
                        Some(heights),
                        seed,
                        evaluations,
                        proof,
                        CanopyLayout::EqualBlocks,
                        n - 5,
                        true,
                    )
                };
                verify(&run.evaluations, &run.proof).unwrap();
                for j in 0..eta {
                    let mut wrong = run.evaluations.clone();
                    wrong[j] += Fr::ONE;
                    assert!(rejected(|| verify(&wrong, &run.proof)));
                }
                for index in [
                    0,
                    32,
                    run.proof.narg_string.len() / 2,
                    run.proof.narg_string.len() - 1,
                ] {
                    let mut bad = run.proof.clone();
                    bad.narg_string[index] ^= 1;
                    assert!(rejected(|| verify(&run.evaluations, &bad)));
                }
                if !run.proof.hints.is_empty() {
                    let mut bad = run.proof.clone();
                    bad.hints[0] ^= 1;
                    assert!(rejected(|| verify(&run.evaluations, &bad)));
                }
                let mut bad = run.proof.clone();
                bad.narg_string.pop();
                assert!(rejected(|| verify(&run.evaluations, &bad)));
                let mut bad = run.proof.clone();
                bad.hints.push(0);
                assert!(rejected(|| verify(&run.evaluations, &bad)));
                assert!(rejected(|| multi::verify(
                    &encoder,
                    n,
                    Some((3, 1)),
                    seed,
                    &run.evaluations,
                    &run.proof,
                    CanopyLayout::EqualBlocks,
                    n - 5
                )));
            }
        }
    }

    #[test]
    fn standalone_batch_size_matches_serialized_proof() {
        // Two rows, many queries: every sampled row set is saturated. This
        // gives an exact byte check, including the native vector framing.
        let cfg = whir_config(1, 1, true).unwrap();
        for eta in [1, 2, 4, 8] {
            let domain = DomainSeparator::protocol(&"black-box-size-test").instance(&Empty);
            let mut state = ProverState::new(&domain, Blake3Transcript::default());
            let mut claims = Vec::new();
            for j in 0..eta {
                let message = Buffer::from(vec![Fr::from(j as u64), Fr::ONE]);
                let form = vec![Fr::ONE; 2];
                let value = dot(&form, message.to_slice());
                let witness = cfg.commit(&mut state, &[&message]);
                claims.push(batching::ProverClaim {
                    config: cfg.clone(),
                    message,
                    witness,
                    forms: vec![Box::new(Covector::new(form))],
                    values: vec![value],
                });
            }
            batching::prove_batch(claims, &cfg, &mut state, &mut Vec::new()).unwrap();
            let proof = state.proof();
            let model = canopy_batch_size(&cfg, eta, &mut ChaCha20Rng::seed_from_u64(42))
                .unwrap()
                .bytes() as usize;
            let framing = 8 * (eta + cfg.round_configs.len());
            assert_eq!(proof.narg_string.len() + proof.hints.len(), model + framing);
        }
    }
}
