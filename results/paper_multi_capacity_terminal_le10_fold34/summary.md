# Multi-input Experiments 2--4 (capacity_conjecture)

This report uses `t in {4,8}` input oracles, a 1/2 starting rate, no proof of work, 256-bit inputs, folding parameters in `{3,4}`, terminal messages of at most `2^10` elements, and the existing capacity eta=rho/20 slack. The capacity-bound rows are conjectural. Proof sizes are means over 100 deterministic query samples from seed `0x50415045525f5631`.

The independent baseline contains `t` complete Merkle-WHIR proofs. The batched baseline keeps `t` independent first transitions and batches the `t` second-oracle claims into one tail. Experiment 2 makes `2t` recursive claims and proves all independently. Experiment 3 batches its `2t` aligned 256-bit claims into one generalized Schwartz--Zippel identity, sending `2(2t)-3` cross-term coefficients before the challenge. Experiment 4 expands those `2t` source claims into `8t` GF(2^64) coordinate equations, projects them once over GF(2^192), and runs one GF(2^192) recursive tail.

## Selected proof sizes

Both relative columns use the corresponding Merkle-WHIR baseline at the same `t,n`.

| t | log2 n | scheme | batched sources | folds | h | terminal | min event bits | union bits | 128-bit RBR? | KiB | vs independent | vs batched |
|---:|---:|---|---:|---|---:|---:|---:|---:|:---:|---:|---:|---:|
| 4 | 22 | merkle_whir_independent | 0 | 3,3,4,4 | - | 8 | 128.17 | 124.83 | yes | 646.84 | 1.000x | 1.275x |
| 4 | 22 | merkle_whir_batched_after_second | 4 | 3,3,4,4 | - | 8 | 128.17 | 125.76 | yes | 507.16 | 0.784x | 1.000x |
| 4 | 22 | canopy_unbatched | 0 | 3,4,4,4 | 9 | 7 | 128.29 | 125.14 | yes | 610.88 | 0.944x | 1.205x |
| 4 | 22 | canopy_schwartz_zippel | 8 | 3,3,4,4 | 4 | 8 | 128.29 | 126.08 | yes | 382.51 | 0.591x | 0.754x |
| 4 | 22 | canopy_field_switch_192 | 8 | 3,3,4,4 | 4 | 8 | 128.29 | 126.08 | yes | 375.15 | 0.580x | 0.740x |
| 8 | 22 | merkle_whir_independent | 0 | 3,3,4,4 | - | 8 | 128.17 | 123.83 | yes | 1293.72 | 1.000x | 1.337x |
| 8 | 22 | merkle_whir_batched_after_second | 8 | 3,3,4,4 | - | 8 | 128.17 | 125.00 | yes | 967.44 | 0.748x | 1.000x |
| 8 | 22 | canopy_unbatched | 0 | 3,4,4,4 | 9 | 7 | 128.29 | 124.14 | yes | 1221.64 | 0.944x | 1.263x |
| 8 | 22 | canopy_schwartz_zippel | 16 | 3,3,4,4 | 4 | 8 | 128.29 | 125.18 | yes | 721.82 | 0.558x | 0.746x |
| 8 | 22 | canopy_field_switch_192 | 16 | 3,3,4,4 | 4 | 8 | 128.29 | 125.18 | yes | 714.24 | 0.552x | 0.738x |
| 4 | 24 | merkle_whir_independent | 0 | 3,3,3,4,4 | - | 7 | 128.17 | 124.52 | yes | 718.41 | 1.000x | 1.285x |
| 4 | 24 | merkle_whir_batched_after_second | 4 | 3,3,3,4,4 | - | 7 | 128.17 | 125.61 | yes | 559.24 | 0.778x | 1.000x |
| 4 | 24 | canopy_unbatched | 0 | 3,3,4,3,4 | 10 | 7 | 128.29 | 124.78 | yes | 671.23 | 0.934x | 1.200x |
| 4 | 24 | canopy_schwartz_zippel | 8 | 3,3,4,3,4 | 4 | 7 | 128.29 | 126.04 | yes | 405.29 | 0.564x | 0.725x |
| 4 | 24 | canopy_field_switch_192 | 8 | 3,3,4,3,4 | 4 | 7 | 128.29 | 126.04 | yes | 398.30 | 0.554x | 0.712x |
| 8 | 24 | merkle_whir_independent | 0 | 3,3,3,4,4 | - | 7 | 128.17 | 123.52 | yes | 1435.58 | 1.000x | 1.348x |
| 8 | 24 | merkle_whir_batched_after_second | 8 | 3,3,3,4,4 | - | 7 | 128.17 | 124.91 | yes | 1065.25 | 0.742x | 1.000x |
| 8 | 24 | canopy_unbatched | 0 | 3,3,4,3,4 | 10 | 7 | 128.29 | 123.78 | yes | 1342.03 | 0.935x | 1.260x |
| 8 | 24 | canopy_schwartz_zippel | 16 | 3,3,4,3,4 | 4 | 7 | 128.29 | 125.16 | yes | 761.29 | 0.530x | 0.715x |
| 8 | 24 | canopy_field_switch_192 | 16 | 3,3,4,3,4 | 4 | 7 | 128.29 | 125.16 | yes | 753.80 | 0.525x | 0.708x |
| 4 | 26 | merkle_whir_independent | 0 | 3,3,3,3,3,4 | - | 7 | 128.17 | 124.71 | yes | 801.24 | 1.000x | 1.305x |
| 4 | 26 | merkle_whir_batched_after_second | 4 | 3,3,3,3,3,4 | - | 7 | 128.17 | 125.70 | yes | 613.85 | 0.766x | 1.000x |
| 4 | 26 | canopy_unbatched | 0 | 3,4,4,4,4 | 12 | 7 | 128.29 | 124.86 | yes | 735.39 | 0.918x | 1.198x |
| 4 | 26 | canopy_schwartz_zippel | 8 | 3,3,3,3,3,4 | 4 | 7 | 128.29 | 125.87 | yes | 430.64 | 0.537x | 0.702x |
| 4 | 26 | canopy_field_switch_192 | 8 | 3,3,4,4,4 | 4 | 8 | 128.29 | 126.04 | yes | 422.44 | 0.527x | 0.688x |
| 8 | 26 | merkle_whir_independent | 0 | 3,3,3,3,3,4 | - | 7 | 128.17 | 123.71 | yes | 1602.11 | 1.000x | 1.375x |
| 8 | 26 | merkle_whir_batched_after_second | 8 | 3,3,3,3,3,4 | - | 7 | 128.17 | 124.97 | yes | 1164.88 | 0.727x | 1.000x |
| 8 | 26 | canopy_unbatched | 0 | 3,4,4,4,4 | 12 | 7 | 128.29 | 123.86 | yes | 1470.86 | 0.918x | 1.263x |
| 8 | 26 | canopy_schwartz_zippel | 16 | 3,3,3,3,3,4 | 4 | 7 | 128.29 | 125.07 | yes | 803.23 | 0.501x | 0.690x |
| 8 | 26 | canopy_field_switch_192 | 16 | 3,3,4,4,4 | 4 | 8 | 128.29 | 125.16 | yes | 794.72 | 0.496x | 0.682x |
| 4 | 28 | merkle_whir_independent | 0 | 3,3,3,4,4,4 | - | 7 | 128.17 | 124.32 | yes | 879.92 | 1.000x | 1.317x |
| 4 | 28 | merkle_whir_batched_after_second | 4 | 3,3,3,4,4,4 | - | 7 | 128.17 | 125.50 | yes | 667.88 | 0.759x | 1.000x |
| 4 | 28 | canopy_unbatched | 0 | 3,4,4,3,3,4 | 10 | 7 | 128.29 | 124.58 | yes | 803.73 | 0.913x | 1.203x |
| 4 | 28 | canopy_schwartz_zippel | 8 | 3,3,4,4,3,4 | 4 | 7 | 128.29 | 126.04 | yes | 455.21 | 0.517x | 0.682x |
| 4 | 28 | canopy_field_switch_192 | 8 | 3,3,4,4,3,4 | 4 | 7 | 128.29 | 126.03 | yes | 446.83 | 0.508x | 0.669x |
| 8 | 28 | merkle_whir_independent | 0 | 3,3,4,3,4,4 | - | 7 | 128.17 | 123.35 | yes | 1759.60 | 1.000x | 1.391x |
| 8 | 28 | merkle_whir_batched_after_second | 8 | 3,3,4,3,4,4 | - | 7 | 128.17 | 124.85 | yes | 1264.66 | 0.719x | 1.000x |
| 8 | 28 | canopy_unbatched | 0 | 3,4,4,3,3,4 | 10 | 7 | 128.29 | 123.58 | yes | 1607.25 | 0.913x | 1.271x |
| 8 | 28 | canopy_schwartz_zippel | 16 | 3,3,4,4,3,4 | 4 | 7 | 128.29 | 125.16 | yes | 843.80 | 0.480x | 0.667x |
| 8 | 28 | canopy_field_switch_192 | 16 | 3,3,4,4,3,4 | 4 | 7 | 128.29 | 125.15 | yes | 835.74 | 0.475x | 0.661x |
| 4 | 30 | merkle_whir_independent | 0 | 3,4,4,4,4,4 | - | 7 | 128.29 | 125.26 | yes | 958.26 | 1.000x | 1.324x |
| 4 | 30 | merkle_whir_batched_after_second | 4 | 3,3,4,3,3,3,4 | - | 7 | 128.17 | 125.60 | yes | 723.92 | 0.755x | 1.000x |
| 4 | 30 | canopy_unbatched | 0 | 3,4,4,4,4,4 | 12 | 7 | 128.29 | 124.86 | yes | 863.12 | 0.901x | 1.192x |
| 4 | 30 | canopy_schwartz_zippel | 8 | 3,3,4,3,3,3,4 | 4 | 7 | 128.29 | 125.89 | yes | 480.25 | 0.501x | 0.663x |
| 4 | 30 | canopy_field_switch_192 | 8 | 3,3,4,4,4,4 | 4 | 8 | 128.29 | 125.97 | yes | 470.75 | 0.491x | 0.650x |
| 8 | 30 | merkle_whir_independent | 0 | 3,4,4,4,4,4 | - | 7 | 128.29 | 124.26 | yes | 1915.38 | 1.000x | 1.402x |
| 8 | 30 | merkle_whir_batched_after_second | 8 | 3,3,4,3,3,3,4 | - | 7 | 128.17 | 124.90 | yes | 1366.38 | 0.713x | 1.000x |
| 8 | 30 | canopy_unbatched | 0 | 3,4,4,4,4,4 | 12 | 7 | 128.29 | 123.86 | yes | 1726.43 | 0.901x | 1.264x |
| 8 | 30 | canopy_schwartz_zippel | 16 | 3,3,4,3,3,3,4 | 4 | 7 | 128.29 | 125.07 | yes | 886.00 | 0.463x | 0.648x |
| 8 | 30 | canopy_field_switch_192 | 16 | 3,3,4,4,4,4 | 4 | 8 | 128.29 | 125.12 | yes | 875.57 | 0.457x | 0.641x |

## Component breakdown

All entries are KiB.

| t | log2 n | scheme | roots | auth | OOD | sumcheck | batching | answers | coefficients | total |
|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 4 | 22 | merkle_whir_independent | 0.50 | 337.63 | 0.50 | 8.25 | 0.00 | 267.96 | 32.00 | 646.84 |
| 4 | 22 | merkle_whir_batched_after_second | 0.31 | 291.98 | 0.31 | 2.91 | 0.16 | 203.49 | 8.00 | 507.16 |
| 4 | 22 | canopy_unbatched | 0.62 | 257.04 | 0.75 | 13.50 | 0.00 | 306.97 | 32.00 | 610.88 |
| 4 | 22 | canopy_schwartz_zippel | 0.31 | 147.97 | 0.44 | 2.91 | 0.41 | 222.49 | 8.00 | 382.51 |
| 4 | 22 | canopy_field_switch_192 | 0.31 | 147.90 | 0.27 | 2.81 | 0.00 | 217.86 | 6.00 | 375.15 |
| 8 | 22 | merkle_whir_independent | 1.00 | 675.24 | 1.00 | 16.50 | 0.00 | 535.98 | 64.00 | 1293.72 |
| 8 | 22 | merkle_whir_batched_after_second | 0.56 | 568.44 | 0.56 | 4.03 | 0.41 | 385.44 | 8.00 | 967.44 |
| 8 | 22 | canopy_unbatched | 1.25 | 513.93 | 1.50 | 27.00 | 0.00 | 613.96 | 64.00 | 1221.64 |
| 8 | 22 | canopy_schwartz_zippel | 0.56 | 281.04 | 0.81 | 4.03 | 0.91 | 426.46 | 8.00 | 721.82 |
| 8 | 22 | canopy_field_switch_192 | 0.56 | 281.45 | 0.39 | 4.01 | 0.00 | 421.83 | 6.00 | 714.24 |
| 4 | 24 | merkle_whir_independent | 0.62 | 420.18 | 0.62 | 9.00 | 0.00 | 271.98 | 16.00 | 718.41 |
| 4 | 24 | merkle_whir_batched_after_second | 0.34 | 346.80 | 0.34 | 3.09 | 0.16 | 204.50 | 4.00 | 559.24 |
| 4 | 24 | canopy_unbatched | 0.75 | 322.98 | 0.88 | 14.62 | 0.00 | 300.00 | 32.00 | 671.23 |
| 4 | 24 | canopy_schwartz_zippel | 0.34 | 172.23 | 0.47 | 3.09 | 0.41 | 224.75 | 4.00 | 405.29 |
| 4 | 24 | canopy_field_switch_192 | 0.34 | 172.13 | 0.31 | 2.95 | 0.00 | 219.56 | 3.00 | 398.30 |
| 8 | 24 | merkle_whir_independent | 1.25 | 839.10 | 1.25 | 18.00 | 0.00 | 543.98 | 32.00 | 1435.58 |
| 8 | 24 | merkle_whir_batched_after_second | 0.59 | 668.93 | 0.59 | 4.22 | 0.41 | 386.50 | 4.00 | 1065.25 |
| 8 | 24 | canopy_unbatched | 1.50 | 645.53 | 1.75 | 29.25 | 0.00 | 600.00 | 64.00 | 1342.03 |
| 8 | 24 | canopy_schwartz_zippel | 0.59 | 321.98 | 0.84 | 4.22 | 0.91 | 428.75 | 4.00 | 761.29 |
| 8 | 24 | canopy_field_switch_192 | 0.59 | 322.05 | 0.44 | 4.15 | 0.00 | 423.56 | 3.00 | 753.80 |
| 4 | 26 | merkle_whir_independent | 0.75 | 507.99 | 0.75 | 9.75 | 0.00 | 266.00 | 16.00 | 801.24 |
| 4 | 26 | merkle_whir_batched_after_second | 0.38 | 402.67 | 0.38 | 3.28 | 0.16 | 203.00 | 4.00 | 613.85 |
| 4 | 26 | canopy_unbatched | 0.75 | 358.39 | 0.88 | 15.38 | 0.00 | 328.00 | 32.00 | 735.39 |
| 4 | 26 | canopy_schwartz_zippel | 0.38 | 199.58 | 0.50 | 3.28 | 0.41 | 222.50 | 4.00 | 430.64 |
| 4 | 26 | canopy_field_switch_192 | 0.34 | 190.69 | 0.31 | 3.09 | 0.00 | 222.00 | 6.00 | 422.44 |
| 8 | 26 | merkle_whir_independent | 1.50 | 1015.61 | 1.50 | 19.50 | 0.00 | 532.00 | 32.00 | 1602.11 |
| 8 | 26 | merkle_whir_batched_after_second | 0.62 | 769.82 | 0.62 | 4.41 | 0.41 | 385.00 | 4.00 | 1164.88 |
| 8 | 26 | canopy_unbatched | 1.50 | 716.86 | 1.75 | 30.75 | 0.00 | 656.00 | 64.00 | 1470.86 |
| 8 | 26 | canopy_schwartz_zippel | 0.62 | 365.91 | 0.88 | 4.41 | 0.91 | 426.50 | 4.00 | 803.23 |
| 8 | 26 | canopy_field_switch_192 | 0.59 | 357.40 | 0.44 | 4.29 | 0.00 | 426.00 | 6.00 | 794.72 |
| 4 | 28 | merkle_whir_independent | 0.75 | 559.92 | 0.75 | 10.50 | 0.00 | 292.00 | 16.00 | 879.92 |
| 4 | 28 | merkle_whir_batched_after_second | 0.38 | 450.00 | 0.38 | 3.47 | 0.16 | 209.50 | 4.00 | 667.88 |
| 4 | 28 | canopy_unbatched | 1.00 | 401.98 | 1.12 | 17.62 | 0.00 | 350.00 | 32.00 | 803.73 |
| 4 | 28 | canopy_schwartz_zippel | 0.38 | 216.21 | 0.50 | 3.47 | 0.41 | 230.25 | 4.00 | 455.21 |
| 4 | 28 | canopy_field_switch_192 | 0.38 | 216.17 | 0.36 | 3.23 | 0.00 | 223.69 | 3.00 | 446.83 |
| 8 | 28 | merkle_whir_independent | 1.50 | 1109.60 | 1.50 | 21.00 | 0.00 | 594.00 | 32.00 | 1759.60 |
| 8 | 28 | merkle_whir_batched_after_second | 0.62 | 861.66 | 0.62 | 4.59 | 0.41 | 392.75 | 4.00 | 1264.66 |
| 8 | 28 | canopy_unbatched | 2.00 | 803.75 | 2.25 | 35.25 | 0.00 | 700.00 | 64.00 | 1607.25 |
| 8 | 28 | canopy_schwartz_zippel | 0.62 | 398.55 | 0.88 | 4.59 | 0.91 | 434.25 | 4.00 | 843.80 |
| 8 | 28 | canopy_field_switch_192 | 0.62 | 399.51 | 0.48 | 4.43 | 0.00 | 427.69 | 3.00 | 835.74 |
| 4 | 30 | merkle_whir_independent | 0.75 | 589.51 | 0.75 | 11.25 | 0.00 | 340.00 | 16.00 | 958.26 |
| 4 | 30 | merkle_whir_batched_after_second | 0.41 | 505.05 | 0.41 | 3.66 | 0.16 | 210.25 | 4.00 | 723.92 |
| 4 | 30 | canopy_unbatched | 1.00 | 448.62 | 1.12 | 18.38 | 0.00 | 362.00 | 32.00 | 863.12 |
| 4 | 30 | canopy_schwartz_zippel | 0.41 | 242.50 | 0.53 | 3.66 | 0.41 | 228.75 | 4.00 | 480.25 |
| 4 | 30 | canopy_field_switch_192 | 0.38 | 235.26 | 0.36 | 3.38 | 0.00 | 225.38 | 6.00 | 470.75 |
| 8 | 30 | merkle_whir_independent | 1.50 | 1177.88 | 1.50 | 22.50 | 0.00 | 680.00 | 32.00 | 1915.38 |
| 8 | 30 | merkle_whir_batched_after_second | 0.66 | 963.63 | 0.66 | 4.78 | 0.41 | 392.25 | 4.00 | 1366.38 |
| 8 | 30 | canopy_unbatched | 2.00 | 897.43 | 2.25 | 36.75 | 0.00 | 724.00 | 64.00 | 1726.43 |
| 8 | 30 | canopy_schwartz_zippel | 0.66 | 442.00 | 0.91 | 4.78 | 0.91 | 432.75 | 4.00 | 886.00 |
| 8 | 30 | canopy_field_switch_192 | 0.62 | 434.51 | 0.48 | 4.57 | 0.00 | 429.38 | 6.00 | 875.57 |

## Oracle queries at n=30

`unique` is the mean number of distinct positions per queried oracle group; authentication and answer columns are total transmitted values across the reported oracle count.

| exp | t | scheme | role | oracles | code | field | requested | unique | auth | answer fields |
|---:|---:|---|---|---:|---|---:|---:|---:|---:|---:|
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
| 4 | 4 | canopy_field_switch_192 | GF(2^192) recursive oracle 1 | 1 | [2^30,2^24] | 192 | 22 | 22.00 | 455.15 | 352.00 |
| 4 | 4 | canopy_field_switch_192 | GF(2^192) recursive oracle 2 | 1 | [2^29,2^20] | 192 | 15 | 15.00 | 304.05 | 240.00 |
| 4 | 4 | canopy_field_switch_192 | GF(2^192) recursive oracle 3 | 1 | [2^28,2^16] | 192 | 11 | 11.00 | 217.37 | 176.00 |
| 4 | 4 | canopy_field_switch_192 | GF(2^192) recursive oracle 4 | 1 | [2^27,2^12] | 192 | 9 | 9.00 | 171.29 | 144.00 |
| 4 | 4 | canopy_field_switch_192 | field-switch source: 8 GF(2^256) roots / 32 GF(2^64) rows | 8 | [2^31,2^27] | 256 | 33 | 33.00 | 5828.48 | 2112.00 |
| 4 | 4 | canopy_field_switch_192 | outer first oracle via Canopy | 4 | [2^31,2^30] | 256 | 138 | 138.00 | 552.00 | 4416.00 |
| 4 | 8 | canopy_field_switch_192 | GF(2^192) recursive oracle 1 | 1 | [2^30,2^24] | 192 | 22 | 22.00 | 454.25 | 352.00 |
| 4 | 8 | canopy_field_switch_192 | GF(2^192) recursive oracle 2 | 1 | [2^29,2^20] | 192 | 15 | 15.00 | 303.56 | 240.00 |
| 4 | 8 | canopy_field_switch_192 | GF(2^192) recursive oracle 3 | 1 | [2^28,2^16] | 192 | 11 | 11.00 | 217.15 | 176.00 |
| 4 | 8 | canopy_field_switch_192 | GF(2^192) recursive oracle 4 | 1 | [2^27,2^12] | 192 | 9 | 9.00 | 170.65 | 144.00 |
| 4 | 8 | canopy_field_switch_192 | field-switch source: 16 GF(2^256) roots / 64 GF(2^64) rows | 16 | [2^31,2^27] | 256 | 33 | 33.00 | 11654.72 | 4224.00 |
| 4 | 8 | canopy_field_switch_192 | outer first oracle via Canopy | 8 | [2^31,2^30] | 256 | 138 | 138.00 | 1104.00 | 8832.00 |

## Experiment 4 soundness events at n=30

| t | phase | event | bits |
|---:|---|---|---:|
| 4 | outer initial transition input 1 | initial folding / sumcheck | 218.68 |
| 4 | outer initial transition input 1 | initial OOD sampling | 154.36 |
| 4 | outer initial transition input 1 | oracle 0 proximity queries | 128.29 |
| 4 | outer initial transition input 2 | initial folding / sumcheck | 218.68 |
| 4 | outer initial transition input 2 | initial OOD sampling | 154.36 |
| 4 | outer initial transition input 2 | oracle 0 proximity queries | 128.29 |
| 4 | outer initial transition input 3 | initial folding / sumcheck | 218.68 |
| 4 | outer initial transition input 3 | initial OOD sampling | 154.36 |
| 4 | outer initial transition input 3 | oracle 0 proximity queries | 128.29 |
| 4 | outer initial transition input 4 | initial folding / sumcheck | 218.68 |
| 4 | outer initial transition input 4 | initial OOD sampling | 154.36 |
| 4 | outer initial transition input 4 | oracle 0 proximity queries | 128.29 |
| 4 | field switch | one-round 32-coordinate random projection | 142.72 |
| 4 | GF(2^192) recursive WHIR | initial folding / sumcheck | 143.68 |
| 4 | GF(2^192) recursive WHIR | initial OOD sampling | 232.36 |
| 4 | GF(2^192) recursive WHIR | oracle 0 proximity queries | 129.68 |
| 4 | GF(2^192) recursive WHIR | oracle 1 proximity queries | 130.45 |
| 4 | GF(2^192) recursive WHIR | oracle 2 proximity queries | 133.94 |
| 4 | GF(2^192) recursive WHIR | oracle 3 proximity queries | 131.23 |
| 4 | GF(2^192) recursive WHIR | oracle 4 proximity queries | 134.37 |
| 4 | GF(2^192) recursive WHIR | recursive round 1 folding / sumcheck | 145.68 |
| 4 | GF(2^192) recursive WHIR | recursive round 1 claim combination | 145.55 |
| 4 | GF(2^192) recursive WHIR | recursive round 1 OOD sampling | 256.36 |
| 4 | GF(2^192) recursive WHIR | recursive round 2 folding / sumcheck | 140.68 |
| 4 | GF(2^192) recursive WHIR | recursive round 2 claim combination | 144.09 |
| 4 | GF(2^192) recursive WHIR | recursive round 2 OOD sampling | 260.36 |
| 4 | GF(2^192) recursive WHIR | recursive round 3 folding / sumcheck | 135.68 |
| 4 | GF(2^192) recursive WHIR | recursive round 3 claim combination | 142.59 |
| 4 | GF(2^192) recursive WHIR | recursive round 3 OOD sampling | 264.36 |
| 4 | GF(2^192) recursive WHIR | recursive round 4 folding / sumcheck | 130.68 |
| 4 | GF(2^192) recursive WHIR | recursive round 4 claim combination | 140.98 |
| 4 | GF(2^192) recursive WHIR | recursive round 4 OOD sampling | 268.36 |
| 4 | GF(2^192) recursive WHIR | terminal sumcheck | 191.00 |
| 8 | outer initial transition input 1 | initial folding / sumcheck | 218.68 |
| 8 | outer initial transition input 1 | initial OOD sampling | 154.36 |
| 8 | outer initial transition input 1 | oracle 0 proximity queries | 128.29 |
| 8 | outer initial transition input 2 | initial folding / sumcheck | 218.68 |
| 8 | outer initial transition input 2 | initial OOD sampling | 154.36 |
| 8 | outer initial transition input 2 | oracle 0 proximity queries | 128.29 |
| 8 | outer initial transition input 3 | initial folding / sumcheck | 218.68 |
| 8 | outer initial transition input 3 | initial OOD sampling | 154.36 |
| 8 | outer initial transition input 3 | oracle 0 proximity queries | 128.29 |
| 8 | outer initial transition input 4 | initial folding / sumcheck | 218.68 |
| 8 | outer initial transition input 4 | initial OOD sampling | 154.36 |
| 8 | outer initial transition input 4 | oracle 0 proximity queries | 128.29 |
| 8 | outer initial transition input 5 | initial folding / sumcheck | 218.68 |
| 8 | outer initial transition input 5 | initial OOD sampling | 154.36 |
| 8 | outer initial transition input 5 | oracle 0 proximity queries | 128.29 |
| 8 | outer initial transition input 6 | initial folding / sumcheck | 218.68 |
| 8 | outer initial transition input 6 | initial OOD sampling | 154.36 |
| 8 | outer initial transition input 6 | oracle 0 proximity queries | 128.29 |
| 8 | outer initial transition input 7 | initial folding / sumcheck | 218.68 |
| 8 | outer initial transition input 7 | initial OOD sampling | 154.36 |
| 8 | outer initial transition input 7 | oracle 0 proximity queries | 128.29 |
| 8 | outer initial transition input 8 | initial folding / sumcheck | 218.68 |
| 8 | outer initial transition input 8 | initial OOD sampling | 154.36 |
| 8 | outer initial transition input 8 | oracle 0 proximity queries | 128.29 |
| 8 | field switch | one-round 64-coordinate random projection | 140.70 |
| 8 | GF(2^192) recursive WHIR | initial folding / sumcheck | 142.68 |
| 8 | GF(2^192) recursive WHIR | initial OOD sampling | 228.36 |
| 8 | GF(2^192) recursive WHIR | oracle 0 proximity queries | 129.68 |
| 8 | GF(2^192) recursive WHIR | oracle 1 proximity queries | 130.45 |
| 8 | GF(2^192) recursive WHIR | oracle 2 proximity queries | 133.94 |
| 8 | GF(2^192) recursive WHIR | oracle 3 proximity queries | 131.23 |
| 8 | GF(2^192) recursive WHIR | oracle 4 proximity queries | 134.37 |
| 8 | GF(2^192) recursive WHIR | recursive round 1 folding / sumcheck | 145.68 |
| 8 | GF(2^192) recursive WHIR | recursive round 1 claim combination | 145.55 |
| 8 | GF(2^192) recursive WHIR | recursive round 1 OOD sampling | 256.36 |
| 8 | GF(2^192) recursive WHIR | recursive round 2 folding / sumcheck | 140.68 |
| 8 | GF(2^192) recursive WHIR | recursive round 2 claim combination | 144.09 |
| 8 | GF(2^192) recursive WHIR | recursive round 2 OOD sampling | 260.36 |
| 8 | GF(2^192) recursive WHIR | recursive round 3 folding / sumcheck | 135.68 |
| 8 | GF(2^192) recursive WHIR | recursive round 3 claim combination | 142.59 |
| 8 | GF(2^192) recursive WHIR | recursive round 3 OOD sampling | 264.36 |
| 8 | GF(2^192) recursive WHIR | recursive round 4 folding / sumcheck | 130.68 |
| 8 | GF(2^192) recursive WHIR | recursive round 4 claim combination | 140.98 |
| 8 | GF(2^192) recursive WHIR | recursive round 4 OOD sampling | 268.36 |
| 8 | GF(2^192) recursive WHIR | terminal sumcheck | 191.00 |
