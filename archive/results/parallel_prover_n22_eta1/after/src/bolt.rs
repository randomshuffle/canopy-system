//! GF(2^256) Bolt-max experiment primitives.
//!
//! The degree-8, compression-8, all-one ensemble follows `bolt-xor` in
//! bcc-research/bolt-rs at 3832e47b24e7b3e10525c9c5bcfc1cfe66d525f2.
//! The experiment explicitly assumes the ensemble distance gamma = 0.013.

use anyhow::{ensure, Result};
use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rayon::prelude::*;

use crate::Fr;

pub const DEFAULT_INTERLEAVING: usize = 128;
pub const DEGREE: usize = 8;
pub const COMPRESSION: usize = 8;
pub const ASSUMED_DISTANCE: f64 = 0.013;
pub const UPSTREAM_REVISION: &str = "3832e47b24e7b3e10525c9c5bcfc1cfe66d525f2";

/// Row-major X (k x width), U=HX (k/8 x width), Y=RS(U) (k/4 x width).
pub struct EncodedBolt {
    pub syndrome: Vec<Fr>,
    pub parity: Vec<Fr>,
}

/// Public experiment setup. Parallel edges are retained, as in the reference
/// ensemble; repeated edges cancel in characteristic two.
pub struct BoltMaxEncoder {
    pub message_rows: usize,
    pub interleaving: usize,
    constraints: Vec<Vec<usize>>,
    // Each message row has exactly DEGREE incident checks. Preserve repeated
    // edges, so the gather computes the same characteristic-two sum as H.
    transpose_checks: Vec<[u32; DEGREE]>,
}

impl BoltMaxEncoder {
    pub fn new(message_rows: usize, interleaving: usize, seed: u64) -> Result<Self> {
        validate_interleaving(interleaving)?;
        ensure!(
            message_rows.is_power_of_two() && message_rows >= 64,
            "Bolt requires a power-of-two row count of at least 64"
        );
        let mut edges = (0..message_rows)
            .flat_map(|i| std::iter::repeat_n(i, DEGREE))
            .collect::<Vec<_>>();
        edges.shuffle(&mut ChaCha20Rng::seed_from_u64(seed));
        let constraints: Vec<Vec<usize>> = edges
            .chunks_exact(DEGREE * COMPRESSION)
            .map(|c| {
                let mut c = c.to_vec();
                c.sort_unstable();
                c
            })
            .collect();
        ensure!(
            constraints.len() <= u32::MAX as usize,
            "too many Bolt checks"
        );
        let mut transpose_checks = vec![[0; DEGREE]; message_rows];
        let mut degrees = vec![0u8; message_rows];
        for (check, rows) in constraints.iter().enumerate() {
            for &row in rows {
                transpose_checks[row][degrees[row] as usize] = check as u32;
                degrees[row] += 1;
            }
        }
        Ok(Self {
            message_rows,
            interleaving,
            constraints,
            transpose_checks,
        })
    }

    pub fn syndrome(&self, values: &[Fr], width: usize) -> Result<Vec<Fr>> {
        ensure!(
            width > 0 && values.len() == self.message_rows * width,
            "wrong Bolt message matrix shape"
        );
        let mut result = vec![Fr::ZERO; self.constraints.len() * width];
        result
            .par_chunks_mut(width)
            .zip(&self.constraints)
            .for_each(|(out, rows)| {
                for &row in rows {
                    for (dst, src) in out.iter_mut().zip(&values[row * width..(row + 1) * width]) {
                        *dst += src;
                    }
                }
            });
        Ok(result)
    }

    /// Apply H^T to a covector, without materializing the dense parity matrix.
    pub fn transpose(&self, weights: &[Fr]) -> Result<Vec<Fr>> {
        ensure!(
            weights.len() == self.constraints.len(),
            "wrong syndrome covector size"
        );
        Ok(self
            .transpose_checks
            .par_iter()
            .with_min_len(1024)
            .map(|checks| checks.iter().map(|&check| weights[check as usize]).sum())
            .collect())
    }

    pub fn encode(&self, values: &[Fr]) -> Result<EncodedBolt> {
        let syndrome = self.syndrome(values, self.interleaving)?;
        let parity = encode_columns(&syndrome, self.interleaving)?;
        Ok(EncodedBolt { syndrome, parity })
    }
}

pub fn validate_interleaving(width: usize) -> Result<()> {
    ensure!(
        width.is_power_of_two() && (2..=128).contains(&width),
        "Bolt interleaving must be a power of two in 2..=128"
    );
    Ok(())
}

/// Monomial-basis, rate-1/2 additive RS, one column at a time.
pub fn encode_columns(values: &[Fr], width: usize) -> Result<Vec<Fr>> {
    ensure!(
        width > 0 && values.len().is_multiple_of(width),
        "invalid matrix width"
    );
    let rows = values.len() / width;
    ensure!(
        rows.is_power_of_two(),
        "RS message row count must be a power of two"
    );
    let columns = (0..width)
        .into_par_iter()
        .map(|col| {
            let mut v = vec![Fr::ZERO; rows];
            for row in 0..rows {
                v[row] = values[row * width + col];
            }
            crate::additive_rs::encode(&v, 2 * rows)
        })
        .collect::<Vec<_>>();
    let mut output = vec![Fr::ZERO; 2 * values.len()];
    output
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|(row, out)| {
            for col in 0..width {
                out[col] = columns[col][row];
            }
        });
    Ok(output)
}

pub fn project_rows(values: &[Fr], weights: &[Fr]) -> Vec<Fr> {
    assert!(!weights.is_empty() && values.len().is_multiple_of(weights.len()));
    values
        .par_chunks_exact(weights.len())
        .map(|row| row.iter().zip(weights).map(|(&x, &w)| x * w).sum())
        .collect()
}

/// Contract the row dimension of a row-major matrix. Each task walks contiguous
/// rows and accumulates all columns, avoiding width strided passes over X.
pub fn partial_evaluation(values: &[Fr], weights: &[Fr], width: usize) -> Vec<Fr> {
    assert!(width > 0 && values.len() == weights.len() * width);
    const ROWS_PER_CHUNK: usize = 1024;
    values
        .par_chunks(width * ROWS_PER_CHUNK)
        .zip(weights.par_chunks(ROWS_PER_CHUNK))
        .map(|(rows, weights)| {
            let mut sum = vec![Fr::ZERO; width];
            for (row, &weight) in rows.chunks_exact(width).zip(weights) {
                for (dst, &value) in sum.iter_mut().zip(row) {
                    *dst += value * weight;
                }
            }
            sum
        })
        .reduce(
            || vec![Fr::ZERO; width],
            |mut a, b| {
                for (a, b) in a.iter_mut().zip(b) {
                    *a += b;
                }
                a
            },
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_contractions_match_serial_including_duplicate_edges() {
        let rows = 16384;
        let width = 4;
        let enc = BoltMaxEncoder::new(rows, width, 42).unwrap();
        let weights = (0..rows)
            .map(|i| crate::benchmark_field_element(7, 1, i as u64))
            .collect::<Vec<_>>();
        let values = (0..rows * width)
            .map(|i| crate::benchmark_field_element(7, 2, i as u64))
            .collect::<Vec<_>>();
        let expected_g = (0..width)
            .map(|col| {
                values
                    .chunks_exact(width)
                    .zip(&weights)
                    .map(|(row, &w)| row[col] * w)
                    .sum::<Fr>()
            })
            .collect::<Vec<_>>();
        let mut expected_transpose = vec![Fr::ZERO; rows];
        let mut duplicates = 0;
        for (&w, checks) in weights.iter().zip(&enc.constraints) {
            duplicates += checks.windows(2).filter(|p| p[0] == p[1]).count();
            for &row in checks {
                expected_transpose[row] += w;
            }
        }
        assert!(duplicates > 0);
        for threads in [1, 4] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap()
                .install(|| {
                    assert_eq!(partial_evaluation(&values, &weights, width), expected_g);
                    assert_eq!(
                        enc.transpose(&weights[..rows / COMPRESSION]).unwrap(),
                        expected_transpose
                    );
                    // Also exercise a final partial chunk.
                    assert_eq!(
                        partial_evaluation(&values[..4100], &weights[..1025], width),
                        (0..width)
                            .map(|col| values[..4100]
                                .chunks_exact(width)
                                .zip(&weights)
                                .map(|(row, &w)| row[col] * w)
                                .sum::<Fr>())
                            .collect::<Vec<_>>()
                    );
                });
        }
    }

    #[test]
    fn binary_field_encoder_is_linear_and_rs_matches_direct_evaluation() {
        let width = 16;
        let enc = BoltMaxEncoder::new(64, width, 42).unwrap();
        let x = (0..64 * width)
            .map(|i| crate::benchmark_field_element(42, 1, i as u64))
            .collect::<Vec<_>>();
        let weights = (0..width)
            .map(|i| crate::benchmark_field_element(42, 2, i as u64))
            .collect::<Vec<_>>();
        let encoded = enc.encode(&x).unwrap();
        let projected = project_rows(&x, &weights);
        let u = enc.syndrome(&projected, 1).unwrap();
        assert_eq!(u, project_rows(&encoded.syndrome, &weights));
        let y = project_rows(&encoded.parity, &weights);
        for (index, &expected) in y.iter().enumerate() {
            let point = crate::additive_rs::point(index);
            let actual = u.iter().rev().fold(Fr::ZERO, |a, &c| a * point + c);
            assert_eq!(actual, expected);
        }
        let r = (0..8)
            .map(|i| crate::benchmark_field_element(42, 3, i as u64))
            .collect::<Vec<_>>();
        let lhs: Fr = u.iter().zip(&r).map(|(&a, &b)| a * b).sum();
        let rhs: Fr = projected
            .iter()
            .zip(enc.transpose(&r).unwrap())
            .map(|(&a, b)| a * b)
            .sum();
        assert_eq!(lhs, rhs);
        let mut degrees = vec![0; 64];
        for constraint in &enc.constraints {
            assert_eq!(constraint.len(), 64);
            for &i in constraint {
                degrees[i] += 1;
            }
        }
        assert!(degrees.iter().all(|&d| d == DEGREE));
    }
}
