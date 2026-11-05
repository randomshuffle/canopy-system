//! Cache-local codeword RLC followed immediately by canonical row hashing.
use crate::Fr;
use rayon::prelude::*;
use whir_ref::{
    buffer::{Buffer, BufferOps},
    hash::{Hash, HashEngine},
    protocols::matrix_commit,
};

/// Retains the output codeword as a witness while hashing each completed tile.
/// The first weight must be one, as in the batching transcript.
pub(crate) fn combine_and_hash(
    inputs: &[&Buffer<Fr>],
    weights: &[Fr],
    rows: usize,
    width: usize,
    engine: &dyn HashEngine,
    tile_rows: usize,
) -> (Buffer<Fr>, Vec<Hash>) {
    assert!(!inputs.is_empty());
    assert_eq!(inputs.len(), weights.len());
    assert_eq!(weights[0], Fr::ONE);
    assert!(width > 0 && tile_rows > 0);
    let len = rows.checked_mul(width).unwrap();
    assert!(inputs.iter().all(|input| input.len() == len));
    let mut matrix = vec![Fr::ZERO; len];
    let mut leaves = vec![Hash::default(); rows];
    matrix
        .par_chunks_mut(tile_rows * width)
        .zip(leaves.par_chunks_mut(tile_rows))
        .enumerate()
        .for_each(|(tile, (dst, hashes))| {
            let start = tile * tile_rows * width;
            for (j, value) in dst.iter_mut().enumerate() {
                let i = start + j;
                *value = inputs[0].to_slice()[i]
                    + inputs
                        .iter()
                        .zip(weights)
                        .skip(1)
                        .map(|(input, &weight)| input.to_slice()[i] * weight)
                        .sum::<Fr>();
            }
            matrix_commit::hash_rows(engine, dst, hashes);
        });
    (Buffer::from(matrix), leaves)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmark_field_element as elem;
    use whir_ref::hash;
    #[test]
    fn tiled_codewords_and_hashes_match_separate_kernels() {
        let engine = hash::ENGINES.retrieve(hash::BLAKE3).unwrap();
        for (rows, width, sources) in [(1, 1, 1), (33, 1, 2), (1031, 8, 9), (19, 4, 65)] {
            let len = rows * width;
            let inputs = (0..sources)
                .map(|s| {
                    Buffer::from(
                        (0..len)
                            .map(|i| elem(91, s as u64, i as u64))
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>();
            let mut weights = (0..sources)
                .map(|s| elem(93, 7, s as u64))
                .collect::<Vec<_>>();
            weights[0] = Fr::ONE;
            let refs = inputs.iter().collect::<Vec<_>>();
            let expected = (0..len)
                .map(|i| {
                    inputs
                        .iter()
                        .zip(&weights)
                        .map(|(m, &w)| m.to_slice()[i] * w)
                        .sum::<Fr>()
                })
                .collect::<Vec<_>>();
            let mut leaves = vec![Hash::default(); rows];
            matrix_commit::hash_rows(&*engine, &expected, &mut leaves);
            for tile in [1, 16, 128, 512, 2048] {
                let (matrix, hashes) =
                    combine_and_hash(&refs, &weights, rows, width, &*engine, tile);
                assert_eq!(matrix.to_slice(), expected);
                assert_eq!(hashes, leaves);
            }
        }
    }
}
