# Final Johnson-bound paper proof-size experiments

All experiments target 128-bit soundness over the 256-bit prime field, use BLAKE3-256, use no proof of work, use terminal polynomials of at most `2^8`, use three-variable folds everywhere, and use the johnson RS proximity regime. Means use 100 deterministic query samples from seed `0x50415045525f5631`.

## Experiment 1: one Canopy commitment vs Merkle

The table reports the proof-size-optimal Canopy height over the full feasible grid with three-variable WHIR folds. Detailed rows and proof components are in `experiment1_canopy_vs_merkle/`.

| log2 vector | batch | best h | Merkle KiB | Canopy KiB | Canopy / Merkle |
|---:|---:|---:|---:|---:|---:|
| 22 | 128 | 19 | 60.48 | 70.59 | 1.167x |
| 22 | 256 | 8 | 112.94 | 119.52 | 1.058x |
| 22 | 512 | 5 | 209.87 | 180.46 | 0.860x |
| 22 | 1024 | 3 | 387.71 | 260.26 | 0.671x |
| 24 | 128 | 14 | 68.51 | 78.47 | 1.145x |
| 24 | 256 | 10 | 128.88 | 131.86 | 1.023x |
| 24 | 512 | 5 | 241.72 | 199.83 | 0.827x |
| 24 | 1024 | 4 | 451.73 | 284.39 | 0.630x |
| 26 | 128 | 16 | 76.43 | 86.04 | 1.126x |
| 26 | 256 | 10 | 144.95 | 145.45 | 1.003x |
| 26 | 512 | 6 | 273.73 | 215.47 | 0.787x |
| 26 | 1024 | 4 | 515.51 | 306.24 | 0.594x |
| 28 | 128 | 18 | 84.48 | 93.50 | 1.107x |
| 28 | 256 | 11 | 160.92 | 157.78 | 0.980x |
| 28 | 512 | 6 | 305.84 | 233.73 | 0.764x |
| 28 | 1024 | 4 | 579.51 | 330.07 | 0.570x |
| 30 | 128 | 20 | 92.45 | 100.61 | 1.088x |
| 30 | 256 | 13 | 176.90 | 170.75 | 0.965x |
| 30 | 512 | 7 | 337.79 | 250.39 | 0.741x |
| 30 | 1024 | 4 | 643.53 | 349.44 | 0.543x |

## Experiments 2 and 3: WHIR hybrids

Every initial and recursive WHIR fold uses three variables, so `f0=3`. The hybrids use no Canopy codeword shortening. Experiment 2 proves the claims separately and searches Canopy height `h` independently over its full feasible range. Experiment 3 needs identical RS codes and therefore fixes `h=f0+1=4`; both aligned claims encode `2^(n-3)` coefficients into `2^(n+1)` symbols at inverse rate `2^4`. It sends one pre-challenge cross-term sum, samples `alpha`, and proves the single degree-two identity for `f+alpha*g` under `L_f+alpha*L_g`. Its additional batching error is at most `2/|F|`, approximately `2^-255`. The complete fixed-fold height search is in `hybrid_parameter_grid.csv`.

## WHIR and hybrid proof sizes

| log2 message | scheme | f0 | h | KiB | vs Merkle WHIR | hashes KiB | fields KiB |
|---:|---|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 3 | - | 308.49 | 1.000x | 175.28 | 133.21 |
| 22 | canopy_separate | 3 | 9 | 291.19 | 0.944x | 145.36 | 145.84 |
| 22 | canopy_schwartz_zippel | 3 | 4 | 243.20 | 0.788x | 103.17 | 140.03 |
| 24 | merkle_whir | 3 | - | 353.18 | 1.000x | 215.75 | 137.43 |
| 24 | canopy_separate | 3 | 9 | 328.26 | 0.929x | 177.48 | 150.78 |
| 24 | canopy_schwartz_zippel | 3 | 4 | 270.53 | 0.766x | 126.78 | 143.75 |
| 26 | merkle_whir | 3 | - | 392.67 | 1.000x | 249.04 | 143.62 |
| 26 | canopy_separate | 3 | 10 | 359.98 | 0.917x | 200.42 | 159.56 |
| 26 | canopy_schwartz_zippel | 3 | 4 | 293.03 | 0.746x | 143.10 | 149.94 |
| 28 | merkle_whir | 3 | - | 436.46 | 1.000x | 291.61 | 144.84 |
| 28 | canopy_separate | 3 | 10 | 396.69 | 0.909x | 235.69 | 161.00 |
| 28 | canopy_schwartz_zippel | 3 | 4 | 319.63 | 0.732x | 168.72 | 150.91 |
| 30 | merkle_whir | 3 | - | 482.74 | 1.000x | 335.18 | 147.56 |
| 30 | canopy_separate | 3 | 11 | 434.95 | 0.901x | 268.89 | 166.06 |
| 30 | canopy_schwartz_zippel | 3 | 4 | 348.51 | 0.722x | 195.13 | 153.38 |

## Experiment 3 vs Merkle WHIR components

All entries below are KiB. Roots and authentication paths are hashes; the remaining columns are 256-bit field elements.

| log2 message | scheme | roots | authentication | OOD | sumcheck | batching | query answers | terminal coefficients | total |
|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 0.16 | 175.13 | 0.16 | 2.06 | 0.00 | 126.99 | 4.00 | 308.49 |
| 22 | canopy_schwartz_zippel | 0.16 | 103.01 | 0.19 | 2.06 | 0.03 | 133.75 | 4.00 | 243.20 |
| 24 | merkle_whir | 0.19 | 215.56 | 0.19 | 2.25 | 0.00 | 133.00 | 2.00 | 353.18 |
| 24 | canopy_schwartz_zippel | 0.19 | 126.59 | 0.22 | 2.25 | 0.03 | 139.25 | 2.00 | 270.53 |
| 26 | merkle_whir | 0.19 | 248.85 | 0.19 | 2.44 | 0.00 | 133.00 | 8.00 | 392.67 |
| 26 | canopy_schwartz_zippel | 0.19 | 142.91 | 0.22 | 2.44 | 0.03 | 139.25 | 8.00 | 293.03 |
| 28 | merkle_whir | 0.22 | 291.39 | 0.22 | 2.62 | 0.00 | 138.00 | 4.00 | 436.46 |
| 28 | canopy_schwartz_zippel | 0.22 | 168.50 | 0.25 | 2.62 | 0.03 | 144.00 | 4.00 | 319.63 |
| 30 | merkle_whir | 0.25 | 334.93 | 0.25 | 2.81 | 0.00 | 142.50 | 2.00 | 482.74 |
| 30 | canopy_schwartz_zippel | 0.25 | 194.88 | 0.28 | 2.81 | 0.03 | 148.25 | 2.00 | 348.51 |

## Oracle-query breakdown

The machine-readable table is `oracle_query_breakdown.csv`. Values below are the largest proof-size point (`n=30`).

| experiment | scheme | oracle | code | requested | unique | auth hashes | answer fields |
|---:|---|---|---|---:|---:|---:|---:|
| 2 | canopy_separate | Canopy claim WHIR initial oracle | [2^31, 2^20] | 24 | 24.00 | 541.82 | 192.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 1 | [2^30, 2^17] | 20 | 20.00 | 436.69 | 160.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 2 | [2^29, 2^14] | 18 | 18.00 | 376.80 | 144.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 3 | [2^28, 2^11] | 16 | 16.00 | 322.63 | 128.00 |
| 2 | canopy_separate | outer first oracle via Canopy | [2^31, 2^30] | 298 | 298.00 | 2383.77 | 2384.00 |
| 2 | canopy_separate | outer second-oracle WHIR initial oracle | [2^31, 2^27] | 67 | 67.00 | 1409.64 | 536.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 1 | [2^30, 2^24] | 44 | 44.00 | 909.31 | 352.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 2 | [2^29, 2^21] | 33 | 33.00 | 663.60 | 264.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 3 | [2^28, 2^18] | 26 | 26.00 | 505.72 | 208.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 4 | [2^27, 2^15] | 22 | 22.00 | 411.52 | 176.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 5 | [2^26, 2^12] | 19 | 19.00 | 341.47 | 152.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 6 | [2^25, 2^9] | 17 | 17.00 | 290.45 | 136.00 |
| 2 | merkle_whir | outer WHIR initial oracle | [2^31, 2^30] | 298 | 298.00 | 5631.69 | 2384.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 1 | [2^30, 2^27] | 90 | 90.00 | 1765.91 | 720.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 2 | [2^29, 2^24] | 53 | 53.00 | 1027.15 | 424.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 3 | [2^28, 2^21] | 38 | 38.00 | 718.36 | 304.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 4 | [2^27, 2^18] | 29 | 29.00 | 530.07 | 232.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 5 | [2^26, 2^15] | 24 | 24.00 | 421.23 | 192.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 6 | [2^25, 2^12] | 20 | 20.00 | 335.67 | 160.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 7 | [2^24, 2^9] | 18 | 18.00 | 287.52 | 144.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim initial oracle | [2^31, 2^27] | 67 | 67.00 | 2816.86 | 1072.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 1 | [2^30, 2^24] | 44 | 44.00 | 909.65 | 352.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 2 | [2^29, 2^21] | 33 | 33.00 | 663.70 | 264.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 3 | [2^28, 2^18] | 26 | 26.00 | 505.51 | 208.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 4 | [2^27, 2^15] | 22 | 22.00 | 412.26 | 176.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 5 | [2^26, 2^12] | 19 | 19.00 | 339.91 | 152.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 6 | [2^25, 2^9] | 17 | 17.00 | 290.43 | 136.00 |
| 3 | canopy_schwartz_zippel | outer first oracle via Canopy | [2^31, 2^30] | 298 | 298.00 | 298.00 | 2384.00 |
