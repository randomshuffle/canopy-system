# Final Johnson-bound paper proof-size experiments

All experiments target 128-bit soundness over the 256-bit prime field, use BLAKE3-256, use no proof of work, use terminal polynomials of at most `2^8`, and use the johnson RS proximity regime. Means use 100 deterministic query samples from seed `0x50415045525f5631`.

## Experiment 1: one Canopy commitment vs Merkle

The table reports the proof-size-optimal Canopy height over the full feasible grid. Detailed rows and proof components are in `experiment1_canopy_vs_merkle/`.

| log2 vector | batch | best h | Merkle KiB | Canopy KiB | Canopy / Merkle |
|---:|---:|---:|---:|---:|---:|
| 22 | 128 | 12 | 60.48 | 73.97 | 1.223x |
| 22 | 256 | 10 | 112.94 | 119.55 | 1.059x |
| 22 | 512 | 6 | 209.87 | 184.20 | 0.878x |
| 22 | 1024 | 3 | 387.71 | 269.26 | 0.694x |
| 24 | 128 | 13 | 68.51 | 80.01 | 1.168x |
| 24 | 256 | 9 | 128.88 | 133.31 | 1.034x |
| 24 | 512 | 5 | 241.72 | 202.05 | 0.836x |
| 24 | 1024 | 4 | 451.73 | 288.17 | 0.638x |
| 26 | 128 | 15 | 76.43 | 87.20 | 1.141x |
| 26 | 256 | 10 | 144.95 | 143.73 | 0.992x |
| 26 | 512 | 6 | 273.73 | 214.99 | 0.785x |
| 26 | 1024 | 4 | 515.51 | 309.34 | 0.600x |
| 28 | 128 | 17 | 84.48 | 94.29 | 1.116x |
| 28 | 256 | 12 | 160.92 | 156.43 | 0.972x |
| 28 | 512 | 6 | 305.84 | 233.15 | 0.762x |
| 28 | 1024 | 4 | 579.51 | 324.99 | 0.561x |
| 30 | 128 | 19 | 92.45 | 101.05 | 1.093x |
| 30 | 256 | 11 | 176.90 | 168.25 | 0.951x |
| 30 | 512 | 7 | 337.79 | 245.62 | 0.727x |
| 30 | 1024 | 4 | 643.53 | 346.21 | 0.538x |

## Experiments 2 and 3: WHIR hybrids

For every scheme and message length, the first WHIR fold `f0` is grid-searched independently; later folds use four variables. The hybrids use no Canopy codeword shortening and set `h=f0+1`. Thus Canopy's output and outer WHIR's second oracle both encode `2^(n-f0)` coefficients into `2^(n+1)` symbols at inverse rate `2^(f0+1)`. Experiment 2 proves the aligned claims separately. Experiment 3 sends one pre-challenge cross-term sum, samples `alpha`, and proves the single degree-two identity for `f+alpha*g` under `L_f+alpha*L_g`; after the two initial-root openings it carries only one recursive oracle and one algebraic claim. Its additional batching error is at most `2/|F|`, approximately `2^-255`. The complete search is in `hybrid_initial_folding_grid.csv`.

## WHIR and hybrid proof sizes

| log2 message | scheme | f0 | h | KiB | vs Merkle WHIR | hashes KiB | fields KiB |
|---:|---|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 3 | - | 317.52 | 1.000x | 155.35 | 162.17 |
| 22 | canopy_separate | 3 | 4 | 324.33 | 1.021x | 106.79 | 217.53 |
| 22 | canopy_schwartz_zippel | 3 | 4 | 262.96 | 0.828x | 83.22 | 179.74 |
| 24 | merkle_whir | 3 | - | 362.57 | 1.000x | 192.18 | 170.39 |
| 24 | canopy_separate | 3 | 4 | 371.25 | 1.024x | 139.25 | 232.00 |
| 24 | canopy_schwartz_zippel | 3 | 4 | 290.68 | 0.802x | 103.72 | 186.96 |
| 26 | merkle_whir | 3 | - | 396.00 | 1.000x | 222.41 | 173.59 |
| 26 | canopy_separate | 3 | 4 | 396.53 | 1.001x | 158.16 | 238.38 |
| 26 | canopy_schwartz_zippel | 3 | 4 | 307.44 | 0.776x | 117.28 | 190.16 |
| 28 | merkle_whir | 3 | - | 440.97 | 1.000x | 261.16 | 179.81 |
| 28 | canopy_separate | 3 | 4 | 443.90 | 1.007x | 194.09 | 249.81 |
| 28 | canopy_schwartz_zippel | 3 | 4 | 335.36 | 0.761x | 139.49 | 195.88 |
| 30 | merkle_whir | 3 | - | 475.47 | 1.000x | 292.47 | 183.00 |
| 30 | canopy_separate | 3 | 4 | 471.44 | 0.992x | 215.25 | 256.19 |
| 30 | canopy_schwartz_zippel | 3 | 4 | 353.36 | 0.743x | 154.30 | 199.06 |

## Experiment 3 vs Merkle WHIR components

All entries below are KiB. Roots and authentication paths are hashes; the remaining columns are 256-bit field elements.

| log2 message | scheme | roots | authentication | OOD | sumcheck | batching | query answers | terminal coefficients | total |
|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 0.12 | 155.22 | 0.12 | 2.06 | 0.00 | 155.98 | 4.00 | 317.52 |
| 22 | canopy_schwartz_zippel | 0.12 | 83.10 | 0.16 | 2.06 | 0.03 | 173.49 | 4.00 | 262.96 |
| 24 | merkle_whir | 0.16 | 192.03 | 0.16 | 2.25 | 0.00 | 166.98 | 1.00 | 362.57 |
| 24 | canopy_schwartz_zippel | 0.16 | 103.56 | 0.19 | 2.25 | 0.03 | 183.50 | 1.00 | 290.68 |
| 26 | merkle_whir | 0.16 | 222.26 | 0.16 | 2.44 | 0.00 | 167.00 | 4.00 | 396.00 |
| 26 | canopy_schwartz_zippel | 0.16 | 117.12 | 0.19 | 2.44 | 0.03 | 183.50 | 4.00 | 307.44 |
| 28 | merkle_whir | 0.19 | 260.97 | 0.19 | 2.62 | 0.00 | 176.00 | 1.00 | 440.97 |
| 28 | canopy_schwartz_zippel | 0.19 | 139.30 | 0.22 | 2.62 | 0.03 | 192.00 | 1.00 | 335.36 |
| 30 | merkle_whir | 0.19 | 292.29 | 0.19 | 2.81 | 0.00 | 176.00 | 4.00 | 475.47 |
| 30 | canopy_schwartz_zippel | 0.19 | 154.11 | 0.22 | 2.81 | 0.03 | 192.00 | 4.00 | 353.36 |

## Oracle-query breakdown

The machine-readable table is `oracle_query_breakdown.csv`. Values below are the largest proof-size point (`n=30`).

| experiment | scheme | oracle | code | requested | unique | auth hashes | answer fields |
|---:|---|---|---|---:|---:|---:|---:|
| 2 | canopy_separate | Canopy claim WHIR initial oracle | [2^31, 2^27] | 67 | 67.00 | 1344.14 | 1072.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 1 | [2^30, 2^23] | 38 | 38.00 | 756.26 | 608.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 2 | [2^29, 2^19] | 26 | 26.00 | 506.05 | 416.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 3 | [2^28, 2^15] | 20 | 20.00 | 376.50 | 320.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 4 | [2^27, 2^11] | 17 | 17.00 | 307.78 | 272.00 |
| 2 | canopy_separate | outer first oracle via Canopy | [2^31, 2^30] | 298 | 298.00 | 298.00 | 2384.00 |
| 2 | canopy_separate | outer second-oracle WHIR initial oracle | [2^31, 2^27] | 67 | 67.00 | 1344.98 | 1072.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 1 | [2^30, 2^23] | 38 | 38.00 | 755.87 | 608.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 2 | [2^29, 2^19] | 26 | 26.00 | 504.76 | 416.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 3 | [2^28, 2^15] | 20 | 20.00 | 376.47 | 320.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 4 | [2^27, 2^11] | 17 | 17.00 | 307.17 | 272.00 |
| 2 | merkle_whir | outer WHIR initial oracle | [2^31, 2^30] | 298 | 298.00 | 5631.69 | 2384.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 1 | [2^30, 2^27] | 90 | 90.00 | 1675.63 | 1440.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 2 | [2^29, 2^23] | 44 | 44.00 | 821.46 | 704.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 3 | [2^28, 2^19] | 29 | 29.00 | 529.98 | 464.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 4 | [2^27, 2^15] | 22 | 22.00 | 388.33 | 352.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 5 | [2^26, 2^11] | 18 | 18.00 | 306.07 | 288.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim initial oracle | [2^31, 2^27] | 67 | 67.00 | 2686.60 | 2144.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 1 | [2^30, 2^23] | 38 | 38.00 | 756.58 | 608.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 2 | [2^29, 2^19] | 26 | 26.00 | 505.66 | 416.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 3 | [2^28, 2^15] | 20 | 20.00 | 376.89 | 320.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 4 | [2^27, 2^11] | 17 | 17.00 | 307.73 | 272.00 |
| 3 | canopy_schwartz_zippel | outer first oracle via Canopy | [2^31, 2^30] | 298 | 298.00 | 298.00 | 2384.00 |
