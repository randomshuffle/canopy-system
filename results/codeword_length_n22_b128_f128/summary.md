# Canopy vs. Merkle batch-opening proof sizes

Mean of 100 deterministic random index/query samples; seed `0x43414e4f5059`. Sizes include opened values, exclude verifier-generated indices and randomness.

WHIR security target: 100 bits. Recursive folding terminates once message length is at most `2^8`; all folds use `2^4`.

Input-vector elements use the fixed 128-bit field and occupy 16 bytes. The Canopy/WHIR layer field is searched independently. Merkle hashes occupy 32 bytes.

No proof of work is used. Oracle queries and OOD samples target 100 bits; every fold uses one 128-bit-field sumcheck transcript. The RBR/list-adjusted single-challenge estimate can be below the target and is reported explicitly in `whir_parameters.csv`.

## Field feasibility

| log2 N | requested bits | feasible | log2 field size | bytes | two-adicity | reason |
|---:|---:|:---:|---:|---:|---:|---|
| 22 | 128 | yes | 127.584963 | 16 | 30 | supports the largest requested 2^22 multiplicative radix-2 domain |

## Best height for each feasible layer field and codeword length

| log2 N | batch | layer bits | codeword | best h | Merkle KiB | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |
|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|
| 22 | 128 | 128 | N/1 | 7 | 58.48 | 50.43 | 38.91 | 11.52 | 0.862 |
| 22 | 128 | 128 | N/2 | 7 | 58.48 | 51.94 | 39.42 | 12.52 | 0.888 |
| 22 | 128 | 128 | N/4 | 8 | 58.48 | 53.90 | 42.44 | 11.46 | 0.922 |

## Overall optimum

| log2 N | batch | h candidates | Merkle KiB | layer bits | codeword | best h | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |
|---:|---:|---|---:|---:|---|---:|---:|---:|---:|---:|
| 22 | 128 | 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15 | 58.48 | 128 | N/1 | 7 | 50.43 | 38.91 | 11.52 | 0.862 |
