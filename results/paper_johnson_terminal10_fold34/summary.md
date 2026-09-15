# Final Johnson-bound paper proof-size experiments

All experiments target 128-bit soundness over the 256-bit prime field, use BLAKE3-256, use no proof of work, search every ordered folding schedule over `{3,4}` that terminates at exactly `2^10` coefficients, and use the johnson RS proximity regime. Means use 100 deterministic query samples from seed `0x50415045525f5631`.

## Experiment 1: one Canopy commitment vs Merkle

The table reports the proof-size-optimal Canopy height and folding schedule over the full feasible grid. Detailed rows and proof components are in `experiment1_canopy_vs_merkle/`.

| log2 vector | batch | best h | folds | Merkle KiB | Canopy KiB | Canopy / Merkle |
|---:|---:|---:|---|---:|---:|---:|
| 22 | 128 | 9 | 3 | 60.48 | 92.44 | 1.528x |
| 22 | 256 | 9 | 3 | 112.94 | 132.14 | 1.170x |
| 22 | 512 | 6 | 3,3 | 209.87 | 194.54 | 0.927x |
| 22 | 1024 | 3 | 3,3,3 | 387.71 | 271.54 | 0.700x |
| 24 | 128 | 11 | 3 | 68.51 | 98.86 | 1.443x |
| 24 | 256 | 10 | 4 | 128.88 | 145.94 | 1.132x |
| 24 | 512 | 5 | 3,3,3 | 241.72 | 212.31 | 0.878x |
| 24 | 1024 | 4 | 3,4,3 | 451.73 | 298.85 | 0.662x |
| 26 | 128 | 13 | 3 | 76.43 | 105.29 | 1.378x |
| 26 | 256 | 10 | 3,3 | 144.95 | 159.21 | 1.098x |
| 26 | 512 | 6 | 4,3,3 | 273.73 | 229.23 | 0.837x |
| 26 | 1024 | 4 | 3,3,3,3 | 515.51 | 319.13 | 0.619x |
| 28 | 128 | 15 | 3 | 84.48 | 112.91 | 1.337x |
| 28 | 256 | 10 | 4,4 | 160.92 | 170.86 | 1.062x |
| 28 | 512 | 6 | 4,4,4 | 305.84 | 245.13 | 0.801x |
| 28 | 1024 | 4 | 3,4,4,3 | 579.51 | 340.07 | 0.587x |
| 30 | 128 | 17 | 3 | 92.45 | 120.22 | 1.300x |
| 30 | 256 | 12 | 4,4 | 176.90 | 183.23 | 1.036x |
| 30 | 512 | 6 | 4,4,3,3 | 337.79 | 262.52 | 0.777x |
| 30 | 1024 | 4 | 4,4,4,4 | 643.53 | 358.64 | 0.557x |

## Experiments 2 and 3: WHIR hybrids

Every folding step is independently selected from `{3,4}`, subject to ending at exactly `2^10` coefficients. The hybrids use no Canopy codeword shortening. Experiment 2 independently searches Canopy height, the outer WHIR schedule, and the Canopy-claim schedule. Experiment 3 needs identical RS codes, fixes `h=f0+1`, and uses the outer schedule after `f0` for its combined aligned claim. It sends one pre-challenge cross-term sum, samples `alpha`, and proves the single degree-two identity for `f+alpha*g` under `L_f+alpha*L_g`. Its additional batching error is at most `2/|F|`, approximately `2^-255`. The complete search is in `hybrid_parameter_grid.csv`.

## WHIR and hybrid proof sizes

| log2 message | scheme | outer folds | h | Canopy folds | KiB | vs Merkle WHIR | hashes KiB | fields KiB |
|---:|---|---|---:|---|---:|---:|---:|---:|
| 22 | merkle_whir | 3,3,3,3 | - | - | 319.82 | 1.000x | 165.90 | 153.93 |
| 22 | canopy_separate | 3,3,3,3 | 9 | 4 | 317.45 | 0.993x | 124.93 | 192.52 |
| 22 | canopy_schwartz_zippel | 3,3,3,3 | 4 | 3,3,3 | 255.37 | 0.798x | 93.88 | 161.50 |
| 24 | merkle_whir | 3,3,4,4 | - | - | 365.76 | 1.000x | 191.39 | 174.37 |
| 24 | canopy_separate | 3,3,4,4 | 9 | 3,3 | 353.72 | 0.967x | 144.79 | 208.93 |
| 24 | canopy_schwartz_zippel | 3,3,4,4 | 4 | 3,4,4 | 282.18 | 0.772x | 103.25 | 178.93 |
| 26 | merkle_whir | 3,3,4,3,3 | - | - | 406.68 | 1.000x | 233.84 | 172.84 |
| 26 | canopy_separate | 3,3,4,3,3 | 10 | 4,3 | 387.03 | 0.952x | 174.28 | 212.75 |
| 26 | canopy_schwartz_zippel | 3,3,4,3,3 | 4 | 3,4,3,3 | 306.65 | 0.754x | 128.75 | 177.91 |
| 28 | merkle_whir | 3,3,4,4,4 | - | - | 449.21 | 1.000x | 262.43 | 186.78 |
| 28 | canopy_separate | 3,4,4,4,3 | 11 | 4,4 | 419.97 | 0.935x | 184.94 | 235.03 |
| 28 | canopy_schwartz_zippel | 3,3,4,4,4 | 4 | 3,4,4,4 | 331.06 | 0.737x | 140.71 | 190.34 |
| 30 | merkle_whir | 3,4,4,3,3,3 | - | - | 491.53 | 1.000x | 296.78 | 194.75 |
| 30 | canopy_separate | 3,4,4,3,3,3 | 10 | 4,4,3 | 454.12 | 0.924x | 214.06 | 240.06 |
| 30 | canopy_schwartz_zippel | 3,3,4,4,3,3 | 4 | 3,4,4,3,3 | 356.70 | 0.726x | 166.89 | 189.81 |

## Experiment 3 vs Merkle WHIR components

All entries below are KiB. Roots and authentication paths are hashes; the remaining columns are 256-bit field elements.

| log2 message | scheme | roots | authentication | OOD | sumcheck | batching | query answers | terminal coefficients | total |
|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 0.12 | 165.77 | 0.12 | 2.06 | 0.00 | 119.74 | 32.00 | 319.82 |
| 22 | canopy_schwartz_zippel | 0.12 | 93.75 | 0.16 | 2.06 | 0.03 | 127.25 | 32.00 | 255.37 |
| 24 | merkle_whir | 0.12 | 191.26 | 0.12 | 2.25 | 0.00 | 140.00 | 32.00 | 365.76 |
| 24 | canopy_schwartz_zippel | 0.12 | 103.13 | 0.16 | 2.25 | 0.03 | 144.50 | 32.00 | 282.18 |
| 26 | merkle_whir | 0.16 | 233.68 | 0.16 | 2.44 | 0.00 | 138.25 | 32.00 | 406.68 |
| 26 | canopy_schwartz_zippel | 0.16 | 128.59 | 0.19 | 2.44 | 0.03 | 143.25 | 32.00 | 306.65 |
| 28 | merkle_whir | 0.16 | 262.27 | 0.16 | 2.62 | 0.00 | 152.00 | 32.00 | 449.21 |
| 28 | canopy_schwartz_zippel | 0.16 | 140.56 | 0.19 | 2.62 | 0.03 | 155.50 | 32.00 | 331.06 |
| 30 | merkle_whir | 0.19 | 296.59 | 0.19 | 2.81 | 0.00 | 159.75 | 32.00 | 491.53 |
| 30 | canopy_schwartz_zippel | 0.19 | 166.70 | 0.22 | 2.81 | 0.03 | 154.75 | 32.00 | 356.70 |

## Oracle-query breakdown

The machine-readable table is `oracle_query_breakdown.csv`. Values below are the largest proof-size point (`n=30`).

| experiment | scheme | oracle | code | requested | unique | auth hashes | answer fields |
|---:|---|---|---|---:|---:|---:|---:|
| 2 | canopy_separate | Canopy claim WHIR initial oracle | [2^31, 2^21] | 26 | 26.00 | 557.44 | 416.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 1 | [2^30, 2^17] | 20 | 20.00 | 417.06 | 320.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 2 | [2^29, 2^13] | 17 | 17.00 | 357.47 | 136.00 |
| 2 | canopy_separate | outer first oracle via Canopy | [2^31, 2^30] | 298 | 298.00 | 2085.87 | 2384.00 |
| 2 | canopy_separate | outer second-oracle WHIR initial oracle | [2^31, 2^27] | 67 | 67.00 | 1344.98 | 1072.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 1 | [2^30, 2^23] | 38 | 38.00 | 755.87 | 608.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 2 | [2^29, 2^19] | 26 | 26.00 | 530.94 | 208.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 3 | [2^28, 2^16] | 22 | 22.00 | 432.78 | 176.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 4 | [2^27, 2^13] | 19 | 19.00 | 359.50 | 152.00 |
| 2 | merkle_whir | outer WHIR initial oracle | [2^31, 2^30] | 298 | 298.00 | 5631.69 | 2384.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 1 | [2^30, 2^27] | 90 | 90.00 | 1675.63 | 1440.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 2 | [2^29, 2^23] | 44 | 44.00 | 821.46 | 704.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 3 | [2^28, 2^19] | 29 | 29.00 | 559.33 | 232.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 4 | [2^27, 2^16] | 24 | 24.00 | 445.83 | 192.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 5 | [2^26, 2^13] | 20 | 20.00 | 357.00 | 160.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim initial oracle | [2^31, 2^27] | 67 | 67.00 | 2816.86 | 1072.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 1 | [2^30, 2^24] | 44 | 44.00 | 867.22 | 704.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 2 | [2^29, 2^20] | 29 | 29.00 | 559.16 | 464.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 3 | [2^28, 2^16] | 22 | 22.00 | 432.94 | 176.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 4 | [2^27, 2^13] | 19 | 19.00 | 360.26 | 152.00 |
| 3 | canopy_schwartz_zippel | outer first oracle via Canopy | [2^31, 2^30] | 298 | 298.00 | 298.00 | 2384.00 |
