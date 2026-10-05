# Batched WHIR and Canopy Exp3 (johnson)

Batched WHIR rates 1/2 and 1/4, and Canopy Exp3; GF(2^256), BLAKE3, 128-bit target with a 136-bit local algebraic/query budget. t=1,2,4,8; n=18,20,...,30. Fold logs 3/4 searched with terminal cap 10. 100 deterministic query samples, seed 0x50415045525f5631; Johnson decoding. Authentication counts are verifier leaf and parent calls; Digest-to-field conversion is an invertible byte mapping with zero hash calls. WHIR proof models deduplicate query rows.

## Selected proof sizes

Both relative columns use the corresponding Merkle-WHIR baseline at the same `t,n`.

| t | log2 n | scheme | rate | batched sources | folds | h | terminal | min event bits | union bits | 128-bit RBR? | KiB | verifier auth hashes | H2F | vs batched | vs batched |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|:---:|---:|---:|---:|---:|---:|
| 1 | 28 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,4,3,3,4 | - | 8 | 136.06 | 134.14 | yes | 460.53 | 10417.09 | 0.00 | 1.000x | 1.000x |
| 1 | 28 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,3,3,3 | - | 8 | 136.38 | 134.54 | yes | 309.33 | 6678.55 | 0.00 | 0.672x | 0.672x |
| 1 | 28 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,3,3,4 | 4 | 8 | 136.06 | 134.14 | yes | 380.53 | 6962.07 | 0.00 | 0.826x | 0.826x |
| 2 | 28 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,4,3,3,4 | - | 8 | 136.06 | 133.83 | yes | 783.21 | 18248.76 | 0.00 | 1.000x | 1.000x |
| 2 | 28 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,4,3,3 | - | 8 | 136.38 | 134.23 | yes | 493.75 | 11460.09 | 0.00 | 0.630x | 0.630x |
| 2 | 28 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,4,3,3,4 | 4 | 8 | 136.06 | 133.83 | yes | 623.71 | 11352.73 | 0.00 | 0.796x | 0.796x |

## Component breakdown

Size entries are KiB; authentication columns count hash calls; the legacy H2F column is zero.

| t | log2 n | scheme | roots | auth | OOD | sumcheck | batching | answers | coefficients | total | verifier auth hashes | H2F |
|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 28 | merkle_whir_batched_after_second | 0.19 | 291.03 | 0.19 | 2.62 | 0.00 | 158.50 | 8.00 | 460.53 | 10417.09 | 0.00 |
| 1 | 28 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 188.33 | 0.19 | 2.62 | 0.00 | 110.00 | 8.00 | 309.33 | 6678.55 | 0.00 |
| 1 | 28 | canopy_schwartz_zippel | 0.19 | 186.97 | 0.22 | 2.62 | 0.03 | 182.50 | 8.00 | 380.53 | 6962.07 | 0.00 |
| 2 | 28 | merkle_whir_batched_after_second | 0.25 | 510.02 | 0.25 | 2.91 | 0.03 | 261.75 | 8.00 | 783.21 | 18248.76 | 0.00 |
| 2 | 28 | merkle_whir_batched_rate_quarter_after_second | 0.25 | 323.57 | 0.25 | 2.91 | 0.03 | 158.75 | 8.00 | 493.75 | 11460.09 | 0.00 |
| 2 | 28 | canopy_schwartz_zippel | 0.25 | 302.34 | 0.31 | 2.91 | 0.16 | 309.75 | 8.00 | 623.71 | 11352.73 | 0.00 |

## Oracle queries at n=30

`unique` is the mean number of distinct positions per queried oracle group; authentication and answer columns are total transmitted values across the reported oracle count.

| exp | t | scheme | role | oracles | code | field | requested | unique | auth | answer fields |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|---:|
