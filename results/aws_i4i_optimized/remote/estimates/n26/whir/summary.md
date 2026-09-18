# Batched WHIR and Canopy Exp3 (johnson)

Batched WHIR rates 1/2 and 1/4, and Canopy Exp3; GF(2^256), BLAKE3, 128-bit target with a 136-bit local algebraic/query budget. t=1,2,4,8; n=18,20,...,30. Fold logs 3/4 searched with terminal cap 10. 100 deterministic query samples, seed 0x50415045525f5631; Johnson decoding. Authentication counts are verifier leaf and parent calls; H2F is separate. WHIR proof models deduplicate query rows.

## Selected proof sizes

Both relative columns use the corresponding Merkle-WHIR baseline at the same `t,n`.

| t | log2 n | scheme | rate | batched sources | folds | h | terminal | min event bits | union bits | 128-bit RBR? | KiB | verifier auth hashes | H2F | vs batched | vs batched |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|:---:|---:|---:|---:|---:|---:|
| 1 | 26 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,3,3,3 | - | 8 | 136.06 | 134.35 | yes | 415.49 | 9547.82 | 0.00 | 1.000x | 1.000x |
| 1 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,3,4 | - | 8 | 136.38 | 134.63 | yes | 279.18 | 5658.78 | 0.00 | 0.672x | 0.672x |
| 1 | 26 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3,3,3,3 | 4 | 8 | 136.06 | 134.35 | yes | 349.32 | 6535.12 | 317.00 | 0.841x | 0.841x |
| 2 | 26 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,3,3,3,3 | - | 8 | 136.06 | 134.00 | yes | 712.45 | 16556.44 | 0.00 | 1.000x | 1.000x |
| 2 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,4,4 | - | 8 | 136.38 | 134.30 | yes | 449.37 | 9984.85 | 0.00 | 0.631x | 0.631x |
| 2 | 26 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,3,3,3,3 | 4 | 8 | 136.06 | 134.00 | yes | 580.62 | 10545.87 | 633.98 | 0.815x | 0.815x |
| 4 | 26 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,3,3,3,3 | - | 8 | 136.06 | 133.47 | yes | 1306.37 | 30571.83 | 0.00 | 1.000x | 1.000x |
| 4 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,4,4,4 | - | 8 | 136.38 | 133.82 | yes | 790.00 | 18234.93 | 0.00 | 0.605x | 0.605x |
| 4 | 26 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,3,3,3,3 | 4 | 8 | 136.06 | 133.47 | yes | 1042.54 | 18545.60 | 1267.96 | 0.798x | 0.798x |
| 8 | 26 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,3,3,3,3 | - | 8 | 136.06 | 132.79 | yes | 2493.82 | 58590.91 | 0.00 | 1.000x | 1.000x |
| 8 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,4,4,4 | - | 8 | 136.38 | 133.18 | yes | 1472.27 | 34768.10 | 0.00 | 0.590x | 0.590x |
| 8 | 26 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,3,3,3,3 | 4 | 8 | 136.06 | 132.79 | yes | 1965.80 | 34525.55 | 2536.00 | 0.788x | 0.788x |

## Component breakdown

Size entries are KiB; authentication and H2F columns count hash calls.

| t | log2 n | scheme | roots | auth | OOD | sumcheck | batching | answers | coefficients | total | verifier auth hashes | H2F |
|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 26 | merkle_whir_batched_after_second | 0.19 | 263.18 | 0.19 | 2.44 | 0.00 | 141.50 | 8.00 | 415.49 | 9547.82 | 0.00 |
| 1 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.16 | 157.68 | 0.16 | 2.44 | 0.00 | 110.75 | 8.00 | 279.18 | 5658.78 | 0.00 |
| 1 | 26 | canopy_schwartz_zippel | 0.19 | 172.94 | 0.22 | 2.44 | 0.03 | 165.50 | 8.00 | 349.32 | 6535.12 | 317.00 |
| 2 | 26 | merkle_whir_batched_after_second | 0.25 | 456.45 | 0.25 | 2.72 | 0.03 | 244.75 | 8.00 | 712.45 | 16556.44 | 0.00 |
| 2 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.22 | 278.68 | 0.22 | 2.72 | 0.03 | 159.50 | 8.00 | 449.37 | 9984.85 | 0.00 |
| 2 | 26 | canopy_schwartz_zippel | 0.25 | 276.43 | 0.31 | 2.72 | 0.16 | 292.75 | 8.00 | 580.62 | 10545.87 | 633.98 |
| 4 | 26 | merkle_whir_batched_after_second | 0.38 | 842.93 | 0.38 | 3.28 | 0.16 | 451.25 | 8.00 | 1306.37 | 30571.83 | 0.00 |
| 4 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.34 | 509.37 | 0.34 | 3.28 | 0.16 | 268.50 | 8.00 | 790.00 | 18234.93 | 0.00 |
| 4 | 26 | canopy_schwartz_zippel | 0.38 | 482.74 | 0.50 | 3.28 | 0.41 | 547.24 | 8.00 | 1042.54 | 18545.60 | 1267.96 |
| 8 | 26 | merkle_whir_batched_after_second | 0.62 | 1615.53 | 0.62 | 4.41 | 0.41 | 864.23 | 8.00 | 2493.82 | 58590.91 | 0.00 |
| 8 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.59 | 971.79 | 0.59 | 4.41 | 0.41 | 486.48 | 8.00 | 1472.27 | 34768.10 | 0.00 |
| 8 | 26 | canopy_schwartz_zippel | 0.62 | 894.74 | 0.88 | 4.41 | 0.91 | 1056.25 | 8.00 | 1965.80 | 34525.55 | 2536.00 |

## Oracle queries at n=30

`unique` is the mean number of distinct positions per queried oracle group; authentication and answer columns are total transmitted values across the reported oracle count.

| exp | t | scheme | role | oracles | code | field | requested | unique | auth | answer fields |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|---:|
