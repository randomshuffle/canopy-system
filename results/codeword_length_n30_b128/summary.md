# Canopy vs. Merkle batch-opening proof sizes

Mean of 100 deterministic random index/query samples; seed `0x43414e4f5059`. Sizes include opened values, exclude verifier-generated indices and randomness.

WHIR security target: 128 bits. Recursive folding terminates once message length is at most `2^8`; all folds use `2^4`.

Input-vector elements use the fixed 32-bit field and occupy 4 bytes. The Canopy/WHIR layer field is searched independently. Merkle hashes occupy 32 bytes.

No proof of work is used. Oracle queries are selected from the capacity-assumption code distance; OOD samples and independent sumcheck repetitions are selected from the field/list soundness bounds and the 256-bit hash collision-security cap.

## Field feasibility

| log2 N | requested bits | feasible | log2 field size | bytes | two-adicity | reason |
|---:|---:|:---:|---:|---:|---:|---|
| 30 | 32 | yes | 31.584963 | 4 | 30 | supports the largest requested 2^30 radix-2 domain |

## Best height for each feasible layer field and codeword length

| log2 N | batch | layer bits | codeword | best h | Merkle KiB | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |
|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|
| 30 | 128 | 32 | N/1 | 10 | 89.00 | 67.27 | 62.15 | 5.11 | 0.756 |
| 30 | 128 | 32 | N/2 | 10 | 89.00 | 68.16 | 62.87 | 5.29 | 0.766 |
| 30 | 128 | 32 | N/4 | 10 | 89.00 | 69.34 | 63.80 | 5.54 | 0.779 |

## Overall optimum

| log2 N | batch | h candidates | Merkle KiB | layer bits | codeword | best h | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |
|---:|---:|---|---:|---:|---|---:|---:|---:|---:|---:|
| 30 | 128 | 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15 | 89.00 | 32 | N/1 | 10 | 67.27 | 62.15 | 5.11 | 0.756 |
