use std::path::PathBuf;

use anyhow::{Context, Result};
use canopy_system::benchmark::{
    run_commitment_timings, run_size_matrix, run_size_matrix_with_regime_and_terminal, run_timings,
    SizeMatrixConfig,
};
use canopy_system::paper_experiments::{
    run_proof_estimates, run_proof_estimates_with_regime,
    run_proof_estimates_with_regime_and_terminal, run_prover_timings,
    run_prover_timings_with_regime,
};
use canopy_system::separate_benchmark::{
    run_separate_commitment_timings, run_separate_size_matrix, select_separate_height,
    SeparateCommitmentTimingConfig, SeparateHeightSearchConfig, SeparateSizeMatrixConfig,
};
use canopy_system::whir::WhirDecodingRegime;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Generate the three final paper proof-size experiments and query tables.
    PaperEstimate {
        #[arg(long, default_value_t = 100)]
        samples: usize,
        #[arg(long, default_value_t = 0x5041_5045_525f_5631)]
        seed: u64,
        #[arg(long, default_value = "results/paper")]
        output_dir: PathBuf,
    },
    /// Generate conjectural capacity-bound versions of all three experiments.
    PaperCapacityEstimate {
        #[arg(long, default_value_t = 100)]
        samples: usize,
        #[arg(long, default_value_t = 0x5041_5045_525f_5631)]
        seed: u64,
        #[arg(long, default_value = "results/paper_capacity")]
        output_dir: PathBuf,
    },
    /// Estimate capacity-bound Canopy vs Merkle for batch 128 and terminal 2^8.
    PaperCapacityTerminal8Estimate {
        #[arg(long, default_value_t = 100)]
        samples: usize,
        #[arg(long, default_value_t = 0x5041_5045_525f_5631)]
        seed: u64,
        #[arg(long, default_value = "results/paper_capacity_terminal8_batch128")]
        output_dir: PathBuf,
    },
    /// Estimate all experiments with fixed fold 3, capacity, and terminal 2^8.
    PaperCapacityTerminal8HybridEstimate {
        #[arg(long, default_value_t = 100)]
        samples: usize,
        #[arg(long, default_value_t = 0x5041_5045_525f_5631)]
        seed: u64,
        #[arg(long, default_value = "results/paper_capacity_terminal8_fold3")]
        output_dir: PathBuf,
    },
    /// Estimate all experiments with fixed fold 3, Johnson, and terminal 2^8.
    PaperTerminal8HybridEstimate {
        #[arg(long, default_value_t = 100)]
        samples: usize,
        #[arg(long, default_value_t = 0x5041_5045_525f_5631)]
        seed: u64,
        #[arg(long, default_value = "results/paper_johnson_terminal8_fold3")]
        output_dir: PathBuf,
    },
    /// Run the executable 2^20 prover benchmarks for all three experiments.
    PaperTime {
        #[arg(long, default_value_t = 20)]
        log_length: usize,
        #[arg(long, default_value_t = 3)]
        warmups: usize,
        #[arg(long, default_value_t = 20)]
        repetitions: usize,
        #[arg(long, default_value_t = 100)]
        height_search_samples: usize,
        #[arg(long, default_value_t = 0x5041_5045_525f_5631)]
        seed: u64,
        #[arg(long, default_value = "results/paper")]
        output_dir: PathBuf,
    },
    /// Run conjectural capacity-bound executable prover benchmarks at 2^20.
    PaperCapacityTime {
        #[arg(long, default_value_t = 20)]
        log_length: usize,
        #[arg(long, default_value_t = 3)]
        warmups: usize,
        #[arg(long, default_value_t = 20)]
        repetitions: usize,
        #[arg(long, default_value_t = 100)]
        height_search_samples: usize,
        #[arg(long, default_value_t = 0x5041_5045_525f_5631)]
        seed: u64,
        #[arg(long, default_value = "results/paper_capacity")]
        output_dir: PathBuf,
    },
    /// Estimate the requested proof-size matrix without allocating the vectors.
    Estimate {
        #[arg(long, value_delimiter = ',', default_values_t = vec![22, 24, 26, 28, 30])]
        log_lengths: Vec<usize>,
        #[arg(long, value_delimiter = ',', default_values_t = vec![64, 128, 256, 512, 1024])]
        batch_sizes: Vec<usize>,
        /// Optional global h list. Omit to search the full default grid 1..=15.
        #[arg(long, value_delimiter = ',')]
        heights: Vec<usize>,
        /// Candidate bit widths for the independent Canopy/WHIR layer field.
        #[arg(long, value_delimiter = ',', default_values_t = vec![256])]
        layer_field_bits: Vec<usize>,
        /// Reduce the Canopy codeword log length by these amounts (0, 1, or 2).
        #[arg(long, value_delimiter = ',', default_values_t = vec![0])]
        codeword_log_reductions: Vec<usize>,
        #[arg(long, default_value_t = 100)]
        samples: usize,
        #[arg(long, default_value_t = 0x4341_4e4f_5059)]
        seed: u64,
        #[arg(long, default_value = "results")]
        output_dir: PathBuf,
    },
    /// Estimate t separate commitments with random-linear-combination WHIR batching.
    EstimateSeparate {
        #[arg(long, value_delimiter = ',', default_values_t = vec![22, 24, 26, 28, 30])]
        log_lengths: Vec<usize>,
        #[arg(long, value_delimiter = ',', default_values_t = vec![64, 128, 256, 512, 1024])]
        batch_sizes: Vec<usize>,
        /// Optional global h list. Omit to search the full default grid 1..=15.
        #[arg(long, value_delimiter = ',')]
        heights: Vec<usize>,
        /// Number t of independently committed vectors.
        #[arg(long, value_delimiter = ',', default_values_t = vec![8, 16, 32])]
        commitment_counts: Vec<usize>,
        /// Candidate bit widths for the independent Canopy/WHIR layer field.
        #[arg(long, value_delimiter = ',', default_values_t = vec![256])]
        layer_field_bits: Vec<usize>,
        /// Reduce the Canopy codeword log length by these amounts (0, 1, or 2).
        #[arg(long, value_delimiter = ',', default_values_t = vec![0])]
        codeword_log_reductions: Vec<usize>,
        /// Split each RS codeword into this many interleaved equal-size blocks.
        #[arg(long, value_delimiter = ',', default_values_t = vec![1])]
        rs_interleaving_factors: Vec<usize>,
        /// Derive WHIR's initial oracle virtually from the old commitments.
        /// This currently requires RS interleaving factor 16.
        #[arg(long, default_value_t = false)]
        virtual_initial_oracle: bool,
        #[arg(long, default_value_t = 100)]
        samples: usize,
        #[arg(long, default_value_t = 0x5345_5041_5241_5445)]
        seed: u64,
        #[arg(long, default_value = "results/separate_commitments")]
        output_dir: PathBuf,
    },
    /// Run real commitments and front-end opening/verification timings.
    Time {
        #[arg(long, default_value_t = 22)]
        log_length: usize,
        #[arg(long, value_delimiter = ',', default_values_t = vec![64, 128, 256, 512, 1024])]
        batch_sizes: Vec<usize>,
        #[arg(long, value_delimiter = ',', default_values_t = vec![4, 5, 6, 7])]
        heights: Vec<usize>,
        #[arg(long, default_value_t = 10)]
        repetitions: usize,
        #[arg(long, default_value_t = 0x4341_4e4f_5059)]
        seed: u64,
        #[arg(long, default_value = "results/timings.csv")]
        output: PathBuf,
    },
    /// Compare prover commitment time for original, half, and quarter Canopy.
    CommitTime {
        #[arg(long, default_value_t = 22)]
        log_length: usize,
        /// Codeword log reductions paired positionally with `heights`.
        #[arg(long, value_delimiter = ',', default_values_t = vec![0, 1, 2])]
        codeword_log_reductions: Vec<usize>,
        /// Proof-size-optimal heights paired with the reductions.
        #[arg(long, value_delimiter = ',', default_values_t = vec![6, 6, 7])]
        heights: Vec<usize>,
        #[arg(long, default_value_t = 10)]
        repetitions: usize,
        #[arg(long, default_value = "results/codeword_commitment_timings.csv")]
        output: PathBuf,
    },
    /// Profile t separate commitments and Canopy's combined-oracle commitment.
    SeparateCommitTime {
        #[arg(long, default_value_t = 18)]
        log_length: usize,
        #[arg(long, default_value_t = 16)]
        commitment_count: usize,
        /// Batch size used to select the proof-size-optimal Canopy height.
        #[arg(long, default_value_t = 128)]
        batch_size: usize,
        /// Override the automatically selected proof-size-optimal height.
        #[arg(long)]
        height: Option<usize>,
        /// Reduce the Canopy codeword log length (2 selects Canopy-quarter).
        #[arg(long, default_value_t = 0)]
        codeword_log_reduction: usize,
        #[arg(long, default_value_t = 20)]
        repetitions: usize,
        #[arg(long, default_value_t = 100)]
        height_search_samples: usize,
        #[arg(long, default_value_t = 0x5345_5041_5241_5445)]
        seed: u64,
        #[arg(
            long,
            default_value = "results/separate_commitments/commitment_timings_n18_t16_b128.csv"
        )]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::PaperEstimate {
            samples,
            seed,
            output_dir,
        } => {
            run_proof_estimates(&output_dir, samples, seed)
                .with_context(|| format!("writing paper estimates to {}", output_dir.display()))?;
            println!("wrote final paper estimates to {}", output_dir.display());
        }
        Command::PaperCapacityEstimate {
            samples,
            seed,
            output_dir,
        } => {
            run_proof_estimates_with_regime(
                &output_dir,
                samples,
                seed,
                WhirDecodingRegime::Capacity,
            )
            .with_context(|| {
                format!(
                    "writing conjectural capacity estimates to {}",
                    output_dir.display()
                )
            })?;
            println!(
                "wrote conjectural capacity estimates to {}",
                output_dir.display()
            );
        }
        Command::PaperCapacityTerminal8Estimate {
            samples,
            seed,
            output_dir,
        } => {
            let config = SizeMatrixConfig {
                log_lengths: vec![22, 24, 26, 28, 30],
                batch_sizes: vec![128],
                heights: (1..=26).collect(),
                layer_field_bits: vec![256],
                codeword_log_reductions: vec![0],
                folding_log: canopy_system::whir::DEFAULT_FOLDING_LOG,
                samples,
                seed,
            };
            let rows = run_size_matrix_with_regime_and_terminal(
                &config,
                &output_dir,
                WhirDecodingRegime::Capacity,
                8,
            )
            .with_context(|| {
                format!(
                    "writing terminal-2^8 capacity estimates to {}",
                    output_dir.display()
                )
            })?;
            println!(
                "wrote {} terminal-2^8 capacity rows to {}",
                rows.len(),
                output_dir.display()
            );
        }
        Command::PaperCapacityTerminal8HybridEstimate {
            samples,
            seed,
            output_dir,
        } => {
            run_proof_estimates_with_regime_and_terminal(
                &output_dir,
                samples,
                seed,
                WhirDecodingRegime::Capacity,
                8,
            )
            .with_context(|| {
                format!(
                    "writing terminal-2^8 capacity hybrid estimates to {}",
                    output_dir.display()
                )
            })?;
            println!(
                "wrote terminal-2^8 capacity hybrid estimates to {}",
                output_dir.display()
            );
        }
        Command::PaperTerminal8HybridEstimate {
            samples,
            seed,
            output_dir,
        } => {
            run_proof_estimates_with_regime_and_terminal(
                &output_dir,
                samples,
                seed,
                WhirDecodingRegime::Johnson,
                8,
            )
            .with_context(|| {
                format!(
                    "writing terminal-2^8 Johnson hybrid estimates to {}",
                    output_dir.display()
                )
            })?;
            println!(
                "wrote terminal-2^8 Johnson hybrid estimates to {}",
                output_dir.display()
            );
        }
        Command::PaperTime {
            log_length,
            warmups,
            repetitions,
            height_search_samples,
            seed,
            output_dir,
        } => {
            run_prover_timings(
                &output_dir,
                log_length,
                warmups,
                repetitions,
                height_search_samples,
                seed,
            )
            .with_context(|| format!("writing paper timings to {}", output_dir.display()))?;
            println!("wrote final paper timings to {}", output_dir.display());
        }
        Command::PaperCapacityTime {
            log_length,
            warmups,
            repetitions,
            height_search_samples,
            seed,
            output_dir,
        } => {
            run_prover_timings_with_regime(
                &output_dir,
                log_length,
                warmups,
                repetitions,
                height_search_samples,
                seed,
                WhirDecodingRegime::Capacity,
            )
            .with_context(|| {
                format!(
                    "writing conjectural capacity timings to {}",
                    output_dir.display()
                )
            })?;
            println!(
                "wrote conjectural capacity timings to {}",
                output_dir.display()
            );
        }
        Command::Estimate {
            log_lengths,
            batch_sizes,
            heights,
            layer_field_bits,
            codeword_log_reductions,
            samples,
            seed,
            output_dir,
        } => {
            let config = SizeMatrixConfig {
                log_lengths,
                batch_sizes,
                heights,
                layer_field_bits,
                codeword_log_reductions,
                folding_log: canopy_system::whir::DEFAULT_FOLDING_LOG,
                samples,
                seed,
            };
            let rows = run_size_matrix(&config, &output_dir)
                .with_context(|| format!("writing estimates to {}", output_dir.display()))?;
            println!(
                "wrote {} best-h rows to {}",
                rows.len(),
                output_dir.display()
            );
        }
        Command::EstimateSeparate {
            log_lengths,
            batch_sizes,
            heights,
            commitment_counts,
            layer_field_bits,
            codeword_log_reductions,
            rs_interleaving_factors,
            virtual_initial_oracle,
            samples,
            seed,
            output_dir,
        } => {
            let config = SeparateSizeMatrixConfig {
                log_lengths,
                batch_sizes,
                heights,
                commitment_counts,
                layer_field_bits,
                codeword_log_reductions,
                rs_interleaving_factors,
                virtual_initial_oracle,
                samples,
                seed,
            };
            let rows = run_separate_size_matrix(&config, &output_dir).with_context(|| {
                format!(
                    "writing separate-commitment estimates to {}",
                    output_dir.display()
                )
            })?;
            println!(
                "wrote {} separate-commitment best-h rows to {}",
                rows.len(),
                output_dir.display()
            );
        }
        Command::Time {
            log_length,
            batch_sizes,
            heights,
            repetitions,
            seed,
            output,
        } => {
            let rows = run_timings(
                log_length,
                &batch_sizes,
                &heights,
                repetitions,
                seed,
                &output,
            )?;
            println!("wrote {} timing rows to {}", rows.len(), output.display());
        }
        Command::CommitTime {
            log_length,
            codeword_log_reductions,
            heights,
            repetitions,
            output,
        } => {
            anyhow::ensure!(
                codeword_log_reductions.len() == heights.len(),
                "codeword-log-reductions and heights must have equal lengths"
            );
            let configurations: Vec<_> = codeword_log_reductions.into_iter().zip(heights).collect();
            let rows = run_commitment_timings(log_length, &configurations, repetitions, &output)?;
            println!(
                "wrote {} commitment timing rows to {}",
                rows.len(),
                output.display()
            );
        }
        Command::SeparateCommitTime {
            log_length,
            commitment_count,
            batch_size,
            height,
            codeword_log_reduction,
            repetitions,
            height_search_samples,
            seed,
            output,
        } => {
            let height = match height {
                Some(height) => height,
                None => select_separate_height(
                    log_length,
                    commitment_count,
                    batch_size,
                    &SeparateHeightSearchConfig {
                        requested_heights: &[],
                        codeword_log_reduction,
                        layer_field_bits: 256,
                        samples: height_search_samples,
                        seed,
                    },
                )?,
            };
            let rows = run_separate_commitment_timings(&SeparateCommitmentTimingConfig {
                log_length,
                commitment_count,
                batch_size,
                h: height,
                codeword_log_reduction,
                repetitions,
                seed,
                output: &output,
            })?;
            println!(
                "selected h={height}, codeword=N/{}; wrote {} separate-commitment timing rows to {}",
                1usize << codeword_log_reduction,
                rows.len(),
                output.display()
            );
        }
    }
    Ok(())
}
