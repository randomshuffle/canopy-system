# Separate-commitment Canopy vs. Merkle proof sizes

Each of the `t` vectors has a separate commitment and separate Merkle path. All commitments open the same `B` logical indices and use the same SEP-reduction challenge so their functionals align. Canopy replaces `t` independent WHIR executions with independently randomized linear combinations, interleaves the resulting codewords in one commitment, checks that commitment against all old codeword commitments, and runs parallel WHIR instances with shared oracle paths.

The number of linear combinations is `ceil(128 / log2(|F|))`. If the RS interleaving factor is `s`, each total codeword is split into `s` equal blocks. The random batching challenge combines all `s*t` stripe claims into that same number of scalar WHIR instances; their committed oracles and paths are interleaved. For codeword log reduction `r`, consistency uses the RS minimum distance `1-2^-(h-r)` and `ceil(128/(h-r))` shared queries. The factor-1 baseline derives complete packed leaves from old openings. For interleaved RS, consistency instead samples one coordinate: all old stripes are opened, the checked combined values are verifier-derived, and only the unchecked offsets in the packed WHIR leaf are serialized. The combined-oracle authentication frontier is merged with its first WHIR opening. WHIR retains the existing capacity-bound estimator.

Mean of 100 deterministic samples; seed `0x5345504152415445`. WHIR targets 128 bits, terminates at at most `2^8` variables, and folds `2^4` values per column.

| log2 N | t | B each | total opened | Merkle KiB | field | codeword | RS blocks | combinations | WHIR instances | best h | consistency q | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 28 | 8 | 64 | 512 | 339.99 | 24-bit | N/4 | 8 x 2^23 | 6 | 6 | 10 | 16 | 278.34 | 252.12 | 26.22 | 0.819 |
| 28 | 16 | 64 | 1024 | 679.98 | 24-bit | N/4 | 8 x 2^23 | 6 | 6 | 9 | 19 | 515.97 | 479.57 | 36.40 | 0.759 |
| 28 | 32 | 64 | 2048 | 1359.96 | 24-bit | N/4 | 8 x 2^23 | 6 | 6 | 8 | 22 | 983.06 | 930.33 | 52.73 | 0.723 |
| 28 | 8 | 128 | 1024 | 647.89 | 24-bit | N/4 | 8 x 2^23 | 6 | 6 | 7 | 26 | 412.58 | 371.40 | 41.18 | 0.637 |
| 28 | 16 | 128 | 2048 | 1295.79 | 24-bit | N/4 | 8 x 2^23 | 6 | 6 | 6 | 32 | 756.40 | 697.54 | 58.86 | 0.584 |
| 28 | 32 | 128 | 4096 | 2591.57 | 24-bit | N/4 | 8 x 2^23 | 6 | 6 | 6 | 32 | 1434.22 | 1355.36 | 78.86 | 0.553 |
| 28 | 8 | 256 | 2048 | 1231.65 | 24-bit | N/4 | 8 x 2^23 | 6 | 6 | 6 | 32 | 613.73 | 560.87 | 52.86 | 0.498 |
| 28 | 16 | 256 | 4096 | 2463.31 | 24-bit | N/4 | 8 x 2^23 | 6 | 6 | 5 | 43 | 1132.31 | 1049.43 | 82.87 | 0.460 |
| 28 | 32 | 256 | 8192 | 4926.62 | 24-bit | N/4 | 8 x 2^23 | 6 | 6 | 5 | 43 | 2163.37 | 2048.37 | 115.00 | 0.439 |
| 28 | 8 | 512 | 4096 | 2334.68 | 24-bit | N/4 | 8 x 2^23 | 6 | 6 | 4 | 64 | 942.77 | 845.32 | 97.45 | 0.404 |
| 28 | 16 | 512 | 8192 | 4669.36 | 24-bit | N/4 | 8 x 2^23 | 6 | 6 | 4 | 64 | 1740.94 | 1615.49 | 125.45 | 0.373 |
| 28 | 32 | 512 | 16384 | 9338.72 | 24-bit | N/4 | 8 x 2^23 | 6 | 6 | 4 | 64 | 3337.29 | 3155.84 | 181.45 | 0.357 |
| 28 | 8 | 1024 | 8192 | 4410.55 | 24-bit | N/4 | 8 x 2^23 | 6 | 6 | 4 | 64 | 1470.14 | 1356.70 | 113.45 | 0.333 |
| 28 | 16 | 1024 | 16384 | 8821.10 | 24-bit | N/4 | 8 x 2^23 | 6 | 6 | 4 | 64 | 2795.67 | 2638.22 | 157.45 | 0.317 |
| 28 | 32 | 1024 | 32768 | 17642.21 | 24-bit | N/4 | 8 x 2^23 | 6 | 6 | 4 | 64 | 5446.71 | 5201.27 | 245.45 | 0.309 |
