//! Multi-input Bolt experiment: independent commitments and front transitions,
//! followed by one shared WHIR tail across all inputs. Widths four and sixteen are supported.
use super::*;

pub(super) fn query_counts_for_inputs(input_oracles: usize) -> (usize, usize) {
    assert!(input_oracles.is_power_of_two());
    let bits = QUERY_SECURITY + input_oracles.ilog2() as usize;
    let (x, y) = proximity_radii();
    (
        (bits as f64 / -(1.0 - x).log2()).ceil() as usize,
        (bits as f64 / -(1.0 - y).log2()).ceil() as usize,
    )
}
fn domain(
    n: usize,
    width: usize,
    heights: Option<(usize, usize)>,
    seed: u64,
    layout: CanopyLayout,
    target: usize,
    input_oracles: usize,
) -> DomainSeparator<'static, ()> {
    DomainSeparator::protocol(&(
        "canopy-system/bolt-max/v5/multi-input",
        crate::FIELD_MODULUS_LABEL,
        n,
        width,
        heights,
        seed,
        format!("{layout:?}"),
        target,
        input_oracles,
        (
            SECURITY,
            QUERY_SECURITY + input_oracles.ilog2() as usize,
            WHIR_SECURITY,
            FOLD,
        ),
    ))
}
#[allow(clippy::too_many_arguments)]
pub(super) fn prove_front(
    encoder: &BoltMaxEncoder,
    input: &[Fr],
    encoded: &bolt::EncodedBolt,
    x_layer: Layer,
    y_layer: Layer,
    common: [&WhirConfig; 2],
    input_oracles: usize,
    state: &mut ProverState,
    profile: &mut Vec<StageTiming>,
) -> Result<(Vec<batching::ProverClaim>, Fr)> {
    let k = encoder.message_rows;
    let width = encoder.interleaving;
    let (z, _z_weights, g, evaluation) =
        measure(profile, "opening", "Bolt", "partial evaluation g", || {
            let z = public_point(k.ilog2() as usize);
            let z_weights = eq_weights(&z);
            let g = (0..width)
                .map(|col| {
                    input
                        .chunks_exact(width)
                        .zip(&z_weights)
                        .map(|(row, &w)| row[col] * w)
                        .sum::<Fr>()
                })
                .collect::<Vec<_>>();
            let evaluation = dot(&g, &eq_weights(&public_point(width.ilog2() as usize)));
            state.prover_messages(&g);
            (z, z_weights, g, evaluation)
        });
    let (r, x, u) = measure(
        profile,
        "opening",
        "Bolt",
        "column projections x=Xr and u=Ur",
        || {
            let r: Vec<Fr> = state.verifier_message_vec(width);
            let x = Buffer::from(bolt::project_rows(input, &r));
            let u = Buffer::from(bolt::project_rows(&encoded.syndrome, &r));
            (r, x, u)
        },
    );
    let (x_cfg, u_cfg) = measure(
        profile,
        "opening",
        "common WHIR",
        "WHIR configuration",
        || -> Result<_> {
            Ok((
                batching::initial_config(k.ilog2() as usize, common[0])?,
                batching::initial_config(k.ilog2() as usize - 3, common[1])?,
            ))
        },
    )?;
    let x_witness = measure(
        profile,
        "opening",
        "x",
        "common WHIR RS encode and oracle commitment",
        || x_cfg.commit(state, &[&x]),
    );
    let u_witness = measure(
        profile,
        "opening",
        "u",
        "common WHIR RS encode and oracle commitment",
        || u_cfg.commit(state, &[&u]),
    );
    let (s, s_weights, sigma) = measure(
        profile,
        "opening",
        "Bolt",
        "syndrome evaluation sigma",
        || {
            let s: Vec<Fr> = state.verifier_message_vec(k.ilog2() as usize - 3);
            let s_weights = eq_weights(&s);
            let sigma = dot(u.to_slice(), &s_weights);
            state.prover_message(&sigma);
            (s, s_weights, sigma)
        },
    );
    let (ix, iy) = measure(profile, "opening", "Bolt", "sample row queries", || {
        let (qx, qy) = query_counts_for_inputs(input_oracles);
        let ix = queries(state, k, qx);
        let iy = queries(state, k / 4, qy);
        (ix, iy)
    });
    let ox = x_layer.open(state, input, &ix, profile, "X")?;
    let oy = y_layer.open(state, &encoded.parity, &iy, profile, "Y")?;
    let (gamma_x, gamma_y, vx, vy) = measure(
        profile,
        "opening",
        "Bolt",
        "project queried rows and sample batching challenges",
        || {
            let gamma_x: Fr = state.verifier_message();
            let gamma_y: Fr = state.verifier_message();
            let vx = ox.iter().map(|v| dot(v, &r)).collect::<Vec<_>>();
            let vy = oy.iter().map(|v| dot(v, &r)).collect::<Vec<_>>();
            (gamma_x, gamma_y, vx, vy)
        },
    );
    let (x_spots, x_value) = measure(
        profile,
        "opening",
        "Bolt",
        "systematic query covector",
        || sparse_claim(k, &ix, &vx, gamma_x),
    );
    let (u_spots, u_value) = measure(
        profile,
        "opening",
        "Bolt",
        "parity query covector and transpose RS FFT",
        || parity_claim(k / 4, &iy, &vy, gamma_y),
    );
    let h_form = measure(
        profile,
        "opening",
        "Bolt",
        "public H transpose covector",
        || encoder.transpose(&s_weights).map(Covector::new),
    )?;
    let mut claims = vec![
        batching::ProverClaim {
            config: x_cfg,
            message: x,
            witness: x_witness,
            forms: vec![
                Box::new(MultilinearExtension::new(z)),
                Box::new(h_form),
                Box::new(x_spots),
            ],
            values: vec![dot(&g, &r), sigma, x_value],
        },
        batching::ProverClaim {
            config: u_cfg,
            message: u,
            witness: u_witness,
            forms: vec![Box::new(MultilinearExtension::new(s)), Box::new(u_spots)],
            values: vec![sigma, u_value],
        },
    ];
    if let Some(claim) = x_layer.batch_claim(state, &ix, profile, "X")? {
        claims.push(claim);
    }
    if let Some(claim) = y_layer.batch_claim(state, &iy, profile, "Y")? {
        claims.push(claim);
    }
    Ok((claims, evaluation))
}

#[allow(clippy::too_many_arguments)]
pub(super) fn verify_front(
    encoder: &BoltMaxEncoder,
    evaluation: Fr,
    x_layer: ReceivedLayer,
    y_layer: ReceivedLayer,
    common: [&WhirConfig; 2],
    input_oracles: usize,
    state: &mut VerifierState,
) -> Result<Vec<batching::VerifierClaim>> {
    let k = encoder.message_rows;
    let width = encoder.interleaving;
    let g: Vec<Fr> = state
        .prover_messages_vec(width)
        .map_err(|e| anyhow::anyhow!("g: {e:?}"))?;
    ensure!(
        dot(&g, &eq_weights(&public_point(width.ilog2() as usize))) == evaluation,
        "evaluation claim mismatch"
    );
    let r: Vec<Fr> = state.verifier_message_vec(width);
    let x_cfg = batching::initial_config(k.ilog2() as usize, common[0])?;
    let u_cfg = batching::initial_config(k.ilog2() as usize - 3, common[1])?;
    let xc = x_cfg
        .receive_commitment(state)
        .map_err(|e| anyhow::anyhow!("x commitment: {e:?}"))?;
    let uc = u_cfg
        .receive_commitment(state)
        .map_err(|e| anyhow::anyhow!("u commitment: {e:?}"))?;
    let s: Vec<Fr> = state.verifier_message_vec(k.ilog2() as usize - 3);
    let s_weights = eq_weights(&s);
    let sigma: Fr = state
        .prover_message()
        .map_err(|e| anyhow::anyhow!("sigma: {e:?}"))?;
    let (qx, qy) = query_counts_for_inputs(input_oracles);
    let ix = queries(state, k, qx);
    let iy = queries(state, k / 4, qy);
    let (ox, hx) = x_layer.open(state, &ix)?;
    let (oy, hy) = y_layer.open(state, &iy)?;
    let gamma_x = state.verifier_message();
    let gamma_y = state.verifier_message();
    let vx = ox.iter().map(|v| dot(v, &r)).collect::<Vec<_>>();
    let vy = oy.iter().map(|v| dot(v, &r)).collect::<Vec<_>>();
    let (x_spots, x_value) = sparse_claim(k, &ix, &vx, gamma_x);
    let (u_spots, u_value) = parity_claim(k / 4, &iy, &vy, gamma_y);
    let h_form = Covector::new(encoder.transpose(&s_weights)?);
    let mut claims = vec![
        batching::VerifierClaim {
            config: x_cfg,
            commitment: xc,
            forms: vec![
                Box::new(MultilinearExtension::new(public_point(k.ilog2() as usize))),
                Box::new(h_form),
                Box::new(x_spots),
            ],
            values: vec![dot(&g, &r), sigma, x_value],
        },
        batching::VerifierClaim {
            config: u_cfg,
            commitment: uc,
            forms: vec![Box::new(MultilinearExtension::new(s)), Box::new(u_spots)],
            values: vec![sigma, u_value],
        },
    ];
    if let Some(claim) = x_layer.batch_claim(state, &ix, &ox, &hx)? {
        claims.push(claim);
    }
    if let Some(claim) = y_layer.batch_claim(state, &iy, &oy, &hy)? {
        claims.push(claim);
    }
    Ok(claims)
}

pub(super) struct Run {
    pub proof: Proof,
    pub row: TimingRow,
    pub evaluations: Vec<Fr>,
    pub profile: Vec<StageTiming>,
}

pub(super) fn prove(
    encoder: &BoltMaxEncoder,
    inputs: &[Vec<Fr>],
    heights: Option<(usize, usize)>,
    seed: u64,
    layout: CanopyLayout,
    target: usize,
) -> Result<Run> {
    let eta = inputs.len();
    let k = encoder.message_rows;
    let width = encoder.interleaving;
    ensure!(
        [4, 16].contains(&width) && [1, 2, 4, 8].contains(&eta),
        "expected w in 4,16 and eta in 1,2,4,8"
    );
    ensure!(
        inputs.iter().all(|v| v.len() == k * width),
        "wrong input shape"
    );
    let n = (k * width).ilog2() as usize;
    let common = batching::common_config(k, heights, layout, target)?;
    let source_count = eta
        * if heights.is_some_and(|h| h.0 < k.ilog2() as usize) {
            4
        } else {
            2
        };
    ensure!(
        common.security_level(source_count, source_count) + 1e-9 >= WHIR_SECURITY as f64,
        "insufficient shared WHIR security for batch"
    );
    let domain = domain(n, width, heights, seed, layout, target, eta).instance(&Empty);
    let mut state = ProverState::new(&domain, Blake3Transcript::default());
    let mut profile = Vec::new();
    let mut committed = Vec::with_capacity(eta);
    let start = Instant::now();
    for input in inputs {
        let syndrome = measure(
            &mut profile,
            "commitment",
            "Bolt",
            "sparse syndrome U=HX",
            || encoder.syndrome(input, width),
        )?;
        let parity = measure(
            &mut profile,
            "commitment",
            "Bolt",
            "parity RS encoding Y=RS(U)",
            || bolt::encode_columns(&syndrome, width),
        )?;
        let x_layer = Layer::commit(
            &mut state,
            input,
            width,
            heights.map(|h| h.0),
            layout.systematic_rs_length(k),
            &mut profile,
            "X",
        )?;
        let y_layer = Layer::commit(
            &mut state,
            &parity,
            width,
            heights.map(|h| h.1),
            layout.parity_rs_length(k),
            &mut profile,
            "Y",
        )?;
        committed.push((bolt::EncodedBolt { syndrome, parity }, x_layer, y_layer));
    }
    let commitment = start.elapsed().as_secs_f64();
    let encoding = profile
        .iter()
        .filter(|p| p.part == "Bolt" && p.phase == "commitment")
        .map(|p| p.seconds)
        .sum();
    let start = Instant::now();
    let mut claims = Vec::new();
    let mut evaluations = Vec::new();
    for (input, (encoded, x_layer, y_layer)) in inputs.iter().zip(committed) {
        let (front, value) = prove_front(
            encoder,
            input,
            &encoded,
            x_layer,
            y_layer,
            [&common, &common],
            eta,
            &mut state,
            &mut profile,
        )?;
        claims.extend(front);
        evaluations.push(value);
    }
    batching::prove_batch(claims, &common, &mut state, &mut profile)?;
    let opening = start.elapsed().as_secs_f64();
    for (phase, total) in [("commitment", commitment), ("opening", opening)] {
        let accounted = profile
            .iter()
            .filter(|p| p.phase == phase)
            .map(|p| p.seconds)
            .sum::<f64>();
        ensure!(accounted <= total, "overlapping timers in {phase}");
        profile.push(StageTiming {
            phase,
            part: "other",
            stage: "unassigned phase work and timing overhead",
            seconds: total - accounted,
        });
    }
    let proof = state.proof();
    let row = TimingRow {
        input_oracles: eta,
        canopy_layout: layout,
        interleaving: width,
        log_input: n,
        setup_seed: seed,
        rayon_threads: rayon::current_num_threads(),
        scheme: if heights.is_some() {
            "bolt_canopy_batched"
        } else {
            "bolt_merkle_batched"
        }
        .into(),
        systematic_h: heights.map(|h| h.0),
        parity_h: heights.map(|h| h.1),
        iteration: 0,
        encoding_seconds: encoding,
        commitment_seconds: commitment,
        opening_seconds: opening,
        verification_seconds: 0.,
        total_prover_seconds: commitment + opening,
        proof_bytes: proof.narg_string.len() + proof.hints.len(),
        proof_verified: false,
    };
    Ok(Run {
        proof,
        row,
        evaluations,
        profile,
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn verify(
    encoder: &BoltMaxEncoder,
    n: usize,
    heights: Option<(usize, usize)>,
    seed: u64,
    evaluations: &[Fr],
    proof: &Proof,
    layout: CanopyLayout,
    target: usize,
) -> Result<()> {
    let eta = evaluations.len();
    let k = encoder.message_rows;
    let width = encoder.interleaving;
    ensure!(
        [4, 16].contains(&width) && [1, 2, 4, 8].contains(&eta) && (1 << n) == k * width,
        "invalid instance shape"
    );
    let common = batching::common_config(k, heights, layout, target)?;
    let domain = domain(n, width, heights, seed, layout, target, eta).instance(&Empty);
    let mut state = VerifierState::new(&domain, proof, Blake3Transcript::default());
    let mut layers = Vec::new();
    for _ in 0..eta {
        let x = ReceivedLayer::receive(
            &mut state,
            k,
            width,
            heights.map(|h| h.0),
            layout.systematic_rs_length(k),
        )?;
        let y = ReceivedLayer::receive(
            &mut state,
            k / 4,
            width,
            heights.map(|h| h.1),
            layout.parity_rs_length(k),
        )?;
        layers.push((x, y));
    }
    let mut claims = Vec::new();
    for (&value, (x, y)) in evaluations.iter().zip(layers) {
        claims.extend(verify_front(
            encoder,
            value,
            x,
            y,
            [&common, &common],
            eta,
            &mut state,
        )?);
    }
    batching::verify_batch(claims, &common, &mut state)?;
    state
        .check_eof()
        .map_err(|e| anyhow::anyhow!("trailing proof data: {e:?}"))?;
    Ok(())
}

pub fn time_selected(
    n: usize,
    eta: usize,
    warmups: usize,
    repetitions: usize,
    seed: u64,
    estimates: &Path,
    output: &Path,
) -> Result<()> {
    ensure!(repetitions > 0, "positive repetition count required");
    let mut choices = csv::Reader::from_path(estimates.join("proof_sizes.csv"))?
        .deserialize::<batching::ComparisonRow>()
        .collect::<std::result::Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|r| r.log_input == n && r.input_oracles == eta && r.interleaving == 4)
        .collect::<Vec<_>>();
    ensure!(
        choices.len() == 4,
        "expected Merkle and three Canopy layouts"
    );
    let best = choices
        .iter()
        .filter(|c| c.systematic_h.is_some())
        .min_by(|a, b| a.total_bytes.total_cmp(&b.total_bytes))
        .unwrap()
        .layout;
    choices.retain(|c| c.systematic_h.is_none() || c.layout == best);
    std::fs::create_dir_all(output)?;
    let encoder = BoltMaxEncoder::new((1 << n) / 4, 4, seed)?;
    let inputs = (0..eta)
        .map(|j| {
            (0..1usize << n)
                .map(|i| crate::benchmark_field_element(seed, 0x424f4c54 + j as u64, i as u64))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut rows = Vec::new();
    let mut stages = Vec::new();
    for iteration in 0..warmups + repetitions {
        choices.rotate_left(1);
        for c in &choices {
            eprintln!(
                "Bolt timing n={n} eta={eta} w=4 iteration={iteration}: {} {:?}",
                c.scheme, c.layout
            );
            let heights = c.systematic_h.zip(c.parity_h);
            let mut run = prove(
                &encoder,
                &inputs,
                heights,
                seed,
                c.layout,
                c.aligned_log.unwrap(),
            )?;
            let start = Instant::now();
            verify(
                &encoder,
                n,
                heights,
                seed,
                &run.evaluations,
                &run.proof,
                c.layout,
                c.aligned_log.unwrap(),
            )?;
            run.row.verification_seconds = start.elapsed().as_secs_f64();
            run.row.proof_verified = true;
            if iteration < warmups {
                continue;
            }
            run.row.iteration = iteration - warmups;
            for p in run.profile {
                stages.push(StageRow {
                    input_oracles: eta,
                    canopy_layout: c.layout,
                    log_input: n,
                    interleaving: 4,
                    scheme: c.scheme.clone(),
                    systematic_h: c.systematic_h,
                    parity_h: c.parity_h,
                    setup_seed: seed,
                    rayon_threads: rayon::current_num_threads(),
                    iteration: iteration - warmups,
                    phase: p.phase,
                    part: p.part,
                    stage: p.stage,
                    seconds: p.seconds,
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
    #[test]
    fn multi_input_proofs_bind_every_evaluation_and_batch_shape() {
        let n = 10;
        let k = (1 << n) / 4;
        let encoder = BoltMaxEncoder::new(k, 4, 42).unwrap();
        for eta in [1, 2, 4, 8] {
            let inputs = (0..eta)
                .map(|j| {
                    (0..1usize << n)
                        .map(|i| crate::benchmark_field_element(42, j as u64, i as u64))
                        .collect()
                })
                .collect::<Vec<_>>();
            for heights in [None, Some((3, 1)), Some((8, 6))] {
                let r = prove(&encoder, &inputs, heights, 42, CanopyLayout::EqualFull, 5).unwrap();
                verify(
                    &encoder,
                    n,
                    heights,
                    42,
                    &r.evaluations,
                    &r.proof,
                    CanopyLayout::EqualFull,
                    5,
                )
                .unwrap();
                for i in 0..eta {
                    let mut bad = r.evaluations.clone();
                    bad[i] += Fr::ONE;
                    assert!(verify(
                        &encoder,
                        n,
                        heights,
                        42,
                        &bad,
                        &r.proof,
                        CanopyLayout::EqualFull,
                        5
                    )
                    .is_err());
                }
                if eta > 1 {
                    // Native debug transcripts assert on a mismatched message type;
                    // release transcripts return an ordinary verification error.
                    let bad = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        verify(
                            &encoder,
                            n,
                            heights,
                            42,
                            &r.evaluations[..eta / 2],
                            &r.proof,
                            CanopyLayout::EqualFull,
                            5,
                        )
                    }));
                    assert!(bad.is_err() || bad.unwrap().is_err());
                }
                let mut bad = r.proof.clone();
                bad.narg_string[0] ^= 1;
                assert!(verify(
                    &encoder,
                    n,
                    heights,
                    42,
                    &r.evaluations,
                    &bad,
                    CanopyLayout::EqualFull,
                    5
                )
                .is_err());
            }
        }
    }
}
