use anyhow::{ensure, Result};
use ark_poly::{EvaluationDomain, Radix2EvaluationDomain};

use crate::Fr;

/// Encode coefficient vector `message` as a Reed-Solomon codeword over a
/// radix-2 multiplicative domain. The code is `[codeword_len, message.len()]`.
pub fn encode(message: &[Fr], codeword_len: usize) -> Result<Vec<Fr>> {
    ensure!(!message.is_empty(), "RS message cannot be empty");
    ensure!(
        message.len().is_power_of_two(),
        "RS message length must be a power of two"
    );
    ensure!(
        codeword_len.is_power_of_two(),
        "RS codeword length must be a power of two"
    );
    ensure!(
        message.len() <= codeword_len,
        "RS message exceeds codeword length"
    );
    let domain = Radix2EvaluationDomain::<Fr>::new(codeword_len).ok_or_else(|| {
        anyhow::anyhow!("the configured field has no radix-2 domain of size {codeword_len}")
    })?;
    let mut coefficients = vec![Fr::from(0u64); codeword_len];
    coefficients[..message.len()].copy_from_slice(message);
    domain.fft_in_place(&mut coefficients);
    Ok(coefficients)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rs_encoding_has_requested_length() {
        let message: Vec<_> = (0..8).map(Fr::from).collect();
        let codeword = encode(&message, 32).unwrap();
        assert_eq!(codeword.len(), 32);
        assert!(codeword.iter().any(|x| *x != Fr::from(0u64)));
    }
}
