//! Executable same-code batching reductions used by Section 8.
//!
//! `ifold` keeps the original RS code and commits its output with Canopy.
//! Its input openings produce deferred, Merkle-committed SEP claims; `iacc`
//! batches precisely those claims with the previous accumulators. No WHIR
//! folding or final decider is included in the measured reduction time.
use std::{
    collections::{BTreeMap, BTreeSet},
    time::Instant,
};

use anyhow::{ensure, Result};
use ark_ff::Field;
use ark_serialize::CanonicalSerialize;
use rayon::prelude::*;
use serde::Serialize;
use whir_ref::{
    algebra::{
        embedding::Identity,
        multilinear_extend,
        ntt::{self, PolynomialSegment, Polynomials},
    },
    buffer::{Buffer, BufferMath, BufferOps},
    hash::{self, Hash},
    protocols::{
        challenge_indices::challenge_indices,
        irs_commit::{self, IrsMode, IrsParams},
        matrix_commit, merkle_tree,
        params::DecodingRegime,
    },
    transcript::{
        codecs::Empty, DomainSeparator, Encoding, Proof, ProverState, VerifierMessage,
        VerifierState,
    },
};

use crate::{
    additive_rs, benchmark_field_element as elem,
    merkle::{authentication_work_by_height, frontier_hash_count},
    register_benchmark_ntt, Fr, FIELD_BYTES, HASH_BYTES,
};

#[cfg(not(feature = "fs-accounting"))]
type Sponge = spongefish::instantiations::XOF<blake3::Hasher>;
#[cfg(feature = "fs-accounting")]
type Sponge = crate::fiat_shamir::RecordingBlake3;
type P = ProverState<Sponge>;
type V<'a> = VerifierState<'a, Sponge>;
pub type CommitmentConfig = irs_commit::Config<Identity<Fr>>;
type Witness = irs_commit::Witness<Fr>;
type Commitment = irs_commit::Commitment;

#[derive(Clone, Debug, Default, Serialize)]
pub struct ReductionMeasurement {
    pub scheme: String,
    pub log_n: usize,
    pub sources: usize,
    pub height: Option<usize>,
    pub queries: usize,
    pub step_seconds: f64,
    /// Setup is reported separately and excluded from `step_seconds`.
    pub incoming_commit_seconds: f64,
    /// Commit only newly supplied oracles, excluding cached prior state.
    pub fresh_commit_sources: usize,
    pub fresh_commit_seconds: f64,
    /// Synthetic preparation of carried inputs; excluded from total prover cost.
    pub prior_commit_seconds: f64,
    pub fresh_encoding_seconds: f64,
    pub fresh_merkle_leaf_seconds: f64,
    pub fresh_merkle_parent_seconds: f64,
    pub fresh_h2f_seconds: f64,
    pub fresh_ood_seconds: f64,
    pub fresh_other_seconds: f64,
    pub form_seconds: f64,
    pub sumcheck_seconds: f64,
    pub combination_seconds: f64,
    pub output_commit_seconds: f64,
    pub opening_seconds: f64,
    /// Disjoint reduction phases; parent timing columns remain for compatibility.
    pub encoding_seconds: f64,
    pub merkle_leaf_seconds: f64,
    pub merkle_parent_seconds: f64,
    pub h2f_seconds: f64,
    pub canopy_ood_seconds: f64,
    pub output_ood_seconds: f64,
    pub output_commit_other_seconds: f64,
    pub message_combination_seconds: f64,
    /// Includes canonical leaf hashing when `codeword_leaf_fused` is true.
    pub codeword_combination_seconds: f64,
    pub codeword_leaf_fused: bool,
    pub sumcheck_setup_seconds: f64,
    pub sumcheck_polynomial_seconds: f64,
    pub sumcheck_fold_seconds: f64,
    pub sumcheck_transcript_seconds: f64,
    pub opening_auth_seconds: f64,
    pub opening_claim_seconds: f64,
    /// Final proof-buffer copy only; incremental transcript writes belong to their phases.
    pub serialization_seconds: f64,
    pub prover_other_seconds: f64,
    /// Input commitment phases are excluded from the reduction timer.
    pub incoming_encoding_seconds: f64,
    pub incoming_merkle_leaf_seconds: f64,
    pub incoming_merkle_parent_seconds: f64,
    pub incoming_h2f_seconds: f64,
    pub incoming_ood_seconds: f64,
    pub incoming_other_seconds: f64,
    /// Leaf hashes plus distinct parent hashes, excluding Fiat--Shamir.
    pub auth_hashes: usize,
    pub fs_finalizations: usize,
    /// Proof excluding newly returned commitment data.
    pub proof_bytes: usize,
    pub proof_hint_bytes: usize,
    pub proof_field_bytes: usize,
    pub proof_hash_bytes: usize,
    pub proof_framing_bytes: usize,
    /// The new commitment's serialized root and, for Canopy, its OOD value.
    /// Verifier-derived challenges and generated SEP descriptions need no wire bytes.
    pub output_instance_bytes: usize,
    pub serialized_total_bytes: usize,
    pub verified: bool,
}

#[derive(Default)]
struct CommitmentProfile {
    encoding: f64,
    leaf: f64,
    parent: f64,
    h2f: f64,
    ood: f64,
}
// Floating-point sums of nested timers can exceed their enclosing timer by
// a few ulps. Normalize only that rounding error; retain meaningful negatives
// so the profile validation still catches overlapping or incorrect boundaries.
fn phase_remainder(total: f64, accounted: f64) -> f64 {
    let remainder = total - accounted;
    let tolerance = 8.0 * f64::EPSILON * total.abs().max(accounted.abs());
    if remainder < 0.0 && remainder >= -tolerance {
        0.0
    } else {
        remainder
    }
}
impl CommitmentProfile {
    fn accounted(&self) -> f64 {
        self.encoding + self.leaf + self.parent + self.h2f + self.ood
    }
    fn merge(&mut self, other: &Self) {
        self.encoding += other.encoding;
        self.leaf += other.leaf;
        self.parent += other.parent;
        self.h2f += other.h2f;
        self.ood += other.ood;
    }
    fn set_fresh(&self, sources: usize, seconds: f64, times: &mut ReductionMeasurement) {
        times.fresh_commit_sources = sources;
        times.fresh_commit_seconds = seconds;
        times.fresh_encoding_seconds = self.encoding;
        times.fresh_merkle_leaf_seconds = self.leaf;
        times.fresh_merkle_parent_seconds = self.parent;
        times.fresh_h2f_seconds = self.h2f;
        times.fresh_ood_seconds = self.ood;
        times.fresh_other_seconds = phase_remainder(seconds, self.accounted());
    }
    fn set_incoming(&self, times: &mut ReductionMeasurement) {
        times.incoming_encoding_seconds = self.encoding;
        times.incoming_merkle_leaf_seconds = self.leaf;
        times.incoming_merkle_parent_seconds = self.parent;
        times.incoming_h2f_seconds = self.h2f;
        times.incoming_ood_seconds = self.ood;
        times.incoming_other_seconds =
            phase_remainder(times.incoming_commit_seconds, self.accounted());
    }
    fn set_output(&self, times: &mut ReductionMeasurement) {
        times.encoding_seconds = self.encoding;
        times.merkle_leaf_seconds = self.leaf;
        times.merkle_parent_seconds = self.parent;
        times.h2f_seconds = self.h2f;
        times.canopy_ood_seconds = self.ood;
        times.output_commit_other_seconds = phase_remainder(
            times.output_commit_seconds,
            self.accounted() + times.output_ood_seconds,
        );
    }
}
impl ReductionMeasurement {
    fn finish_phases(&mut self) {
        let sumcheck_other = phase_remainder(
            self.sumcheck_seconds,
            self.sumcheck_setup_seconds
                + self.sumcheck_polynomial_seconds
                + self.sumcheck_fold_seconds,
        );
        self.sumcheck_transcript_seconds = sumcheck_other;
        let accounted = self.form_seconds
            + self.sumcheck_seconds
            + self.combination_seconds
            + self.output_commit_seconds
            + self.opening_seconds
            + self.serialization_seconds;
        self.prover_other_seconds = phase_remainder(self.step_seconds, accounted);
    }
}

/// Split the native CPU matrix Merklize operation at its existing leaf/parent
/// boundary, using precisely the same hash_rows and build_nodes kernels.
fn merklize_profiled(
    matrix: &Buffer<Fr>,
    cfg: &CommitmentConfig,
    profile: &mut CommitmentProfile,
) -> (Buffer<Hash>, Hash) {
    merklize_prehashed_profiled(matrix, cfg, None, profile)
}
fn merklize_prehashed_profiled(
    matrix: &Buffer<Fr>,
    cfg: &CommitmentConfig,
    leaves: Option<Vec<Hash>>,
    profile: &mut CommitmentProfile,
) -> (Buffer<Hash>, Hash) {
    let mc = cfg.matrix_commit();
    assert_eq!(matrix.len(), mc.size());
    let leaves = leaves.unwrap_or_else(|| {
        let t = Instant::now();
        let engine = hash::ENGINES.retrieve(mc.leaf_hash_id).unwrap();
        let mut leaves = vec![Hash::default(); mc.num_rows()];
        matrix_commit::hash_rows(&*engine, matrix.to_slice(), &mut leaves);
        profile.leaf += t.elapsed().as_secs_f64();
        leaves
    });
    assert_eq!(leaves.len(), mc.num_rows());
    let t = Instant::now();
    let nodes = mc.merkle_tree.build_nodes(leaves);
    let root = nodes[mc.merkle_tree.num_nodes() - 1];
    let nodes = Buffer::from(nodes);
    profile.parent += t.elapsed().as_secs_f64();
    (nodes, root)
}

#[derive(Clone, Debug, Serialize)]
pub struct CanopyRun {
    pub ifold: ReductionMeasurement,
    pub iacc: ReductionMeasurement,
}

/// The same native IRS parameters are used for commitments and query budgets.
pub fn commitment_config(
    log_message: usize,
    log_inv_rate: usize,
    log_width: usize,
    security_bits: f64,
) -> Result<CommitmentConfig> {
    ensure!(
        log_message <= 30 && (1..=32).contains(&log_inv_rate),
        "invalid message length or RS rate"
    );
    ensure!(
        log_width <= log_message,
        "leaf width exceeds message length"
    );
    ensure!(
        log_message + log_inv_rate - log_width <= 32,
        "RS domain exceeds 2^32"
    );
    ensure!(
        security_bits.is_finite() && security_bits > 0.0,
        "invalid local security budget"
    );
    register_benchmark_ntt();
    Ok(CommitmentConfig::new(IrsParams {
        security_target: security_bits,
        decoding_regime: DecodingRegime::Johnson,
        hash_id: hash::BLAKE3,
        num_vectors: 1,
        vector_size: 1 << log_message,
        interleaving_depth: 1 << log_width,
        rate: 2f64.powi(-(log_inv_rate as i32)),
        mode: IrsMode::Standard,
    }))
}

#[derive(Clone)]
struct PublicInput(Vec<u8>);
impl Encoding<[u8]> for PublicInput {
    fn encode(&self) -> impl AsRef<[u8]> {
        &self.0
    }
}
impl PublicInput {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn integer(&mut self, n: usize) {
        self.0.extend_from_slice(&(n as u64).to_le_bytes());
    }
    fn field(&mut self, x: Fr) {
        x.serialize_compressed(&mut self.0).unwrap();
    }
    fn root(&mut self, x: Hash) {
        self.0.extend_from_slice(&x.0);
    }
    fn fields(&mut self, xs: &[Fr]) {
        self.integer(xs.len());
        for &x in xs {
            self.field(x);
        }
    }
}

/// Compact linear SEP covectors. Materialization is charged to the prover.
#[derive(Clone, Debug, PartialEq)]
enum Form {
    Mle(Vec<Fr>),
    Batch {
        point: Vec<Fr>,
        alpha: Fr,
        lambda: Fr,
        indices: Vec<usize>,
        rows: usize,
        width: usize,
    },
    Canopy {
        point: Vec<Fr>,
        gamma: Fr,
        indices: Vec<usize>,
    },
}
impl Form {
    fn bind(&self, input: &mut PublicInput) {
        match self {
            Self::Mle(point) => {
                input.integer(0);
                input.fields(point);
            }
            Self::Batch {
                point,
                alpha,
                lambda,
                indices,
                rows,
                width,
            } => {
                input.integer(1);
                input.fields(point);
                input.field(*alpha);
                input.field(*lambda);
                input.integer(*rows);
                input.integer(*width);
                input.integer(indices.len());
                for &i in indices {
                    input.integer(i);
                }
            }
            Self::Canopy {
                point,
                gamma,
                indices,
            } => {
                input.integer(2);
                input.fields(point);
                input.field(*gamma);
                input.integer(indices.len());
                for &i in indices {
                    input.integer(i);
                }
            }
        }
    }
    fn validate(&self, cfg: &CommitmentConfig) -> Result<()> {
        let log_message = cfg.vector_size().ilog2() as usize;
        match self {
            Self::Mle(point) => ensure!(point.len() == log_message, "MLE covector shape"),
            Self::Batch {
                point,
                indices,
                rows,
                width,
                ..
            } => {
                ensure!(point.len() == log_message, "batch covector shape");
                ensure!(
                    *rows == cfg.codeword_length() && *width == cfg.interleaving_depth(),
                    "batch covector RS layout"
                );
                ensure!(
                    indices.iter().all(|i| i < rows),
                    "batch covector query index"
                );
            }
            Self::Canopy { point, indices, .. } => {
                ensure!(point.len() == log_message, "Canopy covector shape");
                ensure!(
                    indices.iter().all(|&i| i < cfg.vector_size()),
                    "Canopy covector query index"
                );
            }
        }
        Ok(())
    }
    fn materialize(&self) -> Buffer<Fr> {
        match self {
            Self::Mle(point) => Buffer::eq_weights(point),
            Self::Canopy {
                point,
                gamma,
                indices,
            } => {
                let mut values = Buffer::eq_weights(point).into_vec();
                for (&i, w) in indices.iter().zip(powers(*gamma, *gamma, indices.len())) {
                    values[i] += w;
                }
                Buffer::from(values)
            }
            Self::Batch {
                point,
                alpha,
                lambda,
                indices,
                rows,
                width,
            } => {
                let n = 1usize << point.len();
                let mut form = Buffer::eq_weights(point);
                form.accumulate_geometric(&[*alpha], &Buffer::from(vec![*lambda]), n);
                let weights = powers(
                    lambda.pow([*width as u64]),
                    *lambda * *lambda,
                    indices.len(),
                );
                let low = additive_rs::power_sums(indices, &weights, *rows, n / width);
                let cols = powers(*lambda, Fr::ONE, *width);
                // The public covector is no longer needed in its old buffer:
                // add the query term in place instead of allocating a second N-vector.
                let mut values = form.into_vec();
                values.par_iter_mut().enumerate().for_each(|(i, value)| {
                    *value += cols[i / (n / width)] * low.to_slice()[i % (n / width)];
                });
                Buffer::from(values)
            }
        }
    }
}

fn powers(base: Fr, start: Fr, n: usize) -> Vec<Fr> {
    (0..n)
        .scan(start, |p, _| {
            let x = *p;
            *p *= base;
            Some(x)
        })
        .collect()
}
fn point(log: usize, seed: u64, tag: u64) -> Vec<Fr> {
    (0..log).map(|i| elem(seed, tag, i as u64)).collect()
}
fn message(log: usize, seed: u64, tag: u64) -> Buffer<Fr> {
    Buffer::from(
        (0..1usize << log)
            .map(|i| elem(seed, tag, i as u64))
            .collect::<Vec<_>>(),
    )
}
fn read<T: whir_ref::transcript::ProverMessage>(v: &mut V) -> Result<T> {
    v.prover_message()
        .map_err(|e| anyhow::anyhow!("transcript: {e:?}"))
}
fn read_fields(v: &mut V, n: usize) -> Result<Vec<Fr>> {
    (0..n).map(|_| read(v)).collect()
}
fn tampering_rejected(f: impl FnOnce() -> Result<usize>) -> bool {
    // Debug transcript-pattern instrumentation deliberately panics when a
    // changed challenge changes the number or type of opening messages.
    #[cfg(debug_assertions)]
    {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
            Ok(result) => result.is_err(),
            Err(payload) => {
                let text = payload
                    .downcast_ref::<String>()
                    .map(String::as_str)
                    .or_else(|| payload.downcast_ref::<&str>().copied())
                    .unwrap_or("");
                if text.contains("Transcript error: Expected interaction") {
                    true
                } else {
                    // An unrelated panic is a bug, never evidence that a
                    // malformed proof was cryptographically rejected.
                    std::panic::resume_unwind(payload)
                }
            }
        }
    }
    #[cfg(not(debug_assertions))]
    {
        f().is_err()
    }
}
fn encode(cfg: &CommitmentConfig, m: &Buffer<Fr>) -> Buffer<Fr> {
    let refs = [m];
    let segments = [PolynomialSegment::from_rows(
        &refs,
        cfg.interleaving_depth(),
    )];
    ntt::interleaved_rs_encode(Polynomials::from_segments(&segments), cfg.codeword_length())
}

#[derive(Clone)]
struct Prepared {
    message: Buffer<Fr>,
    witness: Witness,
    root: Hash,
    form: Form,
    sigma: Fr,
}
fn decode_root(cfg: &CommitmentConfig, root: Hash) -> Commitment {
    let ds = DomainSeparator::protocol(cfg)
        .session(&"section8/root-codec/v1")
        .instance(&Empty);
    let mut p = ProverState::<spongefish::instantiations::XOF<blake3::Hasher>>::new(
        &ds,
        Default::default(),
    );
    p.prover_message(&root);
    let proof = p.proof();
    let mut v = VerifierState::<spongefish::instantiations::XOF<blake3::Hasher>>::new(
        &ds,
        &proof,
        Default::default(),
    );
    cfg.receive_commitment(&mut v).unwrap()
}
fn prepare(cfg: &CommitmentConfig, m: Buffer<Fr>, form: Form) -> Prepared {
    let sigma = m.dot(&form.materialize());
    prepare_with_sigma(cfg, m, form, sigma)
}
fn prepare_with_sigma(cfg: &CommitmentConfig, m: Buffer<Fr>, form: Form, sigma: Fr) -> Prepared {
    prepare_with_sigma_profiled(cfg, m, form, sigma, &mut CommitmentProfile::default())
}
fn prepare_with_sigma_profiled(
    cfg: &CommitmentConfig,
    m: Buffer<Fr>,
    form: Form,
    sigma: Fr,
    profile: &mut CommitmentProfile,
) -> Prepared {
    assert_eq!(
        cfg.mask_length(),
        0,
        "profiled setup is the unmasked Standard IRS mode"
    );
    assert_eq!(m.len(), cfg.vector_size());
    let ds = DomainSeparator::protocol(cfg)
        .session(&"section8/setup/v1")
        .instance(&Empty);
    let mut p = P::new(&ds, Sponge::default());
    let t = Instant::now();
    let matrix = encode(cfg, &m);
    profile.encoding += t.elapsed().as_secs_f64();
    let (nodes, root) = merklize_profiled(&matrix, cfg, profile);
    p.prover_message(&root);
    let witness = Witness {
        masks: Buffer::from(Vec::<Fr>::new()),
        matrix,
        matrix_witness: merkle_tree::Witness::new(nodes),
    };
    Prepared {
        message: m,
        witness,
        root,
        form,
        sigma,
    }
}
#[derive(Clone)]
struct BatchPublicInput {
    root: Hash,
    form: Form,
    sigma: Fr,
}
impl From<&Prepared> for BatchPublicInput {
    fn from(s: &Prepared) -> Self {
        Self {
            root: s.root,
            form: s.form.clone(),
            sigma: s.sigma,
        }
    }
}
#[derive(Clone)]
struct BatchPublicOutput {
    root: Hash,
    form: Form,
    sigma: Fr,
}
impl From<&Prepared> for BatchPublicOutput {
    fn from(s: &Prepared) -> Self {
        Self {
            root: s.root,
            form: s.form.clone(),
            sigma: s.sigma,
        }
    }
}
fn bind_batch_inputs(input: &mut PublicInput, sources: &[BatchPublicInput]) {
    input.integer(sources.len());
    for s in sources {
        input.root(s.root);
        s.form.bind(input);
        input.field(s.sigma);
    }
}
fn prior_form(cfg: &CommitmentConfig, seed: u64, tag: u64) -> Form {
    Form::Batch {
        point: point(cfg.vector_size().ilog2() as usize, seed, tag),
        alpha: elem(seed, tag + 1, 0),
        lambda: elem(seed, tag + 2, 0),
        indices: (0..cfg.in_domain_samples())
            .map(|i| (i * 7919 + tag as usize * 97) % cfg.codeword_length())
            .collect(),
        rows: cfg.codeword_length(),
        width: cfg.interleaving_depth(),
    }
}

struct Prefix {
    point: Vec<Fr>,
    values: Vec<Fr>,
    weights: Vec<Fr>,
    message: Buffer<Fr>,
    matrix: Buffer<Fr>,
    leaves: Option<Vec<Hash>>,
}

// A fresh MLE covector stays factored throughout sumcheck. General carried
// claims retain the dense path; the public Form and verifier remain unchanged.
enum ProverForm {
    Dense(Buffer<Fr>),
    Mle {
        point: Vec<Fr>,
        scale: Fr,
        round: usize,
        next_polynomial: Option<(Fr, Fr)>,
    },
}
impl ProverForm {
    fn from_form(form: &Form) -> Self {
        match form {
            Form::Mle(point) => Self::Mle {
                point: point.clone(),
                scale: Fr::ONE,
                round: 0,
                next_polynomial: None,
            },
            _ => Self::Dense(form.materialize()),
        }
    }

    fn polynomial(&self, message: &Buffer<Fr>) -> (Fr, Fr) {
        match self {
            Self::Dense(form) => message.sumcheck_polynomial(form),
            Self::Mle {
                point,
                scale,
                round,
                next_polynomial,
            } => {
                if let Some(coefficients) = next_polynomial {
                    return *coefficients;
                }
                let (low, high) = message.to_slice().split_at(message.len() / 2);
                let z = point[*round];
                let tail = &point[*round + 1..];
                let l0 = multilinear_extend(low, tail);
                let l1 = multilinear_extend(high, tail);
                // eq(z, X) = 1-z-X in characteristic two. These are exactly
                // the dense constant/quadratic coefficients, including zero
                // factors. They depend only on the witness, never its sigma.
                (*scale * (Fr::ONE - z) * l0, *scale * (l1 - l0))
            }
        }
    }

    // The first round reads the original message directly, then allocates only
    // its half-length folded witness. Dense carried covectors keep native fold.
    fn fold_first(&mut self, message: &Buffer<Fr>, challenge: Fr) -> Buffer<Fr> {
        match self {
            Self::Dense(form) => {
                let folded = fold_borrowed_message(message.to_slice(), challenge);
                form.fold(challenge);
                folded
            }
            Self::Mle {
                point,
                scale,
                round,
                next_polynomial,
            } => {
                *scale *= Fr::ONE - point[*round] - challenge;
                *round += 1;
                if message.len() <= 2 || *scale == Fr::ZERO {
                    *next_polynomial = (message.len() > 2).then_some((Fr::ZERO, Fr::ZERO));
                    return fold_borrowed_message(message.to_slice(), challenge);
                }
                let half = message.len() / 2;
                let (low, high) = message.to_slice().split_at(half);
                let mut folded = vec![Fr::ZERO; half];
                let (out0, out1) = folded.split_at_mut(half / 2);
                let (low0, low1) = low.split_at(half / 2);
                let (high0, high1) = high.split_at(half / 2);
                let tail = &point[*round + 1..];
                let (l0, l1) = if out_work_parallel(half) {
                    rayon::join(
                        || fold_evaluate_borrowed(out0, low0, high0, challenge, tail),
                        || fold_evaluate_borrowed(out1, low1, high1, challenge, tail),
                    )
                } else {
                    (
                        fold_evaluate_borrowed(out0, low0, high0, challenge, tail),
                        fold_evaluate_borrowed(out1, low1, high1, challenge, tail),
                    )
                };
                *next_polynomial =
                    Some((*scale * (Fr::ONE - point[*round]) * l0, *scale * (l1 - l0)));
                Buffer::from(folded)
            }
        }
    }

    fn fold_pair(&mut self, message: &mut Buffer<Fr>, challenge: Fr) {
        match self {
            Self::Dense(form) => message.fold_pair(form, challenge),
            Self::Mle {
                point,
                scale,
                round,
                next_polynomial,
            } => {
                *scale *= Fr::ONE - point[*round] - challenge;
                *round += 1;
                if message.len() <= 2 || *scale == Fr::ZERO {
                    message.fold(challenge);
                    *next_polynomial = (message.len() > 1).then_some((Fr::ZERO, Fr::ZERO));
                    return;
                }
                let mut values = std::mem::take(message).into_vec();
                let half = values.len() / 2;
                let (low, high) = values.split_at_mut(half);
                let (low0, low1) = low.split_at_mut(half / 2);
                let (high0, high1) = high.split_at(half / 2);
                let tail = &point[*round + 1..];
                let (l0, l1) = if out_work_parallel(half) {
                    rayon::join(
                        || fold_evaluate_in_place(low0, high0, challenge, tail),
                        || fold_evaluate_in_place(low1, high1, challenge, tail),
                    )
                } else {
                    (
                        fold_evaluate_in_place(low0, high0, challenge, tail),
                        fold_evaluate_in_place(low1, high1, challenge, tail),
                    )
                };
                *next_polynomial =
                    Some((*scale * (Fr::ONE - point[*round]) * l0, *scale * (l1 - l0)));
                // Preserve the native fold's allocation policy after round one.
                values.truncate(half);
                values.shrink_to_fit();
                *message = Buffer::from(values);
            }
        }
    }
}

fn fold_borrowed_message(message: &[Fr], challenge: Fr) -> Buffer<Fr> {
    let (low, high) = message.split_at(message.len() / 2);
    Buffer::from(
        low.par_iter()
            .zip(high)
            .map(|(&a, &b)| a + (b - a) * challenge)
            .collect::<Vec<_>>(),
    )
}

// Each recursive subtree writes its folded witness and evaluates that same
// subtree at `point`. Returning the evaluation eliminates the next MLE scan;
// no claim value, division, or assumption about equality factors is used.
fn fold_evaluate_borrowed(
    out: &mut [Fr],
    low: &[Fr],
    high: &[Fr],
    challenge: Fr,
    point: &[Fr],
) -> Fr {
    // Match the native MLE's unrolled four-variable base case. The folded
    // tile stays in cache, avoiding recursive calls for each field element.
    if point.len() <= 4 {
        for ((out, &a), &b) in out.iter_mut().zip(low).zip(high) {
            *out = a + (b - a) * challenge;
        }
        return multilinear_extend(out, point);
    }
    if let Some((&x, tail)) = point.split_first() {
        let half = out.len() / 2;
        let (o0, o1) = out.split_at_mut(half);
        let (l0, l1) = low.split_at(half);
        let (h0, h1) = high.split_at(half);
        let (a, b) = if out_work_parallel(half * 2) {
            rayon::join(
                || fold_evaluate_borrowed(o0, l0, h0, challenge, tail),
                || fold_evaluate_borrowed(o1, l1, h1, challenge, tail),
            )
        } else {
            (
                fold_evaluate_borrowed(o0, l0, h0, challenge, tail),
                fold_evaluate_borrowed(o1, l1, h1, challenge, tail),
            )
        };
        a + (b - a) * x
    } else {
        out[0] = low[0] + (high[0] - low[0]) * challenge;
        out[0]
    }
}

fn fold_evaluate_in_place(low: &mut [Fr], high: &[Fr], challenge: Fr, point: &[Fr]) -> Fr {
    if point.len() <= 4 {
        for (a, &b) in low.iter_mut().zip(high) {
            *a += (b - *a) * challenge;
        }
        return multilinear_extend(low, point);
    }
    if let Some((&x, tail)) = point.split_first() {
        let half = low.len() / 2;
        let (l0, l1) = low.split_at_mut(half);
        let (h0, h1) = high.split_at(half);
        let (a, b) = if out_work_parallel(half * 2) {
            rayon::join(
                || fold_evaluate_in_place(l0, h0, challenge, tail),
                || fold_evaluate_in_place(l1, h1, challenge, tail),
            )
        } else {
            (
                fold_evaluate_in_place(l0, h0, challenge, tail),
                fold_evaluate_in_place(l1, h1, challenge, tail),
            )
        };
        a + (b - a) * x
    } else {
        low[0] += (high[0] - low[0]) * challenge;
        low[0]
    }
}

fn out_work_parallel(length: usize) -> bool {
    length > whir_ref::utils::workload_size::<Fr>()
}

enum SumcheckMessage<'a> {
    Borrowed(&'a Buffer<Fr>),
    Folded(Buffer<Fr>),
}
impl SumcheckMessage<'_> {
    fn buffer(&self) -> &Buffer<Fr> {
        match self {
            Self::Borrowed(message) => message,
            Self::Folded(message) => message,
        }
    }
    fn fold_pair(&mut self, form: &mut ProverForm, challenge: Fr) {
        match self {
            Self::Borrowed(message) => *self = Self::Folded(form.fold_first(message, challenge)),
            Self::Folded(message) => form.fold_pair(message, challenge),
        }
    }
}

fn prefix_prove(
    cfg: &CommitmentConfig,
    p: &mut P,
    messages: &[&Buffer<Fr>],
    matrices: &[&Buffer<Fr>],
    forms: Vec<ProverForm>,
    sigmas: &[Fr],
    times: &mut ReductionMeasurement,
) -> Prefix {
    let start = Instant::now();
    let beta: Fr = p.verifier_message();
    let bs = powers(beta, Fr::ONE, messages.len());
    let mut sum = sigmas.iter().zip(&bs).map(|(&s, &b)| s * b).sum::<Fr>();
    let mut pairs = messages
        .iter()
        .zip(forms)
        .map(|(m, f)| (SumcheckMessage::Borrowed(*m), f))
        .collect::<Vec<_>>();
    times.sumcheck_setup_seconds = start.elapsed().as_secs_f64();
    let mut point = Vec::new();
    for _ in 0..cfg.vector_size().ilog2() {
        let t = Instant::now();
        let (a, c) = pairs
            .iter()
            .zip(&bs)
            .map(|((m, f), &b)| {
                let (a, c) = f.polynomial(m.buffer());
                (a * b, c * b)
            })
            .fold((Fr::ZERO, Fr::ZERO), |(a, b), (c, d)| (a + c, b + d));
        times.sumcheck_polynomial_seconds += t.elapsed().as_secs_f64();
        p.prover_message(&a);
        p.prover_message(&c);
        let r: Fr = p.verifier_message();
        sum = (c * r + sum - c) * r + a;
        let t = Instant::now();
        for (m, f) in &mut pairs {
            m.fold_pair(f, r);
        }
        times.sumcheck_fold_seconds += t.elapsed().as_secs_f64();
        point.push(r);
    }
    let values = pairs
        .iter()
        .map(|(m, _)| m.buffer().to_slice()[0])
        .collect::<Vec<_>>();
    drop(pairs);
    p.prover_messages(&values);
    let gamma: Fr = p.verifier_message();
    let weights = powers(gamma, Fr::ONE, messages.len());
    times.sumcheck_seconds = start.elapsed().as_secs_f64();
    let start = Instant::now();
    let combine = |buffers: &[&Buffer<Fr>], len: usize| {
        assert_eq!(weights[0], Fr::ONE);
        // The first geometric coefficient is ONE. This is the same shortcut
        // as native mixed_linear_combination, retaining the parallel fused sweep.
        if buffers.len() == 1 {
            return (*buffers[0]).clone();
        }
        Buffer::from(
            (0..len)
                .into_par_iter()
                .map(|i| {
                    buffers[0].to_slice()[i]
                        + buffers
                            .iter()
                            .zip(&weights)
                            .skip(1)
                            .map(|(m, &w)| m.to_slice()[i] * w)
                            .sum::<Fr>()
                })
                .collect::<Vec<_>>(),
        )
    };
    let t = Instant::now();
    let message = combine(messages, cfg.vector_size());
    times.message_combination_seconds = t.elapsed().as_secs_f64();
    let t = Instant::now();
    // At larger arities, hash completed tiles while they are still cache-local.
    // The measured combined phase includes those leaves; do not charge them
    // again to output commitment or pretend to split overlapping wall time.
    let fused = matrices.len() >= 8 && cfg.size() >= 65_536;
    let (matrix, leaves) = if fused {
        let engine = hash::ENGINES
            .retrieve(cfg.matrix_commit().leaf_hash_id)
            .unwrap();
        let (matrix, leaves) = crate::section8_codeword::combine_and_hash(
            matrices,
            &weights,
            cfg.codeword_length(),
            cfg.interleaving_depth(),
            &*engine,
            512,
        );
        (matrix, Some(leaves))
    } else {
        (combine(matrices, cfg.size()), None)
    };
    times.codeword_leaf_fused = fused;
    times.codeword_combination_seconds = t.elapsed().as_secs_f64();
    times.combination_seconds = start.elapsed().as_secs_f64();
    times.message_combination_seconds = phase_remainder(
        times.combination_seconds,
        times.codeword_combination_seconds,
    );
    Prefix {
        point,
        values,
        weights,
        message,
        matrix,
        leaves,
    }
}
fn prefix_verify(
    cfg: &CommitmentConfig,
    v: &mut V,
    forms: &[Buffer<Fr>],
    sigmas: &[Fr],
) -> Result<(Vec<Fr>, Vec<Fr>, Vec<Fr>)> {
    let beta: Fr = v.verifier_message();
    let bs = powers(beta, Fr::ONE, forms.len());
    let mut sum = sigmas.iter().zip(&bs).map(|(&s, &b)| s * b).sum::<Fr>();
    let mut point = Vec::new();
    for _ in 0..cfg.vector_size().ilog2() {
        let a: Fr = read(v)?;
        let c: Fr = read(v)?;
        let r: Fr = v.verifier_message();
        sum = (c * r + sum - c) * r + a;
        point.push(r);
    }
    let values = read_fields(v, forms.len())?;
    ensure!(
        sum == forms
            .iter()
            .zip(&values)
            .zip(&bs)
            .map(|((f, &y), &b)| multilinear_extend(f.to_slice(), &point) * y * b)
            .sum::<Fr>(),
        "batch sumcheck failed"
    );
    let gamma: Fr = v.verifier_message();
    Ok((point, values, powers(gamma, Fr::ONE, forms.len())))
}

fn batch_claim(
    point: Vec<Fr>,
    alpha: Fr,
    y: Fr,
    lambda: Fr,
    values: &[Fr],
    weights: &[Fr],
    indices: Vec<usize>,
    opened: &[Vec<Fr>],
    cfg: &CommitmentConfig,
) -> (Form, Fr) {
    let width = cfg.interleaving_depth();
    let qweights = powers(lambda, lambda * lambda, indices.len() * width);
    let sigma = values.iter().zip(weights).map(|(&y, &w)| y * w).sum::<Fr>()
        + lambda * y
        + opened
            .iter()
            .zip(weights)
            .map(|(rows, &w)| {
                rows.iter()
                    .zip(&qweights)
                    .map(|(&a, &b)| a * b * w)
                    .sum::<Fr>()
            })
            .sum::<Fr>();
    (
        Form::Batch {
            point,
            alpha,
            lambda,
            indices,
            rows: cfg.codeword_length(),
            width,
        },
        sigma,
    )
}

fn measure_proof(
    times: &mut ReductionMeasurement,
    proof: &Proof,
    root_bytes: usize,
    output_fields: usize,
    transcript_fields: usize,
    opening_fields: usize,
    auth_proof_hashes: usize,
) {
    times.output_instance_bytes = root_bytes + output_fields * FIELD_BYTES;
    times.serialized_total_bytes = proof.narg_string.len() + proof.hints.len();
    times.proof_bytes = times.serialized_total_bytes - times.output_instance_bytes;
    times.proof_hint_bytes = proof.hints.len();
    times.proof_field_bytes = (transcript_fields - output_fields + opening_fields) * FIELD_BYTES;
    times.proof_hash_bytes = auth_proof_hashes * HASH_BYTES;
    times.proof_framing_bytes =
        times.proof_bytes - times.proof_field_bytes - times.proof_hash_bytes;
}
fn verify_fs<T>(f: impl FnOnce() -> Result<T>, analytic: usize) -> Result<(T, usize)> {
    #[cfg(feature = "fs-accounting")]
    {
        let (result, ledgers) = crate::fiat_shamir::capture(f);
        ensure!(
            ledgers.len() == 1,
            "unexpected auxiliary verifier transcript"
        );
        // Public-instance codecs may absorb more than 64 bytes at once. Count
        // XOF finalizations directly instead of using the legacy compression ledger.
        let mut n = 0;
        let mut squeezing = false;
        for event in &ledgers[0].events {
            if event.operation == 'a' {
                squeezing = false;
            } else if !squeezing {
                n += 1;
                squeezing = true;
            }
        }
        ensure!(n == analytic, "FS finalizations {n} != analytic {analytic}");
        Ok((result?, n))
    }
    #[cfg(not(feature = "fs-accounting"))]
    {
        Ok((f()?, analytic))
    }
}

fn check_batch_output(cfg: &CommitmentConfig, out: &Prepared, full_check: bool) -> Result<()> {
    ensure!(
        out.message.dot(&out.form.materialize()) == out.sigma,
        "output SEP witness failed"
    );
    if full_check {
        ensure!(
            encode(cfg, &out.message) == out.witness.matrix,
            "output RS codeword failed"
        );
        let checked = prepare(cfg, out.message.clone(), out.form.clone());
        ensure!(checked.root == out.root, "output Merkle commitment failed");
    }
    Ok(())
}

/// Verify against public input/output statements only; no message or tree witness is available.
fn verify_batch_proof(
    cfg: &CommitmentConfig,
    sources: &[BatchPublicInput],
    expected: &BatchPublicOutput,
    proof: &Proof,
) -> Result<usize> {
    ensure!(!sources.is_empty(), "batch needs a public input");
    let fields = 2 * cfg.vector_size().ilog2() as usize + sources.len() + 1;
    ensure!(
        proof.narg_string.len() == fields * FIELD_BYTES + HASH_BYTES,
        "incorrect batch transcript length"
    );

    for source in sources {
        source.form.validate(cfg)?;
    }
    expected.form.validate(cfg)?;
    let mut input = PublicInput::new();
    bind_batch_inputs(&mut input, sources);
    let ds = DomainSeparator::protocol(cfg)
        .session(&"section8/merkle-batch/v1")
        .instance(&input);
    let commitments = sources
        .iter()
        .map(|s| decode_root(cfg, s.root))
        .collect::<Vec<_>>();
    let forms = sources
        .iter()
        .map(|s| s.form.materialize())
        .collect::<Vec<_>>();
    let sigmas = sources.iter().map(|s| s.sigma).collect::<Vec<_>>();
    let mut v = V::new(&ds, proof, Sponge::default());
    let (point, values, weights) = prefix_verify(cfg, &mut v, &forms, &sigmas)?;
    ensure!(
        read::<Hash>(&mut v)? == expected.root,
        "output root differs"
    );
    let alpha: Fr = v.verifier_message();
    let y: Fr = read(&mut v)?;
    let eval = cfg
        .verify(&mut v, &commitments.iter().collect::<Vec<_>>())
        .map_err(|e| anyhow::anyhow!("Merkle openings: {e:?}"))?;
    let lambda: Fr = v.verifier_message();
    let indices = eval.domain.as_ref().unwrap().exponents.clone();
    let width = cfg.interleaving_depth();
    let stride = width * sources.len();
    let opened = (0..sources.len())
        .map(|j| {
            eval.matrix
                .to_slice()
                .chunks(stride)
                .flat_map(|r| r[j * width..(j + 1) * width].iter().copied())
                .collect()
        })
        .collect::<Vec<Vec<Fr>>>();
    let (form, sigma) = batch_claim(
        point,
        alpha,
        y,
        lambda,
        &values,
        &weights,
        indices.clone(),
        &opened,
        cfg,
    );
    ensure!(
        form == expected.form && sigma == expected.sigma,
        "output SEP instance differs"
    );
    let work = authentication_work_by_height(&indices, cfg.codeword_length())?;
    // Native IRS hashes every sampled row, then its Merkle verifier
    // reconstructs each distinct parent once. These local counts avoid
    // a process-global instrumentation counter shared by other proofs.
    let hashes = sources.len() * (cfg.in_domain_samples() + work.last().unwrap().parents);
    v.check_eof()
        .map_err(|e| anyhow::anyhow!("trailing proof data: {e:?}"))?;
    Ok(hashes)
}

fn run_batch(
    cfg: &CommitmentConfig,
    sources: &[Prepared],
    scheme: &str,
    incoming_commit_seconds: f64,
    full_check: bool,
) -> Result<(ReductionMeasurement, Prepared, Proof)> {
    ensure!(!sources.is_empty(), "batch needs an input");
    for source in sources {
        source.form.validate(cfg)?;
    }
    let mut input = PublicInput::new();
    let public_sources = sources
        .iter()
        .map(BatchPublicInput::from)
        .collect::<Vec<_>>();
    bind_batch_inputs(&mut input, &public_sources);
    let ds = DomainSeparator::protocol(cfg)
        .session(&"section8/merkle-batch/v1")
        .instance(&input);
    let mut times = ReductionMeasurement {
        scheme: scheme.into(),
        log_n: cfg.vector_size().ilog2() as usize,
        sources: sources.len(),
        queries: cfg.in_domain_samples(),
        incoming_commit_seconds,
        ..Default::default()
    };
    let start = Instant::now();
    let mut p = P::new(&ds, Sponge::default());
    let t = Instant::now();
    let forms = sources
        .iter()
        .map(|s| ProverForm::from_form(&s.form))
        .collect::<Vec<_>>();
    times.form_seconds = t.elapsed().as_secs_f64();
    let sigmas = sources.iter().map(|s| s.sigma).collect::<Vec<_>>();
    let messages = sources.iter().map(|s| &s.message).collect::<Vec<_>>();
    let matrices = sources
        .iter()
        .map(|s| &s.witness.matrix)
        .collect::<Vec<_>>();
    let mut prefix = prefix_prove(
        cfg, &mut p, &messages, &matrices, forms, &sigmas, &mut times,
    );
    let t = Instant::now();
    let before_root =
        2 * cfg.vector_size().ilog2() as usize * FIELD_BYTES + sources.len() * FIELD_BYTES;
    let mut output_profile = CommitmentProfile::default();
    let (nodes, output_root) = merklize_prehashed_profiled(
        &prefix.matrix,
        cfg,
        prefix.leaves.take(),
        &mut output_profile,
    );
    p.prover_message(&output_root);
    let matrix_witness = merkle_tree::Witness::new(nodes);
    let ood_start = Instant::now();
    let alpha: Fr = p.verifier_message();
    let y = prefix
        .message
        .mixed_univariate_evaluate(&Identity::new(), alpha);
    p.prover_message(&y);
    times.output_ood_seconds = ood_start.elapsed().as_secs_f64();
    times.output_commit_seconds = t.elapsed().as_secs_f64();
    output_profile.set_output(&mut times);
    let t = Instant::now();
    let refs = sources.iter().map(|s| &s.witness).collect::<Vec<_>>();
    let eval = cfg.open(&mut p, &refs);
    times.opening_auth_seconds = t.elapsed().as_secs_f64();
    let lambda: Fr = p.verifier_message();
    let indices = eval.domain.as_ref().unwrap().exponents.clone();
    let opened = sources
        .iter()
        .map(|s| {
            s.witness
                .matrix
                .read_rows(cfg.interleaving_depth(), &indices)
        })
        .collect::<Vec<_>>();
    let (form, sigma) = batch_claim(
        prefix.point.clone(),
        alpha,
        y,
        lambda,
        &prefix.values,
        &prefix.weights,
        indices.clone(),
        &opened,
        cfg,
    );
    times.opening_seconds = t.elapsed().as_secs_f64();
    times.opening_claim_seconds =
        phase_remainder(times.opening_seconds, times.opening_auth_seconds);
    let t = Instant::now();
    let proof = p.proof();
    times.serialization_seconds = t.elapsed().as_secs_f64();
    times.step_seconds = start.elapsed().as_secs_f64();
    times.finish_phases();
    let root = Hash(
        proof.narg_string[before_root..before_root + HASH_BYTES]
            .try_into()
            .unwrap(),
    );
    let out = Prepared {
        message: prefix.message,
        witness: Witness {
            masks: Buffer::from(Vec::<Fr>::new()),
            matrix: prefix.matrix,
            matrix_witness,
        },
        root,
        form,
        sigma,
    };
    let expected = BatchPublicOutput::from(&out);
    let verify = |proof: &Proof| verify_batch_proof(cfg, &public_sources, &expected, proof);
    let (hashes, fs) = verify_fs(|| verify(&proof), cfg.vector_size().ilog2() as usize + 4)?;
    times.auth_hashes = hashes;
    times.fs_finalizations = fs;
    let mut unique = indices.clone();
    unique.sort_unstable();
    unique.dedup();
    let auth = sources.len()
        * frontier_hash_count(
            &unique,
            cfg.codeword_length(),
            cfg.codeword_length().ilog2() as usize,
        )?;
    measure_proof(
        &mut times,
        &proof,
        HASH_BYTES,
        0,
        2 * cfg.vector_size().ilog2() as usize + sources.len() + 1,
        sources.len() * indices.len() * cfg.interleaving_depth(),
        auth,
    );
    check_batch_output(cfg, &out, full_check)?;
    if full_check {
        let mut bad = proof.clone();
        bad.narg_string[0] ^= 1;
        ensure!(
            tampering_rejected(|| verify(&bad)),
            "tampered sumcheck accepted"
        );
        let mut bad = proof.clone();
        bad.narg_string[before_root] ^= 1;
        ensure!(
            tampering_rejected(|| verify(&bad)),
            "tampered output root accepted"
        );
        let mut bad = proof.clone();
        bad.narg_string[before_root + HASH_BYTES] ^= 1;
        ensure!(
            tampering_rejected(|| verify(&bad)),
            "tampered output OOD accepted"
        );
        let mut bad = proof.clone();
        bad.hints[8] ^= 1; // Skip the first Vec<Fr> length prefix.
        ensure!(
            tampering_rejected(|| verify(&bad)),
            "tampered queried field accepted"
        );
        let mut bad = proof.clone();
        let last = bad.hints.len() - 1;
        bad.hints[last] ^= 1;
        ensure!(
            tampering_rejected(|| verify(&bad)),
            "tampered Merkle proof accepted"
        );
        let mut bad = proof.clone();
        bad.narg_string.push(0);
        ensure!(
            tampering_rejected(|| verify(&bad)),
            "trailing proof accepted"
        );
    }
    times.verified = true;
    Ok((times, out, proof))
}

/// A complete input to the Canopy reduction, including a carried general SEP claim.
struct CanopySource {
    message: Buffer<Fr>,
    data: CanopyData,
    form: Form,
    sigma: Fr,
}

struct CanopyData {
    base: Buffer<Fr>,
    levels: Vec<Vec<Hash>>,
    small: Prepared,
    layer_point: Vec<Fr>,
    layer_value: Fr,
}
fn parent_level(children: &[Hash]) -> Vec<Hash> {
    let engine = hash::ENGINES.retrieve(hash::BLAKE3).unwrap();
    let batch = engine.preferred_batch_size().max(256);
    let mut out = vec![Hash::default(); children.len() / 2];
    out.par_chunks_mut(batch).enumerate().for_each(|(j, dst)| {
        let children = &children[2 * j * batch..2 * (j * batch + dst.len())];
        let mut bytes = Vec::with_capacity(children.len() * HASH_BYTES);
        for h in children {
            bytes.extend_from_slice(&h.0);
        }
        engine.hash_many(2 * HASH_BYTES, &bytes, dst);
    });
    out
}
fn canopy_commit_profiled(
    base: Buffer<Fr>,
    cfg: &CommitmentConfig,
    layer_cfg: &CommitmentConfig,
    height: usize,
    p: &mut P,
    profile: &mut CommitmentProfile,
) -> CanopyData {
    canopy_commit_prehashed_profiled(base, cfg, layer_cfg, height, p, None, profile)
}
fn canopy_commit_prehashed_profiled(
    base: Buffer<Fr>,
    cfg: &CommitmentConfig,
    layer_cfg: &CommitmentConfig,
    height: usize,
    p: &mut P,
    leaves: Option<Vec<Hash>>,
    profile: &mut CommitmentProfile,
) -> CanopyData {
    let leaves = leaves.unwrap_or_else(|| {
        let t = Instant::now();
        let engine = hash::ENGINES.retrieve(hash::BLAKE3).unwrap();
        let mut leaves = vec![Hash::default(); cfg.codeword_length()];
        matrix_commit::hash_rows(&*engine, base.to_slice(), &mut leaves);
        profile.leaf += t.elapsed().as_secs_f64();
        leaves
    });
    assert_eq!(leaves.len(), cfg.codeword_length());
    let t = Instant::now();
    let mut levels = vec![leaves];
    for _ in 0..height - cfg.interleaving_depth().ilog2() as usize {
        levels.push(parent_level(levels.last().unwrap()));
    }
    profile.parent += t.elapsed().as_secs_f64();
    let t = Instant::now();
    let message = Buffer::from(
        levels
            .last()
            .unwrap()
            .iter()
            .map(|h| Fr::from_bytes(h.0))
            .collect::<Vec<_>>(),
    );
    profile.h2f += t.elapsed().as_secs_f64();
    let t = Instant::now();
    let matrix = encode(layer_cfg, &message);
    profile.encoding += t.elapsed().as_secs_f64();
    let (nodes, root) = merklize_profiled(&matrix, layer_cfg, profile);
    let t = Instant::now();
    p.prover_message(&root);
    let witness = Witness {
        masks: Buffer::from(Vec::<Fr>::new()),
        matrix,
        matrix_witness: merkle_tree::Witness::new(nodes),
    };
    let layer_point = p.verifier_message_vec(message.len().ilog2() as usize);
    let layer_value = multilinear_extend(message.to_slice(), &layer_point);
    p.prover_message(&layer_value);
    let form = Form::Mle(layer_point.clone());
    let small = Prepared {
        message,
        witness,
        root,
        form,
        sigma: layer_value,
    };
    let result = CanopyData {
        base,
        levels,
        small,
        layer_point,
        layer_value,
    };
    profile.ood += t.elapsed().as_secs_f64();
    result
}
fn prepare_canopy(
    m: &Buffer<Fr>,
    cfg: &CommitmentConfig,
    layer_cfg: &CommitmentConfig,
    height: usize,
) -> CanopyData {
    prepare_canopy_profiled(m, cfg, layer_cfg, height, &mut CommitmentProfile::default())
}
fn prepare_canopy_profiled(
    m: &Buffer<Fr>,
    cfg: &CommitmentConfig,
    layer_cfg: &CommitmentConfig,
    height: usize,
    profile: &mut CommitmentProfile,
) -> CanopyData {
    let ds = DomainSeparator::protocol(&(cfg, layer_cfg, height))
        .session(&"section8/canopy-setup/v1")
        .instance(&Empty);
    let mut p = P::new(&ds, Sponge::default());
    let t = Instant::now();
    let base = encode(cfg, m);
    profile.encoding += t.elapsed().as_secs_f64();
    canopy_commit_profiled(base, cfg, layer_cfg, height, &mut p, profile)
}
fn forest_open(data: &CanopyData, indices: &[usize], width: usize) -> (Vec<Fr>, Vec<Hash>) {
    let values = data.base.read_rows(width, indices);
    let mut current = indices.iter().copied().collect::<BTreeSet<_>>();
    let mut auth = Vec::new();
    for level in data.levels.iter().take(data.levels.len() - 1) {
        for &i in &current {
            if !current.contains(&(i ^ 1)) {
                auth.push(level[i ^ 1]);
            }
        }
        current = current.into_iter().map(|i| i / 2).collect();
    }
    (values, auth)
}
fn forest_verify(
    indices: &[usize],
    values: &[Fr],
    auth: &[Hash],
    width: usize,
    depth: usize,
) -> Result<(BTreeMap<usize, Fr>, usize)> {
    ensure!(
        values.len() == indices.len() * width,
        "bottom opening shape"
    );
    let engine = hash::ENGINES.retrieve(hash::BLAKE3).unwrap();
    let mut leaves = vec![Hash::default(); indices.len()];
    matrix_commit::hash_rows(&*engine, values, &mut leaves);
    let mut current = indices
        .iter()
        .copied()
        .zip(leaves)
        .collect::<BTreeMap<_, _>>();
    let mut cursor = 0;
    let mut hashes = indices.len();
    for _ in 0..depth {
        let mut parents = BTreeMap::new();
        for (&i, &h) in &current {
            if parents.contains_key(&(i / 2)) {
                continue;
            }
            let sibling = if let Some(h) = current.get(&(i ^ 1)) {
                *h
            } else {
                ensure!(cursor < auth.len(), "short bottom proof");
                let h = auth[cursor];
                cursor += 1;
                h
            };
            let (left, right) = if i % 2 == 0 {
                (h, sibling)
            } else {
                (sibling, h)
            };
            let mut bytes = [0; 64];
            bytes[..32].copy_from_slice(&left.0);
            bytes[32..].copy_from_slice(&right.0);
            let mut out = [Hash::default()];
            engine.hash_many(64, &bytes, &mut out);
            hashes += 1;
            parents.insert(i / 2, out[0]);
        }
        current = parents;
    }
    ensure!(cursor == auth.len(), "extra bottom hashes");
    Ok((
        current
            .into_iter()
            .map(|(i, h)| (i, Fr::from_bytes(h.0)))
            .collect(),
        hashes,
    ))
}
fn canopy_claim(data: &CanopyData, roots: &BTreeMap<usize, Fr>, gamma: Fr) -> (Form, Fr) {
    canopy_claim_at(&data.layer_point, data.layer_value, roots, gamma)
}
fn canopy_claim_at(
    layer_point: &[Fr],
    layer_value: Fr,
    roots: &BTreeMap<usize, Fr>,
    gamma: Fr,
) -> (Form, Fr) {
    let indices = roots.keys().copied().collect::<Vec<_>>();
    let sigma = layer_value
        + roots
            .values()
            .zip(powers(gamma, gamma, roots.len()))
            .map(|(&x, w)| x * w)
            .sum::<Fr>();
    (
        Form::Canopy {
            point: layer_point.to_vec(),
            gamma,
            indices,
        },
        sigma,
    )
}

#[derive(Clone)]
struct CanopyPublicInput {
    root: Hash,
    layer_point: Vec<Fr>,
    layer_value: Fr,
    form: Form,
    sigma: Fr,
}
#[derive(Clone)]
struct IfoldPublicOutput {
    root: Hash,
    layer_point: Vec<Fr>,
    layer_value: Fr,
    large_form: Form,
    large_sigma: Fr,
    residual_claims: Vec<(Form, Fr)>,
}
fn bind_canopy_inputs(input: &mut PublicInput, sources: &[CanopyPublicInput]) {
    input.integer(sources.len());
    for s in sources {
        input.root(s.root);
        input.fields(&s.layer_point);
        input.field(s.layer_value);
        s.form.bind(input);
        input.field(s.sigma);
    }
}
#[cfg_attr(not(test), allow(dead_code))]
struct IfoldExecution {
    measurement: ReductionMeasurement,
    residual: Vec<Prepared>,
    // The proof and witnesses are also retained for independent regression tests.
    public_inputs: Vec<CanopyPublicInput>,
    public_output: IfoldPublicOutput,
    output_message: Buffer<Fr>,
    output: CanopyData,
    proof: Proof,
}

/// `ifold` verifies bottom paths and returns their deferred public SEP statements.
fn verify_ifold_proof(
    cfg: &CommitmentConfig,
    layer_cfg: &CommitmentConfig,
    height: usize,
    sources: &[CanopyPublicInput],
    expected: &IfoldPublicOutput,
    proof: &Proof,
) -> Result<usize> {
    ensure!(!sources.is_empty(), "ifold needs a public input");
    let fields = 2 * cfg.vector_size().ilog2() as usize + sources.len() + 2;
    ensure!(
        proof.narg_string.len() == fields * FIELD_BYTES + HASH_BYTES,
        "incorrect ifold transcript length"
    );

    let ell_cast = sources.len();
    ensure!(
        expected.residual_claims.len() == ell_cast,
        "residual arity differs"
    );
    let log_n = cfg.vector_size().ilog2() as usize;
    let log_width = cfg.interleaving_depth().ilog2() as usize;
    ensure!(
        height > 1 && height >= log_width && height <= log_n,
        "invalid ifold verifier height"
    );
    ensure!(
        cfg.size() == 2 * cfg.vector_size()
            && layer_cfg.size() == cfg.vector_size()
            && layer_cfg.vector_size() == (2 * cfg.vector_size()) >> height
            && layer_cfg.interleaving_depth() == cfg.interleaving_depth(),
        "incompatible ifold RS geometry"
    );
    let layer_log = layer_cfg.vector_size().ilog2() as usize;
    ensure!(
        expected.layer_point.len() == layer_log,
        "output Canopy OOD shape"
    );
    expected.large_form.validate(cfg)?;
    for (form, _) in &expected.residual_claims {
        form.validate(layer_cfg)?;
    }
    for source in sources {
        source.form.validate(cfg)?;
        ensure!(
            source.layer_point.len() == layer_log,
            "input Canopy OOD shape"
        );
    }
    let depth = height - log_width;
    let mut input = PublicInput::new();
    bind_canopy_inputs(&mut input, sources);
    let ds = DomainSeparator::protocol(&(cfg, layer_cfg, height, ell_cast))
        .session(&"section8/ifold/v1")
        .instance(&input);
    let dense = sources
        .iter()
        .map(|s| s.form.materialize())
        .collect::<Vec<_>>();
    let sigmas = sources.iter().map(|s| s.sigma).collect::<Vec<_>>();
    let mut v = V::new(&ds, proof, Sponge::default());
    let (point, values, weights) = prefix_verify(&cfg, &mut v, &dense, &sigmas)?;
    ensure!(
        read::<Hash>(&mut v)? == expected.root,
        "ifold output commitment differs"
    );
    let layer_point: Vec<Fr> = v.verifier_message_vec(layer_log);
    let layer_value: Fr = read(&mut v)?;
    ensure!(
        layer_point == expected.layer_point && layer_value == expected.layer_value,
        "ifold Canopy OOD differs"
    );
    let alpha: Fr = v.verifier_message();
    let y: Fr = read(&mut v)?;
    let mut indices = challenge_indices(
        &mut v,
        cfg.codeword_length(),
        cfg.in_domain_samples(),
        false,
    );
    indices.sort_unstable();
    indices.dedup();
    let auth_len = frontier_hash_count(&indices, cfg.codeword_length(), depth)?;
    let mut openings = Vec::new();
    let mut roots = Vec::new();
    let mut hashes = 0;
    for _ in sources {
        let fields: Vec<Fr> = v
            .prover_hint_ark()
            .map_err(|e| anyhow::anyhow!("bottom values: {e:?}"))?;
        ensure!(
            fields.len() == indices.len() * cfg.interleaving_depth(),
            "bottom opening shape"
        );
        let auth = (0..auth_len)
            .map(|_| {
                v.prover_hint()
                    .map_err(|e| anyhow::anyhow!("bottom path: {e:?}"))
            })
            .collect::<Result<Vec<Hash>>>()?;
        let (root_values, count) =
            forest_verify(&indices, &fields, &auth, cfg.interleaving_depth(), depth)?;
        roots.push(root_values);
        hashes += count;
        openings.push(fields);
    }
    let lambda: Fr = v.verifier_message();
    let (form, sigma) = batch_claim(
        point,
        alpha,
        y,
        lambda,
        &values,
        &weights,
        indices.clone(),
        &openings,
        &cfg,
    );
    ensure!(
        form == expected.large_form && sigma == expected.large_sigma,
        "ifold large SEP differs"
    );
    for ((d, roots), claim) in sources.iter().zip(&roots).zip(&expected.residual_claims) {
        let gamma: Fr = v.verifier_message();
        let (form, sigma) = canopy_claim_at(&d.layer_point, d.layer_value, roots, gamma);
        ensure!(
            form == claim.0 && sigma == claim.1,
            "ifold residual SEP differs"
        );
    }
    let work = authentication_work_by_height(&indices, cfg.codeword_length())?;
    ensure!(
        hashes == ell_cast * (indices.len() + work[depth].parents),
        "bottom hash accounting failed"
    );
    v.check_eof()
        .map_err(|e| anyhow::anyhow!("trailing ifold data: {e:?}"))?;
    Ok(hashes)
}

/// Low-level all-fresh wrapper used by isolated protocol tests.
#[cfg(test)]
#[allow(clippy::too_many_arguments)]
fn run_ifold(
    log_n: usize,
    source_count: usize,
    height: usize,
    log_width: usize,
    security_bits: f64,
    seed: u64,
    full_check: bool,
) -> Result<IfoldExecution> {
    prepare_and_run_ifold(
        log_n,
        source_count,
        false,
        height,
        log_width,
        security_bits,
        seed,
        full_check,
    )
}

/// A steady-state step absorbs fresh claims and the previous Canopy accumulator.
#[allow(clippy::too_many_arguments)]
fn run_ifold_with_prior(
    log_n: usize,
    fresh_count: usize,
    height: usize,
    log_width: usize,
    security_bits: f64,
    seed: u64,
    full_check: bool,
) -> Result<IfoldExecution> {
    prepare_and_run_ifold(
        log_n,
        fresh_count,
        true,
        height,
        log_width,
        security_bits,
        seed,
        full_check,
    )
}

#[allow(clippy::too_many_arguments)]
fn prepare_and_run_ifold(
    log_n: usize,
    fresh_count: usize,
    include_prior: bool,
    height: usize,
    log_width: usize,
    security_bits: f64,
    seed: u64,
    full_check: bool,
) -> Result<IfoldExecution> {
    ensure!(fresh_count > 0, "ifold needs a fresh input");
    let source_count = fresh_count
        .checked_add(usize::from(include_prior))
        .ok_or_else(|| anyhow::anyhow!("ifold arity overflow"))?;
    let cfg = commitment_config(log_n, 1, log_width, security_bits)?;
    ensure!(
        height > 1 && height >= log_width && height < log_n + 1,
        "invalid Canopy height"
    );
    let layer_log = log_n + 1 - height;
    let layer_cfg = commitment_config(layer_log, height - 1, log_width, security_bits)?;
    let claims = (0..source_count)
        .map(|j| {
            let m = message(log_n, seed, 1000 + j as u64);
            let form = if j < fresh_count {
                Form::Mle(point(log_n, seed, 2000 + j as u64))
            } else {
                // A previous ifold output has a general batching covector, not
                // a fresh MLE claim. Its preparation stays outside prover time.
                prior_form(&cfg, seed, 9000)
            };
            let sigma = m.dot(&form.materialize());
            (m, form, sigma)
        })
        .collect::<Vec<_>>();
    // The fresh batch and the carried state are timed independently. Do not
    // infer fresh cost by prorating their joint commitment preparation.
    let mut claims = claims.into_iter();
    let mut sources = Vec::with_capacity(source_count);
    let mut fresh_profile = CommitmentProfile::default();
    let t = Instant::now();
    sources.extend(
        claims
            .by_ref()
            .take(fresh_count)
            .map(|(message, form, sigma)| {
                let data =
                    prepare_canopy_profiled(&message, &cfg, &layer_cfg, height, &mut fresh_profile);
                CanopySource {
                    message,
                    data,
                    form,
                    sigma,
                }
            }),
    );
    let fresh_seconds = t.elapsed().as_secs_f64();
    let mut prior_profile = CommitmentProfile::default();
    let prior_seconds = if include_prior {
        let t = Instant::now();
        sources.extend(claims.map(|(message, form, sigma)| {
            let data =
                prepare_canopy_profiled(&message, &cfg, &layer_cfg, height, &mut prior_profile);
            CanopySource {
                message,
                data,
                form,
                sigma,
            }
        }));
        t.elapsed().as_secs_f64()
    } else {
        0.0
    };
    let mut incoming_profile = prior_profile;
    incoming_profile.merge(&fresh_profile);
    let mut execution = execute_ifold(
        &cfg,
        &layer_cfg,
        height,
        sources,
        fresh_seconds + prior_seconds,
        incoming_profile,
        full_check,
    )?;
    fresh_profile.set_fresh(fresh_count, fresh_seconds, &mut execution.measurement);
    execution.measurement.prior_commit_seconds = prior_seconds;
    Ok(execution)
}

/// Execute the reduction on supplied witnesses so repeated steps can carry
/// the actual previous Canopy output, including its compact SEP functional.
fn execute_ifold(
    cfg: &CommitmentConfig,
    layer_cfg: &CommitmentConfig,
    height: usize,
    sources: Vec<CanopySource>,
    incoming: f64,
    incoming_profile: CommitmentProfile,
    full_check: bool,
) -> Result<IfoldExecution> {
    let ell_cast = sources.len();
    ensure!(ell_cast > 0, "ifold needs an input");
    let log_n = cfg.vector_size().ilog2() as usize;
    let log_width = cfg.interleaving_depth().ilog2() as usize;
    ensure!(
        height > 1
            && height >= log_width
            && height <= log_n
            && cfg.size() == 2 * cfg.vector_size()
            && layer_cfg.size() == cfg.vector_size()
            && layer_cfg.vector_size() == (2 * cfg.vector_size()) >> height
            && layer_cfg.interleaving_depth() == cfg.interleaving_depth(),
        "incompatible ifold RS geometry"
    );
    let mut messages = Vec::with_capacity(ell_cast);
    let mut forms = Vec::with_capacity(ell_cast);
    let mut sigmas = Vec::with_capacity(ell_cast);
    let mut data = Vec::with_capacity(ell_cast);
    for source in sources {
        ensure!(
            source.message.len() == cfg.vector_size(),
            "ifold message shape"
        );
        ensure!(source.data.base.len() == cfg.size(), "ifold codeword shape");
        source.form.validate(cfg)?;
        messages.push(source.message);
        forms.push(source.form);
        sigmas.push(source.sigma);
        data.push(source.data);
    }
    let public_inputs = data
        .iter()
        .zip(&forms)
        .zip(&sigmas)
        .map(|((d, f), &sigma)| CanopyPublicInput {
            root: d.small.root,
            layer_point: d.layer_point.clone(),
            layer_value: d.layer_value,
            form: f.clone(),
            sigma,
        })
        .collect::<Vec<_>>();
    let mut input = PublicInput::new();
    bind_canopy_inputs(&mut input, &public_inputs);
    let ds = DomainSeparator::protocol(&(&cfg, &layer_cfg, height, ell_cast))
        .session(&"section8/ifold/v1")
        .instance(&input);
    let mut times = ReductionMeasurement {
        scheme: "ifold_canopy".into(),
        log_n,
        sources: ell_cast,
        height: Some(height),
        queries: cfg.in_domain_samples(),
        incoming_commit_seconds: incoming,
        ..Default::default()
    };
    incoming_profile.set_incoming(&mut times);
    let start = Instant::now();
    let mut p = P::new(&ds, Sponge::default());
    let t = Instant::now();
    let prover_forms = forms.iter().map(ProverForm::from_form).collect::<Vec<_>>();
    times.form_seconds = t.elapsed().as_secs_f64();
    let mut prefix = prefix_prove(
        &cfg,
        &mut p,
        &messages.iter().collect::<Vec<_>>(),
        &data.iter().map(|d| &d.base).collect::<Vec<_>>(),
        prover_forms,
        &sigmas,
        &mut times,
    );
    let t = Instant::now();
    let mut output_profile = CommitmentProfile::default();
    let output = canopy_commit_prehashed_profiled(
        std::mem::take(&mut prefix.matrix),
        &cfg,
        &layer_cfg,
        height,
        &mut p,
        prefix.leaves.take(),
        &mut output_profile,
    );
    let ood_start = Instant::now();
    let alpha: Fr = p.verifier_message();
    let y = prefix
        .message
        .mixed_univariate_evaluate(&Identity::new(), alpha);
    p.prover_message(&y);
    times.output_ood_seconds = ood_start.elapsed().as_secs_f64();
    times.output_commit_seconds = t.elapsed().as_secs_f64();
    output_profile.set_output(&mut times);
    let t = Instant::now();
    let mut indices = challenge_indices(
        &mut p,
        cfg.codeword_length(),
        cfg.in_domain_samples(),
        false,
    );
    indices.sort_unstable();
    indices.dedup();
    let mut openings = Vec::new();
    let mut auth_count = 0;
    for d in &data {
        let (values, auth) = forest_open(d, &indices, cfg.interleaving_depth());
        p.prover_hint_ark(&values);
        for h in &auth {
            p.prover_hint(h);
        }
        auth_count += auth.len();
        openings.push(values);
    }
    times.opening_auth_seconds = t.elapsed().as_secs_f64();
    let lambda: Fr = p.verifier_message();
    let (large_form, large_sigma) = batch_claim(
        prefix.point.clone(),
        alpha,
        y,
        lambda,
        &prefix.values,
        &prefix.weights,
        indices.clone(),
        &openings,
        &cfg,
    );
    let depth = height - log_width;
    let touched = indices.iter().map(|i| i >> depth).collect::<BTreeSet<_>>();
    let mut residual_claims = Vec::new();
    for d in &data {
        let gamma: Fr = p.verifier_message();
        let roots = touched
            .iter()
            .map(|&i| (i, d.small.message.to_slice()[i]))
            .collect::<BTreeMap<_, _>>();
        let (form, sigma) = canopy_claim(d, &roots, gamma);
        residual_claims.push((form, sigma));
    }
    times.opening_seconds = t.elapsed().as_secs_f64();
    times.opening_claim_seconds =
        phase_remainder(times.opening_seconds, times.opening_auth_seconds);
    let t = Instant::now();
    let proof = p.proof();
    times.serialization_seconds = t.elapsed().as_secs_f64();
    times.step_seconds = start.elapsed().as_secs_f64();
    times.finish_phases();
    let public_output = IfoldPublicOutput {
        root: output.small.root,
        layer_point: output.layer_point.clone(),
        layer_value: output.layer_value,
        large_form: large_form.clone(),
        large_sigma,
        residual_claims: residual_claims.clone(),
    };
    let verify = |proof: &Proof| {
        verify_ifold_proof(
            &cfg,
            &layer_cfg,
            height,
            &public_inputs,
            &public_output,
            proof,
        )
    };
    let (hashes, fs) = verify_fs(|| verify(&proof), log_n + 5)?;
    times.auth_hashes = hashes;
    times.fs_finalizations = fs;
    measure_proof(
        &mut times,
        &proof,
        HASH_BYTES,
        1,
        2 * log_n + ell_cast + 2,
        ell_cast * indices.len() * cfg.interleaving_depth(),
        auth_count,
    );
    ensure!(
        prefix.message.dot(&large_form.materialize()) == large_sigma,
        "ifold large output witness failed"
    );
    for (d, (form, sigma)) in data.iter().zip(&residual_claims) {
        ensure!(
            d.small.message.dot(&form.materialize()) == *sigma,
            "ifold residual witness failed"
        );
    }
    if full_check {
        ensure!(
            encode(&cfg, &prefix.message) == output.base,
            "ifold output RS codeword failed"
        );
        let checked = prepare_canopy(&prefix.message, &cfg, &layer_cfg, height);
        ensure!(
            checked.small.root == output.small.root
                && checked.small.witness.matrix == output.small.witness.matrix,
            "ifold output Canopy witness failed"
        );
        let mut bad = proof.clone();
        bad.narg_string[0] ^= 1;
        ensure!(
            tampering_rejected(|| verify(&bad)),
            "changed ifold sumcheck accepted"
        );
        let output_offset = (2 * log_n + ell_cast) * FIELD_BYTES;
        let mut bad = proof.clone();
        bad.narg_string[output_offset] ^= 1;
        ensure!(
            tampering_rejected(|| verify(&bad)),
            "changed ifold output root accepted"
        );
        let mut bad = proof.clone();
        bad.narg_string[output_offset + HASH_BYTES] ^= 1;
        ensure!(
            tampering_rejected(|| verify(&bad)),
            "changed Canopy commitment OOD accepted"
        );
        let mut bad = proof.clone();
        bad.narg_string[output_offset + HASH_BYTES + FIELD_BYTES] ^= 1;
        ensure!(
            tampering_rejected(|| verify(&bad)),
            "changed ifold code-switch OOD accepted"
        );
        let mut bad = proof.clone();
        bad.hints[8] ^= 1; // Skip the first bottom-value vector length.
        ensure!(
            tampering_rejected(|| verify(&bad)),
            "changed bottom queried field accepted"
        );
        let mut bad = proof.clone();
        let last = bad.hints.len() - 1;
        bad.hints[last] ^= 1;
        ensure!(
            tampering_rejected(|| verify(&bad)),
            "changed bottom proof accepted"
        );
        let mut bad = proof.clone();
        bad.narg_string.push(0);
        ensure!(
            tampering_rejected(|| verify(&bad)),
            "trailing ifold proof accepted"
        );
    }
    times.verified = true;
    // Deferred claims reuse the incoming Canopy layer witnesses by ownership
    // transfer; no full codeword or Merkle-tree copy is necessary.
    let residual = data
        .into_iter()
        .zip(residual_claims)
        .map(|(d, (form, sigma))| {
            let mut s = d.small;
            s.form = form;
            s.sigma = sigma;
            s
        })
        .collect::<Vec<_>>();
    Ok(IfoldExecution {
        measurement: times,
        residual,
        public_inputs,
        public_output,
        output_message: prefix.message,
        output,
        proof,
    })
}

/// Execute a steady-state accumulation step: `ell_cast` fresh Canopy claims
/// plus one previous Canopy claim produce `ell_cast + 1` actual residuals;
/// `iacc` combines those with `ell_acc` previous Merkle accumulator claims.
#[allow(clippy::too_many_arguments)]
pub fn run_canopy(
    log_n: usize,
    ell_cast: usize,
    ell_acc: usize,
    height: usize,
    log_width: usize,
    security_bits: f64,
    seed: u64,
    full_check: bool,
) -> Result<CanopyRun> {
    ell_cast
        .checked_add(1)
        .and_then(|sources| sources.checked_add(ell_acc))
        .ok_or_else(|| anyhow::anyhow!("accumulation arity overflow"))?;
    let execution = run_ifold_with_prior(
        log_n,
        ell_cast,
        height,
        log_width,
        security_bits,
        seed,
        full_check,
    )?;
    let IfoldExecution {
        measurement: times,
        mut residual,
        public_inputs,
        public_output,
        output_message,
        output,
        proof,
    } = execution;
    // Release ifold's retained audit artifacts before constructing iacc. Only
    // incoming layer witnesses and their actual deferred claims survive.
    drop((public_inputs, public_output, output_message, output, proof));
    let layer_log = log_n + 1 - height;
    let layer_cfg = commitment_config(layer_log, height - 1, log_width, security_bits)?;
    let prior_messages = (0..ell_acc)
        .map(|j| message(layer_log, seed, 3000 + j as u64))
        .collect::<Vec<_>>();
    let prior_claims = prior_messages
        .into_iter()
        .enumerate()
        .map(|(j, m)| {
            let f = prior_form(&layer_cfg, seed, 4000 + 10 * j as u64);
            let s = m.dot(&f.materialize());
            (m, f, s)
        })
        .collect::<Vec<_>>();
    let mut incoming_profile = CommitmentProfile::default();
    let mut priors = Vec::with_capacity(ell_acc);
    let prior_seconds = if ell_acc > 0 {
        let t = Instant::now();
        priors.extend(prior_claims.into_iter().map(|(m, f, s)| {
            prepare_with_sigma_profiled(&layer_cfg, m, f, s, &mut incoming_profile)
        }));
        t.elapsed().as_secs_f64()
    } else {
        0.0
    };
    residual.extend(priors);
    let (mut iacc, _, _) = run_batch(
        &layer_cfg,
        &residual,
        "iacc_merkle",
        prior_seconds,
        full_check,
    )?;
    incoming_profile.set_incoming(&mut iacc);
    iacc.prior_commit_seconds = prior_seconds;
    iacc.height = Some(height);
    Ok(CanopyRun { ifold: times, iacc })
}

/// Ordinary Merkle batching control, at the same message length and arities.
#[allow(clippy::too_many_arguments)]
pub fn run_merkle(
    log_n: usize,
    fresh_count: usize,
    prior_count: usize,
    log_inv_rate: usize,
    log_width: usize,
    security_bits: f64,
    seed: u64,
    full_check: bool,
) -> Result<ReductionMeasurement> {
    ensure!(
        (1..=2).contains(&log_inv_rate),
        "Merkle controls support RS rates 1/2 and 1/4"
    );
    let source_count = fresh_count
        .checked_add(prior_count)
        .ok_or_else(|| anyhow::anyhow!("batch arity overflow"))?;
    ensure!(source_count > 0, "batch needs an input");
    let cfg = commitment_config(log_n, log_inv_rate, log_width, security_bits)?;
    let messages = (0..source_count)
        .map(|j| message(log_n, seed, 5000 + j as u64))
        .collect::<Vec<_>>();
    let claims = messages
        .into_iter()
        .enumerate()
        .map(|(j, m)| {
            let form = if j < fresh_count {
                Form::Mle(point(log_n, seed, 6000 + j as u64))
            } else {
                prior_form(&cfg, seed, 7000 + 10 * j as u64)
            };
            let sigma = m.dot(&form.materialize());
            (m, form, sigma)
        })
        .collect::<Vec<_>>();
    let mut claims = claims.into_iter();
    let mut sources = Vec::with_capacity(source_count);
    let mut fresh_profile = CommitmentProfile::default();
    let fresh_seconds = if fresh_count > 0 {
        let t = Instant::now();
        sources.extend(
            claims
                .by_ref()
                .take(fresh_count)
                .map(|(m, f, s)| prepare_with_sigma_profiled(&cfg, m, f, s, &mut fresh_profile)),
        );
        t.elapsed().as_secs_f64()
    } else {
        0.0
    };
    let mut prior_profile = CommitmentProfile::default();
    let prior_seconds = if prior_count > 0 {
        let t = Instant::now();
        sources.extend(
            claims.map(|(m, f, s)| prepare_with_sigma_profiled(&cfg, m, f, s, &mut prior_profile)),
        );
        t.elapsed().as_secs_f64()
    } else {
        0.0
    };
    let mut incoming_profile = prior_profile;
    incoming_profile.merge(&fresh_profile);
    let incoming = fresh_seconds + prior_seconds;
    let mut measurement = run_batch(
        &cfg,
        &sources,
        if log_inv_rate == 1 {
            "merkle_rate_half"
        } else {
            "merkle_rate_quarter"
        },
        incoming,
        full_check,
    )?
    .0;
    incoming_profile.set_incoming(&mut measurement);
    fresh_profile.set_fresh(fresh_count, fresh_seconds, &mut measurement);
    measurement.prior_commit_seconds = prior_seconds;
    Ok(measurement)
}

#[cfg(test)]
#[path = "section8_tests.rs"]
mod tests;
