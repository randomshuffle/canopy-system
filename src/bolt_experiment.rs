//! Executable Bolt-max/Canopy composition over the paper prime field.
//!
//! This is an H-oracle-aided experiment: H is public, and its final evaluation
//! is checked directly. It does not implement Spark or Mulperm preprocessing.
//! WHIR supplies constrained RS proofs in place of the binary-field Ligerito
//! implementation. See docs/experiment_bolt_max.md for the exact protocol.

use std::{collections::BTreeMap, path::Path, time::Instant};

use anyhow::{ensure, Context, Result};
use ark_ff::{AdditiveGroup, Field, UniformRand};
use ark_poly::{EvaluationDomain, Radix2EvaluationDomain};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use whir_ref::{
    algebra::{
        embedding::Identity,
        linear_form::{Covector, Evaluate, LinearForm, MultilinearExtension},
    },
    bits::Bits,
    buffer::{Buffer, BufferOps},
    hash::{self, Hash},
    parameters::ProtocolParameters,
    protocols::{
        challenge_indices::challenge_indices,
        irs_commit,
        params::DecodingRegime,
        whir::{Commitment, Config, Witness},
    },
    transcript::{codecs::Empty, DomainSeparator, Proof, VerifierMessage},
};

use crate::{
    benchmark::sample_unique_indices,
    bolt::{self, BoltMaxEncoder},
    canopy::hash_bytes_to_field,
    executable_whir::register_field_ntt,
    merkle::{
        frontier_hash_count, hash_children, hash_field_leaf, reconstruct_roots, MerkleLevels,
    },
    Fr, FIELD_BYTES, HASH_BYTES,
};

pub const SECURITY: usize = 128;
pub const DEFAULT_SEED: u64 = 0x424f_4c54_4341_4e31;
/// Canopy RS lengths relative to the systematic row count k=N/width.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize, Serialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum CanopyLayout {
    #[default]
    QuarterEach,
    EqualBlocks,
    EqualHalf,
    EqualFull,
}

impl CanopyLayout {
    fn systematic_rs_length(self, k: usize) -> usize {
        match self {
            Self::QuarterEach | Self::EqualBlocks => k / 4,
            Self::EqualHalf => k / 2,
            Self::EqualFull => k,
        }
    }

    fn parity_rs_length(self, k: usize) -> usize {
        match self {
            Self::QuarterEach => k / 16,
            _ => self.systematic_rs_length(k),
        }
    }

    fn scheme(self) -> &'static str {
        match self {
            Self::QuarterEach => "bolt_max_canopy_quarter",
            Self::EqualBlocks => "bolt_max_canopy_equal_blocks",
            Self::EqualHalf => "bolt_max_canopy_equal_half",
            Self::EqualFull => "bolt_max_canopy_equal_full",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::QuarterEach => "Canopy RS block lengths: systematic k/4 and parity k/16; total 5k/16, one quarter of Bolt's encoded rows. Heights start at 3 for both parts.",
            Self::EqualBlocks => "Canopy RS block lengths: systematic k/4 and parity k/4; total k/2, 40% of Bolt's encoded rows. Heights start at 3 for systematic and 1 for parity.",
            Self::EqualHalf => "Canopy RS block lengths: systematic k/2 and parity k/2; total k, 80% of Bolt's encoded rows. Heights start at 2 for systematic and 0 for parity.",
            Self::EqualFull => "Canopy RS block lengths: systematic k and parity k; total 2k, 160% of Bolt's encoded rows. Heights start at 1 for systematic and 0 for parity.",
        }
    }
}

/// Smallest forest height giving a Canopy RS rate at most 1/2.
/// A codeword longer than its part's row count also permits height zero.
fn minimum_canopy_height(rows: usize, rs_length: usize) -> usize {
    (rows.ilog2() as usize + 1).saturating_sub(rs_length.ilog2() as usize)
}

fn canopy_rate_log(rows: usize, rs_length: usize, height: usize) -> usize {
    height + rs_length.ilog2() as usize - rows.ilog2() as usize
}

const FOLD: usize = 3;
type WhirConfig = Config<Identity<Fr>>;
type WhirWitness = Witness<Fr, Identity<Fr>>;
type Blake3Transcript = spongefish::instantiations::XOF<blake3::Hasher>;
type ProverState = whir_ref::transcript::ProverState<Blake3Transcript>;
type VerifierState<'a> = whir_ref::transcript::VerifierState<'a, Blake3Transcript>;

fn whir_config(log_message: usize, rate_log: usize, canopy: bool) -> Result<WhirConfig> {
    ensure!(
        log_message > 0 && rate_log > 0,
        "nonconstant WHIR message and rate required"
    );
    register_field_ntt();
    let f = FOLD.min(log_message);
    let config = WhirConfig::new(
        1 << log_message,
        &ProtocolParameters {
            decoding_regime: if canopy {
                DecodingRegime::Johnson
            } else {
                DecodingRegime::Unique
            },
            starting_log_inv_rate: rate_log,
            initial_folding_factor: f,
            folding_factor: f,
            security_level: SECURITY,
            pow_bits: 0,
            batch_size: 1,
            hash_id: hash::BLAKE3,
        },
    );
    ensure!(
        config.check_max_pow_bits(Bits::new(0.0)),
        "WHIR parameters require PoW"
    );
    ensure!(
        config.security_level(1, 3) >= SECURITY as f64,
        "insufficient WHIR RBR security"
    );
    Ok(config)
}

pub fn query_counts() -> (usize, usize) {
    (
        (SECURITY as f64 / -(1.0 - bolt::ASSUMED_DISTANCE / 3.0).log2()).ceil() as usize,
        (SECURITY as f64 / -0.75_f64.log2()).ceil() as usize,
    )
}

fn queries<T: VerifierMessage>(state: &mut T, rows: usize, count: usize) -> Vec<usize>
where
    u8: whir_ref::transcript::Decoding<[T::U]>,
{
    if count >= rows {
        return (0..rows).collect();
    }
    let mut set = std::collections::BTreeSet::new();
    while set.len() < count {
        set.insert(challenge_indices(state, rows, 1, false)[0]);
    }
    set.into_iter().collect()
}

fn eq_weights(point: &[Fr]) -> Vec<Fr> {
    let form = MultilinearExtension::new(point.to_vec());
    let mut v = vec![Fr::ZERO; 1 << point.len()];
    form.accumulate(&mut v, Fr::ONE);
    v
}

fn dot(a: &[Fr], b: &[Fr]) -> Fr {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).map(|(&a, &b)| a * b).sum()
}

fn sparse_claim(size: usize, indices: &[usize], values: &[Fr], gamma: Fr) -> (Covector<Fr>, Fr) {
    let mut weights = vec![Fr::ZERO; size];
    let mut value = Fr::ZERO;
    let mut power = Fr::ONE;
    for (&index, &v) in indices.iter().zip(values) {
        weights[index] += power;
        value += power * v;
        power *= gamma;
    }
    (Covector::new(weights), value)
}

/// The transpose RS map: sum_j gamma^j * omega^(index_j * i).
/// A single FFT avoids one dense power vector per queried parity row.
fn parity_claim(rows: usize, indices: &[usize], values: &[Fr], gamma: Fr) -> (Covector<Fr>, Fr) {
    let (mut covector, value) = sparse_claim(rows, indices, values, gamma);
    Radix2EvaluationDomain::<Fr>::new(rows)
        .unwrap()
        .fft_in_place(&mut covector.vector);
    covector.vector.truncate(rows / 2);
    (covector, value)
}

fn opening_claims(
    rows: usize,
    width: usize,
    height: usize,
    indices: &[usize],
    opened: &[Vec<Fr>],
    hashes: &[[u8; 32]],
) -> Result<(Vec<usize>, Vec<Fr>)> {
    let roots = reconstruct_roots(rows, width, height, indices, opened, hashes)?;
    Ok(roots
        .into_iter()
        .map(|(i, root)| (i, hash_bytes_to_field(&root)))
        .unzip())
}

fn constant_root(value: Fr, code_len: usize) -> [u8; 32] {
    let mut root = hash_field_leaf(&[value]);
    for _ in 0..code_len.ilog2() {
        root = hash_children(&root, &root);
    }
    root
}

struct Layer {
    width: usize,
    height: usize,
    tree: MerkleLevels,
    message: Option<Buffer<Fr>>,
    config: Option<WhirConfig>,
    irs: Option<irs_commit::Witness<Fr>>,
}

impl Layer {
    fn commit(
        state: &mut ProverState,
        values: &[Fr],
        width: usize,
        h: Option<usize>,
        rs_length: usize,
        profile: &mut Vec<StageTiming>,
        part: &'static str,
    ) -> Result<Self> {
        let rows = values.len() / width;
        let height = h.unwrap_or(rows.ilog2() as usize);
        let tree = measure(
            profile,
            "commitment",
            part,
            "row Merkle tree or forest",
            || MerkleLevels::commit_forest(values, width, height),
        )?;
        let (message, config, irs) = if let Some(h) = h {
            ensure!(
                (minimum_canopy_height(rows, rs_length)..=rows.ilog2() as usize).contains(&h),
                "invalid Canopy height"
            );
            let m = measure(profile, "commitment", part, "Canopy hash-to-field", || {
                tree.roots()
                    .iter()
                    .map(hash_bytes_to_field)
                    .collect::<Vec<_>>()
            });
            if m.len() == 1 {
                measure(
                    profile,
                    "commitment",
                    part,
                    "Canopy constant RS commitment",
                    || {
                        state.prover_message(&Hash(constant_root(m[0], rs_length)));
                    },
                );
                (Some(Buffer::from(m)), None, None)
            } else {
                let cfg = measure(
                    profile,
                    "commitment",
                    part,
                    "Canopy WHIR configuration",
                    || {
                        whir_config(
                            rows.ilog2() as usize - h,
                            canopy_rate_log(rows, rs_length, h),
                            true,
                        )
                    },
                )?;
                ensure!(
                    cfg.initial_committer.size() == rs_length,
                    "Canopy RS length does not match selected geometry"
                );
                let m = Buffer::from(m);
                let irs = measure(
                    profile,
                    "commitment",
                    part,
                    "Canopy RS encode and WHIR oracle commitment",
                    || cfg.initial_committer.commit(state, &[&m]),
                );
                (Some(m), Some(cfg), Some(irs))
            }
        } else {
            measure(
                profile,
                "commitment",
                part,
                "Merkle root transcript",
                || -> Result<()> {
                    state.prover_message(&Hash(tree.root()?));
                    Ok(())
                },
            )?;
            (None, None, None)
        };
        Ok(Self {
            width,
            height,
            tree,
            message,
            config,
            irs,
        })
    }

    fn open(
        &self,
        state: &mut ProverState,
        values: &[Fr],
        indices: &[usize],
        profile: &mut Vec<StageTiming>,
        part: &'static str,
    ) -> Result<Vec<Vec<Fr>>> {
        let opened = measure(
            profile,
            "opening",
            part,
            "copy and serialize queried row values",
            || {
                let opened = indices
                    .iter()
                    .map(|&i| values[i * self.width..(i + 1) * self.width].to_vec())
                    .collect::<Vec<_>>();
                for row in &opened {
                    state.prover_messages(row);
                }
                opened
            },
        );
        measure(
            profile,
            "opening",
            part,
            "collect and serialize row authentication paths",
            || -> Result<()> {
                let hashes = self.tree.open_hashes(indices)?;
                for &h in &hashes {
                    state.prover_message(&Hash(h));
                }
                Ok(())
            },
        )?;
        Ok(opened)
    }

    fn prove_canopy(
        mut self,
        state: &mut ProverState,
        indices: &[usize],
        profile: &mut Vec<StageTiming>,
        part: &'static str,
    ) -> Result<()> {
        let Some(message) = self.message.take() else {
            return Ok(());
        };
        // The prover already holds these H2F roots from commitment construction.
        // Only the verifier must reconstruct them by hashing the opened rows.
        let (ancestors, values) =
            measure(profile, "opening", part, "Canopy ancestor claims", || {
                let mut ancestors = indices.iter().map(|i| i >> self.height).collect::<Vec<_>>();
                ancestors.sort_unstable();
                ancestors.dedup();
                let values = ancestors
                    .iter()
                    .map(|&i| message.to_slice()[i])
                    .collect::<Vec<_>>();
                (ancestors, values)
            });
        if let Some(cfg) = self.config {
            // Bottom messages are already bound before this SEP batching challenge.
            let (form, value) = measure(
                profile,
                "opening",
                part,
                "Canopy SEP batching covector",
                || {
                    let gamma: Fr = state.verifier_message();
                    sparse_claim(message.len(), &ancestors, &values, gamma)
                },
            );
            let witness = measure(profile, "opening", part, "Canopy OOD evaluations", || {
                finish_ood(&cfg, state, &message, self.irs.take().unwrap())
            });
            measure(profile, "opening", part, "Canopy WHIR SEP proof", || {
                prove_whir(
                    &cfg,
                    state,
                    &message,
                    &witness,
                    vec![Box::new(form)],
                    &[value],
                )
            });
        } else {
            measure(profile, "opening", part, "Canopy constant opening", || {
                state.prover_message(&message.to_slice()[0]);
            });
        }
        Ok(())
    }
}

fn finish_ood(
    config: &WhirConfig,
    state: &mut ProverState,
    message: &Buffer<Fr>,
    irs: irs_commit::Witness<Fr>,
) -> WhirWitness {
    let points: Vec<Fr> = state.verifier_message_vec(config.initial_out_domain_samples);
    let evaluations = points
        .iter()
        .map(|&p| {
            whir_ref::algebra::linear_form::UnivariateEvaluation::new(p, message.len())
                .evaluate(&Identity::default(), message.to_slice())
        })
        .collect::<Vec<_>>();
    state.prover_messages(&evaluations);
    Witness {
        irs,
        out_of_domain: irs_commit::Evaluations {
            points,
            matrix: Buffer::from(evaluations),
            domain: None,
        },
    }
}

fn receive_ood(
    config: &WhirConfig,
    state: &mut VerifierState,
    irs: irs_commit::Commitment,
) -> Result<Commitment<Fr>> {
    let points = state.verifier_message_vec(config.initial_out_domain_samples);
    let values: Vec<Fr> = state
        .prover_messages_vec(config.initial_out_domain_samples)
        .map_err(|e| anyhow::anyhow!("OOD: {e:?}"))?;
    Ok(Commitment {
        irs,
        out_of_domain: irs_commit::Evaluations {
            points,
            matrix: Buffer::from(values),
            domain: None,
        },
    })
}

fn prove_whir(
    cfg: &WhirConfig,
    state: &mut ProverState,
    message: &Buffer<Fr>,
    witness: &WhirWitness,
    forms: Vec<Box<dyn LinearForm<Fr>>>,
    values: &[Fr],
) {
    let _ = cfg.prove(
        state,
        &[message],
        vec![witness],
        forms,
        Buffer::from(values),
    );
}

fn verify_whir(
    cfg: &WhirConfig,
    state: &mut VerifierState,
    commitment: &Commitment<Fr>,
    forms: &[Box<dyn LinearForm<Fr>>],
    values: &[Fr],
) -> Result<()> {
    cfg.verify(state, &[commitment], values)
        .map_err(|e| anyhow::anyhow!("WHIR: {e:?}"))?
        .verify(forms.iter().map(|f| f.as_ref()))
        .map_err(|e| anyhow::anyhow!("WHIR final claim: {e:?}"))?;
    Ok(())
}

struct ReceivedLayer {
    width: usize,
    rows: usize,
    height: Option<usize>,
    rs_length: usize,
    root: Hash,
    irs: Option<irs_commit::Commitment>,
}

impl ReceivedLayer {
    fn receive(
        state: &mut VerifierState,
        rows: usize,
        width: usize,
        height: Option<usize>,
        rs_length: usize,
    ) -> Result<Self> {
        if let Some(h) = height {
            ensure!(
                (minimum_canopy_height(rows, rs_length)..=rows.ilog2() as usize).contains(&h),
                "invalid received Canopy height"
            );
        }
        let (root, irs) = if let Some(h) = height.filter(|&h| h < rows.ilog2() as usize) {
            let cfg = whir_config(
                rows.ilog2() as usize - h,
                canopy_rate_log(rows, rs_length, h),
                true,
            )?;
            let irs = cfg
                .initial_committer
                .receive_commitment(state)
                .map_err(|e| anyhow::anyhow!("Canopy root: {e:?}"))?;
            (Hash::default(), Some(irs))
        } else {
            (
                state
                    .prover_message()
                    .map_err(|e| anyhow::anyhow!("root: {e:?}"))?,
                None,
            )
        };
        Ok(Self {
            width,
            rows,
            height,
            rs_length,
            root,
            irs,
        })
    }

    fn open(
        &self,
        state: &mut VerifierState,
        indices: &[usize],
    ) -> Result<(Vec<Vec<Fr>>, Vec<[u8; 32]>)> {
        let opened = indices
            .iter()
            .map(|_| {
                state
                    .prover_messages_vec(self.width)
                    .map_err(|e| anyhow::anyhow!("row: {e:?}"))
            })
            .collect::<Result<Vec<_>>>()?;
        let height = self.height.unwrap_or(self.rows.ilog2() as usize);
        let hash_count = frontier_hash_count(indices, self.rows, height)?;
        let hashes = (0..hash_count)
            .map(|_| {
                state
                    .prover_message::<Hash>()
                    .map(|h| h.0)
                    .map_err(|e| anyhow::anyhow!("hash: {e:?}"))
            })
            .collect::<Result<Vec<_>>>()?;
        if self.height.is_none() {
            let roots =
                reconstruct_roots(self.rows, self.width, height, indices, &opened, &hashes)?;
            ensure!(roots.get(&0) == Some(&self.root.0), "Merkle root mismatch");
        }
        Ok((opened, hashes))
    }

    fn verify_canopy(
        self,
        state: &mut VerifierState,
        indices: &[usize],
        opened: &[Vec<Fr>],
        hashes: &[[u8; 32]],
    ) -> Result<()> {
        let Some(h) = self.height else {
            return Ok(());
        };
        let (ancestors, values) =
            opening_claims(self.rows, self.width, h, indices, opened, hashes)?;
        let size = self.rows >> h;
        if let Some(irs) = self.irs {
            let cfg = whir_config(
                size.ilog2() as usize,
                canopy_rate_log(self.rows, self.rs_length, h),
                true,
            )?;
            let gamma = state.verifier_message();
            let (form, value) = sparse_claim(size, &ancestors, &values, gamma);
            let commitment = receive_ood(&cfg, state, irs)?;
            verify_whir(&cfg, state, &commitment, &[Box::new(form)], &[value])?;
        } else {
            let value: Fr = state
                .prover_message()
                .map_err(|e| anyhow::anyhow!("constant: {e:?}"))?;
            ensure!(
                values == vec![value] && ancestors == vec![0],
                "constant ancestor mismatch"
            );
            ensure!(
                constant_root(value, self.rs_length) == self.root.0,
                "constant RS root mismatch"
            );
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct TimingRow {
    pub canopy_layout: CanopyLayout,
    pub interleaving: usize,
    pub log_input: usize,
    pub setup_seed: u64,
    pub rayon_threads: usize,
    pub scheme: String,
    pub systematic_h: Option<usize>,
    pub parity_h: Option<usize>,
    pub iteration: usize,
    pub encoding_seconds: f64,
    pub commitment_seconds: f64,
    pub opening_seconds: f64,
    pub verification_seconds: f64,
    pub total_prover_seconds: f64,
    pub proof_bytes: usize,
    pub proof_verified: bool,
}

struct ProtocolRun {
    proof: Proof,
    row: TimingRow,
    evaluation: Fr,
    profile: Vec<StageTiming>,
}

/// Disjoint wall-clock stages. Parallel work is included in each call's elapsed
/// time; these are not sums of worker CPU times or overlapping tracing spans.
#[derive(Clone, Debug, Serialize)]
struct StageTiming {
    phase: &'static str,
    part: &'static str,
    stage: &'static str,
    seconds: f64,
}

fn measure<T>(
    profile: &mut Vec<StageTiming>,
    phase: &'static str,
    part: &'static str,
    stage: &'static str,
    operation: impl FnOnce() -> T,
) -> T {
    let start = Instant::now();
    let result = operation();
    profile.push(StageTiming {
        phase,
        part,
        stage,
        seconds: start.elapsed().as_secs_f64(),
    });
    result
}

#[derive(Clone, Debug, Serialize)]
struct StageRow {
    canopy_layout: CanopyLayout,
    log_input: usize,
    interleaving: usize,
    scheme: String,
    systematic_h: Option<usize>,
    parity_h: Option<usize>,
    setup_seed: u64,
    rayon_threads: usize,
    iteration: usize,
    phase: &'static str,
    part: &'static str,
    stage: &'static str,
    seconds: f64,
}

fn protocol_domain(
    log_input: usize,
    width: usize,
    heights: Option<(usize, usize)>,
    setup_seed: u64,
    layout: CanopyLayout,
) -> DomainSeparator<'static, ()> {
    // Preserve existing transcripts; bind each new Canopy geometry separately.
    let version = match (heights.is_some(), layout) {
        (true, CanopyLayout::EqualBlocks) => "canopy-system/bolt-max/v3/equal-blocks",
        (true, CanopyLayout::EqualHalf) => "canopy-system/bolt-max/v4/equal-half",
        (true, CanopyLayout::EqualFull) => "canopy-system/bolt-max/v4/equal-full",
        _ => "canopy-system/bolt-max/v2",
    };
    DomainSeparator::protocol(&(
        version, log_input, width, heights, setup_seed, SECURITY, FOLD,
    ))
}

fn prove_with_layout(
    encoder: &BoltMaxEncoder,
    input: &[Fr],
    heights: Option<(usize, usize)>,
    setup_seed: u64,
    layout: CanopyLayout,
) -> Result<ProtocolRun> {
    let n = input.len().ilog2() as usize;
    let k = encoder.message_rows;
    let width = encoder.interleaving;
    ensure!(input.len() == k * width, "wrong input shape");
    let mut profile = Vec::with_capacity(48);
    let start = Instant::now();
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
    let encoded = bolt::EncodedBolt { syndrome, parity };
    let encoding = start.elapsed();
    let domain = protocol_domain(n, width, heights, setup_seed, layout).instance(&Empty);
    let mut state = ProverState::new(&domain, Blake3Transcript::default());
    let start = Instant::now();
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
        &encoded.parity,
        width,
        heights.map(|h| h.1),
        layout.parity_rs_length(k),
        &mut profile,
        "Y",
    )?;
    let commit = start.elapsed();

    let start = Instant::now();
    // The benchmark's public evaluation point is fixed and bound by the protocol version.
    // Keep the evaluation weights alive through the original scope, so adding
    // profiling does not move a large buffer's deallocation into the timing.
    let (z, _z_weights, g, evaluation) = measure(
        &mut profile,
        "opening",
        "Bolt",
        "partial evaluation g",
        || {
            let z = vec![Fr::from(7u64); k.ilog2() as usize];
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
            let evaluation = dot(
                &g,
                &eq_weights(&vec![Fr::from(7u64); width.ilog2() as usize]),
            );
            state.prover_messages(&g);
            (z, z_weights, g, evaluation)
        },
    );
    let (r, x, u) = measure(
        &mut profile,
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
        &mut profile,
        "opening",
        "common WHIR",
        "WHIR configuration",
        || -> Result<_> {
            Ok((
                whir_config(k.ilog2() as usize, 1, false)?,
                whir_config(k.ilog2() as usize - 3, 1, false)?,
            ))
        },
    )?;
    let x_witness = measure(
        &mut profile,
        "opening",
        "x",
        "common WHIR RS encode and oracle commitment",
        || x_cfg.commit(&mut state, &[&x]),
    );
    let u_witness = measure(
        &mut profile,
        "opening",
        "u",
        "common WHIR RS encode and oracle commitment",
        || u_cfg.commit(&mut state, &[&u]),
    );
    let (s, s_weights, sigma) = measure(
        &mut profile,
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
    let (ix, iy) = measure(
        &mut profile,
        "opening",
        "Bolt",
        "sample row queries",
        || {
            let (qx, qy) = query_counts();
            let ix = queries(&mut state, k, qx);
            let iy = queries(&mut state, k / 4, qy);
            (ix, iy)
        },
    );
    let ox = x_layer.open(&mut state, input, &ix, &mut profile, "X")?;
    let oy = y_layer.open(&mut state, &encoded.parity, &iy, &mut profile, "Y")?;
    let (gamma_x, gamma_y, vx, vy) = measure(
        &mut profile,
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
        &mut profile,
        "opening",
        "Bolt",
        "systematic query covector",
        || sparse_claim(k, &ix, &vx, gamma_x),
    );
    let (u_spots, u_value) = measure(
        &mut profile,
        "opening",
        "Bolt",
        "parity query covector and transpose RS FFT",
        || parity_claim(k / 4, &iy, &vy, gamma_y),
    );
    let h_form = measure(
        &mut profile,
        "opening",
        "Bolt",
        "public H transpose covector",
        || encoder.transpose(&s_weights).map(Covector::new),
    )?;
    measure(
        &mut profile,
        "opening",
        "x",
        "common WHIR constrained proof",
        || {
            prove_whir(
                &x_cfg,
                &mut state,
                &x,
                &x_witness,
                vec![
                    Box::new(MultilinearExtension::new(z)),
                    Box::new(h_form),
                    Box::new(x_spots),
                ],
                &[dot(&g, &r), sigma, x_value],
            )
        },
    );
    measure(
        &mut profile,
        "opening",
        "u",
        "common WHIR constrained proof",
        || {
            prove_whir(
                &u_cfg,
                &mut state,
                &u,
                &u_witness,
                vec![Box::new(MultilinearExtension::new(s)), Box::new(u_spots)],
                &[sigma, u_value],
            )
        },
    );
    x_layer.prove_canopy(&mut state, &ix, &mut profile, "X")?;
    y_layer.prove_canopy(&mut state, &iy, &mut profile, "Y")?;
    let opening = start.elapsed();
    // Retain the established phase boundaries. The residual accounts for
    // allocation/drop, timer bookkeeping, and other work between timed calls.
    for (phase, total) in [
        ("commitment", (encoding + commit).as_secs_f64()),
        ("opening", opening.as_secs_f64()),
    ] {
        let accounted = profile
            .iter()
            .filter(|p| p.phase == phase)
            .map(|p| p.seconds)
            .sum::<f64>();
        ensure!(accounted <= total, "overlapping stage timers in {phase}");
        profile.push(StageTiming {
            phase,
            part: "other",
            stage: "unassigned phase work and timing overhead",
            seconds: total - accounted,
        });
    }
    let proof = state.proof();
    let row = TimingRow {
        canopy_layout: layout,
        interleaving: width,
        log_input: n,
        setup_seed,
        rayon_threads: rayon::current_num_threads(),
        scheme: if heights.is_some() {
            layout.scheme()
        } else {
            "bolt_max_merkle"
        }
        .into(),
        systematic_h: heights.map(|h| h.0),
        parity_h: heights.map(|h| h.1),
        iteration: 0,
        encoding_seconds: encoding.as_secs_f64(),
        commitment_seconds: (encoding + commit).as_secs_f64(),
        opening_seconds: opening.as_secs_f64(),
        verification_seconds: 0.0,
        total_prover_seconds: (encoding + commit + opening).as_secs_f64(),
        proof_bytes: proof.narg_string.len() + proof.hints.len(),
        proof_verified: false,
    };
    Ok(ProtocolRun {
        proof,
        row,
        evaluation,
        profile,
    })
}

fn verify_with_layout(
    encoder: &BoltMaxEncoder,
    log_input: usize,
    heights: Option<(usize, usize)>,
    setup_seed: u64,
    evaluation: Fr,
    proof: &Proof,
    layout: CanopyLayout,
) -> Result<()> {
    let k = encoder.message_rows;
    let width = encoder.interleaving;
    ensure!(
        log_input == k.ilog2() as usize + width.ilog2() as usize,
        "wrong public input shape"
    );
    let domain = protocol_domain(log_input, width, heights, setup_seed, layout).instance(&Empty);
    let mut state = VerifierState::new(&domain, proof, Blake3Transcript::default());
    let x_layer = ReceivedLayer::receive(
        &mut state,
        k,
        width,
        heights.map(|h| h.0),
        layout.systematic_rs_length(k),
    )?;
    let y_layer = ReceivedLayer::receive(
        &mut state,
        k / 4,
        width,
        heights.map(|h| h.1),
        layout.parity_rs_length(k),
    )?;
    let g: Vec<Fr> = state
        .prover_messages_vec(width)
        .map_err(|e| anyhow::anyhow!("g: {e:?}"))?;
    ensure!(
        dot(
            &g,
            &eq_weights(&vec![Fr::from(7u64); width.ilog2() as usize])
        ) == evaluation,
        "evaluation claim mismatch"
    );
    let r: Vec<Fr> = state.verifier_message_vec(width);
    let x_cfg = whir_config(k.ilog2() as usize, 1, false)?;
    let u_cfg = whir_config(k.ilog2() as usize - 3, 1, false)?;
    let xc = x_cfg
        .receive_commitment(&mut state)
        .map_err(|e| anyhow::anyhow!("x commitment: {e:?}"))?;
    let uc = u_cfg
        .receive_commitment(&mut state)
        .map_err(|e| anyhow::anyhow!("u commitment: {e:?}"))?;
    let s: Vec<Fr> = state.verifier_message_vec(k.ilog2() as usize - 3);
    let s_weights = eq_weights(&s);
    let sigma: Fr = state
        .prover_message()
        .map_err(|e| anyhow::anyhow!("sigma: {e:?}"))?;
    let (qx, qy) = query_counts();
    let ix = queries(&mut state, k, qx);
    let iy = queries(&mut state, k / 4, qy);
    let (ox, hx) = x_layer.open(&mut state, &ix)?;
    let (oy, hy) = y_layer.open(&mut state, &iy)?;
    let gamma_x = state.verifier_message();
    let gamma_y = state.verifier_message();
    let vx = ox.iter().map(|v| dot(v, &r)).collect::<Vec<_>>();
    let vy = oy.iter().map(|v| dot(v, &r)).collect::<Vec<_>>();
    let (x_spots, x_value) = sparse_claim(k, &ix, &vx, gamma_x);
    let (u_spots, u_value) = parity_claim(k / 4, &iy, &vy, gamma_y);
    let h_form = Covector::new(encoder.transpose(&s_weights)?);
    verify_whir(
        &x_cfg,
        &mut state,
        &xc,
        &[
            Box::new(MultilinearExtension::new(vec![
                Fr::from(7u64);
                k.ilog2() as usize
            ])),
            Box::new(h_form),
            Box::new(x_spots),
        ],
        &[dot(&g, &r), sigma, x_value],
    )?;
    verify_whir(
        &u_cfg,
        &mut state,
        &uc,
        &[Box::new(MultilinearExtension::new(s)), Box::new(u_spots)],
        &[sigma, u_value],
    )?;
    x_layer.verify_canopy(&mut state, &ix, &ox, &hx)?;
    y_layer.verify_canopy(&mut state, &iy, &oy, &hy)?;
    state
        .check_eof()
        .map_err(|e| anyhow::anyhow!("trailing proof data: {e:?}"))?;
    Ok(())
}

#[derive(Default, Clone, Copy)]
struct Size {
    hashes: f64,
    fields: f64,
}
impl Size {
    fn bytes(self) -> f64 {
        self.hashes * HASH_BYTES as f64 + self.fields * FIELD_BYTES as f64
    }
    fn add(&mut self, other: Self) {
        self.hashes += other.hashes;
        self.fields += other.fields;
    }
}

/// Mirror the executable reference: two transmitted coefficients per sumcheck
/// round, complete terminal coefficients, and WITH-replacement WHIR queries.
/// The reference retains duplicate row values but merges authentication paths.
/// Serialization headers are recorded only in actual bytes.
fn whir_size(config: &WhirConfig, rng: &mut ChaCha20Rng) -> Result<Size> {
    let mut result = Size {
        hashes: 1.0,
        fields: (2 * config.initial_sumcheck.num_rounds() + config.initial_out_domain_samples)
            as f64,
    };
    let mut previous = &config.initial_committer;
    for round in &config.round_configs {
        result.add(oracle_size(previous, rng)?);
        result.hashes += 1.0;
        result.fields += (round.out_domain_samples + 2 * round.sumcheck.num_rounds()) as f64;
        previous = &round.irs_committer;
    }
    result.add(oracle_size(previous, rng)?);
    result.fields +=
        (config.final_sumcheck.initial_size() + 2 * config.final_sumcheck.num_rounds()) as f64;
    Ok(result)
}

fn oracle_size(config: &irs_commit::Config<Identity<Fr>>, rng: &mut ChaCha20Rng) -> Result<Size> {
    let rows = config.codeword_length();
    let mut queries = (0..config.in_domain_samples())
        .map(|_| rng.gen_range(0..rows))
        .collect::<Vec<_>>();
    queries.sort_unstable();
    queries.dedup();
    let transmitted = if config.deduplicate_in_domain() {
        queries.len()
    } else {
        config.in_domain_samples()
    };
    Ok(Size {
        hashes: frontier_hash_count(&queries, rows, rows.ilog2() as usize)? as f64,
        fields: (transmitted * config.interleaving_depth()) as f64,
    })
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HeightRow {
    pub log_input: usize,
    pub part: String,
    pub bolt_rows: usize,
    pub canopy_rs_length: usize,
    pub h: usize,
    pub canopy_dimension: usize,
    pub requested_queries: usize,
    pub distinct_queries: usize,
    pub bottom_hash_bytes: f64,
    pub canopy_proof_hash_bytes: f64,
    pub canopy_proof_field_bytes: f64,
    pub authentication_bytes: f64,
    pub selected: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SizeRow {
    #[serde(default)]
    pub canopy_layout: CanopyLayout,
    pub log_input: usize,
    pub seed: u64,
    pub input_elements: usize,
    pub interleaving: usize,
    pub bolt_rows: usize,
    pub canopy_rs_length: usize,
    pub systematic_h: usize,
    pub parity_h: usize,
    pub systematic_queries: usize,
    pub parity_queries: usize,
    pub opened_row_field_bytes: usize,
    pub common_iopp_bytes: f64,
    pub merkle_authentication_bytes: f64,
    pub canopy_authentication_bytes: f64,
    pub merkle_total_bytes: f64,
    pub canopy_total_bytes: f64,
    pub canopy_over_merkle: f64,
    pub target_rbr_bits: usize,
    pub assumed_ldpc_distance: f64,
    pub samples: usize,
}

#[derive(Clone, Debug, Serialize)]
struct BreakdownRow {
    log_input: usize,
    scheme: &'static str,
    component: String,
    hash_bytes: f64,
    field_bytes: f64,
    total_bytes: f64,
}

#[derive(Clone, Debug, Serialize)]
struct OracleBreakdownRow {
    log_input: usize,
    proof: &'static str,
    oracle: usize,
    message_elements: usize,
    codeword_elements: usize,
    merkle_leaves: usize,
    leaf_width: usize,
    requested_queries: usize,
    transmitted_rows: f64,
    mean_distinct_rows: f64,
    mean_authentication_hash_bytes: f64,
    row_field_bytes: f64,
}

/// Seven disjoint WHIR components, in bytes. The initial root is separated
/// from subsequent roots so it can be counted exactly once in the comparison.
const DETAIL_NAMES: [&str; 7] = [
    "initial commitment root",
    "recursive commitment roots",
    "authentication paths",
    "queried row values",
    "sumcheck messages",
    "OOD evaluations",
    "terminal coefficients",
];

fn detailed_whir_size(
    config: &WhirConfig,
    rng: &mut ChaCha20Rng,
    log_input: usize,
    proof: &'static str,
) -> Result<([f64; 7], Vec<OracleBreakdownRow>)> {
    let mut amounts = [0.0; 7];
    amounts[0] = HASH_BYTES as f64;
    amounts[1] = (config.round_configs.len() * HASH_BYTES) as f64;
    amounts[4] = (2
        * FIELD_BYTES
        * (config.initial_sumcheck.num_rounds()
            + config
                .round_configs
                .iter()
                .map(|r| r.sumcheck.num_rounds())
                .sum::<usize>()
            + config.final_sumcheck.num_rounds())) as f64;
    amounts[5] = (FIELD_BYTES
        * (config.initial_out_domain_samples
            + config
                .round_configs
                .iter()
                .map(|r| r.out_domain_samples)
                .sum::<usize>())) as f64;
    amounts[6] = (FIELD_BYTES * config.final_sumcheck.initial_size()) as f64;
    let oracles = std::iter::once(&config.initial_committer)
        .chain(config.round_configs.iter().map(|r| &r.irs_committer));
    let mut rows = Vec::new();
    for (oracle, cfg) in oracles.enumerate() {
        let leaves = cfg.codeword_length();
        let mut queries = (0..cfg.in_domain_samples())
            .map(|_| rng.gen_range(0..leaves))
            .collect::<Vec<_>>();
        queries.sort_unstable();
        queries.dedup();
        let transmitted = if cfg.deduplicate_in_domain() {
            queries.len()
        } else {
            cfg.in_domain_samples()
        };
        let hashes =
            (frontier_hash_count(&queries, leaves, leaves.ilog2() as usize)? * HASH_BYTES) as f64;
        let fields = (transmitted * cfg.interleaving_depth() * FIELD_BYTES) as f64;
        amounts[2] += hashes;
        amounts[3] += fields;
        rows.push(OracleBreakdownRow {
            log_input,
            proof,
            oracle,
            message_elements: cfg.vector_size(),
            codeword_elements: cfg.size(),
            merkle_leaves: leaves,
            leaf_width: cfg.interleaving_depth(),
            requested_queries: cfg.in_domain_samples(),
            transmitted_rows: transmitted as f64,
            mean_distinct_rows: queries.len() as f64,
            mean_authentication_hash_bytes: hashes,
            row_field_bytes: fields,
        });
    }
    Ok((amounts, rows))
}

fn mean_whir_details(
    config: &WhirConfig,
    rngs: &mut [ChaCha20Rng],
    n: usize,
    proof: &'static str,
) -> Result<([f64; 7], Vec<OracleBreakdownRow>)> {
    let sample_count = rngs.len() as f64;
    let mut mean = [0.0; 7];
    let mut oracles: Vec<OracleBreakdownRow> = Vec::new();
    for rng in rngs.iter_mut() {
        let (amounts, sample) = detailed_whir_size(config, rng, n, proof)?;
        for (acc, value) in mean.iter_mut().zip(amounts) {
            *acc += value / sample_count;
        }
        if oracles.is_empty() {
            oracles = sample;
        } else {
            for (acc, value) in oracles.iter_mut().zip(sample) {
                acc.mean_distinct_rows += value.mean_distinct_rows;
                acc.mean_authentication_hash_bytes += value.mean_authentication_hash_bytes;
                acc.transmitted_rows += value.transmitted_rows;
                acc.row_field_bytes += value.row_field_bytes;
            }
        }
    }
    for row in &mut oracles {
        row.mean_distinct_rows /= rngs.len() as f64;
        row.mean_authentication_hash_bytes /= rngs.len() as f64;
        row.transmitted_rows /= rngs.len() as f64;
        row.row_field_bytes /= rngs.len() as f64;
    }
    Ok((mean, oracles))
}

/// Explain an existing size report using exactly its parameters and sample seeds.
/// Does not change height selection or overwrite the original experiment data.
pub fn write_proof_breakdown(output: &Path) -> Result<()> {
    let selected = csv::Reader::from_path(output.join("proof_sizes.csv"))?
        .deserialize::<SizeRow>()
        .collect::<std::result::Result<Vec<_>, _>>()?;
    ensure!(!selected.is_empty(), "proof_sizes.csv is empty");
    let mut breakdown = Vec::<BreakdownRow>::new();
    let mut oracle_rows = Vec::<OracleBreakdownRow>::new();
    for selected in &selected {
        let n = selected.log_input;
        let canopy_scheme = selected.canopy_layout.scheme();
        let seed = selected.seed;
        let samples = selected.samples;
        ensure!(
            samples > 0 && (13..=30).contains(&n),
            "invalid saved experiment parameters"
        );
        let width = selected.interleaving;
        bolt::validate_interleaving(width)?;
        let log_k = n - width.ilog2() as usize;
        let k = 1usize << log_k;
        let mut entries = Vec::<(&'static str, String, f64, f64)>::new();
        for scheme in ["bolt_max_merkle", canopy_scheme] {
            for (part, q) in [
                ("systematic", selected.systematic_queries),
                ("parity", selected.parity_queries),
            ] {
                entries.push((
                    scheme,
                    format!("Bolt {part} opened row values"),
                    0.0,
                    (q * width * FIELD_BYTES) as f64,
                ));
            }
            entries.push((
                scheme,
                "Bolt partial evaluation g".into(),
                0.0,
                (width * FIELD_BYTES) as f64,
            ));
            entries.push((
                scheme,
                "Bolt syndrome evaluation sigma".into(),
                0.0,
                FIELD_BYTES as f64,
            ));
        }
        let mut common_rngs = (0..samples)
            .map(|s| ChaCha20Rng::seed_from_u64(seed ^ n as u64 ^ s as u64))
            .collect::<Vec<_>>();
        for (proof, log_message) in [
            ("common message WHIR", log_k),
            ("common syndrome WHIR", log_k - 3),
        ] {
            let cfg = whir_config(log_message, 1, false)?;
            let (amounts, oracles) = mean_whir_details(&cfg, &mut common_rngs, n, proof)?;
            oracle_rows.extend(oracles);
            for scheme in ["bolt_max_merkle", canopy_scheme] {
                for (i, amount) in amounts.iter().copied().enumerate() {
                    entries.push((
                        scheme,
                        format!("{proof}: {}", DETAIL_NAMES[i]),
                        if i < 3 { amount } else { 0.0 },
                        if i >= 3 { amount } else { 0.0 },
                    ));
                }
            }
        }
        for (part, rows, q, h, rs_length, part_seed, proof) in [
            (
                "systematic",
                k,
                selected.systematic_queries,
                selected.systematic_h,
                selected.canopy_layout.systematic_rs_length(k),
                seed ^ n as u64,
                "Canopy systematic WHIR",
            ),
            (
                "parity",
                k / 4,
                selected.parity_queries,
                selected.parity_h,
                selected.canopy_layout.parity_rs_length(k),
                seed ^ n as u64 ^ 0x1000,
                "Canopy parity WHIR",
            ),
        ] {
            let mut merkle = 0.0;
            let mut bottom = 0.0;
            for s in 0..samples {
                let queries = sample_unique_indices(rows, q, part_seed.wrapping_add(s as u64))?;
                merkle += (frontier_hash_count(&queries, rows, rows.ilog2() as usize)? * HASH_BYTES)
                    as f64
                    / samples as f64;
                bottom +=
                    (frontier_hash_count(&queries, rows, h)? * HASH_BYTES) as f64 / samples as f64;
            }
            entries.push((
                "bolt_max_merkle",
                format!("Bolt {part} commitment root"),
                HASH_BYTES as f64,
                0.0,
            ));
            entries.push((
                "bolt_max_merkle",
                format!("Bolt {part} full authentication paths"),
                merkle,
                0.0,
            ));
            entries.push((
                canopy_scheme,
                format!("Bolt {part} bottom authentication paths"),
                bottom,
                0.0,
            ));
            let dimension = rows >> h;
            if dimension == 1 {
                entries.push((
                    canopy_scheme,
                    format!("Canopy {part} constant RS root"),
                    HASH_BYTES as f64,
                    0.0,
                ));
                entries.push((
                    canopy_scheme,
                    format!("Canopy {part} constant coefficient"),
                    0.0,
                    FIELD_BYTES as f64,
                ));
            } else {
                let cfg = whir_config(
                    dimension.ilog2() as usize,
                    canopy_rate_log(rows, rs_length, h),
                    true,
                )?;
                let mut rngs = (0..samples)
                    .map(|s| ChaCha20Rng::seed_from_u64(part_seed ^ s as u64 ^ (h as u64) << 32))
                    .collect::<Vec<_>>();
                let (amounts, oracles) = mean_whir_details(&cfg, &mut rngs, n, proof)?;
                oracle_rows.extend(oracles);
                for (i, amount) in amounts.into_iter().enumerate() {
                    entries.push((
                        canopy_scheme,
                        format!("{proof}: {}", DETAIL_NAMES[i]),
                        if i < 3 { amount } else { 0.0 },
                        if i >= 3 { amount } else { 0.0 },
                    ));
                }
            }
        }
        for (scheme, component, hash_bytes, field_bytes) in entries {
            breakdown.push(BreakdownRow {
                log_input: n,
                scheme,
                component,
                hash_bytes,
                field_bytes,
                total_bytes: hash_bytes + field_bytes,
            });
        }
        for (scheme, expected) in [
            ("bolt_max_merkle", selected.merkle_total_bytes),
            (canopy_scheme, selected.canopy_total_bytes),
        ] {
            let actual = breakdown
                .iter()
                .filter(|r| r.log_input == n && r.scheme == scheme)
                .map(|r| r.total_bytes)
                .sum::<f64>();
            ensure!(
                (actual - expected).abs() < 0.0001,
                "{scheme} 2^{n} components do not reconcile: {actual} != {expected}"
            );
        }
    }
    write_csv(output.join("proof_size_breakdown.csv"), &breakdown)?;
    write_csv(output.join("whir_oracle_breakdown.csv"), &oracle_rows)?;
    let mut report = String::from("# Bolt-max versus Canopy: proof-size breakdown\n\nThese are the saved 256-bit-prime/BLAKE3 experiment parameters, not the published binary-field Bolt benchmark. Each row is a mean over the original sample seeds. Sizes include initial commitment roots; exclude serializer framing and sparse-PCS preprocessing. The per-event 128-bit target is conditional on LDPC distance 0.013. Both schemes use the same common constrained-WHIR proofs.\n\n1 MiB = 1,048,576 bytes; 1 KiB = 1,024 bytes. Every component below reconciles to proof_sizes.csv.\n\n");
    for selected in &selected {
        let n = selected.log_input;
        report.push_str(selected.canopy_layout.description());
        report.push_str("\n\n");
        report.push_str(&format!("## Input 2^{n}: width={}, h_X={}, h_Y={}\n\n| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |\n|---|---:|---:|\n", selected.interleaving, selected.systematic_h, selected.parity_h));
        let mut by_component = BTreeMap::<String, [f64; 2]>::new();
        let mut hashes = [0.0; 2];
        let mut fields = [0.0; 2];
        for row in breakdown.iter().filter(|r| r.log_input == n) {
            let index = usize::from(row.scheme != "bolt_max_merkle");
            by_component.entry(row.component.clone()).or_default()[index] += row.total_bytes;
            hashes[index] += row.hash_bytes;
            fields[index] += row.field_bytes;
        }
        for (component, values) in by_component {
            if values.iter().any(|&v| v != 0.0) {
                report.push_str(&format!(
                    "| {component} | {:.5} | {:.5} |\n",
                    values[0] / 1024.0,
                    values[1] / 1024.0
                ));
            }
        }
        report.push_str(&format!("| **Total hashes** | **{:.5}** | **{:.5}** |\n| **Total field-value bytes** | **{:.5}** | **{:.5}** |\n| **Total proof (MiB)** | **{:.6}** | **{:.6}** |\n\n", hashes[0] / 1024.0, hashes[1] / 1024.0, fields[0] / 1024.0, fields[1] / 1024.0, selected.merkle_total_bytes / 1048576.0, selected.canopy_total_bytes / 1048576.0));
    }
    report.push_str("## Interpretation\n\nEach opened Bolt row contains the selected interleaving width in field elements, each 32 bytes. Canopy changes authentication, not these row values. The common message/syndrome WHIR proofs, g, and sigma also remain unchanged. At the full-height endpoint, a Canopy part retains the complete bottom tree and adds a 32-byte constant coefficient; its listed RS block is not transmitted. Nonconstant Canopy parts add an RS/SEP proof to replace the upper authentication paths. WHIR retains repeated queried row values but merges their Merkle paths; the per-oracle CSV makes that cost visible.\n");
    std::fs::write(output.join("proof_size_breakdown.md"), report)?;
    Ok(())
}

fn write_csv<T: Serialize>(path: impl AsRef<Path>, rows: &[T]) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

fn search_part(
    log_input: usize,
    rows: usize,
    count: usize,
    part: &str,
    rs_length: usize,
    samples: usize,
    seed: u64,
) -> Result<(usize, f64, f64, Vec<HeightRow>)> {
    let sampled = (0..samples)
        .map(|s| sample_unique_indices(rows, count.min(rows), seed.wrapping_add(s as u64)))
        .collect::<Result<Vec<_>>>()?;
    let baseline = HASH_BYTES as f64
        * (1.0
            + sampled
                .iter()
                .map(|q| frontier_hash_count(q, rows, rows.ilog2() as usize).unwrap() as f64)
                .sum::<f64>()
                / samples as f64);
    let mut grid = Vec::new();
    for h in minimum_canopy_height(rows, rs_length)..=rows.ilog2() as usize {
        let dimension = rows >> h;
        let cfg = if dimension > 1 {
            Some(whir_config(
                dimension.ilog2() as usize,
                canopy_rate_log(rows, rs_length, h),
                true,
            )?)
        } else {
            None
        };
        let mut bottom = 0.0;
        let mut size = Size::default();
        for (s, q) in sampled.iter().enumerate() {
            bottom += frontier_hash_count(q, rows, h)? as f64;
            size.add(if let Some(cfg) = &cfg {
                whir_size(
                    cfg,
                    &mut ChaCha20Rng::seed_from_u64(seed ^ s as u64 ^ (h as u64) << 32),
                )?
            } else {
                Size {
                    hashes: 1.0,
                    fields: 1.0,
                }
            });
        }
        bottom *= HASH_BYTES as f64 / samples as f64;
        size.hashes /= samples as f64;
        size.fields /= samples as f64;
        grid.push(HeightRow {
            log_input,
            part: part.into(),
            bolt_rows: rows,
            canopy_rs_length: rs_length,
            h,
            canopy_dimension: dimension,
            requested_queries: count,
            distinct_queries: count.min(rows),
            bottom_hash_bytes: bottom,
            canopy_proof_hash_bytes: size.hashes * HASH_BYTES as f64,
            canopy_proof_field_bytes: size.fields * FIELD_BYTES as f64,
            authentication_bytes: bottom + size.bytes(),
            selected: false,
        });
    }
    let best = (0..grid.len())
        .min_by(|&a, &b| {
            grid[a]
                .authentication_bytes
                .total_cmp(&grid[b].authentication_bytes)
        })
        .unwrap();
    grid[best].selected = true;
    Ok((
        grid[best].h,
        baseline,
        grid[best].authentication_bytes,
        grid,
    ))
}

pub fn estimate_with_layout(
    log_lengths: &[usize],
    width: usize,
    layout: CanopyLayout,
    samples: usize,
    seed: u64,
    output: &Path,
) -> Result<Vec<SizeRow>> {
    ensure!(
        samples > 0 && !log_lengths.is_empty(),
        "nonempty grid and positive samples required"
    );
    bolt::validate_interleaving(width)?;
    std::fs::create_dir_all(output)?;
    let (qx, qy) = query_counts();
    let mut rows = Vec::new();
    let mut grid = Vec::new();
    for &n in log_lengths {
        ensure!(
            (13..=30).contains(&n),
            "input log length must be in 13..=30"
        );
        let log_k = n - width.ilog2() as usize;
        let k = 1usize << log_k;
        let (hx, mx, cx, gx) = search_part(
            n,
            k,
            qx,
            "systematic",
            layout.systematic_rs_length(k),
            samples,
            seed ^ n as u64,
        )?;
        let (hy, my, cy, gy) = search_part(
            n,
            k / 4,
            qy,
            "parity",
            layout.parity_rs_length(k),
            samples,
            seed ^ n as u64 ^ 0x1000,
        )?;
        grid.extend(gx);
        grid.extend(gy);
        let x_cfg = whir_config(log_k, 1, false)?;
        let u_cfg = whir_config(log_k - 3, 1, false)?;
        let mut common = 0.0;
        for s in 0..samples {
            let mut rng = ChaCha20Rng::seed_from_u64(seed ^ n as u64 ^ s as u64);
            common += whir_size(&x_cfg, &mut rng)?.bytes() + whir_size(&u_cfg, &mut rng)?.bytes();
        }
        common = common / samples as f64 + ((width + 1) * FIELD_BYTES) as f64;
        let fields = (qx.min(k) + qy.min(k / 4)) * width * FIELD_BYTES;
        let merkle = fields as f64 + common + mx + my;
        let canopy = fields as f64 + common + cx + cy;
        rows.push(SizeRow {
            canopy_layout: layout,
            log_input: n,
            seed,
            input_elements: 1 << n,
            interleaving: width,
            bolt_rows: k + k / 4,
            canopy_rs_length: layout.systematic_rs_length(k) + layout.parity_rs_length(k),
            systematic_h: hx,
            parity_h: hy,
            systematic_queries: qx.min(k),
            parity_queries: qy.min(k / 4),
            opened_row_field_bytes: fields,
            common_iopp_bytes: common,
            merkle_authentication_bytes: mx + my,
            canopy_authentication_bytes: cx + cy,
            merkle_total_bytes: merkle,
            canopy_total_bytes: canopy,
            canopy_over_merkle: canopy / merkle,
            target_rbr_bits: SECURITY,
            assumed_ldpc_distance: bolt::ASSUMED_DISTANCE,
            samples,
        });
    }
    write_csv(output.join("proof_sizes.csv"), &rows)?;
    write_csv(output.join("height_search.csv"), &grid)?;
    let geometry = layout.description();
    let mut report = format!("# Bolt-max with Canopy\n\nPrime field: `{}`. BLAKE3-256 commitments, H2F, and Fiat-Shamir challenges; 128-bit per-event target, no PoW.\n\nDegree 8, compression 8, rate-1/2 syndrome RS, {width} field elements per row.\nThe LDPC distance 0.013 is an assumption for this prime-field port, not a proved consequence of the binary-field Bolt theorem.\n\nSystematic/parity parts have separate commitments in both schemes. {geometry} Heights count Merkle levels above whole rows.\n\nConstrained RS uses unique-decoding WHIR; Canopy uses Johnson WHIR. Three-variable folds (smaller for tiny messages), native terminal, independent proofs. Public H is evaluated directly; Spark/Mulperm is excluded. Full executable protocol and limits: [experiment notes](../../docs/experiment_bolt_max.md).\n\nQueries before full-read saturation: X={qx}, Y={qy}. External queries are sampled without replacement; when the budget exceeds the row count, read all rows. WHIR retains repeated row values from its native with-replacement queries; authentication paths are merged.\n\nSizes include both commitment roots, opened rows, g, sigma, both common WHIR proofs, and Canopy proofs. Estimates omit reference serialization framing; timings record actual serialized bytes. {samples} deterministic samples per height.\n\n| log2 input | h X / Y | Merkle MiB | Canopy MiB | ratio | Merkle auth KiB | Canopy auth KiB |\n|---:|:---:|---:|---:|---:|---:|---:|\n", crate::whir::FIELD_256.modulus);
    for row in &rows {
        report.push_str(&format!(
            "| {} | {} / {} | {:.3} | {:.3} | {:.5} | {:.2} | {:.2} |\n",
            row.log_input,
            row.systematic_h,
            row.parity_h,
            row.merkle_total_bytes / 1048576.0,
            row.canopy_total_bytes / 1048576.0,
            row.canopy_over_merkle,
            row.merkle_authentication_bytes / 1024.0,
            row.canopy_authentication_bytes / 1024.0
        ));
    }
    report.push_str(&format!("\nUpstream inspected: https://github.com/bcc-research/bolt-rs/tree/{} ; paper: https://eprint.iacr.org/2026/310 (2026-05-20 revision).\n", bolt::UPSTREAM_REVISION));
    std::fs::write(output.join("summary.md"), report)?;
    Ok(rows)
}

pub fn time_with_layout(
    log_input: usize,
    width: usize,
    layout: CanopyLayout,
    warmups: usize,
    repetitions: usize,
    samples: usize,
    seed: u64,
    output: &Path,
) -> Result<Vec<TimingRow>> {
    ensure!(repetitions > 0, "positive repetition count required");
    ensure!(
        (13..=30).contains(&log_input),
        "timing input log length must be in 13..=30"
    );
    let selected = estimate_with_layout(
        &[log_input],
        width,
        layout,
        samples,
        seed,
        &output.join("timing_height_search"),
    )?
    .remove(0);
    let encoder = BoltMaxEncoder::new((1 << log_input) / width, width, seed)?;
    let mut rng = ChaCha20Rng::seed_from_u64(seed ^ 0x4d53_47);
    let input = (0..1usize << log_input)
        .map(|_| Fr::rand(&mut rng))
        .collect::<Vec<_>>();
    let mut rows = Vec::new();
    let mut stages = Vec::new();
    for i in 0..warmups + repetitions {
        // Alternate execution order to avoid assigning all warm-cache runs to Canopy.
        let variants = if i % 2 == 0 {
            vec![None, Some((selected.systematic_h, selected.parity_h))]
        } else {
            vec![Some((selected.systematic_h, selected.parity_h)), None]
        };
        for heights in variants {
            let mut run = prove_with_layout(&encoder, &input, heights, seed, layout)?;
            let start = Instant::now();
            verify_with_layout(
                &encoder,
                log_input,
                heights,
                seed,
                run.evaluation,
                &run.proof,
                layout,
            )
            .context("Bolt experiment verifier")?;
            run.row.verification_seconds = start.elapsed().as_secs_f64();
            run.row.proof_verified = true;
            if i >= warmups {
                run.row.iteration = i - warmups;
                stages.extend(run.profile.iter().map(|p| StageRow {
                    canopy_layout: layout,
                    log_input,
                    interleaving: width,
                    scheme: run.row.scheme.clone(),
                    systematic_h: run.row.systematic_h,
                    parity_h: run.row.parity_h,
                    setup_seed: seed,
                    rayon_threads: run.row.rayon_threads,
                    iteration: run.row.iteration,
                    phase: p.phase,
                    part: p.part,
                    stage: p.stage,
                    seconds: p.seconds,
                }));
                rows.push(run.row);
            }
        }
    }
    write_csv(output.join("prover_timings.csv"), &rows)?;
    let geometry = layout.description();
    let mut report = format!("# Bolt-max prover timings\n\nInput 2^{log_input}, width {width}, {}-bit field, BLAKE3-256 commitments. {warmups} warmups, {repetitions} measured repetitions, {} Rayon threads. Every proof verified.\n\n{geometry}\n\nCPU-only, same encoder/hash code in both schemes. Public H setup and random input generation excluded. Commitment includes encoding. Opening includes g, projection, constrained WHIR proofs, row openings, and Canopy SEP proofs. Verification evaluates public H directly; no sparse-PCS preprocessing claim is made. The 128-bit per-event target is conditional on prime-field LDPC distance 0.013.\n\n| scheme | h X / Y | commit median s | opening median s | total prover median s | verify median s | actual proof MiB |\n|---|:---:|---:|---:|---:|---:|---:|\n", crate::FIELD_BITS, rayon::current_num_threads());
    let mut groups = BTreeMap::<String, Vec<&TimingRow>>::new();
    for row in &rows {
        groups.entry(row.scheme.clone()).or_default().push(row);
    }
    for (name, group) in groups {
        let med = |f: fn(&TimingRow) -> f64| {
            let mut v = group.iter().map(|r| f(r)).collect::<Vec<_>>();
            v.sort_by(f64::total_cmp);
            if v.len() % 2 == 1 {
                v[v.len() / 2]
            } else {
                (v[v.len() / 2 - 1] + v[v.len() / 2]) / 2.0
            }
        };
        report.push_str(&format!(
            "| {name} | {} / {} | {:.6} | {:.6} | {:.6} | {:.6} | {:.3} |\n",
            group[0].systematic_h.map_or("-".into(), |h| h.to_string()),
            group[0].parity_h.map_or("-".into(), |h| h.to_string()),
            med(|r| r.commitment_seconds),
            med(|r| r.opening_seconds),
            med(|r| r.total_prover_seconds),
            med(|r| r.verification_seconds),
            med(|r| r.proof_bytes as f64 / 1048576.0)
        ));
    }
    std::fs::write(output.join("prover_timings.md"), report)?;
    write_stage_profile(output, &rows, &stages, warmups)?;
    Ok(rows)
}

fn write_stage_profile(
    output: &Path,
    runs: &[TimingRow],
    stages: &[StageRow],
    warmups: usize,
) -> Result<()> {
    write_csv(output.join("prover_stage_timings.csv"), stages)?;
    let schemes = ["bolt_max_merkle", runs[0].canopy_layout.scheme()];
    let counts = schemes.map(|s| runs.iter().filter(|r| r.scheme == s).count());
    ensure!(
        counts[0] > 0 && counts[0] == counts[1],
        "unbalanced profile runs"
    );
    for run in runs {
        for (phase, expected) in [
            ("commitment", run.commitment_seconds),
            ("opening", run.opening_seconds),
        ] {
            let total = stages
                .iter()
                .filter(|p| {
                    p.scheme == run.scheme && p.iteration == run.iteration && p.phase == phase
                })
                .map(|p| p.seconds)
                .sum::<f64>();
            ensure!(
                (total - expected).abs() < 1e-9,
                "stage durations do not reconcile"
            );
        }
    }
    let mut report = format!("# Bolt-max prover stage profile\n\nInput 2^{}, width {}, 256-bit field, BLAKE3, {} Rayon threads. {} warmups and {} measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.\n\nAll component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.\n\nSetup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.\n", runs[0].log_input, runs[0].interleaving, runs[0].rayon_threads, warmups, counts[0]);
    report.push_str(runs[0].canopy_layout.description());
    report.push_str("\n");
    for phase in ["commitment", "opening"] {
        report.push_str(&format!("\n## {phase}\n\n| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |\n|---|---|---:|---:|---:|\n"));
        let mut keys = Vec::new();
        for p in stages.iter().filter(|p| p.phase == phase) {
            if !keys.contains(&(p.part, p.stage)) {
                keys.push((p.part, p.stage));
            }
        }
        let mut totals = [0.0; 2];
        for (part, stage) in keys {
            let means = std::array::from_fn::<_, 2, _>(|i| {
                stages
                    .iter()
                    .filter(|p| {
                        p.scheme == schemes[i]
                            && p.phase == phase
                            && p.part == part
                            && p.stage == stage
                    })
                    .map(|p| p.seconds * 1000.0)
                    .sum::<f64>()
                    / counts[i] as f64
            });
            for i in 0..2 {
                totals[i] += means[i];
            }
            report.push_str(&format!(
                "| {part} | {stage} | {:.6} | {:.6} | {:+.6} |\n",
                means[0],
                means[1],
                means[1] - means[0]
            ));
        }
        report.push_str(&format!(
            "| **Subtotal** | **{phase}** | **{:.6}** | **{:.6}** | **{:+.6}** |\n",
            totals[0],
            totals[1],
            totals[1] - totals[0]
        ));
    }
    report.push_str("\n## Total timings and variation\n\n| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for scheme in schemes {
        let group = runs
            .iter()
            .filter(|r| r.scheme == scheme)
            .collect::<Vec<_>>();
        let mut totals = group
            .iter()
            .map(|r| r.total_prover_seconds * 1000.0)
            .collect::<Vec<_>>();
        totals.sort_by(f64::total_cmp);
        let mean = totals.iter().sum::<f64>() / totals.len() as f64;
        let sd = if totals.len() > 1 {
            (totals.iter().map(|t| (t - mean).powi(2)).sum::<f64>() / (totals.len() - 1) as f64)
                .sqrt()
        } else {
            0.0
        };
        let median = (totals[(totals.len() - 1) / 2] + totals[totals.len() / 2]) / 2.0;
        let commit = group
            .iter()
            .map(|r| r.commitment_seconds * 1000.0)
            .sum::<f64>()
            / group.len() as f64;
        let opening = group
            .iter()
            .map(|r| r.opening_seconds * 1000.0)
            .sum::<f64>()
            / group.len() as f64;
        report.push_str(&format!("| {scheme} | {mean:.6} | {median:.6} | {:.6} | {:.6} | {sd:.6} | {commit:.6} | {opening:.6} | {} |\n", totals[0], totals[totals.len() - 1], group.len()));
    }
    std::fs::write(output.join("prover_stage_timings.md"), report)?;
    Ok(())
}

/// Compare widths at fixed input sizes, with a separate height search per part.
/// Keep each width's component reports and raw timings independently reviewable.
pub fn grid_with_layout(
    log_lengths: &[usize],
    widths: &[usize],
    layout: CanopyLayout,
    samples: usize,
    timing_log_input: usize,
    warmups: usize,
    repetitions: usize,
    seed: u64,
    output: &Path,
) -> Result<()> {
    ensure!(!widths.is_empty(), "nonempty interleaving grid required");
    for &width in widths {
        bolt::validate_interleaving(width)?;
    }
    ensure!(
        widths
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            == widths.len(),
        "duplicate interleaving widths"
    );
    let mut sizes = Vec::new();
    for &width in widths {
        println!("Estimating width {width} and optimizing both Canopy heights...");
        let dir = output.join(format!("width_{width}"));
        sizes.extend(estimate_with_layout(
            log_lengths,
            width,
            layout,
            samples,
            seed,
            &dir,
        )?);
        write_proof_breakdown(&dir)?;
    }
    sizes.sort_by_key(|r| (r.log_input, r.interleaving));
    write_csv(output.join("proof_sizes.csv"), &sizes)?;
    let geometry = layout.description();
    let mut report = format!(
        "# Bolt-max initial interleaving grid\n\nInitial widths: {widths:?}; input sizes are field-element counts. Subsequent WHIR folds remain factor 8 (three variables). Each width and input size gets an independent exhaustive Canopy height search for X and Y. Both schemes use the same width within each comparison.\n\n256-bit prime field, BLAKE3, Bolt-max degree 8 / compression 8, 128-bit per-event target conditional on prime-field LDPC distance 0.013. {geometry} Here k=N/width. Public-H constrained WHIR adaptation; Spark/Mulperm preprocessing excluded.\n\n{samples} deterministic size samples per height; sizes include commitment roots and omit serializer framing. Positive savings mean Canopy is smaller.\n\n| log2 input | width | h X / Y | Merkle MiB | Canopy MiB | saved MiB | saved % | auth saved % |\n|---:|---:|:---:|---:|---:|---:|---:|---:|\n"
    );
    for r in &sizes {
        report.push_str(&format!(
            "| {} | {} | {} / {} | {:.6} | {:.6} | {:.6} | {:.3} | {:.3} |\n",
            r.log_input,
            r.interleaving,
            r.systematic_h,
            r.parity_h,
            r.merkle_total_bytes / 1048576.0,
            r.canopy_total_bytes / 1048576.0,
            (r.merkle_total_bytes - r.canopy_total_bytes) / 1048576.0,
            100.0 * (1.0 - r.canopy_over_merkle),
            100.0 * (1.0 - r.canopy_authentication_bytes / r.merkle_authentication_bytes)
        ));
    }
    let relative = sizes
        .iter()
        .min_by(|a, b| a.canopy_over_merkle.total_cmp(&b.canopy_over_merkle))
        .unwrap();
    let absolute = sizes
        .iter()
        .max_by(|a, b| {
            (a.merkle_total_bytes - a.canopy_total_bytes)
                .total_cmp(&(b.merkle_total_bytes - b.canopy_total_bytes))
        })
        .unwrap();
    report.push_str(&format!(
        "\nLargest percentage saving: width {}, input 2^{}, {:.3}%. Largest byte saving: width {}, input 2^{}, {:.6} MiB.\n\n## Smallest proofs within each input size\n\n| log2 input | best Merkle width | Merkle MiB | best Canopy width | Canopy MiB | independently optimized saving % |\n|---:|---:|---:|---:|---:|---:|\n",
        relative.interleaving, relative.log_input, 100.0 * (1.0 - relative.canopy_over_merkle),
        absolute.interleaving, absolute.log_input, (absolute.merkle_total_bytes - absolute.canopy_total_bytes) / 1048576.0
    ));
    for n in sizes
        .iter()
        .map(|r| r.log_input)
        .collect::<std::collections::BTreeSet<_>>()
    {
        let m = sizes
            .iter()
            .filter(|r| r.log_input == n)
            .min_by(|a, b| a.merkle_total_bytes.total_cmp(&b.merkle_total_bytes))
            .unwrap();
        let c = sizes
            .iter()
            .filter(|r| r.log_input == n)
            .min_by(|a, b| a.canopy_total_bytes.total_cmp(&b.canopy_total_bytes))
            .unwrap();
        report.push_str(&format!(
            "| {n} | {} | {:.6} | {} | {:.6} | {:.3} |\n",
            m.interleaving,
            m.merkle_total_bytes / 1048576.0,
            c.interleaving,
            c.canopy_total_bytes / 1048576.0,
            100.0 * (1.0 - c.canopy_total_bytes / m.merkle_total_bytes)
        ));
    }
    // Save the completed size search before starting executable timings.
    std::fs::write(output.join("summary.md"), &report)?;
    let mut timings = Vec::new();
    for &width in widths {
        println!("Timing and verifying width {width} at input 2^{timing_log_input}...");
        timings.extend(time_with_layout(
            timing_log_input,
            width,
            layout,
            warmups,
            repetitions,
            samples,
            seed,
            &output.join(format!("width_{width}")),
        )?);
    }
    write_csv(output.join("prover_timings.csv"), &timings)?;
    report.push_str(&format!("\n## Executable timings at input 2^{timing_log_input}\n\n{warmups} warmups and {repetitions} measured repetitions per scheme and width, {} Rayon threads. Every proof verified; encoding included, setup and input generation excluded. This grid command times only input 2^{timing_log_input}; any additional timing runs are reported separately.\n\n| width | scheme | commitment median s | opening median s | total prover median s | verify median s | actual proof MiB |\n|---:|---|---:|---:|---:|---:|---:|\n", rayon::current_num_threads()));
    for &width in widths {
        for scheme in ["bolt_max_merkle", layout.scheme()] {
            let med = |f: fn(&TimingRow) -> f64| {
                let mut v = timings
                    .iter()
                    .filter(|r| r.interleaving == width && r.scheme == scheme)
                    .map(f)
                    .collect::<Vec<_>>();
                v.sort_by(f64::total_cmp);
                (v[(v.len() - 1) / 2] + v[v.len() / 2]) / 2.0
            };
            report.push_str(&format!(
                "| {width} | {scheme} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} |\n",
                med(|r| r.commitment_seconds),
                med(|r| r.opening_seconds),
                med(|r| r.total_prover_seconds),
                med(|r| r.verification_seconds),
                med(|r| r.proof_bytes as f64 / 1048576.0)
            ));
        }
    }
    report.push_str("\nEach width subdirectory contains all height candidates, detailed proof components, per-WHIR-oracle costs, and raw timings. The earlier width-128 experiment is preserved separately.\n");
    std::fs::write(output.join("summary.md"), report)?;
    Ok(())
}

/// Original quarter-per-part estimator; use `estimate_with_layout` for equal blocks.
pub fn estimate(
    log_lengths: &[usize],
    width: usize,
    samples: usize,
    seed: u64,
    output: &Path,
) -> Result<Vec<SizeRow>> {
    estimate_with_layout(
        log_lengths,
        width,
        CanopyLayout::QuarterEach,
        samples,
        seed,
        output,
    )
}

pub fn time(
    log_input: usize,
    width: usize,
    warmups: usize,
    repetitions: usize,
    samples: usize,
    seed: u64,
    output: &Path,
) -> Result<Vec<TimingRow>> {
    time_with_layout(
        log_input,
        width,
        CanopyLayout::QuarterEach,
        warmups,
        repetitions,
        samples,
        seed,
        output,
    )
}

pub fn grid(
    log_lengths: &[usize],
    widths: &[usize],
    samples: usize,
    timing_log_input: usize,
    warmups: usize,
    repetitions: usize,
    seed: u64,
    output: &Path,
) -> Result<()> {
    grid_with_layout(
        log_lengths,
        widths,
        CanopyLayout::QuarterEach,
        samples,
        timing_log_input,
        warmups,
        repetitions,
        seed,
        output,
    )
}

#[cfg(test)]
fn prove(
    encoder: &BoltMaxEncoder,
    input: &[Fr],
    heights: Option<(usize, usize)>,
    seed: u64,
) -> Result<ProtocolRun> {
    prove_with_layout(encoder, input, heights, seed, CanopyLayout::QuarterEach)
}

#[cfg(test)]
fn verify(
    encoder: &BoltMaxEncoder,
    log_input: usize,
    heights: Option<(usize, usize)>,
    seed: u64,
    evaluation: Fr,
    proof: &Proof,
) -> Result<()> {
    verify_with_layout(
        encoder,
        log_input,
        heights,
        seed,
        evaluation,
        proof,
        CanopyLayout::QuarterEach,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_block_geometries_and_complete_height_ranges() {
        let dir = tempfile::tempdir().unwrap();
        for (layout, divisor, min_x, min_y) in [
            (CanopyLayout::EqualBlocks, 4, 3, 1),
            (CanopyLayout::EqualHalf, 2, 2, 0),
            (CanopyLayout::EqualFull, 1, 1, 0),
        ] {
            for width in [4, 8, 16] {
                let rows =
                    estimate_with_layout(&[18, 22], width, layout, 2, 42, dir.path()).unwrap();
                let heights = csv::Reader::from_path(dir.path().join("height_search.csv"))
                    .unwrap()
                    .deserialize::<HeightRow>()
                    .collect::<std::result::Result<Vec<_>, _>>()
                    .unwrap();
                for r in rows {
                    let k = r.input_elements / width;
                    assert_eq!(r.canopy_layout, layout);
                    assert_eq!(r.canopy_rs_length, 2 * k / divisor);
                    for part in ["systematic", "parity"] {
                        let candidates = heights
                            .iter()
                            .filter(|h| h.log_input == r.log_input && h.part == part)
                            .collect::<Vec<_>>();
                        assert!(candidates.iter().all(|h| h.canopy_rs_length == k / divisor));
                        let min_h = if part == "systematic" { min_x } else { min_y };
                        let max_h = if part == "systematic" {
                            k.ilog2()
                        } else {
                            (k / 4).ilog2()
                        } as usize;
                        assert_eq!(
                            candidates.iter().map(|h| h.h).collect::<Vec<_>>(),
                            (min_h..=max_h).collect::<Vec<_>>()
                        );
                        assert_eq!(candidates.iter().filter(|h| h.selected).count(), 1);
                        let minimum = candidates
                            .iter()
                            .map(|h| h.authentication_bytes)
                            .min_by(f64::total_cmp)
                            .unwrap();
                        assert_eq!(
                            candidates
                                .iter()
                                .find(|h| h.selected)
                                .unwrap()
                                .authentication_bytes,
                            minimum
                        );
                    }
                }
                write_proof_breakdown(dir.path()).unwrap();
            }
        }
    }

    #[test]
    fn equal_blocks_verify_low_parity_heights_and_bind_layout() {
        let width = 4;
        let encoder = BoltMaxEncoder::new(256, width, 42).unwrap();
        let input = (0..256 * width)
            .map(|i| Fr::from((i * 9 + 7) as u64))
            .collect::<Vec<_>>();
        let log_input = input.len().ilog2() as usize;
        for heights in [Some((3, 1)), Some((3, 2)), Some((3, 3)), Some((8, 6))] {
            let run = prove_with_layout(&encoder, &input, heights, 42, CanopyLayout::EqualBlocks)
                .unwrap();
            verify_with_layout(
                &encoder,
                log_input,
                heights,
                42,
                run.evaluation,
                &run.proof,
                CanopyLayout::EqualBlocks,
            )
            .unwrap();
            assert!(verify_with_layout(
                &encoder,
                log_input,
                heights,
                42,
                run.evaluation,
                &run.proof,
                CanopyLayout::QuarterEach
            )
            .is_err());
            let mut corrupt = run.proof.clone();
            corrupt.narg_string[0] ^= 1;
            assert!(verify_with_layout(
                &encoder,
                log_input,
                heights,
                42,
                run.evaluation,
                &corrupt,
                CanopyLayout::EqualBlocks
            )
            .is_err());
        }
        let old = prove(&encoder, &input, None, 42).unwrap();
        let new = prove_with_layout(&encoder, &input, None, 42, CanopyLayout::EqualBlocks).unwrap();
        assert_eq!(old.proof.narg_string, new.proof.narg_string);
        assert_eq!(old.proof.hints, new.proof.hints);
    }

    #[test]
    fn larger_blocks_verify_zero_parity_height_and_bind_lengths() {
        let width = 4;
        let encoder = BoltMaxEncoder::new(256, width, 42).unwrap();
        let input = (0..256 * width)
            .map(|i| Fr::from((i * 9 + 7) as u64))
            .collect::<Vec<_>>();
        let log_input = input.len().ilog2() as usize;
        let baseline = prove(&encoder, &input, None, 42).unwrap();
        for (layout, min_x, length) in [
            (CanopyLayout::EqualHalf, 2, 128),
            (CanopyLayout::EqualFull, 1, 256),
        ] {
            assert_eq!(layout.systematic_rs_length(256), length);
            assert_eq!(layout.parity_rs_length(256), length);
            for heights in [Some((min_x, 0)), Some((3, 1)), Some((8, 6))] {
                let run = prove_with_layout(&encoder, &input, heights, 42, layout).unwrap();
                verify_with_layout(
                    &encoder,
                    log_input,
                    heights,
                    42,
                    run.evaluation,
                    &run.proof,
                    layout,
                )
                .unwrap();
                for wrong in [
                    CanopyLayout::EqualBlocks,
                    CanopyLayout::EqualHalf,
                    CanopyLayout::EqualFull,
                ] {
                    if wrong != layout {
                        assert!(verify_with_layout(
                            &encoder,
                            log_input,
                            heights,
                            42,
                            run.evaluation,
                            &run.proof,
                            wrong
                        )
                        .is_err());
                    }
                }
                let mut corrupt = run.proof.clone();
                corrupt.narg_string[0] ^= 1;
                assert!(verify_with_layout(
                    &encoder,
                    log_input,
                    heights,
                    42,
                    run.evaluation,
                    &corrupt,
                    layout
                )
                .is_err());
                // Mutate an opened row as well as the commitment. At h_Y=0,
                // the Canopy claim must bind the row leaf hash directly.
                // All 256 systematic rows are opened, with no sibling hashes.
                let row_start = 4 * HASH_BYTES + (width + 1 + 256 * width) * FIELD_BYTES;
                let mut corrupt = run.proof.clone();
                corrupt.narg_string[row_start] ^= 1;
                assert!(verify_with_layout(
                    &encoder,
                    log_input,
                    heights,
                    42,
                    run.evaluation,
                    &corrupt,
                    layout
                )
                .is_err());
            }
            assert!(prove_with_layout(&encoder, &input, Some((min_x - 1, 0)), 42, layout).is_err());
            let unchanged = prove_with_layout(&encoder, &input, None, 42, layout).unwrap();
            assert_eq!(baseline.proof.narg_string, unchanged.proof.narg_string);
            assert_eq!(baseline.proof.hints, unchanged.proof.hints);
        }
    }

    #[test]
    fn size_model_matches_reference_duplicate_row_serialization() {
        let cfg = whir_config(3, 1, false).unwrap();
        assert!(!cfg.initial_committer.deduplicate_in_domain());
        let domain = DomainSeparator::protocol(&cfg).instance(&Empty);
        let mut state = ProverState::new(&domain, Blake3Transcript::default());
        let message = Buffer::from((0..8).map(Fr::from).collect::<Vec<_>>());
        let witness = cfg.commit(&mut state, &[&message]);
        let form = Covector::new(vec![Fr::ONE; 8]);
        let value = message.to_slice().iter().copied().sum::<Fr>();
        prove_whir(
            &cfg,
            &mut state,
            &message,
            &witness,
            vec![Box::new(form)],
            &[value],
        );
        let proof = state.proof();
        let modeled = whir_size(&cfg, &mut ChaCha20Rng::seed_from_u64(42)).unwrap();
        // Two leaves are both visited by 309 draws; one serialized Vec length.
        assert_eq!(
            proof.narg_string.len() + proof.hints.len(),
            modeled.bytes() as usize + 8
        );
    }

    #[test]
    fn quarter_geometry_and_full_height_search() {
        let dir = tempfile::tempdir().unwrap();
        for width in [4, 8, 16, 128] {
            let rows = estimate(&[18, 22], width, 2, 42, dir.path()).unwrap();
            for row in rows {
                assert_eq!(row.canopy_rs_length * 4, row.bolt_rows);
                assert_eq!(row.interleaving, width);
                assert_eq!(row.bolt_rows, (row.input_elements / width) * 5 / 4);
                assert_eq!(
                    row.opened_row_field_bytes,
                    (row.systematic_queries + row.parity_queries) * width * FIELD_BYTES
                );
            }
            write_proof_breakdown(dir.path()).unwrap();
        }
        assert!(estimate(&[12], 16, 1, 42, dir.path()).is_err());
        assert!(estimate(&[18], 16, 0, 42, dir.path()).is_err());
        assert!(estimate(&[18], 3, 1, 42, dir.path()).is_err());
        assert!(BoltMaxEncoder::new(64, 3, 42).is_err());
    }

    #[test]
    fn round_trip_and_reject_tampering() {
        for width in [4, 8, 16, 128] {
            let encoder = BoltMaxEncoder::new(64, width, 42).unwrap();
            let input = (0..64 * width)
                .map(|i| Fr::from((i * i + 9) as u64))
                .collect::<Vec<_>>();
            let log_input = input.len().ilog2() as usize;
            for heights in [None, Some((3, 3)), Some((6, 4))] {
                let run = prove(&encoder, &input, heights, 42).unwrap();
                verify(&encoder, log_input, heights, 42, run.evaluation, &run.proof).unwrap();
                // Check the public evaluation independently of the row decomposition.
                assert_eq!(
                    run.evaluation,
                    dot(&input, &eq_weights(&vec![Fr::from(7u64); log_input]))
                );
                assert!(verify(
                    &encoder,
                    log_input,
                    heights,
                    42,
                    run.evaluation + Fr::ONE,
                    &run.proof
                )
                .is_err());
                assert!(verify(
                    &encoder,
                    log_input + 1,
                    heights,
                    42,
                    run.evaluation,
                    &run.proof
                )
                .is_err());
                let mut corrupt = run.proof.clone();
                corrupt.narg_string[0] ^= 1;
                assert!(
                    verify(&encoder, log_input, heights, 42, run.evaluation, &corrupt).is_err()
                );
            }
        }
    }

    #[test]
    fn recursive_canopy_checks_rows_and_original_rs_commitments() {
        let width = 16;
        let log_input = 15;
        let encoder = BoltMaxEncoder::new(2048, width, 44).unwrap();
        let input = (0..2048 * width)
            .map(|i| Fr::from((i * 3 + 9) as u64))
            .collect::<Vec<_>>();
        let heights = Some((3, 3));
        let run = prove(&encoder, &input, heights, 44).unwrap();
        verify(&encoder, log_input, heights, 44, run.evaluation, &run.proof).unwrap();
        // Two base roots, g, two unique-decoding WHIR roots, sigma, then rows.
        let row_start = 4 * HASH_BYTES + (width + 1) * FIELD_BYTES;
        let mut corrupt = run.proof.clone();
        corrupt.narg_string[row_start] ^= 1;
        assert!(verify(&encoder, log_input, heights, 44, run.evaluation, &corrupt).is_err());
        let mut corrupt = run.proof.clone();
        assert!(!corrupt.hints.is_empty());
        let last = corrupt.hints.len() - 1;
        corrupt.hints[last] ^= 1;
        assert!(verify(&encoder, log_input, heights, 44, run.evaluation, &corrupt).is_err());
    }
}
