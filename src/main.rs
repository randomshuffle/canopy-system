use std::path::PathBuf;

use anyhow::{Context, Result};
use canopy_system::paper_experiments::{
    run_proof_estimates_grid, run_prover_timings_configured, PAPER_TERMINAL_MESSAGE_VARIABLES,
};
use canopy_system::whir::WhirDecodingRegime;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "WHIR and Canopy-WHIR experiment estimates and timings"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Estimate the two Merkle baselines and Canopy-WHIR without allocating the inputs.
    PaperEstimate {
        #[arg(long, value_delimiter = ',', default_value = "22,24,26,28,30")]
        logs: Vec<usize>,
        #[arg(long, default_value_t = 100)]
        samples: usize,
        /// Log reduction from the base rate-1/2 codeword; the experiment uses 1.
        #[arg(long, default_value_t = 1)]
        canopy_codeword_log_reduction: usize,
        #[arg(long, value_delimiter = ',', default_value = "1,2,4,8")]
        inputs: Vec<usize>,
        #[arg(long, default_value_t = 0x5041_5045_525f_5631)]
        seed: u64,
        #[arg(long, default_value = "results/reproduce/whir/estimates")]
        output_dir: PathBuf,
    },
    /// Time verifier-checked native WHIR components for all three methods.
    PaperTime {
        #[arg(long, default_value_t = 18)]
        log_length: usize,
        #[arg(long, default_value_t = 1)]
        warmups: usize,
        #[arg(long, default_value_t = 2)]
        repetitions: usize,
        #[arg(long, default_value_t = 1)]
        canopy_codeword_log_reduction: usize,
        #[arg(long, value_delimiter = ',', default_value = "1,2,4,8")]
        inputs: Vec<usize>,
        #[arg(long, default_value_t = 0x5041_5045_525f_5631)]
        seed: u64,
        #[arg(long, default_value = "results/reproduce/whir/timings")]
        output_dir: PathBuf,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::PaperEstimate {
            logs,
            samples,
            canopy_codeword_log_reduction,
            inputs,
            seed,
            output_dir,
        } => {
            run_proof_estimates_grid(
                &output_dir,
                samples,
                seed,
                WhirDecodingRegime::Johnson,
                PAPER_TERMINAL_MESSAGE_VARIABLES,
                canopy_codeword_log_reduction,
                &inputs,
                &logs,
            )
            .with_context(|| format!("writing WHIR estimates to {}", output_dir.display()))?;
            println!("wrote WHIR estimates to {}", output_dir.display());
        }
        Command::PaperTime {
            log_length,
            warmups,
            repetitions,
            canopy_codeword_log_reduction,
            inputs,
            seed,
            output_dir,
        } => {
            run_prover_timings_configured(
                &output_dir,
                log_length,
                warmups,
                repetitions,
                seed,
                WhirDecodingRegime::Johnson,
                canopy_codeword_log_reduction,
                &inputs,
            )
            .with_context(|| format!("writing WHIR timings to {}", output_dir.display()))?;
            println!("wrote WHIR timings to {}", output_dir.display());
        }
    }
    Ok(())
}
