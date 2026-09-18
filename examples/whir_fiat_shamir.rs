//! Account for the composed transcript of a selected WHIR experiment directory.
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    input_dir: PathBuf,
    #[arg(long)]
    output: PathBuf,
}
fn main() -> Result<()> {
    let args = Args::parse();
    canopy_system::paper_experiments::fiat_shamir::account(
        &args.input_dir.join("hybrid_proof_sizes.csv"),
        &args.input_dir.join("oracle_query_breakdown.csv"),
        &args.output,
    )
}
