use anyhow::{ensure, Result};

use crate::Fr;

/// Encode coefficient vector `message` as a Reed-Solomon codeword over a
/// Cantor additive domain. The code is `[codeword_len, message.len()]`.
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
    ensure!(codeword_len <= 1usize << 32, "additive domain exceeds 2^32");
    Ok(crate::additive_rs::encode(message, codeword_len))
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
