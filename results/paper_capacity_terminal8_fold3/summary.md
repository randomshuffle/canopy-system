# Conjectural capacity-bound paper proof-size experiments

All experiments target 128-bit soundness over the 256-bit prime field, use BLAKE3-256, use no proof of work, use terminal polynomials of at most `2^8`, use three-variable folds everywhere, and use the capacity_conjecture RS proximity regime. Capacity-bound soundness relies on STIR Conjecture 5.6 and is not proved. Means use 100 deterministic query samples from seed `0x50415045525f5631`.

## Experiment 1: one Canopy commitment vs Merkle

The table reports the proof-size-optimal Canopy height over the full feasible grid with three-variable WHIR folds. Detailed rows and proof components are in `experiment1_canopy_vs_merkle/`.

| log2 vector | batch | best h | Merkle KiB | Canopy KiB | Canopy / Merkle |
|---:|---:|---:|---:|---:|---:|
| 22 | 128 | 10 | 60.48 | 63.63 | 1.052x |
| 22 | 256 | 6 | 112.94 | 96.00 | 0.850x |
| 22 | 512 | 3 | 209.87 | 135.83 | 0.647x |
| 22 | 1024 | 2 | 387.71 | 192.40 | 0.496x |
| 24 | 128 | 11 | 68.51 | 69.93 | 1.021x |
| 24 | 256 | 6 | 128.88 | 105.02 | 0.815x |
| 24 | 512 | 3 | 241.72 | 148.29 | 0.613x |
| 24 | 1024 | 2 | 451.73 | 205.75 | 0.455x |
| 26 | 128 | 11 | 76.43 | 76.65 | 1.003x |
| 26 | 256 | 7 | 144.95 | 113.93 | 0.786x |
| 26 | 512 | 4 | 273.73 | 159.31 | 0.582x |
| 26 | 1024 | 2 | 515.51 | 220.92 | 0.429x |
| 28 | 128 | 13 | 84.48 | 83.24 | 0.985x |
| 28 | 256 | 7 | 160.92 | 122.62 | 0.762x |
| 28 | 512 | 4 | 305.84 | 171.06 | 0.559x |
| 28 | 1024 | 3 | 579.51 | 236.66 | 0.408x |
| 30 | 128 | 11 | 92.45 | 90.00 | 0.974x |
| 30 | 256 | 8 | 176.90 | 132.44 | 0.749x |
| 30 | 512 | 5 | 337.79 | 182.99 | 0.542x |
| 30 | 1024 | 3 | 643.53 | 249.84 | 0.388x |

## Experiments 2 and 3: WHIR hybrids

Every initial and recursive WHIR fold uses three variables, so `f0=3`. The hybrids use no Canopy codeword shortening. Experiment 2 proves the claims separately and searches Canopy height `h` independently over its full feasible range. Experiment 3 needs identical RS codes and therefore fixes `h=f0+1=4`; both aligned claims encode `2^(n-3)` coefficients into `2^(n+1)` symbols at inverse rate `2^4`. It sends one pre-challenge cross-term sum, samples `alpha`, and proves the single degree-two identity for `f+alpha*g` under `L_f+alpha*L_g`. Its additional batching error is at most `2/|F|`, approximately `2^-255`. The complete fixed-fold height search is in `hybrid_parameter_grid.csv`.

## WHIR and hybrid proof sizes

| log2 message | scheme | f0 | h | KiB | vs Merkle WHIR | hashes KiB | fields KiB |
|---:|---|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 3 | - | 158.42 | 1.000x | 91.71 | 66.71 |
| 22 | canopy_separate | 3 | 10 | 150.74 | 0.952x | 77.49 | 73.25 |
| 22 | canopy_schwartz_zippel | 3 | 4 | 125.15 | 0.790x | 54.87 | 70.28 |
| 24 | merkle_whir | 3 | - | 179.56 | 1.000x | 111.63 | 67.94 |
| 24 | canopy_separate | 3 | 10 | 169.13 | 0.942x | 94.19 | 74.94 |
| 24 | canopy_schwartz_zippel | 3 | 4 | 138.17 | 0.769x | 66.92 | 71.25 |
| 26 | merkle_whir | 3 | - | 201.48 | 1.000x | 127.35 | 74.12 |
| 26 | canopy_separate | 3 | 11 | 187.51 | 0.931x | 105.04 | 82.47 |
| 26 | canopy_schwartz_zippel | 3 | 4 | 152.56 | 0.757x | 75.13 | 77.44 |
| 28 | merkle_whir | 3 | - | 221.31 | 1.000x | 148.46 | 72.84 |
| 28 | canopy_separate | 3 | 11 | 204.50 | 0.924x | 123.09 | 81.41 |
| 28 | canopy_schwartz_zippel | 3 | 4 | 164.70 | 0.744x | 88.54 | 76.16 |
| 30 | merkle_whir | 3 | - | 242.99 | 1.000x | 169.68 | 73.31 |
| 30 | canopy_separate | 3 | 13 | 223.25 | 0.919x | 142.63 | 80.62 |
| 30 | canopy_schwartz_zippel | 3 | 4 | 178.76 | 0.736x | 102.14 | 76.62 |

## Experiment 3 vs Merkle WHIR components

All entries below are KiB. Roots and authentication paths are hashes; the remaining columns are 256-bit field elements.

| log2 message | scheme | roots | authentication | OOD | sumcheck | batching | query answers | terminal coefficients | total |
|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 0.16 | 91.55 | 0.16 | 2.06 | 0.00 | 60.49 | 4.00 | 158.42 |
| 22 | canopy_schwartz_zippel | 0.16 | 54.72 | 0.19 | 2.06 | 0.03 | 64.00 | 4.00 | 125.15 |
| 24 | merkle_whir | 0.19 | 111.44 | 0.19 | 2.25 | 0.00 | 63.50 | 2.00 | 179.56 |
| 24 | canopy_schwartz_zippel | 0.19 | 66.74 | 0.22 | 2.25 | 0.03 | 66.75 | 2.00 | 138.17 |
| 26 | merkle_whir | 0.19 | 127.16 | 0.19 | 2.44 | 0.00 | 63.50 | 8.00 | 201.48 |
| 26 | canopy_schwartz_zippel | 0.19 | 74.94 | 0.22 | 2.44 | 0.03 | 66.75 | 8.00 | 152.56 |
| 28 | merkle_whir | 0.22 | 148.24 | 0.22 | 2.62 | 0.00 | 66.00 | 4.00 | 221.31 |
| 28 | canopy_schwartz_zippel | 0.22 | 88.32 | 0.25 | 2.62 | 0.03 | 69.25 | 4.00 | 164.70 |
| 30 | merkle_whir | 0.25 | 169.43 | 0.25 | 2.81 | 0.00 | 68.25 | 2.00 | 242.99 |
| 30 | canopy_schwartz_zippel | 0.25 | 101.89 | 0.28 | 2.81 | 0.03 | 71.50 | 2.00 | 178.76 |

## Oracle-query breakdown

The machine-readable table is `oracle_query_breakdown.csv`. Values below are the largest proof-size point (`n=30`).

| experiment | scheme | oracle | code | requested | unique | auth hashes | answer fields |
|---:|---|---|---|---:|---:|---:|---:|
| 2 | canopy_separate | Canopy claim WHIR initial oracle | [2^31, 2^18] | 10 | 10.00 | 238.85 | 80.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 1 | [2^30, 2^15] | 9 | 9.00 | 207.61 | 72.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 2 | [2^29, 2^12] | 8 | 8.00 | 178.59 | 64.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 3 | [2^28, 2^9] | 7 | 7.00 | 149.26 | 56.00 |
| 2 | canopy_separate | outer first oracle via Canopy | [2^31, 2^30] | 138 | 138.00 | 1379.83 | 1104.00 |
| 2 | canopy_separate | outer second-oracle WHIR initial oracle | [2^31, 2^27] | 33 | 33.00 | 729.18 | 264.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 1 | [2^30, 2^24] | 22 | 22.00 | 477.53 | 176.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 2 | [2^29, 2^21] | 17 | 17.00 | 357.72 | 136.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 3 | [2^28, 2^18] | 13 | 13.00 | 266.38 | 104.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 4 | [2^27, 2^15] | 11 | 11.00 | 217.06 | 88.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 5 | [2^26, 2^12] | 10 | 10.00 | 188.36 | 80.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 6 | [2^25, 2^9] | 9 | 9.00 | 162.74 | 72.00 |
| 2 | merkle_whir | outer WHIR initial oracle | [2^31, 2^30] | 138 | 138.00 | 2760.30 | 1104.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 1 | [2^30, 2^27] | 44 | 44.00 | 908.53 | 352.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 2 | [2^29, 2^24] | 26 | 26.00 | 531.46 | 208.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 3 | [2^28, 2^21] | 19 | 19.00 | 378.13 | 152.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 4 | [2^27, 2^18] | 15 | 15.00 | 288.00 | 120.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 5 | [2^26, 2^15] | 12 | 12.00 | 223.11 | 96.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 6 | [2^25, 2^12] | 10 | 10.00 | 178.29 | 80.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 7 | [2^24, 2^9] | 9 | 9.00 | 153.95 | 72.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim initial oracle | [2^31, 2^27] | 33 | 33.00 | 1453.30 | 528.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 1 | [2^30, 2^24] | 22 | 22.00 | 477.33 | 176.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 2 | [2^29, 2^21] | 17 | 17.00 | 357.94 | 136.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 3 | [2^28, 2^18] | 13 | 13.00 | 266.59 | 104.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 4 | [2^27, 2^15] | 11 | 11.00 | 216.27 | 88.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 5 | [2^26, 2^12] | 10 | 10.00 | 188.73 | 80.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 6 | [2^25, 2^9] | 9 | 9.00 | 162.23 | 72.00 |
| 3 | canopy_schwartz_zippel | outer first oracle via Canopy | [2^31, 2^30] | 138 | 138.00 | 138.00 | 1104.00 |
