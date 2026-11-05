//! Deterministic query sampling shared by the Bolt proof-size estimators.
use std::collections::BTreeSet;

use anyhow::{ensure, Result};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

pub fn sample_unique_indices(universe: usize, count: usize, seed: u64) -> Result<Vec<usize>> {
    ensure!(
        count <= universe,
        "cannot sample more indices than the universe"
    );
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    let mut set = BTreeSet::new();
    while set.len() < count {
        set.insert(rng.gen_range(0..universe));
    }
    Ok(set.into_iter().collect())
}
