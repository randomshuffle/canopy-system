//! Monomial RS encoding on additive Cantor subspaces, preserving WHIR's
//! univariate-evaluation semantics. Novel-basis conversion is internal only.
use crate::binary_field::Binary256 as F;
use ark_ff::Field;
use rayon::prelude::*;
use std::sync::LazyLock;
use whir_ref::{
    algebra::ntt::{Polynomials, ReedSolomon},
    buffer::{Buffer, BufferOps},
};

#[derive(Debug)]
pub struct AdditiveRs;
struct Cantor {
    #[cfg(test)]
    basis: Vec<F>,
    tables: Vec<Vec<F>>,
}
static CANTOR: LazyLock<Cantor> = LazyLock::new(|| {
    // Solve L(x)=x²+x by binary Gaussian elimination. Reuse the reduced
    // operator for all basis vectors; these public setup costs are untimed.
    let columns: Vec<_> = (0..256)
        .map(|i| {
            let x = F::basis(i);
            x.square() + x
        })
        .collect();
    let mut rows: Vec<(F, F)> = (0..256)
        .map(|i| {
            let mut r = F::ZERO;
            for j in 0..256 {
                if columns[j].bit(i) {
                    r += F::basis(j)
                }
            }
            (r, F::basis(i))
        })
        .collect();
    let mut pivots = Vec::new();
    for col in 0..256 {
        let rank = pivots.len();
        if let Some(pivot) = (rank..256).find(|&r| rows[r].0.bit(col)) {
            rows.swap(rank, pivot);
            let p = rows[rank];
            for (i, row) in rows.iter_mut().enumerate() {
                if i != rank && row.0.bit(col) {
                    row.0 += p.0;
                    row.1 += p.1
                }
            }
            pivots.push(col);
        }
    }
    assert_eq!(pivots.len(), 255);
    let parity = |a: F, b: F| ((a.0[0] & b.0[0]).count_ones() + (a.0[1] & b.0[1]).count_ones()) & 1;
    let mut basis = vec![F::ONE];
    for i in 1..32 {
        let rhs = basis[i - 1];
        assert_eq!(parity(rows[255].1, rhs), 0);
        let mut x = F::ZERO;
        for (r, &col) in pivots.iter().enumerate() {
            if parity(rows[r].1, rhs) != 0 {
                x += F::basis(col)
            }
        }
        assert_eq!(x.square() + x, rhs);
        basis.push(x);
    }
    let tables = (0..4)
        .map(|block| {
            (0..256)
                .map(|mask| {
                    let mut x = F::ZERO;
                    for bit in 0..8 {
                        if (mask >> bit) & 1 != 0 {
                            x += basis[block * 8 + bit]
                        }
                    }
                    x
                })
                .collect()
        })
        .collect();
    Cantor {
        #[cfg(test)]
        basis,
        tables,
    }
});
pub fn setup() {
    LazyLock::force(&CANTOR);
}
#[inline(always)]
pub fn point(index: usize) -> F {
    let t = &CANTOR.tables;
    t[0][index & 255]
        + t[1][(index >> 8) & 255]
        + t[2][(index >> 16) & 255]
        + t[3][(index >> 24) & 255]
}

// Recursive children already expose substantial parallelism. Only split large
// within-node sweeps, so nested FFTs do not flood the pool with tiny jobs.
const PARALLEL_SWEEP: usize = 1 << 16;

#[inline(always)]
fn butterflies<const TRANSPOSE: bool>(left: &mut [F], right: &mut [F], c: F) {
    let butterfly = |(a, b): (&mut F, &mut F)| {
        let old_b = *b;
        if TRANSPOSE {
            *a += old_b;
            *b = c * *a + old_b;
        } else {
            *a += c * old_b;
            *b = *a + old_b;
        }
    };
    if left.len() >= PARALLEL_SWEEP {
        left.par_iter_mut()
            .zip(right.par_iter_mut())
            .with_min_len(4096)
            .for_each(butterfly);
    } else {
        left.iter_mut().zip(right.iter_mut()).for_each(butterfly);
    }
}

/// Convert monomials into products of s_i(X), where s_0=X and
/// s_{i+1}=s_i²+s_i. Each s_i has only binary coefficients.
fn monomial_to_novel(a: &mut [F]) {
    if a.len() <= 2 {
        return;
    }
    let half = a.len() / 2;
    let h = half.ilog2();
    let terms: Vec<usize> = (0..h)
        .filter(|j| j & h == *j)
        .map(|j| 1usize << j)
        .collect();
    if half >= PARALLEL_SWEEP {
        // Long division has dependencies: i writes only at most i-gap. In a
        // descending block of <= gap sources, every destination precedes the
        // block. Terms remain ordered because their destination ranges overlap.
        let gap = half - terms.last().copied().unwrap();
        let mut end = a.len();
        while end > half {
            let start = half.max(end - gap);
            let (dest, source) = a.split_at_mut(start);
            for &term in &terms {
                dest[start - half + term..end - half + term]
                    .par_iter_mut()
                    .zip(source[..end - start].par_iter())
                    .with_min_len(4096)
                    .for_each(|(dst, &src)| *dst += src);
            }
            end = start;
        }
    } else {
        for i in (half..a.len()).rev() {
            let q = a[i];
            for &term in &terms {
                a[i - half + term] += q
            }
        }
    }
    let (left, right) = a.split_at_mut(half);
    if half >= 8192 {
        rayon::join(|| monomial_to_novel(left), || monomial_to_novel(right));
    } else {
        monomial_to_novel(left);
        monomial_to_novel(right);
    }
}
fn forward(a: &mut [F], offset: usize) {
    if a.len() == 1 {
        return;
    }
    let half = a.len() / 2;
    let h = half.ilog2();
    let twiddle = point(offset >> h);
    let (left, right) = a.split_at_mut(half);
    butterflies::<false>(left, right, twiddle);
    if half >= 8192 {
        rayon::join(|| forward(left, offset), || forward(right, offset + half));
    } else {
        forward(left, offset);
        forward(right, offset + half);
    }
}
pub fn encode(coefficients: &[F], length: usize) -> Vec<F> {
    assert!(!coefficients.is_empty() && length.is_power_of_two() && coefficients.len() <= length);
    assert!(length <= 1usize << 32);
    let size = coefficients.len().next_power_of_two();
    let mut novel = coefficients.to_vec();
    novel.resize(size, F::ZERO);
    monomial_to_novel(&mut novel);
    let mut out = vec![F::ZERO; length];
    out.par_chunks_mut(size)
        .enumerate()
        .for_each(|(coset, dst)| {
            dst.copy_from_slice(&novel);
            forward(dst, coset * size);
        });
    out
}
// Transposes of the exact linear maps above; WHIR needs weighted monomial
// power sums, not evaluations in the novel basis.
fn forward_transpose(a: &mut [F], offset: usize) {
    if a.len() == 1 {
        return;
    }
    let half = a.len() / 2;
    let h = half.ilog2();
    let c = point(offset >> h);
    let (left, right) = a.split_at_mut(half);
    if half >= 8192 {
        rayon::join(
            || forward_transpose(left, offset),
            || forward_transpose(right, offset + half),
        );
    } else {
        forward_transpose(left, offset);
        forward_transpose(right, offset + half);
    }
    butterflies::<true>(left, right, c);
}
fn conversion_transpose(a: &mut [F]) {
    if a.len() <= 2 {
        return;
    }
    let half = a.len() / 2;
    let h = half.ilog2();
    let (left, right) = a.split_at_mut(half);
    if half >= 8192 {
        rayon::join(
            || conversion_transpose(left),
            || conversion_transpose(right),
        );
    } else {
        conversion_transpose(left);
        conversion_transpose(right);
    }
    let terms: Vec<usize> = (0..h)
        .filter(|j| j & h == *j)
        .map(|j| 1usize << j)
        .collect();
    if half >= PARALLEL_SWEEP {
        // Transpose the same dependency blocks in the opposite order. Each
        // output can gather all its terms independently from the earlier prefix.
        let gap = half - terms.last().copied().unwrap();
        for start in (half..a.len()).step_by(gap) {
            let end = a.len().min(start + gap);
            let (source, dest) = a.split_at_mut(start);
            dest[..end - start]
                .par_iter_mut()
                .enumerate()
                .with_min_len(4096)
                .for_each(|(i, dst)| {
                    for &term in &terms {
                        *dst += source[start + i - half + term];
                    }
                });
        }
    } else {
        for i in half..a.len() {
            for &term in &terms {
                let delta = a[i - half + term];
                a[i] += delta
            }
        }
    }
}
pub fn power_sums(indices: &[usize], weights: &[F], length: usize, prefix: usize) -> Buffer<F> {
    let size = prefix.next_power_of_two();
    let mut a = vec![F::ZERO; length];
    for (&i, &weight) in indices.iter().zip(weights) {
        a[i] += weight;
    }
    a.par_chunks_mut(size)
        .enumerate()
        .for_each(|(coset, dst)| forward_transpose(dst, coset * size));
    let (first, rest) = a.split_at_mut(size);
    if size >= PARALLEL_SWEEP {
        first
            .par_chunks_mut(1024)
            .enumerate()
            .for_each(|(block, dst)| {
                let start = block * 1024;
                let end = start + dst.len();
                for chunk in rest.chunks_exact(size) {
                    for (a, &b) in dst.iter_mut().zip(&chunk[start..end]) {
                        *a += b;
                    }
                }
            });
    } else {
        for chunk in rest.chunks_exact(size) {
            for (a, &b) in first.iter_mut().zip(chunk) {
                *a += b
            }
        }
    }
    conversion_transpose(first);
    Buffer::from(first[..prefix].to_vec())
}
impl ReedSolomon<F> for AdditiveRs {
    fn additive_power_sums(
        &self,
        points: &[F],
        indices: &[usize],
        weights: &[F],
        length: usize,
        prefix: usize,
    ) -> Option<Buffer<F>> {
        if points.len() != indices.len()
            || indices.len() != weights.len()
            || prefix == 0
            || prefix > length
            || !length.is_power_of_two()
            || !points
                .iter()
                .zip(indices)
                .all(|(&x, &i)| i < length && x == point(i))
        {
            return None;
        }
        Some(power_sums(indices, weights, length, prefix))
    }
    fn next_order(&self, n: usize) -> Option<usize> {
        let order = n.checked_next_power_of_two()?;
        (order <= 1usize << 32).then_some(order)
    }
    fn generator(&self, _: usize) -> F {
        panic!("an additive RS domain has no multiplicative generator")
    }
    fn evaluation_points(&self, poly_len: usize, length: usize, indices: &[usize]) -> Vec<F> {
        assert!(poly_len <= length && length.is_power_of_two());
        indices
            .iter()
            .map(|&i| {
                assert!(i < length);
                point(i)
            })
            .collect()
    }
    fn evaluation_exponents(&self, _: usize, _: usize, indices: &[usize]) -> Vec<usize> {
        indices.to_vec()
    }
    fn interleaved_encode(&self, polys: Polynomials<'_, F>, length: usize) -> Buffer<F> {
        let columns: Vec<Vec<F>> = (0..polys.len())
            .into_par_iter()
            .map(|col| {
                let mut coefficients = Vec::with_capacity(polys.polynomial_length());
                for segment in polys.segments() {
                    let buffer = segment.buffer(col / segment.rows_per_buffer()).to_slice();
                    let start = (col % segment.rows_per_buffer()) * segment.row_width();
                    coefficients.extend_from_slice(&buffer[start..start + segment.row_width()]);
                }
                encode(&coefficients, length)
            })
            .collect();
        let mut matrix = vec![F::ZERO; length * columns.len()];
        if !columns.is_empty() {
            matrix
                .par_chunks_mut(columns.len() * 1024)
                .enumerate()
                .for_each(|(block, dst)| {
                    for (row, dst) in dst.chunks_exact_mut(columns.len()).enumerate() {
                        for (col, value) in dst.iter_mut().enumerate() {
                            *value = columns[col][block * 1024 + row]
                        }
                    }
                });
        }
        Buffer::from(matrix)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, SeedableRng};

    // Keep the original serial long-division maps as independent references:
    // reordering these dependency sweeps incorrectly can still look linear.
    fn serial_conversion(a: &mut [F], transpose: bool) {
        if a.len() <= 2 {
            return;
        }
        let half = a.len() / 2;
        let h = half.ilog2();
        let terms = (0..h)
            .filter(|j| j & h == *j)
            .map(|j| 1usize << j)
            .collect::<Vec<_>>();
        if transpose {
            serial_conversion(&mut a[..half], true);
            serial_conversion(&mut a[half..], true);
            for i in half..a.len() {
                for &term in &terms {
                    let delta = a[i - half + term];
                    a[i] += delta;
                }
            }
        } else {
            for i in (half..a.len()).rev() {
                let q = a[i];
                for &term in &terms {
                    a[i - half + term] += q;
                }
            }
            serial_conversion(&mut a[..half], false);
            serial_conversion(&mut a[half..], false);
        }
    }

    #[test]
    fn parallel_dependency_sweeps_match_serial_conversion() {
        let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(97);
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        for log in [13, 14, 15, 16, 17, 18, 19] {
            let a = (0..1usize << log)
                .map(|_| rng.gen::<F>())
                .collect::<Vec<_>>();
            for transpose in [false, true] {
                let mut expected = a.clone();
                serial_conversion(&mut expected, transpose);
                let mut actual = a.clone();
                pool.install(|| {
                    if transpose {
                        conversion_transpose(&mut actual)
                    } else {
                        monomial_to_novel(&mut actual)
                    }
                });
                assert_eq!(actual, expected, "log={log}, transpose={transpose}");
            }
        }
    }

    #[test]
    fn parallel_fft_matches_horner_and_transpose_adjoint() {
        let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(101);
        let coefficients = (0..65539).map(|_| rng.gen::<F>()).collect::<Vec<_>>();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        pool.install(|| {
            let length = 1 << 19;
            let encoded = encode(&coefficients, length);
            let indices = [
                0,
                1,
                8191,
                8192,
                32767,
                32768,
                32768,
                65535,
                65536,
                length - 1,
            ];
            let weights = indices.iter().map(|_| rng.gen::<F>()).collect::<Vec<_>>();
            for &i in &indices {
                let expected = coefficients
                    .iter()
                    .rev()
                    .fold(F::ZERO, |a, &c| a * point(i) + c);
                assert_eq!(encoded[i], expected, "index={i}");
            }
            let sums = power_sums(&indices, &weights, length, coefficients.len());
            let lhs: F = indices
                .iter()
                .zip(&weights)
                .map(|(&i, &w)| encoded[i] * w)
                .sum();
            let rhs: F = coefficients
                .iter()
                .zip(sums.to_slice())
                .map(|(&a, &b)| a * b)
                .sum();
            assert_eq!(lhs, rhs);
        });
    }
    #[test]
    fn transposed_encoder_matches_direct_power_sums() {
        let indices = vec![0, 1, 2, 7, 13, 31, 31, 67, 99, 127];
        let weights: Vec<_> = (0..indices.len())
            .map(|i| crate::benchmark_field_element(33, 1, i as u64))
            .collect();
        for prefix in [1, 2, 8, 17, 64, 128] {
            let sums = power_sums(&indices, &weights, 128, prefix);
            for i in 0..prefix {
                let expected: F = indices
                    .iter()
                    .zip(&weights)
                    .map(|(&j, &w)| point(j).pow([i as u64]) * w)
                    .sum();
                assert_eq!(sums.to_slice()[i], expected);
            }
        }
    }
    #[test]
    fn cantor_basis_and_rs_match_horner() {
        setup();
        for i in 1..32 {
            assert_eq!(
                CANTOR.basis[i].square() + CANTOR.basis[i],
                CANTOR.basis[i - 1]
            );
        }
        let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
        for log in 0..10 {
            let coeffs: Vec<F> = (0..1usize << log).map(|_| rng.gen()).collect();
            let code = encode(&coeffs, 1usize << (log + 2));
            for index in (0..code.len()).step_by((code.len() / 31).max(1)) {
                let x = point(index);
                let expected = coeffs.iter().rev().fold(F::ZERO, |a, &c| a * x + c);
                assert_eq!(code[index], expected, "log={log} index={index}");
            }
        }
    }
}
