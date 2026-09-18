//! Bolt with independent systematic/parity WHIR batches. Canopy RS block lengths
//! are k, k/2, or k/4, or the systematic layer starts at rate 1/2 and joins later.
use super::*;

#[path = "bolt_delayed.rs"]
pub mod delayed;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parameters {
    /// Canopy RS lengths are k >> canopy_log_divisor, except a delayed systematic layer.
    #[serde(default)]
    pub canopy_log_divisor: usize,
    /// Half-rate systematic Canopy, folded independently before the join.
    #[serde(default)]
    pub delayed_systematic: bool,
    pub systematic_h: Option<usize>,
    pub parity_h: Option<usize>,
    pub systematic_target_log: usize,
    pub parity_target_log: usize,
}
impl Parameters {
    fn systematic_rs_length(self, k: usize) -> usize {
        if self.delayed_systematic {
            2 * (k >> self.systematic_h.expect("delayed join requires Canopy"))
        } else {
            k >> self.canopy_log_divisor
        }
    }
    fn configs(self, k: usize, eta: usize) -> Result<[WhirConfig; 2]> {
        ensure!(
            self.canopy_log_divisor <= 2,
            "supported parity Canopy lengths are k, k/2, and k/4"
        );
        let log_k = k.ilog2() as usize;
        let rs_log = log_k - self.canopy_log_divisor;
        let config = |rows_log: usize,
                      source_log: usize,
                      h: Option<usize>,
                      target: usize,
                      delayed: bool|
         -> Result<_> {
            ensure!(
                target > 0
                    && target <= source_log
                    && (delayed || h.is_none() || target < source_log),
                "invalid independent join length"
            );
            let rate = if delayed {
                let h = h.context("delayed systematic join requires a Canopy height")?;
                ensure!(h <= rows_log, "height exceeds tree");
                if h == rows_log {
                    1 // Constant Canopy is opened directly; only the code-switch claim remains.
                } else {
                    let message_log = rows_log - h;
                    ensure!(target <= message_log, "cannot unfold Canopy");
                    let folded = message_log - target;
                    ensure!(
                        folded % FOLD == 0,
                        "join must follow complete three-variable Canopy folds"
                    );
                    1 + folded - folded / FOLD
                }
            } else {
                if let Some(h) = h {
                    ensure!(h <= rows_log, "height exceeds tree");
                    ensure!(
                        h == rows_log || rows_log - h == target,
                        "Canopy message must match its own WHIR tail"
                    );
                }
                let nonconstant = h.is_some_and(|h| h < rows_log);
                ensure!(
                    !nonconstant || target < rs_log,
                    "Canopy RS rate must be at most 1/2"
                );
                if nonconstant {
                    rs_log - target
                } else {
                    1
                }
            };
            let cfg = whir_config(target, rate, true)?;
            let count = eta
                * if h.is_some_and(|h| h < rows_log) {
                    2
                } else {
                    1
                };
            ensure!(
                cfg.security_level(count, count) + 1e-9 >= WHIR_SECURITY as f64,
                "insufficient independent tail security"
            );
            Ok(cfg)
        };
        Ok([
            config(
                log_k,
                log_k,
                self.systematic_h,
                self.systematic_target_log,
                self.delayed_systematic,
            )?,
            config(
                log_k - 2,
                log_k - 3,
                self.parity_h,
                self.parity_target_log,
                false,
            )?,
        ])
    }
    fn canopy_config(self, k: usize, side: usize) -> Result<Option<WhirConfig>> {
        let (rows, height, length) = if side == 0 {
            (k, self.systematic_h, self.systematic_rs_length(k))
        } else {
            (k / 4, self.parity_h, k >> self.canopy_log_divisor)
        };
        height
            .filter(|&h| h < rows.ilog2() as usize)
            .map(|h| {
                whir_config(
                    rows.ilog2() as usize - h,
                    canopy_rate_log(rows, length, h),
                    true,
                )
            })
            .transpose()
    }
}

fn domain(
    n: usize,
    width: usize,
    p: Parameters,
    seed: u64,
    eta: usize,
) -> DomainSeparator<'static, ()> {
    if p.delayed_systematic {
        return DomainSeparator::protocol(&(
            "canopy-system/bolt-max/v8/delayed-systematic",
            crate::FIELD_MODULUS_LABEL,
            n,
            width,
            p.canopy_log_divisor,
            p.systematic_h,
            p.parity_h,
            p.systematic_target_log,
            p.parity_target_log,
            seed,
            eta,
            (
                SECURITY,
                QUERY_SECURITY + eta.ilog2() as usize,
                WHIR_SECURITY,
                FOLD,
            ),
        ));
    }
    DomainSeparator::protocol(&(
        "canopy-system/bolt-max/v7/independent-tails",
        crate::FIELD_MODULUS_LABEL,
        n,
        width,
        p.canopy_log_divisor,
        p.systematic_h,
        p.parity_h,
        p.systematic_target_log,
        p.parity_target_log,
        seed,
        eta,
        (
            SECURITY,
            QUERY_SECURITY + eta.ilog2() as usize,
            WHIR_SECURITY,
            FOLD,
        ),
    ))
}
fn partition<T>(front: Vec<T>, p: Parameters, k: usize) -> [Vec<T>; 2] {
    let mut i = front.into_iter();
    let mut sides = [vec![i.next().unwrap()], vec![i.next().unwrap()]];
    if p.systematic_h.is_some_and(|h| h < k.ilog2() as usize) {
        sides[0].push(i.next().unwrap());
    }
    if p.parity_h.is_some_and(|h| h < k.ilog2() as usize - 2) {
        sides[1].push(i.next().unwrap());
    }
    assert!(i.next().is_none());
    sides
}

pub(super) fn prove(
    encoder: &BoltMaxEncoder,
    inputs: &[Vec<Fr>],
    parameters: Parameters,
    seed: u64,
) -> Result<multi::Run> {
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
    let common = parameters.configs(k, eta)?;
    let domain = domain(n, width, parameters, seed, eta).instance(&Empty);
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
            parameters.systematic_h,
            parameters.systematic_rs_length(k),
            &mut profile,
            "X",
        )?;
        let y_layer = Layer::commit(
            &mut state,
            &parity,
            width,
            parameters.parity_h,
            k >> parameters.canopy_log_divisor,
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
    let mut claims = [Vec::new(), Vec::new()];
    let mut evaluations = Vec::new();
    for (input, (encoded, x_layer, y_layer)) in inputs.iter().zip(committed) {
        let (front, value) = multi::prove_front(
            encoder,
            input,
            &encoded,
            x_layer,
            y_layer,
            [&common[0], &common[1]],
            eta,
            &mut state,
            &mut profile,
        )?;
        let [x, y] = partition(front, parameters, k);
        claims[0].extend(x);
        claims[1].extend(y);
        evaluations.push(value);
    }
    for (side, (claims, common)) in claims.into_iter().zip(&common).enumerate() {
        let first = profile.len();
        batching::prove_batch(claims, common, &mut state, &mut profile)?;
        for event in &mut profile[first..] {
            event.part = if side == 0 {
                "systematic tail"
            } else {
                "parity tail"
            };
        }
    }
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
        canopy_layout: match parameters.canopy_log_divisor {
            0 => CanopyLayout::EqualFull,
            1 => CanopyLayout::EqualHalf,
            2 => CanopyLayout::EqualBlocks,
            _ => unreachable!("validated Canopy block divisor"),
        },
        interleaving: width,
        log_input: n,
        setup_seed: seed,
        rayon_threads: rayon::current_num_threads(),
        scheme: if parameters.delayed_systematic {
            "bolt_canopy_delayed_systematic"
        } else if parameters.systematic_h.is_some() {
            "bolt_canopy_split"
        } else {
            "bolt_merkle_split"
        }
        .into(),
        systematic_h: parameters.systematic_h,
        parity_h: parameters.parity_h,
        iteration: 0,
        encoding_seconds: encoding,
        commitment_seconds: commitment,
        opening_seconds: opening,
        verification_seconds: 0.,
        total_prover_seconds: commitment + opening,
        proof_bytes: proof.narg_string.len() + proof.hints.len(),
        proof_verified: false,
    };
    Ok(multi::Run {
        proof,
        row,
        evaluations,
        profile,
    })
}

pub(super) fn verify(
    encoder: &BoltMaxEncoder,
    n: usize,
    parameters: Parameters,
    seed: u64,
    evaluations: &[Fr],
    proof: &Proof,
) -> Result<()> {
    let eta = evaluations.len();
    let k = encoder.message_rows;
    let width = encoder.interleaving;
    ensure!(
        [4, 16].contains(&width) && [1, 2, 4, 8].contains(&eta) && (1 << n) == k * width,
        "invalid instance shape"
    );
    let common = parameters.configs(k, eta)?;
    let domain = domain(n, width, parameters, seed, eta).instance(&Empty);
    let mut state = VerifierState::new(&domain, proof, Blake3Transcript::default());
    let mut layers = Vec::new();
    for _ in 0..eta {
        let x = ReceivedLayer::receive(
            &mut state,
            k,
            width,
            parameters.systematic_h,
            parameters.systematic_rs_length(k),
        )?;
        let y = ReceivedLayer::receive(
            &mut state,
            k / 4,
            width,
            parameters.parity_h,
            k >> parameters.canopy_log_divisor,
        )?;
        layers.push((x, y));
    }
    let mut claims = [Vec::new(), Vec::new()];
    for (&value, (x, y)) in evaluations.iter().zip(layers) {
        let front = multi::verify_front(
            encoder,
            value,
            x,
            y,
            [&common[0], &common[1]],
            eta,
            &mut state,
        )?;
        let [x, y] = partition(front, parameters, k);
        claims[0].extend(x);
        claims[1].extend(y);
    }
    for (claims, common) in claims.into_iter().zip(&common) {
        batching::verify_batch(claims, common, &mut state)?;
    }
    state
        .check_eof()
        .map_err(|e| anyhow::anyhow!("trailing proof data: {e:?}"))?;
    Ok(())
}

fn default_width() -> usize {
    4
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SideRow {
    pub log_input: usize,
    #[serde(default = "default_width")]
    pub interleaving: usize,
    pub input_oracles: usize,
    pub canopy: bool,
    pub side: String,
    pub height: Option<usize>,
    pub target_log: usize,
    pub canopy_rs_length: usize,
    pub external_row_bytes: f64,
    pub external_auth_bytes: f64,
    pub whir_and_claim_bytes: f64,
    pub total_bytes: f64,
    pub verifier_leaf_hashes: f64,
    pub verifier_parent_hashes: f64,
    pub verifier_hash_to_field: f64,
    pub selected: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Row {
    pub log_input: usize,
    #[serde(default = "default_width")]
    pub interleaving: usize,
    pub input_oracles: usize,
    pub scheme: String,
    #[serde(default)]
    pub delayed_systematic: bool,
    pub systematic_h: Option<usize>,
    pub parity_h: Option<usize>,
    pub systematic_target_log: usize,
    pub parity_target_log: usize,
    pub systematic_rs_length: usize,
    pub parity_rs_length: usize,
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
    pub fs_finalizations: usize,
    pub fs_absorb_bytes: f64,
    pub fs_squeeze_bytes: f64,
    pub fs_proof_bytes: usize,
    pub verifier_hashes: f64,
    pub samples: usize,
}
impl Row {
    fn parameters(&self) -> Parameters {
        let canopy_log_divisor = if self.systematic_h.is_some() {
            if !self.delayed_systematic {
                assert_eq!(self.systematic_rs_length, self.parity_rs_length);
            }
            assert!(self.parity_rs_length.is_power_of_two());
            self.log_input
                - self.interleaving.ilog2() as usize
                - self.parity_rs_length.ilog2() as usize
        } else {
            0
        };
        Parameters {
            canopy_log_divisor,
            delayed_systematic: self.delayed_systematic,
            systematic_h: self.systematic_h,
            parity_h: self.parity_h,
            systematic_target_log: self.systematic_target_log,
            parity_target_log: self.parity_target_log,
        }
    }
}
fn side_size(
    source_log: usize,
    common: &WhirConfig,
    canopy: bool,
    eta: usize,
    rng: &mut ChaCha20Rng,
) -> Result<Size> {
    side_size_with_canopy(source_log, common, canopy.then_some(common), eta, rng)
}

fn side_size_with_canopy(
    source_log: usize,
    common: &WhirConfig,
    canopy: Option<&WhirConfig>,
    eta: usize,
    rng: &mut ChaCha20Rng,
) -> Result<Size> {
    let count = eta * if canopy.is_some() { 2 } else { 1 };
    let mut size = Size::default();
    for _ in 0..eta {
        size.add(batching::prefix_size(source_log, common, rng)?);
    }
    if let Some(canopy) = canopy {
        for _ in 0..eta {
            size.add(batching::prefix_size_from_config(
                canopy.clone(),
                common,
                rng,
            )?);
        }
    }
    let mut first_queries = rng.clone();
    let mut tail = whir_size(common, rng)?;
    tail.hashes -= 1.;
    tail.fields -= common.initial_out_domain_samples as f64;
    let extra = oracle_size(&common.initial_committer, &mut first_queries)?;
    for _ in 1..count {
        tail.add(extra);
    }
    tail.fields += (count * (count - 1) * (1 + common.initial_out_domain_samples)) as f64;
    size.add(tail);
    Ok(size)
}

/// The two objectives add, so optimize each side independently instead of
/// materializing a redundant Cartesian product of all pairs of heights.
pub fn estimate_grid(
    logs: &[usize],
    inputs: &[usize],
    samples: usize,
    seed: u64,
    block_divisor: usize,
    width: usize,
    output: &Path,
) -> Result<()> {
    ensure!(
        [1, 2, 4].contains(&block_divisor),
        "supported block divisors are 1, 2, and 4"
    );
    ensure!([4, 16].contains(&width), "supported widths are 4 and 16");
    ensure!(samples > 0, "positive samples required");
    std::fs::create_dir_all(output)?;
    let mut selected = Vec::new();
    let mut grid = Vec::new();
    for &eta in inputs {
        ensure!([1, 2, 4, 8].contains(&eta), "invalid input count");
        for &n in logs {
            ensure!(n >= width.ilog2() as usize + 6, "input too small");
            eprintln!("Split Bolt proof search: n={n}, eta={eta}, w={width}");
            let log_k = n - width.ilog2() as usize;
            let k = 1usize << log_k;
            let rs_length = k / block_divisor;
            let rs_log = rs_length.ilog2() as usize;
            let (qx, qy) = multi::query_counts_for_inputs(eta);
            let counts = [qx.min(k), qy.min(k / 4)];
            let mut bottom = Vec::new();
            for (part, count) in [(k, counts[0]), (k / 4, counts[1])] {
                let mut means = vec![[0.; 4]; part.ilog2() as usize + 1];
                for sample in 0..samples {
                    let indices =
                        sample_unique_indices(part, count, seed.wrapping_add(sample as u64))?;
                    for (h, work) in crate::merkle::authentication_work_by_height(&indices, part)?
                        .iter()
                        .enumerate()
                    {
                        for (dst, value) in means[h].iter_mut().zip([
                            work.auth,
                            work.leaves,
                            work.forest_parents,
                            work.roots,
                        ]) {
                            *dst += value as f64 * eta as f64 / samples as f64;
                        }
                    }
                }
                bottom.push(means);
            }
            for canopy in [false, true] {
                let mut best = Vec::new();
                for side in 0..2 {
                    let rows_log = log_k - 2 * side;
                    let source_log = log_k - 3 * side;
                    let mut choices = Vec::new();
                    for target in 1..source_log {
                        let heights = if canopy {
                            vec![Some(rows_log - target), Some(rows_log)]
                        } else {
                            vec![None]
                        };
                        for height in heights {
                            let nonconstant = height.is_some_and(|h| h < rows_log);
                            if nonconstant && target >= rs_log {
                                continue;
                            }
                            let common = whir_config(
                                target,
                                if nonconstant { rs_log - target } else { 1 },
                                true,
                            )?;
                            let count = eta * if nonconstant { 2 } else { 1 };
                            ensure!(
                                common.security_level(count, count) + 1e-9 >= WHIR_SECURITY as f64,
                                "insufficient tail security"
                            );
                            let mut measured = Size::default();
                            for sample in 0..samples {
                                measured.add(side_size(
                                    source_log,
                                    &common,
                                    nonconstant,
                                    eta,
                                    &mut ChaCha20Rng::seed_from_u64(
                                        seed ^ (n as u64) << 32 ^ sample as u64,
                                    ),
                                )?);
                            }
                            let b = bottom[side][height.unwrap_or(rows_log)];
                            let external_row_bytes =
                                (counts[side] * width * FIELD_BYTES * eta) as f64;
                            let external_auth_bytes = b[0] * HASH_BYTES as f64
                                + if !canopy {
                                    (HASH_BYTES * eta) as f64
                                } else {
                                    0.
                                };
                            let mut bytes = measured.bytes() / samples as f64;
                            let mut leaf = b[1] + measured.verifier_leaf_hashes / samples as f64;
                            let mut parent =
                                b[2] + measured.verifier_parent_hashes / samples as f64;
                            if canopy && !nonconstant {
                                bytes += ((HASH_BYTES + FIELD_BYTES) * eta) as f64;
                                leaf += eta as f64;
                                parent += (rs_log * eta) as f64;
                            }
                            choices.push(SideRow {
                                log_input: n,
                                interleaving: width,
                                input_oracles: eta,
                                canopy,
                                side: if side == 0 { "systematic" } else { "parity" }.into(),
                                height,
                                target_log: target,
                                canopy_rs_length: if canopy { rs_length } else { 0 },
                                external_row_bytes,
                                external_auth_bytes,
                                whir_and_claim_bytes: bytes,
                                total_bytes: external_row_bytes + external_auth_bytes + bytes,
                                verifier_leaf_hashes: leaf,
                                verifier_parent_hashes: parent,
                                // Digest-to-field conversion is a bijection, not a hash.
                                verifier_hash_to_field: 0.0,
                                selected: false,
                            });
                        }
                    }
                    let i = (0..choices.len())
                        .min_by(|&a, &b| choices[a].total_bytes.total_cmp(&choices[b].total_bytes))
                        .unwrap();
                    choices[i].selected = true;
                    best.push(choices[i].clone());
                    grid.extend(choices);
                }
                let sum = |f: fn(&SideRow) -> f64| best.iter().map(f).sum::<f64>();
                let bytes =
                    sum(|r| r.whir_and_claim_bytes) + ((width + 1) * FIELD_BYTES * eta) as f64;
                let auth = sum(|r| r.external_auth_bytes);
                let rows = sum(|r| r.external_row_bytes);
                let leaf = sum(|r| r.verifier_leaf_hashes);
                let parent = sum(|r| r.verifier_parent_hashes);
                let mut row = Row {
                    log_input: n,
                    interleaving: width,
                    input_oracles: eta,
                    scheme: if canopy && block_divisor == 4 {
                        "bolt_canopy_split_quarter"
                    } else if canopy {
                        "bolt_canopy_split"
                    } else {
                        "bolt_merkle_split"
                    }
                    .into(),
                    systematic_h: best[0].height,
                    parity_h: best[1].height,
                    systematic_target_log: best[0].target_log,
                    parity_target_log: best[1].target_log,
                    systematic_rs_length: best[0].canopy_rs_length,
                    parity_rs_length: best[1].canopy_rs_length,
                    systematic_queries: counts[0],
                    parity_queries: counts[1],
                    external_row_bytes: rows,
                    external_auth_bytes: auth,
                    whir_and_claim_bytes: bytes,
                    total_bytes: rows + auth + bytes,
                    verifier_leaf_hashes: leaf,
                    verifier_parent_hashes: parent,
                    verifier_auth_hashes: leaf + parent,
                    verifier_hash_to_field: sum(|r| r.verifier_hash_to_field),
                    fs_finalizations: 0,
                    fs_absorb_bytes: 0.,
                    fs_squeeze_bytes: 0.,
                    fs_proof_bytes: 0,
                    verifier_hashes: 0.,
                    samples,
                    delayed_systematic: false,
                };
                account(&mut row)?;
                selected.push(row);
            }
        }
    }
    write_csv(output.join("proof_sizes.csv"), &selected)?;
    write_csv(output.join("independent_height_search.csv"), &grid)
}

fn fs_model(
    row: &Row,
    outer_words: &[usize],
    query_bytes: &[usize],
) -> Result<crate::fiat_shamir::Ledger> {
    use crate::fiat_shamir::{native_commit, native_verify, Ledger};
    let eta = row.input_oracles;
    ensure!(
        outer_words.len() == eta && query_bytes.len() == eta,
        "wrong transcript shape"
    );
    let log_k = row.log_input - row.interleaving.ilog2() as usize;
    let k = 1usize << log_k;
    let p = row.parameters();
    let commons = p.configs(k, eta)?;
    let canopy_configs = [p.canopy_config(k, 0)?, p.canopy_config(k, 1)?];
    let source_logs = [log_k, log_k - 3];
    let initial = [
        batching::initial_config(source_logs[0], &commons[0])?,
        batching::initial_config(source_logs[1], &commons[1])?,
    ];
    let heights = [p.systematic_h, p.parity_h];
    let nonconstant = [
        heights[0].is_some_and(|h| h < log_k),
        heights[1].is_some_and(|h| h < log_k - 2),
    ];
    let mut l = Ledger::domain();
    l.fields(2 * eta);
    for i in 0..eta {
        l.fields(row.interleaving);
        l.challenges(row.interleaving);
        native_commit(&mut l, &initial[0]);
        native_commit(&mut l, &initial[1]);
        l.challenges(log_k - 3);
        l.fields(1);
        l.push('s', 1, query_bytes[i]);
        l.fields(outer_words[i]);
        l.challenges(2);
        for side in 0..2 {
            if nonconstant[side] {
                l.challenges(1);
                let canopy = canopy_configs[side].as_ref().unwrap();
                l.challenges(canopy.initial_out_domain_samples);
                l.fields(canopy.initial_out_domain_samples);
            } else if heights[side].is_some() {
                l.fields(1);
            }
        }
    }
    for side in 0..2 {
        for _ in 0..eta {
            fiat_shamir::reduce(
                &mut l,
                initial[side].clone(),
                &commons[side],
                if side == 0 { 3 } else { 2 },
            )?;
            if nonconstant[side] {
                fiat_shamir::reduce(
                    &mut l,
                    canopy_configs[side].clone().unwrap(),
                    &commons[side],
                    1,
                )?;
            }
        }
        let count = eta * if nonconstant[side] { 2 } else { 1 };
        l.fields(count * (count - 1));
        native_verify(&mut l, &commons[side], count, count);
    }
    Ok(l)
}
fn account(r: &mut Row) -> Result<()> {
    let k = (1usize << r.log_input) / r.interleaving;
    let eta = r.input_oracles;
    let roots = if r.systematic_h.is_none() {
        64. * eta as f64
    } else {
        0.
    };
    let outer = (r.external_row_bytes + r.external_auth_bytes - roots) / (32. * eta as f64);
    let qbytes = fiat_shamir::expected_query_bytes(k, r.systematic_queries)
        + fiat_shamir::expected_query_bytes(k / 4, r.parity_queries);
    let l = fs_model(
        r,
        &vec![outer.round() as usize; eta],
        &vec![qbytes.round() as usize; eta],
    )?;
    let c = l.counts();
    r.fs_finalizations = c.finalizations;
    r.fs_absorb_bytes = c.absorb_bytes as f64 + 32. * (outer - outer.round()) * eta as f64;
    r.fs_squeeze_bytes = c.squeeze_bytes as f64 + (qbytes - qbytes.round()) * eta as f64;
    r.verifier_hashes = r.verifier_auth_hashes + r.verifier_hash_to_field + c.finalizations as f64;
    Ok(())
}

fn input_data(n: usize, eta: usize, seed: u64) -> Vec<Vec<Fr>> {
    (0..eta)
        .map(|j| {
            (0..1usize << n)
                .map(|i| crate::benchmark_field_element(seed, 0x424f4c54 + j as u64, i as u64))
                .collect()
        })
        .collect()
}

/// Time both architectures in rotating order on identical inputs. Keep the old
/// configurations explicit so the effect of splitting is directly measurable.
pub fn time_selected(
    n: usize,
    eta: usize,
    warmups: usize,
    repetitions: usize,
    estimates: &Path,
    shared_estimates: &Path,
    output: &Path,
    paper_only: bool,
    width: usize,
    alternate_estimates: Option<&Path>,
    shared_merkle_baseline: bool,
) -> Result<()> {
    ensure!(repetitions > 0, "positive repetitions required");
    ensure!(
        !shared_merkle_baseline || paper_only,
        "fixed shared Merkle baseline requires --paper-only"
    );
    let mut split = csv::Reader::from_path(estimates.join("proof_sizes.csv"))?
        .deserialize::<Row>()
        .collect::<std::result::Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|r| r.log_input == n && r.input_oracles == eta && r.interleaving == width)
        .collect::<Vec<_>>();
    let shared = csv::Reader::from_path(shared_estimates.join("proof_sizes.csv"))?
        .deserialize::<batching::ComparisonRow>()
        .collect::<std::result::Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|r| {
            r.log_input == n
                && r.input_oracles == eta
                && r.interleaving == width
                && (r.systematic_h.is_none() || r.layout == CanopyLayout::EqualFull)
        })
        .collect::<Vec<_>>();
    ensure!(
        split.len() == 2 && shared.len() == 2,
        "expected two shared and two split configurations"
    );
    std::fs::create_dir_all(output)?;
    let seed = DEFAULT_SEED;
    if let Some(other) = alternate_estimates {
        ensure!(paper_only, "alternate estimates require --paper-only");
        let alternate = csv::Reader::from_path(other.join("proof_sizes.csv"))?
            .deserialize::<Row>()
            .collect::<std::result::Result<Vec<_>, _>>()?
            .into_iter()
            .filter(|r| {
                r.log_input == n
                    && r.input_oracles == eta
                    && r.interleaving == width
                    && r.systematic_h.is_some()
            })
            .collect::<Vec<_>>();
        ensure!(
            alternate.len() == 1,
            "expected one alternate Canopy configuration"
        );
        ensure!(
            alternate[0].parameters()
                != split
                    .iter()
                    .find(|r| r.systematic_h.is_some())
                    .unwrap()
                    .parameters(),
            "alternate must use a different Canopy geometry or join mode"
        );
        split.extend(alternate);
    }
    let encoder = BoltMaxEncoder::new((1 << n) / width, width, seed)?;
    let inputs = input_data(n, eta, seed);
    let mut rows = Vec::new();
    let mut stages = Vec::new();
    let order = if paper_only {
        let shared_merkle = shared
            .iter()
            .position(|r| r.systematic_h.is_none())
            .unwrap();
        let split_merkle = split.iter().position(|r| r.systematic_h.is_none()).unwrap();
        let canopy = split.iter().position(|r| r.systematic_h.is_some()).unwrap() + 2;
        let baseline = if shared_merkle_baseline
            || shared[shared_merkle].total_bytes <= split[split_merkle].total_bytes
        {
            shared_merkle
        } else {
            split_merkle + 2
        };
        let mut choices = vec![baseline, canopy];
        choices.extend((2..split.len()).map(|i| i + 2));
        choices
    } else {
        (0..4).collect::<Vec<_>>()
    };
    for iteration in 0..warmups + repetitions {
        for j in 0..order.len() {
            let index = order[(j + iteration) % order.len()];
            let scheme = if index < 2 {
                shared[index].scheme.as_str()
            } else {
                split[index - 2].scheme.as_str()
            };
            eprintln!("Bolt split comparison n={n} eta={eta} iteration={iteration}: {scheme}");
            let mut run = if index < 2 {
                let c = &shared[index];
                multi::prove(
                    &encoder,
                    &inputs,
                    c.systematic_h.zip(c.parity_h),
                    seed,
                    c.layout,
                    c.aligned_log.unwrap(),
                )?
            } else {
                prove(&encoder, &inputs, split[index - 2].parameters(), seed)?
            };
            let start = Instant::now();
            if index < 2 {
                let c = &shared[index];
                multi::verify(
                    &encoder,
                    n,
                    c.systematic_h.zip(c.parity_h),
                    seed,
                    &run.evaluations,
                    &run.proof,
                    c.layout,
                    c.aligned_log.unwrap(),
                )?;
                run.row.scheme = run.row.scheme.replace("_batched", "_shared");
            } else {
                verify(
                    &encoder,
                    n,
                    split[index - 2].parameters(),
                    seed,
                    &run.evaluations,
                    &run.proof,
                )?;
                // Preserve experiment labels when two configurations differ
                // only in their join round, so their timings remain separate.
                run.row.scheme = split[index - 2].scheme.clone();
            }
            run.row.verification_seconds = start.elapsed().as_secs_f64();
            run.row.proof_verified = true;
            if iteration < warmups {
                continue;
            }
            run.row.iteration = iteration - warmups;
            for p in run.profile {
                stages.push(StageRow {
                    input_oracles: eta,
                    canopy_layout: run.row.canopy_layout,
                    log_input: n,
                    interleaving: width,
                    scheme: run.row.scheme.clone(),
                    systematic_h: run.row.systematic_h,
                    parity_h: run.row.parity_h,
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

#[cfg(feature = "fs-accounting")]
pub fn validate(input: &Path, output: &Path) -> Result<()> {
    use crate::fiat_shamir::capture;
    #[derive(Serialize)]
    struct Validation {
        log_input: usize,
        input_oracles: usize,
        scheme: String,
        proof_verified: bool,
        trace_matches_model: bool,
        finalizations: usize,
        absorb_bytes: usize,
        squeeze_bytes: usize,
        narg_bytes: usize,
        hint_bytes: usize,
    }
    let rows = csv::Reader::from_path(input)?
        .deserialize::<Row>()
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let mut checks = Vec::new();
    for r in rows
        .iter()
        .filter(|r| r.log_input == 18 || (r.log_input == 22 && [1, 8].contains(&r.input_oracles)))
    {
        let n = r.log_input;
        let eta = r.input_oracles;
        eprintln!("Split FS validation n={n}, eta={eta}, {}", r.scheme);
        let encoder = BoltMaxEncoder::new((1 << n) / r.interleaving, r.interleaving, DEFAULT_SEED)?;
        let inputs = input_data(n, eta, DEFAULT_SEED);
        let run = prove(&encoder, &inputs, r.parameters(), DEFAULT_SEED)?;
        let (verified, traces) = capture(|| {
            verify(
                &encoder,
                n,
                r.parameters(),
                DEFAULT_SEED,
                &run.evaluations,
                &run.proof,
            )
        });
        verified?;
        ensure!(traces.len() == 1, "expected one transcript with two tails");
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
            ensure!(
                trace.events[i - 1].operation == 's' && trace.events[i - 1].bytes == 1,
                "missing outer queries"
            );
            words.push(e.count);
            queries.push(trace.events[i - 1].count);
        }
        let modeled = fs_model(r, &words, &queries)?;
        if modeled.normalized() != trace.normalized() {
            write_csv(output.with_extension("actual.csv"), &trace.events)?;
            write_csv(output.with_extension("expected.csv"), &modeled.events)?;
            anyhow::bail!("split FS trace mismatch");
        }
        let c = trace.counts();
        ensure!(
            c.absorb_bytes == 96 + run.proof.narg_string.len(),
            "FS absorbed bytes do not reconcile"
        );
        checks.push(Validation {
            log_input: n,
            input_oracles: eta,
            scheme: r.scheme.clone(),
            proof_verified: true,
            trace_matches_model: true,
            finalizations: c.finalizations,
            absorb_bytes: c.absorb_bytes,
            squeeze_bytes: c.squeeze_bytes,
            narg_bytes: run.proof.narg_string.len(),
            hint_bytes: run.proof.hints.len(),
        });
        write_csv(output, &checks)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parallel_arithmetic_preserves_batched_proof_transcript() {
        let n = 16;
        let k = 1 << (n - 2);
        let seed = 71;
        let encoder = BoltMaxEncoder::new(k, 4, seed).unwrap();
        let data = input_data(n, 2, seed);
        let p = Parameters {
            canopy_log_divisor: 2,
            delayed_systematic: false,
            systematic_h: Some(3),
            parity_h: Some(3),
            systematic_target_log: n - 5,
            parity_target_log: n - 7,
        };
        let run = |threads| {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap()
                .install(|| {
                    let result = prove(&encoder, &data, p, seed).unwrap();
                    verify(&encoder, n, p, seed, &result.evaluations, &result.proof).unwrap();
                    result
                })
        };
        let serial = run(1);
        let parallel = run(4);
        assert_eq!(serial.evaluations, parallel.evaluations);
        assert_eq!(serial.proof.narg_string, parallel.proof.narg_string);
        assert_eq!(serial.proof.hints, parallel.proof.hints);
    }
    fn rejected(f: impl FnOnce() -> Result<()>) -> bool {
        // Native debug transcript shape checks panic after a challenge-changing
        // corruption; release verification reports an ordinary error.
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
            Err(_) => true,
            Ok(result) => result.is_err(),
        }
    }
    #[test]
    fn quarter_blocks_bind_geometry_and_claims() {
        let k = 512;
        let n = 11;
        let seed = 37;
        let encoder = BoltMaxEncoder::new(k, 4, seed).unwrap();
        let data = input_data(n, 1, seed);
        for (hx, hy) in [(3, 2), (9, 2), (3, 7), (9, 7)] {
            let p = Parameters {
                canopy_log_divisor: 2,
                delayed_systematic: false,
                systematic_h: Some(hx),
                parity_h: Some(hy),
                systematic_target_log: 6,
                parity_target_log: 5,
            };
            let run = prove(&encoder, &data, p, seed).unwrap();
            verify(&encoder, n, p, seed, &run.evaluations, &run.proof).unwrap();
            assert_eq!(run.row.canopy_layout, CanopyLayout::EqualBlocks);
            let wrong_length = Parameters {
                canopy_log_divisor: 1,
                ..p
            };
            assert!(rejected(|| verify(
                &encoder,
                n,
                wrong_length,
                seed,
                &run.evaluations,
                &run.proof
            )));
            let mut wrong_value = run.evaluations.clone();
            wrong_value[0] += Fr::ONE;
            assert!(rejected(|| verify(
                &encoder,
                n,
                p,
                seed,
                &wrong_value,
                &run.proof
            )));
        }
    }
    #[test]
    fn independent_tails_bind_both_sides_inputs_and_geometry() {
        for width in [4usize, 16] {
            let n = 8 + width.ilog2() as usize;
            let k = (1 << n) / width;
            let seed = 42;
            let encoder = BoltMaxEncoder::new(k, width, seed).unwrap();
            for (eta, canopy_log_divisor) in [1, 2, 4, 8]
                .into_iter()
                .flat_map(|eta| [0, 1].map(|d| (eta, d)))
            {
                let data = input_data(n, eta, seed);
                for (hx, hy) in [
                    (None, None),
                    (Some(2), Some(4)),
                    (Some(8), Some(4)),
                    (Some(2), Some(6)),
                    (Some(8), Some(6)),
                ] {
                    let p = Parameters {
                        canopy_log_divisor,
                        delayed_systematic: false,
                        systematic_h: hx,
                        parity_h: hy,
                        systematic_target_log: 6,
                        parity_target_log: 2,
                    };
                    let r = prove(&encoder, &data, p, seed).unwrap();
                    verify(&encoder, n, p, seed, &r.evaluations, &r.proof).unwrap();
                    let wrong_length = Parameters {
                        canopy_log_divisor: 1 - canopy_log_divisor,
                        ..p
                    };
                    assert!(rejected(|| verify(
                        &encoder,
                        n,
                        wrong_length,
                        seed,
                        &r.evaluations,
                        &r.proof
                    )));
                    for i in 0..eta {
                        let mut bad = r.evaluations.clone();
                        bad[i] += Fr::ONE;
                        assert!(rejected(|| verify(&encoder, n, p, seed, &bad, &r.proof)));
                    }
                    // Both initial commitment roots must be bound even though the
                    // reductions are placed in different WHIR batches.
                    for offset in [0, 32] {
                        let mut bad = r.proof.clone();
                        bad.narg_string[offset] ^= 1;
                        assert!(rejected(|| verify(
                            &encoder,
                            n,
                            p,
                            seed,
                            &r.evaluations,
                            &bad
                        )));
                    }
                    let mut bad = r.proof.clone();
                    let i = bad.narg_string.len() - 1;
                    bad.narg_string[i] ^= 1;
                    assert!(rejected(|| verify(
                        &encoder,
                        n,
                        p,
                        seed,
                        &r.evaluations,
                        &bad
                    )));
                }
            }
            assert!(Parameters {
                canopy_log_divisor: 1,
                delayed_systematic: false,
                systematic_h: Some(3),
                parity_h: Some(4),
                systematic_target_log: 6,
                parity_target_log: 2
            }
            .configs(k, 1)
            .is_err());
            assert!(Parameters {
                canopy_log_divisor: 1,
                delayed_systematic: false,
                systematic_h: Some(2),
                parity_h: Some(1),
                systematic_target_log: 6,
                parity_target_log: 5
            }
            .configs(k, 1)
            .is_err());
        }
    }
    #[test]
    fn independent_tail_size_matches_serialized_proof() {
        for eta in [1, 2, 4, 8] {
            for canopy in [false, true] {
                let source_log = 5;
                let common = whir_config(1, 1, true).unwrap();
                let source = batching::initial_config(source_log, &common).unwrap();
                let mut configs = Vec::new();
                for _ in 0..eta {
                    configs.push(source.clone());
                    if canopy {
                        configs.push(common.clone());
                    }
                }
                let domain = DomainSeparator::protocol(&"split-tail-size-test").instance(&Empty);
                let mut state = ProverState::new(&domain, Blake3Transcript::default());
                let mut claims = Vec::new();
                for (i, cfg) in configs.iter().enumerate() {
                    let message = Buffer::from(
                        (0..cfg.initial_size())
                            .map(|j| crate::benchmark_field_element(73, i as u64, j as u64))
                            .collect::<Vec<_>>(),
                    );
                    let form = vec![Fr::ONE; message.len()];
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
                batching::prove_batch(claims, &common, &mut state, &mut Vec::new()).unwrap();
                let proof = state.proof();
                let model = side_size(
                    source_log,
                    &common,
                    canopy,
                    eta,
                    &mut ChaCha20Rng::seed_from_u64(42),
                )
                .unwrap()
                .bytes() as usize;
                let framing = 8 * (2 * eta + configs.len() + common.round_configs.len());
                assert_eq!(proof.narg_string.len() + proof.hints.len(), model + framing);
            }
        }
    }
}
