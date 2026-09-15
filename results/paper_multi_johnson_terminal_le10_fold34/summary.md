# Multi-input Experiments 2--4 (johnson)

This report uses `t in {4,8}` input oracles, a 1/2 starting rate, no proof of work, 256-bit inputs, folding parameters in `{3,4}`, terminal messages of at most `2^10` elements, and the existing Johnson eta=sqrt(rho)/20 slack. The Johnson folding bound uses the exact exceptional-set expression in Theorem 4.6. Proof sizes are means over 100 deterministic query samples from seed `0x50415045525f5631`.

The independent baseline contains `t` complete Merkle-WHIR proofs. The batched baseline keeps `t` independent first transitions and batches the `t` second-oracle claims into one tail. Experiment 2 makes `2t` recursive claims and proves all independently. Experiment 3 batches its `2t` aligned 256-bit claims into one generalized Schwartz--Zippel identity, sending `2(2t)-3` cross-term coefficients before the challenge. Experiment 4 expands those `2t` source claims into `8t` GF(2^64) coordinate equations, projects them once over GF(2^192), and runs one GF(2^192) recursive tail.

## Selected proof sizes

Both relative columns use the corresponding Merkle-WHIR baseline at the same `t,n`.

| t | log2 n | scheme | batched sources | folds | h | terminal | min event bits | union bits | 128-bit RBR? | KiB | vs independent | vs batched |
|---:|---:|---|---:|---|---:|---:|---:|---:|:---:|---:|---:|---:|
| 4 | 22 | merkle_whir_independent | 0 | 3,3,4,4 | - | 8 | 128.02 | 124.67 | yes | 1248.45 | 1.000x | 1.239x |
| 4 | 22 | merkle_whir_batched_after_second | 4 | 3,3,4,4 | - | 8 | 128.02 | 125.55 | yes | 1007.28 | 0.807x | 1.000x |
| 4 | 22 | canopy_unbatched | 0 | 3,3,4,4 | 9 | 8 | 128.02 | 124.16 | yes | 1172.38 | 0.939x | 1.164x |
| 4 | 22 | canopy_schwartz_zippel | 8 | 3,3,4,4 | 4 | 8 | 128.02 | 125.51 | yes | 762.48 | 0.611x | 0.757x |
| 4 | 22 | canopy_field_switch_192 | 8 | 3,3,4,4 | 4 | 8 | 128.02 | 125.51 | yes | 750.64 | 0.601x | 0.745x |
| 8 | 22 | merkle_whir_independent | 0 | 3,3,4,4 | - | 8 | 128.02 | 123.67 | yes | 2497.93 | 1.000x | 1.292x |
| 8 | 22 | merkle_whir_batched_after_second | 8 | 3,3,4,4 | - | 8 | 128.02 | 124.77 | yes | 1932.73 | 0.774x | 1.000x |
| 8 | 22 | canopy_unbatched | 0 | 3,3,4,4 | 9 | 8 | 128.02 | 123.16 | yes | 2345.15 | 0.939x | 1.213x |
| 8 | 22 | canopy_schwartz_zippel | 16 | 3,3,4,4 | 4 | 8 | 128.02 | 124.75 | yes | 1452.72 | 0.582x | 0.752x |
| 8 | 22 | canopy_field_switch_192 | 16 | 3,3,4,4 | 4 | 8 | 128.02 | 124.74 | yes | 1440.01 | 0.576x | 0.745x |
| 4 | 24 | merkle_whir_independent | 0 | 3,3,3,3,4 | - | 8 | 128.02 | 124.35 | yes | 1404.21 | 1.000x | 1.255x |
| 4 | 24 | merkle_whir_batched_after_second | 4 | 3,3,3,3,4 | - | 8 | 128.02 | 125.40 | yes | 1118.59 | 0.797x | 1.000x |
| 4 | 24 | canopy_unbatched | 0 | 3,3,3,3,4 | 9 | 8 | 128.02 | 123.99 | yes | 1300.59 | 0.926x | 1.163x |
| 4 | 24 | canopy_schwartz_zippel | 8 | 3,3,3,3,4 | 4 | 8 | 128.02 | 125.40 | yes | 810.69 | 0.577x | 0.725x |
| 4 | 24 | canopy_field_switch_192 | 8 | 3,3,4,3,3 | 4 | 8 | 128.02 | 125.44 | yes | 799.47 | 0.569x | 0.715x |
| 8 | 24 | merkle_whir_independent | 0 | 3,3,3,3,4 | - | 8 | 128.02 | 123.35 | yes | 2806.71 | 1.000x | 1.311x |
| 8 | 24 | merkle_whir_batched_after_second | 8 | 3,3,3,3,4 | - | 8 | 128.02 | 124.68 | yes | 2141.44 | 0.763x | 1.000x |
| 8 | 24 | canopy_unbatched | 0 | 3,3,3,3,4 | 9 | 8 | 128.02 | 122.99 | yes | 2601.35 | 0.927x | 1.215x |
| 8 | 24 | canopy_schwartz_zippel | 16 | 3,3,3,3,4 | 4 | 8 | 128.02 | 124.68 | yes | 1534.37 | 0.547x | 0.717x |
| 8 | 24 | canopy_field_switch_192 | 16 | 3,3,4,3,3 | 4 | 8 | 128.02 | 124.70 | yes | 1522.64 | 0.543x | 0.711x |
| 4 | 26 | merkle_whir_independent | 0 | 3,3,3,3,3,3 | - | 8 | 128.02 | 124.26 | yes | 1571.32 | 1.000x | 1.275x |
| 4 | 26 | merkle_whir_batched_after_second | 4 | 3,3,3,3,3,3 | - | 8 | 128.02 | 125.35 | yes | 1232.83 | 0.785x | 1.000x |
| 4 | 26 | canopy_unbatched | 0 | 3,4,4,4,3 | 9 | 8 | 128.02 | 123.95 | yes | 1436.30 | 0.914x | 1.165x |
| 4 | 26 | canopy_schwartz_zippel | 8 | 3,3,4,4,4 | 4 | 8 | 128.02 | 125.47 | yes | 860.18 | 0.547x | 0.698x |
| 4 | 26 | canopy_field_switch_192 | 8 | 3,3,4,4,4 | 4 | 8 | 128.02 | 125.31 | yes | 845.47 | 0.538x | 0.686x |
| 8 | 26 | merkle_whir_independent | 0 | 3,3,3,3,3,3 | - | 8 | 128.02 | 123.26 | yes | 3143.09 | 1.000x | 1.336x |
| 8 | 26 | merkle_whir_batched_after_second | 8 | 3,3,3,3,3,3 | - | 8 | 128.02 | 124.65 | yes | 2353.24 | 0.749x | 1.000x |
| 8 | 26 | canopy_unbatched | 0 | 3,4,4,4,3 | 9 | 8 | 128.02 | 122.95 | yes | 2872.15 | 0.914x | 1.221x |
| 8 | 26 | canopy_schwartz_zippel | 16 | 3,3,4,4,4 | 4 | 8 | 128.02 | 124.72 | yes | 1617.32 | 0.515x | 0.687x |
| 8 | 26 | canopy_field_switch_192 | 16 | 3,3,4,4,4 | 4 | 8 | 128.02 | 124.62 | yes | 1602.38 | 0.510x | 0.681x |
| 4 | 28 | merkle_whir_independent | 0 | 3,3,4,3,4,3 | - | 8 | 128.02 | 123.98 | yes | 1739.32 | 1.000x | 1.290x |
| 4 | 28 | merkle_whir_batched_after_second | 4 | 3,3,4,3,4,3 | - | 8 | 128.02 | 125.19 | yes | 1347.88 | 0.775x | 1.000x |
| 4 | 28 | canopy_unbatched | 0 | 3,4,4,3,3,3 | 10 | 8 | 128.02 | 123.89 | yes | 1563.08 | 0.899x | 1.160x |
| 4 | 28 | canopy_schwartz_zippel | 8 | 3,3,4,4,3,3 | 4 | 8 | 128.02 | 125.45 | yes | 910.29 | 0.523x | 0.675x |
| 4 | 28 | canopy_field_switch_192 | 8 | 3,3,3,3,3,4 | 4 | 9 | 127.80 | 125.01 | no | 901.22 | 0.518x | 0.669x |
| 8 | 28 | merkle_whir_independent | 0 | 3,3,4,3,4,3 | - | 8 | 128.02 | 122.98 | yes | 3480.23 | 1.000x | 1.357x |
| 8 | 28 | merkle_whir_batched_after_second | 8 | 3,3,4,3,4,3 | - | 8 | 128.02 | 124.55 | yes | 2564.70 | 0.737x | 1.000x |
| 8 | 28 | canopy_unbatched | 0 | 3,4,4,3,3,3 | 10 | 8 | 128.02 | 122.89 | yes | 3126.15 | 0.898x | 1.219x |
| 8 | 28 | canopy_schwartz_zippel | 16 | 3,3,4,4,3,3 | 4 | 8 | 128.02 | 124.71 | yes | 1699.70 | 0.488x | 0.663x |
| 8 | 28 | canopy_field_switch_192 | 16 | 3,3,3,3,3,4 | 4 | 9 | 127.80 | 124.41 | no | 1691.33 | 0.486x | 0.659x |
| 4 | 30 | merkle_whir_independent | 0 | 3,4,4,4,4,4 | - | 7 | 128.02 | 124.38 | yes | 1901.76 | 1.000x | 1.301x |
| 4 | 30 | merkle_whir_batched_after_second | 4 | 3,3,4,4,4,4 | - | 8 | 128.02 | 125.48 | yes | 1461.93 | 0.769x | 1.000x |
| 4 | 30 | canopy_unbatched | 0 | 3,4,4,4,4,4 | 11 | 7 | 128.02 | 124.22 | yes | 1684.68 | 0.886x | 1.152x |
| 4 | 30 | canopy_schwartz_zippel | 8 | 3,3,4,4,4,4 | 4 | 8 | 128.02 | 125.46 | yes | 958.67 | 0.504x | 0.656x |
| 4 | 30 | canopy_field_switch_192 | 8 | 3,3,4,3,3,4 | 4 | 10 | 124.30 | 123.40 | no | 954.80 | 0.502x | 0.653x |
| 8 | 30 | merkle_whir_independent | 0 | 3,4,4,4,4,4 | - | 7 | 128.02 | 123.38 | yes | 3803.34 | 1.000x | 1.370x |
| 8 | 30 | merkle_whir_batched_after_second | 8 | 3,3,4,4,4,4 | - | 8 | 128.02 | 124.73 | yes | 2776.88 | 0.730x | 1.000x |
| 8 | 30 | canopy_unbatched | 0 | 3,4,4,4,4,4 | 11 | 7 | 128.02 | 123.22 | yes | 3368.67 | 0.886x | 1.213x |
| 8 | 30 | canopy_schwartz_zippel | 16 | 3,3,4,4,4,4 | 4 | 8 | 128.02 | 124.72 | yes | 1782.59 | 0.469x | 0.642x |
| 8 | 30 | canopy_field_switch_192 | 16 | 3,3,4,3,3,4 | 4 | 10 | 124.30 | 123.16 | no | 1777.73 | 0.467x | 0.640x |

## Component breakdown

All entries are KiB.

| t | log2 n | scheme | roots | auth | OOD | sumcheck | batching | answers | coefficients | total |
|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 4 | 22 | merkle_whir_independent | 0.50 | 647.30 | 0.50 | 8.25 | 0.00 | 559.90 | 32.00 | 1248.45 |
| 4 | 22 | merkle_whir_batched_after_second | 0.31 | 564.65 | 0.31 | 2.91 | 0.16 | 430.94 | 8.00 | 1007.28 |
| 4 | 22 | canopy_unbatched | 0.62 | 529.55 | 0.75 | 13.50 | 0.00 | 563.96 | 64.00 | 1172.38 |
| 4 | 22 | canopy_schwartz_zippel | 0.31 | 281.98 | 0.44 | 2.91 | 0.41 | 468.44 | 8.00 | 762.48 |
| 4 | 22 | canopy_field_switch_192 | 0.31 | 281.98 | 0.20 | 2.81 | 0.00 | 459.33 | 6.00 | 750.64 |
| 8 | 22 | merkle_whir_independent | 1.00 | 1295.55 | 1.00 | 16.50 | 0.00 | 1119.88 | 64.00 | 2497.93 |
| 8 | 22 | merkle_whir_batched_after_second | 0.56 | 1100.26 | 0.56 | 4.03 | 0.41 | 818.90 | 8.00 | 1932.73 |
| 8 | 22 | canopy_unbatched | 1.25 | 1059.54 | 1.50 | 27.00 | 0.00 | 1127.86 | 128.00 | 2345.15 |
| 8 | 22 | canopy_schwartz_zippel | 0.56 | 538.05 | 0.81 | 4.03 | 0.91 | 900.36 | 8.00 | 1452.72 |
| 8 | 22 | canopy_field_switch_192 | 0.56 | 537.89 | 0.32 | 4.01 | 0.00 | 891.23 | 6.00 | 1440.01 |
| 4 | 24 | merkle_whir_independent | 0.62 | 824.97 | 0.62 | 9.00 | 0.00 | 536.99 | 32.00 | 1404.21 |
| 4 | 24 | merkle_whir_batched_after_second | 0.34 | 681.40 | 0.34 | 3.09 | 0.16 | 425.25 | 8.00 | 1118.59 |
| 4 | 24 | canopy_unbatched | 0.75 | 624.00 | 0.88 | 15.00 | 0.00 | 595.97 | 64.00 | 1300.59 |
| 4 | 24 | canopy_schwartz_zippel | 0.34 | 334.15 | 0.47 | 3.09 | 0.41 | 464.22 | 8.00 | 810.69 |
| 4 | 24 | canopy_field_switch_192 | 0.34 | 331.54 | 0.22 | 2.95 | 0.00 | 458.41 | 6.00 | 799.47 |
| 8 | 24 | merkle_whir_independent | 1.25 | 1648.29 | 1.25 | 18.00 | 0.00 | 1073.92 | 64.00 | 2806.71 |
| 8 | 24 | merkle_whir_batched_after_second | 0.59 | 1314.46 | 0.59 | 4.22 | 0.41 | 813.17 | 8.00 | 2141.44 |
| 8 | 24 | canopy_unbatched | 1.50 | 1248.14 | 1.75 | 30.00 | 0.00 | 1191.96 | 128.00 | 2601.35 |
| 8 | 24 | canopy_schwartz_zippel | 0.59 | 623.58 | 0.84 | 4.22 | 0.91 | 896.23 | 8.00 | 1534.37 |
| 8 | 24 | canopy_field_switch_192 | 0.59 | 621.14 | 0.34 | 4.15 | 0.00 | 890.42 | 6.00 | 1522.64 |
| 4 | 26 | merkle_whir_independent | 0.75 | 996.08 | 0.75 | 9.75 | 0.00 | 531.99 | 32.00 | 1571.32 |
| 4 | 26 | merkle_whir_batched_after_second | 0.38 | 796.65 | 0.38 | 3.28 | 0.16 | 424.00 | 8.00 | 1232.83 |
| 4 | 26 | canopy_unbatched | 0.88 | 674.92 | 1.00 | 16.50 | 0.00 | 679.00 | 64.00 | 1436.30 |
| 4 | 26 | canopy_schwartz_zippel | 0.34 | 368.18 | 0.47 | 3.28 | 0.41 | 479.50 | 8.00 | 860.18 |
| 4 | 26 | canopy_field_switch_192 | 0.34 | 368.19 | 0.22 | 3.09 | 0.00 | 467.62 | 6.00 | 845.47 |
| 8 | 26 | merkle_whir_independent | 1.50 | 1992.59 | 1.50 | 19.50 | 0.00 | 1064.00 | 64.00 | 3143.09 |
| 8 | 26 | merkle_whir_batched_after_second | 0.62 | 1527.20 | 0.62 | 4.41 | 0.41 | 811.98 | 8.00 | 2353.24 |
| 8 | 26 | canopy_unbatched | 1.75 | 1349.40 | 2.00 | 33.00 | 0.00 | 1358.00 | 128.00 | 2872.15 |
| 8 | 26 | canopy_schwartz_zippel | 0.59 | 691.07 | 0.84 | 4.41 | 0.91 | 911.50 | 8.00 | 1617.32 |
| 8 | 26 | canopy_field_switch_192 | 0.59 | 691.53 | 0.34 | 4.29 | 0.00 | 899.62 | 6.00 | 1602.38 |
| 4 | 28 | merkle_whir_independent | 0.75 | 1096.32 | 0.75 | 10.50 | 0.00 | 599.00 | 32.00 | 1739.32 |
| 4 | 28 | merkle_whir_batched_after_second | 0.38 | 894.75 | 0.38 | 3.47 | 0.16 | 440.75 | 8.00 | 1347.88 |
| 4 | 28 | canopy_unbatched | 1.00 | 794.33 | 1.12 | 17.62 | 0.00 | 701.00 | 48.00 | 1563.08 |
| 4 | 28 | canopy_schwartz_zippel | 0.38 | 418.79 | 0.50 | 3.47 | 0.41 | 478.75 | 8.00 | 910.29 |
| 4 | 28 | canopy_field_switch_192 | 0.38 | 425.81 | 0.24 | 3.23 | 0.00 | 459.56 | 12.00 | 901.22 |
| 8 | 28 | merkle_whir_independent | 1.50 | 2194.23 | 1.50 | 21.00 | 0.00 | 1198.00 | 64.00 | 3480.23 |
| 8 | 28 | merkle_whir_batched_after_second | 0.62 | 1721.70 | 0.62 | 4.59 | 0.41 | 828.75 | 8.00 | 2564.70 |
| 8 | 28 | canopy_unbatched | 2.00 | 1588.67 | 2.25 | 35.25 | 0.00 | 1401.98 | 96.00 | 3126.15 |
| 8 | 28 | canopy_schwartz_zippel | 0.62 | 773.97 | 0.88 | 4.59 | 0.91 | 910.73 | 8.00 | 1699.70 |
| 8 | 28 | canopy_field_switch_192 | 0.62 | 782.37 | 0.37 | 4.43 | 0.00 | 891.54 | 12.00 | 1691.33 |
| 4 | 30 | merkle_whir_independent | 0.75 | 1169.01 | 0.75 | 11.25 | 0.00 | 704.00 | 16.00 | 1901.76 |
| 4 | 30 | merkle_whir_batched_after_second | 0.38 | 996.87 | 0.38 | 3.66 | 0.16 | 452.50 | 8.00 | 1461.93 |
| 4 | 30 | canopy_unbatched | 1.00 | 863.80 | 1.12 | 18.75 | 0.00 | 752.00 | 48.00 | 1684.68 |
| 4 | 30 | canopy_schwartz_zippel | 0.38 | 457.23 | 0.50 | 3.66 | 0.41 | 488.50 | 8.00 | 958.67 |
| 4 | 30 | canopy_field_switch_192 | 0.38 | 460.87 | 0.24 | 3.38 | 0.00 | 465.94 | 24.00 | 954.80 |
| 8 | 30 | merkle_whir_independent | 1.50 | 2337.84 | 1.50 | 22.50 | 0.00 | 1408.00 | 32.00 | 3803.34 |
| 8 | 30 | merkle_whir_batched_after_second | 0.62 | 1921.95 | 0.62 | 4.78 | 0.41 | 840.50 | 8.00 | 2776.88 |
| 8 | 30 | canopy_unbatched | 2.00 | 1726.92 | 2.25 | 37.50 | 0.00 | 1504.00 | 96.00 | 3368.67 |
| 8 | 30 | canopy_schwartz_zippel | 0.62 | 846.90 | 0.88 | 4.78 | 0.91 | 920.50 | 8.00 | 1782.59 |
| 8 | 30 | canopy_field_switch_192 | 0.62 | 850.23 | 0.37 | 4.57 | 0.00 | 897.94 | 24.00 | 1777.73 |

## Oracle queries at n=30

`unique` is the mean number of distinct positions per queried oracle group; authentication and answer columns are total transmitted values across the reported oracle count.

| exp | t | scheme | role | oracles | code | field | requested | unique | auth | answer fields |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|---:|
| 0 | 4 | merkle_whir_batched_after_second | SZ-batched second-oracle claim initial oracle | 4 | [2^30,2^27] | 256 | 90 | 90.00 | 7071.68 | 2880.00 |
| 0 | 4 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^29,2^24] | 256 | 53 | 53.00 | 974.80 | 848.00 |
| 0 | 4 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^28,2^20] | 256 | 33 | 33.00 | 596.92 | 528.00 |
| 0 | 4 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^27,2^16] | 256 | 24 | 24.00 | 420.72 | 384.00 |
| 0 | 4 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^26,2^12] | 256 | 19 | 19.00 | 321.25 | 304.00 |
| 0 | 4 | merkle_whir_batched_after_second | independent first Merkle oracles | 4 | [2^31,2^30] | 256 | 298 | 298.00 | 22514.52 | 9536.00 |
| 0 | 8 | merkle_whir_batched_after_second | SZ-batched second-oracle claim initial oracle | 8 | [2^30,2^27] | 256 | 90 | 90.00 | 14130.88 | 5760.00 |
| 0 | 8 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^29,2^24] | 256 | 53 | 53.00 | 974.92 | 848.00 |
| 0 | 8 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^28,2^20] | 256 | 33 | 33.00 | 595.34 | 528.00 |
| 0 | 8 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^27,2^16] | 256 | 24 | 24.00 | 421.78 | 384.00 |
| 0 | 8 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^26,2^12] | 256 | 19 | 19.00 | 321.92 | 304.00 |
| 0 | 8 | merkle_whir_batched_after_second | independent first Merkle oracles | 8 | [2^31,2^30] | 256 | 298 | 298.00 | 45057.44 | 19072.00 |
| 0 | 4 | merkle_whir_independent | independent WHIR initial oracle | 4 | [2^31,2^30] | 256 | 298 | 298.00 | 22508.48 | 9536.00 |
| 0 | 4 | merkle_whir_independent | independent WHIR recursive oracle 1 | 4 | [2^30,2^27] | 256 | 90 | 90.00 | 6708.64 | 5760.00 |
| 0 | 4 | merkle_whir_independent | independent WHIR recursive oracle 2 | 4 | [2^29,2^23] | 256 | 44 | 44.00 | 3290.08 | 2816.00 |
| 0 | 4 | merkle_whir_independent | independent WHIR recursive oracle 3 | 4 | [2^28,2^19] | 256 | 29 | 29.00 | 2117.64 | 1856.00 |
| 0 | 4 | merkle_whir_independent | independent WHIR recursive oracle 4 | 4 | [2^27,2^15] | 256 | 22 | 22.00 | 1557.68 | 1408.00 |
| 0 | 4 | merkle_whir_independent | independent WHIR recursive oracle 5 | 4 | [2^26,2^11] | 256 | 18 | 18.00 | 1225.84 | 1152.00 |
| 0 | 8 | merkle_whir_independent | independent WHIR initial oracle | 8 | [2^31,2^30] | 256 | 298 | 298.00 | 45031.44 | 19072.00 |
| 0 | 8 | merkle_whir_independent | independent WHIR recursive oracle 1 | 8 | [2^30,2^27] | 256 | 90 | 90.00 | 13425.20 | 11520.00 |
| 0 | 8 | merkle_whir_independent | independent WHIR recursive oracle 2 | 8 | [2^29,2^23] | 256 | 44 | 44.00 | 6564.48 | 5632.00 |
| 0 | 8 | merkle_whir_independent | independent WHIR recursive oracle 3 | 8 | [2^28,2^19] | 256 | 29 | 29.00 | 4235.20 | 3712.00 |
| 0 | 8 | merkle_whir_independent | independent WHIR recursive oracle 4 | 8 | [2^27,2^15] | 256 | 22 | 22.00 | 3107.44 | 2816.00 |
| 0 | 8 | merkle_whir_independent | independent WHIR recursive oracle 5 | 8 | [2^26,2^11] | 256 | 18 | 18.00 | 2447.20 | 2304.00 |
| 2 | 4 | canopy_unbatched | Canopy claim WHIR initial oracle | 4 | [2^31,2^20] | 256 | 24 | 24.00 | 2072.76 | 1536.00 |
| 2 | 4 | canopy_unbatched | Canopy claim WHIR recursive oracle 1 | 4 | [2^30,2^16] | 256 | 19 | 19.00 | 1583.72 | 1216.00 |
| 2 | 4 | canopy_unbatched | Canopy claim WHIR recursive oracle 2 | 4 | [2^29,2^12] | 256 | 16 | 16.00 | 1287.96 | 1024.00 |
| 2 | 4 | canopy_unbatched | outer first oracle via Canopy | 4 | [2^31,2^30] | 256 | 298 | 298.00 | 9535.60 | 9536.00 |
| 2 | 4 | canopy_unbatched | outer second-oracle WHIR initial oracle | 4 | [2^31,2^27] | 256 | 67 | 67.00 | 5377.48 | 4288.00 |
| 2 | 4 | canopy_unbatched | outer second-oracle WHIR recursive oracle 1 | 4 | [2^30,2^23] | 256 | 38 | 38.00 | 3029.08 | 2432.00 |
| 2 | 4 | canopy_unbatched | outer second-oracle WHIR recursive oracle 2 | 4 | [2^29,2^19] | 256 | 26 | 26.00 | 2019.00 | 1664.00 |
| 2 | 4 | canopy_unbatched | outer second-oracle WHIR recursive oracle 3 | 4 | [2^28,2^15] | 256 | 20 | 20.00 | 1508.12 | 1280.00 |
| 2 | 4 | canopy_unbatched | outer second-oracle WHIR recursive oracle 4 | 4 | [2^27,2^11] | 256 | 17 | 17.00 | 1227.92 | 1088.00 |
| 2 | 8 | canopy_unbatched | Canopy claim WHIR initial oracle | 8 | [2^31,2^20] | 256 | 24 | 24.00 | 4145.52 | 3072.00 |
| 2 | 8 | canopy_unbatched | Canopy claim WHIR recursive oracle 1 | 8 | [2^30,2^16] | 256 | 19 | 19.00 | 3167.44 | 2432.00 |
| 2 | 8 | canopy_unbatched | Canopy claim WHIR recursive oracle 2 | 8 | [2^29,2^12] | 256 | 16 | 16.00 | 2575.92 | 2048.00 |
| 2 | 8 | canopy_unbatched | outer first oracle via Canopy | 8 | [2^31,2^30] | 256 | 298 | 298.00 | 19070.80 | 19072.00 |
| 2 | 8 | canopy_unbatched | outer second-oracle WHIR initial oracle | 8 | [2^31,2^27] | 256 | 67 | 67.00 | 10742.80 | 8576.00 |
| 2 | 8 | canopy_unbatched | outer second-oracle WHIR recursive oracle 1 | 8 | [2^30,2^23] | 256 | 38 | 38.00 | 6047.04 | 4864.00 |
| 2 | 8 | canopy_unbatched | outer second-oracle WHIR recursive oracle 2 | 8 | [2^29,2^19] | 256 | 26 | 26.00 | 4041.44 | 3328.00 |
| 2 | 8 | canopy_unbatched | outer second-oracle WHIR recursive oracle 3 | 8 | [2^28,2^15] | 256 | 20 | 20.00 | 3009.20 | 2560.00 |
| 2 | 8 | canopy_unbatched | outer second-oracle WHIR recursive oracle 4 | 8 | [2^27,2^11] | 256 | 17 | 17.00 | 2461.20 | 2176.00 |
| 3 | 4 | canopy_schwartz_zippel | SZ-batched aligned claims initial oracle | 8 | [2^31,2^27] | 256 | 67 | 67.00 | 11279.04 | 4288.00 |
| 3 | 4 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 1 | 1 | [2^30,2^24] | 256 | 44 | 44.00 | 865.23 | 704.00 |
| 3 | 4 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 2 | 1 | [2^29,2^20] | 256 | 29 | 29.00 | 558.97 | 464.00 |
| 3 | 4 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 3 | 1 | [2^28,2^16] | 256 | 22 | 22.00 | 412.03 | 352.00 |
| 3 | 4 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 4 | 1 | [2^27,2^12] | 256 | 18 | 18.00 | 324.11 | 288.00 |
| 3 | 4 | canopy_schwartz_zippel | outer first oracle via Canopy | 4 | [2^31,2^30] | 256 | 298 | 298.00 | 1192.00 | 9536.00 |
| 3 | 8 | canopy_schwartz_zippel | SZ-batched aligned claims initial oracle | 16 | [2^31,2^27] | 256 | 67 | 67.00 | 22559.20 | 8576.00 |
| 3 | 8 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 1 | 1 | [2^30,2^24] | 256 | 44 | 44.00 | 864.80 | 704.00 |
| 3 | 8 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 2 | 1 | [2^29,2^20] | 256 | 29 | 29.00 | 558.15 | 464.00 |
| 3 | 8 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 3 | 1 | [2^28,2^16] | 256 | 22 | 22.00 | 410.98 | 352.00 |
| 3 | 8 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 4 | 1 | [2^27,2^12] | 256 | 18 | 18.00 | 323.69 | 288.00 |
| 3 | 8 | canopy_schwartz_zippel | outer first oracle via Canopy | 8 | [2^31,2^30] | 256 | 298 | 298.00 | 2384.00 | 19072.00 |
| 4 | 4 | canopy_field_switch_192 | GF(2^192) recursive oracle 1 | 1 | [2^30,2^24] | 192 | 44 | 44.00 | 864.64 | 704.00 |
| 4 | 4 | canopy_field_switch_192 | GF(2^192) recursive oracle 2 | 1 | [2^29,2^20] | 192 | 29 | 29.00 | 587.38 | 232.00 |
| 4 | 4 | canopy_field_switch_192 | GF(2^192) recursive oracle 3 | 1 | [2^28,2^17] | 192 | 24 | 24.00 | 468.67 | 192.00 |
| 4 | 4 | canopy_field_switch_192 | GF(2^192) recursive oracle 4 | 1 | [2^27,2^14] | 192 | 20 | 20.00 | 356.83 | 320.00 |
| 4 | 4 | canopy_field_switch_192 | field-switch source: 8 GF(2^256) roots / 32 GF(2^64) rows | 8 | [2^31,2^27] | 256 | 67 | 67.00 | 11278.40 | 4288.00 |
| 4 | 4 | canopy_field_switch_192 | outer first oracle via Canopy | 4 | [2^31,2^30] | 256 | 298 | 298.00 | 1192.00 | 9536.00 |
| 4 | 8 | canopy_field_switch_192 | GF(2^192) recursive oracle 1 | 1 | [2^30,2^24] | 192 | 44 | 44.00 | 866.13 | 704.00 |
| 4 | 8 | canopy_field_switch_192 | GF(2^192) recursive oracle 2 | 1 | [2^29,2^20] | 192 | 29 | 29.00 | 587.71 | 232.00 |
| 4 | 8 | canopy_field_switch_192 | GF(2^192) recursive oracle 3 | 1 | [2^28,2^17] | 192 | 24 | 24.00 | 469.42 | 192.00 |
| 4 | 8 | canopy_field_switch_192 | GF(2^192) recursive oracle 4 | 1 | [2^27,2^14] | 192 | 20 | 20.00 | 356.93 | 320.00 |
| 4 | 8 | canopy_field_switch_192 | field-switch source: 16 GF(2^256) roots / 64 GF(2^64) rows | 16 | [2^31,2^27] | 256 | 67 | 67.00 | 22543.04 | 8576.00 |
| 4 | 8 | canopy_field_switch_192 | outer first oracle via Canopy | 8 | [2^31,2^30] | 256 | 298 | 298.00 | 2384.00 | 19072.00 |

## Experiment 4 soundness events at n=30

| t | phase | event | bits |
|---:|---|---|---:|
| 4 | outer initial transition input 1 | initial folding / sumcheck | 202.30 |
| 4 | outer initial transition input 1 | initial OOD sampling | 218.36 |
| 4 | outer initial transition input 1 | oracle 0 proximity queries | 128.02 |
| 4 | outer initial transition input 2 | initial folding / sumcheck | 202.30 |
| 4 | outer initial transition input 2 | initial OOD sampling | 218.36 |
| 4 | outer initial transition input 2 | oracle 0 proximity queries | 128.02 |
| 4 | outer initial transition input 3 | initial folding / sumcheck | 202.30 |
| 4 | outer initial transition input 3 | initial OOD sampling | 218.36 |
| 4 | outer initial transition input 3 | oracle 0 proximity queries | 128.02 |
| 4 | outer initial transition input 4 | initial folding / sumcheck | 202.30 |
| 4 | outer initial transition input 4 | initial OOD sampling | 218.36 |
| 4 | outer initial transition input 4 | oracle 0 proximity queries | 128.02 |
| 4 | field switch | one-round 32-coordinate random projection | 179.72 |
| 4 | GF(2^192) recursive WHIR | initial folding / sumcheck | 128.80 |
| 4 | GF(2^192) recursive WHIR | initial OOD sampling | 146.36 |
| 4 | GF(2^192) recursive WHIR | oracle 0 proximity queries | 129.28 |
| 4 | GF(2^192) recursive WHIR | oracle 1 proximity queries | 128.90 |
| 4 | GF(2^192) recursive WHIR | oracle 2 proximity queries | 128.46 |
| 4 | GF(2^192) recursive WHIR | oracle 3 proximity queries | 130.31 |
| 4 | GF(2^192) recursive WHIR | oracle 4 proximity queries | 128.59 |
| 4 | GF(2^192) recursive WHIR | recursive round 1 folding / sumcheck | 131.80 |
| 4 | GF(2^192) recursive WHIR | recursive round 1 claim combination | 175.59 |
| 4 | GF(2^192) recursive WHIR | recursive round 1 OOD sampling | 150.36 |
| 4 | GF(2^192) recursive WHIR | recursive round 2 folding / sumcheck | 128.30 |
| 4 | GF(2^192) recursive WHIR | recursive round 2 claim combination | 173.19 |
| 4 | GF(2^192) recursive WHIR | recursive round 2 OOD sampling | 148.36 |
| 4 | GF(2^192) recursive WHIR | recursive round 3 folding / sumcheck | 126.30 |
| 4 | GF(2^192) recursive WHIR | recursive round 3 claim combination | 171.77 |
| 4 | GF(2^192) recursive WHIR | recursive round 3 OOD sampling | 147.36 |
| 4 | GF(2^192) recursive WHIR | recursive round 4 folding / sumcheck | 124.30 |
| 4 | GF(2^192) recursive WHIR | recursive round 4 claim combination | 170.03 |
| 4 | GF(2^192) recursive WHIR | recursive round 4 OOD sampling | 146.36 |
| 4 | GF(2^192) recursive WHIR | terminal sumcheck | 191.00 |
| 8 | outer initial transition input 1 | initial folding / sumcheck | 202.30 |
| 8 | outer initial transition input 1 | initial OOD sampling | 218.36 |
| 8 | outer initial transition input 1 | oracle 0 proximity queries | 128.02 |
| 8 | outer initial transition input 2 | initial folding / sumcheck | 202.30 |
| 8 | outer initial transition input 2 | initial OOD sampling | 218.36 |
| 8 | outer initial transition input 2 | oracle 0 proximity queries | 128.02 |
| 8 | outer initial transition input 3 | initial folding / sumcheck | 202.30 |
| 8 | outer initial transition input 3 | initial OOD sampling | 218.36 |
| 8 | outer initial transition input 3 | oracle 0 proximity queries | 128.02 |
| 8 | outer initial transition input 4 | initial folding / sumcheck | 202.30 |
| 8 | outer initial transition input 4 | initial OOD sampling | 218.36 |
| 8 | outer initial transition input 4 | oracle 0 proximity queries | 128.02 |
| 8 | outer initial transition input 5 | initial folding / sumcheck | 202.30 |
| 8 | outer initial transition input 5 | initial OOD sampling | 218.36 |
| 8 | outer initial transition input 5 | oracle 0 proximity queries | 128.02 |
| 8 | outer initial transition input 6 | initial folding / sumcheck | 202.30 |
| 8 | outer initial transition input 6 | initial OOD sampling | 218.36 |
| 8 | outer initial transition input 6 | oracle 0 proximity queries | 128.02 |
| 8 | outer initial transition input 7 | initial folding / sumcheck | 202.30 |
| 8 | outer initial transition input 7 | initial OOD sampling | 218.36 |
| 8 | outer initial transition input 7 | oracle 0 proximity queries | 128.02 |
| 8 | outer initial transition input 8 | initial folding / sumcheck | 202.30 |
| 8 | outer initial transition input 8 | initial OOD sampling | 218.36 |
| 8 | outer initial transition input 8 | oracle 0 proximity queries | 128.02 |
| 8 | field switch | one-round 64-coordinate random projection | 178.70 |
| 8 | GF(2^192) recursive WHIR | initial folding / sumcheck | 127.80 |
| 8 | GF(2^192) recursive WHIR | initial OOD sampling | 145.36 |
| 8 | GF(2^192) recursive WHIR | oracle 0 proximity queries | 129.28 |
| 8 | GF(2^192) recursive WHIR | oracle 1 proximity queries | 128.90 |
| 8 | GF(2^192) recursive WHIR | oracle 2 proximity queries | 128.46 |
| 8 | GF(2^192) recursive WHIR | oracle 3 proximity queries | 130.31 |
| 8 | GF(2^192) recursive WHIR | oracle 4 proximity queries | 128.59 |
| 8 | GF(2^192) recursive WHIR | recursive round 1 folding / sumcheck | 131.80 |
| 8 | GF(2^192) recursive WHIR | recursive round 1 claim combination | 175.59 |
| 8 | GF(2^192) recursive WHIR | recursive round 1 OOD sampling | 150.36 |
| 8 | GF(2^192) recursive WHIR | recursive round 2 folding / sumcheck | 128.30 |
| 8 | GF(2^192) recursive WHIR | recursive round 2 claim combination | 173.19 |
| 8 | GF(2^192) recursive WHIR | recursive round 2 OOD sampling | 148.36 |
| 8 | GF(2^192) recursive WHIR | recursive round 3 folding / sumcheck | 126.30 |
| 8 | GF(2^192) recursive WHIR | recursive round 3 claim combination | 171.77 |
| 8 | GF(2^192) recursive WHIR | recursive round 3 OOD sampling | 147.36 |
| 8 | GF(2^192) recursive WHIR | recursive round 4 folding / sumcheck | 124.30 |
| 8 | GF(2^192) recursive WHIR | recursive round 4 claim combination | 170.03 |
| 8 | GF(2^192) recursive WHIR | recursive round 4 OOD sampling | 146.36 |
| 8 | GF(2^192) recursive WHIR | terminal sumcheck | 191.00 |
