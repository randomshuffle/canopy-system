//! Section 8 batching reductions, with a verifier-hash height search.
use anyhow::{ensure, Context, Result};
use canopy_system::{
    register_benchmark_ntt,
    section8::{run_canopy, run_merkle},
    section8_estimator::{
        estimate_height_search, Section8EstimateParameters, Section8HeightEstimate,
    },
};
use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::Serialize;
use std::{fs::File, path::PathBuf};

#[derive(Parser)]
#[command(about = "Benchmark the Section 8 ifold and iacc reductions")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Estimate(EstimateArgs),
    Time(TimeArgs),
}

#[derive(Clone, Copy, ValueEnum)]
enum Objective {
    Total,
    Ifold,
}

#[derive(Args)]
struct Common {
    /// Message lengths N=2^log_n, in field elements.
    #[arg(long, value_delimiter = ',', default_value = "18,20,22")]
    logs: Vec<usize>,
    /// Fresh full-message claims per step; one previous Canopy claim is also folded.
    #[arg(long, value_delimiter = ',', default_value = "1,2,4,8")]
    ell_cast: Vec<usize>,
    #[arg(long, value_delimiter = ',', default_value = "1")]
    ell_acc: Vec<usize>,
    /// Field elements per authenticated RS row, as a power of two.
    #[arg(long, default_value_t = 3)]
    log_leaf_width: usize,
    /// Local round-by-round target, without a union bound over protocol rounds.
    #[arg(long, default_value_t = 128)]
    security_bits: usize,
    #[arg(long, value_enum, default_value = "total")]
    objective: Objective,
}

#[derive(Args)]
struct EstimateArgs {
    #[command(flatten)]
    common: Common,
    #[arg(long)]
    output_dir: PathBuf,
}

#[derive(Clone, Copy, ValueEnum)]
enum Scheme {
    Canopy,
    MerkleHalf,
    MerkleQuarter,
}

#[derive(Args)]
struct TimeArgs {
    #[command(flatten)]
    common: Common,
    #[arg(long)]
    output: PathBuf,
    #[arg(long, default_value_t = 1)]
    warmups: usize,
    #[arg(long, default_value_t = 2)]
    repetitions: usize,
    #[arg(long, default_value_t = 91)]
    seed: u64,
    /// Override the optimized height for geometry validation or controls.
    #[arg(long)]
    height: Option<usize>,
    #[arg(long)]
    check_tampering: bool,
    #[arg(
        long,
        value_enum,
        value_delimiter = ',',
        default_value = "canopy,merkle-half,merkle-quarter"
    )]
    schemes: Vec<Scheme>,
}

// Serialize a flat record once, then prepend the common experiment columns.
// This keeps estimator and measured accounting fields intact in their CSVs.
struct OutputCsv {
    writer: csv::Writer<File>,
    headers: Option<Vec<String>>,
}
impl OutputCsv {
    fn new(path: &std::path::Path) -> Result<Self> {
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent)?;
        }
        ensure!(!path.exists(), "refusing to overwrite {}", path.display());
        Ok(Self {
            writer: csv::Writer::from_path(path)?,
            headers: None,
        })
    }
    fn row<T: Serialize>(&mut self, context: &[(&str, String)], row: &T) -> Result<()> {
        let mut one = csv::Writer::from_writer(Vec::new());
        one.serialize(row)?;
        let bytes = one.into_inner()?;
        let mut reader = csv::Reader::from_reader(bytes.as_slice());
        let keep = reader
            .headers()?
            .iter()
            .enumerate()
            .filter(|(_, k)| !context.iter().any(|(name, _)| name == k))
            .map(|(i, k)| (i, k.to_owned()))
            .collect::<Vec<_>>();
        let mut headers = context
            .iter()
            .map(|(k, _)| k.to_string())
            .collect::<Vec<_>>();
        headers.extend(keep.iter().map(|(_, k)| k.clone()));
        if let Some(previous) = &self.headers {
            ensure!(previous == &headers, "CSV schemas differ");
        } else {
            self.writer.write_record(&headers)?;
            self.headers = Some(headers);
        }
        let mut values = context.iter().map(|(_, v)| v.clone()).collect::<Vec<_>>();
        let record = reader
            .records()
            .next()
            .context("missing serialized row")??;
        values.extend(keep.iter().map(|(i, _)| record[*i].to_owned()));
        self.writer.write_record(values)?;
        self.writer.flush()?;
        Ok(())
    }
}

fn parameters(
    common: &Common,
    log_n: usize,
    ell_cast: usize,
    ell_acc: usize,
) -> Section8EstimateParameters {
    Section8EstimateParameters {
        log_message: log_n,
        ell_cast,
        ell_acc,
        log_leaf_width: common.log_leaf_width,
        local_security_bits: common.security_bits,
    }
}

fn select(
    common: &Common,
    candidates: &[Section8HeightEstimate],
) -> Result<Section8HeightEstimate> {
    candidates
        .iter()
        .filter(|c| c.eligible)
        .min_by(|a, b| {
            let score = |c: &Section8HeightEstimate| match common.objective {
                Objective::Total => c.total_verifier_hashes,
                Objective::Ifold => c.ifold.total_verifier_hashes,
            };
            score(a).total_cmp(&score(b)).then(a.height.cmp(&b.height))
        })
        .cloned()
        .context("no eligible Canopy height")
}

fn context(
    common: &Common,
    log_n: usize,
    ell_cast: usize,
    ell_acc: usize,
    height: usize,
    scheme: &str,
) -> Vec<(&'static str, String)> {
    vec![
        ("workload", "steady_state_v2".into()),
        ("log_n", log_n.to_string()),
        ("ell_cast", ell_cast.to_string()),
        ("ell_acc", ell_acc.to_string()),
        ("height", height.to_string()),
        ("log_leaf_width", common.log_leaf_width.to_string()),
        ("security_bits", common.security_bits.to_string()),
        ("scheme", scheme.into()),
    ]
}

fn estimate(args: EstimateArgs) -> Result<()> {
    std::fs::create_dir_all(&args.output_dir)?;
    let mut estimates = OutputCsv::new(&args.output_dir.join("estimates.csv"))?;
    let mut search = OutputCsv::new(&args.output_dir.join("height_search.csv"))?;
    for &log_n in &args.common.logs {
        for &ell_cast in &args.common.ell_cast {
            for &ell_acc in &args.common.ell_acc {
                let result =
                    estimate_height_search(&parameters(&args.common, log_n, ell_cast, ell_acc))?;
                let selected = select(&args.common, &result.candidates)?;
                for candidate in &result.candidates {
                    for (scheme, row) in [("ifold", &candidate.ifold), ("iacc", &candidate.iacc)] {
                        let mut cx = context(
                            &args.common,
                            log_n,
                            ell_cast,
                            ell_acc,
                            candidate.height,
                            scheme,
                        );
                        cx.push((
                            "selected",
                            (candidate.height == selected.height).to_string(),
                        ));
                        cx.push(("eligible", candidate.eligible.to_string()));
                        search.row(&cx, row)?;
                    }
                }
                for (height, scheme, row) in [
                    (selected.height, "ifold", &selected.ifold),
                    (selected.height, "iacc", &selected.iacc),
                    (0, "merkle_rate_half", &result.merkle_rate_half),
                    (0, "merkle_rate_quarter", &result.merkle_rate_quarter),
                ] {
                    estimates.row(
                        &context(&args.common, log_n, ell_cast, ell_acc, height, scheme),
                        row,
                    )?;
                }
                eprintln!(
                    "N=2^{log_n} cast={ell_cast} acc={ell_acc}: selected h={}",
                    selected.height
                );
            }
        }
    }
    Ok(())
}

fn time(args: TimeArgs) -> Result<()> {
    ensure!(
        args.repetitions > 0 && !args.schemes.is_empty(),
        "need repetitions and schemes"
    );
    let mut output = OutputCsv::new(&args.output)?;
    for &log_n in &args.common.logs {
        for &ell_cast in &args.common.ell_cast {
            for &ell_acc in &args.common.ell_acc {
                let result =
                    estimate_height_search(&parameters(&args.common, log_n, ell_cast, ell_acc))?;
                let height = args
                    .height
                    .unwrap_or(select(&args.common, &result.candidates)?.height);
                ensure!(
                    result
                        .candidates
                        .iter()
                        .any(|c| c.height == height && c.eligible),
                    "height {height} is ineligible"
                );
                for pass in 0..args.warmups + args.repetitions {
                    let seed = args.seed.wrapping_add(pass as u64);
                    for slot in 0..args.schemes.len() {
                        // Rotate all three configurations, including warmups.
                        let scheme = args.schemes[(slot + pass) % args.schemes.len()];
                        let mut save =
                            |name: &str,
                             h: usize,
                             measurement: &canopy_system::section8::ReductionMeasurement|
                             -> Result<()> {
                                eprintln!("N=2^{log_n} cast={ell_cast} acc={ell_acc} h={h} {name} pass={pass}: {:.6}s verified={}", measurement.step_seconds, measurement.verified);
                                if pass >= args.warmups {
                                    let mut cx =
                                        context(&args.common, log_n, ell_cast, ell_acc, h, name);
                                    cx.push(("repetition", (pass - args.warmups).to_string()));
                                    cx.push(("input_seed", seed.to_string()));
                                    output.row(&cx, measurement)?;
                                }
                                Ok(())
                            };
                        match scheme {
                            Scheme::Canopy => {
                                let run = run_canopy(
                                    log_n,
                                    ell_cast,
                                    ell_acc,
                                    height,
                                    args.common.log_leaf_width,
                                    args.common.security_bits as f64,
                                    seed,
                                    args.check_tampering,
                                )?;
                                save("ifold", height, &run.ifold)?;
                                save("iacc", height, &run.iacc)?;
                            }
                            Scheme::MerkleHalf | Scheme::MerkleQuarter => {
                                let quarter = matches!(scheme, Scheme::MerkleQuarter);
                                let row = run_merkle(
                                    log_n,
                                    ell_cast,
                                    ell_acc,
                                    if quarter { 2 } else { 1 },
                                    args.common.log_leaf_width,
                                    args.common.security_bits as f64,
                                    seed,
                                    args.check_tampering,
                                )?;
                                save(
                                    if quarter {
                                        "merkle_rate_quarter"
                                    } else {
                                        "merkle_rate_half"
                                    },
                                    0,
                                    &row,
                                )?;
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    register_benchmark_ntt();
    match cli.command {
        Command::Estimate(args) => estimate(args),
        Command::Time(args) => time(args),
    }
}
