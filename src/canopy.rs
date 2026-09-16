use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use anyhow::{ensure, Result};
use ark_ff::BigInt;
use rayon::prelude::*;
use whir_ref::{
    buffer::BufferOps,
    hash::{Hash as WhirHash, ENGINES},
    protocols::matrix_commit,
};

use crate::executable_whir::{
    commit_matrix_with_whir, encode_many_interleaved_with_whir, WhirEncodedMatrix,
    WhirMatrixCommitment,
};
use crate::merkle::{reconstruct_roots, Digest, MerkleLevels};
use crate::reed_solomon;
use crate::whir::{
    estimate_proof, WhirFieldProfile, WhirParameters, WhirProofSize, DEFAULT_FOLDING_FACTOR,
};
use crate::{Fr, FIELD_BYTES, HASH_BYTES};

/// WHIR groups one folding column into each Merkle leaf.
pub const WHIR_LEAF_WIDTH: usize = DEFAULT_FOLDING_FACTOR;

pub(crate) fn hash_bytes_to_field(root: &[u8; HASH_BYTES]) -> Fr {
    // All 256-bit strings represent binary-field elements. Keep the v2 input
    // framing (including counter zero) to preserve the measured H2F workload.
    for counter in 0u32.. {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"canopy-system/hash-to-field/v2");
        hasher.update(root);
        hasher.update(&counter.to_le_bytes());
        let bytes = hasher.finalize();
        let bytes = bytes.as_bytes();
        let candidate = BigInt::<4>::new([
            u64::from_le_bytes(bytes[0..8].try_into().unwrap()),
            u64::from_le_bytes(bytes[8..16].try_into().unwrap()),
            u64::from_le_bytes(bytes[16..24].try_into().unwrap()),
            u64::from_le_bytes(bytes[24..32].try_into().unwrap()),
        ]);
        if let Some(value) = Fr::from_bigint(candidate) {
            return value;
        }
    }
    unreachable!("the 32-bit H2F rejection counter cannot be exhausted")
}

fn hash_to_field(root: &Digest) -> Fr {
    hash_bytes_to_field(root)
}

/// Hash one Merkle parent layer with the same BLAKE3 engine and 64-byte
/// `left || right` input used by WHIR's Merkle tree.
fn whir_parent_level(children: &[WhirHash]) -> Result<Vec<WhirHash>> {
    ensure!(
        children.len().is_power_of_two() && children.len() >= 2,
        "WHIR Merkle layer must have a positive power-of-two size"
    );
    let engine = ENGINES
        .retrieve(whir_ref::hash::BLAKE3)
        .ok_or_else(|| anyhow::anyhow!("WHIR BLAKE3 engine is unavailable"))?;
    let batch = engine.preferred_batch_size().max(256);
    let mut parents = vec![WhirHash::default(); children.len() / 2];
    parents
        .par_chunks_mut(batch)
        .enumerate()
        .for_each(|(chunk_index, output)| {
            let first_parent = chunk_index * batch;
            let child_slice = &children[2 * first_parent..2 * (first_parent + output.len())];
            let mut input = Vec::with_capacity(child_slice.len() * HASH_BYTES);
            for child in child_slice {
                input.extend_from_slice(&child.0);
            }
            engine.hash_many(2 * HASH_BYTES, &input, output);
        });
    Ok(parents)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanopyCommitment {
    pub root: Digest,
    pub vector_len: usize,
    pub height: usize,
    pub rs_dimension: usize,
    pub rs_length: usize,
    pub codeword_log_reduction: usize,
    pub whir_leaf_width: usize,
}

#[derive(Clone, Debug)]
pub struct CanopyProverData {
    bottom_forest: MerkleLevels,
    pub canopy_message: Vec<Fr>,
    pub codeword: Vec<Fr>,
    pub code_tree: MerkleLevels,
}

/// Canopy witness data whose base codeword and recursive codeword use WHIR's
/// exact interleaved RS layout and matrix/Merkle commitment implementation.
#[derive(Clone, Debug)]
pub struct WhirCanopyProverData {
    /// WHIR row hashes followed by the retained bottom forest levels. Level
    /// zero hashes rows of `base_codeword`; the final level is the Canopy
    /// message before H2F.
    bottom_levels: Vec<Vec<WhirHash>>,
    pub canopy_message: Vec<Fr>,
    pub codeword: WhirEncodedMatrix,
    pub codeword_commitment: WhirMatrixCommitment,
}

#[derive(Clone, Debug)]
pub struct CanopyCommitmentProfile {
    pub total: Duration,
    pub bottom_forest: Duration,
    pub hash_to_field: Duration,
    pub reed_solomon_encoding: Duration,
    pub codeword_merkle_tree: Duration,
}

impl CanopyCommitmentProfile {
    pub fn accounted(&self) -> Duration {
        self.bottom_forest
            + self.hash_to_field
            + self.reed_solomon_encoding
            + self.codeword_merkle_tree
    }

    pub fn other(&self) -> Duration {
        self.total.saturating_sub(self.accounted())
    }
}

#[derive(Clone, Debug)]
pub struct CanopyBottomOpening {
    pub values: Vec<Fr>,
    pub auth_hashes: Vec<Digest>,
}

impl CanopyBottomOpening {
    pub fn hash_bytes(&self) -> usize {
        self.auth_hashes.len() * HASH_BYTES
    }

    pub fn field_bytes(&self) -> usize {
        self.values.len() * FIELD_BYTES
    }
}

/// Output-committed SEP instance produced by Canopy's verifier reduction.
#[derive(Clone, Debug)]
pub struct CommittedSepInstance {
    pub codeword_commitment: Digest,
    /// `(canopy_index, H2F(subtree_root))`, sorted and deduplicated.
    pub ancestor_claims: Vec<(usize, Fr)>,
    pub gamma: Fr,
    pub sigma: Fr,
    pub message_variables: usize,
    pub log_inverse_rate: usize,
}

#[derive(Clone, Debug)]
pub struct CanopyProofEstimate {
    pub bottom_hashes: usize,
    pub opened_values: usize,
    pub whir: WhirProofSize,
}

impl CanopyProofEstimate {
    pub fn hash_count(&self) -> usize {
        self.bottom_hashes + self.whir.hash_count
    }

    pub fn field_count(&self) -> usize {
        self.opened_values + self.whir.field_count
    }

    pub fn hash_bytes(&self) -> usize {
        self.hash_count() * HASH_BYTES
    }

    pub fn field_bytes(&self) -> usize {
        self.opened_values * FIELD_BYTES + self.whir.field_bytes()
    }

    pub fn opened_value_bytes(&self) -> usize {
        self.opened_values * FIELD_BYTES
    }

    pub fn total_bytes(&self) -> usize {
        self.hash_bytes() + self.field_bytes()
    }
}

pub struct CanopyVectorCommitment;

impl CanopyVectorCommitment {
    /// Commit exactly as in Canopy: build the bottom `h` Merkle layers, map
    /// their roots to the field, RS-encode the resulting message, then commit
    /// to the codeword using WHIR-compatible folding-column Merkle leaves.
    pub fn commit(values: &[Fr], height: usize) -> Result<(CanopyCommitment, CanopyProverData)> {
        Self::commit_with_codeword_log_reduction(values, height, 0)
    }

    /// Commit with RS codeword length `values.len() / 2^reduction`.
    pub fn commit_with_codeword_log_reduction(
        values: &[Fr],
        height: usize,
        reduction: usize,
    ) -> Result<(CanopyCommitment, CanopyProverData)> {
        ensure!(
            values.len().is_power_of_two(),
            "vector length must be a power of two"
        );
        ensure!(height > 0, "canopy height must be positive");
        ensure!(
            height < values.len().ilog2() as usize,
            "canopy height is too large"
        );
        ensure!(
            reduction < height,
            "codeword reduction must leave the Canopy code rate below one"
        );

        let bottom_forest = MerkleLevels::commit_forest(values, 1, height)?;
        let canopy_message: Vec<Fr> = bottom_forest
            .roots()
            .par_iter()
            .map(hash_to_field)
            .collect();
        let rs_length = values.len() >> reduction;
        let codeword = reed_solomon::encode(&canopy_message, rs_length)?;
        let code_tree = MerkleLevels::commit(&codeword, WHIR_LEAF_WIDTH)?;
        let commitment = CanopyCommitment {
            root: code_tree.root()?,
            vector_len: values.len(),
            height,
            rs_dimension: canopy_message.len(),
            rs_length: codeword.len(),
            codeword_log_reduction: reduction,
            whir_leaf_width: WHIR_LEAF_WIDTH,
        };
        Ok((
            commitment,
            CanopyProverData {
                bottom_forest,
                canopy_message,
                codeword,
                code_tree,
            },
        ))
    }

    /// The same commitment operation with wall-clock timings at its four
    /// material stage boundaries. Deallocation occurs after this returns.
    pub fn commit_profiled(
        values: &[Fr],
        height: usize,
        reduction: usize,
    ) -> Result<(CanopyCommitment, CanopyProverData, CanopyCommitmentProfile)> {
        Self::commit_profiled_with_leaf_width(values, height, reduction, WHIR_LEAF_WIDTH)
    }

    /// Profile a Canopy commitment using the leaf width of a selected WHIR
    /// folding factor.
    pub fn commit_profiled_with_leaf_width(
        values: &[Fr],
        height: usize,
        reduction: usize,
        whir_leaf_width: usize,
    ) -> Result<(CanopyCommitment, CanopyProverData, CanopyCommitmentProfile)> {
        let total_start = Instant::now();
        ensure!(
            values.len().is_power_of_two(),
            "vector length must be a power of two"
        );
        ensure!(height > 0, "canopy height must be positive");
        ensure!(
            height < values.len().ilog2() as usize,
            "canopy height is too large"
        );
        ensure!(
            reduction < height,
            "codeword reduction must leave the Canopy code rate below one"
        );
        ensure!(
            whir_leaf_width.is_power_of_two(),
            "WHIR leaf width must be a power of two"
        );

        let start = Instant::now();
        let bottom_forest = MerkleLevels::commit_forest(values, 1, height)?;
        let bottom_forest_time = start.elapsed();

        let start = Instant::now();
        let canopy_message: Vec<Fr> = bottom_forest
            .roots()
            .par_iter()
            .map(hash_to_field)
            .collect();
        let hash_to_field_time = start.elapsed();

        let start = Instant::now();
        let rs_length = values.len() >> reduction;
        let codeword = reed_solomon::encode(&canopy_message, rs_length)?;
        let reed_solomon_encoding_time = start.elapsed();

        let start = Instant::now();
        let code_tree = MerkleLevels::commit(&codeword, whir_leaf_width)?;
        let codeword_merkle_tree_time = start.elapsed();

        let commitment = CanopyCommitment {
            root: code_tree.root()?,
            vector_len: values.len(),
            height,
            rs_dimension: canopy_message.len(),
            rs_length: codeword.len(),
            codeword_log_reduction: reduction,
            whir_leaf_width,
        };
        let profile = CanopyCommitmentProfile {
            total: total_start.elapsed(),
            bottom_forest: bottom_forest_time,
            hash_to_field: hash_to_field_time,
            reed_solomon_encoding: reed_solomon_encoding_time,
            codeword_merkle_tree: codeword_merkle_tree_time,
        };
        Ok((
            commitment,
            CanopyProverData {
                bottom_forest,
                canopy_message,
                codeword,
                code_tree,
            },
            profile,
        ))
    }

    /// Profile the paper Canopy commitment using WHIR's exact primitives:
    /// the input is WHIR's interleaved base RS matrix, its rows are hashed by
    /// `matrix_commit::hash_rows`, every Merkle parent uses WHIR's BLAKE3
    /// engine, and the Canopy-layer codeword is produced and committed by the
    /// same WHIR interleaved encoder and matrix commitment.
    pub fn commit_whir_profiled(
        base_codeword: &WhirEncodedMatrix,
        height: usize,
        reduction: usize,
        folding_log: usize,
    ) -> Result<(
        CanopyCommitment,
        WhirCanopyProverData,
        CanopyCommitmentProfile,
    )> {
        let (mut commitments, mut prover_data, profile) = Self::commit_whir_profiled_batch(
            std::slice::from_ref(base_codeword),
            height,
            reduction,
            folding_log,
        )?;
        Ok((commitments.remove(0), prover_data.remove(0), profile))
    }

    /// Commit equal-geometry Canopy instances as one staged parallel workload.
    ///
    /// The bottom forests, H2F conversions, and Merkle commitments run in
    /// parallel. The Canopy layer messages are RS encoded concurrently on
    /// WHIR's shared NTT worker pool while retaining the independently
    /// committable matrix layout required by the protocol.
    pub fn commit_whir_profiled_batch(
        base_codewords: &[WhirEncodedMatrix],
        height: usize,
        reduction: usize,
        folding_log: usize,
    ) -> Result<(
        Vec<CanopyCommitment>,
        Vec<WhirCanopyProverData>,
        CanopyCommitmentProfile,
    )> {
        let total_start = Instant::now();
        ensure!(
            !base_codewords.is_empty(),
            "at least one base codeword is required"
        );
        let base_len = base_codewords[0].len();
        let base_row_width = base_codewords[0].row_width;
        ensure!(
            base_len.is_power_of_two(),
            "base codeword length must be a power of two"
        );
        ensure!(
            base_row_width.is_power_of_two(),
            "base WHIR row width must be a power of two"
        );
        ensure!(
            base_codewords.iter().all(|base| {
                base.len() == base_len
                    && base.row_width == base_row_width
                    && base.row_count == base_codewords[0].row_count
            }),
            "batched Canopy base codewords must have equal geometry"
        );
        let row_log = base_row_width.ilog2() as usize;
        ensure!(
            height >= row_log,
            "Canopy height must contain at least one full WHIR row"
        );
        ensure!(
            height < base_len.ilog2() as usize,
            "Canopy height is too large"
        );
        ensure!(
            reduction < height,
            "codeword reduction must leave the Canopy code rate below one"
        );

        let start = Instant::now();
        let forest_levels = height - row_log;
        let bottom_levels = base_codewords
            .par_iter()
            .map(|base_codeword| {
                let engine = ENGINES
                    .retrieve(whir_ref::hash::BLAKE3)
                    .ok_or_else(|| anyhow::anyhow!("WHIR BLAKE3 engine is unavailable"))?;
                let mut leaves = vec![WhirHash::default(); base_codeword.row_count];
                matrix_commit::hash_rows(&*engine, base_codeword.values(), &mut leaves);
                let mut levels = Vec::with_capacity(forest_levels + 1);
                levels.push(leaves);
                for _ in 0..forest_levels {
                    let parents = whir_parent_level(levels.last().unwrap())?;
                    levels.push(parents);
                }
                Ok(levels)
            })
            .collect::<Result<Vec<_>>>()?;
        let bottom_forest_time = start.elapsed();

        let start = Instant::now();
        let canopy_messages = bottom_levels
            .par_iter()
            .map(|levels| {
                levels
                    .last()
                    .unwrap()
                    .par_iter()
                    .map(|root| hash_bytes_to_field(&root.0))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let hash_to_field_time = start.elapsed();

        let start = Instant::now();
        let layer_log_inverse_rate = height - reduction;
        let canopy_message_refs = canopy_messages
            .iter()
            .map(Vec::as_slice)
            .collect::<Vec<_>>();
        let codewords = encode_many_interleaved_with_whir(
            &canopy_message_refs,
            layer_log_inverse_rate,
            folding_log,
        )?;
        let reed_solomon_encoding_time = start.elapsed();

        let merkle_row_width = 1usize << folding_log;
        let start = Instant::now();
        let committed_codewords = codewords
            .into_par_iter()
            .map(|codeword| {
                let (codeword_commitment, _) =
                    commit_matrix_with_whir(&codeword, merkle_row_width)?;
                Ok((codeword, codeword_commitment))
            })
            .collect::<Result<Vec<_>>>()?;
        let codeword_merkle_tree_time = start.elapsed();

        let mut commitments = Vec::with_capacity(base_codewords.len());
        let mut prover_data = Vec::with_capacity(base_codewords.len());
        for ((bottom_levels, canopy_message), (codeword, codeword_commitment)) in bottom_levels
            .into_iter()
            .zip(canopy_messages)
            .zip(committed_codewords)
        {
            commitments.push(CanopyCommitment {
                root: codeword_commitment.root.0,
                vector_len: base_len,
                height,
                rs_dimension: canopy_message.len(),
                rs_length: codeword.len(),
                codeword_log_reduction: reduction,
                whir_leaf_width: merkle_row_width,
            });
            prover_data.push(WhirCanopyProverData {
                bottom_levels,
                canopy_message,
                codeword,
                codeword_commitment,
            });
        }
        let profile = CanopyCommitmentProfile {
            total: total_start.elapsed(),
            bottom_forest: bottom_forest_time,
            hash_to_field: hash_to_field_time,
            reed_solomon_encoding: reed_solomon_encoding_time,
            codeword_merkle_tree: codeword_merkle_tree_time,
        };
        Ok((commitments, prover_data, profile))
    }

    /// Open queried rows through the retained bottom Canopy forest. Query
    /// indices are WHIR matrix-row indices and each answer contains one full
    /// interleaving row, exactly as in a WHIR matrix opening.
    pub fn open_whir_bottom(
        data: &WhirCanopyProverData,
        base_codeword: &WhirEncodedMatrix,
        row_indices: &[usize],
    ) -> Result<CanopyBottomOpening> {
        ensure!(
            data.bottom_levels[0].len() == base_codeword.row_count,
            "base codeword does not match the Canopy witness"
        );
        ensure!(
            row_indices
                .iter()
                .all(|&index| index < base_codeword.row_count),
            "Canopy query row is out of range"
        );
        let values = base_codeword
            .matrix
            .read_rows(base_codeword.row_width, row_indices);
        let mut current = row_indices
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        let mut auth_hashes = Vec::new();
        for level in data
            .bottom_levels
            .iter()
            .take(data.bottom_levels.len().saturating_sub(1))
        {
            for &index in &current {
                let sibling = index ^ 1;
                if !current.contains(&sibling) {
                    auth_hashes.push(level[sibling].0);
                }
            }
            current = current.into_iter().map(|index| index / 2).collect();
        }
        Ok(CanopyBottomOpening {
            values,
            auth_hashes,
        })
    }

    /// First prover message in the interactive opening: opened values and
    /// authentication hashes through the bottom `h` layers.
    pub fn open_bottom(
        data: &CanopyProverData,
        values: &[Fr],
        indices: &[usize],
    ) -> Result<CanopyBottomOpening> {
        ensure!(
            values.len() == data.bottom_forest.leaf_count(),
            "witness length mismatch"
        );
        let auth_hashes = data.bottom_forest.open_hashes(indices)?;
        let opened_values = indices.iter().map(|&index| values[index]).collect();
        Ok(CanopyBottomOpening {
            values: opened_values,
            auth_hashes,
        })
    }

    /// Verifier checks the bottom paths, samples/supplies `gamma`, batches the
    /// distinct ancestor claims, and outputs the committed SEP instance. A
    /// complete verifier would next invoke WHIR on this instance.
    pub fn verify_bottom_and_reduce(
        commitment: &CanopyCommitment,
        indices: &[usize],
        opening: &CanopyBottomOpening,
        gamma: Fr,
    ) -> Result<CommittedSepInstance> {
        ensure!(
            commitment.vector_len.is_power_of_two(),
            "invalid committed length"
        );
        let leaves: Vec<Vec<Fr>> = opening.values.iter().map(|value| vec![*value]).collect();
        let roots = reconstruct_roots(
            commitment.vector_len,
            1,
            commitment.height,
            indices,
            &leaves,
            &opening.auth_hashes,
        )?;
        let ancestor_claims: Vec<_> = roots
            .into_iter()
            .map(|(index, root)| (index, hash_to_field(&root)))
            .collect();
        ensure!(
            !ancestor_claims.is_empty(),
            "no ancestor claims were reconstructed"
        );

        let mut power = Fr::from(1u64);
        let mut sigma = Fr::from(0u64);
        for (_, alpha) in &ancestor_claims {
            sigma += power * alpha;
            power *= gamma;
        }
        let message_variables = commitment.rs_dimension.ilog2() as usize;
        let codeword_variables = commitment.rs_length.ilog2() as usize;
        Ok(CommittedSepInstance {
            codeword_commitment: commitment.root,
            ancestor_claims,
            gamma,
            sigma,
            message_variables,
            log_inverse_rate: codeword_variables - message_variables,
        })
    }

    /// Produce the WHIR portion of the size estimate after the verifier has
    /// reduced the batch opening to one output-committed SEP claim.
    pub fn estimate_whir(instance: &CommittedSepInstance, seed: [u8; 32]) -> Result<WhirProofSize> {
        let params =
            WhirParameters::johnson_bound(instance.message_variables, instance.log_inverse_rate)?;
        estimate_proof(&params, seed)
    }

    /// Estimate with an independently selected Canopy/WHIR layer field.
    pub fn estimate_whir_with_field(
        instance: &CommittedSepInstance,
        seed: [u8; 32],
        field: WhirFieldProfile,
    ) -> Result<WhirProofSize> {
        let params = WhirParameters::johnson_bound_with_field(
            instance.message_variables,
            instance.log_inverse_rate,
            field,
        )?;
        estimate_proof(&params, seed)
    }
}

/// Reconstruct the expected `y` claims directly from prover data. This is a
/// test/debug helper; the verifier never receives the full message.
pub fn claimed_message_values(
    data: &CanopyProverData,
    instance: &CommittedSepInstance,
) -> BTreeMap<usize, Fr> {
    instance
        .ancestor_claims
        .iter()
        .map(|(index, _)| (*index, data.canopy_message[*index]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executable_whir::encode_interleaved_with_whir;

    #[test]
    fn canopy_reduces_batch_to_sep() {
        let values: Vec<_> = (0..256).map(Fr::from).collect();
        let indices = [1, 2, 17, 18, 200];
        let (commitment, data) = CanopyVectorCommitment::commit(&values, 4).unwrap();
        let opening = CanopyVectorCommitment::open_bottom(&data, &values, &indices).unwrap();
        let instance = CanopyVectorCommitment::verify_bottom_and_reduce(
            &commitment,
            &indices,
            &opening,
            Fr::from(9u64),
        )
        .unwrap();
        assert!(instance.ancestor_claims.len() < indices.len());
        for (index, claim) in &instance.ancestor_claims {
            assert_eq!(*claim, data.canopy_message[*index]);
        }
    }

    #[test]
    fn bottom_tampering_fails() {
        let values: Vec<_> = (0..256).map(Fr::from).collect();
        let indices = [3, 99];
        let (commitment, data) = CanopyVectorCommitment::commit(&values, 4).unwrap();
        let mut opening = CanopyVectorCommitment::open_bottom(&data, &values, &indices).unwrap();
        opening.auth_hashes[0][0] ^= 1;
        // The bottom stage has no public forest roots; tampering changes the
        // downstream y claim and is rejected by the committed WHIR relation.
        let instance = CanopyVectorCommitment::verify_bottom_and_reduce(
            &commitment,
            &indices,
            &opening,
            Fr::from(9u64),
        )
        .unwrap();
        assert!(instance
            .ancestor_claims
            .iter()
            .any(|(i, claim)| *claim != data.canopy_message[*i]));
    }

    #[test]
    fn shortened_codewords_preserve_the_sep_rate() {
        let values: Vec<_> = (0..256).map(Fr::from).collect();
        for (reduction, expected_length, expected_rate_log) in [(1, 128, 3), (2, 64, 2)] {
            let (commitment, data) =
                CanopyVectorCommitment::commit_with_codeword_log_reduction(&values, 4, reduction)
                    .unwrap();
            assert_eq!(commitment.rs_length, expected_length);
            assert_eq!(commitment.codeword_log_reduction, reduction);
            let opening = CanopyVectorCommitment::open_bottom(&data, &values, &[1, 17]).unwrap();
            let instance = CanopyVectorCommitment::verify_bottom_and_reduce(
                &commitment,
                &[1, 17],
                &opening,
                Fr::from(3u64),
            )
            .unwrap();
            assert_eq!(instance.log_inverse_rate, expected_rate_log);
        }
    }

    #[test]
    fn profiled_commitment_matches_regular_commitment() {
        let values: Vec<_> = (0..256).map(Fr::from).collect();
        let (regular, _) =
            CanopyVectorCommitment::commit_with_codeword_log_reduction(&values, 4, 2).unwrap();
        let (profiled, _, profile) =
            CanopyVectorCommitment::commit_profiled(&values, 4, 2).unwrap();
        assert_eq!(regular, profiled);
        assert!(profile.total >= profile.accounted());
    }

    #[test]
    fn whir_native_canopy_uses_whir_rows_end_to_end() {
        let message = (0..256u64).map(Fr::from).collect::<Vec<_>>();
        let base = encode_interleaved_with_whir(&message, 1, 3).unwrap();
        let (commitment, data, profile) =
            CanopyVectorCommitment::commit_whir_profiled(&base, 4, 0, 3).unwrap();

        assert_eq!(base.row_width, 8);
        assert_eq!(base.row_count, 64);
        assert_eq!(commitment.vector_len, 512);
        assert_eq!(commitment.rs_dimension, 32);
        assert_eq!(commitment.rs_length, 512);
        assert_eq!(data.canopy_message.len(), 32);
        assert_eq!(data.codeword.row_width, 8);
        assert_eq!(data.codeword.row_count, 64);
        assert_eq!(commitment.root, data.codeword_commitment.root.0);
        assert!(profile.total >= profile.accounted());

        // Rows 0 and 1 are siblings at h=4, so opening both needs no bottom
        // authentication hash and returns both complete WHIR rows.
        let opening = CanopyVectorCommitment::open_whir_bottom(&data, &base, &[0, 1]).unwrap();
        assert_eq!(opening.values.len(), 16);
        assert!(opening.auth_hashes.is_empty());

        let (_, terminal, _) =
            CanopyVectorCommitment::commit_whir_profiled(&base, 8, 0, 3).unwrap();
        assert_eq!(terminal.canopy_message.len(), 2);
        assert_eq!(terminal.codeword.row_width, 1);
        assert_eq!(terminal.codeword.len(), 512);
    }

    #[test]
    fn batched_whir_canopy_matches_individual_commitments() {
        let first = (0..256u64).map(Fr::from).collect::<Vec<_>>();
        let second = (0..256u64)
            .map(|value| Fr::from(5 * value + 2))
            .collect::<Vec<_>>();
        let bases = encode_many_interleaved_with_whir(&[first.as_slice(), second.as_slice()], 1, 3)
            .unwrap();
        let expected = bases
            .iter()
            .map(|base| {
                CanopyVectorCommitment::commit_whir_profiled(base, 4, 0, 3)
                    .unwrap()
                    .0
            })
            .collect::<Vec<_>>();
        let (actual, _, profile) =
            CanopyVectorCommitment::commit_whir_profiled_batch(&bases, 4, 0, 3).unwrap();
        assert_eq!(actual, expected);
        assert!(profile.total >= profile.accounted());
    }

    #[test]
    fn partial_forest_parent_hashes_match_whir_merkle() {
        let leaves = (0u8..8)
            .map(|byte| WhirHash([byte; HASH_BYTES]))
            .collect::<Vec<_>>();
        let mut level = leaves.clone();
        while level.len() > 1 {
            level = whir_parent_level(&level).unwrap();
        }
        let config = whir_ref::protocols::merkle_tree::Config::with_hash(
            whir_ref::hash::BLAKE3,
            leaves.len(),
        );
        let nodes = config.build_nodes(leaves);
        assert_eq!(level[0], *nodes.last().unwrap());
    }
}
