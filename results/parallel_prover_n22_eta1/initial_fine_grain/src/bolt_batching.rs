//! Aligned WHIR batching for Bolt. Each long code-switch message is reduced
//! by authenticated WHIR transitions to the Canopy message length. The source
//! commitments stay separate; one native matrix-batched WHIR proves the tail.
use super::*;
use rayon::prelude::*;
use whir_ref::{buffer::BufferMath, protocols::geometric_challenge::geometric_challenge};

pub(super) struct ProverClaim {
    pub config: WhirConfig,
    pub message: Buffer<Fr>,
    pub witness: WhirWitness,
    pub forms: Vec<Box<dyn LinearForm<Fr>>>,
    pub values: Vec<Fr>,
}
pub(super) struct VerifierClaim {
    pub config: WhirConfig,
    pub commitment: Commitment<Fr>,
    pub forms: Vec<Box<dyn LinearForm<Fr>>>,
    pub values: Vec<Fr>,
}

pub(super) fn common_config(
    k: usize,
    heights: Option<(usize, usize)>,
    layout: CanopyLayout,
    target: usize,
) -> Result<WhirConfig> {
    ensure!(
        target > 0 && target <= k.ilog2() as usize - 3,
        "invalid aligned message length"
    );
    let rate = if let Some((hx, hy)) = heights.filter(|h| h.0 < k.ilog2() as usize) {
        ensure!(
            k >> hx == 1 << target && (k / 4) >> hy == 1 << target,
            "both Canopy messages must align with the shared tail"
        );
        ensure!(
            layout.systematic_rs_length(k) == layout.parity_rs_length(k),
            "batched Canopy requires equal RS lengths"
        );
        layout.systematic_rs_length(k).ilog2() as usize - target
    } else {
        1
    };
    whir_config(target, rate, true)
}

pub(super) fn initial_config(log: usize, common: &WhirConfig) -> Result<WhirConfig> {
    let target = common.initial_num_variables();
    if log == target {
        return Ok(common.clone());
    }
    ensure!(log > target, "cannot unfold a shorter message");
    whir_config_fold(log, 1, false, FOLD.min(log - target))
}

pub(super) fn next_config(current: &WhirConfig, common: &WhirConfig) -> Result<WhirConfig> {
    let next = current.initial_sumcheck.final_size().ilog2() as usize;
    if next == common.initial_num_variables() {
        return Ok(common.clone());
    }
    let rate = current.initial_committer.size().ilog2() as usize - current.initial_num_variables();
    let rate = rate + current.initial_sumcheck.num_rounds() - 1;
    whir_config_fold(
        next,
        rate,
        !current.initial_committer.unique_decoding(),
        FOLD.min(next - common.initial_num_variables()),
    )
}

fn combine(
    forms: &[Box<dyn LinearForm<Fr>>],
    values: &[Fr],
    ood: &irs_commit::Evaluations<Fr>,
    coefficients: &[Fr],
    size: usize,
) -> (Buffer<Fr>, Fr) {
    let mut covector = vec![Fr::ZERO; size];
    for (form, &c) in forms.iter().zip(coefficients) {
        form.accumulate(&mut covector, c);
    }
    let mut value = dot(values, &coefficients[..values.len()]);
    let mut covector = Buffer::from(covector);
    covector.accumulate_geometric(
        &ood.points,
        &Buffer::from(&coefficients[values.len()..]),
        size,
    );
    value += dot(ood.matrix.to_slice(), &coefficients[values.len()..]);
    (covector, value)
}

fn add_queries(
    covector: &mut Buffer<Fr>,
    sum: &mut Fr,
    in_domain: &irs_commit::Evaluations<Fr>,
    fold: &[Fr],
    coefficients: &[Fr],
) {
    let values = in_domain.values(&eq_weights(fold)).collect::<Vec<_>>();
    *sum += dot(coefficients, &values);
    if let Some(domain) = &in_domain.domain {
        let extra = crate::additive_rs::power_sums(
            &domain.exponents,
            coefficients,
            domain.length,
            covector.len(),
        );
        let combined = covector
            .to_slice()
            .iter()
            .zip(extra.to_slice())
            .map(|(&a, &b)| a + b)
            .collect::<Vec<_>>();
        *covector = Buffer::from(combined);
    } else {
        covector.accumulate_geometric(
            &in_domain.points,
            &Buffer::from(coefficients),
            covector.len(),
        );
    }
}

pub(super) fn reduce_prover(
    mut claim: ProverClaim,
    common: &WhirConfig,
    state: &mut ProverState,
) -> Result<ProverClaim> {
    let coefficients: Vec<Fr> = geometric_challenge(
        state,
        claim.forms.len() + claim.witness.out_of_domain.points.len(),
    );
    let (mut covector, mut sum) = combine(
        &claim.forms,
        &claim.values,
        &claim.witness.out_of_domain,
        &coefficients,
        claim.message.len(),
    );
    while claim.message.len() > common.initial_size() {
        let next = next_config(&claim.config, common)?;
        let (message, opening) = claim.config.initial_sumcheck.prove(
            state,
            &Identity::new(),
            claim.message,
            &mut covector,
            &mut sum,
            &[],
        );
        // Bind the new oracle before selecting queries into the old oracle.
        let witness = next.commit(state, &[&message]);
        let in_domain = claim
            .config
            .initial_committer
            .open(state, &[&claim.witness.irs]);
        let coefficients: Vec<Fr> = geometric_challenge(state, in_domain.points.len());
        add_queries(
            &mut covector,
            &mut sum,
            &in_domain,
            &opening.round_challenges,
            &coefficients,
        );
        // Intermediate OOD constraints enter the next sumcheck. At the join,
        // the native shared proof adds these OOD constraints itself.
        if message.len() > common.initial_size() {
            let coefficients: Vec<Fr> =
                geometric_challenge(state, witness.out_of_domain.points.len());
            covector.accumulate_geometric(
                &witness.out_of_domain.points,
                &Buffer::from(coefficients.as_slice()),
                message.len(),
            );
            sum += dot(&coefficients, witness.out_of_domain.matrix.to_slice());
        }
        claim.config = next;
        claim.message = message;
        claim.witness = witness;
    }
    claim.forms = vec![Box::new(Covector::new(covector.to_slice().to_vec()))];
    claim.values = vec![sum];
    Ok(claim)
}

pub(super) fn reduce_verifier(
    mut claim: VerifierClaim,
    common: &WhirConfig,
    state: &mut VerifierState,
) -> Result<VerifierClaim> {
    let coefficients: Vec<Fr> = geometric_challenge(
        state,
        claim.forms.len() + claim.commitment.out_of_domain.points.len(),
    );
    let (mut covector, mut sum) = combine(
        &claim.forms,
        &claim.values,
        &claim.commitment.out_of_domain,
        &coefficients,
        claim.config.initial_size(),
    );
    while covector.len() > common.initial_size() {
        let next = next_config(&claim.config, common)?;
        let opening = claim
            .config
            .initial_sumcheck
            .verify(state, &mut sum)
            .map_err(|e| anyhow::anyhow!("alignment sumcheck: {e:?}"))?;
        for &r in &opening.round_challenges {
            covector.fold(r);
        }
        let commitment = next
            .receive_commitment(state)
            .map_err(|e| anyhow::anyhow!("alignment root: {e:?}"))?;
        let in_domain = claim
            .config
            .initial_committer
            .verify(state, &[&claim.commitment.irs])
            .map_err(|e| anyhow::anyhow!("alignment opening: {e:?}"))?;
        let coefficients: Vec<Fr> = geometric_challenge(state, in_domain.points.len());
        add_queries(
            &mut covector,
            &mut sum,
            &in_domain,
            &opening.round_challenges,
            &coefficients,
        );
        if covector.len() > common.initial_size() {
            let coefficients: Vec<Fr> =
                geometric_challenge(state, commitment.out_of_domain.points.len());
            covector.accumulate_geometric(
                &commitment.out_of_domain.points,
                &Buffer::from(coefficients.as_slice()),
                covector.len(),
            );
            sum += dot(&coefficients, commitment.out_of_domain.matrix.to_slice());
        }
        claim.config = next;
        claim.commitment = commitment;
    }
    claim.forms = vec![Box::new(Covector::new(covector.to_slice().to_vec()))];
    claim.values = vec![sum];
    Ok(claim)
}

pub(super) fn prove_batch(
    claims: Vec<ProverClaim>,
    common: &WhirConfig,
    state: &mut ProverState,
    profile: &mut Vec<StageTiming>,
) -> Result<()> {
    let mut reduced = Vec::new();
    for claim in claims {
        let part = "source";
        reduced.push(measure(
            profile,
            "opening",
            part,
            "authenticated WHIR alignment and constraint combination",
            || reduce_prover(claim, common, state),
        )?);
    }
    let count = reduced.len();
    // Distinct linear forms require the full evaluation matrix. The known
    // diagonal is not resent; bind every cross evaluation before native RLC.
    let cross_start = Instant::now();
    // reduce_prover has already made each form dense. Borrow those vectors
    // instead of allocating and copying one for every off-diagonal entry.
    let covectors = reduced
        .iter()
        .map(|claim| {
            let form: &dyn std::any::Any = claim.forms[0].as_ref();
            &form
                .downcast_ref::<Covector<Fr>>()
                .expect("reduced form is dense")
                .vector
        })
        .collect::<Vec<_>>();
    let evaluations = (0..count * count)
        .into_par_iter()
        .map(|entry| {
            let (i, j) = (entry / count, entry % count);
            if i == j {
                reduced[i].values[0]
            } else {
                dot(covectors[i], reduced[j].message.to_slice())
            }
        })
        .collect::<Vec<_>>();
    // Parallel arithmetic must not reorder Fiat--Shamir messages.
    for (entry, value) in evaluations.iter().enumerate() {
        if entry / count != entry % count {
            state.prover_message(value);
        }
    }
    profile.push(StageTiming {
        phase: "opening",
        part: "batched WHIR",
        stage: "distinct-functional cross evaluations",
        seconds: cross_start.elapsed().as_secs_f64(),
    });
    let forms = reduced
        .iter_mut()
        .map(|c| c.forms.pop().expect("reduced form"))
        .collect();
    let vectors = reduced.iter().map(|c| &c.message).collect::<Vec<_>>();
    let witnesses = reduced.iter().map(|c| &c.witness).collect();
    measure(
        profile,
        "opening",
        "batched WHIR",
        "shared recursive WHIR proof",
        || {
            let _ = common.prove(state, &vectors, witnesses, forms, Buffer::from(evaluations));
        },
    );
    Ok(())
}

pub(super) fn verify_batch(
    claims: Vec<VerifierClaim>,
    common: &WhirConfig,
    state: &mut VerifierState,
) -> Result<()> {
    let mut reduced = Vec::new();
    for claim in claims {
        reduced.push(reduce_verifier(claim, common, state)?);
    }
    let count = reduced.len();
    let mut evaluations = Vec::with_capacity(count * count);
    for (i, claim) in reduced.iter().enumerate() {
        for j in 0..count {
            evaluations.push(if i == j {
                claim.values[0]
            } else {
                state
                    .prover_message()
                    .map_err(|e| anyhow::anyhow!("batch cross term: {e:?}"))?
            });
        }
    }
    let commitments = reduced.iter().map(|c| &c.commitment).collect::<Vec<_>>();
    common
        .verify(state, &commitments, &evaluations)
        .map_err(|e| anyhow::anyhow!("batched WHIR: {e:?}"))?
        .verify(reduced.iter().map(|c| c.forms[0].as_ref()))
        .map_err(|e| anyhow::anyhow!("batched forms: {e:?}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn batching_binds_alignment_roots_claims_and_target() {
        let k: usize = 512;
        let width = 4;
        let encoder = BoltMaxEncoder::new(k, width, 42).unwrap();
        let input = (0..k * width)
            .map(|i| crate::benchmark_field_element(42, 29, i as u64))
            .collect::<Vec<_>>();
        for (layout, heights, target) in [
            (CanopyLayout::EqualBlocks, None, 6),
            (CanopyLayout::EqualBlocks, None, 2),
            (CanopyLayout::EqualBlocks, Some((3, 1)), 6),
            (CanopyLayout::EqualHalf, Some((5, 3)), 4),
            (CanopyLayout::EqualFull, Some((9, 7)), 3),
        ] {
            let run = prove_mode(&encoder, &input, heights, 42, layout, Some(target)).unwrap();
            verify_mode(
                &encoder,
                11,
                heights,
                42,
                run.evaluation,
                &run.proof,
                layout,
                Some(target),
            )
            .unwrap();
            assert!(verify_mode(
                &encoder,
                11,
                heights,
                42,
                run.evaluation + Fr::ONE,
                &run.proof,
                layout,
                Some(target)
            )
            .is_err());
            #[cfg(not(debug_assertions))]
            assert!(verify_mode(
                &encoder,
                11,
                heights,
                42,
                run.evaluation,
                &run.proof,
                layout,
                None
            )
            .is_err());
            for offset in [
                0,
                run.proof.narg_string.len() / 2,
                run.proof.narg_string.len() - 1,
            ] {
                let mut corrupt = run.proof.clone();
                corrupt.narg_string[offset] ^= 1;
                assert!(verify_mode(
                    &encoder,
                    11,
                    heights,
                    42,
                    run.evaluation,
                    &corrupt,
                    layout,
                    Some(target)
                )
                .is_err());
            }
        }
    }
    #[test]
    fn batch_size_matches_serialized_small_proof() {
        for eta in [1, 2, 4, 8] {
            for canopy in [false, true] {
                let log_k = 6;
                let common = whir_config(1, 1, true).unwrap();
                let mut configs = vec![
                    initial_config(log_k, &common).unwrap(),
                    initial_config(log_k - 3, &common).unwrap(),
                ];
                if canopy {
                    configs.extend([common.clone(), common.clone()]);
                }
                configs = (0..eta).flat_map(|_| configs.clone()).collect();
                let domain = DomainSeparator::protocol(&"binary-batch-size-test").instance(&Empty);
                let mut state = ProverState::new(&domain, Blake3Transcript::default());
                let mut claims = Vec::new();
                let mut values = Vec::new();
                for (i, cfg) in configs.iter().enumerate() {
                    let message = Buffer::from(
                        (0..cfg.initial_size())
                            .map(|j| crate::benchmark_field_element(73, i as u64, j as u64))
                            .collect::<Vec<_>>(),
                    );
                    let witness = cfg.commit(&mut state, &[&message]);
                    let form = vec![Fr::ONE; message.len()];
                    let value = dot(&form, message.to_slice());
                    values.push(value);
                    claims.push(ProverClaim {
                        config: cfg.clone(),
                        message,
                        witness,
                        forms: vec![Box::new(Covector::new(form))],
                        values: vec![value],
                    });
                }
                prove_batch(claims, &common, &mut state, &mut Vec::new()).unwrap();
                let proof = state.proof();
                let modeled = batch_size_inputs(
                    log_k,
                    &common,
                    canopy,
                    eta,
                    &mut ChaCha20Rng::seed_from_u64(42),
                )
                .unwrap()
                .bytes() as usize;
                let framing = 8 * (3 * eta + configs.len() + common.round_configs.len());
                assert_eq!(
                    proof.narg_string.len() + proof.hints.len(),
                    modeled + framing
                );
                let mut verifier = VerifierState::new(&domain, &proof, Blake3Transcript::default());
                let claims = configs
                    .into_iter()
                    .zip(values)
                    .map(|(config, value)| {
                        let commitment = config.receive_commitment(&mut verifier).unwrap();
                        let form = vec![Fr::ONE; config.initial_size()];
                        VerifierClaim {
                            config,
                            commitment,
                            forms: vec![Box::new(Covector::new(form))],
                            values: vec![value],
                        }
                    })
                    .collect();
                verify_batch(claims, &common, &mut verifier).unwrap();
                verifier.check_eof().unwrap();
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComparisonRow {
    #[serde(default = "one_input")]
    pub input_oracles: usize,
    pub log_input: usize,
    pub interleaving: usize,
    pub layout: CanopyLayout,
    pub scheme: String,
    pub batched: bool,
    pub systematic_h: Option<usize>,
    pub parity_h: Option<usize>,
    pub aligned_log: Option<usize>,
    pub systematic_queries: usize,
    pub parity_queries: usize,
    pub external_row_bytes: f64,
    pub external_auth_bytes: f64,
    pub whir_and_claim_bytes: f64,
    pub total_bytes: f64,
    pub verifier_leaf_hashes: f64,
    pub verifier_parent_hashes: f64,
    pub verifier_auth_hashes: f64,
    pub verifier_hash_to_field: f64,
    pub external_verifier_auth_hashes: f64,
    pub whir_verifier_auth_hashes: f64,
    pub samples: usize,
    pub selected: bool,
}

fn one_input() -> usize {
    1
}

pub(super) fn prefix_size(log: usize, common: &WhirConfig, rng: &mut ChaCha20Rng) -> Result<Size> {
    prefix_size_from_config(initial_config(log, common)?, common, rng)
}

pub(super) fn prefix_size_from_config(
    mut current: WhirConfig,
    common: &WhirConfig,
    rng: &mut ChaCha20Rng,
) -> Result<Size> {
    let mut size = Size {
        hashes: 1.,
        fields: current.initial_out_domain_samples as f64,
        ..Default::default()
    };
    while current.initial_size() > common.initial_size() {
        size.fields += (2 * current.initial_sumcheck.num_rounds()) as f64;
        size.add(oracle_size(&current.initial_committer, rng)?);
        current = next_config(&current, common)?;
        size.hashes += 1.;
        size.fields += current.initial_out_domain_samples as f64;
    }
    Ok(size)
}

fn batch_size_inputs(
    log_k: usize,
    common: &WhirConfig,
    canopy: bool,
    input_oracles: usize,
    rng: &mut ChaCha20Rng,
) -> Result<Size> {
    let count = input_oracles * if canopy { 4 } else { 2 };
    let mut size = Size::default();
    for _ in 0..input_oracles {
        size.add(prefix_size(log_k, common, rng)?);
        size.add(prefix_size(log_k - 3, common, rng)?);
    }
    if canopy {
        size.hashes += (2 * input_oracles) as f64;
        size.fields += (2 * input_oracles * common.initial_out_domain_samples) as f64;
    }
    let mut shared_queries = rng.clone();
    let mut tail = whir_size(common, rng)?;
    tail.hashes -= 1.; // Every input root/OOD is counted above.
    tail.fields -= common.initial_out_domain_samples as f64;
    // Native batching shares query positions, but authenticates each source root.
    let extra_oracle = oracle_size(&common.initial_committer, &mut shared_queries)?;
    for _ in 1..count {
        tail.add(extra_oracle);
    }
    // Functional and OOD cross matrices. The diagonal is already known.
    tail.fields += (count * (count - 1) * (1 + common.initial_out_domain_samples)) as f64;
    size.add(tail);
    Ok(size)
}

/// Search every compatible shared message length for the batched suite.
/// Full-height constant Canopy layers are included as separate openings, with
/// x/u still batched at their independently optimized join length.
pub fn estimate_grid(
    logs: &[usize],
    input_counts: &[usize],
    samples: usize,
    seed: u64,
    width: usize,
    output: &Path,
) -> Result<Vec<ComparisonRow>> {
    ensure!(samples > 0, "positive samples required");
    std::fs::create_dir_all(output)?;
    let mut rows = Vec::new();
    let mut grid = Vec::new();
    ensure!([4, 16].contains(&width), "supported widths are 4 and 16");
    for &input_oracles in input_counts {
        ensure!([1, 2, 4, 8].contains(&input_oracles), "invalid input count");
        let (qx, qy) = multi::query_counts_for_inputs(input_oracles);
        for &n in logs {
            eprintln!("Bolt proof search: n={n}, eta={input_oracles}, width={width}");
            ensure!(n >= width.ilog2() as usize + 6, "input too small");
            let log_k = n - width.ilog2() as usize;
            let k = 1usize << log_k;
            let external =
                ((qx.min(k) + qy.min(k / 4)) * width * FIELD_BYTES * input_oracles) as f64;
            // Sample once per part and reuse every height/layout. No codewords
            // are allocated, including at N=2^30.
            let mut bottom = Vec::new();
            for (part, count) in [(k, qx), (k / 4, qy)] {
                let mut means = vec![[0.0; 4]; part.ilog2() as usize + 1];
                for sample in 0..samples {
                    let indices = sample_unique_indices(
                        part,
                        count.min(part),
                        seed.wrapping_add(sample as u64),
                    )?;
                    for (h, work) in crate::merkle::authentication_work_by_height(&indices, part)?
                        .iter()
                        .enumerate()
                    {
                        means[h][0] += work.auth as f64 / samples as f64;
                        means[h][1] += work.leaves as f64 / samples as f64;
                        means[h][2] += work.forest_parents as f64 / samples as f64;
                        means[h][3] += work.roots as f64 / samples as f64;
                    }
                }
                bottom.push(means);
            }
            for layout in [
                CanopyLayout::EqualBlocks,
                CanopyLayout::EqualHalf,
                CanopyLayout::EqualFull,
            ] {
                let mut groups = BTreeMap::<&str, Vec<ComparisonRow>>::new();
                for target in 1..=log_k - 3 {
                    let h = log_k - target;
                    for (scheme, choice) in [
                        ("bolt_merkle_batched", None),
                        ("bolt_canopy_batched", Some((h, h - 2))),
                        ("bolt_canopy_batched", Some((log_k, log_k - 2))),
                    ] {
                        if scheme == "bolt_merkle_batched" && layout != CanopyLayout::EqualBlocks {
                            continue;
                        }
                        let common = common_config(k, choice, layout, target)?;
                        let has_canopy = choice.is_some_and(|h| h.0 < log_k);
                        let mut measured = Size::default();
                        for sample in 0..samples {
                            measured.add(batch_size_inputs(
                                log_k,
                                &common,
                                has_canopy,
                                input_oracles,
                                &mut ChaCha20Rng::seed_from_u64(
                                    seed ^ (n as u64) << 32 ^ sample as u64,
                                ),
                            )?);
                        }
                        let mut bytes = measured.bytes() / samples as f64
                            + ((width + 1) * FIELD_BYTES * input_oracles) as f64;
                        let (hx, hy) = choice.unwrap_or((log_k, log_k - 2));
                        let bx = bottom[0][hx].map(|x| x * input_oracles as f64);
                        let by = bottom[1][hy].map(|x| x * input_oracles as f64);
                        let mut auth = (bx[0] + by[0]) * HASH_BYTES as f64;
                        let external_leaf = bx[1] + by[1];
                        let external_parent = bx[2] + by[2];
                        let mut leaf =
                            external_leaf + measured.verifier_leaf_hashes / samples as f64;
                        let mut parent =
                            external_parent + measured.verifier_parent_hashes / samples as f64;
                        let h2f = if choice.is_some() { bx[3] + by[3] } else { 0.0 };
                        if choice.is_none() {
                            auth += (2 * HASH_BYTES * input_oracles) as f64;
                        } else if !has_canopy {
                            bytes += (2 * (HASH_BYTES + FIELD_BYTES) * input_oracles) as f64;
                            // constant_root hashes one field and doubles its
                            // digest once per level of the constant RS tree.
                            leaf += (2 * input_oracles) as f64;
                            parent += (layout.systematic_rs_length(k).ilog2()
                                + layout.parity_rs_length(k).ilog2())
                                as f64
                                * input_oracles as f64;
                        }
                        groups.entry(scheme).or_default().push(ComparisonRow {
                            input_oracles,
                            log_input: n,
                            interleaving: width,
                            layout,
                            scheme: scheme.into(),
                            batched: true,
                            systematic_h: choice.map(|h| h.0),
                            parity_h: choice.map(|h| h.1),
                            aligned_log: Some(target),
                            systematic_queries: qx.min(k),
                            parity_queries: qy.min(k / 4),
                            external_row_bytes: external,
                            external_auth_bytes: auth,
                            whir_and_claim_bytes: bytes,
                            total_bytes: external + auth + bytes,
                            verifier_leaf_hashes: leaf,
                            verifier_parent_hashes: parent,
                            verifier_auth_hashes: leaf + parent,
                            verifier_hash_to_field: h2f,
                            external_verifier_auth_hashes: external_leaf + external_parent,
                            whir_verifier_auth_hashes: leaf + parent
                                - external_leaf
                                - external_parent,
                            samples,
                            selected: false,
                        });
                    }
                }
                for (_, mut candidates) in groups {
                    let best = (0..candidates.len())
                        .min_by(|&a, &b| {
                            candidates[a]
                                .total_bytes
                                .total_cmp(&candidates[b].total_bytes)
                        })
                        .unwrap();
                    candidates[best].selected = true;
                    rows.push(candidates[best].clone());
                    grid.extend(candidates);
                }
            }
        }
    }
    write_csv(output.join("proof_sizes.csv"), &rows)?;
    write_csv(output.join("batching_height_search.csv"), &grid)?;
    Ok(rows)
}

pub fn time_selected(
    n: usize,
    width: usize,
    warmups: usize,
    repetitions: usize,
    seed: u64,
    estimate_dir: &Path,
    output: &Path,
) -> Result<()> {
    ensure!(repetitions > 0, "positive repetitions required");
    let mut choices: Vec<ComparisonRow> =
        csv::Reader::from_path(estimate_dir.join("proof_sizes.csv"))?
            .deserialize::<ComparisonRow>()
            .collect::<std::result::Result<Vec<_>, _>>()?
            .into_iter()
            .filter(|r| r.log_input == n && r.interleaving == width)
            .collect();
    ensure!(
        choices.len() == 4,
        "expected one batched Merkle and three batched Canopy layouts for n={n}, width={width}"
    );
    std::fs::create_dir_all(output)?;
    let encoder = BoltMaxEncoder::new((1 << n) / width, width, seed)?;
    let input = (0..1usize << n)
        .map(|i| crate::benchmark_field_element(seed, 0x424f4c54, i as u64))
        .collect::<Vec<_>>();
    let mut timings = Vec::new();
    let mut stage_rows = Vec::new();
    for iteration in 0..warmups + repetitions {
        choices.rotate_left(1);
        for choice in &choices {
            eprintln!(
                "Bolt timing n={n} w={width}, iteration={iteration}: {} {:?}",
                choice.scheme, choice.layout
            );
            let heights = choice.systematic_h.zip(choice.parity_h);
            let mut run = prove_mode(
                &encoder,
                &input,
                heights,
                seed,
                choice.layout,
                choice.aligned_log,
            )?;
            let start = Instant::now();
            verify_mode(
                &encoder,
                n,
                heights,
                seed,
                run.evaluation,
                &run.proof,
                choice.layout,
                choice.aligned_log,
            )
            .context("verify measured Bolt proof")?;
            run.row.verification_seconds = start.elapsed().as_secs_f64();
            run.row.proof_verified = true;
            if iteration < warmups {
                continue;
            }
            run.row.iteration = iteration - warmups;
            run.row.scheme = choice.scheme.clone();
            for p in run.profile {
                stage_rows.push(StageRow {
                    input_oracles: 1,
                    canopy_layout: choice.layout,
                    log_input: n,
                    interleaving: width,
                    scheme: choice.scheme.clone(),
                    systematic_h: choice.systematic_h,
                    parity_h: choice.parity_h,
                    setup_seed: seed,
                    rayon_threads: rayon::current_num_threads(),
                    iteration: iteration - warmups,
                    phase: p.phase,
                    part: p.part,
                    stage: p.stage,
                    seconds: p.seconds,
                });
            }
            timings.push(run.row);
            // Persist completed samples so long runs remain inspectable.
            write_csv(output.join("prover_timings.csv"), &timings)?;
            write_csv(output.join("prover_timing_stages.csv"), &stage_rows)?;
        }
    }
    Ok(())
}
