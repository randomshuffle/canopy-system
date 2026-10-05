use canopy_system::Fr;
use whir::{algebra::embedding::Identity, parameters::ProtocolParameters, protocols::{params::DecodingRegime, whir::Config}, hash};
fn main() {
    canopy_system::register_benchmark_ntt();
    for log_k in 3..=30 {
        for log_inverse_rate in 1..=(32-log_k) {
            for security_level in [128, 136] {
                let parameters = ProtocolParameters {
                    decoding_regime: DecodingRegime::Johnson,
                    starting_log_inv_rate: log_inverse_rate,
                    initial_folding_factor: 3,
                    folding_factor: 3,
                    security_level,
                    pow_bits: 0,
                    batch_size: 1,
                    hash_id: hash::BLAKE3,
                };
                let config = Config::<Identity<Fr>>::new(1 << log_k, &parameters);
                println!("{log_k},{log_inverse_rate},{security_level}: {config:?}");
            }
        }
    }
}
