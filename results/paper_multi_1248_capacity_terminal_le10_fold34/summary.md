# Multi-input Experiments 2 and 3 (capacity_conjecture)

This report uses `t in {1,2,4,8}` input oracles and `n in {2^18,2^22,2^26,2^30}`, no proof of work, 256-bit inputs, folding parameters in `{3,4}`, terminal messages of at most `2^10` elements, and the existing capacity eta=rho/20 slack. The independent and original batched baselines start at rate 1/2; the additional batched baseline starts at rate 1/4. The capacity-bound rows are conjectural. Proof sizes are means over 100 deterministic query samples from seed `0x50415045525f5631`.

The independent baseline contains `t` complete rate-1/2 Merkle-WHIR proofs. The two batched baselines, at initial rates 1/2 and 1/4, keep `t` independent first transitions and batch the `t` second-oracle claims into one tail (a no-op when `t=1`). Experiment 2 makes `2t` recursive claims and proves all independently. Experiment 3 batches its `2t` aligned 256-bit claims into one generalized Schwartz--Zippel identity, sending `2(2t)-3` cross-term coefficients before the challenge.

## Selected proof sizes

Both relative columns use the corresponding Merkle-WHIR baseline at the same `t,n`.

| t | log2 n | scheme | rate | batched sources | folds | h | terminal | min event bits | union bits | 128-bit RBR? | KiB | vs independent | vs batched |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|:---:|---:|---:|---:|
| 1 | 18 | merkle_whir_independent | 1/2 | 0 | 3,3,4 | - | 8 | 128.17 | 126.83 | yes | 120.82 | 1.000x | 1.000x |
| 1 | 18 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,4 | - | 8 | 128.17 | 126.83 | yes | 120.82 | 1.000x | 1.000x |
| 1 | 18 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,3,4 | - | 8 | 129.28 | 128.14 | yes | 83.36 | 0.690x | 0.690x |
| 1 | 18 | canopy_unbatched | 1/2 | 0 | 3,3,4 | 18 | 8 | 128.29 | 127.60 | yes | 118.73 | 0.983x | 0.983x |
| 1 | 18 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4 | 4 | 8 | 128.29 | 127.60 | yes | 103.05 | 0.853x | 0.853x |
| 2 | 18 | merkle_whir_independent | 1/2 | 0 | 3,3,4 | - | 8 | 128.17 | 125.83 | yes | 241.68 | 1.000x | 1.133x |
| 2 | 18 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,4 | - | 8 | 128.17 | 126.38 | yes | 213.33 | 0.883x | 1.000x |
| 2 | 18 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4 | - | 8 | 129.28 | 127.60 | yes | 140.18 | 0.580x | 0.657x |
| 2 | 18 | canopy_unbatched | 1/2 | 0 | 3,3,4 | 18 | 8 | 128.29 | 126.60 | yes | 237.36 | 0.982x | 1.113x |
| 2 | 18 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,4 | 4 | 8 | 128.29 | 126.91 | yes | 179.54 | 0.743x | 0.842x |
| 4 | 18 | merkle_whir_independent | 1/2 | 0 | 3,3,4 | - | 8 | 128.17 | 124.83 | yes | 483.62 | 1.000x | 1.214x |
| 4 | 18 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,4 | - | 8 | 128.17 | 125.77 | yes | 398.37 | 0.824x | 1.000x |
| 4 | 18 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,4 | - | 8 | 129.28 | 126.90 | yes | 254.10 | 0.525x | 0.638x |
| 4 | 18 | canopy_unbatched | 1/2 | 0 | 3,3,4 | 18 | 8 | 128.29 | 125.60 | yes | 474.46 | 0.981x | 1.191x |
| 4 | 18 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,4 | 4 | 8 | 128.29 | 126.08 | yes | 332.81 | 0.688x | 0.835x |
| 8 | 18 | merkle_whir_independent | 1/2 | 0 | 3,3,4 | - | 8 | 128.17 | 123.83 | yes | 966.39 | 1.000x | 1.259x |
| 8 | 18 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,4 | - | 8 | 128.17 | 125.00 | yes | 767.59 | 0.794x | 1.000x |
| 8 | 18 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,4 | - | 8 | 129.28 | 126.08 | yes | 481.93 | 0.499x | 0.628x |
| 8 | 18 | canopy_unbatched | 1/2 | 0 | 3,3,4 | 18 | 8 | 128.29 | 124.60 | yes | 949.41 | 0.982x | 1.237x |
| 8 | 18 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,4 | 4 | 8 | 128.29 | 125.18 | yes | 639.41 | 0.662x | 0.833x |
| 1 | 22 | merkle_whir_independent | 1/2 | 0 | 3,3,4,4 | - | 8 | 128.17 | 126.83 | yes | 161.62 | 1.000x | 1.000x |
| 1 | 22 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,4,4 | - | 8 | 128.17 | 126.83 | yes | 161.62 | 1.000x | 1.000x |
| 1 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,4 | - | 7 | 129.08 | 127.65 | yes | 111.20 | 0.688x | 0.688x |
| 1 | 22 | canopy_unbatched | 1/2 | 0 | 3,4,4,4 | 9 | 7 | 128.29 | 127.14 | yes | 152.76 | 0.945x | 0.945x |
| 1 | 22 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,4 | 4 | 8 | 128.29 | 127.59 | yes | 127.64 | 0.790x | 0.790x |
| 2 | 22 | merkle_whir_independent | 1/2 | 0 | 3,3,4,4 | - | 8 | 128.17 | 125.83 | yes | 323.33 | 1.000x | 1.168x |
| 2 | 22 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,4,4 | - | 8 | 128.17 | 126.38 | yes | 276.82 | 0.856x | 1.000x |
| 2 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,4 | - | 8 | 129.28 | 127.59 | yes | 181.60 | 0.562x | 0.656x |
| 2 | 22 | canopy_unbatched | 1/2 | 0 | 3,4,4,4 | 9 | 7 | 128.29 | 126.14 | yes | 305.39 | 0.945x | 1.103x |
| 2 | 22 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,4,4 | 4 | 8 | 128.29 | 126.89 | yes | 212.65 | 0.658x | 0.768x |
| 4 | 22 | merkle_whir_independent | 1/2 | 0 | 3,3,4,4 | - | 8 | 128.17 | 124.83 | yes | 646.84 | 1.000x | 1.275x |
| 4 | 22 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,4,4 | - | 8 | 128.17 | 125.76 | yes | 507.16 | 0.784x | 1.000x |
| 4 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,4,4 | - | 8 | 129.28 | 126.89 | yes | 320.23 | 0.495x | 0.631x |
| 4 | 22 | canopy_unbatched | 1/2 | 0 | 3,4,4,4 | 9 | 7 | 128.29 | 125.14 | yes | 610.88 | 0.944x | 1.205x |
| 4 | 22 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,4,4 | 4 | 8 | 128.29 | 126.08 | yes | 382.51 | 0.591x | 0.754x |
| 8 | 22 | merkle_whir_independent | 1/2 | 0 | 3,3,4,4 | - | 8 | 128.17 | 123.83 | yes | 1293.72 | 1.000x | 1.337x |
| 8 | 22 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,4,4 | - | 8 | 128.17 | 125.00 | yes | 967.44 | 0.748x | 1.000x |
| 8 | 22 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,4,4 | - | 8 | 129.28 | 126.08 | yes | 598.61 | 0.463x | 0.619x |
| 8 | 22 | canopy_unbatched | 1/2 | 0 | 3,4,4,4 | 9 | 7 | 128.29 | 124.14 | yes | 1221.64 | 0.944x | 1.263x |
| 8 | 22 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,4,4 | 4 | 8 | 128.29 | 125.18 | yes | 721.82 | 0.558x | 0.746x |
| 1 | 26 | merkle_whir_independent | 1/2 | 0 | 3,3,3,3,3,4 | - | 7 | 128.17 | 126.71 | yes | 200.28 | 1.000x | 1.000x |
| 1 | 26 | merkle_whir_batched_after_second | 1/2 | 1 | 3,3,3,3,3,4 | - | 7 | 128.17 | 126.71 | yes | 200.28 | 1.000x | 1.000x |
| 1 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,4,4 | - | 7 | 129.08 | 127.25 | yes | 138.09 | 0.689x | 0.689x |
| 1 | 26 | canopy_unbatched | 1/2 | 0 | 3,4,4,4,4 | 12 | 7 | 128.29 | 126.86 | yes | 183.86 | 0.918x | 0.918x |
| 1 | 26 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,3,3,3,4 | 4 | 7 | 128.29 | 127.07 | yes | 150.96 | 0.754x | 0.754x |
| 2 | 26 | merkle_whir_independent | 1/2 | 0 | 3,3,3,3,3,4 | - | 7 | 128.17 | 125.71 | yes | 400.56 | 1.000x | 1.184x |
| 2 | 26 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,3,3,3,4 | - | 7 | 128.17 | 126.29 | yes | 338.21 | 0.844x | 1.000x |
| 2 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,3,3,3,4 | - | 7 | 129.08 | 127.07 | yes | 221.64 | 0.553x | 0.655x |
| 2 | 26 | canopy_unbatched | 1/2 | 0 | 3,4,4,4,4 | 12 | 7 | 128.29 | 125.86 | yes | 367.63 | 0.918x | 1.087x |
| 2 | 26 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,3,3,3,4 | 4 | 7 | 128.29 | 126.55 | yes | 244.20 | 0.610x | 0.722x |
| 4 | 26 | merkle_whir_independent | 1/2 | 0 | 3,3,3,3,3,4 | - | 7 | 128.17 | 124.71 | yes | 801.24 | 1.000x | 1.305x |
| 4 | 26 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,3,3,3,4 | - | 7 | 128.17 | 125.70 | yes | 613.85 | 0.766x | 1.000x |
| 4 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,3,3,3,4 | - | 7 | 129.08 | 126.55 | yes | 385.49 | 0.481x | 0.628x |
| 4 | 26 | canopy_unbatched | 1/2 | 0 | 3,4,4,4,4 | 12 | 7 | 128.29 | 124.86 | yes | 735.39 | 0.918x | 1.198x |
| 4 | 26 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,3,3,3,4 | 4 | 7 | 128.29 | 125.87 | yes | 430.64 | 0.537x | 0.702x |
| 8 | 26 | merkle_whir_independent | 1/2 | 0 | 3,3,3,3,3,4 | - | 7 | 128.17 | 123.71 | yes | 1602.11 | 1.000x | 1.375x |
| 8 | 26 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,3,3,3,4 | - | 7 | 128.17 | 124.97 | yes | 1164.88 | 0.727x | 1.000x |
| 8 | 26 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,3,3,3,4 | - | 7 | 129.08 | 125.87 | yes | 713.33 | 0.445x | 0.612x |
| 8 | 26 | canopy_unbatched | 1/2 | 0 | 3,4,4,4,4 | 12 | 7 | 128.29 | 123.86 | yes | 1470.86 | 0.918x | 1.263x |
| 8 | 26 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,3,3,3,4 | 4 | 7 | 128.29 | 125.07 | yes | 803.23 | 0.501x | 0.690x |
| 1 | 30 | merkle_whir_independent | 1/2 | 0 | 3,4,4,4,4,4 | - | 7 | 128.29 | 127.26 | yes | 239.47 | 1.000x | 1.000x |
| 1 | 30 | merkle_whir_batched_after_second | 1/2 | 1 | 3,4,4,4,4,4 | - | 7 | 128.29 | 127.26 | yes | 239.47 | 1.000x | 1.000x |
| 1 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 1 | 3,4,4,4,4,4 | - | 7 | 129.08 | 127.25 | yes | 166.15 | 0.694x | 0.694x |
| 1 | 30 | canopy_unbatched | 1/2 | 0 | 3,4,4,4,4,4 | 12 | 7 | 128.29 | 126.86 | yes | 215.72 | 0.901x | 0.901x |
| 1 | 30 | canopy_schwartz_zippel | 1/2 | 2 | 3,3,4,3,3,3,4 | 4 | 7 | 128.29 | 127.10 | yes | 176.21 | 0.736x | 0.736x |
| 2 | 30 | merkle_whir_independent | 1/2 | 0 | 3,4,4,4,4,4 | - | 7 | 128.29 | 126.26 | yes | 479.04 | 1.000x | 1.190x |
| 2 | 30 | merkle_whir_batched_after_second | 1/2 | 2 | 3,3,4,3,3,3,4 | - | 7 | 128.17 | 126.13 | yes | 402.42 | 0.840x | 1.000x |
| 2 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 2 | 3,3,4,3,3,3,4 | - | 7 | 129.28 | 127.10 | yes | 263.47 | 0.550x | 0.655x |
| 2 | 30 | canopy_unbatched | 1/2 | 0 | 3,4,4,4,4,4 | 12 | 7 | 128.29 | 125.86 | yes | 431.55 | 0.901x | 1.072x |
| 2 | 30 | canopy_schwartz_zippel | 1/2 | 4 | 3,3,4,3,3,3,4 | 4 | 7 | 128.29 | 126.58 | yes | 277.47 | 0.579x | 0.689x |
| 4 | 30 | merkle_whir_independent | 1/2 | 0 | 3,4,4,4,4,4 | - | 7 | 128.29 | 125.26 | yes | 958.26 | 1.000x | 1.324x |
| 4 | 30 | merkle_whir_batched_after_second | 1/2 | 4 | 3,3,4,3,3,3,4 | - | 7 | 128.17 | 125.60 | yes | 723.92 | 0.755x | 1.000x |
| 4 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 4 | 3,3,4,3,3,3,4 | - | 7 | 129.28 | 126.57 | yes | 452.30 | 0.472x | 0.625x |
| 4 | 30 | canopy_unbatched | 1/2 | 0 | 3,4,4,4,4,4 | 12 | 7 | 128.29 | 124.86 | yes | 863.12 | 0.901x | 1.192x |
| 4 | 30 | canopy_schwartz_zippel | 1/2 | 8 | 3,3,4,3,3,3,4 | 4 | 7 | 128.29 | 125.89 | yes | 480.25 | 0.501x | 0.663x |
| 8 | 30 | merkle_whir_independent | 1/2 | 0 | 3,4,4,4,4,4 | - | 7 | 128.29 | 124.26 | yes | 1915.38 | 1.000x | 1.402x |
| 8 | 30 | merkle_whir_batched_after_second | 1/2 | 8 | 3,3,4,3,3,3,4 | - | 7 | 128.17 | 124.90 | yes | 1366.38 | 0.713x | 1.000x |
| 8 | 30 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 8 | 3,3,4,3,3,3,4 | - | 7 | 129.28 | 125.89 | yes | 829.73 | 0.433x | 0.607x |
| 8 | 30 | canopy_unbatched | 1/2 | 0 | 3,4,4,4,4,4 | 12 | 7 | 128.29 | 123.86 | yes | 1726.43 | 0.901x | 1.264x |
| 8 | 30 | canopy_schwartz_zippel | 1/2 | 16 | 3,3,4,3,3,3,4 | 4 | 7 | 128.29 | 125.07 | yes | 886.00 | 0.463x | 0.648x |

## Component breakdown

All entries are KiB.

| t | log2 n | scheme | roots | auth | OOD | sumcheck | batching | answers | coefficients | total |
|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 18 | merkle_whir_independent | 0.09 | 52.52 | 0.09 | 1.69 | 0.00 | 58.43 | 8.00 | 120.82 |
| 1 | 18 | merkle_whir_batched_after_second | 0.09 | 52.52 | 0.09 | 1.69 | 0.00 | 58.43 | 8.00 | 120.82 |
| 1 | 18 | merkle_whir_batched_rate_quarter_after_second | 0.09 | 37.50 | 0.09 | 1.69 | 0.00 | 35.99 | 8.00 | 83.36 |
| 1 | 18 | canopy_unbatched | 0.09 | 54.73 | 0.09 | 1.78 | 0.00 | 53.97 | 8.06 | 118.73 |
| 1 | 18 | canopy_schwartz_zippel | 0.09 | 31.14 | 0.12 | 1.69 | 0.03 | 61.97 | 8.00 | 103.05 |
| 2 | 18 | merkle_whir_independent | 0.19 | 105.06 | 0.19 | 3.38 | 0.00 | 116.88 | 16.00 | 241.68 |
| 2 | 18 | merkle_whir_batched_after_second | 0.16 | 99.12 | 0.16 | 1.97 | 0.03 | 103.90 | 8.00 | 213.33 |
| 2 | 18 | merkle_whir_batched_rate_quarter_after_second | 0.16 | 68.87 | 0.16 | 1.97 | 0.03 | 60.99 | 8.00 | 140.18 |
| 2 | 18 | canopy_unbatched | 0.19 | 109.37 | 0.19 | 3.56 | 0.00 | 107.92 | 16.12 | 237.36 |
| 2 | 18 | canopy_schwartz_zippel | 0.16 | 56.11 | 0.22 | 1.97 | 0.16 | 112.93 | 8.00 | 179.54 |
| 4 | 18 | merkle_whir_independent | 0.38 | 210.44 | 0.38 | 6.75 | 0.00 | 233.69 | 32.00 | 483.62 |
| 4 | 18 | merkle_whir_batched_after_second | 0.28 | 192.30 | 0.28 | 2.53 | 0.16 | 194.82 | 8.00 | 398.37 |
| 4 | 18 | merkle_whir_batched_rate_quarter_after_second | 0.28 | 131.88 | 0.28 | 2.53 | 0.16 | 110.97 | 8.00 | 254.10 |
| 4 | 18 | canopy_unbatched | 0.38 | 218.59 | 0.38 | 7.12 | 0.00 | 215.75 | 32.25 | 474.46 |
| 4 | 18 | canopy_schwartz_zippel | 0.28 | 106.39 | 0.41 | 2.53 | 0.41 | 214.79 | 8.00 | 332.81 |
| 8 | 18 | merkle_whir_independent | 0.75 | 419.87 | 0.75 | 13.50 | 0.00 | 467.52 | 64.00 | 966.39 |
| 8 | 18 | merkle_whir_batched_after_second | 0.53 | 377.66 | 0.53 | 3.66 | 0.41 | 376.81 | 8.00 | 767.59 |
| 8 | 18 | merkle_whir_batched_rate_quarter_after_second | 0.53 | 257.88 | 0.53 | 3.66 | 0.41 | 210.93 | 8.00 | 481.93 |
| 8 | 18 | canopy_unbatched | 0.75 | 437.52 | 0.75 | 14.25 | 0.00 | 431.64 | 64.50 | 949.41 |
| 8 | 18 | canopy_schwartz_zippel | 0.53 | 206.97 | 0.78 | 3.66 | 0.91 | 418.56 | 8.00 | 639.41 |
| 1 | 22 | merkle_whir_independent | 0.12 | 84.32 | 0.12 | 2.06 | 0.00 | 66.99 | 8.00 | 161.62 |
| 1 | 22 | merkle_whir_batched_after_second | 0.12 | 84.32 | 0.12 | 2.06 | 0.00 | 66.99 | 8.00 | 161.62 |
| 1 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.12 | 55.64 | 0.12 | 2.06 | 0.00 | 49.24 | 4.00 | 111.20 |
| 1 | 22 | canopy_unbatched | 0.16 | 64.30 | 0.19 | 3.38 | 0.00 | 76.75 | 8.00 | 152.76 |
| 1 | 22 | canopy_schwartz_zippel | 0.12 | 47.77 | 0.16 | 2.06 | 0.03 | 69.50 | 8.00 | 127.64 |
| 2 | 22 | merkle_whir_independent | 0.25 | 168.72 | 0.25 | 4.12 | 0.00 | 133.99 | 16.00 | 323.33 |
| 2 | 22 | merkle_whir_batched_after_second | 0.19 | 153.58 | 0.19 | 2.34 | 0.03 | 112.48 | 8.00 | 276.82 |
| 2 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 102.35 | 0.19 | 2.34 | 0.03 | 68.50 | 8.00 | 181.60 |
| 2 | 22 | canopy_unbatched | 0.31 | 128.46 | 0.38 | 6.75 | 0.00 | 153.49 | 16.00 | 305.39 |
| 2 | 22 | canopy_schwartz_zippel | 0.19 | 81.22 | 0.25 | 2.34 | 0.16 | 120.50 | 8.00 | 212.65 |
| 4 | 22 | merkle_whir_independent | 0.50 | 337.63 | 0.50 | 8.25 | 0.00 | 267.96 | 32.00 | 646.84 |
| 4 | 22 | merkle_whir_batched_after_second | 0.31 | 291.98 | 0.31 | 2.91 | 0.16 | 203.49 | 8.00 | 507.16 |
| 4 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.31 | 190.06 | 0.31 | 2.91 | 0.16 | 118.49 | 8.00 | 320.23 |
| 4 | 22 | canopy_unbatched | 0.62 | 257.04 | 0.75 | 13.50 | 0.00 | 306.97 | 32.00 | 610.88 |
| 4 | 22 | canopy_schwartz_zippel | 0.31 | 147.97 | 0.44 | 2.91 | 0.41 | 222.49 | 8.00 | 382.51 |
| 8 | 22 | merkle_whir_independent | 1.00 | 675.24 | 1.00 | 16.50 | 0.00 | 535.98 | 64.00 | 1293.72 |
| 8 | 22 | merkle_whir_batched_after_second | 0.56 | 568.44 | 0.56 | 4.03 | 0.41 | 385.44 | 8.00 | 967.44 |
| 8 | 22 | merkle_whir_batched_rate_quarter_after_second | 0.56 | 366.55 | 0.56 | 4.03 | 0.41 | 218.50 | 8.00 | 598.61 |
| 8 | 22 | canopy_unbatched | 1.25 | 513.93 | 1.50 | 27.00 | 0.00 | 613.96 | 64.00 | 1221.64 |
| 8 | 22 | canopy_schwartz_zippel | 0.56 | 281.04 | 0.81 | 4.03 | 0.91 | 426.46 | 8.00 | 721.82 |
| 1 | 26 | merkle_whir_independent | 0.19 | 126.97 | 0.19 | 2.44 | 0.00 | 66.50 | 4.00 | 200.28 |
| 1 | 26 | merkle_whir_batched_after_second | 0.19 | 126.97 | 0.19 | 2.44 | 0.00 | 66.50 | 4.00 | 200.28 |
| 1 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.16 | 77.09 | 0.16 | 2.44 | 0.00 | 54.25 | 4.00 | 138.09 |
| 1 | 26 | canopy_unbatched | 0.19 | 89.61 | 0.22 | 3.84 | 0.00 | 82.00 | 8.00 | 183.86 |
| 1 | 26 | canopy_schwartz_zippel | 0.19 | 74.59 | 0.22 | 2.44 | 0.03 | 69.50 | 4.00 | 150.96 |
| 2 | 26 | merkle_whir_independent | 0.38 | 253.94 | 0.38 | 4.88 | 0.00 | 133.00 | 8.00 | 400.56 |
| 2 | 26 | merkle_whir_batched_after_second | 0.25 | 218.96 | 0.25 | 2.72 | 0.03 | 112.00 | 4.00 | 338.21 |
| 2 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.25 | 145.89 | 0.25 | 2.72 | 0.03 | 68.50 | 4.00 | 221.64 |
| 2 | 26 | canopy_unbatched | 0.38 | 179.13 | 0.44 | 7.69 | 0.00 | 164.00 | 16.00 | 367.63 |
| 2 | 26 | canopy_schwartz_zippel | 0.25 | 116.26 | 0.31 | 2.72 | 0.16 | 120.50 | 4.00 | 244.20 |
| 4 | 26 | merkle_whir_independent | 0.75 | 507.99 | 0.75 | 9.75 | 0.00 | 266.00 | 16.00 | 801.24 |
| 4 | 26 | merkle_whir_batched_after_second | 0.38 | 402.67 | 0.38 | 3.28 | 0.16 | 203.00 | 4.00 | 613.85 |
| 4 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.38 | 258.80 | 0.38 | 3.28 | 0.16 | 118.50 | 4.00 | 385.49 |
| 4 | 26 | canopy_unbatched | 0.75 | 358.39 | 0.88 | 15.38 | 0.00 | 328.00 | 32.00 | 735.39 |
| 4 | 26 | canopy_schwartz_zippel | 0.38 | 199.58 | 0.50 | 3.28 | 0.41 | 222.50 | 4.00 | 430.64 |
| 8 | 26 | merkle_whir_independent | 1.50 | 1015.61 | 1.50 | 19.50 | 0.00 | 532.00 | 32.00 | 1602.11 |
| 8 | 26 | merkle_whir_batched_after_second | 0.62 | 769.82 | 0.62 | 4.41 | 0.41 | 385.00 | 4.00 | 1164.88 |
| 8 | 26 | merkle_whir_batched_rate_quarter_after_second | 0.62 | 484.77 | 0.62 | 4.41 | 0.41 | 218.50 | 4.00 | 713.33 |
| 8 | 26 | canopy_unbatched | 1.50 | 716.86 | 1.75 | 30.75 | 0.00 | 656.00 | 64.00 | 1470.86 |
| 8 | 26 | canopy_schwartz_zippel | 0.62 | 365.91 | 0.88 | 4.41 | 0.91 | 426.50 | 4.00 | 803.23 |
| 1 | 30 | merkle_whir_independent | 0.19 | 147.28 | 0.19 | 2.81 | 0.00 | 85.00 | 4.00 | 239.47 |
| 1 | 30 | merkle_whir_batched_after_second | 0.19 | 147.28 | 0.19 | 2.81 | 0.00 | 85.00 | 4.00 | 239.47 |
| 1 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.19 | 100.22 | 0.19 | 2.81 | 0.00 | 58.75 | 4.00 | 166.15 |
| 1 | 30 | canopy_unbatched | 0.25 | 112.09 | 0.28 | 4.59 | 0.00 | 90.50 | 8.00 | 215.72 |
| 1 | 30 | canopy_schwartz_zippel | 0.22 | 93.15 | 0.25 | 2.81 | 0.03 | 75.75 | 4.00 | 176.21 |
| 2 | 30 | merkle_whir_independent | 0.38 | 294.67 | 0.38 | 5.62 | 0.00 | 170.00 | 8.00 | 479.04 |
| 2 | 30 | merkle_whir_batched_after_second | 0.28 | 275.49 | 0.28 | 3.09 | 0.03 | 119.25 | 4.00 | 402.42 |
| 2 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.28 | 181.03 | 0.28 | 3.09 | 0.03 | 74.75 | 4.00 | 263.47 |
| 2 | 30 | canopy_unbatched | 0.50 | 224.30 | 0.56 | 9.19 | 0.00 | 181.00 | 16.00 | 431.55 |
| 2 | 30 | canopy_schwartz_zippel | 0.28 | 142.84 | 0.34 | 3.09 | 0.16 | 126.75 | 4.00 | 277.47 |
| 4 | 30 | merkle_whir_independent | 0.75 | 589.51 | 0.75 | 11.25 | 0.00 | 340.00 | 16.00 | 958.26 |
| 4 | 30 | merkle_whir_batched_after_second | 0.41 | 505.05 | 0.41 | 3.66 | 0.16 | 210.25 | 4.00 | 723.92 |
| 4 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.41 | 318.93 | 0.41 | 3.66 | 0.16 | 124.75 | 4.00 | 452.30 |
| 4 | 30 | canopy_unbatched | 1.00 | 448.62 | 1.12 | 18.38 | 0.00 | 362.00 | 32.00 | 863.12 |
| 4 | 30 | canopy_schwartz_zippel | 0.41 | 242.50 | 0.53 | 3.66 | 0.41 | 228.75 | 4.00 | 480.25 |
| 8 | 30 | merkle_whir_independent | 1.50 | 1177.88 | 1.50 | 22.50 | 0.00 | 680.00 | 32.00 | 1915.38 |
| 8 | 30 | merkle_whir_batched_after_second | 0.66 | 963.63 | 0.66 | 4.78 | 0.41 | 392.25 | 4.00 | 1366.38 |
| 8 | 30 | merkle_whir_batched_rate_quarter_after_second | 0.66 | 594.48 | 0.66 | 4.78 | 0.41 | 224.75 | 4.00 | 829.73 |
| 8 | 30 | canopy_unbatched | 2.00 | 897.43 | 2.25 | 36.75 | 0.00 | 724.00 | 64.00 | 1726.43 |
| 8 | 30 | canopy_schwartz_zippel | 0.66 | 442.00 | 0.91 | 4.78 | 0.91 | 432.75 | 4.00 | 886.00 |

## Oracle queries at n=30

`unique` is the mean number of distinct positions per queried oracle group; authentication and answer columns are total transmitted values across the reported oracle count.

| exp | t | scheme | role | oracles | code | field | requested | unique | auth | answer fields |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | SZ-batched second-oracle claim initial oracle | 1 | [2^30,2^27] | 256 | 44 | 44.00 | 865.22 | 704.00 |
| 0 | 1 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^29,2^23] | 256 | 22 | 22.00 | 433.22 | 352.00 |
| 0 | 1 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^28,2^19] | 256 | 15 | 15.00 | 287.58 | 240.00 |
| 0 | 1 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^27,2^15] | 256 | 11 | 11.00 | 207.13 | 176.00 |
| 0 | 1 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^26,2^11] | 256 | 9 | 9.00 | 162.29 | 144.00 |
| 0 | 1 | merkle_whir_batched_after_second | independent first Merkle oracles | 1 | [2^31,2^30] | 256 | 138 | 138.00 | 2759.27 | 1104.00 |
| 0 | 2 | merkle_whir_batched_after_second | SZ-batched second-oracle claim initial oracle | 2 | [2^30,2^27] | 256 | 44 | 44.00 | 1819.14 | 704.00 |
| 0 | 2 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^29,2^24] | 256 | 26 | 26.00 | 506.32 | 416.00 |
| 0 | 2 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^28,2^20] | 256 | 17 | 17.00 | 341.67 | 136.00 |
| 0 | 2 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^27,2^17] | 256 | 13 | 13.00 | 253.15 | 104.00 |
| 0 | 2 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^26,2^14] | 256 | 11 | 11.00 | 207.20 | 88.00 |
| 0 | 2 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 5 | 1 | [2^25,2^11] | 256 | 10 | 10.00 | 168.33 | 160.00 |
| 0 | 2 | merkle_whir_batched_after_second | independent first Merkle oracles | 2 | [2^31,2^30] | 256 | 138 | 138.00 | 5519.74 | 2208.00 |
| 0 | 4 | merkle_whir_batched_after_second | SZ-batched second-oracle claim initial oracle | 4 | [2^30,2^27] | 256 | 44 | 44.00 | 3635.80 | 1408.00 |
| 0 | 4 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^29,2^24] | 256 | 26 | 26.00 | 505.93 | 416.00 |
| 0 | 4 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^28,2^20] | 256 | 17 | 17.00 | 341.51 | 136.00 |
| 0 | 4 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^27,2^17] | 256 | 13 | 13.00 | 252.48 | 104.00 |
| 0 | 4 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^26,2^14] | 256 | 11 | 11.00 | 206.35 | 88.00 |
| 0 | 4 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 5 | 1 | [2^25,2^11] | 256 | 10 | 10.00 | 168.61 | 160.00 |
| 0 | 4 | merkle_whir_batched_after_second | independent first Merkle oracles | 4 | [2^31,2^30] | 256 | 138 | 138.00 | 11050.84 | 4416.00 |
| 0 | 8 | merkle_whir_batched_after_second | SZ-batched second-oracle claim initial oracle | 8 | [2^30,2^27] | 256 | 44 | 44.00 | 7271.04 | 2816.00 |
| 0 | 8 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^29,2^24] | 256 | 26 | 26.00 | 505.29 | 416.00 |
| 0 | 8 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^28,2^20] | 256 | 17 | 17.00 | 341.57 | 136.00 |
| 0 | 8 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^27,2^17] | 256 | 13 | 13.00 | 252.00 | 104.00 |
| 0 | 8 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^26,2^14] | 256 | 11 | 11.00 | 206.22 | 88.00 |
| 0 | 8 | merkle_whir_batched_after_second | SZ-batched second-oracle claim recursive oracle 5 | 1 | [2^25,2^11] | 256 | 10 | 10.00 | 168.65 | 160.00 |
| 0 | 8 | merkle_whir_batched_after_second | independent first Merkle oracles | 8 | [2^31,2^30] | 256 | 138 | 138.00 | 22091.36 | 8832.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim initial oracle | 1 | [2^31,2^27] | 256 | 33 | 33.00 | 696.13 | 528.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^30,2^23] | 256 | 19 | 19.00 | 397.11 | 304.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^29,2^19] | 256 | 13 | 13.00 | 266.00 | 208.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^28,2^15] | 256 | 10 | 10.00 | 198.85 | 160.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^27,2^11] | 256 | 9 | 9.00 | 171.30 | 144.00 |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | independent first Merkle oracles | 1 | [2^32,2^30] | 256 | 67 | 67.00 | 1479.27 | 536.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim initial oracle | 2 | [2^31,2^27] | 256 | 33 | 33.00 | 1458.34 | 528.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^30,2^24] | 256 | 22 | 22.00 | 454.27 | 352.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^29,2^20] | 256 | 15 | 15.00 | 318.41 | 120.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^28,2^17] | 256 | 12 | 12.00 | 246.98 | 96.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^27,2^14] | 256 | 10 | 10.00 | 198.52 | 80.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 5 | 1 | [2^26,2^11] | 256 | 9 | 9.00 | 161.74 | 144.00 |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | independent first Merkle oracles | 2 | [2^32,2^30] | 256 | 67 | 67.00 | 2954.70 | 1072.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim initial oracle | 4 | [2^31,2^27] | 256 | 33 | 33.00 | 2918.64 | 1056.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^30,2^24] | 256 | 22 | 22.00 | 454.51 | 352.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^29,2^20] | 256 | 15 | 15.00 | 319.41 | 120.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^28,2^17] | 256 | 12 | 12.00 | 247.15 | 96.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^27,2^14] | 256 | 10 | 10.00 | 198.71 | 80.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 5 | 1 | [2^26,2^11] | 256 | 9 | 9.00 | 162.38 | 144.00 |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | independent first Merkle oracles | 4 | [2^32,2^30] | 256 | 67 | 67.00 | 5904.80 | 2144.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim initial oracle | 8 | [2^31,2^27] | 256 | 33 | 33.00 | 5834.16 | 2112.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 1 | 1 | [2^30,2^24] | 256 | 22 | 22.00 | 454.56 | 352.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 2 | 1 | [2^29,2^20] | 256 | 15 | 15.00 | 318.97 | 120.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 3 | 1 | [2^28,2^17] | 256 | 12 | 12.00 | 246.50 | 96.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 4 | 1 | [2^27,2^14] | 256 | 10 | 10.00 | 199.12 | 80.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | SZ-batched second-oracle claim recursive oracle 5 | 1 | [2^26,2^11] | 256 | 9 | 9.00 | 162.24 | 144.00 |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | independent first Merkle oracles | 8 | [2^32,2^30] | 256 | 67 | 67.00 | 11807.84 | 4288.00 |
| 0 | 1 | merkle_whir_independent | independent WHIR initial oracle | 1 | [2^31,2^30] | 256 | 138 | 138.00 | 2760.28 | 1104.00 |
| 0 | 1 | merkle_whir_independent | independent WHIR recursive oracle 1 | 1 | [2^30,2^27] | 256 | 44 | 44.00 | 864.22 | 704.00 |
| 0 | 1 | merkle_whir_independent | independent WHIR recursive oracle 2 | 1 | [2^29,2^23] | 256 | 22 | 22.00 | 432.10 | 352.00 |
| 0 | 1 | merkle_whir_independent | independent WHIR recursive oracle 3 | 1 | [2^28,2^19] | 256 | 15 | 15.00 | 288.58 | 240.00 |
| 0 | 1 | merkle_whir_independent | independent WHIR recursive oracle 4 | 1 | [2^27,2^15] | 256 | 11 | 11.00 | 206.39 | 176.00 |
| 0 | 1 | merkle_whir_independent | independent WHIR recursive oracle 5 | 1 | [2^26,2^11] | 256 | 9 | 9.00 | 161.37 | 144.00 |
| 0 | 2 | merkle_whir_independent | independent WHIR initial oracle | 2 | [2^31,2^30] | 256 | 138 | 138.00 | 5519.58 | 2208.00 |
| 0 | 2 | merkle_whir_independent | independent WHIR recursive oracle 1 | 2 | [2^30,2^27] | 256 | 44 | 44.00 | 1729.98 | 1408.00 |
| 0 | 2 | merkle_whir_independent | independent WHIR recursive oracle 2 | 2 | [2^29,2^23] | 256 | 22 | 22.00 | 865.42 | 704.00 |
| 0 | 2 | merkle_whir_independent | independent WHIR recursive oracle 3 | 2 | [2^28,2^19] | 256 | 15 | 15.00 | 577.74 | 480.00 |
| 0 | 2 | merkle_whir_independent | independent WHIR recursive oracle 4 | 2 | [2^27,2^15] | 256 | 11 | 11.00 | 411.10 | 352.00 |
| 0 | 2 | merkle_whir_independent | independent WHIR recursive oracle 5 | 2 | [2^26,2^11] | 256 | 9 | 9.00 | 325.54 | 288.00 |
| 0 | 4 | merkle_whir_independent | independent WHIR initial oracle | 4 | [2^31,2^30] | 256 | 138 | 138.00 | 11048.24 | 4416.00 |
| 0 | 4 | merkle_whir_independent | independent WHIR recursive oracle 1 | 4 | [2^30,2^27] | 256 | 44 | 44.00 | 3460.84 | 2816.00 |
| 0 | 4 | merkle_whir_independent | independent WHIR recursive oracle 2 | 4 | [2^29,2^23] | 256 | 22 | 22.00 | 1729.24 | 1408.00 |
| 0 | 4 | merkle_whir_independent | independent WHIR recursive oracle 3 | 4 | [2^28,2^19] | 256 | 15 | 15.00 | 1151.88 | 960.00 |
| 0 | 4 | merkle_whir_independent | independent WHIR recursive oracle 4 | 4 | [2^27,2^15] | 256 | 11 | 11.00 | 825.28 | 704.00 |
| 0 | 4 | merkle_whir_independent | independent WHIR recursive oracle 5 | 4 | [2^26,2^11] | 256 | 9 | 9.00 | 648.80 | 576.00 |
| 0 | 8 | merkle_whir_independent | independent WHIR initial oracle | 8 | [2^31,2^30] | 256 | 138 | 138.00 | 22068.08 | 8832.00 |
| 0 | 8 | merkle_whir_independent | independent WHIR recursive oracle 1 | 8 | [2^30,2^27] | 256 | 44 | 44.00 | 6907.76 | 5632.00 |
| 0 | 8 | merkle_whir_independent | independent WHIR recursive oracle 2 | 8 | [2^29,2^23] | 256 | 22 | 22.00 | 3461.68 | 2816.00 |
| 0 | 8 | merkle_whir_independent | independent WHIR recursive oracle 3 | 8 | [2^28,2^19] | 256 | 15 | 15.00 | 2312.56 | 1920.00 |
| 0 | 8 | merkle_whir_independent | independent WHIR recursive oracle 4 | 8 | [2^27,2^15] | 256 | 11 | 11.00 | 1644.08 | 1408.00 |
| 0 | 8 | merkle_whir_independent | independent WHIR recursive oracle 5 | 8 | [2^26,2^11] | 256 | 9 | 9.00 | 1298.08 | 1152.00 |
| 2 | 1 | canopy_unbatched | Canopy claim WHIR initial oracle | 1 | [2^31,2^19] | 256 | 11 | 11.00 | 249.70 | 176.00 |
| 2 | 1 | canopy_unbatched | Canopy claim WHIR recursive oracle 1 | 1 | [2^30,2^15] | 256 | 9 | 9.00 | 198.23 | 144.00 |
| 2 | 1 | canopy_unbatched | Canopy claim WHIR recursive oracle 2 | 1 | [2^29,2^11] | 256 | 8 | 8.00 | 169.23 | 128.00 |
| 2 | 1 | canopy_unbatched | outer first oracle via Canopy | 1 | [2^31,2^30] | 256 | 138 | 138.00 | 1241.98 | 1104.00 |
| 2 | 1 | canopy_unbatched | outer second-oracle WHIR initial oracle | 1 | [2^31,2^27] | 256 | 33 | 33.00 | 694.46 | 528.00 |
| 2 | 1 | canopy_unbatched | outer second-oracle WHIR recursive oracle 1 | 1 | [2^30,2^23] | 256 | 19 | 19.00 | 396.42 | 304.00 |
| 2 | 1 | canopy_unbatched | outer second-oracle WHIR recursive oracle 2 | 1 | [2^29,2^19] | 256 | 13 | 13.00 | 266.09 | 208.00 |
| 2 | 1 | canopy_unbatched | outer second-oracle WHIR recursive oracle 3 | 1 | [2^28,2^15] | 256 | 10 | 10.00 | 199.25 | 160.00 |
| 2 | 1 | canopy_unbatched | outer second-oracle WHIR recursive oracle 4 | 1 | [2^27,2^11] | 256 | 9 | 9.00 | 171.60 | 144.00 |
| 2 | 2 | canopy_unbatched | Canopy claim WHIR initial oracle | 2 | [2^31,2^19] | 256 | 11 | 11.00 | 499.40 | 352.00 |
| 2 | 2 | canopy_unbatched | Canopy claim WHIR recursive oracle 1 | 2 | [2^30,2^15] | 256 | 9 | 9.00 | 396.46 | 288.00 |
| 2 | 2 | canopy_unbatched | Canopy claim WHIR recursive oracle 2 | 2 | [2^29,2^11] | 256 | 8 | 8.00 | 338.46 | 256.00 |
| 2 | 2 | canopy_unbatched | outer first oracle via Canopy | 2 | [2^31,2^30] | 256 | 138 | 138.00 | 2483.90 | 2208.00 |
| 2 | 2 | canopy_unbatched | outer second-oracle WHIR initial oracle | 2 | [2^31,2^27] | 256 | 33 | 33.00 | 1391.76 | 1056.00 |
| 2 | 2 | canopy_unbatched | outer second-oracle WHIR recursive oracle 1 | 2 | [2^30,2^23] | 256 | 19 | 19.00 | 793.52 | 608.00 |
| 2 | 2 | canopy_unbatched | outer second-oracle WHIR recursive oracle 2 | 2 | [2^29,2^19] | 256 | 13 | 13.00 | 532.62 | 416.00 |
| 2 | 2 | canopy_unbatched | outer second-oracle WHIR recursive oracle 3 | 2 | [2^28,2^15] | 256 | 10 | 10.00 | 398.52 | 320.00 |
| 2 | 2 | canopy_unbatched | outer second-oracle WHIR recursive oracle 4 | 2 | [2^27,2^11] | 256 | 9 | 9.00 | 343.08 | 288.00 |
| 2 | 4 | canopy_unbatched | Canopy claim WHIR initial oracle | 4 | [2^31,2^19] | 256 | 11 | 11.00 | 998.80 | 704.00 |
| 2 | 4 | canopy_unbatched | Canopy claim WHIR recursive oracle 1 | 4 | [2^30,2^15] | 256 | 9 | 9.00 | 792.92 | 576.00 |
| 2 | 4 | canopy_unbatched | Canopy claim WHIR recursive oracle 2 | 4 | [2^29,2^11] | 256 | 8 | 8.00 | 676.92 | 512.00 |
| 2 | 4 | canopy_unbatched | outer first oracle via Canopy | 4 | [2^31,2^30] | 256 | 138 | 138.00 | 4967.72 | 4416.00 |
| 2 | 4 | canopy_unbatched | outer second-oracle WHIR initial oracle | 4 | [2^31,2^27] | 256 | 33 | 33.00 | 2787.04 | 2112.00 |
| 2 | 4 | canopy_unbatched | outer second-oracle WHIR recursive oracle 1 | 4 | [2^30,2^23] | 256 | 19 | 19.00 | 1589.64 | 1216.00 |
| 2 | 4 | canopy_unbatched | outer second-oracle WHIR recursive oracle 2 | 4 | [2^29,2^19] | 256 | 13 | 13.00 | 1063.64 | 832.00 |
| 2 | 4 | canopy_unbatched | outer second-oracle WHIR recursive oracle 3 | 4 | [2^28,2^15] | 256 | 10 | 10.00 | 791.12 | 640.00 |
| 2 | 4 | canopy_unbatched | outer second-oracle WHIR recursive oracle 4 | 4 | [2^27,2^11] | 256 | 9 | 9.00 | 688.04 | 576.00 |
| 2 | 8 | canopy_unbatched | Canopy claim WHIR initial oracle | 8 | [2^31,2^19] | 256 | 11 | 11.00 | 1997.60 | 1408.00 |
| 2 | 8 | canopy_unbatched | Canopy claim WHIR recursive oracle 1 | 8 | [2^30,2^15] | 256 | 9 | 9.00 | 1585.84 | 1152.00 |
| 2 | 8 | canopy_unbatched | Canopy claim WHIR recursive oracle 2 | 8 | [2^29,2^11] | 256 | 8 | 8.00 | 1353.84 | 1024.00 |
| 2 | 8 | canopy_unbatched | outer first oracle via Canopy | 8 | [2^31,2^30] | 256 | 138 | 138.00 | 9935.20 | 8832.00 |
| 2 | 8 | canopy_unbatched | outer second-oracle WHIR initial oracle | 8 | [2^31,2^27] | 256 | 33 | 33.00 | 5574.48 | 4224.00 |
| 2 | 8 | canopy_unbatched | outer second-oracle WHIR recursive oracle 1 | 8 | [2^30,2^23] | 256 | 19 | 19.00 | 3174.24 | 2432.00 |
| 2 | 8 | canopy_unbatched | outer second-oracle WHIR recursive oracle 2 | 8 | [2^29,2^19] | 256 | 13 | 13.00 | 2127.52 | 1664.00 |
| 2 | 8 | canopy_unbatched | outer second-oracle WHIR recursive oracle 3 | 8 | [2^28,2^15] | 256 | 10 | 10.00 | 1592.96 | 1280.00 |
| 2 | 8 | canopy_unbatched | outer second-oracle WHIR recursive oracle 4 | 8 | [2^27,2^11] | 256 | 9 | 9.00 | 1376.16 | 1152.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims initial oracle | 2 | [2^31,2^27] | 256 | 33 | 33.00 | 1458.64 | 528.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 1 | 1 | [2^30,2^24] | 256 | 22 | 22.00 | 455.35 | 352.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 2 | 1 | [2^29,2^20] | 256 | 15 | 15.00 | 319.29 | 120.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 3 | 1 | [2^28,2^17] | 256 | 12 | 12.00 | 247.43 | 96.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 4 | 1 | [2^27,2^14] | 256 | 10 | 10.00 | 198.88 | 80.00 |
| 3 | 1 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 5 | 1 | [2^26,2^11] | 256 | 9 | 9.00 | 163.16 | 144.00 |
| 3 | 1 | canopy_schwartz_zippel | outer first oracle via Canopy | 1 | [2^31,2^30] | 256 | 138 | 138.00 | 138.00 | 1104.00 |
| 3 | 2 | canopy_schwartz_zippel | SZ-batched aligned claims initial oracle | 4 | [2^31,2^27] | 256 | 33 | 33.00 | 2914.44 | 1056.00 |
| 3 | 2 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 1 | 1 | [2^30,2^24] | 256 | 22 | 22.00 | 454.48 | 352.00 |
| 3 | 2 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 2 | 1 | [2^29,2^20] | 256 | 15 | 15.00 | 318.82 | 120.00 |
| 3 | 2 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 3 | 1 | [2^28,2^17] | 256 | 12 | 12.00 | 246.76 | 96.00 |
| 3 | 2 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 4 | 1 | [2^27,2^14] | 256 | 10 | 10.00 | 198.47 | 80.00 |
| 3 | 2 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 5 | 1 | [2^26,2^11] | 256 | 9 | 9.00 | 162.07 | 144.00 |
| 3 | 2 | canopy_schwartz_zippel | outer first oracle via Canopy | 2 | [2^31,2^30] | 256 | 138 | 138.00 | 276.00 | 2208.00 |
| 3 | 4 | canopy_schwartz_zippel | SZ-batched aligned claims initial oracle | 8 | [2^31,2^27] | 256 | 33 | 33.00 | 5825.44 | 2112.00 |
| 3 | 4 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 1 | 1 | [2^30,2^24] | 256 | 22 | 22.00 | 454.56 | 352.00 |
| 3 | 4 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 2 | 1 | [2^29,2^20] | 256 | 15 | 15.00 | 319.64 | 120.00 |
| 3 | 4 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 3 | 1 | [2^28,2^17] | 256 | 12 | 12.00 | 247.25 | 96.00 |
| 3 | 4 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 4 | 1 | [2^27,2^14] | 256 | 10 | 10.00 | 198.68 | 80.00 |
| 3 | 4 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 5 | 1 | [2^26,2^11] | 256 | 9 | 9.00 | 162.28 | 144.00 |
| 3 | 4 | canopy_schwartz_zippel | outer first oracle via Canopy | 4 | [2^31,2^30] | 256 | 138 | 138.00 | 552.00 | 4416.00 |
| 3 | 8 | canopy_schwartz_zippel | SZ-batched aligned claims initial oracle | 16 | [2^31,2^27] | 256 | 33 | 33.00 | 11659.52 | 4224.00 |
| 3 | 8 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 1 | 1 | [2^30,2^24] | 256 | 22 | 22.00 | 454.33 | 352.00 |
| 3 | 8 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 2 | 1 | [2^29,2^20] | 256 | 15 | 15.00 | 318.86 | 120.00 |
| 3 | 8 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 3 | 1 | [2^28,2^17] | 256 | 12 | 12.00 | 247.01 | 96.00 |
| 3 | 8 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 4 | 1 | [2^27,2^14] | 256 | 10 | 10.00 | 198.17 | 80.00 |
| 3 | 8 | canopy_schwartz_zippel | SZ-batched aligned claims recursive oracle 5 | 1 | [2^26,2^11] | 256 | 9 | 9.00 | 162.27 | 144.00 |
| 3 | 8 | canopy_schwartz_zippel | outer first oracle via Canopy | 8 | [2^31,2^30] | 256 | 138 | 138.00 | 1104.00 | 8832.00 |
