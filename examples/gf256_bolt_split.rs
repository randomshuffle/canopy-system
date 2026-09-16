use anyhow::Result;
use canopy_system::bolt_experiment::{split, DEFAULT_SEED};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Estimate {
        #[arg(long, value_delimiter = ',', default_value = "18,20,22,24,26,28,30")]
        logs: Vec<usize>,
        #[arg(long, value_delimiter = ',', default_value = "1,2,4,8")]
        inputs: Vec<usize>,
        #[arg(long, default_value_t = 100)]
        samples: usize,
        #[arg(long, default_value_t = 1)]
        block_divisor: usize,
        #[arg(long, default_value_t = 4)]
        width: usize,
        #[arg(long, default_value = "results/gf256_split_suite/proof_sizes")]
        output: PathBuf,
    },
    Time {
        #[arg(long)]
        log: usize,
        #[arg(long)]
        inputs: usize,
        #[arg(long, default_value_t = 1)]
        warmups: usize,
        #[arg(long, default_value_t = 5)]
        repetitions: usize,
        #[arg(long, default_value = "results/gf256_split_suite/proof_sizes")]
        estimates: PathBuf,
        #[arg(long, default_value = "results/gf256_batched_suite/bolt/proof_sizes")]
        shared_estimates: PathBuf,
        #[arg(long)]
        paper_only: bool,
        #[arg(long, default_value_t = 4)]
        width: usize,
        #[arg(long)]
        alternate_estimates: Option<PathBuf>,
        #[arg(long)]
        output: PathBuf,
    },
    #[cfg(feature = "fs-accounting")]
    Validate {
        #[arg(
            long,
            default_value = "results/gf256_split_suite/proof_sizes/proof_sizes.csv"
        )]
        input: PathBuf,
        #[arg(
            long,
            default_value = "results/gf256_split_suite/fiat_shamir_validation.csv"
        )]
        output: PathBuf,
    },
}
fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Estimate {
            logs,
            inputs,
            samples,
            block_divisor,
            width,
            output,
        } => split::estimate_grid(
            &logs,
            &inputs,
            samples,
            DEFAULT_SEED,
            block_divisor,
            width,
            &output,
        ),
        Command::Time {
            log,
            inputs,
            warmups,
            repetitions,
            estimates,
            shared_estimates,
            paper_only,
            width,
            alternate_estimates,
            output,
        } => split::time_selected(
            log,
            inputs,
            warmups,
            repetitions,
            &estimates,
            &shared_estimates,
            &output,
            paper_only,
            width,
            alternate_estimates.as_deref(),
        ),
        #[cfg(feature = "fs-accounting")]
        Command::Validate { input, output } => split::validate(&input, &output),
    }
}
