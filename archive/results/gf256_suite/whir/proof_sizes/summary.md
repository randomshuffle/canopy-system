# Multi-input Experiments 2 and 3 (johnson)

This report uses `t in {1,2,4,8}` input oracles and `n in {2^18,2^20,2^22,2^24,2^26,2^28,2^30}`, no proof of work, 256-bit inputs, folding parameters in `{3,4}`, terminal messages of at most `2^10` elements, and the existing Johnson eta=sqrt(rho)/20 slack. Independent and batched baselines are each reported at rates 1/2 and 1/4. The Johnson folding bound uses the exact exceptional-set expression in Theorem 4.6. Proof sizes are means over 100 deterministic query samples from seed `0x50415045525f5631`.

Each independent baseline contains `t` complete Merkle-WHIR proofs at its selected starting rate. The two batched baselines, at initial rates 1/2 and 1/4, keep `t` independent first transitions and batch the `t` second-oracle claims into one tail (a no-op when `t=1`). Experiment 2 makes `2t` recursive claims and proves all independently. Experiment 3 batches its `2t` aligned 256-bit claims into one generalized Schwartz--Zippel identity, sending `2(2t)-3` cross-term coefficients before the challenge.

## Selected proof sizes

Both relative columns use the corresponding Merkle-WHIR baseline at the same `t,n`.

| t | log2 n | scheme | rate | batched sources | folds | h | terminal | min event bits | union bits | 128-bit RBR? | KiB | vs independent | vs batched |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|:---:|---:|---:|---:|
| 1 | 18 | merkle_whir_independent | 1/2 | 0 | 3,3,3 | - | 9 | 128.02 | 126.86 | yes | 225.91 | 1.000x | 1.000x |
| 1 | 18 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,3,3 | - | 9 | 128.29 | 127.18 | yes | 149.79 | 0.663x | 0.663x |
| 1 | 18 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3 | - | 9 | 128.02 | 126.86 | yes | 225.91 | 1.000x | 1.000x |
| 1 | 18 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,3 | - | 9 | 128.29 | 127.18 | yes | 149.79 | 0.663x | 0.663x |
| 1 | 18 | canopy_unbatched | 1/2 | 0 | 3,3,3 | 18 | 9 | 128.02 | 127.04 | yes | 222.37 | 0.984x | 0.984x |
| 1 | 18 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3 | 4 | 9 | 128.02 | 127.05 | yes | 195.81 | 0.867x | 0.867x |
| 2 | 18 | merkle_whir_independent | 1/2 | 0 | 3,3,3 | - | 9 | 128.02 | 125.86 | yes | 451.56 | 1.000x | 1.105x |
| 2 | 18 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,3,3 | - | 9 | 128.29 | 126.18 | yes | 299.49 | 0.663x | 0.733x |
| 2 | 18 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,3 | - | 9 | 128.02 | 126.33 | yes | 408.73 | 0.905x | 1.000x |
| 2 | 18 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,3 | - | 9 | 128.29 | 126.63 | yes | 259.25 | 0.574x | 0.634x |
| 2 | 18 | canopy_unbatched | 1/2 | 0 | 3,3,3 | 18 | 9 | 128.02 | 126.04 | yes | 444.82 | 0.985x | 1.088x |
| 2 | 18 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,3 | 4 | 9 | 128.02 | 126.46 | yes | 351.52 | 0.778x | 0.860x |
| 4 | 18 | merkle_whir_independent | 1/2 | 0 | 3,3,3 | - | 9 | 128.02 | 124.86 | yes | 903.29 | 1.000x | 1.166x |
| 4 | 18 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,3,3 | - | 9 | 128.29 | 125.18 | yes | 599.37 | 0.664x | 0.774x |
| 4 | 18 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,3 | - | 9 | 128.02 | 125.64 | yes | 774.60 | 0.858x | 1.000x |
| 4 | 18 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,3 | - | 9 | 128.29 | 125.92 | yes | 478.17 | 0.529x | 0.617x |
| 4 | 18 | canopy_unbatched | 1/2 | 0 | 3,3,3 | 18 | 9 | 128.02 | 125.04 | yes | 889.02 | 0.984x | 1.148x |
| 4 | 18 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,3 | 4 | 9 | 128.02 | 125.71 | yes | 662.32 | 0.733x | 0.855x |
| 8 | 18 | merkle_whir_independent | 1/2 | 0 | 3,3,3 | - | 9 | 128.02 | 123.86 | yes | 1807.33 | 1.000x | 1.200x |
| 8 | 18 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,3,3 | - | 9 | 128.29 | 124.18 | yes | 1198.74 | 0.663x | 0.796x |
| 8 | 18 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,3 | - | 9 | 128.02 | 124.82 | yes | 1506.40 | 0.833x | 1.000x |
| 8 | 18 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,3 | - | 9 | 128.29 | 125.09 | yes | 916.29 | 0.507x | 0.608x |
| 8 | 18 | canopy_unbatched | 1/2 | 0 | 3,3,3 | 18 | 9 | 128.02 | 124.04 | yes | 1778.39 | 0.984x | 1.181x |
| 8 | 18 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,3 | 4 | 9 | 128.02 | 124.86 | yes | 1284.51 | 0.711x | 0.853x |
| 1 | 20 | merkle_whir_independent | 1/2 | 0 | 3,3,3,3 | - | 8 | 128.02 | 126.74 | yes | 265.86 | 1.000x | 1.000x |
| 1 | 20 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,3,3,3 | - | 8 | 128.29 | 126.94 | yes | 176.32 | 0.663x | 0.663x |
| 1 | 20 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,3 | - | 8 | 128.02 | 126.74 | yes | 265.86 | 1.000x | 1.000x |
| 1 | 20 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,3,3 | - | 8 | 128.29 | 126.94 | yes | 176.32 | 0.663x | 0.663x |
| 1 | 20 | canopy_unbatched | 1/2 | 0 | 3,3,3,3 | 7 | 8 | 128.02 | 126.34 | yes | 256.23 | 0.964x | 0.964x |
| 1 | 20 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3,3 | 4 | 8 | 128.02 | 126.84 | yes | 218.12 | 0.820x | 0.820x |
| 2 | 20 | merkle_whir_independent | 1/2 | 0 | 3,3,3,3 | - | 8 | 128.02 | 125.74 | yes | 531.86 | 1.000x | 1.123x |
| 2 | 20 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,3,3,3 | - | 8 | 128.29 | 125.94 | yes | 352.61 | 0.663x | 0.745x |
| 2 | 20 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,3,3 | - | 8 | 128.02 | 126.24 | yes | 473.49 | 0.890x | 1.000x |
| 2 | 20 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,3,3 | - | 8 | 128.29 | 126.46 | yes | 298.61 | 0.561x | 0.631x |
| 2 | 20 | canopy_unbatched | 1/2 | 0 | 3,3,3,3 | 7 | 8 | 128.02 | 125.34 | yes | 512.65 | 0.964x | 1.083x |
| 2 | 20 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,3,3 | 4 | 8 | 128.02 | 126.31 | yes | 381.90 | 0.718x | 0.807x |
| 4 | 20 | merkle_whir_independent | 1/2 | 0 | 3,3,3,3 | - | 8 | 128.02 | 124.74 | yes | 1063.83 | 1.000x | 1.198x |
| 4 | 20 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,3,3,3 | - | 8 | 128.29 | 124.94 | yes | 705.00 | 0.663x | 0.794x |
| 4 | 20 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,3,3 | - | 8 | 128.02 | 125.58 | yes | 887.65 | 0.834x | 1.000x |
| 4 | 20 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,3,3 | - | 8 | 128.29 | 125.82 | yes | 543.22 | 0.511x | 0.612x |
| 4 | 20 | canopy_unbatched | 1/2 | 0 | 3,3,3,3 | 7 | 8 | 128.02 | 124.34 | yes | 1025.61 | 0.964x | 1.155x |
| 4 | 20 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,3,3 | 4 | 8 | 128.02 | 125.62 | yes | 710.26 | 0.668x | 0.800x |
| 8 | 20 | merkle_whir_independent | 1/2 | 0 | 3,3,3,3 | - | 8 | 128.02 | 123.74 | yes | 2127.06 | 1.000x | 1.239x |
| 8 | 20 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,3,3,3 | - | 8 | 128.29 | 123.94 | yes | 1409.93 | 0.663x | 0.821x |
| 8 | 20 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,3,3 | - | 8 | 128.02 | 124.79 | yes | 1716.51 | 0.807x | 1.000x |
| 8 | 20 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,3,3 | - | 8 | 128.29 | 125.03 | yes | 1032.64 | 0.485x | 0.602x |
| 8 | 20 | canopy_unbatched | 1/2 | 0 | 3,3,3,3 | 7 | 8 | 128.02 | 123.34 | yes | 2049.89 | 0.964x | 1.194x |
| 8 | 20 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,3,3 | 4 | 8 | 128.02 | 124.81 | yes | 1366.83 | 0.643x | 0.796x |
| 1 | 22 | merkle_whir_independent | 1/2 | 0 | 3,3,4,4 | - | 8 | 128.02 | 126.67 | yes | 311.88 | 1.000x | 1.000x |
| 1 | 22 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,3,4,4 | - | 8 | 128.29 | 126.68 | yes | 207.79 | 0.666x | 0.666x |
| 1 | 22 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,4,4 | - | 8 | 128.02 | 126.67 | yes | 311.88 | 1.000x | 1.000x |
| 1 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,4,4 | - | 8 | 128.29 | 126.68 | yes | 207.79 | 0.666x | 0.666x |
| 1 | 22 | canopy_unbatched | 1/2 | 0 | 3,3,4,4 | 9 | 8 | 128.02 | 126.16 | yes | 293.02 | 0.940x | 0.940x |
| 1 | 22 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,4 | 4 | 8 | 128.02 | 126.59 | yes | 244.97 | 0.785x | 0.785x |
| 2 | 22 | merkle_whir_independent | 1/2 | 0 | 3,3,4,4 | - | 8 | 128.02 | 125.67 | yes | 624.05 | 1.000x | 1.148x |
| 2 | 22 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,3,4,4 | - | 8 | 128.29 | 125.68 | yes | 415.58 | 0.666x | 0.764x |
| 2 | 22 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,4,4 | - | 8 | 128.02 | 126.19 | yes | 543.72 | 0.871x | 1.000x |
| 2 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,4 | - | 8 | 128.29 | 126.27 | yes | 342.69 | 0.549x | 0.630x |
| 2 | 22 | canopy_unbatched | 1/2 | 0 | 3,3,4,4 | 9 | 8 | 128.02 | 125.16 | yes | 586.23 | 0.939x | 1.078x |
| 2 | 22 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,4,4 | 4 | 8 | 128.02 | 126.14 | yes | 417.53 | 0.669x | 0.768x |
| 4 | 22 | merkle_whir_independent | 1/2 | 0 | 3,3,4,4 | - | 8 | 128.02 | 124.67 | yes | 1248.45 | 1.000x | 1.239x |
| 4 | 22 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,3,4,4 | - | 8 | 128.29 | 124.68 | yes | 830.51 | 0.665x | 0.825x |
| 4 | 22 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,4,4 | - | 8 | 128.02 | 125.55 | yes | 1007.28 | 0.807x | 1.000x |
| 4 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,4,4 | - | 8 | 128.29 | 125.69 | yes | 612.80 | 0.491x | 0.608x |
| 4 | 22 | canopy_unbatched | 1/2 | 0 | 3,3,4,4 | 9 | 8 | 128.02 | 124.16 | yes | 1172.38 | 0.939x | 1.164x |
| 4 | 22 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,4,4 | 4 | 8 | 128.02 | 125.51 | yes | 762.48 | 0.611x | 0.757x |
| 8 | 22 | merkle_whir_independent | 1/2 | 0 | 3,3,4,4 | - | 8 | 128.02 | 123.67 | yes | 2497.93 | 1.000x | 1.292x |
| 8 | 22 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,3,4,4 | - | 8 | 128.29 | 123.68 | yes | 1661.73 | 0.665x | 0.860x |
| 8 | 22 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,4,4 | - | 8 | 128.02 | 124.77 | yes | 1932.73 | 0.774x | 1.000x |
| 8 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,4,4 | - | 8 | 128.29 | 124.96 | yes | 1154.05 | 0.462x | 0.597x |
| 8 | 22 | canopy_unbatched | 1/2 | 0 | 3,3,4,4 | 9 | 8 | 128.02 | 123.16 | yes | 2345.15 | 0.939x | 1.213x |
| 8 | 22 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,4,4 | 4 | 8 | 128.02 | 124.75 | yes | 1452.72 | 0.582x | 0.752x |
| 1 | 24 | merkle_whir_independent | 1/2 | 0 | 3,3,3,3,4 | - | 8 | 128.02 | 126.35 | yes | 350.83 | 1.000x | 1.000x |
| 1 | 24 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,3,3,3,4 | - | 8 | 128.17 | 126.43 | yes | 235.15 | 0.670x | 0.670x |
| 1 | 24 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,3,4 | - | 8 | 128.02 | 126.35 | yes | 350.83 | 1.000x | 1.000x |
| 1 | 24 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,3,3,4 | - | 8 | 128.17 | 126.43 | yes | 235.15 | 0.670x | 0.670x |
| 1 | 24 | canopy_unbatched | 1/2 | 0 | 3,3,3,3,4 | 9 | 8 | 128.02 | 125.99 | yes | 325.31 | 0.927x | 0.927x |
| 1 | 24 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3,3,4 | 4 | 8 | 128.02 | 126.35 | yes | 268.22 | 0.765x | 0.765x |
| 2 | 24 | merkle_whir_independent | 1/2 | 0 | 3,3,3,3,4 | - | 8 | 128.02 | 125.35 | yes | 702.24 | 1.000x | 1.157x |
| 2 | 24 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,3,3,3,4 | - | 8 | 128.17 | 125.43 | yes | 470.29 | 0.670x | 0.775x |
| 2 | 24 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,3,3,4 | - | 8 | 128.02 | 125.96 | yes | 606.86 | 0.864x | 1.000x |
| 2 | 24 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,3,3 | - | 8 | 128.29 | 126.19 | yes | 383.17 | 0.546x | 0.631x |
| 2 | 24 | canopy_unbatched | 1/2 | 0 | 3,3,3,3,4 | 9 | 8 | 128.02 | 124.99 | yes | 650.67 | 0.927x | 1.072x |
| 2 | 24 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,3,3,4 | 4 | 8 | 128.02 | 125.96 | yes | 449.01 | 0.639x | 0.740x |
| 4 | 24 | merkle_whir_independent | 1/2 | 0 | 3,3,3,3,4 | - | 8 | 128.02 | 124.35 | yes | 1404.21 | 1.000x | 1.255x |
| 4 | 24 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,3,3,3,4 | - | 8 | 128.17 | 124.43 | yes | 940.11 | 0.669x | 0.840x |
| 4 | 24 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,3,3,4 | - | 8 | 128.02 | 125.40 | yes | 1118.59 | 0.797x | 1.000x |
| 4 | 24 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,3,3,4 | - | 8 | 128.17 | 125.56 | yes | 679.05 | 0.484x | 0.607x |
| 4 | 24 | canopy_unbatched | 1/2 | 0 | 3,3,3,3,4 | 9 | 8 | 128.02 | 123.99 | yes | 1300.59 | 0.926x | 1.163x |
| 4 | 24 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,3,3,4 | 4 | 8 | 128.02 | 125.40 | yes | 810.69 | 0.577x | 0.725x |
| 8 | 24 | merkle_whir_independent | 1/2 | 0 | 3,3,3,3,4 | - | 8 | 128.02 | 123.35 | yes | 2806.71 | 1.000x | 1.311x |
| 8 | 24 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,3,3,3,4 | - | 8 | 128.17 | 123.43 | yes | 1880.91 | 0.670x | 0.878x |
| 8 | 24 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,3,3,4 | - | 8 | 128.02 | 124.68 | yes | 2141.44 | 0.763x | 1.000x |
| 8 | 24 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,3,3,4 | - | 8 | 128.17 | 124.88 | yes | 1271.07 | 0.453x | 0.594x |
| 8 | 24 | canopy_unbatched | 1/2 | 0 | 3,3,3,3,4 | 9 | 8 | 128.02 | 122.99 | yes | 2601.35 | 0.927x | 1.215x |
| 8 | 24 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,3,3,4 | 4 | 8 | 128.02 | 124.68 | yes | 1534.37 | 0.547x | 0.717x |
| 1 | 26 | merkle_whir_independent | 1/2 | 0 | 3,3,3,3,3,3 | - | 8 | 128.02 | 126.26 | yes | 392.74 | 1.000x | 1.000x |
| 1 | 26 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,4,4,4,3 | - | 8 | 128.17 | 126.42 | yes | 264.00 | 0.672x | 0.672x |
| 1 | 26 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,3,3,3 | - | 8 | 128.02 | 126.26 | yes | 392.74 | 1.000x | 1.000x |
| 1 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,4,3 | - | 8 | 128.17 | 126.42 | yes | 264.00 | 0.672x | 0.672x |
| 1 | 26 | canopy_unbatched | 1/2 | 0 | 3,4,4,4,3 | 9 | 8 | 128.02 | 125.95 | yes | 359.05 | 0.914x | 0.914x |
| 1 | 26 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,4,4 | 4 | 8 | 128.02 | 126.49 | yes | 292.66 | 0.745x | 0.745x |
| 2 | 26 | merkle_whir_independent | 1/2 | 0 | 3,3,3,3,3,3 | - | 8 | 128.02 | 125.26 | yes | 785.60 | 1.000x | 1.168x |
| 2 | 26 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,4,4,4,3 | - | 8 | 128.17 | 125.42 | yes | 527.87 | 0.672x | 0.785x |
| 2 | 26 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,3,3,3,3 | - | 8 | 128.02 | 125.89 | yes | 672.81 | 0.856x | 1.000x |
| 2 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,4,4 | - | 8 | 128.29 | 126.19 | yes | 424.67 | 0.541x | 0.631x |
| 2 | 26 | canopy_unbatched | 1/2 | 0 | 3,4,4,4,3 | 9 | 8 | 128.02 | 124.95 | yes | 718.13 | 0.914x | 1.067x |
| 2 | 26 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,4,4,4 | 4 | 8 | 128.02 | 126.07 | yes | 481.91 | 0.613x | 0.716x |
| 4 | 26 | merkle_whir_independent | 1/2 | 0 | 3,3,3,3,3,3 | - | 8 | 128.02 | 124.26 | yes | 1571.32 | 1.000x | 1.275x |
| 4 | 26 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,3,4,4,4 | - | 8 | 128.29 | 124.58 | yes | 1056.22 | 0.672x | 0.857x |
| 4 | 26 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,3,3,3,3 | - | 8 | 128.02 | 125.35 | yes | 1232.83 | 0.785x | 1.000x |
| 4 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,4,4,4 | - | 8 | 128.29 | 125.64 | yes | 746.24 | 0.475x | 0.605x |
| 4 | 26 | canopy_unbatched | 1/2 | 0 | 3,4,4,4,3 | 9 | 8 | 128.02 | 123.95 | yes | 1436.30 | 0.914x | 1.165x |
| 4 | 26 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,4,4,4 | 4 | 8 | 128.02 | 125.47 | yes | 860.18 | 0.547x | 0.698x |
| 8 | 26 | merkle_whir_independent | 1/2 | 0 | 3,3,3,3,3,3 | - | 8 | 128.02 | 123.26 | yes | 3143.09 | 1.000x | 1.336x |
| 8 | 26 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,4,4,4,3 | - | 8 | 128.17 | 123.42 | yes | 2111.32 | 0.672x | 0.897x |
| 8 | 26 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,3,3,3,3 | - | 8 | 128.02 | 124.65 | yes | 2353.24 | 0.749x | 1.000x |
| 8 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,4,4,4 | - | 8 | 128.29 | 124.93 | yes | 1389.76 | 0.442x | 0.591x |
| 8 | 26 | canopy_unbatched | 1/2 | 0 | 3,4,4,4,3 | 9 | 8 | 128.02 | 122.95 | yes | 2872.15 | 0.914x | 1.221x |
| 8 | 26 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,4,4,4 | 4 | 8 | 128.02 | 124.72 | yes | 1617.32 | 0.515x | 0.687x |
| 1 | 28 | merkle_whir_independent | 1/2 | 0 | 3,3,4,3,4,3 | - | 8 | 128.02 | 125.98 | yes | 434.90 | 1.000x | 1.000x |
| 1 | 28 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,4,4,3,3,3 | - | 8 | 128.17 | 126.63 | yes | 292.97 | 0.674x | 0.674x |
| 1 | 28 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,4,3,4,3 | - | 8 | 128.02 | 125.98 | yes | 434.90 | 1.000x | 1.000x |
| 1 | 28 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,3,3,3 | - | 8 | 128.17 | 126.63 | yes | 292.97 | 0.674x | 0.674x |
| 1 | 28 | canopy_unbatched | 1/2 | 0 | 3,4,4,3,3,3 | 10 | 8 | 128.02 | 125.89 | yes | 390.79 | 0.899x | 0.899x |
| 1 | 28 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,4,3,3 | 4 | 8 | 128.02 | 126.45 | yes | 317.07 | 0.729x | 0.729x |
| 2 | 28 | merkle_whir_independent | 1/2 | 0 | 3,3,4,3,4,3 | - | 8 | 128.02 | 124.98 | yes | 869.63 | 1.000x | 1.177x |
| 2 | 28 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,4,4,3,3,3 | - | 8 | 128.17 | 125.63 | yes | 586.00 | 0.674x | 0.793x |
| 2 | 28 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,4,3,4,3 | - | 8 | 128.02 | 125.66 | yes | 739.11 | 0.850x | 1.000x |
| 2 | 28 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,4,3,3 | - | 8 | 128.29 | 126.16 | yes | 466.58 | 0.537x | 0.631x |
| 2 | 28 | canopy_unbatched | 1/2 | 0 | 3,4,4,3,3,3 | 10 | 8 | 128.02 | 124.89 | yes | 781.64 | 0.899x | 1.058x |
| 2 | 28 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,4,4,3,3 | 4 | 8 | 128.02 | 126.04 | yes | 514.72 | 0.592x | 0.696x |
| 4 | 28 | merkle_whir_independent | 1/2 | 0 | 3,3,4,3,4,3 | - | 8 | 128.02 | 123.98 | yes | 1739.32 | 1.000x | 1.290x |
| 4 | 28 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,4,4,3,3,3 | - | 8 | 128.17 | 124.63 | yes | 1171.70 | 0.674x | 0.869x |
| 4 | 28 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,4,3,4,3 | - | 8 | 128.02 | 125.19 | yes | 1347.88 | 0.775x | 1.000x |
| 4 | 28 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,4,4,3,3 | - | 8 | 128.29 | 125.62 | yes | 813.89 | 0.468x | 0.604x |
| 4 | 28 | canopy_unbatched | 1/2 | 0 | 3,4,4,3,3,3 | 10 | 8 | 128.02 | 123.89 | yes | 1563.08 | 0.899x | 1.160x |
| 4 | 28 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,4,4,3,3 | 4 | 8 | 128.02 | 125.45 | yes | 910.29 | 0.523x | 0.675x |
| 8 | 28 | merkle_whir_independent | 1/2 | 0 | 3,3,4,3,4,3 | - | 8 | 128.02 | 122.98 | yes | 3480.23 | 1.000x | 1.357x |
| 8 | 28 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,4,4,3,3,3 | - | 8 | 128.17 | 123.63 | yes | 2343.04 | 0.673x | 0.914x |
| 8 | 28 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,4,3,4,3 | - | 8 | 128.02 | 124.55 | yes | 2564.70 | 0.737x | 1.000x |
| 8 | 28 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,4,4,3,3 | - | 8 | 128.29 | 124.91 | yes | 1506.91 | 0.433x | 0.588x |
| 8 | 28 | canopy_unbatched | 1/2 | 0 | 3,4,4,3,3,3 | 10 | 8 | 128.02 | 122.89 | yes | 3126.15 | 0.898x | 1.219x |
| 8 | 28 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,4,4,3,3 | 4 | 8 | 128.02 | 124.71 | yes | 1699.70 | 0.488x | 0.663x |
| 1 | 30 | merkle_whir_independent | 1/2 | 0 | 3,4,4,4,4,4 | - | 7 | 128.02 | 126.38 | yes | 475.50 | 1.000x | 1.000x |
| 1 | 30 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,4,4,4,4,4 | - | 7 | 128.17 | 126.42 | yes | 318.97 | 0.671x | 0.671x |
| 1 | 30 | merkle_whir_batched_after_second | 1/2 | 1 | 3,4,4,4,4,4 | - | 7 | 128.02 | 126.38 | yes | 475.50 | 1.000x | 1.000x |
| 1 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,4,4,4 | - | 7 | 128.17 | 126.42 | yes | 318.97 | 0.671x | 0.671x |
| 1 | 30 | canopy_unbatched | 1/2 | 0 | 3,4,4,4,4,4 | 11 | 7 | 128.02 | 126.22 | yes | 421.04 | 0.885x | 0.885x |
| 1 | 30 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,4,4,4 | 4 | 8 | 128.02 | 126.48 | yes | 340.76 | 0.717x | 0.717x |
| 2 | 30 | merkle_whir_independent | 1/2 | 0 | 3,4,4,4,4,4 | - | 7 | 128.02 | 125.38 | yes | 950.93 | 1.000x | 1.182x |
| 2 | 30 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,4,4,4,4,4 | - | 7 | 128.17 | 125.42 | yes | 638.04 | 0.671x | 0.793x |
| 2 | 30 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,4,4,4,4 | - | 8 | 128.02 | 126.08 | yes | 804.82 | 0.846x | 1.000x |
| 2 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,4,4,4 | - | 8 | 128.29 | 126.19 | yes | 507.43 | 0.534x | 0.630x |
| 2 | 30 | canopy_unbatched | 1/2 | 0 | 3,4,4,4,4,4 | 11 | 7 | 128.02 | 125.22 | yes | 842.35 | 0.886x | 1.047x |
| 2 | 30 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,4,4,4,4 | 4 | 8 | 128.02 | 126.06 | yes | 546.81 | 0.575x | 0.679x |
| 4 | 30 | merkle_whir_independent | 1/2 | 0 | 3,4,4,4,4,4 | - | 7 | 128.02 | 124.38 | yes | 1901.76 | 1.000x | 1.301x |
| 4 | 30 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,4,4,4,4,4 | - | 7 | 128.17 | 124.42 | yes | 1276.10 | 0.671x | 0.873x |
| 4 | 30 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,4,4,4,4 | - | 8 | 128.02 | 125.48 | yes | 1461.93 | 0.769x | 1.000x |
| 4 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,4,4,4,4 | - | 8 | 128.29 | 125.63 | yes | 880.21 | 0.463x | 0.602x |
| 4 | 30 | canopy_unbatched | 1/2 | 0 | 3,4,4,4,4,4 | 11 | 7 | 128.02 | 124.22 | yes | 1684.68 | 0.886x | 1.152x |
| 4 | 30 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,4,4,4,4 | 4 | 8 | 128.02 | 125.46 | yes | 958.67 | 0.504x | 0.656x |
| 8 | 30 | merkle_whir_independent | 1/2 | 0 | 3,4,4,4,4,4 | - | 7 | 128.02 | 123.38 | yes | 3803.34 | 1.000x | 1.370x |
| 8 | 30 | merkle_whir_independent_rate_quarter | 1/4 | 0 | 3,4,4,4,4,4 | - | 7 | 128.17 | 123.42 | yes | 2553.32 | 0.671x | 0.919x |
| 8 | 30 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,4,4,4,4 | - | 8 | 128.02 | 124.73 | yes | 2776.88 | 0.730x | 1.000x |
| 8 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,4,4,4,4 | - | 8 | 128.29 | 124.92 | yes | 1624.38 | 0.427x | 0.585x |
| 8 | 30 | canopy_unbatched | 1/2 | 0 | 3,4,4,4,4,4 | 11 | 7 | 128.02 | 123.22 | yes | 3368.67 | 0.886x | 1.213x |
| 8 | 30 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,4,4,4,4 | 4 | 8 | 128.02 | 124.72 | yes | 1782.59 | 0.469x | 0.642x |

## Component breakdown

All entries are KiB.

| t | log2 n | scheme | roots | auth | OOD | sumcheck | batching | answers | coefficients | total |
|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 18 | merkle_whir_independent | 0.09 | 97.99 | 0.09 | 1.69 | 0.00 | 110.05 | 16.00 | 225.91 |
| 1 | 18 | merkle_whir_independent_rate_quarter | 0.09 | 69.71 | 0.09 | 1.69 | 0.00 | 62.21 | 16.00 | 149.79 |
| 1 | 18 | merkle_whir_batched_after_second | 0.09 | 97.99 | 0.09 | 1.69 | 0.00 | 110.05 | 16.00 | 225.91 |
| 1 | 18 | merkle_whir_batched_rate_quarter_after_second | 0.09 | 69.71 | 0.09 | 1.69 | 0.00 | 62.21 | 16.00 | 149.79 |
| 1 | 18 | canopy_unbatched | 0.09 | 101.81 | 0.09 | 1.78 | 0.00 | 102.53 | 16.06 | 222.37 |
| 1 | 18 | canopy_schwartz_zippel | 0.09 | 59.06 | 0.12 | 1.69 | 0.03 | 118.81 | 16.00 | 195.81 |
| 2 | 18 | merkle_whir_independent | 0.19 | 195.80 | 0.19 | 3.38 | 0.00 | 220.01 | 32.00 | 451.56 |
| 2 | 18 | merkle_whir_independent_rate_quarter | 0.19 | 139.32 | 0.19 | 3.38 | 0.00 | 124.42 | 32.00 | 299.49 |
| 2 | 18 | merkle_whir_batched_after_second | 0.16 | 183.61 | 0.16 | 1.97 | 0.03 | 206.81 | 16.00 | 408.73 |
| 2 | 18 | merkle_whir_batched_rate_quarter_after_second | 0.16 | 127.51 | 0.16 | 1.97 | 0.03 | 113.42 | 16.00 | 259.25 |
| 2 | 18 | canopy_unbatched | 0.19 | 203.63 | 0.19 | 3.56 | 0.00 | 205.13 | 32.12 | 444.82 |
| 2 | 18 | canopy_schwartz_zippel | 0.16 | 106.33 | 0.22 | 1.97 | 0.16 | 226.69 | 16.00 | 351.52 |
| 4 | 18 | merkle_whir_independent | 0.38 | 391.76 | 0.38 | 6.75 | 0.00 | 440.03 | 64.00 | 903.29 |
| 4 | 18 | merkle_whir_independent_rate_quarter | 0.38 | 278.95 | 0.38 | 6.75 | 0.00 | 248.92 | 64.00 | 599.37 |
| 4 | 18 | merkle_whir_batched_after_second | 0.28 | 354.98 | 0.28 | 2.53 | 0.16 | 400.37 | 16.00 | 774.60 |
| 4 | 18 | merkle_whir_batched_rate_quarter_after_second | 0.28 | 243.04 | 0.28 | 2.53 | 0.16 | 215.88 | 16.00 | 478.17 |
| 4 | 18 | canopy_unbatched | 0.38 | 406.75 | 0.38 | 7.12 | 0.00 | 410.14 | 64.25 | 889.02 |
| 4 | 18 | canopy_schwartz_zippel | 0.28 | 200.52 | 0.41 | 2.53 | 0.41 | 442.18 | 16.00 | 662.32 |
| 8 | 18 | merkle_whir_independent | 0.75 | 783.99 | 0.75 | 13.50 | 0.00 | 880.34 | 128.00 | 1807.33 |
| 8 | 18 | merkle_whir_independent_rate_quarter | 0.75 | 557.96 | 0.75 | 13.50 | 0.00 | 497.78 | 128.00 | 1198.74 |
| 8 | 18 | merkle_whir_batched_after_second | 0.53 | 697.46 | 0.53 | 3.66 | 0.41 | 787.81 | 16.00 | 1506.40 |
| 8 | 18 | merkle_whir_batched_rate_quarter_after_second | 0.53 | 474.34 | 0.53 | 3.66 | 0.41 | 420.82 | 16.00 | 916.29 |
| 8 | 18 | canopy_unbatched | 0.75 | 814.07 | 0.75 | 14.25 | 0.00 | 820.07 | 128.50 | 1778.39 |
| 8 | 18 | canopy_schwartz_zippel | 0.53 | 389.38 | 0.78 | 3.66 | 0.91 | 873.25 | 16.00 | 1284.51 |
| 1 | 20 | merkle_whir_independent | 0.12 | 136.04 | 0.12 | 1.88 | 0.00 | 119.69 | 8.00 | 265.86 |
| 1 | 20 | merkle_whir_independent_rate_quarter | 0.12 | 95.70 | 0.12 | 1.88 | 0.00 | 70.49 | 8.00 | 176.32 |
| 1 | 20 | merkle_whir_batched_after_second | 0.12 | 136.04 | 0.12 | 1.88 | 0.00 | 119.69 | 8.00 | 265.86 |
| 1 | 20 | merkle_whir_batched_rate_quarter_after_second | 0.12 | 95.70 | 0.12 | 1.88 | 0.00 | 70.49 | 8.00 | 176.32 |
| 1 | 20 | canopy_unbatched | 0.16 | 109.49 | 0.19 | 3.19 | 0.00 | 127.21 | 16.00 | 256.23 |
| 1 | 20 | canopy_schwartz_zippel | 0.12 | 80.72 | 0.16 | 1.88 | 0.03 | 127.20 | 8.00 | 218.12 |
| 2 | 20 | merkle_whir_independent | 0.25 | 272.27 | 0.25 | 3.75 | 0.00 | 239.34 | 16.00 | 531.86 |
| 2 | 20 | merkle_whir_independent_rate_quarter | 0.25 | 191.38 | 0.25 | 3.75 | 0.00 | 140.98 | 16.00 | 352.61 |
| 2 | 20 | merkle_whir_batched_after_second | 0.19 | 246.29 | 0.19 | 2.16 | 0.03 | 216.63 | 8.00 | 473.49 |
| 2 | 20 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 166.31 | 0.19 | 2.16 | 0.03 | 121.74 | 8.00 | 298.61 |
| 2 | 20 | canopy_unbatched | 0.31 | 219.21 | 0.38 | 6.38 | 0.00 | 254.38 | 32.00 | 512.65 |
| 2 | 20 | canopy_schwartz_zippel | 0.19 | 136.03 | 0.25 | 2.16 | 0.16 | 235.12 | 8.00 | 381.90 |
| 4 | 20 | merkle_whir_independent | 0.50 | 544.48 | 0.50 | 7.50 | 0.00 | 478.85 | 32.00 | 1063.83 |
| 4 | 20 | merkle_whir_independent_rate_quarter | 0.50 | 382.56 | 0.50 | 7.50 | 0.00 | 281.94 | 32.00 | 705.00 |
| 4 | 20 | merkle_whir_batched_after_second | 0.31 | 465.65 | 0.31 | 2.72 | 0.16 | 410.50 | 8.00 | 887.65 |
| 4 | 20 | merkle_whir_batched_rate_quarter_after_second | 0.31 | 307.50 | 0.31 | 2.72 | 0.16 | 224.22 | 8.00 | 543.22 |
| 4 | 20 | canopy_unbatched | 0.62 | 438.60 | 0.75 | 12.75 | 0.00 | 508.88 | 64.00 | 1025.61 |
| 4 | 20 | canopy_schwartz_zippel | 0.31 | 247.24 | 0.44 | 2.72 | 0.41 | 451.14 | 8.00 | 710.26 |
| 8 | 20 | merkle_whir_independent | 1.00 | 1088.36 | 1.00 | 15.00 | 0.00 | 957.70 | 64.00 | 2127.06 |
| 8 | 20 | merkle_whir_independent_rate_quarter | 1.00 | 765.05 | 1.00 | 15.00 | 0.00 | 563.88 | 64.00 | 1409.93 |
| 8 | 20 | merkle_whir_batched_after_second | 0.56 | 905.02 | 0.56 | 3.84 | 0.41 | 798.11 | 8.00 | 1716.51 |
| 8 | 20 | merkle_whir_batched_rate_quarter_after_second | 0.56 | 590.06 | 0.56 | 3.84 | 0.41 | 429.21 | 8.00 | 1032.64 |
| 8 | 20 | canopy_unbatched | 1.25 | 876.12 | 1.50 | 25.50 | 0.00 | 1017.52 | 128.00 | 2049.89 |
| 8 | 20 | canopy_schwartz_zippel | 0.56 | 469.92 | 0.81 | 3.84 | 0.91 | 882.79 | 8.00 | 1366.83 |
| 1 | 22 | merkle_whir_independent | 0.12 | 161.60 | 0.12 | 2.06 | 0.00 | 139.97 | 8.00 | 311.88 |
| 1 | 22 | merkle_whir_independent_rate_quarter | 0.12 | 109.74 | 0.12 | 2.06 | 0.00 | 87.74 | 8.00 | 207.79 |
| 1 | 22 | merkle_whir_batched_after_second | 0.12 | 161.60 | 0.12 | 2.06 | 0.00 | 139.97 | 8.00 | 311.88 |
| 1 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.12 | 109.74 | 0.12 | 2.06 | 0.00 | 87.74 | 8.00 | 207.79 |
| 1 | 22 | canopy_unbatched | 0.16 | 132.32 | 0.19 | 3.38 | 0.00 | 140.99 | 16.00 | 293.02 |
| 1 | 22 | canopy_schwartz_zippel | 0.12 | 90.12 | 0.16 | 2.06 | 0.03 | 144.47 | 8.00 | 244.97 |
| 2 | 22 | merkle_whir_independent | 0.25 | 323.48 | 0.25 | 4.12 | 0.00 | 279.95 | 16.00 | 624.05 |
| 2 | 22 | merkle_whir_independent_rate_quarter | 0.25 | 219.46 | 0.25 | 4.12 | 0.00 | 175.49 | 16.00 | 415.58 |
| 2 | 22 | merkle_whir_batched_after_second | 0.19 | 296.01 | 0.19 | 2.34 | 0.03 | 236.96 | 8.00 | 543.72 |
| 2 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 192.95 | 0.19 | 2.34 | 0.03 | 138.99 | 8.00 | 342.69 |
| 2 | 22 | canopy_unbatched | 0.31 | 264.80 | 0.38 | 6.75 | 0.00 | 282.00 | 32.00 | 586.23 |
| 2 | 22 | canopy_schwartz_zippel | 0.19 | 154.11 | 0.25 | 2.34 | 0.16 | 252.49 | 8.00 | 417.53 |
| 4 | 22 | merkle_whir_independent | 0.50 | 647.30 | 0.50 | 8.25 | 0.00 | 559.90 | 32.00 | 1248.45 |
| 4 | 22 | merkle_whir_independent_rate_quarter | 0.50 | 438.27 | 0.50 | 8.25 | 0.00 | 350.99 | 32.00 | 830.51 |
| 4 | 22 | merkle_whir_batched_after_second | 0.31 | 564.65 | 0.31 | 2.91 | 0.16 | 430.94 | 8.00 | 1007.28 |
| 4 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.31 | 359.63 | 0.31 | 2.91 | 0.16 | 241.49 | 8.00 | 612.80 |
| 4 | 22 | canopy_unbatched | 0.62 | 529.55 | 0.75 | 13.50 | 0.00 | 563.96 | 64.00 | 1172.38 |
| 4 | 22 | canopy_schwartz_zippel | 0.31 | 281.98 | 0.44 | 2.91 | 0.41 | 468.44 | 8.00 | 762.48 |
| 8 | 22 | merkle_whir_independent | 1.00 | 1295.55 | 1.00 | 16.50 | 0.00 | 1119.88 | 64.00 | 2497.93 |
| 8 | 22 | merkle_whir_independent_rate_quarter | 1.00 | 877.27 | 1.00 | 16.50 | 0.00 | 701.96 | 64.00 | 1661.73 |
| 8 | 22 | merkle_whir_batched_after_second | 0.56 | 1100.26 | 0.56 | 4.03 | 0.41 | 818.90 | 8.00 | 1932.73 |
| 8 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.56 | 693.98 | 0.56 | 4.03 | 0.41 | 446.50 | 8.00 | 1154.05 |
| 8 | 22 | canopy_unbatched | 1.25 | 1059.54 | 1.50 | 27.00 | 0.00 | 1127.86 | 128.00 | 2345.15 |
| 8 | 22 | canopy_schwartz_zippel | 0.56 | 538.05 | 0.81 | 4.03 | 0.91 | 900.36 | 8.00 | 1452.72 |
| 1 | 24 | merkle_whir_independent | 0.16 | 206.03 | 0.16 | 2.25 | 0.00 | 134.25 | 8.00 | 350.83 |
| 1 | 24 | merkle_whir_independent_rate_quarter | 0.16 | 141.10 | 0.16 | 2.25 | 0.00 | 83.50 | 8.00 | 235.15 |
| 1 | 24 | merkle_whir_batched_after_second | 0.16 | 206.03 | 0.16 | 2.25 | 0.00 | 134.25 | 8.00 | 350.83 |
| 1 | 24 | merkle_whir_batched_rate_quarter_after_second | 0.16 | 141.10 | 0.16 | 2.25 | 0.00 | 83.50 | 8.00 | 235.15 |
| 1 | 24 | canopy_unbatched | 0.19 | 156.16 | 0.22 | 3.75 | 0.00 | 148.99 | 16.00 | 325.31 |
| 1 | 24 | canopy_schwartz_zippel | 0.16 | 117.35 | 0.19 | 2.25 | 0.03 | 140.24 | 8.00 | 268.22 |
| 2 | 24 | merkle_whir_independent | 0.31 | 412.62 | 0.31 | 4.50 | 0.00 | 268.50 | 16.00 | 702.24 |
| 2 | 24 | merkle_whir_independent_rate_quarter | 0.31 | 282.17 | 0.31 | 4.50 | 0.00 | 167.00 | 16.00 | 470.29 |
| 2 | 24 | merkle_whir_batched_after_second | 0.22 | 364.62 | 0.22 | 2.53 | 0.03 | 231.25 | 8.00 | 606.86 |
| 2 | 24 | merkle_whir_batched_rate_quarter_after_second | 0.22 | 234.42 | 0.22 | 2.53 | 0.03 | 137.75 | 8.00 | 383.17 |
| 2 | 24 | canopy_unbatched | 0.38 | 312.38 | 0.44 | 7.50 | 0.00 | 297.99 | 32.00 | 650.67 |
| 2 | 24 | canopy_schwartz_zippel | 0.22 | 189.58 | 0.28 | 2.53 | 0.16 | 248.24 | 8.00 | 449.01 |
| 4 | 24 | merkle_whir_independent | 0.62 | 824.97 | 0.62 | 9.00 | 0.00 | 536.99 | 32.00 | 1404.21 |
| 4 | 24 | merkle_whir_independent_rate_quarter | 0.62 | 563.86 | 0.62 | 9.00 | 0.00 | 334.00 | 32.00 | 940.11 |
| 4 | 24 | merkle_whir_batched_after_second | 0.34 | 681.40 | 0.34 | 3.09 | 0.16 | 425.25 | 8.00 | 1118.59 |
| 4 | 24 | merkle_whir_batched_rate_quarter_after_second | 0.34 | 429.86 | 0.34 | 3.09 | 0.16 | 237.25 | 8.00 | 679.05 |
| 4 | 24 | canopy_unbatched | 0.75 | 624.00 | 0.88 | 15.00 | 0.00 | 595.97 | 64.00 | 1300.59 |
| 4 | 24 | canopy_schwartz_zippel | 0.34 | 334.15 | 0.47 | 3.09 | 0.41 | 464.22 | 8.00 | 810.69 |
| 8 | 24 | merkle_whir_independent | 1.25 | 1648.29 | 1.25 | 18.00 | 0.00 | 1073.92 | 64.00 | 2806.71 |
| 8 | 24 | merkle_whir_independent_rate_quarter | 1.25 | 1128.41 | 1.25 | 18.00 | 0.00 | 668.00 | 64.00 | 1880.91 |
| 8 | 24 | merkle_whir_batched_after_second | 0.59 | 1314.46 | 0.59 | 4.22 | 0.41 | 813.17 | 8.00 | 2141.44 |
| 8 | 24 | merkle_whir_batched_rate_quarter_after_second | 0.59 | 815.01 | 0.59 | 4.22 | 0.41 | 442.25 | 8.00 | 1271.07 |
| 8 | 24 | canopy_unbatched | 1.50 | 1248.14 | 1.75 | 30.00 | 0.00 | 1191.96 | 128.00 | 2601.35 |
| 8 | 24 | canopy_schwartz_zippel | 0.59 | 623.58 | 0.84 | 4.22 | 0.91 | 896.23 | 8.00 | 1534.37 |
| 1 | 26 | merkle_whir_independent | 0.19 | 248.93 | 0.19 | 2.44 | 0.00 | 133.00 | 8.00 | 392.74 |
| 1 | 26 | merkle_whir_independent_rate_quarter | 0.16 | 148.25 | 0.16 | 2.44 | 0.00 | 105.00 | 8.00 | 264.00 |
| 1 | 26 | merkle_whir_batched_after_second | 0.19 | 248.93 | 0.19 | 2.44 | 0.00 | 133.00 | 8.00 | 392.74 |
| 1 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.16 | 148.25 | 0.16 | 2.44 | 0.00 | 105.00 | 8.00 | 264.00 |
| 1 | 26 | canopy_unbatched | 0.22 | 168.70 | 0.25 | 4.12 | 0.00 | 169.75 | 16.00 | 359.05 |
| 1 | 26 | canopy_schwartz_zippel | 0.16 | 126.34 | 0.19 | 2.44 | 0.03 | 155.50 | 8.00 | 292.66 |
| 2 | 26 | merkle_whir_independent | 0.38 | 497.98 | 0.38 | 4.88 | 0.00 | 266.00 | 16.00 | 785.60 |
| 2 | 26 | merkle_whir_independent_rate_quarter | 0.31 | 296.37 | 0.31 | 4.88 | 0.00 | 210.00 | 16.00 | 527.87 |
| 2 | 26 | merkle_whir_batched_after_second | 0.25 | 431.56 | 0.25 | 2.72 | 0.03 | 230.00 | 8.00 | 672.81 |
| 2 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.22 | 263.48 | 0.22 | 2.72 | 0.03 | 150.00 | 8.00 | 424.67 |
| 2 | 26 | canopy_unbatched | 0.44 | 337.44 | 0.50 | 8.25 | 0.00 | 339.50 | 32.00 | 718.13 |
| 2 | 26 | canopy_schwartz_zippel | 0.22 | 207.04 | 0.28 | 2.72 | 0.16 | 263.50 | 8.00 | 481.91 |
| 4 | 26 | merkle_whir_independent | 0.75 | 996.08 | 0.75 | 9.75 | 0.00 | 531.99 | 32.00 | 1571.32 |
| 4 | 26 | merkle_whir_independent_rate_quarter | 0.62 | 618.22 | 0.62 | 9.75 | 0.00 | 395.00 | 32.00 | 1056.22 |
| 4 | 26 | merkle_whir_batched_after_second | 0.38 | 796.65 | 0.38 | 3.28 | 0.16 | 424.00 | 8.00 | 1232.83 |
| 4 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.34 | 481.62 | 0.34 | 3.28 | 0.16 | 252.50 | 8.00 | 746.24 |
| 4 | 26 | canopy_unbatched | 0.88 | 674.92 | 1.00 | 16.50 | 0.00 | 679.00 | 64.00 | 1436.30 |
| 4 | 26 | canopy_schwartz_zippel | 0.34 | 368.18 | 0.47 | 3.28 | 0.41 | 479.50 | 8.00 | 860.18 |
| 8 | 26 | merkle_whir_independent | 1.50 | 1992.59 | 1.50 | 19.50 | 0.00 | 1064.00 | 64.00 | 3143.09 |
| 8 | 26 | merkle_whir_independent_rate_quarter | 1.25 | 1185.32 | 1.25 | 19.50 | 0.00 | 840.00 | 64.00 | 2111.32 |
| 8 | 26 | merkle_whir_batched_after_second | 0.62 | 1527.20 | 0.62 | 4.41 | 0.41 | 811.98 | 8.00 | 2353.24 |
| 8 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.59 | 918.28 | 0.59 | 4.41 | 0.41 | 457.48 | 8.00 | 1389.76 |
| 8 | 26 | canopy_unbatched | 1.75 | 1349.40 | 2.00 | 33.00 | 0.00 | 1358.00 | 128.00 | 2872.15 |
| 8 | 26 | canopy_schwartz_zippel | 0.59 | 691.07 | 0.84 | 4.41 | 0.91 | 911.50 | 8.00 | 1617.32 |
| 1 | 28 | merkle_whir_independent | 0.19 | 274.15 | 0.19 | 2.62 | 0.00 | 149.75 | 8.00 | 434.90 |
| 1 | 28 | merkle_whir_independent_rate_quarter | 0.19 | 178.22 | 0.19 | 2.62 | 0.00 | 103.75 | 8.00 | 292.97 |
| 1 | 28 | merkle_whir_batched_after_second | 0.19 | 274.15 | 0.19 | 2.62 | 0.00 | 149.75 | 8.00 | 434.90 |
| 1 | 28 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 178.22 | 0.19 | 2.62 | 0.00 | 103.75 | 8.00 | 292.97 |
| 1 | 28 | canopy_unbatched | 0.25 | 198.60 | 0.28 | 4.41 | 0.00 | 175.25 | 12.00 | 390.79 |
| 1 | 28 | canopy_schwartz_zippel | 0.19 | 151.26 | 0.22 | 2.62 | 0.03 | 154.75 | 8.00 | 317.07 |
| 2 | 28 | merkle_whir_independent | 0.38 | 548.13 | 0.38 | 5.25 | 0.00 | 299.50 | 16.00 | 869.63 |
| 2 | 28 | merkle_whir_independent_rate_quarter | 0.38 | 356.50 | 0.38 | 5.25 | 0.00 | 207.50 | 16.00 | 586.00 |
| 2 | 28 | merkle_whir_batched_after_second | 0.25 | 480.93 | 0.25 | 2.91 | 0.03 | 246.75 | 8.00 | 739.11 |
| 2 | 28 | merkle_whir_batched_rate_quarter_after_second | 0.25 | 305.89 | 0.25 | 2.91 | 0.03 | 149.25 | 8.00 | 466.58 |
| 2 | 28 | canopy_unbatched | 0.50 | 397.26 | 0.56 | 8.81 | 0.00 | 350.50 | 24.00 | 781.64 |
| 2 | 28 | canopy_schwartz_zippel | 0.25 | 240.34 | 0.31 | 2.91 | 0.16 | 262.75 | 8.00 | 514.72 |
| 4 | 28 | merkle_whir_independent | 0.75 | 1096.32 | 0.75 | 10.50 | 0.00 | 599.00 | 32.00 | 1739.32 |
| 4 | 28 | merkle_whir_independent_rate_quarter | 0.75 | 712.71 | 0.75 | 10.50 | 0.00 | 415.00 | 32.00 | 1171.70 |
| 4 | 28 | merkle_whir_batched_after_second | 0.38 | 894.75 | 0.38 | 3.47 | 0.16 | 440.75 | 8.00 | 1347.88 |
| 4 | 28 | merkle_whir_batched_rate_quarter_after_second | 0.38 | 549.77 | 0.38 | 3.47 | 0.16 | 251.75 | 8.00 | 813.89 |
| 4 | 28 | canopy_unbatched | 1.00 | 794.33 | 1.12 | 17.62 | 0.00 | 701.00 | 48.00 | 1563.08 |
| 4 | 28 | canopy_schwartz_zippel | 0.38 | 418.79 | 0.50 | 3.47 | 0.41 | 478.75 | 8.00 | 910.29 |
| 8 | 28 | merkle_whir_independent | 1.50 | 2194.23 | 1.50 | 21.00 | 0.00 | 1198.00 | 64.00 | 3480.23 |
| 8 | 28 | merkle_whir_independent_rate_quarter | 1.50 | 1425.04 | 1.50 | 21.00 | 0.00 | 830.00 | 64.00 | 2343.04 |
| 8 | 28 | merkle_whir_batched_after_second | 0.62 | 1721.70 | 0.62 | 4.59 | 0.41 | 828.75 | 8.00 | 2564.70 |
| 8 | 28 | merkle_whir_batched_rate_quarter_after_second | 0.62 | 1035.91 | 0.62 | 4.59 | 0.41 | 456.75 | 8.00 | 1506.91 |
| 8 | 28 | canopy_unbatched | 2.00 | 1588.67 | 2.25 | 35.25 | 0.00 | 1401.98 | 96.00 | 3126.15 |
| 8 | 28 | canopy_schwartz_zippel | 0.62 | 773.97 | 0.88 | 4.59 | 0.91 | 910.73 | 8.00 | 1699.70 |
| 1 | 30 | merkle_whir_independent | 0.19 | 292.31 | 0.19 | 2.81 | 0.00 | 176.00 | 4.00 | 475.50 |
| 1 | 30 | merkle_whir_independent_rate_quarter | 0.19 | 193.28 | 0.19 | 2.81 | 0.00 | 118.50 | 4.00 | 318.97 |
| 1 | 30 | merkle_whir_batched_after_second | 0.19 | 292.31 | 0.19 | 2.81 | 0.00 | 176.00 | 4.00 | 475.50 |
| 1 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 193.28 | 0.19 | 2.81 | 0.00 | 118.50 | 4.00 | 318.97 |
| 1 | 30 | canopy_unbatched | 0.25 | 215.82 | 0.28 | 4.69 | 0.00 | 188.00 | 12.00 | 421.04 |
| 1 | 30 | canopy_schwartz_zippel | 0.19 | 165.01 | 0.22 | 2.81 | 0.03 | 164.50 | 8.00 | 340.76 |
| 2 | 30 | merkle_whir_independent | 0.38 | 584.56 | 0.38 | 5.62 | 0.00 | 352.00 | 8.00 | 950.93 |
| 2 | 30 | merkle_whir_independent_rate_quarter | 0.38 | 386.66 | 0.38 | 5.62 | 0.00 | 237.00 | 8.00 | 638.04 |
| 2 | 30 | merkle_whir_batched_after_second | 0.25 | 534.70 | 0.25 | 3.09 | 0.03 | 258.50 | 8.00 | 804.82 |
| 2 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.25 | 336.80 | 0.25 | 3.09 | 0.03 | 159.00 | 8.00 | 507.43 |
| 2 | 30 | canopy_unbatched | 0.50 | 431.91 | 0.56 | 9.38 | 0.00 | 376.00 | 24.00 | 842.35 |
| 2 | 30 | canopy_schwartz_zippel | 0.25 | 262.50 | 0.31 | 3.09 | 0.16 | 272.50 | 8.00 | 546.81 |
| 4 | 30 | merkle_whir_independent | 0.75 | 1169.01 | 0.75 | 11.25 | 0.00 | 704.00 | 16.00 | 1901.76 |
| 4 | 30 | merkle_whir_independent_rate_quarter | 0.75 | 773.35 | 0.75 | 11.25 | 0.00 | 474.00 | 16.00 | 1276.10 |
| 4 | 30 | merkle_whir_batched_after_second | 0.38 | 996.87 | 0.38 | 3.66 | 0.16 | 452.50 | 8.00 | 1461.93 |
| 4 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.38 | 606.15 | 0.38 | 3.66 | 0.16 | 261.50 | 8.00 | 880.21 |
| 4 | 30 | canopy_unbatched | 1.00 | 863.80 | 1.12 | 18.75 | 0.00 | 752.00 | 48.00 | 1684.68 |
| 4 | 30 | canopy_schwartz_zippel | 0.38 | 457.23 | 0.50 | 3.66 | 0.41 | 488.50 | 8.00 | 958.67 |
| 8 | 30 | merkle_whir_independent | 1.50 | 2337.84 | 1.50 | 22.50 | 0.00 | 1408.00 | 32.00 | 3803.34 |
| 8 | 30 | merkle_whir_independent_rate_quarter | 1.50 | 1547.82 | 1.50 | 22.50 | 0.00 | 948.00 | 32.00 | 2553.32 |
| 8 | 30 | merkle_whir_batched_after_second | 0.62 | 1921.95 | 0.62 | 4.78 | 0.41 | 840.50 | 8.00 | 2776.88 |
| 8 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.62 | 1143.45 | 0.62 | 4.78 | 0.41 | 466.50 | 8.00 | 1624.38 |
| 8 | 30 | canopy_unbatched | 2.00 | 1726.92 | 2.25 | 37.50 | 0.00 | 1504.00 | 96.00 | 3368.67 |
| 8 | 30 | canopy_schwartz_zippel | 0.62 | 846.90 | 0.88 | 4.78 | 0.91 | 920.50 | 8.00 | 1782.59 |

## Oracle queries at n=30

`unique` is the mean number of distinct positions per queried oracle group; authentication and answer columns are total transmitted values across the reported oracle count.

| exp | t | scheme | role | oracles | code | field | requested | unique | auth | answer fields |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | SZ-batched second-oracle claim initial oracle | 1 | [2^30,2^27] | 256 | 90 | 90.00 | 1675.49 | 1440.00 |
| 0 | 1 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^29,2^23] | 256 | 44 | 44.00 | 819.89 | 704.00 |
| 0 | 1 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^28,2^19] | 256 | 29 | 29.00 | 528.23 | 464.00 |
| 0 | 1 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^27,2^15] | 256 | 22 | 22.00 | 388.55 | 352.00 |
| 0 | 1 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^26,2^11] | 256 | 18 | 18.00 | 305.62 | 288.00 |
| 0 | 1 | merkle_whir_batched_after_second | independent first Merkle oracles | 1 | [2^31,2^30] | 256 | 298 | 298.00 | 5627.70 | 2384.00 |
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
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim initial oracle | 1 | [2^31,2^27] | 256 | 67 | 67.00 | 1343.34 | 1072.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^30,2^23] | 256 | 38 | 38.00 | 756.35 | 608.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^29,2^19] | 256 | 26 | 26.00 | 506.41 | 416.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^28,2^15] | 256 | 20 | 20.00 | 376.22 | 320.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^27,2^11] | 256 | 17 | 17.00 | 307.23 | 272.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | independent first Merkle oracles | 1 | [2^32,2^30] | 256 | 138 | 138.00 | 2899.53 | 1104.00 |
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
| 0 | 1 | merkle_whir_independent | independent WHIR initial oracle | 1 | [2^31,2^30] | 256 | 298 | 298.00 | 5630.14 | 2384.00 |
| 0 | 1 | merkle_whir_independent | independent WHIR recursive oracle 1 | 1 | [2^30,2^27] | 256 | 90 | 90.00 | 1677.21 | 1440.00 |
| 0 | 1 | merkle_whir_independent | independent WHIR recursive oracle 2 | 1 | [2^29,2^23] | 256 | 44 | 44.00 | 821.84 | 704.00 |
| 0 | 1 | merkle_whir_independent | independent WHIR recursive oracle 3 | 1 | [2^28,2^19] | 256 | 29 | 29.00 | 530.15 | 464.00 |
| 0 | 1 | merkle_whir_independent | independent WHIR recursive oracle 4 | 1 | [2^27,2^15] | 256 | 22 | 22.00 | 389.41 | 352.00 |
| 0 | 1 | merkle_whir_independent | independent WHIR recursive oracle 5 | 1 | [2^26,2^11] | 256 | 18 | 18.00 | 305.12 | 288.00 |
| 0 | 2 | merkle_whir_independent | independent WHIR initial oracle | 2 | [2^31,2^30] | 256 | 298 | 298.00 | 11260.20 | 4768.00 |
| 0 | 2 | merkle_whir_independent | independent WHIR recursive oracle 1 | 2 | [2^30,2^27] | 256 | 90 | 90.00 | 3356.10 | 2880.00 |
| 0 | 2 | merkle_whir_independent | independent WHIR recursive oracle 2 | 2 | [2^29,2^23] | 256 | 44 | 44.00 | 1641.08 | 1408.00 |
| 0 | 2 | merkle_whir_independent | independent WHIR recursive oracle 3 | 2 | [2^28,2^19] | 256 | 29 | 29.00 | 1060.90 | 928.00 |
| 0 | 2 | merkle_whir_independent | independent WHIR recursive oracle 4 | 2 | [2^27,2^15] | 256 | 22 | 22.00 | 777.32 | 704.00 |
| 0 | 2 | merkle_whir_independent | independent WHIR recursive oracle 5 | 2 | [2^26,2^11] | 256 | 18 | 18.00 | 610.26 | 576.00 |
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
| 0 | 1 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR initial oracle | 1 | [2^32,2^30] | 256 | 138 | 138.00 | 2898.42 | 1104.00 |
| 0 | 1 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 1 | 1 | [2^31,2^27] | 256 | 67 | 67.00 | 1343.77 | 1072.00 |
| 0 | 1 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 2 | 1 | [2^30,2^23] | 256 | 38 | 38.00 | 754.37 | 608.00 |
| 0 | 1 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 3 | 1 | [2^29,2^19] | 256 | 26 | 26.00 | 504.43 | 416.00 |
| 0 | 1 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 4 | 1 | [2^28,2^15] | 256 | 20 | 20.00 | 376.91 | 320.00 |
| 0 | 1 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 5 | 1 | [2^27,2^11] | 256 | 17 | 17.00 | 307.16 | 272.00 |
| 0 | 2 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR initial oracle | 2 | [2^32,2^30] | 256 | 138 | 138.00 | 5801.34 | 2208.00 |
| 0 | 2 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 1 | 2 | [2^31,2^27] | 256 | 67 | 67.00 | 2685.48 | 2144.00 |
| 0 | 2 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 2 | 2 | [2^30,2^23] | 256 | 38 | 38.00 | 1510.44 | 1216.00 |
| 0 | 2 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 3 | 2 | [2^29,2^19] | 256 | 26 | 26.00 | 1009.68 | 832.00 |
| 0 | 2 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 4 | 2 | [2^28,2^15] | 256 | 20 | 20.00 | 753.60 | 640.00 |
| 0 | 2 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 5 | 2 | [2^27,2^11] | 256 | 17 | 17.00 | 612.58 | 544.00 |
| 0 | 4 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR initial oracle | 4 | [2^32,2^30] | 256 | 138 | 138.00 | 11590.80 | 4416.00 |
| 0 | 4 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 1 | 4 | [2^31,2^27] | 256 | 67 | 67.00 | 5369.40 | 4288.00 |
| 0 | 4 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 2 | 4 | [2^30,2^23] | 256 | 38 | 38.00 | 3023.80 | 2432.00 |
| 0 | 4 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 3 | 4 | [2^29,2^19] | 256 | 26 | 26.00 | 2025.72 | 1664.00 |
| 0 | 4 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 4 | 4 | [2^28,2^15] | 256 | 20 | 20.00 | 1505.16 | 1280.00 |
| 0 | 4 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 5 | 4 | [2^27,2^11] | 256 | 17 | 17.00 | 1232.32 | 1088.00 |
| 0 | 8 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR initial oracle | 8 | [2^32,2^30] | 256 | 138 | 138.00 | 23214.16 | 8832.00 |
| 0 | 8 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 1 | 8 | [2^31,2^27] | 256 | 67 | 67.00 | 10760.64 | 8576.00 |
| 0 | 8 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 2 | 8 | [2^30,2^23] | 256 | 38 | 38.00 | 6043.92 | 4864.00 |
| 0 | 8 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 3 | 8 | [2^29,2^19] | 256 | 26 | 26.00 | 4046.40 | 3328.00 |
| 0 | 8 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 4 | 8 | [2^28,2^15] | 256 | 20 | 20.00 | 3008.40 | 2560.00 |
| 0 | 8 | merkle_whir_independent_rate_quarter | independent rate-1/4 WHIR recursive oracle 5 | 8 | [2^27,2^11] | 256 | 17 | 17.00 | 2456.72 | 2176.00 |
| 2 | 1 | canopy_unbatched | Canopy claim WHIR initial oracle | 1 | [2^31,2^20] | 256 | 24 | 24.00 | 518.19 | 384.00 |
| 2 | 1 | canopy_unbatched | Canopy claim WHIR recursive oracle 1 | 1 | [2^30,2^16] | 256 | 19 | 19.00 | 395.93 | 304.00 |
| 2 | 1 | canopy_unbatched | Canopy claim WHIR recursive oracle 2 | 1 | [2^29,2^12] | 256 | 16 | 16.00 | 321.99 | 256.00 |
| 2 | 1 | canopy_unbatched | outer first oracle via Canopy | 1 | [2^31,2^30] | 256 | 298 | 298.00 | 2383.74 | 2384.00 |
| 2 | 1 | canopy_unbatched | outer second-oracle WHIR initial oracle | 1 | [2^31,2^27] | 256 | 67 | 67.00 | 1340.96 | 1072.00 |
| 2 | 1 | canopy_unbatched | outer second-oracle WHIR recursive oracle 1 | 1 | [2^30,2^23] | 256 | 38 | 38.00 | 755.97 | 608.00 |
| 2 | 1 | canopy_unbatched | outer second-oracle WHIR recursive oracle 2 | 1 | [2^29,2^19] | 256 | 26 | 26.00 | 506.06 | 416.00 |
| 2 | 1 | canopy_unbatched | outer second-oracle WHIR recursive oracle 3 | 1 | [2^28,2^15] | 256 | 20 | 20.00 | 376.59 | 320.00 |
| 2 | 1 | canopy_unbatched | outer second-oracle WHIR recursive oracle 4 | 1 | [2^27,2^11] | 256 | 17 | 17.00 | 306.82 | 272.00 |
| 2 | 2 | canopy_unbatched | Canopy claim WHIR initial oracle | 2 | [2^31,2^20] | 256 | 24 | 24.00 | 1036.38 | 768.00 |
| 2 | 2 | canopy_unbatched | Canopy claim WHIR recursive oracle 1 | 2 | [2^30,2^16] | 256 | 19 | 19.00 | 791.86 | 608.00 |
| 2 | 2 | canopy_unbatched | Canopy claim WHIR recursive oracle 2 | 2 | [2^29,2^12] | 256 | 16 | 16.00 | 643.98 | 512.00 |
| 2 | 2 | canopy_unbatched | outer first oracle via Canopy | 2 | [2^31,2^30] | 256 | 298 | 298.00 | 4767.50 | 4768.00 |
| 2 | 2 | canopy_unbatched | outer second-oracle WHIR initial oracle | 2 | [2^31,2^27] | 256 | 67 | 67.00 | 2690.94 | 2144.00 |
| 2 | 2 | canopy_unbatched | outer second-oracle WHIR recursive oracle 1 | 2 | [2^30,2^23] | 256 | 38 | 38.00 | 1509.80 | 1216.00 |
| 2 | 2 | canopy_unbatched | outer second-oracle WHIR recursive oracle 2 | 2 | [2^29,2^19] | 256 | 26 | 26.00 | 1010.78 | 832.00 |
| 2 | 2 | canopy_unbatched | outer second-oracle WHIR recursive oracle 3 | 2 | [2^28,2^15] | 256 | 20 | 20.00 | 753.84 | 640.00 |
| 2 | 2 | canopy_unbatched | outer second-oracle WHIR recursive oracle 4 | 2 | [2^27,2^11] | 256 | 17 | 17.00 | 616.14 | 544.00 |
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
