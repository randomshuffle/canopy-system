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

## Experiments 2 and 3: WHIR hybrids

For every scheme and message length, the first WHIR fold `f0` is grid-searched independently; later folds use four variables. The hybrids use no Canopy codeword shortening and set `h=f0+1`. Thus Canopy's output and outer WHIR's second oracle both encode `2^(n-f0)` coefficients into `2^(n+1)` symbols at inverse rate `2^(f0+1)`. Experiment 2 proves the aligned claims separately. Experiment 3 sends one pre-challenge cross-term sum, samples `alpha`, and proves the single degree-two identity for `f+alpha*g` under `L_f+alpha*L_g`; after the two initial-root openings it carries only one recursive oracle and one algebraic claim. Its additional batching error is at most `2/|F|`, approximately `2^-255`. The complete search is in `hybrid_initial_folding_grid.csv`.

## WHIR and hybrid proof sizes

| log2 message | scheme | f0 | h | KiB | vs Merkle WHIR | hashes KiB | fields KiB |
|---:|---|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 3 | - | 162.34 | 1.000x | 81.16 | 81.19 |
| 22 | canopy_separate | 3 | 4 | 168.53 | 1.038x | 56.98 | 111.55 |
| 22 | canopy_schwartz_zippel | 3 | 4 | 133.76 | 0.824x | 44.02 | 89.74 |
| 24 | merkle_whir | 3 | - | 183.19 | 1.000x | 99.28 | 83.90 |
| 24 | canopy_separate | 3 | 4 | 189.81 | 1.036x | 73.82 | 116.00 |
| 24 | canopy_schwartz_zippel | 3 | 4 | 146.60 | 0.800x | 54.63 | 91.97 |
| 26 | merkle_whir | 3 | - | 200.63 | 1.000x | 113.54 | 87.09 |
| 26 | canopy_separate | 3 | 4 | 205.61 | 1.025x | 83.23 | 122.38 |
| 26 | canopy_schwartz_zippel | 3 | 4 | 156.54 | 0.780x | 61.38 | 95.16 |
| 28 | merkle_whir | 3 | - | 221.45 | 1.000x | 132.63 | 88.81 |
| 28 | canopy_separate | 3 | 4 | 227.96 | 1.029x | 102.15 | 125.81 |
| 28 | canopy_schwartz_zippel | 3 | 4 | 169.82 | 0.767x | 72.94 | 96.88 |
| 30 | merkle_whir | 3 | - | 239.49 | 1.000x | 147.49 | 92.00 |
| 30 | canopy_separate | 3 | 4 | 244.90 | 1.023x | 112.72 | 132.19 |
| 30 | canopy_schwartz_zippel | 3 | 4 | 180.37 | 0.753x | 80.30 | 100.06 |

## Experiment 3 vs Merkle WHIR components

All entries below are KiB. Roots and authentication paths are hashes; the remaining columns are 256-bit field elements.

| log2 message | scheme | roots | authentication | OOD | sumcheck | batching | query answers | terminal coefficients | total |
|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 0.12 | 81.03 | 0.12 | 2.06 | 0.00 | 75.00 | 4.00 | 162.34 |
| 22 | canopy_schwartz_zippel | 0.12 | 43.90 | 0.16 | 2.06 | 0.03 | 83.49 | 4.00 | 133.76 |
| 24 | merkle_whir | 0.16 | 99.13 | 0.16 | 2.25 | 0.00 | 80.50 | 1.00 | 183.19 |
| 24 | canopy_schwartz_zippel | 0.16 | 54.48 | 0.19 | 2.25 | 0.03 | 88.50 | 1.00 | 146.60 |
| 26 | merkle_whir | 0.16 | 113.39 | 0.16 | 2.44 | 0.00 | 80.50 | 4.00 | 200.63 |
| 26 | canopy_schwartz_zippel | 0.16 | 61.23 | 0.19 | 2.44 | 0.03 | 88.50 | 4.00 | 156.54 |
| 28 | merkle_whir | 0.19 | 132.45 | 0.19 | 2.62 | 0.00 | 85.00 | 1.00 | 221.45 |
| 28 | canopy_schwartz_zippel | 0.19 | 72.76 | 0.22 | 2.62 | 0.03 | 93.00 | 1.00 | 169.82 |
| 30 | merkle_whir | 0.19 | 147.30 | 0.19 | 2.81 | 0.00 | 85.00 | 4.00 | 239.49 |
| 30 | canopy_schwartz_zippel | 0.19 | 80.12 | 0.22 | 2.81 | 0.03 | 93.00 | 4.00 | 180.37 |

## Oracle-query breakdown

The machine-readable table is `oracle_query_breakdown.csv`. Values below are the largest proof-size point (`n=30`).

| experiment | scheme | oracle | code | requested | unique | auth hashes | answer fields |
|---:|---|---|---|---:|---:|---:|---:|
| 2 | canopy_separate | Canopy claim WHIR initial oracle | [2^31, 2^27] | 33 | 33.00 | 697.05 | 528.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 1 | [2^30, 2^23] | 19 | 19.00 | 397.04 | 304.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 2 | [2^29, 2^19] | 13 | 13.00 | 266.36 | 208.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 3 | [2^28, 2^15] | 10 | 10.00 | 199.11 | 160.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 4 | [2^27, 2^11] | 9 | 9.00 | 170.80 | 144.00 |
| 2 | canopy_separate | outer first oracle via Canopy | [2^31, 2^30] | 138 | 138.00 | 138.00 | 1104.00 |
| 2 | canopy_separate | outer second-oracle WHIR initial oracle | [2^31, 2^27] | 33 | 33.00 | 695.36 | 528.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 1 | [2^30, 2^23] | 19 | 19.00 | 397.26 | 304.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 2 | [2^29, 2^19] | 13 | 13.00 | 266.69 | 208.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 3 | [2^28, 2^15] | 10 | 10.00 | 198.32 | 160.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 4 | [2^27, 2^11] | 9 | 9.00 | 170.96 | 144.00 |
| 2 | merkle_whir | outer WHIR initial oracle | [2^31, 2^30] | 138 | 138.00 | 2760.30 | 1104.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 1 | [2^30, 2^27] | 44 | 44.00 | 864.93 | 704.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 2 | [2^29, 2^23] | 22 | 22.00 | 432.20 | 352.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 3 | [2^28, 2^19] | 15 | 15.00 | 289.17 | 240.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 4 | [2^27, 2^15] | 11 | 11.00 | 205.67 | 176.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 5 | [2^26, 2^11] | 9 | 9.00 | 161.33 | 144.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim initial oracle | [2^31, 2^27] | 33 | 33.00 | 1392.08 | 1056.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 1 | [2^30, 2^23] | 19 | 19.00 | 396.61 | 304.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 2 | [2^29, 2^19] | 13 | 13.00 | 266.27 | 208.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 3 | [2^28, 2^15] | 10 | 10.00 | 199.14 | 160.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 4 | [2^27, 2^11] | 9 | 9.00 | 171.65 | 144.00 |
| 3 | canopy_schwartz_zippel | outer first oracle via Canopy | [2^31, 2^30] | 138 | 138.00 | 138.00 | 1104.00 |
