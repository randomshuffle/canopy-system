use anyhow::Result;
use canopy_system::bolt_experiment::{batching, fiat_shamir, multi, DEFAULT_SEED};
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
        #[arg(long, default_value_t = 4)]
        width: usize,
        #[arg(long, default_value = "results/gf256_batched_suite/bolt/proof_sizes")]
        output: PathBuf,
    },
    Account {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    #[cfg(feature = "fs-accounting")]
    Validate {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
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
        #[arg(long, default_value = "results/gf256_batched_suite/bolt/proof_sizes")]
        estimates: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
}
fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Estimate {
            logs,
            inputs,
            samples,
            width,
            output,
        } => {
            batching::estimate_grid(&logs, &inputs, samples, DEFAULT_SEED, width, &output)?;
        }
        Command::Account { input, output } => fiat_shamir::account(&input, &output)?,
        #[cfg(feature = "fs-accounting")]
        Command::Validate { input, output } => fiat_shamir::validate(&input, &output)?,
        Command::Time {
            log,
            inputs,
            warmups,
            repetitions,
            estimates,
            output,
        } => multi::time_selected(
            log,
            inputs,
            warmups,
            repetitions,
            DEFAULT_SEED,
            &estimates,
            &output,
        )?,
    }
    Ok(())
}
