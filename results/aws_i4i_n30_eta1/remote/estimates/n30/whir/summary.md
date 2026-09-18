# Batched WHIR and Canopy Exp3 (johnson)

Batched WHIR rates 1/2 and 1/4, and Canopy Exp3; GF(2^256), BLAKE3, 128-bit target with a 136-bit local algebraic/query budget. t=1,2,4,8; n=18,20,...,30. Fold logs 3/4 searched with terminal cap 10. 100 deterministic query samples, seed 0x50415045525f5631; Johnson decoding. Authentication counts are verifier leaf and parent calls; Digest-to-field conversion is an invertible byte mapping with zero hash calls. WHIR proof models deduplicate query rows.

## Selected proof sizes

Both relative columns use the corresponding Merkle-WHIR baseline at the same `t,n`.

| t | log2 n | scheme | rate | batched sources | folds | h | terminal | min event bits | union bits | 128-bit RBR? | KiB | verifier auth hashes | H2F | vs batched | vs batched |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|:---:|---:|---:|---:|---:|---:|
| 1 | 30 | merkle_whir_batched_after_second | 1/2 | 1 | 3,4,4,4,4,3 | - | 8 | 136.19 | 134.51 | yes | 503.68 | 10979.73 | 0.00 | 1.000x | 1.000x |
| 1 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,3,4,4 | - | 8 | 136.38 | 134.61 | yes | 338.30 | 7269.48 | 0.00 | 0.672x | 0.672x |
| 1 | 30 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,4,4,4 | 4 | 8 | 136.06 | 134.51 | yes | 410.49 | 7480.64 | 0.00 | 0.815x | 0.815x |

## Component breakdown

Size entries are KiB; authentication columns count hash calls; the legacy H2F column is zero.

| t | log2 n | scheme | roots | auth | OOD | sumcheck | batching | answers | coefficients | total | verifier auth hashes | H2F |
|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 30 | merkle_whir_batched_after_second | 0.19 | 309.99 | 0.19 | 2.81 | 0.00 | 182.50 | 8.00 | 503.68 | 10979.73 | 0.00 |
| 1 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 206.86 | 0.19 | 2.81 | 0.00 | 120.25 | 8.00 | 338.30 | 7269.48 | 0.00 |
| 1 | 30 | canopy_schwartz_zippel | 0.19 | 203.49 | 0.22 | 2.81 | 0.03 | 195.75 | 8.00 | 410.49 | 7480.64 | 0.00 |

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
