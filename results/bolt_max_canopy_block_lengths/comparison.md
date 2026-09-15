# Bolt-max: Canopy RS block-length sweep

Both Canopy RS blocks have length k/4, k/2, or k, where k=N/width is the number of systematic rows. Bolt's sketch matrix always has k/4 rows. Widths are 4, 8, and 16. Each layout independently minimizes proof size over both heights; executable timings use those selected heights.

All variants use the same 256-bit prime field, BLAKE3, degree-8/compression-8 Bolt-max encoder, and common constrained WHIR proofs. The 128-bit per-event target is conditional on prime-field LDPC distance 0.013. Systematic/sketch query budgets remain min(k,20431) and 309. The public-H adaptation excludes Spark/Mulperm preprocessing.

Checked 972 new height candidates across 24 new configurations, plus 456 existing k/4 candidates. All 72 disjoint proof-size totals reconcile. Sizes use 100 deterministic samples per height, include commitment roots, and omit serializer framing. The Merkle baseline, common proofs, row values, and sampling seeds are identical across layouts.

## Full proof sizes

MiB = 2^20 bytes. Savings are relative to Bolt + Merkle at the same input size and width.

| Input | Width | Merkle MiB | k/4 MiB | k/2 MiB | k MiB | k/2 saved % | k saved % |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 2^18 | 4 | 3.889993 | 3.890054 | 3.890054 | 3.730012 | -0.002 | 4.113 |
| 2^18 | 8 | 5.895351 | 5.895412 | 5.895412 | 5.895412 | -0.001 | -0.001 |
| 2^18 | 16 | 8.572146 | 8.572207 | 8.572207 | 8.572207 | -0.001 | -0.001 |
| 2^22 | 4 | 6.458806 | 5.416003 | 4.881348 | 4.304075 | 24.423 | 33.361 |
| 2^22 | 8 | 8.329189 | 7.768090 | 7.282004 | 6.742484 | 12.572 | 19.050 |
| 2^22 | 16 | 12.749962 | 12.571994 | 12.186451 | 11.692677 | 4.420 | 8.292 |
| 2^26 | 4 | 9.409323 | 6.090703 | 5.488057 | 4.883449 | 41.674 | 48.100 |
| 2^26 | 8 | 11.136554 | 8.405315 | 7.815800 | 7.212998 | 29.819 | 35.231 |
| 2^26 | 16 | 15.507876 | 13.365482 | 12.774793 | 12.184808 | 17.624 | 21.428 |
| 2^30 | 4 | 12.360618 | 6.635433 | 6.029576 | 5.433590 | 51.219 | 56.041 |
| 2^30 | 8 | 14.178936 | 9.060604 | 8.454864 | 7.849236 | 40.370 | 44.642 |
| 2^30 | 16 | 18.411580 | 13.888279 | 13.292554 | 12.686980 | 27.803 | 31.092 |

Length k yields the smallest modeled proof in every configuration, allowing ties. At input 2^30, width 4, savings versus Merkle increase from 46.32% (k/4) to 51.22% (k/2) to 56.04% (k).

For every input at least 2^22, the systematic height is 3, 2, or 1 as the block grows from k/4 to k/2 to k. The selected systematic RS rate stays 1/2, while its message length grows from k/8 to k/4 to k/2. This reduces bottom authentication paths and increases H2F, RS commitment, and WHIR proving work.

The sketch layer chooses its full-height constant endpoint in all configurations, including after testing height zero in the enlarged layouts. Its block enlargement adds only equal-child hash steps to commitment construction, without a materialized RS array or a WHIR SEP proof.


## Selected heights

Height counts binary levels above whole row hashes. At height zero the Canopy message contains one H2F value per row, with no bottom authentication paths. At full height the message is one field element; its constant RS commitment requires no FFT or WHIR SEP proof.

| Input | Width | k/4 h_X / h_Y | k/2 h_X / h_Y | k h_X / h_Y |
|---:|---:|:---:|:---:|:---:|
| 2^18 | 4 | 16 / 14 | 16 / 14 | 1 / 14 |
| 2^18 | 8 | 15 / 13 | 15 / 13 | 15 / 13 |
| 2^18 | 16 | 14 / 12 | 14 / 12 | 14 / 12 |
| 2^22 | 4 | 3 / 18 | 2 / 18 | 1 / 18 |
| 2^22 | 8 | 3 / 17 | 2 / 17 | 1 / 17 |
| 2^22 | 16 | 3 / 16 | 2 / 16 | 1 / 16 |
| 2^26 | 4 | 3 / 22 | 2 / 22 | 1 / 22 |
| 2^26 | 8 | 3 / 21 | 2 / 21 | 1 / 21 |
| 2^26 | 16 | 3 / 20 | 2 / 20 | 1 / 20 |
| 2^30 | 4 | 3 / 26 | 2 / 26 | 1 / 26 |
| 2^30 | 8 | 3 / 25 | 2 / 25 | 1 / 25 |
| 2^30 | 16 | 3 / 24 | 2 / 24 | 1 / 24 |

## Total prover time

The two enlarged layouts have 240 freshly measured and verified proofs in total. Each configuration uses eight Rayon threads, two warmups, and ten measured repetitions per scheme; Merkle/Canopy execution order alternates. Layouts execute sequentially. The k/4 measurements are the earlier run (120 verified proofs), retained for context. Compare each Canopy time against its accompanying Merkle baseline; raw cross-run changes can include machine variation.

Median total times include encoding, commitment construction, and opening. Setup, input generation, verification, and cleanup outside the established timing boundaries are excluded. Input sizes 2^26 and 2^30 are estimated, not timed.

| Input | Width | Each block | Merkle median ms | Canopy median ms | Median overhead % |
|---:|---:|---|---:|---:|---:|
| 2^18 | 4 | k/4 | 85.725 | 86.014 | +0.34 |
| 2^18 | 4 | k/2 | 86.484 | 86.368 | -0.13 |
| 2^18 | 4 | k | 89.645 | 103.800 | +15.79 |
| 2^18 | 8 | k/4 | 87.696 | 87.506 | -0.22 |
| 2^18 | 8 | k/2 | 86.193 | 85.236 | -1.11 |
| 2^18 | 8 | k | 87.530 | 87.317 | -0.24 |
| 2^18 | 16 | k/4 | 85.948 | 86.742 | +0.92 |
| 2^18 | 16 | k/2 | 93.539 | 93.481 | -0.06 |
| 2^18 | 16 | k | 90.263 | 90.926 | +0.73 |
| 2^22 | 4 | k/4 | 976.739 | 1027.973 | +5.25 |
| 2^22 | 4 | k/2 | 1003.884 | 1116.004 | +11.17 |
| 2^22 | 4 | k | 1046.714 | 1271.845 | +21.51 |
| 2^22 | 8 | k/4 | 648.923 | 677.614 | +4.42 |
| 2^22 | 8 | k/2 | 685.919 | 732.183 | +6.74 |
| 2^22 | 8 | k | 698.818 | 811.479 | +16.12 |
| 2^22 | 16 | k/4 | 593.683 | 607.721 | +2.36 |
| 2^22 | 16 | k/2 | 602.992 | 636.205 | +5.51 |
| 2^22 | 16 | k | 643.311 | 695.762 | +8.15 |

## Commitment and opening breakdown

Arithmetic mean milliseconds, so commitment plus opening adds to the mean total. Detailed profiles below include all disjoint stages and observed variation.

| Input | Width | Each block | Merkle commit | Canopy commit | Merkle opening | Canopy opening | Merkle total | Canopy total |
|---:|---:|---|---:|---:|---:|---:|---:|---:|
| 2^18 | 4 | k/4 | 19.174 | 16.468 | 73.966 | 69.995 | 93.141 | 86.463 |
| 2^18 | 4 | k/2 | 16.226 | 16.077 | 70.611 | 70.166 | 86.837 | 86.243 |
| 2^18 | 4 | k | 21.588 | 25.589 | 80.950 | 79.760 | 102.537 | 105.349 |
| 2^18 | 8 | k/4 | 15.967 | 15.139 | 73.384 | 75.697 | 89.351 | 90.836 |
| 2^18 | 8 | k/2 | 14.649 | 13.809 | 72.308 | 72.590 | 86.957 | 86.399 |
| 2^18 | 8 | k | 13.905 | 14.026 | 73.798 | 73.572 | 87.703 | 87.597 |
| 2^18 | 16 | k/4 | 12.110 | 12.458 | 73.478 | 74.843 | 85.588 | 87.301 |
| 2^18 | 16 | k/2 | 13.686 | 14.319 | 79.094 | 78.977 | 92.781 | 93.295 |
| 2^18 | 16 | k | 17.185 | 13.007 | 78.814 | 78.027 | 95.999 | 91.034 |
| 2^22 | 4 | k/4 | 315.003 | 352.532 | 662.011 | 697.595 | 977.014 | 1050.127 |
| 2^22 | 4 | k/2 | 339.885 | 399.776 | 766.216 | 720.129 | 1106.102 | 1119.904 |
| 2^22 | 4 | k | 337.673 | 478.509 | 734.167 | 804.375 | 1071.840 | 1282.884 |
| 2^22 | 8 | k/4 | 263.722 | 281.675 | 387.428 | 396.441 | 651.150 | 678.116 |
| 2^22 | 8 | k/2 | 292.494 | 337.939 | 416.235 | 465.168 | 708.729 | 803.106 |
| 2^22 | 8 | k | 292.463 | 361.445 | 416.482 | 452.793 | 708.945 | 814.237 |
| 2^22 | 16 | k/4 | 235.297 | 241.453 | 363.240 | 370.212 | 598.537 | 611.665 |
| 2^22 | 16 | k/2 | 237.111 | 257.951 | 368.793 | 383.347 | 605.904 | 641.298 |
| 2^22 | 16 | k | 289.694 | 321.153 | 384.437 | 421.703 | 674.131 | 742.857 |

## Data and reproduction

- **k/4 each:** [stage profile](../bolt_max_canopy_equal_blocks/prover_profile.md), [proof sizes](../bolt_max_canopy_equal_blocks/proof_sizes.csv), exhaustive height searches and proof components in each `width_*` directory.
- **k/2 each:** [stage profile](../bolt_max_canopy_equal_half/prover_profile.md), [proof sizes](../bolt_max_canopy_equal_half/proof_sizes.csv), exhaustive height searches and proof components in each `width_*` directory.
- **k each:** [stage profile](../bolt_max_canopy_equal_full/prover_profile.md), [proof sizes](../bolt_max_canopy_equal_full/proof_sizes.csv), exhaustive height searches and proof components in each `width_*` directory.

The local [experiment notes](../../docs/experiment_bolt_max.md) contain reproduction commands and soundness assumptions. Rebuild this report with `python3 scripts/compare_bolt_block_lengths.py`.
