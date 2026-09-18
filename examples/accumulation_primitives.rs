//! Cost primitives for same-code SEP accumulation, not a complete accumulation scheme.
//! Scalar RS commitments (interleaving depth one); no shrinking of the accumulator.
use anyhow::{ensure, Result};
use canopy_system::{additive_rs, benchmark_field_element, register_benchmark_ntt, Fr};
use clap::Parser;
use rayon::prelude::*;
use serde::Serialize;
use std::{path::PathBuf, time::Instant};
use whir_ref::{
    algebra::{embedding::Identity, multilinear_extend},
    buffer::{Buffer, BufferMath, BufferOps},
    hash,
    protocols::{
        irs_commit::{Config, IrsMode, IrsParams},
        params::DecodingRegime,
    },
    transcript::{codecs::Empty, DomainSeparator, ProverState, VerifierMessage, VerifierState},
};

type Sponge = spongefish::instantiations::XOF<blake3::Hasher>;
#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "18,20,22", value_delimiter = ',')]
    logs: Vec<usize>,
    #[arg(long, default_value_t = 1)]
    log_inverse_rate: usize,
    #[arg(long, default_value_t = 2)]
    sources: usize,
    #[arg(long, default_value_t = 1)]
    warmups: usize,
    #[arg(long, default_value_t = 5)]
    repetitions: usize,
    #[arg(long)]
    output: PathBuf,
}

#[derive(Serialize)]
struct Row {
    log_message: usize,
    log_inverse_rate: usize,
    sources: usize,
    repetition: usize,
    queries: usize,
    input_commit_mean_seconds: f64,
    covector_seconds: f64,
    batch_sumcheck_seconds: f64,
    combine_seconds: f64,
    output_commit_seconds: f64,
    output_ood_seconds: f64,
    source_open_seconds: f64,
    accounted_step_seconds: f64,
    verified: bool,
}

fn add(a: Buffer<Fr>, b: Buffer<Fr>) -> Buffer<Fr> {
    Buffer::from(
        a.to_slice()
            .par_iter()
            .zip(b.to_slice())
            .map(|(&x, &y)| x + y)
            .collect::<Vec<_>>(),
    )
}

fn run(args: &Args, log: usize, rep: usize, messages: &[Buffer<Fr>]) -> Result<Row> {
    let len = 1usize << log;
    let cfg = Config::<Identity<Fr>>::new(IrsParams {
        security_target: 136.,
        decoding_regime: DecodingRegime::Johnson,
        hash_id: hash::BLAKE3,
        num_vectors: 1,
        vector_size: len,
        interleaving_depth: 1,
        rate: 2f64.powi(-(args.log_inverse_rate as i32)),
        mode: IrsMode::Standard,
    });
    let ds = DomainSeparator::protocol(&cfg)
        .session(&"same-code-accumulation-cost-primitives/v1")
        .instance(&Empty);
    let mut p = ProverState::new(&ds, Sponge::default());
    let start = Instant::now();
    let witnesses = messages
        .iter()
        .map(|m| cfg.commit(&mut p, &[m]))
        .collect::<Vec<_>>();
    let input_commit_mean_seconds = start.elapsed().as_secs_f64() / args.sources as f64;

    // Representative compact input covectors: one MLE plus one OOD and q
    // RS-domain terms. Materializing every source this way is conservative
    // for a fresh MLE claim; report separately so callers can adjust scope.
    let start = Instant::now();
    let forms = (0..args.sources)
        .map(|j| {
            let point = (0..log)
                .map(|i| benchmark_field_element(71, j as u64, i as u64))
                .collect::<Vec<_>>();
            let mut form = Buffer::eq_weights(&point);
            let indices = (0..cfg.in_domain_samples())
                .map(|i| (i * 7919 + j * 97) % cfg.codeword_length())
                .collect::<Vec<_>>();
            let weights = (0..indices.len())
                .map(|i| benchmark_field_element(73, j as u64, i as u64))
                .collect::<Vec<_>>();
            form = add(
                form,
                additive_rs::power_sums(&indices, &weights, cfg.codeword_length(), len),
            );
            form.accumulate_geometric(
                &[benchmark_field_element(77, j as u64, 0)],
                &Buffer::from(vec![Fr::ONE]),
                len,
            );
            form
        })
        .collect::<Vec<_>>();
    let covector_seconds = start.elapsed().as_secs_f64();
    // Public sigmas belong to the input instance, generated outside the timer.
    let sigmas = messages
        .iter()
        .zip(&forms)
        .map(|(m, f)| m.dot(f))
        .collect::<Vec<_>>();
    p.prover_messages(&sigmas);

    let start = Instant::now();
    let beta: Fr = p.verifier_message();
    let betas = (0..args.sources)
        .scan(Fr::ONE, |a, _| {
            let x = *a;
            *a *= beta;
            Some(x)
        })
        .collect::<Vec<_>>();
    let mut sum = sigmas.iter().zip(&betas).map(|(&a, &b)| a * b).sum::<Fr>();
    let mut pairs = messages
        .iter()
        .zip(&forms)
        .map(|(m, f)| (m.clone(), f.clone()))
        .collect::<Vec<_>>();
    let mut point = Vec::new();
    for _ in 0..log {
        let (c0, c2) = pairs
            .iter()
            .zip(&betas)
            .map(|((m, f), &b)| {
                let (a, c) = m.sumcheck_polynomial(f);
                (a * b, c * b)
            })
            .fold((Fr::ZERO, Fr::ZERO), |(a, b), (c, d)| (a + c, b + d));
        p.prover_message(&c0);
        p.prover_message(&c2);
        let r: Fr = p.verifier_message();
        sum = (c2 * r + (sum - c2)) * r + c0; // characteristic two
        pairs.iter_mut().for_each(|(m, f)| m.fold_pair(f, r));
        point.push(r);
    }
    let values = pairs
        .iter()
        .map(|(m, _)| m.to_slice()[0])
        .collect::<Vec<_>>();
    p.prover_messages(&values);
    let gamma: Fr = p.verifier_message();
    let batch_sumcheck_seconds = start.elapsed().as_secs_f64();

    let start = Instant::now();
    let gammas = (0..args.sources)
        .scan(Fr::ONE, |a, _| {
            let x = *a;
            *a *= gamma;
            Some(x)
        })
        .collect::<Vec<_>>();
    let combined = Buffer::from(
        (0..len)
            .into_par_iter()
            .map(|i| {
                messages
                    .iter()
                    .zip(&gammas)
                    .map(|(m, &g)| m.to_slice()[i] * g)
                    .sum::<Fr>()
            })
            .collect::<Vec<_>>(),
    );
    let combine_seconds = start.elapsed().as_secs_f64();
    let start = Instant::now();
    let _output = cfg.commit(&mut p, &[&combined]);
    let output_commit_seconds = start.elapsed().as_secs_f64();
    let start = Instant::now();
    let alpha: Fr = p.verifier_message();
    let ood = combined.mixed_univariate_evaluate(&Identity::new(), alpha);
    p.prover_message(&ood);
    let output_ood_seconds = start.elapsed().as_secs_f64();
    let start = Instant::now();
    let eval = cfg.open(&mut p, &witnesses.iter().collect::<Vec<_>>());
    let _: Fr = p.verifier_message(); // outgoing SEP's constraint batching
    let source_open_seconds = start.elapsed().as_secs_f64();

    // Replay sumcheck and all source Merkle checks. Separately check the
    // outgoing message's MLE and opened evaluations. Verification is untimed.
    let proof = p.proof();
    let mut v = VerifierState::new(&ds, &proof, Sponge::default());
    let commitments = (0..args.sources)
        .map(|_| {
            cfg.receive_commitment(&mut v)
                .map_err(|e| anyhow::anyhow!("root {e:?}"))
        })
        .collect::<Result<Vec<_>>>()?;
    let received: Vec<Fr> = v
        .prover_messages_vec(args.sources)
        .map_err(|e| anyhow::anyhow!("sigmas {e:?}"))?;
    ensure!(received == sigmas, "input sigmas");
    ensure!(v.verifier_message::<Fr>() == beta, "beta");
    let mut check = sigmas.iter().zip(&betas).map(|(&a, &b)| a * b).sum::<Fr>();
    for &r in &point {
        let c0: Fr = v
            .prover_message()
            .map_err(|e| anyhow::anyhow!("c0 {e:?}"))?;
        let c2: Fr = v
            .prover_message()
            .map_err(|e| anyhow::anyhow!("c2 {e:?}"))?;
        ensure!(v.verifier_message::<Fr>() == r, "sumcheck challenge");
        check = (c2 * r + check - c2) * r + c0;
    }
    let ys: Vec<Fr> = v
        .prover_messages_vec(args.sources)
        .map_err(|e| anyhow::anyhow!("ys {e:?}"))?;
    ensure!(
        check == sum
            && check
                == forms
                    .iter()
                    .zip(&ys)
                    .zip(&betas)
                    .map(|((f, &y), &b)| multilinear_extend(f.to_slice(), &point) * y * b)
                    .sum::<Fr>(),
        "batch sumcheck"
    );
    ensure!(v.verifier_message::<Fr>() == gamma, "gamma");
    let _ = cfg
        .receive_commitment(&mut v)
        .map_err(|e| anyhow::anyhow!("output root {e:?}"))?;
    ensure!(v.verifier_message::<Fr>() == alpha, "alpha");
    let y: Fr = v
        .prover_message()
        .map_err(|e| anyhow::anyhow!("OOD {e:?}"))?;
    ensure!(y == ood, "OOD value");
    let checked = cfg
        .verify(&mut v, &commitments.iter().collect::<Vec<_>>())
        .map_err(|e| anyhow::anyhow!("openings {e:?}"))?;
    let _: Fr = v.verifier_message();
    ensure!(eval == checked, "query values");
    ensure!(
        multilinear_extend(combined.to_slice(), &point)
            == ys.iter().zip(&gammas).map(|(&a, &b)| a * b).sum::<Fr>(),
        "output MLE"
    );
    // Check query constraints collectively with an independently fixed weight vector.
    let weights = (0..eval.points.len())
        .map(|i| benchmark_field_element(83, 0, i as u64))
        .collect::<Vec<_>>();
    let domain = eval.domain.as_ref().unwrap();
    let expected = combined.dot(&additive_rs::power_sums(
        &domain.exponents,
        &weights,
        domain.length,
        len,
    ));
    let actual = eval
        .rows()
        .zip(&weights)
        .map(|(row, &w)| row.iter().zip(&gammas).map(|(&a, &g)| a * g).sum::<Fr>() * w)
        .sum::<Fr>();
    ensure!(expected == actual, "output RS query consistency");
    let accounted_step_seconds = covector_seconds
        + batch_sumcheck_seconds
        + combine_seconds
        + output_commit_seconds
        + output_ood_seconds
        + source_open_seconds;
    Ok(Row {
        log_message: log,
        log_inverse_rate: args.log_inverse_rate,
        sources: args.sources,
        repetition: rep,
        queries: cfg.in_domain_samples(),
        input_commit_mean_seconds,
        covector_seconds,
        batch_sumcheck_seconds,
        combine_seconds,
        output_commit_seconds,
        output_ood_seconds,
        source_open_seconds,
        accounted_step_seconds,
        verified: true,
    })
}

fn main() -> Result<()> {
    let args = Args::parse();
    ensure!(
        args.sources >= 2 && args.repetitions > 0,
        "need at least two sources and one repetition"
    );
    register_benchmark_ntt();
    if let Some(parent) = args.output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut out = csv::Writer::from_path(&args.output)?;
    for &log in &args.logs {
        let messages = (0..args.sources)
            .map(|j| {
                Buffer::from(
                    (0..1usize << log)
                        .into_par_iter()
                        .map(|i| benchmark_field_element(89, j as u64, i as u64))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        for rep in 0..args.warmups + args.repetitions {
            let row = run(&args, log, rep.saturating_sub(args.warmups), &messages)?;
            eprintln!(
                "n={log} rate=1/{} sources={} rep={rep} accounted step={:.6}s verified={}",
                1usize << args.log_inverse_rate,
                args.sources,
                row.accounted_step_seconds,
                row.verified
            );
            if rep >= args.warmups {
                out.serialize(row)?;
                out.flush()?;
            }
        }
    }
    Ok(())
}
