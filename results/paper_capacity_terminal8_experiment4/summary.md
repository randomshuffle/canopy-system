# Conjectural capacity-bound paper proof-size experiments

All experiments target 128-bit soundness over the 256-bit prime field, use BLAKE3-256, use no proof of work, use terminal polynomials of at most `2^8`, and use the capacity_conjecture RS proximity regime. Capacity-bound soundness relies on STIR Conjecture 5.6 and is not proved. Means use 100 deterministic query samples from seed `0x50415045525f5631`.

## Experiment 1: one Canopy commitment vs Merkle

The table reports the proof-size-optimal Canopy height over the full feasible grid. Detailed rows and proof components are in `experiment1_canopy_vs_merkle/`.

| log2 vector | batch | best h | Merkle KiB | Canopy KiB | Canopy / Merkle |
|---:|---:|---:|---:|---:|---:|
| 22 | 128 | 11 | 60.48 | 63.79 | 1.055x |
| 22 | 256 | 7 | 112.94 | 98.15 | 0.869x |
| 22 | 512 | 3 | 209.87 | 139.77 | 0.666x |
| 22 | 1024 | 2 | 387.71 | 199.28 | 0.514x |
| 24 | 128 | 10 | 68.51 | 69.80 | 1.019x |
| 24 | 256 | 6 | 128.88 | 105.34 | 0.817x |
| 24 | 512 | 4 | 241.72 | 150.78 | 0.624x |
| 24 | 1024 | 2 | 451.73 | 211.24 | 0.468x |
| 26 | 128 | 12 | 76.43 | 76.23 | 0.997x |
| 26 | 256 | 7 | 144.95 | 112.63 | 0.777x |
| 26 | 512 | 4 | 273.73 | 158.78 | 0.580x |
| 26 | 1024 | 3 | 515.51 | 224.81 | 0.436x |
| 28 | 128 | 13 | 84.48 | 82.53 | 0.977x |
| 28 | 256 | 6 | 160.92 | 121.05 | 0.752x |
| 28 | 512 | 4 | 305.84 | 169.64 | 0.555x |
| 28 | 1024 | 2 | 579.51 | 236.92 | 0.409x |
| 30 | 128 | 12 | 92.45 | 88.25 | 0.955x |
| 30 | 256 | 7 | 176.90 | 128.17 | 0.725x |
| 30 | 512 | 4 | 337.79 | 178.12 | 0.527x |
| 30 | 1024 | 3 | 643.53 | 246.43 | 0.383x |

## Experiments 2--4: WHIR hybrids

For every scheme and message length, the first WHIR fold `f0` is grid-searched independently; later folds use four variables. The hybrid sets `h=f0+1` with a one-bit Canopy codeword shortening, so Canopy's output and outer WHIR's second oracle both encode `2^(n-f0)` coefficients into `2^n` symbols at inverse rate `2^f0`. Experiment 2 proves the aligned claims separately. Experiment 3 shares recursive Merkle authentication but retains both field elements and both algebraic claims. Experiment 4 sends one pre-challenge cross-term sum, samples `alpha`, and proves the single degree-two identity for `f+alpha*g` under `L_f+alpha*L_g`; after the two initial-root openings it carries only one recursive oracle and one algebraic claim. Its additional batching error is at most `2/|F|`, approximately `2^-255`. The complete search is in `hybrid_initial_folding_grid.csv`.

## WHIR and hybrid proof sizes

| log2 message | scheme | f0 | h | KiB | vs Merkle WHIR | hashes KiB | fields KiB |
|---:|---|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 3 | - | 162.34 | 1.000x | 81.16 | 81.19 |
| 22 | canopy_separate | 3 | 4 | 190.76 | 1.175x | 63.20 | 127.56 |
| 22 | canopy_batched | 3 | 4 | 177.36 | 1.092x | 49.80 | 127.55 |
| 22 | canopy_schwartz_zippel | 3 | 4 | 153.02 | 0.943x | 49.77 | 103.25 |
| 24 | merkle_whir | 3 | - | 183.19 | 1.000x | 99.28 | 83.90 |
| 24 | canopy_separate | 3 | 4 | 215.15 | 1.174x | 82.16 | 133.00 |
| 24 | canopy_batched | 3 | 4 | 194.92 | 1.064x | 61.92 | 133.00 |
| 24 | canopy_schwartz_zippel | 3 | 4 | 167.99 | 0.917x | 62.02 | 105.97 |
| 26 | merkle_whir | 3 | - | 200.63 | 1.000x | 113.54 | 87.09 |
| 26 | canopy_separate | 4 | 5 | 231.30 | 1.153x | 78.52 | 152.78 |
| 26 | canopy_batched | 3 | 4 | 209.82 | 1.046x | 70.44 | 139.38 |
| 26 | canopy_schwartz_zippel | 3 | 4 | 179.71 | 0.896x | 70.55 | 109.16 |
| 28 | merkle_whir | 3 | - | 221.45 | 1.000x | 132.63 | 88.81 |
| 28 | canopy_separate | 4 | 5 | 253.05 | 1.143x | 87.89 | 165.16 |
| 28 | canopy_batched | 3 | 4 | 226.41 | 1.022x | 83.60 | 142.81 |
| 28 | canopy_schwartz_zippel | 3 | 4 | 194.38 | 0.878x | 83.51 | 110.88 |
| 30 | merkle_whir | 3 | - | 239.49 | 1.000x | 147.49 | 92.00 |
| 30 | canopy_separate | 4 | 5 | 270.00 | 1.127x | 107.40 | 162.59 |
| 30 | canopy_batched | 4 | 5 | 239.24 | 0.999x | 76.64 | 162.59 |
| 30 | canopy_schwartz_zippel | 3 | 4 | 206.71 | 0.863x | 92.64 | 114.06 |

## Experiments 3 and 4 vs Merkle WHIR components

All entries below are KiB. Roots and authentication paths are hashes; the remaining columns are 256-bit field elements.

| log2 message | scheme | roots | authentication | OOD | sumcheck | batching | query answers | terminal coefficients | total |
|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 0.12 | 81.03 | 0.12 | 2.06 | 0.00 | 75.00 | 4.00 | 162.34 |
| 22 | canopy_batched | 0.12 | 49.68 | 0.22 | 3.84 | 0.00 | 115.49 | 8.00 | 177.36 |
| 22 | canopy_schwartz_zippel | 0.12 | 49.64 | 0.16 | 2.06 | 0.03 | 97.00 | 4.00 | 153.02 |
| 24 | merkle_whir | 0.16 | 99.13 | 0.16 | 2.25 | 0.00 | 80.50 | 1.00 | 183.19 |
| 24 | canopy_batched | 0.16 | 61.77 | 0.28 | 4.22 | 0.00 | 126.50 | 2.00 | 194.92 |
| 24 | canopy_schwartz_zippel | 0.16 | 61.87 | 0.19 | 2.25 | 0.03 | 102.50 | 1.00 | 167.99 |
| 26 | merkle_whir | 0.16 | 113.39 | 0.16 | 2.44 | 0.00 | 80.50 | 4.00 | 200.63 |
| 26 | canopy_batched | 0.16 | 70.28 | 0.28 | 4.59 | 0.00 | 126.50 | 8.00 | 209.82 |
| 26 | canopy_schwartz_zippel | 0.16 | 70.40 | 0.19 | 2.44 | 0.03 | 102.50 | 4.00 | 179.71 |
| 28 | merkle_whir | 0.19 | 132.45 | 0.19 | 2.62 | 0.00 | 85.00 | 1.00 | 221.45 |
| 28 | canopy_batched | 0.19 | 83.41 | 0.34 | 4.97 | 0.00 | 135.50 | 2.00 | 226.41 |
| 28 | canopy_schwartz_zippel | 0.19 | 83.32 | 0.22 | 2.62 | 0.03 | 107.00 | 1.00 | 194.38 |
| 30 | merkle_whir | 0.19 | 147.30 | 0.19 | 2.81 | 0.00 | 85.00 | 4.00 | 239.49 |
| 30 | canopy_batched | 0.19 | 76.45 | 0.34 | 5.25 | 0.00 | 153.00 | 4.00 | 239.24 |
| 30 | canopy_schwartz_zippel | 0.19 | 92.46 | 0.22 | 2.81 | 0.03 | 107.00 | 4.00 | 206.71 |

## Oracle-query breakdown

The machine-readable table is `oracle_query_breakdown.csv`. Values below are the largest proof-size point (`n=30`).

| experiment | scheme | oracle | code | requested | unique | auth hashes | answer fields |
|---:|---|---|---|---:|---:|---:|---:|
| 2 | canopy_separate | Canopy claim WHIR initial oracle | [2^30, 2^26] | 33 | 33.00 | 662.46 | 528.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 1 | [2^29, 2^22] | 19 | 19.00 | 378.86 | 304.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 2 | [2^28, 2^18] | 13 | 13.00 | 252.50 | 208.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 3 | [2^27, 2^14] | 10 | 10.00 | 188.72 | 160.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 4 | [2^26, 2^10] | 9 | 9.00 | 162.06 | 144.00 |
| 2 | canopy_separate | outer first oracle via Canopy | [2^31, 2^30] | 138 | 138.00 | 138.00 | 2208.00 |
| 2 | canopy_separate | outer second-oracle WHIR initial oracle | [2^30, 2^26] | 33 | 33.00 | 661.43 | 528.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 1 | [2^29, 2^22] | 19 | 19.00 | 378.67 | 304.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 2 | [2^28, 2^18] | 13 | 13.00 | 253.44 | 208.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 3 | [2^27, 2^14] | 10 | 10.00 | 188.97 | 160.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 4 | [2^26, 2^10] | 9 | 9.00 | 161.84 | 144.00 |
| 2 | merkle_whir | outer WHIR initial oracle | [2^31, 2^30] | 138 | 138.00 | 2760.30 | 1104.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 1 | [2^30, 2^27] | 44 | 44.00 | 864.93 | 704.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 2 | [2^29, 2^23] | 22 | 22.00 | 432.20 | 352.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 3 | [2^28, 2^19] | 15 | 15.00 | 289.17 | 240.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 4 | [2^27, 2^15] | 11 | 11.00 | 205.67 | 176.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 5 | [2^26, 2^11] | 9 | 9.00 | 161.33 | 144.00 |
| 3 | canopy_batched | batched aligned claims initial oracle | [2^30, 2^26] | 33 | 33.00 | 1325.66 | 1056.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 1 | [2^29, 2^22] | 19 | 19.00 | 378.37 | 608.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 2 | [2^28, 2^18] | 13 | 13.00 | 253.20 | 416.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 3 | [2^27, 2^14] | 10 | 10.00 | 188.91 | 320.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 4 | [2^26, 2^10] | 9 | 9.00 | 162.42 | 288.00 |
| 3 | canopy_batched | outer first oracle via Canopy | [2^31, 2^30] | 138 | 138.00 | 138.00 | 2208.00 |
| 4 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim initial oracle | [2^30, 2^27] | 44 | 44.00 | 1730.18 | 1408.00 |
| 4 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 1 | [2^29, 2^23] | 22 | 22.00 | 433.64 | 352.00 |
| 4 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 2 | [2^28, 2^19] | 15 | 15.00 | 288.91 | 240.00 |
| 4 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 3 | [2^27, 2^15] | 11 | 11.00 | 205.85 | 176.00 |
| 4 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 4 | [2^26, 2^11] | 9 | 9.00 | 162.02 | 144.00 |
| 4 | canopy_schwartz_zippel | outer first oracle via Canopy | [2^31, 2^30] | 138 | 138.00 | 138.00 | 1104.00 |
