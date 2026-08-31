# Separate-commitment Canopy vs. Merkle proof sizes

Each of the `t` vectors has a separate commitment and separate Merkle path. All commitments open the same `B` logical indices and use the same SEP-reduction challenge so their functionals align. Canopy replaces `t` independent WHIR executions with independently randomized linear combinations, interleaves the resulting codewords in one commitment, checks that commitment against all old codeword commitments, and runs parallel WHIR instances with shared oracle paths.

The number of linear combinations is `ceil(128 / log2(|F|))`. If the RS interleaving factor is `s`, each total codeword is split into `s` equal blocks. The random batching challenge combines all `s*t` stripe claims into that same number of scalar WHIR instances; their committed oracles and paths are interleaved. For codeword log reduction `r`, consistency uses the RS minimum distance `1-2^-(h-r)` and `ceil(128/(h-r))` shared queries. The factor-1 baseline derives complete packed leaves from old openings. For interleaved RS, consistency instead samples one coordinate: all old stripes are opened, the checked combined values are verifier-derived, and only the unchecked offsets in the packed WHIR leaf are serialized. The combined-oracle authentication frontier is merged with its first WHIR opening. WHIR retains the existing capacity-bound estimator.

Mean of 100 deterministic samples; seed `0x5345504152415445`. WHIR targets 128 bits, terminates at at most `2^8` variables, and folds `2^4` values per column.

| log2 N | t | B each | total opened | Merkle KiB | field | codeword | RS blocks | combinations | WHIR instances | best h | consistency q | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 28 | 8 | 64 | 512 | 339.99 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 10 | 16 | 273.97 | 246.48 | 27.49 | 0.806 |
| 28 | 16 | 64 | 1024 | 679.98 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 9 | 19 | 509.17 | 468.30 | 40.87 | 0.749 |
| 28 | 32 | 64 | 2048 | 1359.96 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 8 | 22 | 970.66 | 901.93 | 68.74 | 0.714 |
| 28 | 8 | 128 | 1024 | 647.89 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 7 | 26 | 407.04 | 361.99 | 45.05 | 0.628 |
| 28 | 16 | 128 | 2048 | 1295.79 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 6 | 32 | 747.82 | 678.56 | 69.26 | 0.577 |
| 28 | 32 | 128 | 4096 | 2591.57 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 6 | 32 | 1421.94 | 1320.68 | 101.26 | 0.549 |
| 28 | 8 | 256 | 2048 | 1231.65 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 6 | 32 | 606.54 | 549.28 | 57.27 | 0.492 |
| 28 | 16 | 256 | 4096 | 2463.31 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 5 | 43 | 1119.36 | 1023.18 | 96.18 | 0.454 |
| 28 | 32 | 256 | 8192 | 4926.62 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 5 | 43 | 2144.62 | 2000.19 | 144.43 | 0.435 |
| 28 | 8 | 512 | 4096 | 2334.68 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 4 | 64 | 926.76 | 818.14 | 108.62 | 0.397 |
| 28 | 16 | 512 | 8192 | 4669.36 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 4 | 64 | 1720.74 | 1572.12 | 148.62 | 0.369 |
| 28 | 32 | 512 | 16384 | 9338.72 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 4 | 64 | 3308.69 | 3080.07 | 228.62 | 0.354 |
| 28 | 8 | 1024 | 8192 | 4410.55 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 4 | 64 | 1454.68 | 1330.06 | 124.61 | 0.330 |
| 28 | 16 | 1024 | 16384 | 8821.10 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 4 | 64 | 2776.56 | 2595.95 | 180.61 | 0.315 |
| 28 | 32 | 1024 | 32768 | 17642.21 | 24-bit | N/4 | 16 x 2^22 | 6 | 6 | 4 | 64 | 5420.33 | 5127.72 | 292.61 | 0.307 |
