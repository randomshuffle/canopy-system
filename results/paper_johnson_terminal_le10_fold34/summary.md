# Final Johnson-bound paper proof-size experiments

All experiments target 128-bit round-by-round soundness, use BLAKE3-256, use no proof of work, search every ordered folding schedule over `{3,4}` that stops when it first reaches at most `2^10` coefficients, and use the johnson RS proximity regime. Experiments 1--3 use the 256-bit prime field. Experiment 4 uses the compatible binary tower `B=GF(2^64)`, `L=GF(2^256)`, and `M=GF(2^192)`: its two source oracles are over `L`, while every recursive oracle after the one-round field switch is over `M`. Means use 100 deterministic query samples from seed `0x50415045525f5631`.

## Experiment 1: one Canopy commitment vs Merkle

The table reports the proof-size-optimal Canopy height and folding schedule over the full feasible grid. Detailed rows and proof components are in `experiment1_canopy_vs_merkle/`.

| log2 vector | batch | best h | folds | terminal log | Merkle KiB | Canopy KiB | Canopy / Merkle |
|---:|---:|---:|---|---:|---:|---:|---:|
| 22 | 128 | 21 | terminal | 1 | 60.48 | 68.15 | 1.127x |
| 22 | 256 | 10 | 4 | 8 | 112.94 | 119.56 | 1.059x |
| 22 | 512 | 5 | 3,3,3 | 8 | 209.87 | 180.44 | 0.860x |
| 22 | 1024 | 3 | 3,4,4 | 8 | 387.71 | 263.64 | 0.680x |
| 24 | 128 | 23 | terminal | 1 | 68.51 | 76.38 | 1.115x |
| 24 | 256 | 10 | 3,3 | 8 | 128.88 | 131.86 | 1.023x |
| 24 | 512 | 6 | 3,3,4 | 8 | 241.72 | 198.89 | 0.823x |
| 24 | 1024 | 3 | 3,3,3,4 | 8 | 451.73 | 283.80 | 0.628x |
| 26 | 128 | 25 | terminal | 1 | 76.43 | 84.40 | 1.104x |
| 26 | 256 | 10 | 4,4 | 8 | 144.95 | 143.84 | 0.992x |
| 26 | 512 | 6 | 4,4,4 | 8 | 273.73 | 214.97 | 0.785x |
| 26 | 1024 | 4 | 3,4,4,3 | 8 | 515.51 | 305.77 | 0.593x |
| 28 | 128 | 27 | terminal | 1 | 84.48 | 92.41 | 1.094x |
| 28 | 256 | 12 | 4,4 | 8 | 160.92 | 156.56 | 0.973x |
| 28 | 512 | 6 | 4,4,3,3 | 8 | 305.84 | 231.17 | 0.756x |
| 28 | 1024 | 4 | 4,4,4,4 | 8 | 579.51 | 324.94 | 0.561x |
| 30 | 128 | 29 | terminal | 1 | 92.45 | 100.19 | 1.084x |
| 30 | 256 | 11 | 4,4,4 | 7 | 176.90 | 168.23 | 0.951x |
| 30 | 512 | 7 | 4,4,4,4 | 7 | 337.79 | 245.67 | 0.727x |
| 30 | 1024 | 4 | 4,4,4,3,4 | 7 | 643.53 | 344.86 | 0.536x |

## Experiments 2, 3, and 4: WHIR hybrids

Every folding step is independently selected from `{3,4}`, and each WHIR invocation stops when its remaining message first has at most `2^10` coefficients. The hybrids use no Canopy codeword shortening. Experiment 2 independently searches Canopy height, the outer WHIR schedule, and the Canopy-claim schedule. Experiment 3 needs identical RS codes, fixes `h=f0+1`, and uses the outer schedule after `f0` for its combined aligned claim. It sends one pre-challenge cross-term sum, samples `alpha`, and proves the single degree-two identity for `f+alpha*g` under `L_f+alpha*L_g`. Its additional batching error is at most `2/|F|`, approximately `2^-255`. The complete search is in `hybrid_parameter_grid.csv`.

## WHIR and hybrid proof sizes

| log2 message | scheme | recursive field bits | outer folds | outer terminal | h | Canopy folds | Canopy terminal | min RBR event bits | algebraic union-bound bits | KiB | vs Merkle WHIR | hashes KiB | fields KiB |
|---:|---|---:|---|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 256 | 3,3,4,4 | 8 | - | - | - | 128.02 | 126.67 | 311.97 | 1.000x | 161.80 | 150.17 |
| 22 | canopy_separate | 256 | 3,3,4,4 | 8 | 9 | 3,3 | 8 | 128.02 | 126.16 | 292.94 | 0.939x | 132.38 | 160.55 |
| 22 | canopy_schwartz_zippel | 256 | 3,3,4,4 | 8 | 4 | 3,4,4 | 8 | 128.02 | 126.59 | 245.03 | 0.785x | 90.28 | 154.75 |
| 22 | canopy_field_switch_192 | 192 | 3,3,4,4 | 8 | 4 | 3,4,4 | 8 | 128.02 | 126.59 | 233.65 | 0.749x | 90.35 | 143.30 |
| 24 | merkle_whir | 256 | 3,3,3,3,4 | 8 | - | - | - | 128.02 | 126.35 | 350.82 | 1.000x | 206.17 | 144.65 |
| 24 | canopy_separate | 256 | 3,3,3,3,4 | 8 | 9 | 4,4 | 8 | 128.02 | 125.99 | 325.15 | 0.927x | 156.19 | 168.96 |
| 24 | canopy_schwartz_zippel | 256 | 3,3,3,3,4 | 8 | 4 | 3,3,3,4 | 8 | 128.02 | 126.35 | 267.97 | 0.764x | 117.26 | 150.71 |
| 24 | canopy_field_switch_192 | 192 | 3,3,4,3,3 | 8 | 4 | 3,4,3,3 | 8 | 128.02 | 126.48 | 257.00 | 0.733x | 114.47 | 142.53 |
| 26 | merkle_whir | 256 | 3,3,3,3,3,3 | 8 | - | - | - | 128.02 | 126.26 | 392.67 | 1.000x | 249.04 | 143.62 |
| 26 | canopy_separate | 256 | 3,4,4,4,3 | 8 | 9 | 4,3,3 | 8 | 128.02 | 125.95 | 358.85 | 0.914x | 168.72 | 190.12 |
| 26 | canopy_schwartz_zippel | 256 | 3,3,4,4,4 | 8 | 4 | 3,4,4,4 | 8 | 128.02 | 126.49 | 292.50 | 0.745x | 126.34 | 166.15 |
| 26 | canopy_field_switch_192 | 192 | 3,3,4,4,4 | 8 | 4 | 3,4,4,4 | 8 | 128.02 | 126.48 | 278.21 | 0.709x | 126.35 | 151.86 |
| 28 | merkle_whir | 256 | 3,3,4,3,4,3 | 8 | - | - | - | 128.02 | 125.98 | 434.80 | 1.000x | 274.23 | 160.56 |
| 28 | canopy_separate | 256 | 3,4,4,3,3,3 | 8 | 10 | 4,4,4 | 7 | 128.02 | 125.89 | 390.69 | 0.899x | 198.75 | 191.94 |
| 28 | canopy_schwartz_zippel | 256 | 3,3,4,4,3,3 | 8 | 4 | 3,4,4,3,3 | 8 | 128.02 | 126.45 | 317.09 | 0.729x | 151.46 | 165.62 |
| 28 | canopy_field_switch_192 | 192 | 3,3,4,4,3,4 | 7 | 4 | 3,4,4,3,4 | 7 | 128.02 | 126.26 | 302.91 | 0.697x | 150.89 | 152.02 |
| 30 | merkle_whir | 256 | 3,4,4,4,4,4 | 7 | - | - | - | 128.02 | 126.38 | 475.47 | 1.000x | 292.47 | 183.00 |
| 30 | canopy_separate | 256 | 3,4,4,4,4,4 | 7 | 11 | 4,4,4 | 8 | 128.02 | 126.22 | 421.17 | 0.886x | 216.20 | 204.97 |
| 30 | canopy_schwartz_zippel | 256 | 3,3,4,4,4,4 | 8 | 4 | 3,4,4,4,4 | 8 | 128.02 | 126.48 | 340.64 | 0.716x | 165.08 | 175.56 |
| 30 | canopy_field_switch_192 | 192 | 3,3,3,3,3,3,4 | 8 | 4 | 3,3,3,3,3,4 | 8 | 128.02 | 125.91 | 332.63 | 0.700x | 185.51 | 147.12 |

## Experiments 3 and 4 vs Merkle WHIR components

All entries below are KiB. Roots and authentication paths are hashes. Experiment 4's first-oracle and source-root answers are 256-bit elements; its field-switch transcript and recursive proof use 192-bit elements.

| log2 message | scheme | roots | authentication | OOD | sumcheck | batching | query answers | terminal coefficients | total |
|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 0.12 | 161.68 | 0.12 | 2.06 | 0.00 | 139.98 | 8.00 | 311.97 |
| 22 | canopy_schwartz_zippel | 0.12 | 90.16 | 0.16 | 2.06 | 0.03 | 144.50 | 8.00 | 245.03 |
| 22 | canopy_field_switch_192 | 0.12 | 90.22 | 0.10 | 1.83 | 0.00 | 135.37 | 6.00 | 233.65 |
| 24 | merkle_whir | 0.16 | 206.01 | 0.16 | 2.25 | 0.00 | 134.25 | 8.00 | 350.82 |
| 24 | canopy_schwartz_zippel | 0.16 | 117.10 | 0.19 | 2.25 | 0.03 | 140.24 | 8.00 | 267.97 |
| 24 | canopy_field_switch_192 | 0.16 | 114.32 | 0.12 | 1.97 | 0.00 | 134.43 | 6.00 | 257.00 |
| 26 | merkle_whir | 0.19 | 248.85 | 0.19 | 2.44 | 0.00 | 133.00 | 8.00 | 392.67 |
| 26 | canopy_schwartz_zippel | 0.16 | 126.19 | 0.19 | 2.44 | 0.03 | 155.50 | 8.00 | 292.50 |
| 26 | canopy_field_switch_192 | 0.16 | 126.20 | 0.12 | 2.11 | 0.00 | 143.62 | 6.00 | 278.21 |
| 28 | merkle_whir | 0.19 | 274.05 | 0.19 | 2.62 | 0.00 | 149.75 | 8.00 | 434.80 |
| 28 | canopy_schwartz_zippel | 0.19 | 151.27 | 0.22 | 2.62 | 0.03 | 154.75 | 8.00 | 317.09 |
| 28 | canopy_field_switch_192 | 0.19 | 150.70 | 0.15 | 2.25 | 0.00 | 146.62 | 3.00 | 302.91 |
| 30 | merkle_whir | 0.19 | 292.29 | 0.19 | 2.81 | 0.00 | 176.00 | 4.00 | 475.47 |
| 30 | canopy_schwartz_zippel | 0.19 | 164.89 | 0.22 | 2.81 | 0.03 | 164.50 | 8.00 | 340.64 |
| 30 | canopy_field_switch_192 | 0.22 | 185.29 | 0.17 | 2.39 | 0.00 | 138.56 | 6.00 | 332.63 |

## Experiment 4 field-switch reduction

Let `B=GF(2^64)`, `L=GF(2^256)`, and `M=GF(2^192)`, with both `L` and `M` extensions of `B`. The additive RS domain lies in `B`. Each of the two aligned `L`-valued source codewords is fixed by its own Merkle root before the switch challenge. Expanding every `L` symbol in a fixed four-element `B` basis produces eight `B`-valued RS rows. The eight coordinate equations implied by the two `L` claims are batched with one `zeta <- M` using weights `1,zeta,...,zeta^7`. Three selector variables index the eight rows. Their fold is combined with the first spatial WHIR fold, so the next commitment contains one `M`-valued RS word; all subsequent OOD values, sumcheck messages, folding challenges, query answers, and terminal coefficients are in `M`.

This is a one-round random projection, not parallel repetition over `B`. For any fixed false coordinate-claim vector, its batched discrepancy is a nonzero polynomial in `zeta` of degree at most seven. After unioning over the conservative correlated list, the modeled switch error is at most `7*|Lambda|/|M|`. The 64-bit ground field supplies only deterministic coordinates and the common RS domain; no verifier challenge is sampled from it. The selected Experiment 4 row is the smallest proof whose switch bound and every modeled WHIR RBR event are each at least 128 bits. `minimum_rbr_security_bits` is the per-event minimum; `algebraic_rbr_union_bound_bits` additionally applies a plain union bound across the listed algebraic bad events.

### Experiment 4 RBR events at n=30

| phase | event | security bits |
|---|---|---:|
| outer initial transition | initial folding / sumcheck | 207.12 |
| outer initial transition | initial OOD sampling | 218.36 |
| outer initial transition | oracle 0 proximity queries | 128.02 |
| field switch | one-round eight-coordinate random projection | 181.87 |
| GF(2^192) recursive WHIR | initial folding / sumcheck | 135.62 |
| GF(2^192) recursive WHIR | initial OOD sampling | 148.36 |
| GF(2^192) recursive WHIR | oracle 0 proximity queries | 129.28 |
| GF(2^192) recursive WHIR | oracle 1 proximity queries | 128.90 |
| GF(2^192) recursive WHIR | oracle 2 proximity queries | 129.68 |
| GF(2^192) recursive WHIR | oracle 3 proximity queries | 128.17 |
| GF(2^192) recursive WHIR | oracle 4 proximity queries | 130.45 |
| GF(2^192) recursive WHIR | oracle 5 proximity queries | 131.66 |
| GF(2^192) recursive WHIR | recursive round 1 folding / sumcheck | 136.62 |
| GF(2^192) recursive WHIR | recursive round 1 claim combination | 175.59 |
| GF(2^192) recursive WHIR | recursive round 1 OOD sampling | 150.36 |
| GF(2^192) recursive WHIR | recursive round 2 folding / sumcheck | 134.62 |
| GF(2^192) recursive WHIR | recursive round 2 claim combination | 174.19 |
| GF(2^192) recursive WHIR | recursive round 2 OOD sampling | 149.36 |
| GF(2^192) recursive WHIR | recursive round 3 folding / sumcheck | 132.62 |
| GF(2^192) recursive WHIR | recursive round 3 claim combination | 172.59 |
| GF(2^192) recursive WHIR | recursive round 3 OOD sampling | 148.36 |
| GF(2^192) recursive WHIR | recursive round 4 folding / sumcheck | 130.62 |
| GF(2^192) recursive WHIR | recursive round 4 claim combination | 170.92 |
| GF(2^192) recursive WHIR | recursive round 4 OOD sampling | 147.36 |
| GF(2^192) recursive WHIR | recursive round 5 folding / sumcheck | 128.62 |
| GF(2^192) recursive WHIR | recursive round 5 claim combination | 169.15 |
| GF(2^192) recursive WHIR | recursive round 5 OOD sampling | 146.36 |
| GF(2^192) recursive WHIR | terminal sumcheck | 191.00 |

## Oracle-query breakdown

The machine-readable table is `oracle_query_breakdown.csv`. Values below are the largest proof-size point (`n=30`).

| experiment | scheme | oracle | code | field bits | requested | unique | auth hashes | answer fields |
|---:|---|---|---|---:|---:|---:|---:|---:|
| 2 | canopy_separate | Canopy claim WHIR initial oracle | [2^31, 2^20] | 256 | 24 | 24.00 | 516.75 | 384.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 1 | [2^30, 2^16] | 256 | 19 | 19.00 | 397.20 | 304.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 2 | [2^29, 2^12] | 256 | 16 | 16.00 | 323.51 | 256.00 |
| 2 | canopy_separate | outer first oracle via Canopy | [2^31, 2^30] | 256 | 298 | 298.00 | 2383.77 | 2384.00 |
| 2 | canopy_separate | outer second-oracle WHIR initial oracle | [2^31, 2^27] | 256 | 67 | 67.00 | 1344.98 | 1072.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 1 | [2^30, 2^23] | 256 | 38 | 38.00 | 755.87 | 608.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 2 | [2^29, 2^19] | 256 | 26 | 26.00 | 504.76 | 416.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 3 | [2^28, 2^15] | 256 | 20 | 20.00 | 376.47 | 320.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 4 | [2^27, 2^11] | 256 | 17 | 17.00 | 307.17 | 272.00 |
| 2 | merkle_whir | outer WHIR initial oracle | [2^31, 2^30] | 256 | 298 | 298.00 | 5631.69 | 2384.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 1 | [2^30, 2^27] | 256 | 90 | 90.00 | 1675.63 | 1440.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 2 | [2^29, 2^23] | 256 | 44 | 44.00 | 821.46 | 704.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 3 | [2^28, 2^19] | 256 | 29 | 29.00 | 529.98 | 464.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 4 | [2^27, 2^15] | 256 | 22 | 22.00 | 388.33 | 352.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 5 | [2^26, 2^11] | 256 | 18 | 18.00 | 306.07 | 288.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim initial oracle | [2^31, 2^27] | 256 | 67 | 67.00 | 2816.86 | 1072.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 1 | [2^30, 2^24] | 256 | 44 | 44.00 | 867.22 | 704.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 2 | [2^29, 2^20] | 256 | 29 | 29.00 | 559.16 | 464.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 3 | [2^28, 2^16] | 256 | 22 | 22.00 | 411.32 | 352.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 4 | [2^27, 2^12] | 256 | 18 | 18.00 | 323.93 | 288.00 |
| 3 | canopy_schwartz_zippel | outer first oracle via Canopy | [2^31, 2^30] | 256 | 298 | 298.00 | 298.00 | 2384.00 |
| 4 | canopy_field_switch_192 | GF(2^192) recursive oracle 1 | [2^30, 2^24] | 192 | 44 | 44.00 | 908.86 | 352.00 |
| 4 | canopy_field_switch_192 | GF(2^192) recursive oracle 2 | [2^29, 2^21] | 192 | 33 | 33.00 | 662.79 | 264.00 |
| 4 | canopy_field_switch_192 | GF(2^192) recursive oracle 3 | [2^28, 2^18] | 192 | 26 | 26.00 | 506.18 | 208.00 |
| 4 | canopy_field_switch_192 | GF(2^192) recursive oracle 4 | [2^27, 2^15] | 192 | 22 | 22.00 | 410.39 | 176.00 |
| 4 | canopy_field_switch_192 | GF(2^192) recursive oracle 5 | [2^26, 2^12] | 192 | 19 | 19.00 | 321.17 | 304.00 |
| 4 | canopy_field_switch_192 | field-switch source: two GF(2^256) roots / eight GF(2^64) rows | [2^31, 2^27] | 256 | 67 | 67.00 | 2821.84 | 1072.00 |
| 4 | canopy_field_switch_192 | outer first oracle via Canopy | [2^31, 2^30] | 256 | 298 | 298.00 | 298.00 | 2384.00 |
