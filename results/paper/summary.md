# Final Johnson-bound paper proof-size experiments

All experiments target 128-bit soundness over the 256-bit prime field, use BLAKE3-256, use no proof of work, and use the johnson RS proximity regime. Means use 100 deterministic query samples from seed `0x50415045525f5631`.

## Experiment 1: one Canopy commitment vs Merkle

The table reports the proof-size-optimal Canopy height over the full feasible grid. Detailed rows and proof components are in `experiment1_canopy_vs_merkle/`.

| log2 vector | batch | best h | Merkle KiB | Canopy KiB | Canopy / Merkle |
|---:|---:|---:|---:|---:|---:|
| 22 | 128 | 18 | 60.48 | 74.64 | 1.234x |
| 22 | 256 | 18 | 112.94 | 127.08 | 1.125x |
| 22 | 512 | 7 | 209.87 | 201.59 | 0.961x |
| 22 | 1024 | 3 | 387.71 | 283.23 | 0.731x |
| 24 | 128 | 20 | 68.51 | 81.72 | 1.193x |
| 24 | 256 | 20 | 128.88 | 142.08 | 1.102x |
| 24 | 512 | 5 | 241.72 | 214.90 | 0.889x |
| 24 | 1024 | 3 | 451.73 | 310.01 | 0.686x |
| 26 | 128 | 22 | 76.43 | 89.48 | 1.171x |
| 26 | 256 | 11 | 144.95 | 156.63 | 1.081x |
| 26 | 512 | 7 | 273.73 | 228.86 | 0.836x |
| 26 | 1024 | 4 | 515.51 | 322.98 | 0.627x |
| 28 | 128 | 24 | 84.48 | 97.20 | 1.151x |
| 28 | 256 | 13 | 160.92 | 168.17 | 1.045x |
| 28 | 512 | 6 | 305.84 | 245.98 | 0.804x |
| 28 | 1024 | 5 | 579.51 | 343.09 | 0.592x |
| 30 | 128 | 26 | 92.45 | 104.73 | 1.133x |
| 30 | 256 | 11 | 176.90 | 178.56 | 1.009x |
| 30 | 512 | 7 | 337.79 | 256.51 | 0.759x |
| 30 | 1024 | 4 | 643.53 | 358.56 | 0.557x |

## Experiments 2 and 3: WHIR hybrids

For experiments 2 and 3, outer WHIR starts at rate 1/2 and folds four variables. Canopy uses `h=5` and a one-bit codeword shortening: its output and outer WHIR's second oracle both encode `2^(n-4)` coefficients into `2^n` symbols (rate 1/16). Experiment 2 proves those aligned claims separately; experiment 3 batches their two initial roots and shares every recursive oracle.

## WHIR and hybrid proof sizes

| log2 message | scheme | KiB | vs Merkle WHIR | hashes KiB | fields KiB |
|---:|---|---:|---:|---:|---:|
| 22 | merkle_whir | 369.86 | 1.000x | 143.03 | 226.82 |
| 22 | canopy_separate | 415.24 | 1.123x | 110.99 | 304.25 |
| 22 | canopy_batched | 387.49 | 1.048x | 83.26 | 304.24 |
| 24 | merkle_whir | 412.41 | 1.000x | 176.95 | 235.46 |
| 24 | canopy_separate | 463.21 | 1.123x | 141.69 | 321.52 |
| 24 | canopy_batched | 424.36 | 1.029x | 102.84 | 321.53 |
| 26 | merkle_whir | 441.82 | 1.000x | 206.07 | 235.75 |
| 26 | canopy_separate | 484.76 | 1.097x | 162.67 | 322.09 |
| 26 | canopy_batched | 439.68 | 0.995x | 117.59 | 322.09 |
| 28 | merkle_whir | 484.47 | 1.000x | 241.59 | 242.88 |
| 28 | canopy_separate | 532.61 | 1.099x | 196.27 | 336.34 |
| 28 | canopy_batched | 474.88 | 0.980x | 138.54 | 336.34 |
| 30 | merkle_whir | 514.61 | 1.000x | 271.46 | 243.16 |
| 30 | canopy_separate | 555.94 | 1.080x | 219.03 | 336.91 |
| 30 | canopy_batched | 491.09 | 0.954x | 154.18 | 336.91 |

## Oracle-query breakdown

The machine-readable table is `oracle_query_breakdown.csv`. Values below are the largest proof-size point (`n=30`).

| experiment | scheme | oracle | code | requested | unique | auth hashes | answer fields |
|---:|---|---|---|---:|---:|---:|---:|
| 2 | canopy_separate | Canopy claim WHIR initial oracle | [2^30, 2^26] | 67 | 67.00 | 1275.89 | 1072.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 1 | [2^29, 2^22] | 38 | 38.00 | 718.35 | 608.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 2 | [2^28, 2^18] | 26 | 26.00 | 480.67 | 416.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 3 | [2^27, 2^14] | 20 | 20.00 | 356.71 | 320.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 4 | [2^26, 2^10] | 17 | 17.00 | 290.49 | 272.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 5 | [2^25, 2^6] | 14 | 14.00 | 228.85 | 224.00 |
| 2 | canopy_separate | outer first oracle via Canopy | [2^31, 2^30] | 298 | 298.00 | 298.00 | 4768.00 |
| 2 | canopy_separate | outer second-oracle WHIR initial oracle | [2^30, 2^26] | 67 | 67.00 | 1276.56 | 1072.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 1 | [2^29, 2^22] | 38 | 38.00 | 718.35 | 608.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 2 | [2^28, 2^18] | 26 | 26.00 | 479.52 | 416.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 3 | [2^27, 2^14] | 20 | 20.00 | 355.38 | 320.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 4 | [2^26, 2^10] | 17 | 17.00 | 289.33 | 272.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 5 | [2^25, 2^6] | 14 | 14.00 | 228.93 | 224.00 |
| 2 | merkle_whir | outer WHIR initial oracle | [2^31, 2^30] | 298 | 298.00 | 5330.16 | 4768.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 1 | [2^30, 2^26] | 67 | 67.00 | 1275.77 | 1072.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 2 | [2^29, 2^22] | 38 | 38.00 | 719.19 | 608.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 3 | [2^28, 2^18] | 26 | 26.00 | 478.39 | 416.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 4 | [2^27, 2^14] | 20 | 20.00 | 356.49 | 320.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 5 | [2^26, 2^10] | 17 | 17.00 | 290.14 | 272.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 6 | [2^25, 2^6] | 14 | 14.00 | 229.51 | 224.00 |
| 3 | canopy_batched | batched aligned claims initial oracle | [2^30, 2^26] | 67 | 67.00 | 2554.78 | 2144.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 1 | [2^29, 2^22] | 38 | 38.00 | 719.17 | 1216.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 2 | [2^28, 2^18] | 26 | 26.00 | 479.00 | 832.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 3 | [2^27, 2^14] | 20 | 20.00 | 356.88 | 640.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 4 | [2^26, 2^10] | 17 | 17.00 | 290.06 | 544.00 |
| 3 | canopy_batched | batched aligned claims recursive oracle 5 | [2^25, 2^6] | 14 | 14.00 | 228.89 | 448.00 |
| 3 | canopy_batched | outer first oracle via Canopy | [2^31, 2^30] | 298 | 298.00 | 298.00 | 4768.00 |
