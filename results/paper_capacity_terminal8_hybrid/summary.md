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

For experiments 2 and 3, outer WHIR starts at rate 1/2 and folds four variables. Canopy uses `h=5` and a one-bit codeword shortening: its output and outer WHIR's second oracle both encode `2^(n-4)` coefficients into `2^n` symbols (rate 1/16). Experiment 2 proves those aligned claims separately; experiment 3 batches their two initial roots and shares every recursive oracle.

## WHIR and hybrid proof sizes

| log2 message | scheme | KiB | vs Merkle WHIR | hashes KiB | fields KiB |
|---:|---|---:|---:|---:|---:|
| 22 | merkle_whir | 177.45 | 1.000x | 71.78 | 105.67 |
| 22 | canopy_separate | 194.87 | 1.098x | 52.91 | 141.95 |
| 22 | canopy_batched | 182.94 | 1.031x | 41.00 | 141.94 |
| 24 | merkle_whir | 196.35 | 1.000x | 84.48 | 111.88 |
| 24 | canopy_separate | 215.39 | 1.097x | 61.05 | 154.34 |
| 24 | canopy_batched | 201.57 | 1.027x | 47.24 | 154.33 |
| 26 | merkle_whir | 212.88 | 1.000x | 101.78 | 111.09 |
| 26 | canopy_separate | 231.30 | 1.087x | 78.52 | 152.78 |
| 26 | canopy_batched | 210.79 | 0.990x | 58.01 | 152.78 |
| 28 | merkle_whir | 232.41 | 1.000x | 115.12 | 117.28 |
| 28 | canopy_separate | 253.05 | 1.089x | 87.89 | 165.16 |
| 28 | canopy_batched | 229.95 | 0.989x | 64.80 | 165.16 |
| 30 | merkle_whir | 249.54 | 1.000x | 133.54 | 116.00 |
| 30 | canopy_separate | 270.00 | 1.082x | 107.40 | 162.59 |
| 30 | canopy_batched | 239.24 | 0.959x | 76.64 | 162.59 |

## Experiment 3 vs Merkle WHIR components

All entries below are KiB. Roots and authentication paths are hashes; the remaining columns are 256-bit field elements.

| log2 message | scheme | roots | authentication | OOD | sumcheck | query answers | terminal coefficients | total |
|---:|---|---:|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 0.12 | 71.65 | 0.12 | 2.06 | 101.48 | 2.00 | 177.45 |
| 22 | canopy_batched | 0.12 | 40.88 | 0.22 | 3.75 | 133.97 | 4.00 | 182.94 |
| 24 | merkle_whir | 0.12 | 84.35 | 0.12 | 2.25 | 101.50 | 8.00 | 196.35 |
| 24 | canopy_batched | 0.12 | 47.12 | 0.22 | 4.12 | 133.99 | 16.00 | 201.57 |
| 26 | merkle_whir | 0.16 | 101.63 | 0.16 | 2.44 | 106.50 | 2.00 | 212.88 |
| 26 | canopy_batched | 0.16 | 57.86 | 0.28 | 4.50 | 144.00 | 4.00 | 210.79 |
| 28 | merkle_whir | 0.16 | 114.97 | 0.16 | 2.62 | 106.50 | 8.00 | 232.41 |
| 28 | canopy_batched | 0.16 | 64.64 | 0.28 | 4.88 | 144.00 | 16.00 | 229.95 |
| 30 | merkle_whir | 0.19 | 133.35 | 0.19 | 2.81 | 111.00 | 2.00 | 249.54 |
| 30 | canopy_batched | 0.19 | 76.45 | 0.34 | 5.25 | 153.00 | 4.00 | 239.24 |

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
| 2 | merkle_whir | outer WHIR initial oracle | [2^31, 2^30] | 138 | 138.00 | 2622.82 | 2208.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 1 | [2^30, 2^26] | 33 | 33.00 | 663.35 | 528.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 2 | [2^29, 2^22] | 19 | 19.00 | 377.10 | 304.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 3 | [2^28, 2^18] | 13 | 13.00 | 253.02 | 208.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 4 | [2^27, 2^14] | 10 | 10.00 | 188.53 | 160.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 5 | [2^26, 2^10] | 9 | 9.00 | 162.32 | 144.00 |
| 3 | canopy_batched | batched aligned claims initial oracle | [2^30, 2^26] | 33 | 33.00 | 1325.66 | 1056.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 1 | [2^29, 2^22] | 19 | 19.00 | 378.37 | 608.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 2 | [2^28, 2^18] | 13 | 13.00 | 253.20 | 416.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 3 | [2^27, 2^14] | 10 | 10.00 | 188.91 | 320.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 4 | [2^26, 2^10] | 9 | 9.00 | 162.42 | 288.00 |
| 3 | canopy_batched | outer first oracle via Canopy | [2^31, 2^30] | 138 | 138.00 | 138.00 | 2208.00 |
