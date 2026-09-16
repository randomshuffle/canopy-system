//! Compare encoding workloads at the same original input size and field.
//! Run: cargo run --release --example bolt_encoder_audit -- 22 4 2 5
use std::{hint::black_box, path::PathBuf, time::Instant};

use anyhow::{ensure, Result};
use ark_ff::UniformRand;
use canopy_system::{
    bolt::{self, BoltMaxEncoder},
    Fr,
};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use serde::Serialize;
use whir_ref::{
    algebra::ntt::{self, PolynomialSegment, Polynomials},
    buffer::{Buffer, BufferOps},
};

#[derive(Serialize)]
struct Row {
    log_input: usize,
    original_elements: usize,
    initial_width: usize,
    scheme: &'static str,
    encoded_message_elements: usize,
    materialized_output_elements: usize,
    rs_width: usize,
    iteration: usize,
    seconds: f64,
    rayon_threads: usize,
    seed: u64,
}

fn native_rs(input: &Buffer<Fr>, width: usize) -> Buffer<Fr> {
    let refs = [input];
    let segments = [PolynomialSegment::from_rows(&refs, width)];
    ntt::interleaved_rs_encode(
        Polynomials::from_segments(&segments),
        input.len() / width * 2,
    )
}

fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let arg = |i: usize, default: usize| -> Result<usize> {
        Ok(args
            .get(i)
            .map(|s| s.parse())
            .transpose()?
            .unwrap_or(default))
    };
    let (log_input, width, warmups, repetitions) =
        (arg(0, 22)?, arg(1, 4)?, arg(2, 2)?, arg(3, 5)?);
    ensure!(
        (13..=24).contains(&log_input) && repetitions > 0,
        "invalid audit size or repetitions"
    );
    bolt::validate_interleaving(width)?;
    let n = 1usize << log_input;
    let seed = canopy_system::bolt_experiment::DEFAULT_SEED;
    let encoder = BoltMaxEncoder::new(n / width, width, seed)?;
    let mut rng = ChaCha20Rng::seed_from_u64(seed ^ 0x4d53_47);
    let input = (0..n).map(|_| Fr::rand(&mut rng)).collect::<Vec<_>>();
    canopy_system::register_benchmark_ntt();
    // Prepare all source buffers outside the timers. Allocation of encoder
    // outputs/scratch is included; output destruction and hashing are excluded.
    let full_source = Buffer::from(input.as_slice());
    let canopy_len = n / width / 8; // h_X=3; quarter-length Canopy is rate 1/2.
    let canopy_source = Buffer::from(&input[..canopy_len]);
    let mut rows = Vec::new();
    for iteration in 0..warmups + repetitions {
        // Rotate all four positions across repetitions to vary cache/order bias.
        for offset in 0..4 {
            let which = (offset + iteration) % 4;
            let start = Instant::now();
            let (scheme, message_len, output_len, rs_width, elapsed) = match which {
                0 => {
                    let result = encoder.encode(&input)?;
                    let elapsed = start.elapsed();
                    ensure!(
                        result.syndrome.len() == n / 8 && result.parity.len() == n / 4,
                        "wrong Bolt shape"
                    );
                    black_box(&result);
                    ("bolt_max_encoding", n, n / 4, width, elapsed)
                }
                1 => {
                    let result = bolt::encode_columns(&input, width)?;
                    let elapsed = start.elapsed();
                    ensure!(result.len() == 2 * n, "wrong full RS shape");
                    black_box(&result);
                    ("full_input_rs_additive", n, 2 * n, width, elapsed)
                }
                2 => {
                    let result = native_rs(&full_source, width);
                    let elapsed = start.elapsed();
                    ensure!(result.len() == 2 * n, "wrong native RS shape");
                    black_box(&result);
                    ("full_input_rs_whir_backend", n, 2 * n, width, elapsed)
                }
                _ => {
                    let result = native_rs(&canopy_source, 8);
                    let elapsed = start.elapsed();
                    ensure!(result.len() == 2 * canopy_len, "wrong Canopy RS shape");
                    black_box(&result);
                    (
                        "canopy_systematic_rs_only",
                        canopy_len,
                        2 * canopy_len,
                        8,
                        elapsed,
                    )
                }
            };
            if iteration >= warmups {
                rows.push(Row {
                    log_input,
                    original_elements: n,
                    initial_width: width,
                    scheme,
                    encoded_message_elements: message_len,
                    materialized_output_elements: output_len,
                    rs_width,
                    iteration: iteration - warmups,
                    seconds: elapsed.as_secs_f64(),
                    rayon_threads: rayon::current_num_threads(),
                    seed,
                });
            }
        }
    }
    let output = PathBuf::from(format!("results/bolt_encoder_audit/n{log_input}_w{width}"));
    std::fs::create_dir_all(&output)?;
    let mut csv = csv::Writer::from_path(output.join("timings.csv"))?;
    for row in &rows {
        csv.serialize(row)?;
    }
    csv.flush()?;
    let mut report = format!("# Bolt versus full-input rate-1/2 RS encoding audit\n\nN=2^{log_input}, width {width}, 256-bit prime field, {} Rayon threads, {warmups} warmups and {repetitions} measured repetitions. Seed {seed}. Encoder only: no Merkle hashing, H2F, WHIR proof, or setup. Source preparation is excluded; output allocation and internal scratch are included, output destruction excluded. Four execution orders are rotated. Both full-input RS baselines have the same initial width as Bolt. The Canopy-sized RS call uses WHIR width 8.\n\nBolt emits N/4 parity elements plus the unchanged N-element systematic input; it also retains an N/8 syndrome scratch/witness. Its encoder does not copy the systematic input. Full RS emits 2N elements.\n\n| Encoder | Input field elements | New codeword elements | Mean ms | Median ms | Min ms | Max ms |\n|---|---:|---:|---:|---:|---:|---:|\n", rayon::current_num_threads());
    for scheme in [
        "bolt_max_encoding",
        "full_input_rs_additive",
        "full_input_rs_whir_backend",
        "canopy_systematic_rs_only",
    ] {
        let group = rows
            .iter()
            .filter(|r| r.scheme == scheme)
            .collect::<Vec<_>>();
        let mut times = group.iter().map(|r| r.seconds * 1000.0).collect::<Vec<_>>();
        times.sort_by(f64::total_cmp);
        let mean = times.iter().sum::<f64>() / times.len() as f64;
        let median = (times[(times.len() - 1) / 2] + times[times.len() / 2]) / 2.0;
        report.push_str(&format!(
            "| {scheme} | {} | {} | {mean:.3} | {median:.3} | {:.3} | {:.3} |\n",
            group[0].encoded_message_elements,
            group[0].materialized_output_elements,
            times[0],
            times[times.len() - 1]
        ));
    }
    report.push_str("\nThis audits the current prime-field implementations. It does not reproduce the paper's optimized binary-field CPU/GPU commitment benchmark. Small and large RS workloads need not scale linearly.\n");
    std::fs::write(output.join("summary.md"), &report)?;
    print!("{report}");
    Ok(())
}
