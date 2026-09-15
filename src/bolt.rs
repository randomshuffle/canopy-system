//! Prime-field Bolt-max experiment primitives.
//!
//! The degree-8, compression-8, all-one ensemble follows `bolt-xor` in
//! bcc-research/bolt-rs at 3832e47b24e7b3e10525c9c5bcfc1cfe66d525f2.
//! Its binary-field distance is NOT a theorem about this prime-field port.

use anyhow::{ensure, Result};
use ark_ff::AdditiveGroup;
use ark_poly::{EvaluationDomain, Radix2EvaluationDomain};
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
/// ensemble; over this field they add with their integer multiplicity.
pub struct BoltMaxEncoder {
    pub message_rows: usize,
    pub interleaving: usize,
    constraints: Vec<Vec<usize>>,
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
        let constraints = edges
            .chunks_exact(DEGREE * COMPRESSION)
            .map(|c| {
                let mut c = c.to_vec();
                c.sort_unstable();
                c
            })
            .collect();
        Ok(Self {
            message_rows,
            interleaving,
            constraints,
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
        let mut result = vec![Fr::ZERO; self.message_rows];
        for (&weight, rows) in weights.iter().zip(&self.constraints) {
            for &row in rows {
                result[row] += weight;
            }
        }
        Ok(result)
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

/// Ordinary coefficient-basis, rate-1/2 multiplicative RS, one column at a time.
/// No binary-field arithmetic or additive FFT is reused in this prime-field port.
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
    let domain = Radix2EvaluationDomain::<Fr>::new(2 * rows)
        .ok_or_else(|| anyhow::anyhow!("unsupported Bolt RS domain"))?;
    let columns = (0..width)
        .into_par_iter()
        .map(|col| {
            let mut v = vec![Fr::ZERO; 2 * rows];
            for row in 0..rows {
                v[row] = values[row * width + col];
            }
            domain.fft_in_place(&mut v);
            v
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_field_encoder_is_linear_and_rs_matches_direct_evaluation() {
        let width = 16;
        let enc = BoltMaxEncoder::new(64, width, 42).unwrap();
        let x = (0..64 * width)
            .map(|i| Fr::from(i as u64))
            .collect::<Vec<_>>();
        let weights = (0..width)
            .map(|i| Fr::from((i + 2) as u64))
            .collect::<Vec<_>>();
        let encoded = enc.encode(&x).unwrap();
        let projected = project_rows(&x, &weights);
        let u = enc.syndrome(&projected, 1).unwrap();
        assert_eq!(u, project_rows(&encoded.syndrome, &weights));
        let y = project_rows(&encoded.parity, &weights);
        let domain = Radix2EvaluationDomain::<Fr>::new(y.len()).unwrap();
        for (point, &expected) in domain.elements().zip(&y) {
            let actual = u.iter().rev().fold(Fr::ZERO, |a, &c| a * point + c);
            assert_eq!(actual, expected);
        }
        let r = (0..8)
            .map(|i| Fr::from((3 * i + 1) as u64))
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
