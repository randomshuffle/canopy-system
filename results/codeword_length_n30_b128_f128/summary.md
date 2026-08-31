# Canopy vs. Merkle batch-opening proof sizes

Mean of 100 deterministic random index/query samples; seed `0x43414e4f5059`. Sizes include opened values, exclude verifier-generated indices and randomness.

WHIR security target: 100 bits. Recursive folding terminates once message length is at most `2^8`; all folds use `2^4`.

Input-vector elements use the fixed 128-bit field and occupy 16 bytes. The Canopy/WHIR layer field is searched independently. Merkle hashes occupy 32 bytes.

No proof of work is used. Oracle queries and OOD samples target 100 bits; every fold uses one 128-bit-field sumcheck transcript. The RBR/list-adjusted single-challenge estimate can be below the target and is reported explicitly in `whir_parameters.csv`.

## Field feasibility

| log2 N | requested bits | feasible | log2 field size | bytes | two-adicity | reason |
|---:|---:|:---:|---:|---:|---:|---|
| 30 | 128 | yes | 127.584963 | 16 | 30 | supports the largest requested 2^30 multiplicative radix-2 domain |

## Best height for each feasible layer field and codeword length

| log2 N | batch | layer bits | codeword | best h | Merkle KiB | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |
|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|
| 30 | 128 | 128 | N/1 | 9 | 90.50 | 70.34 | 58.23 | 12.11 | 0.777 |
| 30 | 128 | 128 | N/2 | 10 | 90.50 | 71.70 | 57.67 | 14.03 | 0.792 |
| 30 | 128 | 128 | N/4 | 11 | 90.50 | 72.75 | 60.76 | 11.98 | 0.804 |

## Overall optimum

| log2 N | batch | h candidates | Merkle KiB | layer bits | codeword | best h | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |
|---:|---:|---|---:|---:|---|---:|---:|---:|---:|---:|
| 30 | 128 | 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15 | 90.50 | 128 | N/1 | 9 | 70.34 | 58.23 | 12.11 | 0.777 |
