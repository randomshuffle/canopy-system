# Conjectural capacity-bound paper proof-size experiments

All experiments target 128-bit soundness over the 256-bit prime field, use BLAKE3-256, use no proof of work, search every ordered folding schedule over `{3,4}` that terminates at exactly `2^10` coefficients, and use the capacity_conjecture RS proximity regime. Capacity-bound soundness relies on STIR Conjecture 5.6 and is not proved. Means use 100 deterministic query samples from seed `0x50415045525f5631`.

## Experiment 1: one Canopy commitment vs Merkle

The table reports the proof-size-optimal Canopy height and folding schedule over the full feasible grid. Detailed rows and proof components are in `experiment1_canopy_vs_merkle/`.

| log2 vector | batch | best h | folds | Merkle KiB | Canopy KiB | Canopy / Merkle |
|---:|---:|---:|---|---:|---:|---:|
| 22 | 128 | 9 | 3 | 60.48 | 83.55 | 1.381x |
| 22 | 256 | 5 | 3,4 | 112.94 | 115.65 | 1.024x |
| 22 | 512 | 3 | 3,3,3 | 209.87 | 154.75 | 0.737x |
| 22 | 1024 | 2 | 3,3,4 | 387.71 | 211.07 | 0.544x |
| 24 | 128 | 10 | 4 | 68.51 | 90.10 | 1.315x |
| 24 | 256 | 5 | 3,3,3 | 128.88 | 125.00 | 0.970x |
| 24 | 512 | 3 | 3,4,4 | 241.72 | 168.63 | 0.698x |
| 24 | 1024 | 2 | 3,3,3,3 | 451.73 | 225.31 | 0.499x |
| 26 | 128 | 12 | 4 | 76.43 | 96.88 | 1.268x |
| 26 | 256 | 6 | 4,3,3 | 144.95 | 133.63 | 0.922x |
| 26 | 512 | 4 | 4,4,4 | 273.73 | 179.00 | 0.654x |
| 26 | 1024 | 2 | 3,4,4,3 | 515.51 | 240.78 | 0.467x |
| 28 | 128 | 10 | 4,4 | 84.48 | 103.08 | 1.220x |
| 28 | 256 | 7 | 4,4,3 | 160.92 | 141.26 | 0.878x |
| 28 | 512 | 4 | 4,4,3,3 | 305.84 | 189.37 | 0.619x |
| 28 | 1024 | 2 | 3,3,4,3,3 | 579.51 | 255.71 | 0.441x |
| 30 | 128 | 12 | 4,4 | 92.45 | 109.15 | 1.181x |
| 30 | 256 | 7 | 4,3,3,3 | 176.90 | 150.84 | 0.853x |
| 30 | 512 | 4 | 4,4,4,4 | 337.79 | 198.50 | 0.588x |
| 30 | 1024 | 3 | 3,4,3,4,3 | 643.53 | 268.32 | 0.417x |

## Experiments 2 and 3: WHIR hybrids

Every folding step is independently selected from `{3,4}`, subject to ending at exactly `2^10` coefficients. The hybrids use no Canopy codeword shortening. Experiment 2 independently searches Canopy height, the outer WHIR schedule, and the Canopy-claim schedule. Experiment 3 needs identical RS codes, fixes `h=f0+1`, and uses the outer schedule after `f0` for its combined aligned claim. It sends one pre-challenge cross-term sum, samples `alpha`, and proves the single degree-two identity for `f+alpha*g` under `L_f+alpha*L_g`. Its additional batching error is at most `2/|F|`, approximately `2^-255`. The complete search is in `hybrid_parameter_grid.csv`.

## WHIR and hybrid proof sizes

| log2 message | scheme | outer folds | h | Canopy folds | KiB | vs Merkle WHIR | hashes KiB | fields KiB |
|---:|---|---|---:|---|---:|---:|---:|---:|
| 22 | merkle_whir | 3,3,3,3 | - | - | 177.32 | 1.000x | 86.39 | 90.93 |
| 22 | canopy_separate | 3,3,3,3 | 10 | 3 | 190.53 | 1.075x | 67.34 | 123.19 |
| 22 | canopy_schwartz_zippel | 3,3,3,3 | 4 | 3,3,3 | 144.79 | 0.817x | 49.79 | 95.00 |
| 24 | merkle_whir | 3,3,4,4 | - | - | 199.87 | 1.000x | 98.49 | 101.37 |
| 24 | canopy_separate | 3,4,4,3 | 11 | 4 | 208.95 | 1.045x | 71.48 | 137.47 |
| 24 | canopy_schwartz_zippel | 3,3,4,4 | 4 | 3,4,4 | 158.20 | 0.792x | 54.27 | 103.94 |
| 26 | merkle_whir | 3,3,3,4,3 | - | - | 219.70 | 1.000x | 120.35 | 99.34 |
| 26 | canopy_separate | 3,3,4,3,3 | 10 | 4,3 | 225.94 | 1.028x | 88.19 | 137.75 |
| 26 | canopy_schwartz_zippel | 3,3,4,3,3 | 4 | 3,4,3,3 | 170.97 | 0.778x | 67.57 | 103.41 |
| 28 | merkle_whir | 3,3,4,4,4 | - | - | 240.71 | 1.000x | 132.93 | 107.78 |
| 28 | canopy_separate | 3,4,4,4,3 | 11 | 4,4 | 241.98 | 1.005x | 92.94 | 149.03 |
| 28 | canopy_schwartz_zippel | 3,3,4,4,4 | 4 | 3,4,4,4 | 183.31 | 0.762x | 73.47 | 109.84 |
| 30 | merkle_whir | 3,3,4,3,4,3 | - | - | 261.40 | 1.000x | 154.65 | 106.75 |
| 30 | canopy_separate | 3,4,4,3,3,3 | 13 | 4,4 | 260.12 | 0.995x | 112.87 | 147.25 |
| 30 | canopy_schwartz_zippel | 3,3,4,3,3,4 | 4 | 3,4,3,3,4 | 196.56 | 0.752x | 87.74 | 108.81 |

## Experiment 3 vs Merkle WHIR components

All entries below are KiB. Roots and authentication paths are hashes; the remaining columns are 256-bit field elements.

| log2 message | scheme | roots | authentication | OOD | sumcheck | batching | query answers | terminal coefficients | total |
|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 0.12 | 86.26 | 0.12 | 2.06 | 0.00 | 56.74 | 32.00 | 177.32 |
| 22 | canopy_schwartz_zippel | 0.12 | 49.66 | 0.16 | 2.06 | 0.03 | 60.75 | 32.00 | 144.79 |
| 24 | merkle_whir | 0.12 | 98.37 | 0.12 | 2.25 | 0.00 | 67.00 | 32.00 | 199.87 |
| 24 | canopy_schwartz_zippel | 0.12 | 54.14 | 0.16 | 2.25 | 0.03 | 69.50 | 32.00 | 158.20 |
| 26 | merkle_whir | 0.16 | 120.20 | 0.16 | 2.44 | 0.00 | 64.75 | 32.00 | 219.70 |
| 26 | canopy_schwartz_zippel | 0.16 | 67.41 | 0.19 | 2.44 | 0.03 | 68.75 | 32.00 | 170.97 |
| 28 | merkle_whir | 0.16 | 132.77 | 0.16 | 2.62 | 0.00 | 73.00 | 32.00 | 240.71 |
| 28 | canopy_schwartz_zippel | 0.16 | 73.31 | 0.19 | 2.62 | 0.03 | 75.00 | 32.00 | 183.31 |
| 30 | merkle_whir | 0.19 | 154.46 | 0.19 | 2.81 | 0.00 | 71.75 | 32.00 | 261.40 |
| 30 | canopy_schwartz_zippel | 0.19 | 87.56 | 0.22 | 2.81 | 0.03 | 73.75 | 32.00 | 196.56 |

## Oracle-query breakdown

The machine-readable table is `oracle_query_breakdown.csv`. Values below are the largest proof-size point (`n=30`).

| experiment | scheme | oracle | code | requested | unique | auth hashes | answer fields |
|---:|---|---|---|---:|---:|---:|---:|
| 2 | canopy_separate | Canopy claim WHIR initial oracle | [2^31, 2^18] | 10 | 10.00 | 229.17 | 160.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 1 | [2^30, 2^14] | 9 | 9.00 | 198.01 | 144.00 |
| 2 | canopy_separate | outer first oracle via Canopy | [2^31, 2^30] | 138 | 138.00 | 1379.83 | 1104.00 |
| 2 | canopy_separate | outer second-oracle WHIR initial oracle | [2^31, 2^27] | 33 | 33.00 | 695.36 | 528.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 1 | [2^30, 2^23] | 19 | 19.00 | 397.26 | 304.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 2 | [2^29, 2^19] | 13 | 13.00 | 279.24 | 104.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 3 | [2^28, 2^16] | 11 | 11.00 | 227.48 | 88.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 4 | [2^27, 2^13] | 10 | 10.00 | 198.61 | 80.00 |
| 2 | merkle_whir | outer WHIR initial oracle | [2^31, 2^30] | 138 | 138.00 | 2760.30 | 1104.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 1 | [2^30, 2^27] | 44 | 44.00 | 908.53 | 352.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 2 | [2^29, 2^24] | 26 | 26.00 | 504.98 | 416.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 3 | [2^28, 2^20] | 17 | 17.00 | 341.10 | 136.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 4 | [2^27, 2^17] | 13 | 13.00 | 239.44 | 208.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 5 | [2^26, 2^13] | 10 | 10.00 | 188.45 | 80.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim initial oracle | [2^31, 2^27] | 33 | 33.00 | 1453.30 | 528.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 1 | [2^30, 2^24] | 22 | 22.00 | 454.41 | 352.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 2 | [2^29, 2^20] | 15 | 15.00 | 319.31 | 120.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 3 | [2^28, 2^17] | 12 | 12.00 | 247.28 | 96.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 4 | [2^27, 2^14] | 10 | 10.00 | 189.47 | 160.00 |
| 3 | canopy_schwartz_zippel | outer first oracle via Canopy | [2^31, 2^30] | 138 | 138.00 | 138.00 | 1104.00 |
