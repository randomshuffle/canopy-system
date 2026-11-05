use anyhow::Result;
use canopy_system::bolt_experiment::black_box;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    /// Search independent Canopy heights, keeping Bolt's Merkle tail fixed.
    Estimate {
        #[arg(long, value_delimiter = ',', default_value = "20")]
        logs: Vec<usize>,
        #[arg(long, value_delimiter = ',', default_value = "1,8")]
        inputs: Vec<usize>,
        #[arg(long, default_value_t = 100)]
        samples: usize,
        #[arg(long)]
        output: PathBuf,
    },
    /// Measure and verify the Merkle and black-box Canopy arguments.
    Time {
        #[arg(long)]
        log: usize,
        #[arg(long)]
        inputs: usize,
        #[arg(long, default_value_t = 1)]
        warmups: usize,
        #[arg(long, default_value_t = 2)]
        repetitions: usize,
        #[arg(long)]
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
            output,
        } => black_box::estimate(&logs, &inputs, samples, &output),
        Command::Time {
            log,
            inputs,
            warmups,
            repetitions,
            estimates,
            output,
        } => black_box::time(log, inputs, warmups, repetitions, &estimates, &output),
    }
}
