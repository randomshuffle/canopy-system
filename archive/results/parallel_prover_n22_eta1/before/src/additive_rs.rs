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
    for i in (half..a.len()).rev() {
        let q = a[i];
        for &term in &terms {
            a[i - half + term] += q
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
    for (a, b) in left.iter_mut().zip(right.iter_mut()) {
        let old_b = *b;
        *a += twiddle * old_b;
        *b = *a + old_b;
    }
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
    for (a, b) in left.iter_mut().zip(right.iter_mut()) {
        let r = *b;
        *a += r;
        *b = c * (*a) + r;
    }
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
    for i in half..a.len() {
        for &term in &terms {
            let delta = a[i - half + term];
            a[i] += delta
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
    for chunk in rest.chunks_exact(size) {
        for (a, &b) in first.iter_mut().zip(chunk) {
            *a += b
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
