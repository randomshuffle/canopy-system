# Equal Canopy block lengths: comparison with Bolt + Merkle

Both Canopy RS codewords now have length **k/4**, with k=N/width. Their total is k/2 (40% of Bolt's encoded rows). The systematic rate is 2^(2-h_X); the parity rate is 2^(-h_Y). The search includes h_X=3..log2(k) and h_Y=1..log2(k/4), independently minimizing authentication proof size. Parameters remain the 256-bit prime field, BLAKE3, Bolt-max, and the 128-bit per-event target conditional on LDPC distance 0.013. Common constrained WHIR proofs use the public-H adaptation, excluding Spark/Mulperm preprocessing.

Validated all 12 input/width configurations, 456 height candidates, and 24 disjoint proof-size totals. There are 100 deterministic samples per height; full proof sizes include commitments and omit serializer framing. The Merkle baseline, common proof costs, row query counts, and seeds match the previous layout exactly.

**Result: the enlarged parity block changes neither the selected heights nor the modeled proof sizes in this grid.** Parity still selects the full-height constant endpoint in every case. A length-k/4 constant codeword has the same proof payload as a length-k/16 constant codeword: one root and one field coefficient. Its commitment construction adds two equal-child hash steps; it does not materialize a larger array or run an RS FFT/WHIR proof. The systematic layer is unchanged.

## Full proof-size comparison

| Input | Width | h X / Y | Merkle MiB | Canopy MiB | Saved vs Merkle % | Additional saving vs old layout KiB |
|---:|---:|:---:|---:|---:|---:|---:|
| 2^18 | 4 | 16 / 14 | 3.889993 | 3.890054 | -0.002 | 0.000000 |
| 2^18 | 8 | 15 / 13 | 5.895351 | 5.895412 | -0.001 | 0.000000 |
| 2^18 | 16 | 14 / 12 | 8.572146 | 8.572207 | -0.001 | 0.000000 |
| 2^22 | 4 | 3 / 18 | 6.458806 | 5.416003 | 16.145 | 0.000000 |
| 2^22 | 8 | 3 / 17 | 8.329189 | 7.768090 | 6.737 | 0.000000 |
| 2^22 | 16 | 3 / 16 | 12.749962 | 12.571994 | 1.396 | 0.000000 |
| 2^26 | 4 | 3 / 22 | 9.409323 | 6.090703 | 35.269 | 0.000000 |
| 2^26 | 8 | 3 / 21 | 11.136554 | 8.405315 | 24.525 | 0.000000 |
| 2^26 | 16 | 3 / 20 | 15.507876 | 13.365482 | 13.815 | 0.000000 |
| 2^30 | 4 | 3 / 26 | 12.360618 | 6.635433 | 46.318 | 0.000000 |
| 2^30 | 8 | 3 / 25 | 14.178936 | 9.060604 | 36.098 | 0.000000 |
| 2^30 | 16 | 3 / 24 | 18.411580 | 13.888279 | 24.568 | 0.000000 |

## Executable prover time

Two warmups and ten measured repetitions per scheme and configuration, 8 Rayon threads. All 120 measured proofs verified. Times below are fresh arithmetic means including encoding, commitment construction, and opening. Setup and verification are excluded. The [stage profile](prover_profile.md) includes median totals, paired variation, and individual stages. Small differences from prior timing runs are not evidence of a geometry-induced speedup.

| Input | Width | Merkle commit ms | Canopy commit ms | Merkle opening ms | Canopy opening ms | Merkle total ms | Canopy total ms | Overhead % |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 2^18 | 4 | 19.174 | 16.468 | 73.966 | 69.995 | 93.141 | 86.463 | -7.17 |
| 2^18 | 8 | 15.967 | 15.139 | 73.384 | 75.697 | 89.351 | 90.836 | +1.66 |
| 2^18 | 16 | 12.110 | 12.458 | 73.478 | 74.843 | 85.588 | 87.301 | +2.00 |
| 2^22 | 4 | 315.003 | 352.532 | 662.011 | 697.595 | 977.014 | 1050.127 | +7.48 |
| 2^22 | 8 | 263.722 | 281.675 | 387.428 | 396.441 | 651.150 | 678.116 | +4.14 |
| 2^22 | 16 | 235.297 | 241.453 | 363.240 | 370.212 | 598.537 | 611.665 | +2.19 |

## Closest nonconstant parity choices

This shows that nonconstant parity layers were evaluated, including the newly valid heights 1 and 2. Sizes here include only parity authentication, its commitment root, and Canopy proof; row values and common Bolt proofs are excluded.

| Input | Width | Selected h | Selected parity KiB | Best nonconstant h | Nonconstant parity KiB |
|---:|---:|---:|---:|---:|---:|
| 2^18 | 4 | 14 | 47.355 | 13 | 53.586 |
| 2^18 | 8 | 13 | 38.163 | 12 | 44.199 |
| 2^18 | 16 | 12 | 29.180 | 11 | 34.958 |
| 2^22 | 4 | 18 | 85.517 | 17 | 92.672 |
| 2^22 | 8 | 17 | 75.928 | 16 | 82.946 |
| 2^22 | 16 | 16 | 66.283 | 15 | 73.105 |
| 2^26 | 4 | 22 | 124.068 | 21 | 131.641 |
| 2^26 | 8 | 21 | 114.377 | 20 | 121.578 |
| 2^26 | 16 | 20 | 104.924 | 19 | 112.186 |
| 2^30 | 4 | 26 | 162.581 | 9 | 168.569 |
| 2^30 | 8 | 25 | 153.113 | 24 | 160.665 |
| 2^30 | 16 | 24 | 143.362 | 23 | 151.168 |

Each width subdirectory contains the exhaustive height search and full proof breakdown. Input sizes 2^26 and 2^30 were estimated, not timed. Reproduce using the equal-block commands in `docs/experiment_bolt_max.md`, then `python3 scripts/compare_bolt_canopy_layouts.py`.
