use clap::Parser;
use std::path::PathBuf;
#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = 22)]
    log_length: usize,
    #[arg(long, default_value_t = 1)]
    warmups: usize,
    #[arg(long, default_value_t = 3)]
    repetitions: usize,
    #[arg(long, default_value_t = 8)]
    threads: usize,
    #[arg(long, default_value_t = 0x43414e4f5059)]
    seed: u64,
    #[arg(long)]
    output_dir: PathBuf,
}
fn main() -> anyhow::Result<()> {
    let a = Args::parse();
    rayon::ThreadPoolBuilder::new()
        .num_threads(a.threads)
        .build_global()?;
    canopy_binary_whir::register_benchmark_ntt();
    canopy_binary_whir::paper_experiments::run_prover_timings(
        &a.output_dir,
        a.log_length,
        a.warmups,
        a.repetitions,
        0,
        a.seed,
    )
}
