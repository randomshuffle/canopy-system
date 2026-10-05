//! Executable one-step cost prototype: native WHIR first fold plus a
//! same-code three-source accumulator, against a same-code two-source baseline.
//! No recursive SNARK, final proof, or incoming commitment is charged to step time.
use anyhow::{ensure, Result};
use ark_ff::Field;
use canopy_system::{
    additive_rs, benchmark_field_element as elem,
    canopy::{CanopyVectorCommitment, WhirCanopyProverData},
    executable_whir::{encode_interleaved_with_whir, WhirEncodedMatrix},
    merkle::{authentication_work_by_height, frontier_hash_count},
    register_benchmark_ntt, Fr,
};
use clap::Parser;
use rayon::prelude::*;
use serde::Serialize;
use std::{collections::BTreeMap, path::PathBuf, time::Instant};
use whir_ref::{
    algebra::{embedding::Identity, multilinear_extend},
    buffer::{Buffer, BufferMath, BufferOps},
    hash::{self, Hash},
    parameters::ProtocolParameters,
    protocols::{
        challenge_indices::challenge_indices, irs_commit, matrix_commit, params::DecodingRegime,
        whir,
    },
    transcript::{
        codecs::Empty, DomainSeparator, Proof, ProverState, VerifierMessage, VerifierState,
    },
};
#[cfg(not(feature = "fs-accounting"))]
type Sponge = spongefish::instantiations::XOF<blake3::Hasher>;
#[cfg(feature = "fs-accounting")]
type Sponge = canopy_system::fiat_shamir::RecordingBlake3;
type P = ProverState<Sponge>;
type V<'a> = VerifierState<'a, Sponge>;
type Config = irs_commit::Config<Identity<Fr>>;
type Witness = irs_commit::Witness<Fr>;
type Commitment = irs_commit::Commitment;
const WIDTH: usize = 8;
const FOLD: usize = 3;
const HEIGHT: usize = 4;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "18,20,22", value_delimiter = ',')]
    logs: Vec<usize>,
    #[arg(long, default_value_t = 1)]
    warmups: usize,
    #[arg(long, default_value_t = 5)]
    repetitions: usize,
    #[arg(long)]
    output: PathBuf,
    #[arg(long)]
    check_tampering: bool,
}
#[derive(Serialize, Default, Clone)]
struct Row {
    log_n: usize,
    scheme: String,
    repetition: usize,
    queries_0: usize,
    queries_1: usize,
    incoming_commit_seconds: f64,
    initial_form_seconds: f64,
    outer_ood_and_fold_seconds: f64,
    folded_commit_seconds: f64,
    bottom_open_seconds: f64,
    reduction_forms_seconds: f64,
    batch_sumcheck_seconds: f64,
    combination_seconds: f64,
    output_commit_seconds: f64,
    source_open_and_output_claim_seconds: f64,
    step_seconds: f64,
    with_incoming_commit_seconds: f64,
    actual_auth_hashes: usize,
    fs_finalizations: usize,
    proof_bytes: usize,
    verified: bool,
}
fn read<T: whir_ref::transcript::ProverMessage>(v: &mut V) -> Result<T> {
    v.prover_message()
        .map_err(|e| anyhow::anyhow!("transcript: {e:?}"))
}
fn powers(base: Fr, start: Fr, n: usize) -> Vec<Fr> {
    (0..n)
        .scan(start, |p, _| {
            let v = *p;
            *p *= base;
            Some(v)
        })
        .collect()
}
fn point(log: usize, tag: u64) -> Vec<Fr> {
    (0..log)
        .map(|i| elem(73947765755993, tag, i as u64))
        .collect()
}
fn add(a: &mut Buffer<Fr>, b: Buffer<Fr>) {
    *a = Buffer::from(
        a.to_slice()
            .par_iter()
            .zip(b.to_slice())
            .map(|(&x, &y)| x + y)
            .collect::<Vec<_>>(),
    );
}
fn geom(a: &mut Buffer<Fr>, alpha: Fr, weight: Fr) {
    a.accumulate_geometric(&[alpha], &Buffer::from(vec![weight]), a.len());
}
fn config(log: usize) -> whir::Config<Identity<Fr>> {
    whir::Config::new(
        1 << log,
        &ProtocolParameters {
            decoding_regime: DecodingRegime::Johnson,
            starting_log_inv_rate: 1,
            initial_folding_factor: FOLD,
            folding_factor: FOLD,
            security_level: 136,
            pow_bits: 0,
            batch_size: 1,
            hash_id: hash::BLAKE3,
        },
    )
}

// Compact outgoing SEP covector: eq(r,.) + lambda*univariate(alpha,.) +
// column-wise source queries weighted lambda^(2 + row*width + column).
// Each polynomial column occupies a contiguous block of message coefficients.
#[derive(Clone, Debug, PartialEq)]
struct AccForm {
    point: Vec<Fr>,
    alpha: Fr,
    lambda: Fr,
    indices: Vec<usize>,
    rows: usize,
    width: usize,
}
impl AccForm {
    fn materialize(&self) -> Buffer<Fr> {
        let n = 1 << self.point.len();
        let mut a = Buffer::eq_weights(&self.point);
        geom(&mut a, self.alpha, self.lambda);
        let by_row = powers(
            self.lambda.pow([self.width as u64]),
            self.lambda * self.lambda,
            self.indices.len(),
        );
        let low = additive_rs::power_sums(&self.indices, &by_row, self.rows, n / self.width);
        let cols = powers(self.lambda, Fr::ONE, self.width);
        let extra = Buffer::from(
            (0..n)
                .into_par_iter()
                .map(|i| cols[i / (n / self.width)] * low.to_slice()[i % (n / self.width)])
                .collect::<Vec<_>>(),
        );
        add(&mut a, extra);
        a
    }
}
fn prior_form(cfg: &Config) -> AccForm {
    AccForm {
        point: point(cfg.vector_size().ilog2() as usize, 101),
        alpha: elem(17, 3, 0),
        lambda: elem(17, 4, 0),
        indices: (0..cfg.in_domain_samples())
            .map(|i| (i * 7919 + 51) % cfg.codeword_length())
            .collect(),
        rows: cfg.codeword_length(),
        width: cfg.interleaving_depth(),
    }
}
struct Prepared {
    message: Buffer<Fr>,
    witness: Witness,
    root: Hash,
}
fn prepare(cfg: &Config, message: Buffer<Fr>) -> Prepared {
    let ds = DomainSeparator::protocol(cfg)
        .session(&"accumulation/setup/v1")
        .instance(&Empty);
    let mut p = P::new(&ds, Sponge::default());
    let witness = cfg.commit(&mut p, &[&message]);
    let proof = p.proof();
    let root = Hash(proof.narg_string[..32].try_into().unwrap());
    Prepared {
        message,
        witness,
        root,
    }
}
fn bind(p: &mut P, roots: &[Hash], fields: &[Fr]) {
    p.prover_messages(roots);
    p.prover_messages(fields);
}
fn receive_bind(v: &mut V, cfg: &Config, roots: &[Hash], fields: &[Fr]) -> Result<Vec<Commitment>> {
    // Public roots and scalars are bound to the transcript; verify equality.
    let mut out = Vec::new();
    for root in roots {
        let got: Hash = read(v)?;
        ensure!(got == *root, "public root changed");
        // Decode the same native root into the commitment type, whose field is private.
        let ds = DomainSeparator::protocol(cfg)
            .session(&"root-decode")
            .instance(&Empty);
        let mut p = P::new(&ds, Sponge::default());
        p.prover_message(root);
        let proof = p.proof();
        let mut local = V::new(&ds, &proof, Sponge::default());
        out.push(
            cfg.receive_commitment(&mut local)
                .map_err(|e| anyhow::anyhow!("root decode {e:?}"))?,
        );
    }
    for x in fields {
        ensure!(read::<Fr>(v)? == *x, "public field changed");
    }
    Ok(out)
}
fn root_from_commitment_proof(cfg: &Config, root: Hash) -> Commitment {
    let ds = DomainSeparator::protocol(cfg)
        .session(&"root-decode")
        .instance(&Empty);
    let mut p = P::new(&ds, Sponge::default());
    p.prover_message(&root);
    let proof = p.proof();
    let mut v = V::new(&ds, &proof, Sponge::default());
    cfg.receive_commitment(&mut v).unwrap()
}

struct BatchOutput {
    message: Buffer<Fr>,
    witness: Witness,
    form: AccForm,
    sigma: Fr,
}
fn batch_prove(
    cfg: &Config,
    p: &mut P,
    messages: &[&Buffer<Fr>],
    witnesses: &[&Witness],
    forms: &[Buffer<Fr>],
    sigmas: &[Fr],
    times: &mut Row,
) -> BatchOutput {
    let start = Instant::now();
    p.prover_messages(sigmas);
    let beta: Fr = p.verifier_message();
    let bs = powers(beta, Fr::ONE, messages.len());
    let mut sum = sigmas.iter().zip(&bs).map(|(&s, &b)| s * b).sum::<Fr>();
    let mut pairs = messages
        .iter()
        .zip(forms)
        .map(|(m, f)| ((*m).clone(), f.clone()))
        .collect::<Vec<_>>();
    let mut r = Vec::new();
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
        let x: Fr = p.verifier_message();
        sum = (c * x + sum - c) * x + a;
        for (m, f) in &mut pairs {
            m.fold_pair(f, x);
        }
        r.push(x);
    }
    let ys = pairs
        .iter()
        .map(|(m, _)| m.to_slice()[0])
        .collect::<Vec<_>>();
    p.prover_messages(&ys);
    let gamma: Fr = p.verifier_message();
    times.batch_sumcheck_seconds = start.elapsed().as_secs_f64();
    let start = Instant::now();
    let gs = powers(gamma, Fr::ONE, messages.len());
    let message = Buffer::from(
        (0..cfg.vector_size())
            .into_par_iter()
            .map(|i| {
                messages
                    .iter()
                    .zip(&gs)
                    .map(|(m, &g)| m.to_slice()[i] * g)
                    .sum::<Fr>()
            })
            .collect::<Vec<_>>(),
    );
    times.combination_seconds = start.elapsed().as_secs_f64();
    let start = Instant::now();
    let (witness, ood) = cfg.commit_with_ood(p, &[&message], 1);
    times.output_commit_seconds = start.elapsed().as_secs_f64();
    let start = Instant::now();
    let eval = cfg.open(p, witnesses);
    let lambda: Fr = p.verifier_message();
    let weights = powers(
        lambda,
        lambda * lambda,
        eval.points.len() * cfg.interleaving_depth(),
    );
    let sigma = ys.iter().zip(&gs).map(|(&y, &g)| y * g).sum::<Fr>()
        + lambda * ood.matrix.to_slice()[0]
        + eval
            .rows()
            .zip(weights.chunks(cfg.interleaving_depth()))
            .map(|(row, ws)| {
                (0..cfg.interleaving_depth())
                    .map(|c| {
                        ws[c]
                            * gs.iter()
                                .enumerate()
                                .map(|(j, &g)| g * row[j * cfg.interleaving_depth() + c])
                                .sum::<Fr>()
                    })
                    .sum::<Fr>()
            })
            .sum::<Fr>();
    let form = AccForm {
        point: r,
        alpha: ood.points[0],
        lambda,
        indices: eval.domain.unwrap().exponents,
        rows: cfg.codeword_length(),
        width: cfg.interleaving_depth(),
    };
    times.source_open_and_output_claim_seconds = start.elapsed().as_secs_f64();
    BatchOutput {
        message,
        witness,
        form,
        sigma,
    }
}
fn batch_verify(
    cfg: &Config,
    v: &mut V,
    commitments: &[Commitment],
    forms: &[Buffer<Fr>],
    sigmas: &[Fr],
) -> Result<(Commitment, AccForm, Fr, usize)> {
    for s in sigmas {
        ensure!(read::<Fr>(v)? == *s, "batch input claim");
    }
    let beta: Fr = v.verifier_message();
    let bs = powers(beta, Fr::ONE, forms.len());
    let mut sum = sigmas.iter().zip(&bs).map(|(&s, &b)| s * b).sum::<Fr>();
    let mut r = Vec::new();
    for _ in 0..cfg.vector_size().ilog2() {
        let a: Fr = read(v)?;
        let c: Fr = read(v)?;
        let x: Fr = v.verifier_message();
        sum = (c * x + sum - c) * x + a;
        r.push(x);
    }
    let ys = (0..forms.len())
        .map(|_| read::<Fr>(v))
        .collect::<Result<Vec<_>>>()?;
    ensure!(
        sum == forms
            .iter()
            .zip(&ys)
            .zip(&bs)
            .map(|((f, &y), &b)| multilinear_extend(f.to_slice(), &r) * y * b)
            .sum::<Fr>(),
        "batch sumcheck failed"
    );
    let gamma: Fr = v.verifier_message();
    let gs = powers(gamma, Fr::ONE, forms.len());
    let (commitment, ood) = cfg
        .receive_commitment_with_ood(v, 1)
        .map_err(|e| anyhow::anyhow!("batch output {e:?}"))?;
    hash::HASH_COUNTER.reset();
    let eval = cfg
        .verify(v, &commitments.iter().collect::<Vec<_>>())
        .map_err(|e| anyhow::anyhow!("batch paths {e:?}"))?;
    let hashes = hash::HASH_COUNTER.get();
    let lambda: Fr = v.verifier_message();
    let weights = powers(
        lambda,
        lambda * lambda,
        eval.points.len() * cfg.interleaving_depth(),
    );
    let sigma = ys.iter().zip(&gs).map(|(&y, &g)| y * g).sum::<Fr>()
        + lambda * ood.matrix.to_slice()[0]
        + eval
            .rows()
            .zip(weights.chunks(cfg.interleaving_depth()))
            .map(|(row, ws)| {
                (0..cfg.interleaving_depth())
                    .map(|c| {
                        ws[c]
                            * gs.iter()
                                .enumerate()
                                .map(|(j, &g)| g * row[j * cfg.interleaving_depth() + c])
                                .sum::<Fr>()
                    })
                    .sum::<Fr>()
            })
            .sum::<Fr>();
    let indices = eval.domain.unwrap().exponents;
    let expected = forms.len()
        * (cfg.in_domain_samples()
            + authentication_work_by_height(&indices, cfg.codeword_length())?
                .last()
                .unwrap()
                .parents);
    ensure!(
        hashes == expected,
        "native hash accounting differs: {hashes} vs {expected}"
    );
    Ok((
        commitment,
        AccForm {
            point: r,
            alpha: ood.points[0],
            lambda,
            indices,
            rows: cfg.codeword_length(),
            width: cfg.interleaving_depth(),
        },
        sigma,
        hashes,
    ))
}

fn check_output(
    cfg: &Config,
    out: &BatchOutput,
    verified: &(Commitment, AccForm, Fr, usize),
    full_decider: bool,
) -> Result<()> {
    ensure!(
        out.form == verified.1 && out.sigma == verified.2,
        "output SEP differs"
    );
    ensure!(
        out.message.dot(&out.form.materialize()) == out.sigma,
        "output SEP witness invalid"
    );
    if full_decider {
        let prepared = prepare(cfg, out.message.clone());
        ensure!(
            root_from_commitment_proof(cfg, prepared.root) == verified.0,
            "output commitment witness invalid"
        );
        ensure!(
            prepared.witness.matrix == out.witness.matrix,
            "output RS codeword invalid"
        );
    }
    Ok(())
}

// Specialized native WHIR forest reconstruction for h=4 and row width 8:
// one leaf hash per queried distinct row and one parent per touched pair.
fn bottom_roots(indices: &[usize], values: &[Fr], auth: &[Hash]) -> Result<BTreeMap<usize, Fr>> {
    ensure!(values.len() == indices.len() * WIDTH, "bottom value shape");
    let engine = hash::ENGINES.retrieve(hash::BLAKE3).unwrap();
    let mut leaf_hashes = vec![Hash::default(); indices.len()];
    matrix_commit::hash_rows(&*engine, values, &mut leaf_hashes);
    let leaves = indices
        .iter()
        .copied()
        .zip(leaf_hashes)
        .collect::<BTreeMap<_, _>>();
    let mut roots = BTreeMap::new();
    let mut cursor = 0;
    for (&i, &h) in &leaves {
        if roots.contains_key(&(i / 2)) {
            continue;
        }
        let sibling = if let Some(h) = leaves.get(&(i ^ 1)) {
            *h
        } else {
            ensure!(cursor < auth.len(), "short auth");
            let h = auth[cursor];
            cursor += 1;
            h
        };
        let mut bytes = [0; 64];
        let (a, b) = if i % 2 == 0 {
            (h, sibling)
        } else {
            (sibling, h)
        };
        bytes[..32].copy_from_slice(&a.0);
        bytes[32..].copy_from_slice(&b.0);
        roots.insert(i / 2, Fr::from_bytes(*blake3::hash(&bytes).as_bytes()));
    }
    ensure!(cursor == auth.len(), "extra auth");
    Ok(roots)
}
fn canopy_form(
    n: usize,
    ood_point: &[Fr],
    ood_value: Fr,
    roots: &BTreeMap<usize, Fr>,
    gamma: Fr,
) -> (Buffer<Fr>, Fr) {
    let mut values = Buffer::eq_weights(ood_point).to_slice().to_vec();
    let mut sum = ood_value;
    let mut w = gamma;
    for (&i, &value) in roots {
        values[i] += w;
        sum += w * value;
        w *= gamma;
    }
    assert_eq!(values.len(), n);
    (Buffer::from(values), sum)
}
fn fold_form(
    form: &mut Buffer<Fr>,
    sum: &mut Fr,
    ood: &irs_commit::Evaluations<Fr>,
    indices: &[usize],
    opened: &[Fr],
    fold: &[Fr],
    rows: usize,
    gamma: Fr,
) {
    let weights = powers(gamma, gamma, 1 + indices.len());
    geom(form, ood.points[0], weights[0]);
    *sum += weights[0] * ood.matrix.to_slice()[0];
    let fold_weights = Buffer::eq_weights(fold);
    let answers = opened
        .chunks(WIDTH)
        .map(|row| {
            row.iter()
                .zip(fold_weights.to_slice())
                .map(|(&a, &b)| a * b)
                .sum::<Fr>()
        })
        .collect::<Vec<_>>();
    *sum += answers
        .iter()
        .zip(&weights[1..])
        .map(|(&a, &b)| a * b)
        .sum::<Fr>();
    add(
        form,
        additive_rs::power_sums(indices, &weights[1..], rows, form.len()),
    );
}
struct CanopySetup {
    encoded: WhirEncodedMatrix,
    data: WhirCanopyProverData,
    root: Hash,
    layer_point: Vec<Fr>,
    layer_value: Fr,
    commit_seconds: f64,
}
fn prepare_canopy(message: &Buffer<Fr>) -> Result<CanopySetup> {
    let start = Instant::now();
    let encoded = encode_interleaved_with_whir(message.to_slice(), 1, FOLD)?;
    let (commit, data, _) =
        CanopyVectorCommitment::commit_whir_profiled(&encoded, HEIGHT, 1, FOLD)?;
    let root = Hash(commit.root);
    let ds = DomainSeparator::protocol(&"canopy-commitment-ood/v1")
        .session(&message.len())
        .instance(&Empty);
    let mut p = P::new(&ds, Sponge::default());
    p.prover_message(&root);
    let layer_point = p.verifier_message_vec(data.canopy_message.len().ilog2() as usize);
    let layer_value = multilinear_extend(&data.canopy_message, &layer_point);
    p.prover_message(&layer_value);
    Ok(CanopySetup {
        encoded,
        data,
        root,
        layer_point,
        layer_value,
        commit_seconds: start.elapsed().as_secs_f64(),
    })
}

fn run_baseline(
    native: &whir::Config<Identity<Fr>>,
    message: &Buffer<Fr>,
    prior_message: Buffer<Fr>,
    rep: usize,
    check: bool,
) -> Result<Row> {
    let cfg = &native.initial_committer;
    let n = message.len();
    let start = Instant::now();
    let fresh = prepare(cfg, message.clone());
    let commit_time = start.elapsed().as_secs_f64();
    let prior = prepare(cfg, prior_message);
    let prior_compact = prior_form(cfg);
    let fresh_point = point(n.ilog2() as usize, 99);
    let input_sigmas = [
        multilinear_extend(message.to_slice(), &fresh_point),
        prior.message.dot(&prior_compact.materialize()),
    ];
    let roots = [fresh.root, prior.root];
    let ds = DomainSeparator::protocol(cfg)
        .session(&"accumulation/baseline/v1")
        .instance(&Empty);
    let mut times = Row {
        log_n: n.ilog2() as usize,
        scheme: "merkle_same_code_batch".into(),
        repetition: rep,
        queries_0: cfg.in_domain_samples(),
        queries_1: native.round_configs[0].irs_committer.in_domain_samples(),
        incoming_commit_seconds: commit_time,
        ..Row::default()
    };
    let start = Instant::now();
    let mut p = P::new(&ds, Sponge::default());
    bind(&mut p, &roots, &input_sigmas);
    let t = Instant::now();
    let forms = vec![
        Buffer::eq_weights(&fresh_point),
        prior_compact.materialize(),
    ];
    times.initial_form_seconds = t.elapsed().as_secs_f64();
    let out = batch_prove(
        cfg,
        &mut p,
        &[&fresh.message, &prior.message],
        &[&fresh.witness, &prior.witness],
        &forms,
        &input_sigmas,
        &mut times,
    );
    let proof = p.proof();
    times.step_seconds = start.elapsed().as_secs_f64();
    times.proof_bytes = proof.narg_string.len() + proof.hints.len();
    let verify = |proof: &Proof| -> Result<_> {
        let mut v = V::new(&ds, proof, Sponge::default());
        let cms = receive_bind(&mut v, cfg, &roots, &input_sigmas)?;
        batch_verify(cfg, &mut v, &cms, &forms, &input_sigmas)
    };
    #[cfg(feature = "fs-accounting")]
    let verified = {
        let (res, ledgers) = canopy_system::fiat_shamir::capture(|| verify(&proof));
        times.fs_finalizations = ledgers[0].counts().finalizations;
        res?
    };
    #[cfg(not(feature = "fs-accounting"))]
    let verified = verify(&proof)?;
    check_output(cfg, &out, &verified, check)?;
    if check {
        let mut bad = proof.clone();
        bad.narg_string[roots.len() * 32 + input_sigmas.len() * 32 + input_sigmas.len() * 32] ^= 1;
        ensure!(verify(&bad).is_err(), "tampered sumcheck accepted");
        let mut bad = proof.clone();
        let end = bad.hints.len() - 1;
        bad.hints[end] ^= 1;
        ensure!(verify(&bad).is_err(), "tampered Merkle path accepted");
    }
    times.actual_auth_hashes = verified.3;
    times.verified = true;
    times.with_incoming_commit_seconds = times.step_seconds + times.incoming_commit_seconds;
    Ok(times)
}

fn run_canopy(
    native: &whir::Config<Identity<Fr>>,
    message: &Buffer<Fr>,
    prior_message: Buffer<Fr>,
    rep: usize,
    check: bool,
) -> Result<Row> {
    let cfg0 = &native.initial_committer;
    let round = &native.round_configs[0];
    let cfg1 = &round.irs_committer;
    let n = message.len();
    let canopy = prepare_canopy(message)?;
    let prior = prepare(cfg1, prior_message);
    let prior_compact = prior_form(cfg1);
    ensure!(
        canopy.data.codeword.matrix.len() == cfg1.size()
            && canopy.data.codeword.row_width == cfg1.interleaving_depth(),
        "Canopy must match native second-round code"
    );
    let cm_witness = Witness {
        masks: Buffer::from(Vec::<Fr>::new()),
        matrix: canopy.data.codeword.matrix.clone(),
        matrix_witness: canopy.data.codeword_commitment.witness.clone(),
    };
    let cm_message = Buffer::from(canopy.data.canopy_message.clone());
    let fresh_point = point(n.ilog2() as usize, 99);
    let fresh_sigma = multilinear_extend(message.to_slice(), &fresh_point);
    let prior_sigma = prior.message.dot(&prior_compact.materialize());
    // The incoming claim may equal the old claim; both are valid. Its full
    // instance is bound and carried unchanged, with no reduction of it.
    let roots = [canopy.root, prior.root, canopy.root];
    let mut public = vec![fresh_sigma, prior_sigma, canopy.layer_value];
    public.extend_from_slice(&canopy.layer_point);
    let ds = DomainSeparator::protocol(native)
        .session(&"accumulation/canopy-one-fold/v1")
        .instance(&Empty);
    let mut times = Row {
        log_n: n.ilog2() as usize,
        scheme: "canopy_one_fold_three_claims".into(),
        repetition: rep,
        queries_0: cfg0.in_domain_samples(),
        queries_1: cfg1.in_domain_samples(),
        incoming_commit_seconds: canopy.commit_seconds,
        ..Row::default()
    };
    let start = Instant::now();
    let mut p = P::new(&ds, Sponge::default());
    bind(&mut p, &roots, &public);
    let t = Instant::now();
    let prior_dense = prior_compact.materialize();
    let mut f = Buffer::eq_weights(&fresh_point);
    times.initial_form_seconds = t.elapsed().as_secs_f64();
    let t = Instant::now();
    let alpha: Fr = p.verifier_message();
    let y = message.mixed_univariate_evaluate(&Identity::new(), alpha);
    p.prover_message(&y);
    let beta: Fr = p.verifier_message();
    geom(&mut f, alpha, beta);
    let mut sum = fresh_sigma + beta * y;
    let (folded, fold) = native.initial_sumcheck.prove(
        &mut p,
        &Identity::new(),
        message.clone(),
        &mut f,
        &mut sum,
        &[],
    );
    times.outer_ood_and_fold_seconds = t.elapsed().as_secs_f64();
    let t = Instant::now();
    let (fold_witness, ood) = cfg1.commit_with_ood(&mut p, &[&folded], round.out_domain_samples);
    times.folded_commit_seconds = t.elapsed().as_secs_f64();
    ensure!(round.out_domain_samples == 1, "model expects one OOD");
    let t = Instant::now();
    let mut indices = challenge_indices(
        &mut p,
        cfg0.codeword_length(),
        cfg0.in_domain_samples(),
        false,
    );
    indices.sort_unstable();
    indices.dedup();
    let bottom = CanopyVectorCommitment::open_whir_bottom(&canopy.data, &canopy.encoded, &indices)?;
    p.prover_messages(&bottom.values);
    let auth = bottom
        .auth_hashes
        .iter()
        .map(|h| Hash(*h))
        .collect::<Vec<_>>();
    p.prover_messages(&auth);
    times.bottom_open_seconds = t.elapsed().as_secs_f64();
    let t = Instant::now();
    let gamma: Fr = p.verifier_message();
    fold_form(
        &mut f,
        &mut sum,
        &ood,
        &indices,
        &bottom.values,
        &fold.round_challenges,
        cfg0.codeword_length(),
        gamma,
    );
    // Prover knows the touched roots; rebuilding verifier hashes here would
    // charge unnecessary verifier work to the prover.
    let touched = indices
        .iter()
        .map(|i| i / 2)
        .collect::<std::collections::BTreeSet<_>>();
    let root_values = touched
        .iter()
        .map(|&i| (i, cm_message.to_slice()[i]))
        .collect::<BTreeMap<_, _>>();
    let gamma: Fr = p.verifier_message();
    let (cm_form, cm_sigma) = canopy_form(
        cfg1.vector_size(),
        &canopy.layer_point,
        canopy.layer_value,
        &root_values,
        gamma,
    );
    times.reduction_forms_seconds = t.elapsed().as_secs_f64();
    let forms = vec![f, cm_form, prior_dense];
    let sigmas = [sum, cm_sigma, prior_sigma];
    let out = batch_prove(
        cfg1,
        &mut p,
        &[&folded, &cm_message, &prior.message],
        &[&fold_witness, &cm_witness, &prior.witness],
        &forms,
        &sigmas,
        &mut times,
    );
    let proof = p.proof();
    times.step_seconds = start.elapsed().as_secs_f64();
    times.proof_bytes = proof.narg_string.len() + proof.hints.len();
    let verify = |proof: &Proof| -> Result<_> {
        let mut v = V::new(&ds, proof, Sponge::default());
        let cms = receive_bind(&mut v, cfg1, &roots, &public)?;
        let alpha: Fr = v.verifier_message();
        let y: Fr = read(&mut v)?;
        let beta: Fr = v.verifier_message();
        let mut f = Buffer::eq_weights(&fresh_point);
        geom(&mut f, alpha, beta);
        let mut sum = fresh_sigma + beta * y;
        let fold = native
            .initial_sumcheck
            .verify(&mut v, &mut sum)
            .map_err(|e| anyhow::anyhow!("fold sumcheck {e:?}"))?;
        for &r in &fold.round_challenges {
            f.fold(r);
        }
        let (fold_cm, ood) = cfg1
            .receive_commitment_with_ood(&mut v, round.out_domain_samples)
            .map_err(|e| anyhow::anyhow!("fold root {e:?}"))?;
        let mut indices = challenge_indices(
            &mut v,
            cfg0.codeword_length(),
            cfg0.in_domain_samples(),
            false,
        );
        indices.sort_unstable();
        indices.dedup();
        let values = (0..indices.len() * WIDTH)
            .map(|_| read::<Fr>(&mut v))
            .collect::<Result<Vec<_>>>()?;
        let auth_len = frontier_hash_count(&indices, cfg0.codeword_length(), 1)?;
        let auth = (0..auth_len)
            .map(|_| read::<Hash>(&mut v))
            .collect::<Result<Vec<_>>>()?;
        let reconstructed = bottom_roots(&indices, &values, &auth)?;
        let bottom_hashes = indices.len() + reconstructed.len();
        let gamma: Fr = v.verifier_message();
        fold_form(
            &mut f,
            &mut sum,
            &ood,
            &indices,
            &values,
            &fold.round_challenges,
            cfg0.codeword_length(),
            gamma,
        );
        let gamma: Fr = v.verifier_message();
        let (cm_form, cm_sigma) = canopy_form(
            cfg1.vector_size(),
            &canopy.layer_point,
            canopy.layer_value,
            &reconstructed,
            gamma,
        );
        let forms = vec![f, cm_form, prior_compact.materialize()];
        let mut out = batch_verify(
            cfg1,
            &mut v,
            &[fold_cm, cms[0].clone(), cms[1].clone()],
            &forms,
            &[sum, cm_sigma, prior_sigma],
        )?;
        out.3 += bottom_hashes;
        Ok(out)
    };
    #[cfg(feature = "fs-accounting")]
    let verified = {
        let (res, ledgers) = canopy_system::fiat_shamir::capture(|| verify(&proof));
        times.fs_finalizations = ledgers[0].counts().finalizations;
        res?
    };
    #[cfg(not(feature = "fs-accounting"))]
    let verified = verify(&proof)?;
    check_output(cfg1, &out, &verified, check)?;
    if check {
        let mut bad = proof.clone();
        bad.narg_string[roots.len() * 32 + public.len() * 32] ^= 1;
        ensure!(verify(&bad).is_err(), "changed outer OOD accepted");
        let mut bad = proof.clone();
        let end = bad.hints.len() - 1;
        bad.hints[end] ^= 1;
        ensure!(verify(&bad).is_err(), "changed batch Merkle path accepted");
        // First bottom value follows prefix, outer OOD, 3 two-coefficient
        // sumcheck messages, and the folded root/OOD answer.
        let mut bad = proof.clone();
        let offset = roots.len() * 32 + public.len() * 32 + 32 + FOLD * 64 + 32 + 32;
        bad.narg_string[offset] ^= 1;
        ensure!(
            verify(&bad).is_err(),
            "changed Canopy bottom value accepted"
        );
    }
    times.actual_auth_hashes = verified.3;
    times.verified = true;
    times.with_incoming_commit_seconds = times.step_seconds + times.incoming_commit_seconds;
    Ok(times)
}
fn main() -> Result<()> {
    let args = Args::parse();
    register_benchmark_ntt();
    ensure!(args.repetitions > 0, "need repetitions");
    if let Some(parent) = args.output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut csv = csv::Writer::from_path(&args.output)?;
    for &log in &args.logs {
        ensure!((10..=24).contains(&log), "time moderate sizes only");
        let native = config(log);
        let cfg0 = &native.initial_committer;
        let cfg1 = &native.round_configs[0].irs_committer;
        ensure!(
            native.initial_sumcheck.num_rounds() == 3
                && cfg1.vector_size() == 1 << (log - 3)
                && cfg1.size() == 1 << log
                && cfg1.rate() == 0.125
                && cfg0.interleaving_depth() == 8
                && cfg1.interleaving_depth() == 8,
            "unexpected native WHIR geometry"
        );
        eprintln!(
            "N=2^{log}: RS0 msg={} code={} rows={} q={}; RS1 msg={} code={} rows={} q={}",
            cfg0.vector_size(),
            cfg0.size(),
            cfg0.codeword_length(),
            cfg0.in_domain_samples(),
            cfg1.vector_size(),
            cfg1.size(),
            cfg1.codeword_length(),
            cfg1.in_domain_samples()
        );
        let message = Buffer::from(
            (0..1usize << log)
                .into_par_iter()
                .map(|i| elem(91, 0, i as u64))
                .collect::<Vec<_>>(),
        );
        let prior0 = Buffer::from(
            (0..1usize << log)
                .into_par_iter()
                .map(|i| elem(91, 1, i as u64))
                .collect::<Vec<_>>(),
        );
        let prior1 = Buffer::from(prior0.to_slice()[..1 << (log - 3)].to_vec());
        for rep in 0..args.warmups + args.repetitions {
            // Alternate the ordering to reduce systematic local drift.
            for canopy in if rep % 2 == 0 {
                [false, true]
            } else {
                [true, false]
            } {
                let row = if canopy {
                    run_canopy(
                        &native,
                        &message,
                        prior1.clone(),
                        rep.saturating_sub(args.warmups),
                        args.check_tampering,
                    )?
                } else {
                    run_baseline(
                        &native,
                        &message,
                        prior0.clone(),
                        rep.saturating_sub(args.warmups),
                        args.check_tampering,
                    )?
                };
                eprintln!(
                    "{} n={log} rep={rep} step={:.6}s incoming={:.6}s hashes={} FS={} verified={}",
                    row.scheme,
                    row.step_seconds,
                    row.incoming_commit_seconds,
                    row.actual_auth_hashes,
                    row.fs_finalizations,
                    row.verified
                );
                if rep >= args.warmups {
                    csv.serialize(&row)?;
                    csv.flush()?;
                }
            }
        }
    }
    Ok(())
}
