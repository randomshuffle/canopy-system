use std::collections::{BTreeMap, BTreeSet};

use anyhow::{bail, ensure, Result};
use ark_ff::BigInteger;
use rayon::prelude::*;

use crate::{Fr, FIELD_BYTES, HASH_BYTES};

pub type Digest = [u8; HASH_BYTES];

const LEAF_DOMAIN: &[u8] = b"canopy-system/merkle-leaf/v1";
const NODE_DOMAIN: &[u8] = b"canopy-system/merkle-node/v1";

/// Hash one Merkle leaf containing one or more field elements.
pub fn hash_field_leaf(values: &[Fr]) -> Digest {
    let mut hasher = blake3::Hasher::new();
    hasher.update(LEAF_DOMAIN);
    hasher.update(&(values.len() as u64).to_le_bytes());
    for value in values {
        let mut bytes = value.into_bigint().to_bytes_le();
        bytes.resize(FIELD_BYTES, 0);
        hasher.update(&bytes);
    }
    *hasher.finalize().as_bytes()
}

pub fn hash_children(left: &Digest, right: &Digest) -> Digest {
    let mut hasher = blake3::Hasher::new();
    hasher.update(NODE_DOMAIN);
    hasher.update(left);
    hasher.update(right);
    *hasher.finalize().as_bytes()
}

fn validate_indices(indices: &[usize], universe: usize) -> Result<BTreeSet<usize>> {
    ensure!(
        !indices.is_empty(),
        "an opening must contain at least one index"
    );
    let set: BTreeSet<_> = indices.iter().copied().collect();
    ensure!(
        set.len() == indices.len(),
        "opening indices must be distinct"
    );
    ensure!(
        set.iter().all(|&i| i < universe),
        "opening index out of range"
    );
    Ok(set)
}

/// Number of hashes in a minimal binary Merkle multiproof.
///
/// `height` is the number of authenticated levels. Passing the full tree
/// height produces a proof to the root; passing `h` produces a proof to the
/// roots of a depth-`h` forest, as needed by Canopy.
pub fn frontier_hash_count(indices: &[usize], universe: usize, height: usize) -> Result<usize> {
    ensure!(
        universe.is_power_of_two(),
        "universe must be a power of two"
    );
    ensure!(
        height <= universe.ilog2() as usize,
        "height exceeds tree height"
    );
    let mut current = validate_indices(indices, universe)?;
    let mut count = 0;
    for _ in 0..height {
        for &index in &current {
            if !current.contains(&(index ^ 1)) {
                count += 1;
            }
        }
        current = current.into_iter().map(|index| index >> 1).collect();
    }
    Ok(count)
}

/// Per-level geometry of a merged binary authentication proof, without a tree.
/// `parents` counts distinct parent hashes (the native WHIR verifier); the
/// forest verifier currently hashes once per current node, including siblings.
#[derive(Clone, Copy, Debug, Default)]
pub struct AuthenticationWork {
    pub leaves: usize,
    pub auth: usize,
    pub parents: usize,
    pub forest_parents: usize,
    pub roots: usize,
}

pub fn authentication_work_by_height(
    indices: &[usize],
    universe: usize,
) -> Result<Vec<AuthenticationWork>> {
    ensure!(
        universe.is_power_of_two(),
        "universe must be a power of two"
    );
    ensure!(indices.iter().all(|&i| i < universe), "index out of range");
    let mut current = indices.to_vec();
    current.sort_unstable();
    current.dedup();
    let mut work = AuthenticationWork {
        leaves: current.len(),
        roots: current.len(),
        ..Default::default()
    };
    let mut result = vec![work];
    for _ in 0..universe.ilog2() {
        let previous = current.len();
        for i in &mut current {
            *i >>= 1;
        }
        current.dedup();
        work.auth += 2 * current.len() - previous;
        work.parents += current.len();
        work.forest_parents += previous;
        work.roots = current.len();
        result.push(work);
    }
    Ok(result)
}

/// Stored levels of either a complete Merkle tree or a truncated forest.
#[derive(Clone, Debug)]
pub struct MerkleLevels {
    levels: Vec<Vec<Digest>>,
    leaf_width: usize,
}

impl MerkleLevels {
    pub fn commit(values: &[Fr], leaf_width: usize) -> Result<Self> {
        ensure!(leaf_width > 0, "leaf width must be positive");
        ensure!(!values.is_empty(), "cannot commit to an empty vector");
        ensure!(
            values.len().is_multiple_of(leaf_width),
            "leaf width must divide vector length"
        );
        let leaves = values.len() / leaf_width;
        ensure!(
            leaves.is_power_of_two(),
            "number of leaves must be a power of two"
        );
        Self::commit_forest(values, leaf_width, leaves.ilog2() as usize)
    }

    pub fn commit_forest(values: &[Fr], leaf_width: usize, height: usize) -> Result<Self> {
        ensure!(leaf_width > 0, "leaf width must be positive");
        ensure!(!values.is_empty(), "cannot commit to an empty vector");
        ensure!(
            values.len().is_multiple_of(leaf_width),
            "leaf width must divide vector length"
        );
        let leaf_count = values.len() / leaf_width;
        ensure!(
            leaf_count.is_power_of_two(),
            "number of leaves must be a power of two"
        );
        ensure!(
            height <= leaf_count.ilog2() as usize,
            "height exceeds tree height"
        );

        let first: Vec<Digest> = values
            .par_chunks_exact(leaf_width)
            .map(hash_field_leaf)
            .collect();
        let mut levels = Vec::with_capacity(height + 1);
        levels.push(first);
        for _ in 0..height {
            let parent: Vec<Digest> = levels
                .last()
                .expect("at least the leaf level")
                .par_chunks_exact(2)
                .map(|children| hash_children(&children[0], &children[1]))
                .collect();
            levels.push(parent);
        }
        Ok(Self { levels, leaf_width })
    }

    pub fn leaf_count(&self) -> usize {
        self.levels[0].len()
    }

    pub fn leaf_width(&self) -> usize {
        self.leaf_width
    }

    pub fn height(&self) -> usize {
        self.levels.len() - 1
    }

    pub fn roots(&self) -> &[Digest] {
        self.levels.last().expect("levels are nonempty")
    }

    pub fn root(&self) -> Result<Digest> {
        ensure!(
            self.roots().len() == 1,
            "this is a forest, not a complete tree"
        );
        Ok(self.roots()[0])
    }

    /// The proof contains only hashes. Positions are reconstructed from the
    /// verifier-generated indices, so no index metadata is serialized.
    pub fn open_hashes(&self, indices: &[usize]) -> Result<Vec<Digest>> {
        let mut current = validate_indices(indices, self.leaf_count())?;
        let mut proof = Vec::new();
        for level in 0..self.height() {
            for &index in &current {
                let sibling = index ^ 1;
                if !current.contains(&sibling) {
                    proof.push(self.levels[level][sibling]);
                }
            }
            current = current.into_iter().map(|index| index >> 1).collect();
        }
        Ok(proof)
    }
}

/// Verify a proof to either one root or a set of forest roots.
pub fn reconstruct_roots(
    leaf_count: usize,
    leaf_width: usize,
    height: usize,
    indices: &[usize],
    opened_leaves: &[Vec<Fr>],
    auth_hashes: &[Digest],
) -> Result<BTreeMap<usize, Digest>> {
    let sorted = validate_indices(indices, leaf_count)?;
    ensure!(
        opened_leaves.len() == indices.len(),
        "wrong number of opened leaves"
    );
    ensure!(
        opened_leaves.iter().all(|v| v.len() == leaf_width),
        "wrong leaf width"
    );

    let by_index: BTreeMap<usize, &Vec<Fr>> =
        indices.iter().copied().zip(opened_leaves.iter()).collect();
    ensure!(by_index.len() == sorted.len(), "duplicate leaf index");

    let mut current: BTreeMap<usize, Digest> = sorted
        .iter()
        .map(|&index| (index, hash_field_leaf(by_index[&index])))
        .collect();
    let mut proof_offset = 0;

    for _ in 0..height {
        let mut parents = BTreeMap::new();
        let current_indices: BTreeSet<_> = current.keys().copied().collect();
        for &index in &current_indices {
            let sibling_index = index ^ 1;
            let sibling = if let Some(hash) = current.get(&sibling_index) {
                *hash
            } else {
                let Some(hash) = auth_hashes.get(proof_offset) else {
                    bail!("authentication proof is too short");
                };
                proof_offset += 1;
                *hash
            };
            let own = current[&index];
            let parent = if index & 1 == 0 {
                hash_children(&own, &sibling)
            } else {
                hash_children(&sibling, &own)
            };
            if let Some(existing) = parents.insert(index >> 1, parent) {
                ensure!(existing == parent, "inconsistent child paths");
            }
        }
        current = parents;
    }
    ensure!(
        proof_offset == auth_hashes.len(),
        "authentication proof has trailing hashes"
    );
    Ok(current)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MerkleCommitment {
    pub root: Digest,
    pub vector_len: usize,
}

#[derive(Clone, Debug)]
pub struct MerkleProverData {
    tree: MerkleLevels,
}

#[derive(Clone, Debug)]
pub struct MerkleOpening {
    pub values: Vec<Fr>,
    pub auth_hashes: Vec<Digest>,
}

impl MerkleOpening {
    pub fn hash_bytes(&self) -> usize {
        self.auth_hashes.len() * HASH_BYTES
    }

    pub fn field_bytes(&self) -> usize {
        self.values.len() * FIELD_BYTES
    }

    pub fn proof_bytes(&self) -> usize {
        self.hash_bytes() + self.field_bytes()
    }
}

/// A 256-bit BLAKE3 Merkle vector commitment with minimal batch proofs.
pub struct MerkleVectorCommitment;

impl MerkleVectorCommitment {
    pub fn commit(values: &[Fr]) -> Result<(MerkleCommitment, MerkleProverData)> {
        let tree = MerkleLevels::commit(values, 1)?;
        let commitment = MerkleCommitment {
            root: tree.root()?,
            vector_len: values.len(),
        };
        Ok((commitment, MerkleProverData { tree }))
    }

    pub fn open(
        data: &MerkleProverData,
        values: &[Fr],
        indices: &[usize],
    ) -> Result<MerkleOpening> {
        ensure!(
            values.len() == data.tree.leaf_count(),
            "witness length mismatch"
        );
        validate_indices(indices, values.len())?;
        let opened_values = indices.iter().map(|&index| values[index]).collect();
        let auth_hashes = data.tree.open_hashes(indices)?;
        Ok(MerkleOpening {
            values: opened_values,
            auth_hashes,
        })
    }

    pub fn verify(
        commitment: &MerkleCommitment,
        indices: &[usize],
        opening: &MerkleOpening,
    ) -> Result<bool> {
        ensure!(
            commitment.vector_len.is_power_of_two(),
            "invalid committed length"
        );
        let leaves: Vec<Vec<Fr>> = opening.values.iter().map(|value| vec![*value]).collect();
        let roots = reconstruct_roots(
            commitment.vector_len,
            1,
            commitment.vector_len.ilog2() as usize,
            indices,
            &leaves,
            &opening.auth_hashes,
        )?;
        Ok(roots.len() == 1 && roots.get(&0) == Some(&commitment.root))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authentication_work_matches_every_small_multiproof() {
        for mask in 1usize..256 {
            let indices: Vec<_> = (0..8).filter(|i| mask & (1 << i) != 0).collect();
            let work = authentication_work_by_height(&indices, 8).unwrap();
            for (h, w) in work.iter().enumerate() {
                assert_eq!(w.auth, frontier_hash_count(&indices, 8, h).unwrap());
                assert_eq!(w.parents + w.roots, w.leaves + w.auth);
                assert_eq!(w.forest_parents + w.roots, w.parents + w.leaves);
            }
        }
        let work = authentication_work_by_height(&[0, 1, 4], 8).unwrap();
        assert_eq!(
            (work[3].leaves, work[3].parents, work[3].forest_parents),
            (3, 5, 7)
        );
        assert_eq!(work[1].roots, 2);
    }

    #[test]
    fn minimal_multiproof_round_trip() {
        let values: Vec<_> = (0..32).map(Fr::from).collect();
        let indices = [1, 2, 3, 17, 31];
        let (commitment, data) = MerkleVectorCommitment::commit(&values).unwrap();
        let opening = MerkleVectorCommitment::open(&data, &values, &indices).unwrap();
        assert!(MerkleVectorCommitment::verify(&commitment, &indices, &opening).unwrap());
        assert_eq!(
            opening.auth_hashes.len(),
            frontier_hash_count(&indices, values.len(), 5).unwrap()
        );
    }

    #[test]
    fn tampering_is_rejected() {
        let values: Vec<_> = (0..16).map(Fr::from).collect();
        let indices = [0, 7];
        let (commitment, data) = MerkleVectorCommitment::commit(&values).unwrap();
        let mut opening = MerkleVectorCommitment::open(&data, &values, &indices).unwrap();
        opening.values[0] += Fr::from(1u64);
        assert!(!MerkleVectorCommitment::verify(&commitment, &indices, &opening).unwrap());
    }

    #[test]
    fn complete_subtree_needs_no_internal_hashes() {
        let indices: Vec<_> = (0..8).collect();
        assert_eq!(frontier_hash_count(&indices, 8, 3).unwrap(), 0);
    }
}
