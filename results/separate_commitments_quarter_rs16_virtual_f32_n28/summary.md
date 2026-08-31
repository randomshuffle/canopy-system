# Separate-commitment Canopy vs. Merkle proof sizes

Each of the `t` vectors has a separate commitment and separate Merkle path. All commitments open the same `B` logical indices and use the same SEP-reduction challenge so their functionals align. The old codeword roots jointly bind a virtual initial WHIR oracle: at each initial WHIR query, the prover opens one width-16 interleaving column in every old commitment and the verifier derives all random-linear-combination values locally. There is no separate combined-oracle commitment or standalone consistency check.

The number of independent linear combinations is `ceil(128 / log2(|F|))`, providing at least 128 batching-soundness bits for each requested field. RS interleaving factor 16 supplies the effective first width-16 fold, so the additional initial WHIR folding factor is one; recursive folds remain width 16. The initial old-oracle openings use WHIR's capacity-bound query count, all parallel instances share those openings, and only recursive WHIR roots and paths are committed.

Mean of 100 deterministic samples; seed `0x5345504152415445`. WHIR targets 128 bits, terminates at at most `2^8` variables, and folds `2^4` values per column.

| log2 N | t | B each | total opened | Merkle KiB | field | codeword | RS blocks | combinations | WHIR instances | best h | initial WHIR q | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 28 | 8 | 64 | 512 | 339.99 | 32-bit | N/4 | 16 x 2^22 | 5 | 5 | 9 | 20 | 257.98 | 233.81 | 24.17 | 0.759 |
| 28 | 16 | 64 | 1024 | 679.98 | 32-bit | N/4 | 16 x 2^22 | 5 | 5 | 9 | 20 | 498.38 | 462.21 | 36.17 | 0.733 |
| 28 | 32 | 64 | 2048 | 1359.96 | 32-bit | N/4 | 16 x 2^22 | 5 | 5 | 9 | 20 | 979.17 | 919.00 | 60.17 | 0.720 |
| 28 | 8 | 128 | 1024 | 647.89 | 32-bit | N/4 | 16 x 2^22 | 5 | 5 | 7 | 29 | 388.56 | 353.42 | 35.14 | 0.600 |
| 28 | 16 | 128 | 2048 | 1295.79 | 32-bit | N/4 | 16 x 2^22 | 5 | 5 | 7 | 29 | 749.02 | 695.38 | 53.64 | 0.578 |
| 28 | 32 | 128 | 4096 | 2591.57 | 32-bit | N/4 | 16 x 2^22 | 5 | 5 | 7 | 29 | 1469.93 | 1379.29 | 90.64 | 0.567 |
| 28 | 8 | 256 | 2048 | 1231.65 | 32-bit | N/4 | 16 x 2^22 | 5 | 5 | 6 | 38 | 592.68 | 547.16 | 45.52 | 0.481 |
| 28 | 16 | 256 | 4096 | 2463.31 | 32-bit | N/4 | 16 x 2^22 | 5 | 5 | 6 | 38 | 1154.43 | 1081.92 | 72.52 | 0.469 |
| 28 | 32 | 256 | 8192 | 4926.62 | 32-bit | N/4 | 16 x 2^22 | 5 | 5 | 6 | 38 | 2277.94 | 2151.43 | 126.52 | 0.462 |
| 28 | 8 | 512 | 4096 | 2334.68 | 32-bit | N/4 | 16 x 2^22 | 5 | 5 | 5 | 54 | 926.57 | 861.58 | 64.99 | 0.397 |
| 28 | 16 | 512 | 8192 | 4669.36 | 32-bit | N/4 | 16 x 2^22 | 5 | 5 | 5 | 54 | 1816.85 | 1708.86 | 107.99 | 0.389 |
| 28 | 32 | 512 | 16384 | 9338.72 | 32-bit | N/4 | 16 x 2^22 | 5 | 5 | 5 | 54 | 3597.41 | 3403.42 | 193.99 | 0.385 |
| 28 | 8 | 1024 | 8192 | 4410.55 | 32-bit | N/4 | 16 x 2^22 | 5 | 5 | 4 | 91 | 1477.48 | 1372.98 | 104.49 | 0.335 |
| 28 | 16 | 1024 | 16384 | 8821.10 | 32-bit | N/4 | 16 x 2^22 | 5 | 5 | 4 | 91 | 2911.52 | 2729.52 | 181.99 | 0.330 |
| 28 | 32 | 1024 | 32768 | 17642.21 | 32-bit | N/4 | 16 x 2^22 | 5 | 5 | 4 | 91 | 5779.60 | 5442.60 | 336.99 | 0.328 |
