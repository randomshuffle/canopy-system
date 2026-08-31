# Canopy vs. Merkle batch-opening proof sizes

Mean of 100 deterministic random index/query samples; seed `0x43414e4f5059`. Sizes include opened values, exclude verifier-generated indices and randomness.

WHIR security target: 128 bits. Recursive folding terminates once message length is at most `2^8`; all folds use `2^4`.

Input-vector elements use the fixed 32-bit field and occupy 4 bytes. The Canopy/WHIR layer field is searched independently. Merkle hashes occupy 32 bytes.

No proof of work is used. Oracle queries are selected from the capacity-assumption code distance; OOD samples and independent sumcheck repetitions are selected from the field/list soundness bounds and the 256-bit hash collision-security cap.

## Field feasibility

| log2 N | requested bits | feasible | log2 field size | bytes | two-adicity | reason |
|---:|---:|:---:|---:|---:|---:|---|
| 22 | 32 | yes | 31.584963 | 4 | 30 | supports the largest requested 2^22 radix-2 domain |

## Best height for each feasible layer field and codeword length

| log2 N | batch | layer bits | codeword | best h | Merkle KiB | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |
|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|
| 22 | 128 | 32 | N/1 | 6 | 56.98 | 44.65 | 39.53 | 5.12 | 0.783 |
| 22 | 128 | 32 | N/2 | 6 | 56.98 | 45.99 | 40.44 | 5.55 | 0.807 |
| 22 | 128 | 32 | N/4 | 7 | 56.98 | 47.96 | 42.97 | 4.99 | 0.842 |

## Overall optimum

| log2 N | batch | h candidates | Merkle KiB | layer bits | codeword | best h | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |
|---:|---:|---|---:|---:|---|---:|---:|---:|---:|---:|
| 22 | 128 | 1,2,3,4,5,6,7,8,9,10,11,12,13,14,15 | 56.98 | 32 | N/1 | 6 | 44.65 | 39.53 | 5.12 | 0.783 |
