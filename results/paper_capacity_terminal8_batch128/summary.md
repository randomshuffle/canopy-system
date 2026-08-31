# Canopy vs. Merkle batch-opening proof sizes

Mean of 100 deterministic random index/query samples; seed `0x50415045525f5631`. Sizes include opened values, exclude verifier-generated indices and randomness.

WHIR security target: 128 bits. Recursive folding terminates once message length is at most `2^8`; all folds use `2^4`.

Input-vector elements use the fixed 256-bit field and occupy 32 bytes. The Canopy/WHIR layer field is searched independently. Merkle hashes occupy 32 bytes.

No proof of work is used. Oracle queries, OOD samples, and every fold target 128 bits in the capacity_conjecture regime over the 256-bit field. These estimates rely on STIR Conjecture 5.6 and are not proved.

## Field feasibility

| log2 N | requested bits | feasible | log2 field size | bytes | two-adicity | reason |
|---:|---:|:---:|---:|---:|---:|---|
| 22 | 256 | yes | 256.000000 | 32 | 32 | supports the largest requested 2^22 multiplicative radix-2 domain |
| 24 | 256 | yes | 256.000000 | 32 | 32 | supports the largest requested 2^24 multiplicative radix-2 domain |
| 26 | 256 | yes | 256.000000 | 32 | 32 | supports the largest requested 2^26 multiplicative radix-2 domain |
| 28 | 256 | yes | 256.000000 | 32 | 32 | supports the largest requested 2^28 multiplicative radix-2 domain |
| 30 | 256 | yes | 256.000000 | 32 | 32 | supports the largest requested 2^30 multiplicative radix-2 domain |

## Best height for each feasible layer field and codeword length

| log2 N | batch | layer bits | codeword | best h | Merkle KiB | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |
|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|
| 22 | 128 | 256 | N/1 | 11 | 60.48 | 63.79 | 48.73 | 15.06 | 1.055 |
| 24 | 128 | 256 | N/1 | 10 | 68.51 | 69.80 | 50.93 | 18.88 | 1.019 |
| 26 | 128 | 256 | N/1 | 12 | 76.43 | 76.23 | 58.85 | 17.38 | 0.997 |
| 28 | 128 | 256 | N/1 | 13 | 84.48 | 82.53 | 63.56 | 18.97 | 0.977 |
| 30 | 128 | 256 | N/1 | 12 | 92.45 | 88.25 | 66.47 | 21.78 | 0.955 |

## Overall optimum

| log2 N | batch | h candidates | Merkle KiB | layer bits | codeword | best h | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |
|---:|---:|---|---:|---:|---|---:|---:|---:|---:|---:|
| 22 | 128 | 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18 | 60.48 | 256 | N/1 | 11 | 63.79 | 48.73 | 15.06 | 1.055 |
| 24 | 128 | 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20 | 68.51 | 256 | N/1 | 10 | 69.80 | 50.93 | 18.88 | 1.019 |
| 26 | 128 | 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22 | 76.43 | 256 | N/1 | 12 | 76.23 | 58.85 | 17.38 | 0.997 |
| 28 | 128 | 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24 | 84.48 | 256 | N/1 | 13 | 82.53 | 63.56 | 18.97 | 0.977 |
| 30 | 128 | 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26 | 92.45 | 256 | N/1 | 12 | 88.25 | 66.47 | 21.78 | 0.955 |
