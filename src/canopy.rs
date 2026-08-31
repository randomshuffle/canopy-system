use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use anyhow::{ensure, Result};
use ark_ff::PrimeField;
use rayon::prelude::*;

use crate::merkle::{reconstruct_roots, Digest, MerkleLevels};
use crate::reed_solomon;
use crate::whir::{
    estimate_proof, WhirFieldProfile, WhirParameters, WhirProofSize, DEFAULT_FOLDING_FACTOR,
};
use crate::{Fr, FIELD_BYTES, HASH_BYTES};

/// WHIR groups one folding column into each Merkle leaf.
pub const WHIR_LEAF_WIDTH: usize = DEFAULT_FOLDING_FACTOR;

fn hash_to_field(root: &Digest) -> Fr {
    // Reducing 512 XOF bits modulo the 256-bit input field has negligible bias.
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"canopy-system/hash-to-field/v1");
    hasher.update(root);
    let mut reader = hasher.finalize_xof();
    let mut uniform = [0u8; 64];
    reader.fill(&mut uniform);
    Fr::from_le_bytes_mod_order(&uniform)
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
        let code_tree = MerkleLevels::commit(&codeword, WHIR_LEAF_WIDTH)?;
        let codeword_merkle_tree_time = start.elapsed();

        let commitment = CanopyCommitment {
            root: code_tree.root()?,
            vector_len: values.len(),
            height,
            rs_dimension: canopy_message.len(),
            rs_length: codeword.len(),
            codeword_log_reduction: reduction,
            whir_leaf_width: WHIR_LEAF_WIDTH,
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
}
