//! Canopy, Merkle, and executable WHIR benchmarks over a 256-bit field.

pub mod additive_rs;
pub mod benchmark;
pub mod binary_field;
pub mod bolt;
pub mod bolt_experiment;
pub mod canopy;
pub mod executable_whir;
pub mod fiat_shamir;
pub mod field_benchmark;
pub mod merkle;
pub mod paper_experiments;
pub mod reed_solomon;
pub mod separate_benchmark;
pub mod whir;
#[cfg(feature = "whir-tracing")]
pub mod whir_trace;

use ark_ff::{Fp256, Fp64, MontBackend, MontConfig};

/// Historical prime field retained for the field microbenchmark.
#[derive(MontConfig)]
#[modulus = "115792089237316195423570985008687907853269984665640564039457584007728446046209"]
#[generator = "7"]
pub struct FrConfig;

pub type PrimeFr = Fp256<MontBackend<FrConfig, 4>>;
/// Default field for WHIR, Canopy, and Bolt.
pub use binary_field::Binary256 as Fr;

/// Concrete 48-bit Canopy-layer candidate used by the field-size grid.
#[derive(MontConfig)]
#[modulus = "281467460517889"]
#[generator = "11"]
pub struct CanopyLayerFr48Config;

pub type CanopyLayerFr48 = Fp64<MontBackend<CanopyLayerFr48Config, 1>>;

/// Concrete 64-bit Goldilocks Canopy-layer candidate.
#[derive(MontConfig)]
#[modulus = "18446744069414584321"]
#[generator = "7"]
pub struct CanopyLayerFr64Config;

pub type CanopyLayerFr64 = Fp64<MontBackend<CanopyLayerFr64Config, 1>>;

/// Headline bit width of the field used throughout the paper experiments.
pub const FIELD_BITS: usize = 256;
/// Canonical field elements occupy 256 bits / 32 bytes.
pub const FIELD_BYTES: usize = 32;
/// Every Merkle digest is a 256-bit BLAKE3 digest.
pub const HASH_BYTES: usize = 32;

#[cfg(test)]
mod tests {
    use ark_ff::{FftField, PrimeField};

    use super::{CanopyLayerFr48, CanopyLayerFr64, Fr};

    #[test]
    fn field_matches_benchmark_requirements() {
        assert_eq!(<Fr as ark_ff::Field>::extension_degree(), 256);
        assert_eq!(<Fr as ark_ff::Field>::characteristic(), &[2]);
        assert_eq!(CanopyLayerFr48::MODULUS_BIT_SIZE, 48);
        assert_eq!(CanopyLayerFr48::TWO_ADICITY, 30);
        assert_eq!(CanopyLayerFr64::MODULUS_BIT_SIZE, 64);
        assert_eq!(CanopyLayerFr64::TWO_ADICITY, 32);
    }
}

/// Register the shared additive RS backend; public basis setup is untimed.
pub fn register_benchmark_ntt() {
    use std::sync::{Arc, Once};
    use whir_ref::algebra::ntt::{ReedSolomon, NTT};
    static REGISTER: Once = Once::new();
    REGISTER.call_once(|| {
        additive_rs::setup();
        NTT.insert(Arc::new(additive_rs::AdditiveRs) as Arc<dyn ReedSolomon<Fr>>);
    });
}

/// Reproducible full-width benchmark data; generation is outside the timers.
pub fn benchmark_field_element(seed: u64, domain: u64, index: u64) -> Fr {
    use ark_ff::Field;
    for counter in 0u64.. {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"canopy-system/field-comparison-input/v1");
        for value in [seed, domain, index, counter] {
            hasher.update(&value.to_le_bytes());
        }
        if let Some(value) = Fr::from_random_bytes(hasher.finalize().as_bytes()) {
            return value;
        }
    }
    unreachable!()
}

/// Identity recorded in the shared field-comparison timing reports.
pub const BENCHMARK_FIELD_LABEL: &str = "GF(2^256), additive Cantor-subspace RS";

pub const FIELD_MODULUS_LABEL: &str = "GF(2^256) = GF(2^128)[w]/(w^2+w+x^121)";
pub const FIELD_DOMAIN_KIND: &str = "additive Cantor subspace";
pub const FIELD_TWO_ADICITY: usize = 0;
pub const FIELD_MAX_DOMAIN_LOG: usize = 32;

#[cfg(test)]
mod binary_protocol_tests;
