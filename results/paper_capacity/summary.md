# Conjectural capacity-bound paper proof-size experiments

All experiments target 128-bit soundness over the 256-bit prime field, use BLAKE3-256, use no proof of work, and use the capacity_conjecture RS proximity regime. Capacity-bound soundness relies on STIR Conjecture 5.6 and is not proved. Means use 100 deterministic query samples from seed `0x50415045525f5631`.

## Experiment 1: one Canopy commitment vs Merkle

The table reports the proof-size-optimal Canopy height over the full feasible grid. Detailed rows and proof components are in `experiment1_canopy_vs_merkle/`.

| log2 vector | batch | best h | Merkle KiB | Canopy KiB | Canopy / Merkle |
|---:|---:|---:|---:|---:|---:|
| 22 | 128 | 18 | 60.48 | 68.47 | 1.132x |
| 22 | 256 | 7 | 112.94 | 103.17 | 0.914x |
| 22 | 512 | 3 | 209.87 | 145.27 | 0.692x |
| 22 | 1024 | 3 | 387.71 | 209.26 | 0.540x |
| 24 | 128 | 9 | 68.51 | 75.45 | 1.101x |
| 24 | 256 | 5 | 128.88 | 111.07 | 0.862x |
| 24 | 512 | 3 | 241.72 | 159.05 | 0.658x |
| 24 | 1024 | 2 | 451.73 | 218.13 | 0.483x |
| 26 | 128 | 11 | 76.43 | 80.69 | 1.056x |
| 26 | 256 | 7 | 144.95 | 117.66 | 0.812x |
| 26 | 512 | 4 | 273.73 | 165.40 | 0.604x |
| 26 | 1024 | 3 | 515.51 | 229.54 | 0.445x |
| 28 | 128 | 13 | 84.48 | 86.37 | 1.022x |
| 28 | 256 | 6 | 160.92 | 127.28 | 0.791x |
| 28 | 512 | 5 | 305.84 | 175.72 | 0.575x |
| 28 | 1024 | 2 | 579.51 | 242.91 | 0.419x |
| 30 | 128 | 11 | 92.45 | 92.66 | 1.002x |
| 30 | 256 | 7 | 176.90 | 131.99 | 0.746x |
| 30 | 512 | 4 | 337.79 | 183.61 | 0.544x |
| 30 | 1024 | 3 | 643.53 | 251.05 | 0.390x |

## Experiments 2 and 3: WHIR hybrids

For experiments 2 and 3, outer WHIR starts at rate 1/2 and folds four variables. Canopy uses `h=5` and a one-bit codeword shortening: its output and outer WHIR's second oracle both encode `2^(n-4)` coefficients into `2^n` symbols (rate 1/16). Experiment 2 proves those aligned claims separately; experiment 3 batches their two initial roots and shares every recursive oracle.

## WHIR and hybrid proof sizes

| log2 message | scheme | KiB | vs Merkle WHIR | hashes KiB | fields KiB |
|---:|---|---:|---:|---:|---:|
| 22 | merkle_whir | 184.03 | 1.000x | 75.20 | 108.82 |
| 22 | canopy_separate | 208.02 | 1.130x | 59.76 | 148.27 |
| 22 | canopy_batched | 192.68 | 1.047x | 44.42 | 148.26 |
| 24 | merkle_whir | 205.39 | 1.000x | 91.93 | 113.47 |
| 24 | canopy_separate | 233.51 | 1.137x | 75.99 | 157.53 |
| 24 | canopy_batched | 212.22 | 1.033x | 54.70 | 157.52 |
| 26 | merkle_whir | 219.50 | 1.000x | 105.75 | 113.75 |
| 26 | canopy_separate | 244.53 | 1.114x | 86.44 | 158.09 |
| 26 | canopy_batched | 220.06 | 1.003x | 61.97 | 158.09 |
| 28 | merkle_whir | 240.46 | 1.000x | 123.08 | 117.38 |
| 28 | canopy_separate | 269.14 | 1.119x | 103.79 | 165.34 |
| 28 | canopy_batched | 238.08 | 0.990x | 72.74 | 165.34 |
| 30 | merkle_whir | 255.01 | 1.000x | 137.35 | 117.66 |
| 30 | canopy_separate | 280.98 | 1.102x | 115.08 | 165.91 |
| 30 | canopy_batched | 246.40 | 0.966x | 80.50 | 165.91 |

## Oracle-query breakdown

The machine-readable table is `oracle_query_breakdown.csv`. Values below are the largest proof-size point (`n=30`).

| experiment | scheme | oracle | code | requested | unique | auth hashes | answer fields |
|---:|---|---|---|---:|---:|---:|---:|
| 2 | canopy_separate | Canopy claim WHIR initial oracle | [2^30, 2^26] | 33 | 33.00 | 662.46 | 528.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 1 | [2^29, 2^22] | 19 | 19.00 | 378.86 | 304.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 2 | [2^28, 2^18] | 13 | 13.00 | 252.50 | 208.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 3 | [2^27, 2^14] | 10 | 10.00 | 188.72 | 160.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 4 | [2^26, 2^10] | 9 | 9.00 | 162.06 | 144.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 5 | [2^25, 2^6] | 7 | 7.00 | 121.85 | 112.00 |
| 2 | canopy_separate | outer first oracle via Canopy | [2^31, 2^30] | 138 | 138.00 | 138.00 | 2208.00 |
| 2 | canopy_separate | outer second-oracle WHIR initial oracle | [2^30, 2^26] | 33 | 33.00 | 661.43 | 528.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 1 | [2^29, 2^22] | 19 | 19.00 | 378.67 | 304.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 2 | [2^28, 2^18] | 13 | 13.00 | 253.44 | 208.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 3 | [2^27, 2^14] | 10 | 10.00 | 188.97 | 160.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 4 | [2^26, 2^10] | 9 | 9.00 | 161.84 | 144.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 5 | [2^25, 2^6] | 7 | 7.00 | 121.60 | 112.00 |
| 2 | merkle_whir | outer WHIR initial oracle | [2^31, 2^30] | 138 | 138.00 | 2622.82 | 2208.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 1 | [2^30, 2^26] | 33 | 33.00 | 663.35 | 528.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 2 | [2^29, 2^22] | 19 | 19.00 | 377.10 | 304.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 3 | [2^28, 2^18] | 13 | 13.00 | 253.02 | 208.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 4 | [2^27, 2^14] | 10 | 10.00 | 188.53 | 160.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 5 | [2^26, 2^10] | 9 | 9.00 | 162.32 | 144.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 6 | [2^25, 2^6] | 7 | 7.00 | 121.20 | 112.00 |
| 3 | canopy_batched | batched aligned claims initial oracle | [2^30, 2^26] | 33 | 33.00 | 1325.66 | 1056.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 1 | [2^29, 2^22] | 19 | 19.00 | 378.37 | 608.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 2 | [2^28, 2^18] | 13 | 13.00 | 253.20 | 416.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 3 | [2^27, 2^14] | 10 | 10.00 | 188.91 | 320.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 4 | [2^26, 2^10] | 9 | 9.00 | 162.42 | 288.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 5 | [2^25, 2^6] | 7 | 7.00 | 122.30 | 224.00 |
| 3 | canopy_batched | outer first oracle via Canopy | [2^31, 2^30] | 138 | 138.00 | 138.00 | 2208.00 |
