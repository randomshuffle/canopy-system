# Batched WHIR and Canopy Exp3 (johnson)

Batched WHIR rates 1/2 and 1/4, and Canopy Exp3; GF(2^256), BLAKE3, 128-bit target with a 136-bit local algebraic/query budget. t=1,2,4,8; n=18,20,...,30. Fold logs 3/4 searched with terminal cap 10. 100 deterministic query samples, seed 0x50415045525f5631; Johnson decoding. Authentication counts are verifier leaf and parent calls; H2F is separate. WHIR proof models deduplicate query rows.

## Selected proof sizes

Both relative columns use the corresponding Merkle-WHIR baseline at the same `t,n`.

| t | log2 n | scheme | rate | batched sources | folds | h | terminal | min event bits | union bits | 128-bit RBR? | KiB | verifier auth hashes | H2F | vs batched | vs batched |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|:---:|---:|---:|---:|---:|---:|
| 1 | 18 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3 | - | 9 | 136.06 | 134.82 | yes | 237.80 | 4225.66 | 0.00 | 1.000x | 1.000x |
| 1 | 18 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,3 | - | 9 | 136.65 | 135.47 | yes | 157.52 | 2876.48 | 0.00 | 0.662x | 0.662x |
| 1 | 18 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3 | 4 | 9 | 136.06 | 134.82 | yes | 226.72 | 2979.99 | 315.59 | 0.953x | 0.953x |
| 1 | 20 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,3 | - | 8 | 136.06 | 134.57 | yes | 280.49 | 5595.33 | 0.00 | 1.000x | 1.000x |
| 1 | 20 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,3,3 | - | 8 | 136.65 | 135.16 | yes | 186.08 | 3826.89 | 0.00 | 0.663x | 0.663x |
| 1 | 20 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3,3 | 4 | 8 | 136.06 | 134.57 | yes | 255.79 | 3911.07 | 316.60 | 0.912x | 0.912x |
| 1 | 22 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,4 | - | 9 | 136.06 | 134.57 | yes | 329.04 | 6566.08 | 0.00 | 1.000x | 1.000x |
| 1 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,4,4 | - | 8 | 136.65 | 135.12 | yes | 219.78 | 4299.27 | 0.00 | 0.668x | 0.668x |
| 1 | 22 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3,4 | 4 | 9 | 136.06 | 134.57 | yes | 290.73 | 4445.14 | 316.90 | 0.884x | 0.884x |
| 1 | 24 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,3,4 | - | 8 | 136.06 | 134.37 | yes | 370.86 | 8036.59 | 0.00 | 1.000x | 1.000x |
| 1 | 24 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,3,3,4 | - | 8 | 136.65 | 134.98 | yes | 248.80 | 5426.60 | 0.00 | 0.671x | 0.671x |
| 1 | 24 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3,3,4 | 4 | 8 | 136.06 | 134.37 | yes | 318.65 | 5471.07 | 316.96 | 0.859x | 0.859x |
| 1 | 26 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,3,3,3 | - | 8 | 136.06 | 134.35 | yes | 415.49 | 9547.82 | 0.00 | 1.000x | 1.000x |
| 1 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,3,4 | - | 8 | 136.38 | 134.63 | yes | 279.18 | 5658.78 | 0.00 | 0.672x | 0.672x |
| 1 | 26 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3,3,3,3 | 4 | 8 | 136.06 | 134.35 | yes | 349.32 | 6535.12 | 317.00 | 0.841x | 0.841x |
| 1 | 28 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,4,3,3,4 | - | 8 | 136.06 | 134.14 | yes | 460.53 | 10417.09 | 0.00 | 1.000x | 1.000x |
| 1 | 28 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,3,3,3 | - | 8 | 136.38 | 134.54 | yes | 309.33 | 6678.55 | 0.00 | 0.672x | 0.672x |
| 1 | 28 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,3,3,4 | 4 | 8 | 136.06 | 134.14 | yes | 380.53 | 6962.07 | 317.00 | 0.826x | 0.826x |
| 1 | 30 | merkle_whir_batched_after_second | 1/2 | 1 | 3,4,4,4,4,3 | - | 8 | 136.19 | 134.51 | yes | 503.68 | 10979.73 | 0.00 | 1.000x | 1.000x |
| 1 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,3,4,4 | - | 8 | 136.38 | 134.61 | yes | 338.30 | 7269.48 | 0.00 | 0.672x | 0.672x |
| 1 | 30 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,4,4,4 | 4 | 8 | 136.06 | 134.51 | yes | 410.49 | 7480.64 | 317.00 | 0.815x | 0.815x |

## Component breakdown

Size entries are KiB; authentication and H2F columns count hash calls.

| t | log2 n | scheme | roots | auth | OOD | sumcheck | batching | answers | coefficients | total | verifier auth hashes | H2F |
|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 18 | merkle_whir_batched_after_second | 0.09 | 102.89 | 0.09 | 1.69 | 0.00 | 117.04 | 16.00 | 237.80 | 4225.66 | 0.00 |
| 1 | 18 | merkle_whir_batched_rate_quarter_after_second | 0.09 | 73.43 | 0.09 | 1.69 | 0.00 | 66.22 | 16.00 | 157.52 | 2876.48 | 0.00 |
| 1 | 18 | canopy_schwartz_zippel | 0.09 | 67.82 | 0.12 | 1.69 | 0.03 | 140.96 | 16.00 | 226.72 | 2979.99 | 315.59 |
| 1 | 20 | merkle_whir_batched_after_second | 0.12 | 143.18 | 0.12 | 1.88 | 0.00 | 127.18 | 8.00 | 280.49 | 5595.33 | 0.00 |
| 1 | 20 | merkle_whir_batched_rate_quarter_after_second | 0.12 | 100.97 | 0.12 | 1.88 | 0.00 | 74.99 | 8.00 | 186.08 | 3826.89 | 0.00 |
| 1 | 20 | canopy_schwartz_zippel | 0.12 | 94.44 | 0.16 | 1.88 | 0.03 | 151.16 | 8.00 | 255.79 | 3911.07 | 316.60 |
| 1 | 22 | merkle_whir_batched_after_second | 0.12 | 173.51 | 0.12 | 2.06 | 0.00 | 137.22 | 16.00 | 329.04 | 6566.08 | 0.00 |
| 1 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.12 | 115.98 | 0.12 | 2.06 | 0.00 | 93.49 | 8.00 | 219.78 | 4299.27 | 0.00 |
| 1 | 22 | canopy_schwartz_zippel | 0.12 | 111.13 | 0.16 | 2.06 | 0.03 | 161.22 | 16.00 | 290.73 | 4445.14 | 316.90 |
| 1 | 24 | merkle_whir_batched_after_second | 0.16 | 217.55 | 0.16 | 2.25 | 0.00 | 142.75 | 8.00 | 370.86 | 8036.59 | 0.00 |
| 1 | 24 | merkle_whir_batched_rate_quarter_after_second | 0.16 | 149.24 | 0.16 | 2.25 | 0.00 | 89.00 | 8.00 | 248.80 | 5426.60 | 0.00 |
| 1 | 24 | canopy_schwartz_zippel | 0.16 | 141.28 | 0.19 | 2.25 | 0.03 | 166.74 | 8.00 | 318.65 | 5471.07 | 316.96 |
| 1 | 26 | merkle_whir_batched_after_second | 0.19 | 263.18 | 0.19 | 2.44 | 0.00 | 141.50 | 8.00 | 415.49 | 9547.82 | 0.00 |
| 1 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.16 | 157.68 | 0.16 | 2.44 | 0.00 | 110.75 | 8.00 | 279.18 | 5658.78 | 0.00 |
| 1 | 26 | canopy_schwartz_zippel | 0.19 | 172.94 | 0.22 | 2.44 | 0.03 | 165.50 | 8.00 | 349.32 | 6535.12 | 317.00 |
| 1 | 28 | merkle_whir_batched_after_second | 0.19 | 291.03 | 0.19 | 2.62 | 0.00 | 158.50 | 8.00 | 460.53 | 10417.09 | 0.00 |
| 1 | 28 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 188.33 | 0.19 | 2.62 | 0.00 | 110.00 | 8.00 | 309.33 | 6678.55 | 0.00 |
| 1 | 28 | canopy_schwartz_zippel | 0.19 | 186.97 | 0.22 | 2.62 | 0.03 | 182.50 | 8.00 | 380.53 | 6962.07 | 317.00 |
| 1 | 30 | merkle_whir_batched_after_second | 0.19 | 309.99 | 0.19 | 2.81 | 0.00 | 182.50 | 8.00 | 503.68 | 10979.73 | 0.00 |
| 1 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 206.86 | 0.19 | 2.81 | 0.00 | 120.25 | 8.00 | 338.30 | 7269.48 | 0.00 |
| 1 | 30 | canopy_schwartz_zippel | 0.19 | 203.49 | 0.22 | 2.81 | 0.03 | 195.75 | 8.00 | 410.49 | 7480.64 | 317.00 |

## Oracle queries at n=30

`unique` is the mean number of distinct positions per queried oracle group; authentication and answer columns are total transmitted values across the reported oracle count.

| exp | t | scheme | role | oracles | code | field | requested | unique | auth | answer fields |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | independent WHIR initial oracle | 1 | [2^31,2^30] | 256 | 317 | 317.00 | 5961.17 | 2536.00 |
| 0 | 1 | merkle_whir_batched_after_second | independent WHIR recursive oracle 1 | 1 | [2^30,2^27] | 256 | 96 | 96.00 | 1777.73 | 1536.00 |
| 0 | 1 | merkle_whir_batched_after_second | independent WHIR recursive oracle 2 | 1 | [2^29,2^23] | 256 | 47 | 47.00 | 872.13 | 752.00 |
| 0 | 1 | merkle_whir_batched_after_second | independent WHIR recursive oracle 3 | 1 | [2^28,2^19] | 256 | 31 | 31.00 | 562.42 | 496.00 |
| 0 | 1 | merkle_whir_batched_after_second | independent WHIR recursive oracle 4 | 1 | [2^27,2^15] | 256 | 23 | 23.00 | 405.86 | 368.00 |
| 0 | 1 | merkle_whir_batched_after_second | independent WHIR recursive oracle 5 | 1 | [2^26,2^11] | 256 | 19 | 19.00 | 340.42 | 152.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR initial oracle | 1 | [2^32,2^30] | 256 | 147 | 147.00 | 3072.49 | 1176.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR recursive oracle 1 | 1 | [2^31,2^27] | 256 | 71 | 71.00 | 1417.78 | 1136.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR recursive oracle 2 | 1 | [2^30,2^23] | 256 | 40 | 40.00 | 792.35 | 640.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR recursive oracle 3 | 1 | [2^29,2^19] | 256 | 28 | 28.00 | 568.83 | 224.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR recursive oracle 4 | 1 | [2^28,2^16] | 256 | 23 | 23.00 | 428.03 | 368.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR recursive oracle 5 | 1 | [2^27,2^12] | 256 | 19 | 19.00 | 340.00 | 304.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims initial oracle | 2 | [2^30,2^27] | 256 | 96 | 96.00 | 3750.12 | 1536.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 1 | 1 | [2^29,2^24] | 256 | 56 | 56.00 | 1024.95 | 896.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 2 | 1 | [2^28,2^20] | 256 | 35 | 35.00 | 629.68 | 560.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 3 | 1 | [2^27,2^16] | 256 | 26 | 26.00 | 453.30 | 416.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 4 | 1 | [2^26,2^12] | 256 | 20 | 20.00 | 336.59 | 320.00 |
| 3 | 1 | canopy_schwartz_zippel | outer first oracle via Canopy | 1 | [2^31,2^30] | 256 | 317 | 317.00 | 317.00 | 2536.00 |
