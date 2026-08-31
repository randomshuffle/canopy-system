# Separate-commitment Canopy vs. Merkle proof sizes

Each of the `t` vectors has a separate commitment and separate Merkle path. All commitments open the same `B` logical indices and use the same SEP-reduction challenge so their functionals align. The old codeword roots jointly bind a virtual initial WHIR oracle: at each initial WHIR query, the prover opens one width-16 interleaving column in every old commitment and the verifier derives all random-linear-combination values locally. There is no separate combined-oracle commitment or standalone consistency check.

The number of independent linear combinations is `ceil(128 / log2(|F|))`, providing at least 128 batching-soundness bits for each requested field. RS interleaving factor 16 supplies the effective first width-16 fold, so the additional initial WHIR folding factor is one; recursive folds remain width 16. The initial old-oracle openings use WHIR's capacity-bound query count, all parallel instances share those openings, and only recursive WHIR roots and paths are committed.

Mean of 100 deterministic samples; seed `0x5345504152415445`. WHIR targets 128 bits, terminates at at most `2^8` variables, and folds `2^4` values per column.

| log2 N | t | B each | total opened | Merkle KiB | field | codeword | RS blocks | combinations | WHIR instances | best h | initial WHIR q | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 28 | 8 | 64 | 512 | 339.99 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 9 | 20 | 255.30 | 233.81 | 21.49 | 0.751 |
| 28 | 16 | 64 | 1024 | 679.98 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 9 | 20 | 493.20 | 462.21 | 30.99 | 0.725 |
| 28 | 32 | 64 | 2048 | 1359.96 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 9 | 20 | 968.99 | 919.00 | 49.99 | 0.713 |
| 28 | 8 | 128 | 1024 | 647.89 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 7 | 29 | 384.59 | 353.42 | 31.17 | 0.594 |
| 28 | 16 | 128 | 2048 | 1295.79 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 7 | 29 | 741.42 | 695.38 | 46.04 | 0.572 |
| 28 | 32 | 128 | 4096 | 2591.57 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 7 | 29 | 1455.08 | 1379.29 | 75.79 | 0.561 |
| 28 | 8 | 256 | 2048 | 1231.65 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 6 | 38 | 587.55 | 547.16 | 40.39 | 0.477 |
| 28 | 16 | 256 | 4096 | 2463.31 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 6 | 38 | 1144.56 | 1081.92 | 62.64 | 0.465 |
| 28 | 32 | 256 | 8192 | 4926.62 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 5 | 54 | 2257.92 | 2123.51 | 134.41 | 0.458 |
| 28 | 8 | 512 | 4096 | 2334.68 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 5 | 54 | 919.24 | 861.58 | 57.66 | 0.394 |
| 28 | 16 | 512 | 8192 | 4669.36 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 5 | 54 | 1802.77 | 1708.86 | 93.91 | 0.386 |
| 28 | 32 | 512 | 16384 | 9338.72 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 5 | 54 | 3569.83 | 3403.42 | 166.41 | 0.382 |
| 28 | 8 | 1024 | 8192 | 4410.55 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 4 | 91 | 1465.27 | 1372.98 | 92.28 | 0.332 |
| 28 | 16 | 1024 | 16384 | 8821.10 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 4 | 91 | 2887.93 | 2729.52 | 158.41 | 0.327 |
| 28 | 32 | 1024 | 32768 | 17642.21 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 4 | 91 | 5733.26 | 5442.60 | 290.66 | 0.325 |
