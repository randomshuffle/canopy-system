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

## Experiments 2--4: WHIR hybrids

For every scheme and message length, the first WHIR fold `f0` is grid-searched independently; later folds use four variables. The hybrid sets `h=f0+1` with a one-bit Canopy codeword shortening, so Canopy's output and outer WHIR's second oracle both encode `2^(n-f0)` coefficients into `2^n` symbols at inverse rate `2^f0`. Experiment 2 proves the aligned claims separately. Experiment 3 shares recursive Merkle authentication but retains both field elements and both algebraic claims. Experiment 4 sends one pre-challenge cross-term sum, samples `alpha`, and proves the single degree-two identity for `f+alpha*g` under `L_f+alpha*L_g`; after the two initial-root openings it carries only one recursive oracle and one algebraic claim. Its additional batching error is at most `2/|F|`, approximately `2^-255`. The complete search is in `hybrid_initial_folding_grid.csv`.

## WHIR and hybrid proof sizes

| log2 message | scheme | f0 | h | KiB | vs Merkle WHIR | hashes KiB | fields KiB |
|---:|---|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 3 | - | 317.52 | 1.000x | 155.35 | 162.17 |
| 22 | canopy_separate | 3 | 4 | 366.72 | 1.155x | 117.22 | 249.50 |
| 22 | canopy_batched | 3 | 4 | 342.65 | 1.079x | 93.12 | 249.53 |
| 22 | canopy_schwartz_zippel | 3 | 4 | 300.51 | 0.946x | 93.26 | 207.25 |
| 24 | merkle_whir | 3 | - | 362.57 | 1.000x | 192.18 | 170.39 |
| 24 | canopy_separate | 4 | 5 | 415.37 | 1.146x | 115.03 | 300.34 |
| 24 | canopy_batched | 3 | 4 | 382.98 | 1.056x | 116.99 | 266.00 |
| 24 | canopy_schwartz_zippel | 3 | 4 | 332.60 | 0.917x | 117.15 | 215.45 |
| 26 | merkle_whir | 3 | - | 396.00 | 1.000x | 222.41 | 173.59 |
| 26 | canopy_separate | 3 | 4 | 449.16 | 1.134x | 176.79 | 272.38 |
| 26 | canopy_batched | 3 | 4 | 406.79 | 1.027x | 134.41 | 272.38 |
| 26 | canopy_schwartz_zippel | 3 | 4 | 352.97 | 0.891x | 134.31 | 218.66 |
| 28 | merkle_whir | 3 | - | 440.97 | 1.000x | 261.16 | 179.81 |
| 28 | canopy_separate | 4 | 5 | 488.70 | 1.108x | 167.54 | 321.16 |
| 28 | canopy_batched | 3 | 4 | 444.63 | 1.008x | 159.82 | 284.81 |
| 28 | canopy_schwartz_zippel | 3 | 4 | 384.79 | 0.873x | 159.92 | 224.88 |
| 30 | merkle_whir | 3 | - | 475.47 | 1.000x | 292.47 | 183.00 |
| 30 | canopy_separate | 4 | 5 | 531.26 | 1.117x | 204.66 | 326.59 |
| 30 | canopy_batched | 3 | 4 | 469.47 | 0.987x | 178.29 | 291.19 |
| 30 | canopy_schwartz_zippel | 3 | 4 | 406.13 | 0.854x | 178.06 | 228.06 |

## Experiments 3 and 4 vs Merkle WHIR components

All entries below are KiB. Roots and authentication paths are hashes; the remaining columns are 256-bit field elements.

| log2 message | scheme | roots | authentication | OOD | sumcheck | batching | query answers | terminal coefficients | total |
|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 0.12 | 155.22 | 0.12 | 2.06 | 0.00 | 155.98 | 4.00 | 317.52 |
| 22 | canopy_batched | 0.12 | 92.99 | 0.22 | 3.84 | 0.00 | 237.47 | 8.00 | 342.65 |
| 22 | canopy_schwartz_zippel | 0.12 | 93.14 | 0.16 | 2.06 | 0.03 | 201.00 | 4.00 | 300.51 |
| 24 | merkle_whir | 0.16 | 192.03 | 0.16 | 2.25 | 0.00 | 166.98 | 1.00 | 362.57 |
| 24 | canopy_batched | 0.16 | 116.83 | 0.28 | 4.22 | 0.00 | 259.50 | 2.00 | 382.98 |
| 24 | canopy_schwartz_zippel | 0.16 | 116.99 | 0.19 | 2.25 | 0.03 | 211.98 | 1.00 | 332.60 |
| 26 | merkle_whir | 0.16 | 222.26 | 0.16 | 2.44 | 0.00 | 167.00 | 4.00 | 396.00 |
| 26 | canopy_batched | 0.16 | 134.26 | 0.28 | 4.59 | 0.00 | 259.50 | 8.00 | 406.79 |
| 26 | canopy_schwartz_zippel | 0.16 | 134.16 | 0.19 | 2.44 | 0.03 | 212.00 | 4.00 | 352.97 |
| 28 | merkle_whir | 0.19 | 260.97 | 0.19 | 2.62 | 0.00 | 176.00 | 1.00 | 440.97 |
| 28 | canopy_batched | 0.19 | 159.63 | 0.34 | 4.97 | 0.00 | 277.50 | 2.00 | 444.63 |
| 28 | canopy_schwartz_zippel | 0.19 | 159.73 | 0.22 | 2.62 | 0.03 | 221.00 | 1.00 | 384.79 |
| 30 | merkle_whir | 0.19 | 292.29 | 0.19 | 2.81 | 0.00 | 176.00 | 4.00 | 475.47 |
| 30 | canopy_batched | 0.19 | 178.10 | 0.34 | 5.34 | 0.00 | 277.50 | 8.00 | 469.47 |
| 30 | canopy_schwartz_zippel | 0.19 | 177.88 | 0.22 | 2.81 | 0.03 | 221.00 | 4.00 | 406.13 |

## Oracle-query breakdown

The machine-readable table is `oracle_query_breakdown.csv`. Values below are the largest proof-size point (`n=30`).

| experiment | scheme | oracle | code | requested | unique | auth hashes | answer fields |
|---:|---|---|---|---:|---:|---:|---:|
| 2 | canopy_separate | Canopy claim WHIR initial oracle | [2^30, 2^26] | 67 | 67.00 | 1275.89 | 1072.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 1 | [2^29, 2^22] | 38 | 38.00 | 718.35 | 608.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 2 | [2^28, 2^18] | 26 | 26.00 | 480.67 | 416.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 3 | [2^27, 2^14] | 20 | 20.00 | 356.71 | 320.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 4 | [2^26, 2^10] | 17 | 17.00 | 290.49 | 272.00 |
| 2 | canopy_separate | outer first oracle via Canopy | [2^31, 2^30] | 298 | 298.00 | 298.00 | 4768.00 |
| 2 | canopy_separate | outer second-oracle WHIR initial oracle | [2^30, 2^26] | 67 | 67.00 | 1276.56 | 1072.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 1 | [2^29, 2^22] | 38 | 38.00 | 718.35 | 608.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 2 | [2^28, 2^18] | 26 | 26.00 | 479.52 | 416.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 3 | [2^27, 2^14] | 20 | 20.00 | 355.38 | 320.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 4 | [2^26, 2^10] | 17 | 17.00 | 289.33 | 272.00 |
| 2 | merkle_whir | outer WHIR initial oracle | [2^31, 2^30] | 298 | 298.00 | 5631.69 | 2384.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 1 | [2^30, 2^27] | 90 | 90.00 | 1675.63 | 1440.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 2 | [2^29, 2^23] | 44 | 44.00 | 821.46 | 704.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 3 | [2^28, 2^19] | 29 | 29.00 | 529.98 | 464.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 4 | [2^27, 2^15] | 22 | 22.00 | 388.33 | 352.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 5 | [2^26, 2^11] | 18 | 18.00 | 306.07 | 288.00 |
| 3 | canopy_batched | batched aligned claims initial oracle | [2^30, 2^27] | 90 | 90.00 | 3354.58 | 2880.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 1 | [2^29, 2^23] | 44 | 44.00 | 820.60 | 1408.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 2 | [2^28, 2^19] | 29 | 29.00 | 531.05 | 928.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 3 | [2^27, 2^15] | 22 | 22.00 | 389.48 | 704.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 4 | [2^26, 2^11] | 18 | 18.00 | 305.47 | 576.00 |
| 3 | canopy_batched | outer first oracle via Canopy | [2^31, 2^30] | 298 | 298.00 | 298.00 | 2384.00 |
| 4 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim initial oracle | [2^30, 2^27] | 90 | 90.00 | 3349.36 | 2880.00 |
| 4 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 1 | [2^29, 2^23] | 44 | 44.00 | 820.44 | 704.00 |
| 4 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 2 | [2^28, 2^19] | 29 | 29.00 | 528.91 | 464.00 |
| 4 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 3 | [2^27, 2^15] | 22 | 22.00 | 389.95 | 352.00 |
| 4 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 4 | [2^26, 2^11] | 18 | 18.00 | 305.39 | 288.00 |
| 4 | canopy_schwartz_zippel | outer first oracle via Canopy | [2^31, 2^30] | 298 | 298.00 | 298.00 | 2384.00 |
