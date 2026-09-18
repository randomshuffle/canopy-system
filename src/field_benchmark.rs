//! Apples-to-apples RS and hash-to-field benchmarks for candidate large fields.
#![allow(unexpected_cfgs)] // `MontConfig`'s generated asm cfg is dependency-owned.

use std::{
    fs::{self, File},
    hint::black_box,
    io::Write,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{ensure, Result};
use ark_ff::{FftField, Fp256, MontBackend, MontConfig, PrimeField};
use rayon::prelude::*;
use serde::Serialize;
use whir_ref::{
    algebra::ntt::{self, PolynomialSegment, Polynomials, ReedSolomon, NTT},
    buffer::{Buffer, BufferOps, DefaultRs},
};

use crate::PrimeFr as Fr;

/// BN254 scalar field. These are the parameters used by arkworks' `ark-bn254`.
#[derive(MontConfig)]
#[modulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[generator = "5"]
#[small_subgroup_base = "3"]
#[small_subgroup_power = "2"]
pub struct Bn254FrConfig;
pub type Bn254Fr = Fp256<MontBackend<Bn254FrConfig, 4>>;

/// BLS12-381 scalar field. These are the parameters used by arkworks'
/// `ark-bls12-381`.
#[derive(MontConfig)]
#[modulus = "52435875175126190479447740508185965837690552500527637822603658699938581184513"]
#[generator = "7"]
#[small_subgroup_base = "3"]
#[small_subgroup_power = "1"]
pub struct Bls12381FrConfig;
pub type Bls12381Fr = Fp256<MontBackend<Bls12381FrConfig, 4>>;

#[derive(Clone, Debug, Serialize)]
pub struct FieldBenchmarkRow {
    pub field: String,
    pub modulus_bits: u32,
    pub two_adicity: u32,
    pub log_input_length: usize,
    pub input_elements: usize,
    pub folding_log: usize,
    pub interleaving_width: usize,
    pub log_inverse_rate: usize,
    pub encoded_elements: usize,
    pub h2f_roots: usize,
    pub warmups: usize,
    pub repetitions: usize,
    pub mean_rs_seconds: f64,
    pub median_rs_seconds: f64,
    pub min_rs_seconds: f64,
    pub max_rs_seconds: f64,
    pub rs_speedup_vs_current: f64,
    pub mean_h2f_seconds: f64,
    pub median_h2f_seconds: f64,
    pub min_h2f_seconds: f64,
    pub max_h2f_seconds: f64,
    pub mean_h2f_ns_per_root: f64,
    pub h2f_speedup_vs_current: f64,
}

struct RawFieldBenchmark {
    field: &'static str,
    modulus_bits: u32,
    two_adicity: u32,
    rs: Vec<Duration>,
    h2f: Vec<Duration>,
}

fn duration_stats(values: &[Duration]) -> (f64, f64, f64, f64) {
    let mut seconds = values.iter().map(Duration::as_secs_f64).collect::<Vec<_>>();
    seconds.sort_by(f64::total_cmp);
    let mean = seconds.iter().sum::<f64>() / seconds.len() as f64;
    let median = if seconds.len().is_multiple_of(2) {
        (seconds[seconds.len() / 2 - 1] + seconds[seconds.len() / 2]) / 2.0
    } else {
        seconds[seconds.len() / 2]
    };
    (mean, median, seconds[0], *seconds.last().unwrap())
}

fn encode_once<F: PrimeField + 'static>(
    input: &[F],
    interleaving_width: usize,
    log_inverse_rate: usize,
) -> Duration {
    let source = Buffer::from(input);
    let source_refs = [&source];
    let segments = [PolynomialSegment::from_rows(
        &source_refs,
        interleaving_width,
    )];
    let polynomials = Polynomials::from_segments(&segments);
    let row_count = (input.len() / interleaving_width) << log_inverse_rate;
    let start = Instant::now();
    let encoded = ntt::interleaved_rs_encode(polynomials, row_count);
    let elapsed = start.elapsed();
    assert_eq!(encoded.len(), input.len() << log_inverse_rate);
    black_box(encoded);
    elapsed
}

fn hash_to_field<F: PrimeField>(root: &[u8; 32]) -> F {
    for counter in 0u32.. {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"canopy-system/hash-to-field/v2");
        hasher.update(root);
        hasher.update(&counter.to_le_bytes());
        if let Some(value) = F::from_random_bytes(hasher.finalize().as_bytes()) {
            return value;
        }
    }
    unreachable!("the 32-bit H2F rejection counter cannot be exhausted")
}

fn h2f_once<F: PrimeField>(roots: &[[u8; 32]]) -> Duration {
    let start = Instant::now();
    let values = roots.par_iter().map(hash_to_field::<F>).collect::<Vec<_>>();
    let elapsed = start.elapsed();
    black_box(values);
    elapsed
}

fn measure_field_sample<F: PrimeField + 'static>(
    input: &[F],
    roots: &[[u8; 32]],
    interleaving_width: usize,
    log_inverse_rate: usize,
) -> (Duration, Duration) {
    (
        encode_once(input, interleaving_width, log_inverse_rate),
        h2f_once::<F>(roots),
    )
}

fn write_csv(path: impl AsRef<Path>, rows: &[FieldBenchmarkRow]) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

/// Benchmark WHIR's exact interleaved RS encoder and Canopy's exact parallel
/// H2F implementation over the three candidate fields.
pub fn run_field_benchmarks(
    output_dir: &Path,
    log_length: usize,
    folding_log: usize,
    canopy_height: usize,
    warmups: usize,
    repetitions: usize,
) -> Result<Vec<FieldBenchmarkRow>> {
    ensure!(repetitions > 0, "repetitions must be positive");
    ensure!(folding_log > 0, "folding log must be positive");
    ensure!(
        canopy_height >= folding_log,
        "Canopy height must be at least the folding log"
    );
    ensure!(
        log_length >= folding_log,
        "input is shorter than one interleaving row"
    );
    const LOG_INVERSE_RATE: usize = 1;
    let interleaving_width = 1usize << folding_log;
    let encoded_elements = 1usize << (log_length + LOG_INVERSE_RATE);
    let h2f_roots = encoded_elements >> canopy_height;
    let roots = (0..h2f_roots)
        .map(|index| {
            let mut root = [0u8; 32];
            root[..8].copy_from_slice(&(index as u64).to_le_bytes());
            root[8..16].copy_from_slice(&(!(index as u64)).to_le_bytes());
            root
        })
        .collect::<Vec<_>>();

    NTT.insert(Arc::new(DefaultRs::<Fr>::new_from_fftfield()) as Arc<dyn ReedSolomon<Fr>>);
    NTT.insert(
        Arc::new(DefaultRs::<Bn254Fr>::new_from_fftfield())
            as Arc<dyn ReedSolomon<Bn254Fr>>,
    );
    NTT.insert(
        Arc::new(DefaultRs::<Bls12381Fr>::new_from_fftfield()) as Arc<dyn ReedSolomon<Bls12381Fr>>
    );

    let current_input = (0..1usize << log_length)
        .map(|value| Fr::from(value as u64))
        .collect::<Vec<_>>();
    let bn254_input = (0..1usize << log_length)
        .map(|value| Bn254Fr::from(value as u64))
        .collect::<Vec<_>>();
    let bls12381_input = (0..1usize << log_length)
        .map(|value| Bls12381Fr::from(value as u64))
        .collect::<Vec<_>>();
    let mut current_rs = Vec::with_capacity(repetitions);
    let mut current_h2f = Vec::with_capacity(repetitions);
    let mut bn254_rs = Vec::with_capacity(repetitions);
    let mut bn254_h2f = Vec::with_capacity(repetitions);
    let mut bls12381_rs = Vec::with_capacity(repetitions);
    let mut bls12381_h2f = Vec::with_capacity(repetitions);

    macro_rules! measure {
        ($field:ty, $input:expr, $rs:ident, $h2f:ident, $record:expr) => {{
            let (rs_time, h2f_time) = measure_field_sample::<$field>(
                $input,
                &roots,
                interleaving_width,
                LOG_INVERSE_RATE,
            );
            if $record {
                $rs.push(rs_time);
                $h2f.push(h2f_time);
            } else {
                black_box((rs_time, h2f_time));
            }
        }};
    }
    for round in 0..warmups + repetitions {
        let record = round >= warmups;
        // Rotate all six field orders to suppress systematic cache, clock,
        // and thermal bias from always benchmarking one field first.
        match round % 6 {
            0 => {
                measure!(Fr, &current_input, current_rs, current_h2f, record);
                measure!(Bn254Fr, &bn254_input, bn254_rs, bn254_h2f, record);
                measure!(
                    Bls12381Fr,
                    &bls12381_input,
                    bls12381_rs,
                    bls12381_h2f,
                    record
                );
            }
            1 => {
                measure!(Fr, &current_input, current_rs, current_h2f, record);
                measure!(
                    Bls12381Fr,
                    &bls12381_input,
                    bls12381_rs,
                    bls12381_h2f,
                    record
                );
                measure!(Bn254Fr, &bn254_input, bn254_rs, bn254_h2f, record);
            }
            2 => {
                measure!(Bn254Fr, &bn254_input, bn254_rs, bn254_h2f, record);
                measure!(Fr, &current_input, current_rs, current_h2f, record);
                measure!(
                    Bls12381Fr,
                    &bls12381_input,
                    bls12381_rs,
                    bls12381_h2f,
                    record
                );
            }
            3 => {
                measure!(Bn254Fr, &bn254_input, bn254_rs, bn254_h2f, record);
                measure!(
                    Bls12381Fr,
                    &bls12381_input,
                    bls12381_rs,
                    bls12381_h2f,
                    record
                );
                measure!(Fr, &current_input, current_rs, current_h2f, record);
            }
            4 => {
                measure!(
                    Bls12381Fr,
                    &bls12381_input,
                    bls12381_rs,
                    bls12381_h2f,
                    record
                );
                measure!(Fr, &current_input, current_rs, current_h2f, record);
                measure!(Bn254Fr, &bn254_input, bn254_rs, bn254_h2f, record);
            }
            _ => {
                measure!(
                    Bls12381Fr,
                    &bls12381_input,
                    bls12381_rs,
                    bls12381_h2f,
                    record
                );
                measure!(Bn254Fr, &bn254_input, bn254_rs, bn254_h2f, record);
                measure!(Fr, &current_input, current_rs, current_h2f, record);
            }
        }
    }

    let raw = vec![
        RawFieldBenchmark {
            field: "current_256_bit",
            modulus_bits: Fr::MODULUS_BIT_SIZE,
            two_adicity: Fr::TWO_ADICITY,
            rs: current_rs,
            h2f: current_h2f,
        },
        RawFieldBenchmark {
            field: "bn254_scalar",
            modulus_bits: Bn254Fr::MODULUS_BIT_SIZE,
            two_adicity: Bn254Fr::TWO_ADICITY,
            rs: bn254_rs,
            h2f: bn254_h2f,
        },
        RawFieldBenchmark {
            field: "bls12_381_scalar",
            modulus_bits: Bls12381Fr::MODULUS_BIT_SIZE,
            two_adicity: Bls12381Fr::TWO_ADICITY,
            rs: bls12381_rs,
            h2f: bls12381_h2f,
        },
    ];
    let current_rs_mean = duration_stats(&raw[0].rs).0;
    let current_h2f_mean = duration_stats(&raw[0].h2f).0;
    let rows = raw
        .into_iter()
        .map(|raw| {
            let (mean_rs, median_rs, min_rs, max_rs) = duration_stats(&raw.rs);
            let (mean_h2f, median_h2f, min_h2f, max_h2f) = duration_stats(&raw.h2f);
            FieldBenchmarkRow {
                field: raw.field.to_owned(),
                modulus_bits: raw.modulus_bits,
                two_adicity: raw.two_adicity,
                log_input_length: log_length,
                input_elements: 1usize << log_length,
                folding_log,
                interleaving_width,
                log_inverse_rate: LOG_INVERSE_RATE,
                encoded_elements,
                h2f_roots,
                warmups,
                repetitions,
                mean_rs_seconds: mean_rs,
                median_rs_seconds: median_rs,
                min_rs_seconds: min_rs,
                max_rs_seconds: max_rs,
                rs_speedup_vs_current: current_rs_mean / mean_rs,
                mean_h2f_seconds: mean_h2f,
                median_h2f_seconds: median_h2f,
                min_h2f_seconds: min_h2f,
                max_h2f_seconds: max_h2f,
                mean_h2f_ns_per_root: mean_h2f * 1e9 / h2f_roots as f64,
                h2f_speedup_vs_current: current_h2f_mean / mean_h2f,
            }
        })
        .collect::<Vec<_>>();

    fs::create_dir_all(output_dir)?;
    write_csv(output_dir.join("field_benchmarks.csv"), &rows)?;
    let mut report = File::create(output_dir.join("field_benchmarks.md"))?;
    writeln!(report, "# Large-field RS and H2F benchmark\n")?;
    writeln!(
        report,
        "WHIR interleaved rate-1/2 RS encoding: `2^{log_length}` inputs to `2^{}` codeword elements, folding width `{interleaving_width}`. This historical prime-field microbenchmark uses the former Canopy domain-separated BLAKE3 256-bit rejection-sampling method on `{h2f_roots}` roots (the Experiment-3 `h={canopy_height}` workload). Times are release-build wall times after `{warmups}` warmup(s), with `{repetitions}` measured repetitions; the harness rotates all six field orders. Output allocation is included; input generation and output deallocation are excluded.\n",
        log_length + LOG_INVERSE_RATE,
    )?;
    writeln!(
        report,
        "| field | bits | 2-adicity | RS mean ms | RS median ms | RS speedup | H2F mean ms | H2F median ms | H2F ns/root | H2F speedup |"
    )?;
    writeln!(report, "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|")?;
    for row in &rows {
        writeln!(
            report,
            "| {} | {} | {} | {:.3} | {:.3} | {:.3}x | {:.3} | {:.3} | {:.1} | {:.3}x |",
            row.field,
            row.modulus_bits,
            row.two_adicity,
            row.mean_rs_seconds * 1e3,
            row.median_rs_seconds * 1e3,
            row.rs_speedup_vs_current,
            row.mean_h2f_seconds * 1e3,
            row.median_h2f_seconds * 1e3,
            row.mean_h2f_ns_per_root,
            row.h2f_speedup_vs_current,
        )?;
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candidate_field_parameters_are_expected() {
        assert_eq!(Bn254Fr::MODULUS_BIT_SIZE, 254);
        assert_eq!(Bn254Fr::TWO_ADICITY, 28);
        assert_eq!(Bls12381Fr::MODULUS_BIT_SIZE, 255);
        assert_eq!(Bls12381Fr::TWO_ADICITY, 32);
    }
}
