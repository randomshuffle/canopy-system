# Batched WHIR and Canopy Exp3 (johnson)

Batched WHIR rates 1/2 and 1/4, and Canopy Exp3; GF(2^256), BLAKE3, 128-bit target with a 136-bit local algebraic/query budget. t=1,2,4,8; n=18,20,...,30. Fold logs 3/4 searched with terminal cap 10. 100 deterministic query samples, seed 0x50415045525f5631; Johnson decoding. Authentication counts are verifier leaf and parent calls; H2F is separate. WHIR proof models deduplicate query rows.

## Selected proof sizes

Both relative columns use the corresponding Merkle-WHIR baseline at the same `t,n`.

| t | log2 n | scheme | rate | batched sources | folds | h | terminal | min event bits | union bits | 128-bit RBR? | KiB | verifier auth hashes | H2F | vs batched | vs batched |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|:---:|---:|---:|---:|---:|---:|
| 1 | 24 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,3,4 | - | 8 | 136.06 | 134.37 | yes | 370.86 | 8036.59 | 0.00 | 1.000x | 1.000x |
| 1 | 24 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,3,3,4 | - | 8 | 136.65 | 134.98 | yes | 248.80 | 5426.60 | 0.00 | 0.671x | 0.671x |
| 1 | 24 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3,3,4 | 4 | 8 | 136.06 | 134.37 | yes | 318.65 | 5471.07 | 316.96 | 0.859x | 0.859x |
| 2 | 24 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,3,3,4 | - | 8 | 136.06 | 134.01 | yes | 642.16 | 14224.37 | 0.00 | 1.000x | 1.000x |
| 2 | 24 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,3,3,4 | - | 8 | 136.65 | 134.58 | yes | 405.59 | 9119.89 | 0.00 | 0.632x | 0.632x |
| 2 | 24 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,3,3,4 | 4 | 8 | 136.06 | 134.01 | yes | 537.68 | 9089.26 | 633.94 | 0.837x | 0.837x |
| 4 | 24 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,3,3,4 | - | 8 | 136.06 | 133.48 | yes | 1184.42 | 26586.68 | 0.00 | 1.000x | 1.000x |
| 4 | 24 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,3,3,4 | - | 8 | 136.65 | 134.02 | yes | 718.88 | 16495.02 | 0.00 | 0.607x | 0.607x |
| 4 | 24 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,3,3,4 | 4 | 8 | 136.06 | 133.48 | yes | 975.13 | 16306.18 | 1267.84 | 0.823x | 0.823x |
| 8 | 24 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,3,3,4 | - | 8 | 136.06 | 132.79 | yes | 2268.67 | 51304.47 | 0.00 | 1.000x | 1.000x |
| 8 | 24 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,3,3,4 | - | 8 | 136.65 | 133.30 | yes | 1345.86 | 31258.56 | 0.00 | 0.593x | 0.593x |
| 8 | 24 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,3,3,4 | 4 | 8 | 136.06 | 132.79 | yes | 1850.32 | 30748.11 | 2535.84 | 0.816x | 0.816x |

## Component breakdown

Size entries are KiB; authentication and H2F columns count hash calls.

| t | log2 n | scheme | roots | auth | OOD | sumcheck | batching | answers | coefficients | total | verifier auth hashes | H2F |
|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 24 | merkle_whir_batched_after_second | 0.16 | 217.55 | 0.16 | 2.25 | 0.00 | 142.75 | 8.00 | 370.86 | 8036.59 | 0.00 |
| 1 | 24 | merkle_whir_batched_rate_quarter_after_second | 0.16 | 149.24 | 0.16 | 2.25 | 0.00 | 89.00 | 8.00 | 248.80 | 5426.60 | 0.00 |
| 1 | 24 | canopy_schwartz_zippel | 0.16 | 141.28 | 0.19 | 2.25 | 0.03 | 166.74 | 8.00 | 318.65 | 5471.07 | 316.96 |
| 2 | 24 | merkle_whir_batched_after_second | 0.22 | 385.17 | 0.22 | 2.53 | 0.03 | 246.00 | 8.00 | 642.16 | 14224.37 | 0.00 |
| 2 | 24 | merkle_whir_batched_rate_quarter_after_second | 0.22 | 251.09 | 0.22 | 2.53 | 0.03 | 143.50 | 8.00 | 405.59 | 9119.89 | 0.00 |
| 2 | 24 | canopy_schwartz_zippel | 0.22 | 232.51 | 0.28 | 2.53 | 0.16 | 293.99 | 8.00 | 537.68 | 9089.26 | 633.94 |
| 4 | 24 | merkle_whir_batched_after_second | 0.34 | 719.99 | 0.34 | 3.09 | 0.16 | 452.50 | 8.00 | 1184.42 | 26586.68 | 0.00 |
| 4 | 24 | merkle_whir_batched_rate_quarter_after_second | 0.34 | 454.44 | 0.34 | 3.09 | 0.16 | 252.50 | 8.00 | 718.88 | 16495.02 | 0.00 |
| 4 | 24 | canopy_schwartz_zippel | 0.34 | 414.35 | 0.47 | 3.09 | 0.41 | 548.47 | 8.00 | 975.13 | 16306.18 | 1267.84 |
| 8 | 24 | merkle_whir_batched_after_second | 0.59 | 1389.44 | 0.59 | 4.22 | 0.41 | 865.42 | 8.00 | 2268.67 | 51304.47 | 0.00 |
| 8 | 24 | merkle_whir_batched_rate_quarter_after_second | 0.59 | 861.55 | 0.59 | 4.22 | 0.41 | 470.50 | 8.00 | 1345.86 | 31258.56 | 0.00 |
| 8 | 24 | canopy_schwartz_zippel | 0.59 | 778.28 | 0.84 | 4.22 | 0.91 | 1057.48 | 8.00 | 1850.32 | 30748.11 | 2535.84 |

## Oracle queries at n=30

`unique` is the mean number of distinct positions per queried oracle group; authentication and answer columns are total transmitted values across the reported oracle count.

| exp | t | scheme | role | oracles | code | field | requested | unique | auth | answer fields |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|---:|
