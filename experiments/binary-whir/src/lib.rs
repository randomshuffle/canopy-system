//! Isolated GF(2^256) runner. Shared WHIR/Canopy timing and commitment code;
//! separate field and additive RS backend. No Bolt module is compiled here.
#![allow(unused_imports)] // Shared source imports PrimeField for the prime-field build.
pub mod additive;
pub mod field;
pub use field::Binary256 as Fr;
pub const FIELD_BITS: usize = 256;
pub const FIELD_BYTES: usize = 32;
pub const HASH_BYTES: usize = 32;
#[path = "../../../src/canopy.rs"]
pub mod canopy;
#[path = "../../../src/executable_whir.rs"]
pub mod executable_whir;
#[path = "../../../src/merkle.rs"]
pub mod merkle;
#[path = "../../../src/paper_experiments.rs"]
pub mod paper_experiments;
#[path = "../../../src/whir.rs"]
pub mod whir;
pub mod reed_solomon {
    pub fn encode(message: &[crate::Fr], length: usize) -> anyhow::Result<Vec<crate::Fr>> {
        anyhow::ensure!(
            !message.is_empty()
                && message.len().is_power_of_two()
                && length.is_power_of_two()
                && message.len() <= length,
            "invalid additive RS geometry"
        );
        Ok(crate::additive::encode(message, length))
    }
}
pub fn register_benchmark_ntt() {
    use std::sync::{Arc, Once};
    use whir_ref::algebra::ntt::{ReedSolomon, NTT};
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        additive::setup();
        NTT.insert(Arc::new(additive::AdditiveRs) as Arc<dyn ReedSolomon<Fr>>);
    });
}
pub fn benchmark_field_element(seed: u64, domain: u64, index: u64) -> Fr {
    let mut h = blake3::Hasher::new();
    h.update(b"canopy-system/field-comparison-input/v1");
    for value in [seed, domain, index, 0] {
        h.update(&value.to_le_bytes());
    }
    Fr::from_bytes(*h.finalize().as_bytes())
}

#[cfg(test)]
mod protocol_tests;

pub const BENCHMARK_FIELD_LABEL: &str = "GF(2^256), additive Cantor-subspace RS";

pub const FIELD_MODULUS_LABEL: &str = "GF(2^256) = GF(2^128)[w]/(w^2+w+x^121)";
pub const FIELD_DOMAIN_KIND: &str = "additive Cantor subspace";
pub const FIELD_TWO_ADICITY: usize = 0;
pub const FIELD_MAX_DOMAIN_LOG: usize = 32;
