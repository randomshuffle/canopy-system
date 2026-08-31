# Canopy vs. Merkle batch-opening proof sizes

Mean of 100 deterministic random index/query samples; seed `0x50415045525f5631`. Sizes include opened values, exclude verifier-generated indices and randomness.

WHIR security target: 128 bits. Recursive folding terminates once message length is at most `2^3`; all folds use `2^4`.

Input-vector elements use the fixed 256-bit field and occupy 32 bytes. The Canopy/WHIR layer field is searched independently. Merkle hashes occupy 32 bytes.

No proof of work is used. Oracle queries, OOD samples, and every fold target 128 bits in the capacity_conjecture regime over the 256-bit field. These estimates rely on STIR Conjecture 5.6 and are not proved.

## Field feasibility

| log2 N | requested bits | feasible | log2 field size | bytes | two-adicity | reason |
|---:|---:|:---:|---:|---:|---:|---|
| 20 | 256 | yes | 256.000000 | 32 | 32 | supports the largest requested 2^20 multiplicative radix-2 domain |

## Best height for each feasible layer field and codeword length

| log2 N | batch | layer bits | codeword | best h | Merkle KiB | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |
|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|
| 20 | 128 | 256 | N/1 | 16 | 52.50 | 60.83 | 51.90 | 8.94 | 1.159 |
| 20 | 256 | 256 | N/1 | 5 | 97.00 | 94.65 | 57.41 | 37.24 | 0.976 |
| 20 | 512 | 256 | N/1 | 3 | 177.73 | 139.08 | 75.32 | 63.76 | 0.783 |
| 20 | 1024 | 256 | N/1 | 2 | 323.77 | 193.62 | 98.71 | 94.91 | 0.598 |

## Overall optimum

| log2 N | batch | h candidates | Merkle KiB | layer bits | codeword | best h | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |
|---:|---:|---|---:|---:|---|---:|---:|---:|---:|---:|
| 20 | 128 | 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16 | 52.50 | 256 | N/1 | 16 | 60.83 | 51.90 | 8.94 | 1.159 |
| 20 | 256 | 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16 | 97.00 | 256 | N/1 | 5 | 94.65 | 57.41 | 37.24 | 0.976 |
| 20 | 512 | 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16 | 177.73 | 256 | N/1 | 3 | 139.08 | 75.32 | 63.76 | 0.783 |
| 20 | 1024 | 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16 | 323.77 | 256 | N/1 | 2 | 193.62 | 98.71 | 94.91 | 0.598 |
