# Batched WHIR and Canopy Exp3 (johnson)

Batched WHIR rates 1/2 and 1/4, and Canopy Exp3; GF(2^256), BLAKE3, 128-bit target with a 136-bit local algebraic/query budget. t=1,2,4,8; n=18,20,...,30. Fold logs 3/4 searched with terminal cap 10. 100 deterministic query samples, seed 0x50415045525f5631; Johnson decoding. Authentication counts are verifier leaf and parent calls; H2F is separate. WHIR proof models deduplicate query rows.

## Selected proof sizes

Both relative columns use the corresponding Merkle-WHIR baseline at the same `t,n`.

| t | log2 n | scheme | rate | batched sources | folds | h | terminal | min event bits | union bits | 128-bit RBR? | KiB | verifier auth hashes | H2F | vs batched | vs batched |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|:---:|---:|---:|---:|---:|---:|
| 1 | 18 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3 | - | 9 | 136.06 | 134.82 | yes | 237.80 | 4225.66 | 0.00 | 1.000x | 1.000x |
| 1 | 18 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,3 | - | 9 | 136.65 | 135.47 | yes | 157.52 | 2876.48 | 0.00 | 0.662x | 0.662x |
| 1 | 18 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3 | 4 | 9 | 136.19 | 135.25 | yes | 206.42 | 2682.59 | 315.59 | 0.868x | 0.868x |
| 2 | 18 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,3 | - | 9 | 136.06 | 134.35 | yes | 431.24 | 7928.59 | 0.00 | 1.000x | 1.000x |
| 2 | 18 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,3 | - | 9 | 136.65 | 134.94 | yes | 273.26 | 5257.28 | 0.00 | 0.634x | 0.634x |
| 2 | 18 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,3 | 4 | 9 | 136.19 | 134.64 | yes | 371.38 | 4876.68 | 631.04 | 0.861x | 0.861x |
| 4 | 18 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,3 | - | 9 | 136.06 | 133.71 | yes | 818.26 | 15336.03 | 0.00 | 1.000x | 1.000x |
| 4 | 18 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,3 | - | 9 | 136.65 | 134.25 | yes | 504.82 | 10018.61 | 0.00 | 0.617x | 0.617x |
| 4 | 18 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,3 | 4 | 9 | 136.19 | 133.89 | yes | 700.61 | 9246.49 | 1262.44 | 0.856x | 0.856x |
| 8 | 18 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,3 | - | 9 | 136.06 | 132.93 | yes | 1592.73 | 30151.24 | 0.00 | 1.000x | 1.000x |
| 8 | 18 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,3 | - | 9 | 136.65 | 133.44 | yes | 968.09 | 19544.89 | 0.00 | 0.608x | 0.608x |
| 8 | 18 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,3 | 4 | 9 | 136.19 | 133.03 | yes | 1359.42 | 18002.76 | 2522.16 | 0.854x | 0.854x |
| 1 | 20 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,3 | - | 8 | 136.06 | 134.57 | yes | 280.49 | 5595.33 | 0.00 | 1.000x | 1.000x |
| 1 | 20 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,3,3 | - | 8 | 136.65 | 135.16 | yes | 186.08 | 3826.89 | 0.00 | 0.663x | 0.663x |
| 1 | 20 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3,3 | 4 | 8 | 136.19 | 134.98 | yes | 230.50 | 3484.87 | 316.60 | 0.822x | 0.822x |
| 2 | 20 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,3,3 | - | 8 | 136.06 | 134.16 | yes | 500.24 | 10134.68 | 0.00 | 1.000x | 1.000x |
| 2 | 20 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,3,3 | - | 8 | 136.65 | 134.72 | yes | 315.41 | 6641.36 | 0.00 | 0.631x | 0.631x |
| 2 | 20 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,3,3 | 4 | 8 | 136.19 | 134.46 | yes | 404.05 | 5950.56 | 633.02 | 0.808x | 0.808x |
| 4 | 20 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,3,3 | - | 8 | 136.06 | 133.58 | yes | 938.67 | 19178.08 | 0.00 | 1.000x | 1.000x |
| 4 | 20 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,3,3 | - | 8 | 136.65 | 134.11 | yes | 574.46 | 12281.69 | 0.00 | 0.612x | 0.612x |
| 4 | 20 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,3,3 | 4 | 8 | 136.19 | 133.78 | yes | 751.62 | 10891.28 | 1266.76 | 0.801x | 0.801x |
| 8 | 20 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,3,3 | - | 8 | 136.06 | 132.85 | yes | 1816.50 | 37297.49 | 0.00 | 1.000x | 1.000x |
| 8 | 20 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,3,3 | - | 8 | 136.65 | 133.36 | yes | 1092.26 | 23551.54 | 0.00 | 0.601x | 0.601x |
| 8 | 20 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,3,3 | 4 | 8 | 136.19 | 132.97 | yes | 1447.24 | 20798.54 | 2532.48 | 0.797x | 0.797x |
| 1 | 22 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,4 | - | 9 | 136.06 | 134.57 | yes | 329.04 | 6566.08 | 0.00 | 1.000x | 1.000x |
| 1 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,4,4 | - | 8 | 136.65 | 135.12 | yes | 219.78 | 4299.27 | 0.00 | 0.668x | 0.668x |
| 1 | 22 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,4 | 4 | 8 | 136.19 | 134.94 | yes | 259.26 | 3798.09 | 316.90 | 0.788x | 0.788x |
| 2 | 22 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,3,4 | - | 9 | 136.06 | 134.16 | yes | 574.66 | 11932.02 | 0.00 | 1.000x | 1.000x |
| 2 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,4 | - | 8 | 136.65 | 134.69 | yes | 362.59 | 7545.26 | 0.00 | 0.631x | 0.631x |
| 2 | 22 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,4,4 | 4 | 8 | 136.19 | 134.43 | yes | 442.10 | 6557.83 | 633.86 | 0.769x | 0.769x |
| 4 | 22 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,3,4 | - | 9 | 136.06 | 133.58 | yes | 1065.62 | 22653.43 | 0.00 | 1.000x | 1.000x |
| 4 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,4,4 | - | 8 | 136.65 | 134.09 | yes | 648.86 | 14055.99 | 0.00 | 0.609x | 0.609x |
| 4 | 22 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,4,4 | 4 | 8 | 136.19 | 133.76 | yes | 807.33 | 12064.81 | 1267.68 | 0.758x | 0.758x |
| 8 | 22 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,3,4 | - | 9 | 136.06 | 132.85 | yes | 2045.64 | 44035.40 | 0.00 | 1.000x | 1.000x |
| 8 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,4,4 | - | 8 | 136.65 | 133.35 | yes | 1221.50 | 27080.19 | 0.00 | 0.597x | 0.597x |
| 8 | 22 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,4,4 | 4 | 8 | 136.19 | 132.96 | yes | 1538.56 | 23103.30 | 2534.96 | 0.752x | 0.752x |
| 1 | 24 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,3,4 | - | 8 | 136.06 | 134.37 | yes | 370.86 | 8036.59 | 0.00 | 1.000x | 1.000x |
| 1 | 24 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,3,3,4 | - | 8 | 136.65 | 134.98 | yes | 248.80 | 5426.60 | 0.00 | 0.671x | 0.671x |
| 1 | 24 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3,3,4 | 4 | 8 | 136.19 | 134.81 | yes | 283.87 | 4784.03 | 316.96 | 0.765x | 0.765x |
| 2 | 24 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,3,3,4 | - | 8 | 136.06 | 134.01 | yes | 642.16 | 14224.37 | 0.00 | 1.000x | 1.000x |
| 2 | 24 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,3,3,4 | - | 8 | 136.65 | 134.58 | yes | 405.59 | 9119.89 | 0.00 | 0.632x | 0.632x |
| 2 | 24 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,3,3,4 | 4 | 8 | 136.19 | 134.34 | yes | 475.53 | 7826.21 | 633.94 | 0.741x | 0.741x |
| 4 | 24 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,3,3,4 | - | 8 | 136.06 | 133.48 | yes | 1184.42 | 26586.68 | 0.00 | 1.000x | 1.000x |
| 4 | 24 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,3,3,4 | - | 8 | 136.65 | 134.02 | yes | 718.88 | 16495.02 | 0.00 | 0.607x | 0.607x |
| 4 | 24 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,3,3,4 | 4 | 8 | 136.19 | 133.70 | yes | 858.56 | 13901.89 | 1267.84 | 0.725x | 0.725x |
| 8 | 24 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,3,3,4 | - | 8 | 136.06 | 132.79 | yes | 2268.67 | 51304.47 | 0.00 | 1.000x | 1.000x |
| 8 | 24 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,3,3,4 | - | 8 | 136.65 | 133.30 | yes | 1345.86 | 31258.56 | 0.00 | 0.593x | 0.593x |
| 8 | 24 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,3,3,4 | 4 | 8 | 136.19 | 132.92 | yes | 1625.03 | 26064.73 | 2535.84 | 0.716x | 0.716x |
| 1 | 26 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,3,3,3 | - | 8 | 136.06 | 134.35 | yes | 415.49 | 9547.82 | 0.00 | 1.000x | 1.000x |
| 1 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,3,4 | - | 8 | 136.38 | 134.63 | yes | 279.18 | 5658.78 | 0.00 | 0.672x | 0.672x |
| 1 | 26 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,4,4 | 4 | 8 | 136.19 | 134.49 | yes | 309.45 | 5066.46 | 317.00 | 0.745x | 0.745x |
| 2 | 26 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,3,3,3,3 | - | 8 | 136.06 | 134.00 | yes | 712.45 | 16556.44 | 0.00 | 1.000x | 1.000x |
| 2 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,4,4 | - | 8 | 136.38 | 134.30 | yes | 449.37 | 9984.85 | 0.00 | 0.631x | 0.631x |
| 2 | 26 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,4,4,4 | 4 | 8 | 136.19 | 134.10 | yes | 509.86 | 8388.65 | 633.98 | 0.716x | 0.716x |
| 4 | 26 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,3,3,3,3 | - | 8 | 136.06 | 133.47 | yes | 1306.37 | 30571.83 | 0.00 | 1.000x | 1.000x |
| 4 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,4,4,4 | - | 8 | 136.38 | 133.82 | yes | 790.00 | 18234.93 | 0.00 | 0.605x | 0.605x |
| 4 | 26 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,4,4,4 | 4 | 8 | 136.19 | 133.54 | yes | 910.73 | 15034.61 | 1267.96 | 0.697x | 0.697x |
| 8 | 26 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,3,3,3,3 | - | 8 | 136.06 | 132.79 | yes | 2493.82 | 58590.91 | 0.00 | 1.000x | 1.000x |
| 8 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,4,4,4 | - | 8 | 136.38 | 133.18 | yes | 1472.27 | 34768.10 | 0.00 | 0.590x | 0.590x |
| 8 | 26 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,4,4,4 | 4 | 8 | 136.19 | 132.83 | yes | 1712.76 | 28335.48 | 2536.00 | 0.687x | 0.687x |
| 1 | 28 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,4,3,3,4 | - | 8 | 136.06 | 134.14 | yes | 460.53 | 10417.09 | 0.00 | 1.000x | 1.000x |
| 1 | 28 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,3,3,3 | - | 8 | 136.38 | 134.54 | yes | 309.33 | 6678.55 | 0.00 | 0.672x | 0.672x |
| 1 | 28 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,4,3,3 | 4 | 8 | 136.19 | 134.40 | yes | 335.29 | 5948.33 | 317.00 | 0.728x | 0.728x |
| 2 | 28 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,4,3,3,4 | - | 8 | 136.06 | 133.83 | yes | 783.21 | 18248.76 | 0.00 | 1.000x | 1.000x |
| 2 | 28 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,4,3,3 | - | 8 | 136.38 | 134.23 | yes | 493.75 | 11460.09 | 0.00 | 0.630x | 0.630x |
| 2 | 28 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,4,4,3,3 | 4 | 8 | 136.19 | 134.04 | yes | 544.57 | 9554.29 | 634.00 | 0.695x | 0.695x |
| 4 | 28 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,4,3,3,4 | - | 8 | 136.06 | 133.35 | yes | 1428.73 | 33915.51 | 0.00 | 1.000x | 1.000x |
| 4 | 28 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,4,4,3,3 | - | 8 | 136.38 | 133.77 | yes | 861.74 | 20585.56 | 0.00 | 0.603x | 0.603x |
| 4 | 28 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,4,4,3,3 | 4 | 8 | 136.19 | 133.50 | yes | 963.70 | 16784.34 | 1268.00 | 0.675x | 0.675x |
| 8 | 28 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,4,3,3,4 | - | 8 | 136.06 | 132.71 | yes | 2719.15 | 65228.84 | 0.00 | 1.000x | 1.000x |
| 8 | 28 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,4,4,3,3 | - | 8 | 136.38 | 133.15 | yes | 1596.26 | 38790.31 | 0.00 | 0.587x | 0.587x |
| 8 | 28 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,4,4,3,3 | 4 | 8 | 136.19 | 132.80 | yes | 1800.10 | 31185.66 | 2535.92 | 0.662x | 0.662x |
| 1 | 30 | merkle_whir_batched_after_second | 1/2 | 1 | 3,4,4,4,4,3 | - | 8 | 136.19 | 134.51 | yes | 503.68 | 10979.73 | 0.00 | 1.000x | 1.000x |
| 1 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,3,4,4 | - | 8 | 136.38 | 134.61 | yes | 338.30 | 7269.48 | 0.00 | 0.672x | 0.672x |
| 1 | 30 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,4,4,4 | 4 | 8 | 136.19 | 134.47 | yes | 360.32 | 6413.33 | 317.00 | 0.715x | 0.715x |
| 2 | 30 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,4,4,4,4 | - | 8 | 136.06 | 134.12 | yes | 852.75 | 20033.94 | 0.00 | 1.000x | 1.000x |
| 2 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,4,4,4 | - | 8 | 136.38 | 134.29 | yes | 537.19 | 12513.97 | 0.00 | 0.630x | 0.630x |
| 2 | 30 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,4,4,4,4 | 4 | 8 | 136.19 | 134.09 | yes | 578.66 | 10309.10 | 634.00 | 0.679x | 0.679x |
| 4 | 30 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,4,4,4,4 | - | 8 | 136.06 | 133.56 | yes | 1550.04 | 37357.14 | 0.00 | 1.000x | 1.000x |
| 4 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,4,4,4,4 | - | 8 | 136.38 | 133.81 | yes | 932.35 | 22509.34 | 0.00 | 0.602x | 0.602x |
| 4 | 30 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,4,4,4,4 | 4 | 8 | 136.19 | 133.54 | yes | 1014.88 | 18086.30 | 1268.00 | 0.655x | 0.655x |
| 8 | 30 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,4,4,4,4 | - | 8 | 136.06 | 132.84 | yes | 2944.82 | 72010.40 | 0.00 | 1.000x | 1.000x |
| 8 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,4,4,4,4 | - | 8 | 136.38 | 133.17 | yes | 1721.48 | 42461.44 | 0.00 | 0.585x | 0.585x |
| 8 | 30 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,4,4,4,4 | 4 | 8 | 136.19 | 132.82 | yes | 1887.77 | 33654.71 | 2536.00 | 0.641x | 0.641x |

## Component breakdown

Size entries are KiB; authentication and H2F columns count hash calls.

| t | log2 n | scheme | roots | auth | OOD | sumcheck | batching | answers | coefficients | total | verifier auth hashes | H2F |
|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 18 | merkle_whir_batched_after_second | 0.09 | 102.89 | 0.09 | 1.69 | 0.00 | 117.04 | 16.00 | 237.80 | 4225.66 | 0.00 |
| 1 | 18 | merkle_whir_batched_rate_quarter_after_second | 0.09 | 73.43 | 0.09 | 1.69 | 0.00 | 66.22 | 16.00 | 157.52 | 2876.48 | 0.00 |
| 1 | 18 | canopy_schwartz_zippel | 0.09 | 62.20 | 0.12 | 1.69 | 0.03 | 126.28 | 16.00 | 206.42 | 2682.59 | 315.59 |
| 2 | 18 | merkle_whir_batched_after_second | 0.16 | 192.92 | 0.16 | 1.97 | 0.03 | 220.01 | 16.00 | 431.24 | 7928.59 | 0.00 |
| 2 | 18 | merkle_whir_batched_rate_quarter_after_second | 0.16 | 134.28 | 0.16 | 1.97 | 0.03 | 120.67 | 16.00 | 273.26 | 5257.28 | 0.00 |
| 2 | 18 | canopy_schwartz_zippel | 0.16 | 112.01 | 0.22 | 1.97 | 0.16 | 240.87 | 16.00 | 371.38 | 4876.68 | 631.04 |
| 4 | 18 | merkle_whir_batched_after_second | 0.28 | 373.04 | 0.28 | 2.53 | 0.16 | 425.97 | 16.00 | 818.26 | 15336.03 | 0.00 |
| 4 | 18 | merkle_whir_batched_rate_quarter_after_second | 0.28 | 255.96 | 0.28 | 2.53 | 0.16 | 229.62 | 16.00 | 504.82 | 10018.61 | 0.00 |
| 4 | 18 | canopy_schwartz_zippel | 0.28 | 211.15 | 0.41 | 2.53 | 0.41 | 469.84 | 16.00 | 700.61 | 9246.49 | 1262.44 |
| 8 | 18 | merkle_whir_batched_after_second | 0.53 | 733.14 | 0.53 | 3.66 | 0.41 | 838.46 | 16.00 | 1592.73 | 30151.24 | 0.00 |
| 8 | 18 | merkle_whir_batched_rate_quarter_after_second | 0.53 | 499.42 | 0.53 | 3.66 | 0.41 | 447.54 | 16.00 | 968.09 | 19544.89 | 0.00 |
| 8 | 18 | canopy_schwartz_zippel | 0.53 | 409.79 | 0.78 | 3.66 | 0.91 | 927.75 | 16.00 | 1359.42 | 18002.76 | 2522.16 |
| 1 | 20 | merkle_whir_batched_after_second | 0.12 | 143.18 | 0.12 | 1.88 | 0.00 | 127.18 | 8.00 | 280.49 | 5595.33 | 0.00 |
| 1 | 20 | merkle_whir_batched_rate_quarter_after_second | 0.12 | 100.97 | 0.12 | 1.88 | 0.00 | 74.99 | 8.00 | 186.08 | 3826.89 | 0.00 |
| 1 | 20 | canopy_schwartz_zippel | 0.12 | 85.12 | 0.16 | 1.88 | 0.03 | 135.19 | 8.00 | 230.50 | 3484.87 | 316.60 |
| 2 | 20 | merkle_whir_batched_after_second | 0.19 | 259.30 | 0.19 | 2.16 | 0.03 | 230.37 | 8.00 | 500.24 | 10134.68 | 0.00 |
| 2 | 20 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 175.36 | 0.19 | 2.16 | 0.03 | 129.49 | 8.00 | 315.41 | 6641.36 | 0.00 |
| 2 | 20 | canopy_schwartz_zippel | 0.19 | 143.44 | 0.25 | 2.16 | 0.16 | 249.86 | 8.00 | 404.05 | 5950.56 | 633.02 |
| 4 | 20 | merkle_whir_batched_after_second | 0.31 | 490.45 | 0.31 | 2.72 | 0.16 | 436.72 | 8.00 | 938.67 | 19178.08 | 0.00 |
| 4 | 20 | merkle_whir_batched_rate_quarter_after_second | 0.31 | 324.50 | 0.31 | 2.72 | 0.16 | 238.46 | 8.00 | 574.46 | 12281.69 | 0.00 |
| 4 | 20 | canopy_schwartz_zippel | 0.31 | 260.39 | 0.44 | 2.72 | 0.41 | 479.36 | 8.00 | 751.62 | 10891.28 | 1266.76 |
| 8 | 20 | merkle_whir_batched_after_second | 0.56 | 953.77 | 0.56 | 3.84 | 0.41 | 849.35 | 8.00 | 1816.50 | 37297.49 | 0.00 |
| 8 | 20 | merkle_whir_batched_rate_quarter_after_second | 0.56 | 622.43 | 0.56 | 3.84 | 0.41 | 456.45 | 8.00 | 1092.26 | 23551.54 | 0.00 |
| 8 | 20 | canopy_schwartz_zippel | 0.56 | 495.09 | 0.81 | 3.84 | 0.91 | 938.02 | 8.00 | 1447.24 | 20798.54 | 2532.48 |
| 1 | 22 | merkle_whir_batched_after_second | 0.12 | 173.51 | 0.12 | 2.06 | 0.00 | 137.22 | 16.00 | 329.04 | 6566.08 | 0.00 |
| 1 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.12 | 115.98 | 0.12 | 2.06 | 0.00 | 93.49 | 8.00 | 219.78 | 4299.27 | 0.00 |
| 1 | 22 | canopy_schwartz_zippel | 0.12 | 95.16 | 0.16 | 2.06 | 0.03 | 153.72 | 8.00 | 259.26 | 3798.09 | 316.90 |
| 2 | 22 | merkle_whir_batched_after_second | 0.19 | 315.44 | 0.19 | 2.34 | 0.03 | 240.47 | 16.00 | 574.66 | 11932.02 | 0.00 |
| 2 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 203.85 | 0.19 | 2.34 | 0.03 | 147.99 | 8.00 | 362.59 | 7545.26 | 0.00 |
| 2 | 22 | canopy_schwartz_zippel | 0.19 | 162.68 | 0.25 | 2.34 | 0.16 | 268.49 | 8.00 | 442.10 | 6557.83 | 633.86 |
| 4 | 22 | merkle_whir_batched_after_second | 0.31 | 599.00 | 0.31 | 2.91 | 0.16 | 446.93 | 16.00 | 1065.62 | 22653.43 | 0.00 |
| 4 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.31 | 380.19 | 0.31 | 2.91 | 0.16 | 256.99 | 8.00 | 648.86 | 14055.99 | 0.00 |
| 4 | 22 | canopy_schwartz_zippel | 0.31 | 297.34 | 0.44 | 2.91 | 0.41 | 497.93 | 8.00 | 807.33 | 12064.81 | 1267.68 |
| 8 | 22 | merkle_whir_batched_after_second | 0.56 | 1164.20 | 0.56 | 4.03 | 0.41 | 859.88 | 16.00 | 2045.64 | 44035.40 | 0.00 |
| 8 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.56 | 732.94 | 0.56 | 4.03 | 0.41 | 475.00 | 8.00 | 1221.50 | 27080.19 | 0.00 |
| 8 | 22 | canopy_schwartz_zippel | 0.56 | 567.41 | 0.81 | 4.03 | 0.91 | 956.84 | 8.00 | 1538.56 | 23103.30 | 2534.96 |
| 1 | 24 | merkle_whir_batched_after_second | 0.16 | 217.55 | 0.16 | 2.25 | 0.00 | 142.75 | 8.00 | 370.86 | 8036.59 | 0.00 |
| 1 | 24 | merkle_whir_batched_rate_quarter_after_second | 0.16 | 149.24 | 0.16 | 2.25 | 0.00 | 89.00 | 8.00 | 248.80 | 5426.60 | 0.00 |
| 1 | 24 | canopy_schwartz_zippel | 0.16 | 124.00 | 0.19 | 2.25 | 0.03 | 149.24 | 8.00 | 283.87 | 4784.03 | 316.96 |
| 2 | 24 | merkle_whir_batched_after_second | 0.22 | 385.17 | 0.22 | 2.53 | 0.03 | 246.00 | 8.00 | 642.16 | 14224.37 | 0.00 |
| 2 | 24 | merkle_whir_batched_rate_quarter_after_second | 0.22 | 251.09 | 0.22 | 2.53 | 0.03 | 143.50 | 8.00 | 405.59 | 9119.89 | 0.00 |
| 2 | 24 | canopy_schwartz_zippel | 0.22 | 200.35 | 0.28 | 2.53 | 0.16 | 263.99 | 8.00 | 475.53 | 7826.21 | 633.94 |
| 4 | 24 | merkle_whir_batched_after_second | 0.34 | 719.99 | 0.34 | 3.09 | 0.16 | 452.50 | 8.00 | 1184.42 | 26586.68 | 0.00 |
| 4 | 24 | merkle_whir_batched_rate_quarter_after_second | 0.34 | 454.44 | 0.34 | 3.09 | 0.16 | 252.50 | 8.00 | 718.88 | 16495.02 | 0.00 |
| 4 | 24 | canopy_schwartz_zippel | 0.34 | 352.78 | 0.47 | 3.09 | 0.41 | 493.47 | 8.00 | 858.56 | 13901.89 | 1267.84 |
| 8 | 24 | merkle_whir_batched_after_second | 0.59 | 1389.44 | 0.59 | 4.22 | 0.41 | 865.42 | 8.00 | 2268.67 | 51304.47 | 0.00 |
| 8 | 24 | merkle_whir_batched_rate_quarter_after_second | 0.59 | 861.55 | 0.59 | 4.22 | 0.41 | 470.50 | 8.00 | 1345.86 | 31258.56 | 0.00 |
| 8 | 24 | canopy_schwartz_zippel | 0.59 | 657.99 | 0.84 | 4.22 | 0.91 | 952.48 | 8.00 | 1625.03 | 26064.73 | 2535.84 |
| 1 | 26 | merkle_whir_batched_after_second | 0.19 | 263.18 | 0.19 | 2.44 | 0.00 | 141.50 | 8.00 | 415.49 | 9547.82 | 0.00 |
| 1 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.16 | 157.68 | 0.16 | 2.44 | 0.00 | 110.75 | 8.00 | 279.18 | 5658.78 | 0.00 |
| 1 | 26 | canopy_schwartz_zippel | 0.16 | 133.39 | 0.19 | 2.44 | 0.03 | 165.25 | 8.00 | 309.45 | 5066.46 | 317.00 |
| 2 | 26 | merkle_whir_batched_after_second | 0.25 | 456.45 | 0.25 | 2.72 | 0.03 | 244.75 | 8.00 | 712.45 | 16556.44 | 0.00 |
| 2 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.22 | 278.68 | 0.22 | 2.72 | 0.03 | 159.50 | 8.00 | 449.37 | 9984.85 | 0.00 |
| 2 | 26 | canopy_schwartz_zippel | 0.22 | 218.49 | 0.28 | 2.72 | 0.16 | 280.00 | 8.00 | 509.86 | 8388.65 | 633.98 |
| 4 | 26 | merkle_whir_batched_after_second | 0.38 | 842.93 | 0.38 | 3.28 | 0.16 | 451.25 | 8.00 | 1306.37 | 30571.83 | 0.00 |
| 4 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.34 | 509.37 | 0.34 | 3.28 | 0.16 | 268.50 | 8.00 | 790.00 | 18234.93 | 0.00 |
| 4 | 26 | canopy_schwartz_zippel | 0.34 | 388.74 | 0.47 | 3.28 | 0.41 | 509.49 | 8.00 | 910.73 | 15034.61 | 1267.96 |
| 8 | 26 | merkle_whir_batched_after_second | 0.62 | 1615.53 | 0.62 | 4.41 | 0.41 | 864.23 | 8.00 | 2493.82 | 58590.91 | 0.00 |
| 8 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.59 | 971.79 | 0.59 | 4.41 | 0.41 | 486.48 | 8.00 | 1472.27 | 34768.10 | 0.00 |
| 8 | 26 | canopy_schwartz_zippel | 0.59 | 729.51 | 0.84 | 4.41 | 0.91 | 968.50 | 8.00 | 1712.76 | 28335.48 | 2536.00 |
| 1 | 28 | merkle_whir_batched_after_second | 0.19 | 291.03 | 0.19 | 2.62 | 0.00 | 158.50 | 8.00 | 460.53 | 10417.09 | 0.00 |
| 1 | 28 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 188.33 | 0.19 | 2.62 | 0.00 | 110.00 | 8.00 | 309.33 | 6678.55 | 0.00 |
| 1 | 28 | canopy_schwartz_zippel | 0.19 | 159.73 | 0.22 | 2.62 | 0.03 | 164.50 | 8.00 | 335.29 | 5948.33 | 317.00 |
| 2 | 28 | merkle_whir_batched_after_second | 0.25 | 510.02 | 0.25 | 2.91 | 0.03 | 261.75 | 8.00 | 783.21 | 18248.76 | 0.00 |
| 2 | 28 | merkle_whir_batched_rate_quarter_after_second | 0.25 | 323.57 | 0.25 | 2.91 | 0.03 | 158.75 | 8.00 | 493.75 | 11460.09 | 0.00 |
| 2 | 28 | canopy_schwartz_zippel | 0.25 | 253.70 | 0.31 | 2.91 | 0.16 | 279.25 | 8.00 | 544.57 | 9554.29 | 634.00 |
| 4 | 28 | merkle_whir_batched_after_second | 0.38 | 948.11 | 0.38 | 3.47 | 0.16 | 468.25 | 8.00 | 1428.73 | 33915.51 | 0.00 |
| 4 | 28 | merkle_whir_batched_rate_quarter_after_second | 0.38 | 581.61 | 0.38 | 3.47 | 0.16 | 267.75 | 8.00 | 861.74 | 20585.56 | 0.00 |
| 4 | 28 | canopy_schwartz_zippel | 0.38 | 442.20 | 0.50 | 3.47 | 0.41 | 508.75 | 8.00 | 963.70 | 16784.34 | 1268.00 |
| 8 | 28 | merkle_whir_batched_after_second | 0.62 | 1823.65 | 0.62 | 4.59 | 0.41 | 881.25 | 8.00 | 2719.15 | 65228.84 | 0.00 |
| 8 | 28 | merkle_whir_batched_rate_quarter_after_second | 0.62 | 1096.26 | 0.62 | 4.59 | 0.41 | 485.75 | 8.00 | 1596.26 | 38790.31 | 0.00 |
| 8 | 28 | canopy_schwartz_zippel | 0.62 | 817.37 | 0.88 | 4.59 | 0.91 | 967.73 | 8.00 | 1800.10 | 31185.66 | 2535.92 |
| 1 | 30 | merkle_whir_batched_after_second | 0.19 | 309.99 | 0.19 | 2.81 | 0.00 | 182.50 | 8.00 | 503.68 | 10979.73 | 0.00 |
| 1 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 206.86 | 0.19 | 2.81 | 0.00 | 120.25 | 8.00 | 338.30 | 7269.48 | 0.00 |
| 1 | 30 | canopy_schwartz_zippel | 0.19 | 174.32 | 0.22 | 2.81 | 0.03 | 174.75 | 8.00 | 360.32 | 6413.33 | 317.00 |
| 2 | 30 | merkle_whir_batched_after_second | 0.25 | 566.12 | 0.25 | 3.09 | 0.03 | 275.00 | 8.00 | 852.75 | 20033.94 | 0.00 |
| 2 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.25 | 356.56 | 0.25 | 3.09 | 0.03 | 169.00 | 8.00 | 537.19 | 12513.97 | 0.00 |
| 2 | 30 | canopy_schwartz_zippel | 0.25 | 277.35 | 0.31 | 3.09 | 0.16 | 289.50 | 8.00 | 578.66 | 10309.10 | 634.00 |
| 4 | 30 | merkle_whir_batched_after_second | 0.38 | 1055.97 | 0.38 | 3.66 | 0.16 | 481.50 | 8.00 | 1550.04 | 37357.14 | 0.00 |
| 4 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.38 | 641.79 | 0.38 | 3.66 | 0.16 | 278.00 | 8.00 | 932.35 | 22509.34 | 0.00 |
| 4 | 30 | canopy_schwartz_zippel | 0.38 | 482.95 | 0.50 | 3.66 | 0.41 | 519.00 | 8.00 | 1014.88 | 18086.30 | 1268.00 |
| 8 | 30 | merkle_whir_batched_after_second | 0.62 | 2035.89 | 0.62 | 4.78 | 0.41 | 894.50 | 8.00 | 2944.82 | 72010.40 | 0.00 |
| 8 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.62 | 1211.05 | 0.62 | 4.78 | 0.41 | 496.00 | 8.00 | 1721.48 | 42461.44 | 0.00 |
| 8 | 30 | canopy_schwartz_zippel | 0.62 | 894.58 | 0.88 | 4.78 | 0.91 | 978.00 | 8.00 | 1887.77 | 33654.71 | 2536.00 |

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
| 0 | 2 | merkle_whir_batched_after_second | SZ-batched second-oracle claim initial oracle | 2 | [2^30,2^27] | 256 | 96 | 96.00 | 3747.82 | 1536.00 |
| 0 | 2 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^29,2^24] | 256 | 56 | 56.00 | 1025.32 | 896.00 |
| 0 | 2 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^28,2^20] | 256 | 35 | 35.00 | 629.30 | 560.00 |
| 0 | 2 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^27,2^16] | 256 | 26 | 26.00 | 453.05 | 416.00 |
| 0 | 2 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^26,2^12] | 256 | 20 | 20.00 | 337.65 | 320.00 |
| 0 | 2 | merkle_whir_batched_after_second | independent first Merkle oracles | 2 | [2^31,2^30] | 256 | 317 | 317.00 | 11922.80 | 5072.00 |
| 0 | 4 | merkle_whir_batched_after_second | SZ-batched second-oracle claim initial oracle | 4 | [2^30,2^27] | 256 | 96 | 96.00 | 7508.48 | 3072.00 |
| 0 | 4 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^29,2^24] | 256 | 56 | 56.00 | 1025.22 | 896.00 |
| 0 | 4 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^28,2^20] | 256 | 35 | 35.00 | 630.30 | 560.00 |
| 0 | 4 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^27,2^16] | 256 | 26 | 26.00 | 453.76 | 416.00 |
| 0 | 4 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^26,2^12] | 256 | 20 | 20.00 | 335.34 | 320.00 |
| 0 | 4 | merkle_whir_batched_after_second | independent first Merkle oracles | 4 | [2^31,2^30] | 256 | 317 | 317.00 | 23838.04 | 10144.00 |
| 0 | 8 | merkle_whir_batched_after_second | SZ-batched second-oracle claim initial oracle | 8 | [2^30,2^27] | 256 | 96 | 96.00 | 14999.04 | 6144.00 |
| 0 | 8 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^29,2^24] | 256 | 56 | 56.00 | 1026.03 | 896.00 |
| 0 | 8 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^28,2^20] | 256 | 35 | 35.00 | 629.41 | 560.00 |
| 0 | 8 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^27,2^16] | 256 | 26 | 26.00 | 454.66 | 416.00 |
| 0 | 8 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^26,2^12] | 256 | 20 | 20.00 | 337.74 | 320.00 |
| 0 | 8 | merkle_whir_batched_after_second | independent first Merkle oracles | 8 | [2^31,2^30] | 256 | 317 | 317.00 | 47701.52 | 20288.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR initial oracle | 1 | [2^32,2^30] | 256 | 147 | 147.00 | 3072.49 | 1176.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR recursive oracle 1 | 1 | [2^31,2^27] | 256 | 71 | 71.00 | 1417.78 | 1136.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR recursive oracle 2 | 1 | [2^30,2^23] | 256 | 40 | 40.00 | 792.35 | 640.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR recursive oracle 3 | 1 | [2^29,2^19] | 256 | 28 | 28.00 | 568.83 | 224.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR recursive oracle 4 | 1 | [2^28,2^16] | 256 | 23 | 23.00 | 428.03 | 368.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | single-input WHIR recursive oracle 5 | 1 | [2^27,2^12] | 256 | 19 | 19.00 | 340.00 | 304.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim initial oracle | 2 | [2^31,2^27] | 256 | 71 | 71.00 | 2978.16 | 1136.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^30,2^24] | 256 | 47 | 47.00 | 920.40 | 752.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^29,2^20] | 256 | 31 | 31.00 | 592.95 | 496.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^28,2^16] | 256 | 23 | 23.00 | 427.64 | 368.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^27,2^12] | 256 | 19 | 19.00 | 340.56 | 304.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | independent first Merkle oracles | 2 | [2^32,2^30] | 256 | 147 | 147.00 | 6150.26 | 2352.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim initial oracle | 4 | [2^31,2^27] | 256 | 71 | 71.00 | 5960.60 | 2272.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^30,2^24] | 256 | 47 | 47.00 | 920.77 | 752.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^29,2^20] | 256 | 31 | 31.00 | 594.08 | 496.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^28,2^16] | 256 | 23 | 23.00 | 428.76 | 368.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^27,2^12] | 256 | 19 | 19.00 | 340.85 | 304.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | independent first Merkle oracles | 4 | [2^32,2^30] | 256 | 147 | 147.00 | 12292.28 | 4704.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim initial oracle | 8 | [2^31,2^27] | 256 | 71 | 71.00 | 11902.88 | 4544.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^30,2^24] | 256 | 47 | 47.00 | 920.25 | 752.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^29,2^20] | 256 | 31 | 31.00 | 595.43 | 496.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^28,2^16] | 256 | 23 | 23.00 | 429.05 | 368.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^27,2^12] | 256 | 19 | 19.00 | 340.07 | 304.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | independent first Merkle oracles | 8 | [2^32,2^30] | 256 | 147 | 147.00 | 24565.76 | 9408.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims initial oracle | 2 | [2^31,2^27] | 256 | 71 | 71.00 | 2978.74 | 1136.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 1 | 1 | [2^30,2^24] | 256 | 47 | 47.00 | 919.60 | 752.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 2 | 1 | [2^29,2^20] | 256 | 31 | 31.00 | 595.14 | 496.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 3 | 1 | [2^28,2^16] | 256 | 23 | 23.00 | 427.30 | 368.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 4 | 1 | [2^27,2^12] | 256 | 19 | 19.00 | 340.55 | 304.00 |
| 3 | 1 | canopy_schwartz_zippel | outer first oracle via Canopy | 1 | [2^31,2^30] | 256 | 317 | 317.00 | 317.00 | 2536.00 |
| 3 | 2 | canopy_schwartz_zippel | SZ-batched aligned claims initial oracle | 4 | [2^31,2^27] | 256 | 71 | 71.00 | 5958.20 | 2272.00 |
| 3 | 2 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 1 | 1 | [2^30,2^24] | 256 | 47 | 47.00 | 919.37 | 752.00 |
| 3 | 2 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 2 | 1 | [2^29,2^20] | 256 | 31 | 31.00 | 594.96 | 496.00 |
| 3 | 2 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 3 | 1 | [2^28,2^16] | 256 | 23 | 23.00 | 428.06 | 368.00 |
| 3 | 2 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 4 | 1 | [2^27,2^12] | 256 | 19 | 19.00 | 340.51 | 304.00 |
| 3 | 2 | canopy_schwartz_zippel | outer first oracle via Canopy | 2 | [2^31,2^30] | 256 | 317 | 317.00 | 634.00 | 5072.00 |
| 3 | 4 | canopy_schwartz_zippel | SZ-batched aligned claims initial oracle | 8 | [2^31,2^27] | 256 | 71 | 71.00 | 11904.56 | 4544.00 |
| 3 | 4 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 1 | 1 | [2^30,2^24] | 256 | 47 | 47.00 | 919.92 | 752.00 |
| 3 | 4 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 2 | 1 | [2^29,2^20] | 256 | 31 | 31.00 | 593.75 | 496.00 |
| 3 | 4 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 3 | 1 | [2^28,2^16] | 256 | 23 | 23.00 | 428.53 | 368.00 |
| 3 | 4 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 4 | 1 | [2^27,2^12] | 256 | 19 | 19.00 | 339.54 | 304.00 |
| 3 | 4 | canopy_schwartz_zippel | outer first oracle via Canopy | 4 | [2^31,2^30] | 256 | 317 | 317.00 | 1268.00 | 10144.00 |
| 3 | 8 | canopy_schwartz_zippel | SZ-batched aligned claims initial oracle | 16 | [2^31,2^27] | 256 | 71 | 71.00 | 23808.80 | 9088.00 |
| 3 | 8 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 1 | 1 | [2^30,2^24] | 256 | 47 | 47.00 | 919.38 | 752.00 |
| 3 | 8 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 2 | 1 | [2^29,2^20] | 256 | 31 | 31.00 | 594.31 | 496.00 |
| 3 | 8 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 3 | 1 | [2^28,2^16] | 256 | 23 | 23.00 | 427.46 | 368.00 |
| 3 | 8 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 4 | 1 | [2^27,2^12] | 256 | 19 | 19.00 | 340.76 | 304.00 |
| 3 | 8 | canopy_schwartz_zippel | outer first oracle via Canopy | 8 | [2^31,2^30] | 256 | 317 | 317.00 | 2536.00 | 20288.00 |
