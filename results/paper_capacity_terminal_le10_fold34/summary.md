# Conjectural capacity-bound paper proof-size experiments

All experiments target 128-bit round-by-round soundness, use BLAKE3-256, use no proof of work, search every ordered folding schedule over `{3,4}` that stops when it first reaches at most `2^10` coefficients, and use the capacity_conjecture RS proximity regime. Experiments 1--3 use the 256-bit prime field. Experiment 4 uses the compatible binary tower `B=GF(2^64)`, `L=GF(2^256)`, and `M=GF(2^192)`: its two source oracles are over `L`, while every recursive oracle after the one-round field switch is over `M`. Capacity-bound soundness relies on STIR Conjecture 5.6 and is not proved. Means use 100 deterministic query samples from seed `0x50415045525f5631`.

## Experiment 1: one Canopy commitment vs Merkle

The table reports the proof-size-optimal Canopy height and folding schedule over the full feasible grid. Detailed rows and proof components are in `experiment1_canopy_vs_merkle/`.

| log2 vector | batch | best h | folds | terminal log | Merkle KiB | Canopy KiB | Canopy / Merkle |
|---:|---:|---:|---|---:|---:|---:|---:|
| 22 | 128 | 11 | 4 | 7 | 60.48 | 63.77 | 1.054x |
| 22 | 256 | 5 | 3,3,4 | 7 | 112.94 | 96.30 | 0.853x |
| 22 | 512 | 3 | 3,4,4 | 8 | 209.87 | 139.00 | 0.662x |
| 22 | 1024 | 2 | 3,3,3,4 | 7 | 387.71 | 192.18 | 0.496x |
| 24 | 128 | 10 | 3,4 | 7 | 68.51 | 69.87 | 1.020x |
| 24 | 256 | 6 | 4,3,4 | 7 | 128.88 | 104.93 | 0.814x |
| 24 | 512 | 3 | 3,3,4,4 | 7 | 241.72 | 148.30 | 0.614x |
| 24 | 1024 | 2 | 3,4,4,4 | 7 | 451.73 | 207.10 | 0.458x |
| 26 | 128 | 11 | 4,4 | 7 | 76.43 | 76.35 | 0.999x |
| 26 | 256 | 7 | 4,4,4 | 7 | 144.95 | 112.62 | 0.777x |
| 26 | 512 | 4 | 4,4,3,4 | 7 | 273.73 | 158.81 | 0.580x |
| 26 | 1024 | 2 | 3,3,4,3,4 | 7 | 515.51 | 220.72 | 0.428x |
| 28 | 128 | 10 | 4,3,4 | 7 | 84.48 | 82.52 | 0.977x |
| 28 | 256 | 7 | 4,3,3,4 | 7 | 160.92 | 121.54 | 0.755x |
| 28 | 512 | 4 | 3,4,3,3,4 | 7 | 305.84 | 169.44 | 0.554x |
| 28 | 1024 | 2 | 3,4,4,4,4 | 7 | 579.51 | 234.06 | 0.404x |
| 30 | 128 | 12 | 4,3,4 | 7 | 92.45 | 88.29 | 0.955x |
| 30 | 256 | 7 | 4,4,4,4 | 7 | 176.90 | 128.11 | 0.724x |
| 30 | 512 | 4 | 4,4,4,3,4 | 7 | 337.79 | 177.90 | 0.527x |
| 30 | 1024 | 3 | 4,4,4,4,4 | 7 | 643.53 | 246.43 | 0.383x |

## Experiments 2, 3, and 4: WHIR hybrids

Every folding step is independently selected from `{3,4}`, and each WHIR invocation stops when its remaining message first has at most `2^10` coefficients. The hybrids use no Canopy codeword shortening. Experiment 2 independently searches Canopy height, the outer WHIR schedule, and the Canopy-claim schedule. Experiment 3 needs identical RS codes, fixes `h=f0+1`, and uses the outer schedule after `f0` for its combined aligned claim. It sends one pre-challenge cross-term sum, samples `alpha`, and proves the single degree-two identity for `f+alpha*g` under `L_f+alpha*L_g`. Its additional batching error is at most `2/|F|`, approximately `2^-255`. The complete search is in `hybrid_parameter_grid.csv`.

## WHIR and hybrid proof sizes

| log2 message | scheme | recursive field bits | outer folds | outer terminal | h | Canopy folds | Canopy terminal | min RBR event bits | algebraic union-bound bits | KiB | vs Merkle WHIR | hashes KiB | fields KiB |
|---:|---|---:|---|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 256 | 3,3,4,4 | 8 | - | - | - | 128.17 | 126.83 | 161.63 | 1.000x | 84.45 | 77.18 |
| 22 | canopy_separate | 256 | 3,4,4,4 | 7 | 9 | 3,4 | 7 | 128.29 | 127.14 | 152.76 | 0.945x | 64.46 | 88.30 |
| 22 | canopy_schwartz_zippel | 256 | 3,3,4,4 | 8 | 4 | 3,4,4 | 8 | 128.29 | 127.59 | 127.60 | 0.789x | 47.85 | 79.75 |
| 22 | canopy_field_switch_192 | 192 | 3,3,4,4 | 8 | 4 | 3,4,4 | 8 | 128.29 | 127.59 | 120.85 | 0.748x | 47.97 | 72.88 |
| 24 | merkle_whir | 256 | 3,3,3,4,4 | 7 | - | - | - | 128.17 | 126.52 | 179.52 | 1.000x | 105.12 | 74.40 |
| 24 | canopy_separate | 256 | 3,3,4,3,4 | 7 | 10 | 4,4 | 7 | 128.29 | 126.78 | 167.84 | 0.935x | 80.97 | 86.87 |
| 24 | canopy_schwartz_zippel | 256 | 3,3,4,3,4 | 7 | 4 | 3,4,3,4 | 7 | 128.29 | 127.47 | 138.11 | 0.769x | 59.89 | 78.22 |
| 24 | canopy_field_switch_192 | 192 | 3,3,4,3,4 | 7 | 4 | 3,4,3,4 | 7 | 128.29 | 127.47 | 131.76 | 0.734x | 60.02 | 71.75 |
| 26 | merkle_whir | 256 | 3,3,3,3,3,4 | 7 | - | - | - | 128.17 | 126.71 | 200.11 | 1.000x | 126.99 | 73.12 |
| 26 | canopy_separate | 256 | 3,4,4,4,4 | 7 | 12 | 4,4 | 7 | 128.29 | 126.86 | 183.86 | 0.919x | 89.80 | 94.06 |
| 26 | canopy_schwartz_zippel | 256 | 3,3,3,3,3,4 | 7 | 4 | 3,3,3,3,4 | 7 | 128.29 | 127.07 | 150.95 | 0.754x | 74.76 | 76.19 |
| 26 | canopy_field_switch_192 | 192 | 3,3,4,4,4 | 8 | 4 | 3,4,4,4 | 8 | 128.29 | 127.47 | 143.58 | 0.718x | 66.25 | 77.33 |
| 28 | merkle_whir | 256 | 3,3,4,3,4,4 | 7 | - | - | - | 128.17 | 126.35 | 220.04 | 1.000x | 138.97 | 81.06 |
| 28 | canopy_separate | 256 | 3,4,4,3,3,4 | 7 | 10 | 4,4,4 | 7 | 128.29 | 126.58 | 201.02 | 0.914x | 100.83 | 100.19 |
| 28 | canopy_schwartz_zippel | 256 | 3,3,4,4,3,4 | 7 | 4 | 3,4,4,3,4 | 7 | 128.29 | 127.47 | 163.19 | 0.742x | 79.07 | 84.12 |
| 28 | canopy_field_switch_192 | 192 | 3,3,4,4,3,4 | 7 | 4 | 3,4,4,3,4 | 7 | 128.29 | 127.46 | 155.31 | 0.706x | 79.10 | 76.20 |
| 30 | merkle_whir | 256 | 3,4,4,4,4,4 | 7 | - | - | - | 128.29 | 127.26 | 239.49 | 1.000x | 147.49 | 92.00 |
| 30 | canopy_separate | 256 | 3,4,4,4,4,4 | 7 | 12 | 4,4,4 | 7 | 128.29 | 126.86 | 215.76 | 0.901x | 112.39 | 103.38 |
| 30 | canopy_schwartz_zippel | 256 | 3,3,4,3,3,3,4 | 7 | 4 | 3,4,3,3,3,4 | 7 | 128.29 | 127.10 | 176.01 | 0.735x | 93.16 | 82.84 |
| 30 | canopy_field_switch_192 | 192 | 3,3,4,4,4,4 | 8 | 4 | 3,4,4,4,4 | 8 | 128.29 | 127.31 | 166.90 | 0.697x | 85.87 | 81.03 |

## Experiments 3 and 4 vs Merkle WHIR components

All entries below are KiB. Roots and authentication paths are hashes. Experiment 4's first-oracle and source-root answers are 256-bit elements; its field-switch transcript and recursive proof use 192-bit elements.

| log2 message | scheme | roots | authentication | OOD | sumcheck | batching | query answers | terminal coefficients | total |
|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 22 | merkle_whir | 0.12 | 84.32 | 0.12 | 2.06 | 0.00 | 67.00 | 8.00 | 161.63 |
| 22 | canopy_schwartz_zippel | 0.12 | 47.72 | 0.16 | 2.06 | 0.03 | 69.50 | 8.00 | 127.60 |
| 22 | canopy_field_switch_192 | 0.12 | 47.85 | 0.17 | 1.83 | 0.00 | 64.88 | 6.00 | 120.85 |
| 24 | merkle_whir | 0.16 | 104.96 | 0.16 | 2.25 | 0.00 | 68.00 | 4.00 | 179.52 |
| 24 | canopy_schwartz_zippel | 0.16 | 59.74 | 0.19 | 2.25 | 0.03 | 71.75 | 4.00 | 138.11 |
| 24 | canopy_field_switch_192 | 0.16 | 59.86 | 0.22 | 1.97 | 0.00 | 66.56 | 3.00 | 131.76 |
| 26 | merkle_whir | 0.19 | 126.80 | 0.19 | 2.44 | 0.00 | 66.50 | 4.00 | 200.11 |
| 26 | canopy_schwartz_zippel | 0.19 | 74.58 | 0.22 | 2.44 | 0.03 | 69.50 | 4.00 | 150.95 |
| 26 | canopy_field_switch_192 | 0.16 | 66.10 | 0.22 | 2.11 | 0.00 | 69.00 | 6.00 | 143.58 |
| 28 | merkle_whir | 0.19 | 138.79 | 0.19 | 2.62 | 0.00 | 74.25 | 4.00 | 220.04 |
| 28 | canopy_schwartz_zippel | 0.19 | 78.88 | 0.22 | 2.62 | 0.03 | 77.25 | 4.00 | 163.19 |
| 28 | canopy_field_switch_192 | 0.19 | 78.92 | 0.27 | 2.25 | 0.00 | 70.69 | 3.00 | 155.31 |
| 30 | merkle_whir | 0.19 | 147.30 | 0.19 | 2.81 | 0.00 | 85.00 | 4.00 | 239.49 |
| 30 | canopy_schwartz_zippel | 0.22 | 92.94 | 0.25 | 2.81 | 0.03 | 75.75 | 4.00 | 176.01 |
| 30 | canopy_field_switch_192 | 0.19 | 85.68 | 0.27 | 2.39 | 0.00 | 72.38 | 6.00 | 166.90 |

## Experiment 4 field-switch reduction

Let `B=GF(2^64)`, `L=GF(2^256)`, and `M=GF(2^192)`, with both `L` and `M` extensions of `B`. The additive RS domain lies in `B`. Each of the two aligned `L`-valued source codewords is fixed by its own Merkle root before the switch challenge. Expanding every `L` symbol in a fixed four-element `B` basis produces eight `B`-valued RS rows. The eight coordinate equations implied by the two `L` claims are batched with one `zeta <- M` using weights `1,zeta,...,zeta^7`. Three selector variables index the eight rows. Their fold is combined with the first spatial WHIR fold, so the next commitment contains one `M`-valued RS word; all subsequent OOD values, sumcheck messages, folding challenges, query answers, and terminal coefficients are in `M`.

This is a one-round random projection, not parallel repetition over `B`. For any fixed false coordinate-claim vector, its batched discrepancy is a nonzero polynomial in `zeta` of degree at most seven. After unioning over the conservative correlated list, the modeled switch error is at most `7*|Lambda|/|M|`. The 64-bit ground field supplies only deterministic coordinates and the common RS domain; no verifier challenge is sampled from it. The selected Experiment 4 row is the smallest proof whose switch bound and every modeled WHIR RBR event are each at least 128 bits. `minimum_rbr_security_bits` is the per-event minimum; `algebraic_rbr_union_bound_bits` additionally applies a plain union bound across the listed algebraic bad events.

### Experiment 4 RBR events at n=30

| phase | event | security bits |
|---|---|---:|
| outer initial transition | initial folding / sumcheck | 218.68 |
| outer initial transition | initial OOD sampling | 154.36 |
| outer initial transition | oracle 0 proximity queries | 128.29 |
| field switch | one-round eight-coordinate random projection | 146.87 |
| GF(2^192) recursive WHIR | initial folding / sumcheck | 145.68 |
| GF(2^192) recursive WHIR | initial OOD sampling | 240.36 |
| GF(2^192) recursive WHIR | oracle 0 proximity queries | 129.68 |
| GF(2^192) recursive WHIR | oracle 1 proximity queries | 130.45 |
| GF(2^192) recursive WHIR | oracle 2 proximity queries | 133.94 |
| GF(2^192) recursive WHIR | oracle 3 proximity queries | 131.23 |
| GF(2^192) recursive WHIR | oracle 4 proximity queries | 134.37 |
| GF(2^192) recursive WHIR | recursive round 1 folding / sumcheck | 145.68 |
| GF(2^192) recursive WHIR | recursive round 1 claim combination | 145.55 |
| GF(2^192) recursive WHIR | recursive round 1 OOD sampling | 256.36 |
| GF(2^192) recursive WHIR | recursive round 2 folding / sumcheck | 140.68 |
| GF(2^192) recursive WHIR | recursive round 2 claim combination | 144.09 |
| GF(2^192) recursive WHIR | recursive round 2 OOD sampling | 260.36 |
| GF(2^192) recursive WHIR | recursive round 3 folding / sumcheck | 135.68 |
| GF(2^192) recursive WHIR | recursive round 3 claim combination | 142.59 |
| GF(2^192) recursive WHIR | recursive round 3 OOD sampling | 264.36 |
| GF(2^192) recursive WHIR | recursive round 4 folding / sumcheck | 130.68 |
| GF(2^192) recursive WHIR | recursive round 4 claim combination | 140.98 |
| GF(2^192) recursive WHIR | recursive round 4 OOD sampling | 268.36 |
| GF(2^192) recursive WHIR | terminal sumcheck | 191.00 |

## Oracle-query breakdown

The machine-readable table is `oracle_query_breakdown.csv`. Values below are the largest proof-size point (`n=30`).

| experiment | scheme | oracle | code | field bits | requested | unique | auth hashes | answer fields |
|---:|---|---|---|---:|---:|---:|---:|---:|
| 2 | canopy_separate | Canopy claim WHIR initial oracle | [2^31, 2^19] | 256 | 11 | 11.00 | 250.34 | 176.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 1 | [2^30, 2^15] | 256 | 9 | 9.00 | 197.91 | 144.00 |
| 2 | canopy_separate | Canopy claim WHIR recursive oracle 2 | [2^29, 2^11] | 256 | 8 | 8.00 | 169.68 | 128.00 |
| 2 | canopy_separate | outer first oracle via Canopy | [2^31, 2^30] | 256 | 138 | 138.00 | 1241.89 | 1104.00 |
| 2 | canopy_separate | outer second-oracle WHIR initial oracle | [2^31, 2^27] | 256 | 33 | 33.00 | 695.36 | 528.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 1 | [2^30, 2^23] | 256 | 19 | 19.00 | 397.26 | 304.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 2 | [2^29, 2^19] | 256 | 13 | 13.00 | 266.69 | 208.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 3 | [2^28, 2^15] | 256 | 10 | 10.00 | 198.32 | 160.00 |
| 2 | canopy_separate | outer second-oracle WHIR recursive oracle 4 | [2^27, 2^11] | 256 | 9 | 9.00 | 170.96 | 144.00 |
| 2 | merkle_whir | outer WHIR initial oracle | [2^31, 2^30] | 256 | 138 | 138.00 | 2760.30 | 1104.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 1 | [2^30, 2^27] | 256 | 44 | 44.00 | 864.93 | 704.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 2 | [2^29, 2^23] | 256 | 22 | 22.00 | 432.20 | 352.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 3 | [2^28, 2^19] | 256 | 15 | 15.00 | 289.17 | 240.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 4 | [2^27, 2^15] | 256 | 11 | 11.00 | 205.67 | 176.00 |
| 2 | merkle_whir | outer WHIR recursive oracle 5 | [2^26, 2^11] | 256 | 9 | 9.00 | 161.33 | 144.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim initial oracle | [2^31, 2^27] | 256 | 33 | 33.00 | 1453.30 | 528.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 1 | [2^30, 2^24] | 256 | 22 | 22.00 | 454.41 | 352.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 2 | [2^29, 2^20] | 256 | 15 | 15.00 | 319.31 | 120.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 3 | [2^28, 2^17] | 256 | 12 | 12.00 | 247.28 | 96.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 4 | [2^27, 2^14] | 256 | 10 | 10.00 | 198.84 | 80.00 |
| 3 | canopy_schwartz_zippel | Schwartz-Zippel combined aligned claim recursive oracle 5 | [2^26, 2^11] | 256 | 9 | 9.00 | 163.10 | 144.00 |
| 3 | canopy_schwartz_zippel | outer first oracle via Canopy | [2^31, 2^30] | 256 | 138 | 138.00 | 138.00 | 1104.00 |
| 4 | canopy_field_switch_192 | GF(2^192) recursive oracle 1 | [2^30, 2^24] | 192 | 22 | 22.00 | 453.59 | 352.00 |
| 4 | canopy_field_switch_192 | GF(2^192) recursive oracle 2 | [2^29, 2^20] | 192 | 15 | 15.00 | 304.20 | 240.00 |
| 4 | canopy_field_switch_192 | GF(2^192) recursive oracle 3 | [2^28, 2^16] | 192 | 11 | 11.00 | 217.68 | 176.00 |
| 4 | canopy_field_switch_192 | GF(2^192) recursive oracle 4 | [2^27, 2^12] | 192 | 9 | 9.00 | 170.45 | 144.00 |
| 4 | canopy_field_switch_192 | field-switch source: two GF(2^256) roots / eight GF(2^64) rows | [2^31, 2^27] | 256 | 33 | 33.00 | 1457.90 | 528.00 |
| 4 | canopy_field_switch_192 | outer first oracle via Canopy | [2^31, 2^30] | 256 | 138 | 138.00 | 138.00 | 1104.00 |
