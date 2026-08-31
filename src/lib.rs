//! Canopy, Merkle, and executable WHIR benchmarks over a 256-bit field.

pub mod benchmark;
pub mod canopy;
pub mod executable_whir;
pub mod merkle;
pub mod paper_experiments;
pub mod reed_solomon;
pub mod separate_benchmark;
pub mod whir;

use ark_ff::{Fp256, Fp64, MontBackend, MontConfig};

/// The 256-bit FFT-friendly prime field used by all paper experiments.
///
/// The modulus is `2^256 - 43 * 2^32 + 1`. Its two-adicity is 32, which
/// supports the largest rate-1/2 WHIR domain in the proof-size grid (`2^31`).
#[derive(MontConfig)]
#[modulus = "115792089237316195423570985008687907853269984665640564039457584007728446046209"]
#[generator = "7"]
pub struct FrConfig;

pub type Fr = Fp256<MontBackend<FrConfig, 4>>;

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
        assert_eq!(Fr::MODULUS_BIT_SIZE, 256);
        assert_eq!(Fr::TWO_ADICITY, 32);
        assert_eq!(CanopyLayerFr48::MODULUS_BIT_SIZE, 48);
        assert_eq!(CanopyLayerFr48::TWO_ADICITY, 30);
        assert_eq!(CanopyLayerFr64::MODULUS_BIT_SIZE, 64);
        assert_eq!(CanopyLayerFr64::TWO_ADICITY, 32);
    }
}
