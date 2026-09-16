# Batched WHIR and Canopy Exp3 (johnson)

Batched WHIR rates 1/2 and 1/4, and Canopy Exp3; GF(2^256), BLAKE3, 128-bit target. t=1,2,4,8; n=18,20,...,30. Fold logs 3/4 searched with terminal cap 10. 100 deterministic query samples, seed 0x50415045525f5631; Johnson decoding. Authentication counts are verifier leaf and parent calls; H2F is separate. WHIR proof models deduplicate query rows.

## Selected proof sizes

Both relative columns use the corresponding Merkle-WHIR baseline at the same `t,n`.

| t | log2 n | scheme | rate | batched sources | folds | h | terminal | min event bits | union bits | 128-bit RBR? | KiB | verifier auth hashes | H2F | vs batched | vs batched |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|:---:|---:|---:|---:|---:|---:|
| 1 | 18 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3 | - | 9 | 128.02 | 126.86 | yes | 225.91 | 4013.14 | 0.00 | 1.000x | 1.000x |
| 1 | 18 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,3 | - | 9 | 128.29 | 127.18 | yes | 149.79 | 2725.29 | 0.00 | 0.663x | 0.663x |
| 1 | 18 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3 | 4 | 9 | 128.02 | 127.05 | yes | 195.81 | 2541.18 | 296.78 | 0.867x | 0.867x |
| 2 | 18 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,3 | - | 9 | 128.02 | 126.33 | yes | 408.73 | 7524.94 | 0.00 | 1.000x | 1.000x |
| 2 | 18 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,3 | - | 9 | 128.29 | 126.63 | yes | 259.25 | 4982.87 | 0.00 | 0.634x | 0.634x |
| 2 | 18 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,3 | 4 | 9 | 128.02 | 126.46 | yes | 351.52 | 4618.94 | 593.48 | 0.860x | 0.860x |
| 4 | 18 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,3 | - | 9 | 128.02 | 125.64 | yes | 774.60 | 14553.43 | 0.00 | 1.000x | 1.000x |
| 4 | 18 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,3 | - | 9 | 128.29 | 125.92 | yes | 478.17 | 9495.23 | 0.00 | 0.617x | 0.617x |
| 4 | 18 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,3 | 4 | 9 | 128.02 | 125.71 | yes | 662.32 | 8760.41 | 1186.96 | 0.855x | 0.855x |
| 8 | 18 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,3 | - | 9 | 128.02 | 124.82 | yes | 1506.40 | 28604.15 | 0.00 | 1.000x | 1.000x |
| 8 | 18 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,3 | - | 9 | 128.29 | 125.09 | yes | 916.29 | 18528.36 | 0.00 | 0.608x | 0.608x |
| 8 | 18 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,3 | 4 | 9 | 128.02 | 124.86 | yes | 1284.51 | 17064.05 | 2371.52 | 0.853x | 0.853x |
| 1 | 20 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,3 | - | 8 | 128.02 | 126.74 | yes | 265.86 | 5306.94 | 0.00 | 1.000x | 1.000x |
| 1 | 20 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,3,3 | - | 8 | 128.29 | 126.94 | yes | 176.32 | 3622.41 | 0.00 | 0.663x | 0.663x |
| 1 | 20 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3,3 | 4 | 8 | 128.02 | 126.84 | yes | 218.12 | 3299.19 | 297.71 | 0.820x | 0.820x |
| 2 | 20 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,3,3 | - | 8 | 128.02 | 126.24 | yes | 473.49 | 9608.31 | 0.00 | 1.000x | 1.000x |
| 2 | 20 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,3,3 | - | 8 | 128.29 | 126.46 | yes | 298.61 | 6289.84 | 0.00 | 0.631x | 0.631x |
| 2 | 20 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,3,3 | 4 | 8 | 128.02 | 126.31 | yes | 381.90 | 5633.14 | 595.12 | 0.807x | 0.807x |
| 4 | 20 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,3,3 | - | 8 | 128.02 | 125.58 | yes | 887.65 | 18174.89 | 0.00 | 1.000x | 1.000x |
| 4 | 20 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,3,3 | - | 8 | 128.29 | 125.82 | yes | 543.22 | 11623.74 | 0.00 | 0.612x | 0.612x |
| 4 | 20 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,3,3 | 4 | 8 | 128.02 | 125.62 | yes | 710.26 | 10320.68 | 1190.92 | 0.800x | 0.800x |
| 8 | 20 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,3,3 | - | 8 | 128.02 | 124.79 | yes | 1716.51 | 35327.58 | 0.00 | 1.000x | 1.000x |
| 8 | 20 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,3,3 | - | 8 | 128.29 | 125.03 | yes | 1032.64 | 22297.55 | 0.00 | 0.602x | 0.602x |
| 8 | 20 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,3,3 | 4 | 8 | 128.02 | 124.81 | yes | 1366.83 | 19702.35 | 2380.88 | 0.796x | 0.796x |
| 1 | 22 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,4,4 | - | 8 | 128.02 | 126.67 | yes | 311.88 | 6114.86 | 0.00 | 1.000x | 1.000x |
| 1 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,4,4 | - | 8 | 128.29 | 126.68 | yes | 207.79 | 4063.63 | 0.00 | 0.666x | 0.666x |
| 1 | 22 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,4 | 4 | 8 | 128.02 | 126.59 | yes | 244.97 | 3591.79 | 297.91 | 0.785x | 0.785x |
| 2 | 22 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,4,4 | - | 8 | 128.02 | 126.19 | yes | 543.72 | 11190.10 | 0.00 | 1.000x | 1.000x |
| 2 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,4 | - | 8 | 128.29 | 126.27 | yes | 342.69 | 7134.39 | 0.00 | 0.630x | 0.630x |
| 2 | 22 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,4,4 | 4 | 8 | 128.02 | 126.14 | yes | 417.53 | 6203.60 | 595.88 | 0.768x | 0.768x |
| 4 | 22 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,4,4 | - | 8 | 128.02 | 125.55 | yes | 1007.28 | 21334.48 | 0.00 | 1.000x | 1.000x |
| 4 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,4,4 | - | 8 | 128.29 | 125.69 | yes | 612.80 | 13284.09 | 0.00 | 0.608x | 0.608x |
| 4 | 22 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,4,4 | 4 | 8 | 128.02 | 125.51 | yes | 762.48 | 11423.30 | 1191.72 | 0.757x | 0.757x |
| 8 | 22 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,4,4 | - | 8 | 128.02 | 124.77 | yes | 1932.73 | 41569.68 | 0.00 | 1.000x | 1.000x |
| 8 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,4,4 | - | 8 | 128.29 | 124.96 | yes | 1154.05 | 25615.44 | 0.00 | 0.597x | 0.597x |
| 8 | 22 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,4,4 | 4 | 8 | 128.02 | 124.75 | yes | 1452.72 | 21873.80 | 2383.04 | 0.752x | 0.752x |
| 1 | 24 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,3,4 | - | 8 | 128.02 | 126.35 | yes | 350.83 | 7603.77 | 0.00 | 1.000x | 1.000x |
| 1 | 24 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,3,3,4 | - | 8 | 128.17 | 126.43 | yes | 235.15 | 5126.05 | 0.00 | 0.670x | 0.670x |
| 1 | 24 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3,3,4 | 4 | 8 | 128.02 | 126.35 | yes | 268.22 | 4522.34 | 297.96 | 0.765x | 0.765x |
| 2 | 24 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,3,3,4 | - | 8 | 128.02 | 125.96 | yes | 606.86 | 13452.67 | 0.00 | 1.000x | 1.000x |
| 2 | 24 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,3,3 | - | 8 | 128.29 | 126.19 | yes | 383.17 | 8508.57 | 0.00 | 0.631x | 0.631x |
| 2 | 24 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,3,3,4 | 4 | 8 | 128.02 | 125.96 | yes | 449.01 | 7397.53 | 595.94 | 0.740x | 0.740x |
| 4 | 24 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,3,3,4 | - | 8 | 128.02 | 125.40 | yes | 1118.59 | 25137.83 | 0.00 | 1.000x | 1.000x |
| 4 | 24 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,3,3,4 | - | 8 | 128.17 | 125.56 | yes | 679.05 | 15590.46 | 0.00 | 0.607x | 0.607x |
| 4 | 24 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,3,3,4 | 4 | 8 | 128.02 | 125.40 | yes | 810.69 | 13151.85 | 1191.84 | 0.725x | 0.725x |
| 8 | 24 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,3,3,4 | - | 8 | 128.02 | 124.68 | yes | 2141.44 | 48491.08 | 0.00 | 1.000x | 1.000x |
| 8 | 24 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,3,3,4 | - | 8 | 128.17 | 124.88 | yes | 1271.07 | 29547.43 | 0.00 | 0.594x | 0.594x |
| 8 | 24 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,3,3,4 | 4 | 8 | 128.02 | 124.68 | yes | 1534.37 | 24669.64 | 2383.84 | 0.717x | 0.717x |
| 1 | 26 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,3,3,3 | - | 8 | 128.02 | 126.26 | yes | 392.74 | 9023.67 | 0.00 | 1.000x | 1.000x |
| 1 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,4,3 | - | 8 | 128.17 | 126.42 | yes | 264.00 | 5317.05 | 0.00 | 0.672x | 0.672x |
| 1 | 26 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,4,4 | 4 | 8 | 128.02 | 126.49 | yes | 292.66 | 4794.00 | 298.00 | 0.745x | 0.745x |
| 2 | 26 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,3,3,3,3 | - | 8 | 128.02 | 125.89 | yes | 672.81 | 15642.00 | 0.00 | 1.000x | 1.000x |
| 2 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,4,4 | - | 8 | 128.29 | 126.19 | yes | 424.67 | 9434.45 | 0.00 | 0.631x | 0.631x |
| 2 | 26 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,4,4,4 | 4 | 8 | 128.02 | 126.07 | yes | 481.91 | 7940.12 | 595.98 | 0.716x | 0.716x |
| 4 | 26 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,3,3,3,3 | - | 8 | 128.02 | 125.35 | yes | 1232.83 | 28872.69 | 0.00 | 1.000x | 1.000x |
| 4 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,4,4,4 | - | 8 | 128.29 | 125.64 | yes | 746.24 | 17230.68 | 0.00 | 0.605x | 0.605x |
| 4 | 26 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,4,4,4 | 4 | 8 | 128.02 | 125.47 | yes | 860.18 | 14224.91 | 1192.00 | 0.698x | 0.698x |
| 8 | 26 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,3,3,3,3 | - | 8 | 128.02 | 124.65 | yes | 2353.24 | 55346.27 | 0.00 | 1.000x | 1.000x |
| 8 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,4,4,4 | - | 8 | 128.29 | 124.93 | yes | 1389.76 | 32835.93 | 0.00 | 0.591x | 0.591x |
| 8 | 26 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,4,4,4 | 4 | 8 | 128.02 | 124.72 | yes | 1617.32 | 26813.39 | 2384.00 | 0.687x | 0.687x |
| 1 | 28 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,4,3,4,3 | - | 8 | 128.02 | 125.98 | yes | 434.90 | 9806.86 | 0.00 | 1.000x | 1.000x |
| 1 | 28 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,3,3,3 | - | 8 | 128.17 | 126.63 | yes | 292.97 | 6316.97 | 0.00 | 0.674x | 0.674x |
| 1 | 28 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,4,3,3 | 4 | 8 | 128.02 | 126.45 | yes | 317.07 | 5628.40 | 298.00 | 0.729x | 0.729x |
| 2 | 28 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,4,3,4,3 | - | 8 | 128.02 | 125.66 | yes | 739.11 | 17197.68 | 0.00 | 1.000x | 1.000x |
| 2 | 28 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,4,3,3 | - | 8 | 128.29 | 126.16 | yes | 466.58 | 10828.47 | 0.00 | 0.631x | 0.631x |
| 2 | 28 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,4,4,3,3 | 4 | 8 | 128.02 | 126.04 | yes | 514.72 | 9042.88 | 596.00 | 0.696x | 0.696x |
| 4 | 28 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,4,3,4,3 | - | 8 | 128.02 | 125.19 | yes | 1347.88 | 31988.07 | 0.00 | 1.000x | 1.000x |
| 4 | 28 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,4,4,3,3 | - | 8 | 128.29 | 125.62 | yes | 813.89 | 19448.52 | 0.00 | 0.604x | 0.604x |
| 4 | 28 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,4,4,3,3 | 4 | 8 | 128.02 | 125.45 | yes | 910.29 | 15881.25 | 1192.00 | 0.675x | 0.675x |
| 8 | 28 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,4,3,4,3 | - | 8 | 128.02 | 124.55 | yes | 2564.70 | 61546.36 | 0.00 | 1.000x | 1.000x |
| 8 | 28 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,4,4,3,3 | - | 8 | 128.29 | 124.91 | yes | 1506.91 | 36637.27 | 0.00 | 0.588x | 0.588x |
| 8 | 28 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,4,4,3,3 | 4 | 8 | 128.02 | 124.71 | yes | 1699.70 | 29502.96 | 2383.92 | 0.663x | 0.663x |
| 1 | 30 | merkle_whir_batched_after_second | 1/2 | 1 | 3,4,4,4,4,4 | - | 7 | 128.02 | 126.38 | yes | 475.50 | 10349.87 | 0.00 | 1.000x | 1.000x |
| 1 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,4,4,4 | - | 7 | 128.17 | 126.42 | yes | 318.97 | 6791.06 | 0.00 | 0.671x | 0.671x |
| 1 | 30 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,4,4,4 | 4 | 8 | 128.02 | 126.48 | yes | 340.76 | 6066.19 | 298.00 | 0.717x | 0.717x |
| 2 | 30 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,4,4,4,4 | - | 8 | 128.02 | 126.08 | yes | 804.82 | 18912.25 | 0.00 | 1.000x | 1.000x |
| 2 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,4,4,4 | - | 8 | 128.29 | 126.19 | yes | 507.43 | 11815.61 | 0.00 | 0.630x | 0.630x |
| 2 | 30 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,4,4,4,4 | 4 | 8 | 128.02 | 126.06 | yes | 546.81 | 9750.04 | 596.00 | 0.679x | 0.679x |
| 4 | 30 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,4,4,4,4 | - | 8 | 128.02 | 125.48 | yes | 1461.93 | 35249.89 | 0.00 | 1.000x | 1.000x |
| 4 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,4,4,4,4 | - | 8 | 128.29 | 125.63 | yes | 880.21 | 21250.75 | 0.00 | 0.602x | 0.602x |
| 4 | 30 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,4,4,4,4 | 4 | 8 | 128.02 | 125.46 | yes | 958.67 | 17109.38 | 1192.00 | 0.656x | 0.656x |
| 8 | 30 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,4,4,4,4 | - | 8 | 128.02 | 124.73 | yes | 2776.88 | 67948.28 | 0.00 | 1.000x | 1.000x |
| 8 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,4,4,4,4 | - | 8 | 128.29 | 124.92 | yes | 1624.38 | 40076.30 | 0.00 | 0.585x | 0.585x |
| 8 | 30 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,4,4,4,4 | 4 | 8 | 128.02 | 124.72 | yes | 1782.59 | 31834.82 | 2384.00 | 0.642x | 0.642x |

## Component breakdown

Size entries are KiB; authentication and H2F columns count hash calls.

| t | log2 n | scheme | roots | auth | OOD | sumcheck | batching | answers | coefficients | total | verifier auth hashes | H2F |
|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 18 | merkle_whir_batched_after_second | 0.09 | 97.99 | 0.09 | 1.69 | 0.00 | 110.05 | 16.00 | 225.91 | 4013.14 | 0.00 |
| 1 | 18 | merkle_whir_batched_rate_quarter_after_second | 0.09 | 69.71 | 0.09 | 1.69 | 0.00 | 62.21 | 16.00 | 149.79 | 2725.29 | 0.00 |
| 1 | 18 | canopy_schwartz_zippel | 0.09 | 59.06 | 0.12 | 1.69 | 0.03 | 118.81 | 16.00 | 195.81 | 2541.18 | 296.78 |
| 2 | 18 | merkle_whir_batched_after_second | 0.16 | 183.61 | 0.16 | 1.97 | 0.03 | 206.81 | 16.00 | 408.73 | 7524.94 | 0.00 |
| 2 | 18 | merkle_whir_batched_rate_quarter_after_second | 0.16 | 127.51 | 0.16 | 1.97 | 0.03 | 113.42 | 16.00 | 259.25 | 4982.87 | 0.00 |
| 2 | 18 | canopy_schwartz_zippel | 0.16 | 106.33 | 0.22 | 1.97 | 0.16 | 226.69 | 16.00 | 351.52 | 4618.94 | 593.48 |
| 4 | 18 | merkle_whir_batched_after_second | 0.28 | 354.98 | 0.28 | 2.53 | 0.16 | 400.37 | 16.00 | 774.60 | 14553.43 | 0.00 |
| 4 | 18 | merkle_whir_batched_rate_quarter_after_second | 0.28 | 243.04 | 0.28 | 2.53 | 0.16 | 215.88 | 16.00 | 478.17 | 9495.23 | 0.00 |
| 4 | 18 | canopy_schwartz_zippel | 0.28 | 200.52 | 0.41 | 2.53 | 0.41 | 442.18 | 16.00 | 662.32 | 8760.41 | 1186.96 |
| 8 | 18 | merkle_whir_batched_after_second | 0.53 | 697.46 | 0.53 | 3.66 | 0.41 | 787.81 | 16.00 | 1506.40 | 28604.15 | 0.00 |
| 8 | 18 | merkle_whir_batched_rate_quarter_after_second | 0.53 | 474.34 | 0.53 | 3.66 | 0.41 | 420.82 | 16.00 | 916.29 | 18528.36 | 0.00 |
| 8 | 18 | canopy_schwartz_zippel | 0.53 | 389.38 | 0.78 | 3.66 | 0.91 | 873.25 | 16.00 | 1284.51 | 17064.05 | 2371.52 |
| 1 | 20 | merkle_whir_batched_after_second | 0.12 | 136.04 | 0.12 | 1.88 | 0.00 | 119.69 | 8.00 | 265.86 | 5306.94 | 0.00 |
| 1 | 20 | merkle_whir_batched_rate_quarter_after_second | 0.12 | 95.70 | 0.12 | 1.88 | 0.00 | 70.49 | 8.00 | 176.32 | 3622.41 | 0.00 |
| 1 | 20 | canopy_schwartz_zippel | 0.12 | 80.72 | 0.16 | 1.88 | 0.03 | 127.20 | 8.00 | 218.12 | 3299.19 | 297.71 |
| 2 | 20 | merkle_whir_batched_after_second | 0.19 | 246.29 | 0.19 | 2.16 | 0.03 | 216.63 | 8.00 | 473.49 | 9608.31 | 0.00 |
| 2 | 20 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 166.31 | 0.19 | 2.16 | 0.03 | 121.74 | 8.00 | 298.61 | 6289.84 | 0.00 |
| 2 | 20 | canopy_schwartz_zippel | 0.19 | 136.03 | 0.25 | 2.16 | 0.16 | 235.12 | 8.00 | 381.90 | 5633.14 | 595.12 |
| 4 | 20 | merkle_whir_batched_after_second | 0.31 | 465.65 | 0.31 | 2.72 | 0.16 | 410.50 | 8.00 | 887.65 | 18174.89 | 0.00 |
| 4 | 20 | merkle_whir_batched_rate_quarter_after_second | 0.31 | 307.50 | 0.31 | 2.72 | 0.16 | 224.22 | 8.00 | 543.22 | 11623.74 | 0.00 |
| 4 | 20 | canopy_schwartz_zippel | 0.31 | 247.24 | 0.44 | 2.72 | 0.41 | 451.14 | 8.00 | 710.26 | 10320.68 | 1190.92 |
| 8 | 20 | merkle_whir_batched_after_second | 0.56 | 905.02 | 0.56 | 3.84 | 0.41 | 798.11 | 8.00 | 1716.51 | 35327.58 | 0.00 |
| 8 | 20 | merkle_whir_batched_rate_quarter_after_second | 0.56 | 590.06 | 0.56 | 3.84 | 0.41 | 429.21 | 8.00 | 1032.64 | 22297.55 | 0.00 |
| 8 | 20 | canopy_schwartz_zippel | 0.56 | 469.92 | 0.81 | 3.84 | 0.91 | 882.79 | 8.00 | 1366.83 | 19702.35 | 2380.88 |
| 1 | 22 | merkle_whir_batched_after_second | 0.12 | 161.60 | 0.12 | 2.06 | 0.00 | 139.97 | 8.00 | 311.88 | 6114.86 | 0.00 |
| 1 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.12 | 109.74 | 0.12 | 2.06 | 0.00 | 87.74 | 8.00 | 207.79 | 4063.63 | 0.00 |
| 1 | 22 | canopy_schwartz_zippel | 0.12 | 90.12 | 0.16 | 2.06 | 0.03 | 144.47 | 8.00 | 244.97 | 3591.79 | 297.91 |
| 2 | 22 | merkle_whir_batched_after_second | 0.19 | 296.01 | 0.19 | 2.34 | 0.03 | 236.96 | 8.00 | 543.72 | 11190.10 | 0.00 |
| 2 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 192.95 | 0.19 | 2.34 | 0.03 | 138.99 | 8.00 | 342.69 | 7134.39 | 0.00 |
| 2 | 22 | canopy_schwartz_zippel | 0.19 | 154.11 | 0.25 | 2.34 | 0.16 | 252.49 | 8.00 | 417.53 | 6203.60 | 595.88 |
| 4 | 22 | merkle_whir_batched_after_second | 0.31 | 564.65 | 0.31 | 2.91 | 0.16 | 430.94 | 8.00 | 1007.28 | 21334.48 | 0.00 |
| 4 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.31 | 359.63 | 0.31 | 2.91 | 0.16 | 241.49 | 8.00 | 612.80 | 13284.09 | 0.00 |
| 4 | 22 | canopy_schwartz_zippel | 0.31 | 281.98 | 0.44 | 2.91 | 0.41 | 468.44 | 8.00 | 762.48 | 11423.30 | 1191.72 |
| 8 | 22 | merkle_whir_batched_after_second | 0.56 | 1100.26 | 0.56 | 4.03 | 0.41 | 818.90 | 8.00 | 1932.73 | 41569.68 | 0.00 |
| 8 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.56 | 693.98 | 0.56 | 4.03 | 0.41 | 446.50 | 8.00 | 1154.05 | 25615.44 | 0.00 |
| 8 | 22 | canopy_schwartz_zippel | 0.56 | 538.05 | 0.81 | 4.03 | 0.91 | 900.36 | 8.00 | 1452.72 | 21873.80 | 2383.04 |
| 1 | 24 | merkle_whir_batched_after_second | 0.16 | 206.03 | 0.16 | 2.25 | 0.00 | 134.25 | 8.00 | 350.83 | 7603.77 | 0.00 |
| 1 | 24 | merkle_whir_batched_rate_quarter_after_second | 0.16 | 141.10 | 0.16 | 2.25 | 0.00 | 83.50 | 8.00 | 235.15 | 5126.05 | 0.00 |
| 1 | 24 | canopy_schwartz_zippel | 0.16 | 117.35 | 0.19 | 2.25 | 0.03 | 140.24 | 8.00 | 268.22 | 4522.34 | 297.96 |
| 2 | 24 | merkle_whir_batched_after_second | 0.22 | 364.62 | 0.22 | 2.53 | 0.03 | 231.25 | 8.00 | 606.86 | 13452.67 | 0.00 |
| 2 | 24 | merkle_whir_batched_rate_quarter_after_second | 0.22 | 234.42 | 0.22 | 2.53 | 0.03 | 137.75 | 8.00 | 383.17 | 8508.57 | 0.00 |
| 2 | 24 | canopy_schwartz_zippel | 0.22 | 189.58 | 0.28 | 2.53 | 0.16 | 248.24 | 8.00 | 449.01 | 7397.53 | 595.94 |
| 4 | 24 | merkle_whir_batched_after_second | 0.34 | 681.40 | 0.34 | 3.09 | 0.16 | 425.25 | 8.00 | 1118.59 | 25137.83 | 0.00 |
| 4 | 24 | merkle_whir_batched_rate_quarter_after_second | 0.34 | 429.86 | 0.34 | 3.09 | 0.16 | 237.25 | 8.00 | 679.05 | 15590.46 | 0.00 |
| 4 | 24 | canopy_schwartz_zippel | 0.34 | 334.15 | 0.47 | 3.09 | 0.41 | 464.22 | 8.00 | 810.69 | 13151.85 | 1191.84 |
| 8 | 24 | merkle_whir_batched_after_second | 0.59 | 1314.46 | 0.59 | 4.22 | 0.41 | 813.17 | 8.00 | 2141.44 | 48491.08 | 0.00 |
| 8 | 24 | merkle_whir_batched_rate_quarter_after_second | 0.59 | 815.01 | 0.59 | 4.22 | 0.41 | 442.25 | 8.00 | 1271.07 | 29547.43 | 0.00 |
| 8 | 24 | canopy_schwartz_zippel | 0.59 | 623.58 | 0.84 | 4.22 | 0.91 | 896.23 | 8.00 | 1534.37 | 24669.64 | 2383.84 |
| 1 | 26 | merkle_whir_batched_after_second | 0.19 | 248.93 | 0.19 | 2.44 | 0.00 | 133.00 | 8.00 | 392.74 | 9023.67 | 0.00 |
| 1 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.16 | 148.25 | 0.16 | 2.44 | 0.00 | 105.00 | 8.00 | 264.00 | 5317.05 | 0.00 |
| 1 | 26 | canopy_schwartz_zippel | 0.16 | 126.34 | 0.19 | 2.44 | 0.03 | 155.50 | 8.00 | 292.66 | 4794.00 | 298.00 |
| 2 | 26 | merkle_whir_batched_after_second | 0.25 | 431.56 | 0.25 | 2.72 | 0.03 | 230.00 | 8.00 | 672.81 | 15642.00 | 0.00 |
| 2 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.22 | 263.48 | 0.22 | 2.72 | 0.03 | 150.00 | 8.00 | 424.67 | 9434.45 | 0.00 |
| 2 | 26 | canopy_schwartz_zippel | 0.22 | 207.04 | 0.28 | 2.72 | 0.16 | 263.50 | 8.00 | 481.91 | 7940.12 | 595.98 |
| 4 | 26 | merkle_whir_batched_after_second | 0.38 | 796.65 | 0.38 | 3.28 | 0.16 | 424.00 | 8.00 | 1232.83 | 28872.69 | 0.00 |
| 4 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.34 | 481.62 | 0.34 | 3.28 | 0.16 | 252.50 | 8.00 | 746.24 | 17230.68 | 0.00 |
| 4 | 26 | canopy_schwartz_zippel | 0.34 | 368.18 | 0.47 | 3.28 | 0.41 | 479.50 | 8.00 | 860.18 | 14224.91 | 1192.00 |
| 8 | 26 | merkle_whir_batched_after_second | 0.62 | 1527.20 | 0.62 | 4.41 | 0.41 | 811.98 | 8.00 | 2353.24 | 55346.27 | 0.00 |
| 8 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.59 | 918.28 | 0.59 | 4.41 | 0.41 | 457.48 | 8.00 | 1389.76 | 32835.93 | 0.00 |
| 8 | 26 | canopy_schwartz_zippel | 0.59 | 691.07 | 0.84 | 4.41 | 0.91 | 911.50 | 8.00 | 1617.32 | 26813.39 | 2384.00 |
| 1 | 28 | merkle_whir_batched_after_second | 0.19 | 274.15 | 0.19 | 2.62 | 0.00 | 149.75 | 8.00 | 434.90 | 9806.86 | 0.00 |
| 1 | 28 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 178.22 | 0.19 | 2.62 | 0.00 | 103.75 | 8.00 | 292.97 | 6316.97 | 0.00 |
| 1 | 28 | canopy_schwartz_zippel | 0.19 | 151.26 | 0.22 | 2.62 | 0.03 | 154.75 | 8.00 | 317.07 | 5628.40 | 298.00 |
| 2 | 28 | merkle_whir_batched_after_second | 0.25 | 480.93 | 0.25 | 2.91 | 0.03 | 246.75 | 8.00 | 739.11 | 17197.68 | 0.00 |
| 2 | 28 | merkle_whir_batched_rate_quarter_after_second | 0.25 | 305.89 | 0.25 | 2.91 | 0.03 | 149.25 | 8.00 | 466.58 | 10828.47 | 0.00 |
| 2 | 28 | canopy_schwartz_zippel | 0.25 | 240.34 | 0.31 | 2.91 | 0.16 | 262.75 | 8.00 | 514.72 | 9042.88 | 596.00 |
| 4 | 28 | merkle_whir_batched_after_second | 0.38 | 894.75 | 0.38 | 3.47 | 0.16 | 440.75 | 8.00 | 1347.88 | 31988.07 | 0.00 |
| 4 | 28 | merkle_whir_batched_rate_quarter_after_second | 0.38 | 549.77 | 0.38 | 3.47 | 0.16 | 251.75 | 8.00 | 813.89 | 19448.52 | 0.00 |
| 4 | 28 | canopy_schwartz_zippel | 0.38 | 418.79 | 0.50 | 3.47 | 0.41 | 478.75 | 8.00 | 910.29 | 15881.25 | 1192.00 |
| 8 | 28 | merkle_whir_batched_after_second | 0.62 | 1721.70 | 0.62 | 4.59 | 0.41 | 828.75 | 8.00 | 2564.70 | 61546.36 | 0.00 |
| 8 | 28 | merkle_whir_batched_rate_quarter_after_second | 0.62 | 1035.91 | 0.62 | 4.59 | 0.41 | 456.75 | 8.00 | 1506.91 | 36637.27 | 0.00 |
| 8 | 28 | canopy_schwartz_zippel | 0.62 | 773.97 | 0.88 | 4.59 | 0.91 | 910.73 | 8.00 | 1699.70 | 29502.96 | 2383.92 |
| 1 | 30 | merkle_whir_batched_after_second | 0.19 | 292.31 | 0.19 | 2.81 | 0.00 | 176.00 | 4.00 | 475.50 | 10349.87 | 0.00 |
| 1 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 193.28 | 0.19 | 2.81 | 0.00 | 118.50 | 4.00 | 318.97 | 6791.06 | 0.00 |
| 1 | 30 | canopy_schwartz_zippel | 0.19 | 165.01 | 0.22 | 2.81 | 0.03 | 164.50 | 8.00 | 340.76 | 6066.19 | 298.00 |
| 2 | 30 | merkle_whir_batched_after_second | 0.25 | 534.70 | 0.25 | 3.09 | 0.03 | 258.50 | 8.00 | 804.82 | 18912.25 | 0.00 |
| 2 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.25 | 336.80 | 0.25 | 3.09 | 0.03 | 159.00 | 8.00 | 507.43 | 11815.61 | 0.00 |
| 2 | 30 | canopy_schwartz_zippel | 0.25 | 262.50 | 0.31 | 3.09 | 0.16 | 272.50 | 8.00 | 546.81 | 9750.04 | 596.00 |
| 4 | 30 | merkle_whir_batched_after_second | 0.38 | 996.87 | 0.38 | 3.66 | 0.16 | 452.50 | 8.00 | 1461.93 | 35249.89 | 0.00 |
| 4 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.38 | 606.15 | 0.38 | 3.66 | 0.16 | 261.50 | 8.00 | 880.21 | 21250.75 | 0.00 |
| 4 | 30 | canopy_schwartz_zippel | 0.38 | 457.23 | 0.50 | 3.66 | 0.41 | 488.50 | 8.00 | 958.67 | 17109.38 | 1192.00 |
| 8 | 30 | merkle_whir_batched_after_second | 0.62 | 1921.95 | 0.62 | 4.78 | 0.41 | 840.50 | 8.00 | 2776.88 | 67948.28 | 0.00 |
| 8 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.62 | 1143.45 | 0.62 | 4.78 | 0.41 | 466.50 | 8.00 | 1624.38 | 40076.30 | 0.00 |
| 8 | 30 | canopy_schwartz_zippel | 0.62 | 846.90 | 0.88 | 4.78 | 0.91 | 920.50 | 8.00 | 1782.59 | 31834.82 | 2384.00 |

## Oracle queries at n=30

`unique` is the mean number of distinct positions per queried oracle group; authentication and answer columns are total transmitted values across the reported oracle count.

| exp | t | scheme | role | oracles | code | field | requested | unique | auth | answer fields |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | independent WHIR initial oracle | 1 | [2^31,2^30] | 256 | 298 | 298.00 | 5630.14 | 2384.00 |
| 0 | 1 | merkle_whir_batched_after_second | independent WHIR recursive oracle 1 | 1 | [2^30,2^27] | 256 | 90 | 90.00 | 1677.21 | 1440.00 |
| 0 | 1 | merkle_whir_batched_after_second | independent WHIR recursive oracle 2 | 1 | [2^29,2^23] | 256 | 44 | 44.00 | 821.84 | 704.00 |
| 0 | 1 | merkle_whir_batched_after_second | independent WHIR recursive oracle 3 | 1 | [2^28,2^19] | 256 | 29 | 29.00 | 530.15 | 464.00 |
| 0 | 1 | merkle_whir_batched_after_second | independent WHIR recursive oracle 4 | 1 | [2^27,2^15] | 256 | 22 | 22.00 | 389.41 | 352.00 |
| 0 | 1 | merkle_whir_batched_after_second | independent WHIR recursive oracle 5 | 1 | [2^26,2^11] | 256 | 18 | 18.00 | 305.12 | 288.00 |
| 0 | 2 | merkle_whir_batched_after_second | SZ-batched second-oracle claim initial oracle | 2 | [2^30,2^27] | 256 | 90 | 90.00 | 3531.94 | 1440.00 |
| 0 | 2 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^29,2^24] | 256 | 53 | 53.00 | 974.77 | 848.00 |
| 0 | 2 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^28,2^20] | 256 | 33 | 33.00 | 596.47 | 528.00 |
| 0 | 2 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^27,2^16] | 256 | 24 | 24.00 | 422.56 | 384.00 |
| 0 | 2 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^26,2^12] | 256 | 19 | 19.00 | 321.23 | 304.00 |
| 0 | 2 | merkle_whir_batched_after_second | independent first Merkle oracles | 2 | [2^31,2^30] | 256 | 298 | 298.00 | 11263.28 | 4768.00 |
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
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR initial oracle | 1 | [2^32,2^30] | 256 | 138 | 138.00 | 2898.42 | 1104.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR recursive oracle 1 | 1 | [2^31,2^27] | 256 | 67 | 67.00 | 1343.77 | 1072.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR recursive oracle 2 | 1 | [2^30,2^23] | 256 | 38 | 38.00 | 754.37 | 608.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR recursive oracle 3 | 1 | [2^29,2^19] | 256 | 26 | 26.00 | 504.43 | 416.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR recursive oracle 4 | 1 | [2^28,2^15] | 256 | 20 | 20.00 | 376.91 | 320.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR recursive oracle 5 | 1 | [2^27,2^11] | 256 | 17 | 17.00 | 307.16 | 272.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim initial oracle | 2 | [2^31,2^27] | 256 | 67 | 67.00 | 2821.34 | 1072.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^30,2^24] | 256 | 44 | 44.00 | 865.10 | 704.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^29,2^20] | 256 | 29 | 29.00 | 557.91 | 464.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^28,2^16] | 256 | 22 | 22.00 | 410.51 | 352.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^27,2^12] | 256 | 18 | 18.00 | 323.71 | 288.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | independent first Merkle oracles | 2 | [2^32,2^30] | 256 | 138 | 138.00 | 5799.04 | 2208.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim initial oracle | 4 | [2^31,2^27] | 256 | 67 | 67.00 | 5646.96 | 2144.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^30,2^24] | 256 | 44 | 44.00 | 865.58 | 704.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^29,2^20] | 256 | 29 | 29.00 | 558.38 | 464.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^28,2^16] | 256 | 22 | 22.00 | 412.24 | 352.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^27,2^12] | 256 | 18 | 18.00 | 323.51 | 288.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | independent first Merkle oracles | 4 | [2^32,2^30] | 256 | 138 | 138.00 | 11590.08 | 4416.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim initial oracle | 8 | [2^31,2^27] | 256 | 67 | 67.00 | 11275.20 | 4288.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^30,2^24] | 256 | 44 | 44.00 | 865.36 | 704.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^29,2^20] | 256 | 29 | 29.00 | 559.53 | 464.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^28,2^16] | 256 | 22 | 22.00 | 409.93 | 352.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^27,2^12] | 256 | 18 | 18.00 | 324.44 | 288.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | independent first Merkle oracles | 8 | [2^32,2^30] | 256 | 138 | 138.00 | 23155.84 | 8832.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims initial oracle | 2 | [2^31,2^27] | 256 | 67 | 67.00 | 2822.32 | 1072.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 1 | 1 | [2^30,2^24] | 256 | 44 | 44.00 | 864.01 | 704.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 2 | 1 | [2^29,2^20] | 256 | 29 | 29.00 | 560.80 | 464.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 3 | 1 | [2^28,2^16] | 256 | 22 | 22.00 | 410.81 | 352.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 4 | 1 | [2^27,2^12] | 256 | 18 | 18.00 | 324.25 | 288.00 |
| 3 | 1 | canopy_schwartz_zippel | outer first oracle via Canopy | 1 | [2^31,2^30] | 256 | 298 | 298.00 | 298.00 | 2384.00 |
| 3 | 2 | canopy_schwartz_zippel | SZ-batched aligned claims initial oracle | 4 | [2^31,2^27] | 256 | 67 | 67.00 | 5645.48 | 2144.00 |
| 3 | 2 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 1 | 1 | [2^30,2^24] | 256 | 44 | 44.00 | 864.59 | 704.00 |
| 3 | 2 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 2 | 1 | [2^29,2^20] | 256 | 29 | 29.00 | 559.13 | 464.00 |
| 3 | 2 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 3 | 1 | [2^28,2^16] | 256 | 22 | 22.00 | 411.77 | 352.00 |
| 3 | 2 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 4 | 1 | [2^27,2^12] | 256 | 18 | 18.00 | 323.07 | 288.00 |
| 3 | 2 | canopy_schwartz_zippel | outer first oracle via Canopy | 2 | [2^31,2^30] | 256 | 298 | 298.00 | 596.00 | 4768.00 |
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
