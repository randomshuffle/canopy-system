# Batched WHIR and Canopy Exp3 (johnson)

Batched WHIR rates 1/2 and 1/4, and Canopy Exp3; GF(2^256), BLAKE3, 128-bit target with a 136-bit local algebraic/query budget. t=1,2,4,8; n=18,20,...,30. Fold logs 3/4 searched with terminal cap 10. 100 deterministic query samples, seed 0x50415045525f5631; Johnson decoding. Authentication counts are verifier leaf and parent calls; Digest-to-field conversion is an invertible byte mapping with zero hash calls. WHIR proof models deduplicate query rows.

## Selected proof sizes

Both relative columns use the corresponding Merkle-WHIR baseline at the same `t,n`.

| t | log2 n | scheme | rate | batched sources | folds | h | terminal | min event bits | union bits | 128-bit RBR? | KiB | verifier auth hashes | H2F | vs batched | vs batched |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|:---:|---:|---:|---:|---:|---:|
| 1 | 22 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,4 | - | 9 | 136.06 | 134.57 | yes | 329.04 | 6566.08 | 0.00 | 1.000x | 1.000x |
| 1 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,4,4 | - | 8 | 136.65 | 135.12 | yes | 219.78 | 4299.27 | 0.00 | 0.668x | 0.668x |
| 1 | 22 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3,4 | 4 | 9 | 136.06 | 134.57 | yes | 290.73 | 4445.14 | 0.00 | 0.884x | 0.884x |
| 2 | 22 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,3,4 | - | 9 | 136.06 | 134.16 | yes | 574.66 | 11932.02 | 0.00 | 1.000x | 1.000x |
| 2 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,4 | - | 8 | 136.65 | 134.69 | yes | 362.59 | 7545.26 | 0.00 | 0.631x | 0.631x |
| 2 | 22 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,3,4 | 4 | 9 | 136.06 | 134.16 | yes | 497.45 | 7669.11 | 0.00 | 0.866x | 0.866x |
| 4 | 22 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,3,4 | - | 9 | 136.06 | 133.58 | yes | 1065.62 | 22653.43 | 0.00 | 1.000x | 1.000x |
| 4 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,4,4 | - | 8 | 136.65 | 134.09 | yes | 648.86 | 14055.99 | 0.00 | 0.609x | 0.609x |
| 4 | 22 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,3,4 | 4 | 9 | 136.06 | 133.58 | yes | 911.77 | 14146.82 | 0.00 | 0.856x | 0.856x |
| 8 | 22 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,3,4 | - | 9 | 136.06 | 132.85 | yes | 2045.64 | 44035.40 | 0.00 | 1.000x | 1.000x |
| 8 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,4,4 | - | 8 | 136.65 | 133.35 | yes | 1221.50 | 27080.19 | 0.00 | 0.597x | 0.597x |
| 8 | 22 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,3,4 | 4 | 9 | 136.06 | 132.85 | yes | 1738.13 | 27031.03 | 0.00 | 0.850x | 0.850x |

## Component breakdown

Size entries are KiB; authentication columns count hash calls; the legacy H2F column is zero.

| t | log2 n | scheme | roots | auth | OOD | sumcheck | batching | answers | coefficients | total | verifier auth hashes | H2F |
|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 22 | merkle_whir_batched_after_second | 0.12 | 173.51 | 0.12 | 2.06 | 0.00 | 137.22 | 16.00 | 329.04 | 6566.08 | 0.00 |
| 1 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.12 | 115.98 | 0.12 | 2.06 | 0.00 | 93.49 | 8.00 | 219.78 | 4299.27 | 0.00 |
| 1 | 22 | canopy_schwartz_zippel | 0.12 | 111.13 | 0.16 | 2.06 | 0.03 | 161.22 | 16.00 | 290.73 | 4445.14 | 0.00 |
| 2 | 22 | merkle_whir_batched_after_second | 0.19 | 315.44 | 0.19 | 2.34 | 0.03 | 240.47 | 16.00 | 574.66 | 11932.02 | 0.00 |
| 2 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 203.85 | 0.19 | 2.34 | 0.03 | 147.99 | 8.00 | 362.59 | 7545.26 | 0.00 |
| 2 | 22 | canopy_schwartz_zippel | 0.19 | 190.03 | 0.25 | 2.34 | 0.16 | 288.48 | 16.00 | 497.45 | 7669.11 | 0.00 |
| 4 | 22 | merkle_whir_batched_after_second | 0.31 | 599.00 | 0.31 | 2.91 | 0.16 | 446.93 | 16.00 | 1065.62 | 22653.43 | 0.00 |
| 4 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.31 | 380.19 | 0.31 | 2.91 | 0.16 | 256.99 | 8.00 | 648.86 | 14055.99 | 0.00 |
| 4 | 22 | canopy_schwartz_zippel | 0.31 | 348.78 | 0.44 | 2.91 | 0.41 | 542.92 | 16.00 | 911.77 | 14146.82 | 0.00 |
| 8 | 22 | merkle_whir_batched_after_second | 0.56 | 1164.20 | 0.56 | 4.03 | 0.41 | 859.88 | 16.00 | 2045.64 | 44035.40 | 0.00 |
| 8 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.56 | 732.94 | 0.56 | 4.03 | 0.41 | 475.00 | 8.00 | 1221.50 | 27080.19 | 0.00 |
| 8 | 22 | canopy_schwartz_zippel | 0.56 | 664.04 | 0.81 | 4.03 | 0.91 | 1051.78 | 16.00 | 1738.13 | 27031.03 | 0.00 |

## Oracle queries at n=30

`unique` is the mean number of distinct positions per queried oracle group; authentication and answer columns are total transmitted values across the reported oracle count.

| exp | t | scheme | role | oracles | code | field | requested | unique | auth | answer fields |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|---:|
