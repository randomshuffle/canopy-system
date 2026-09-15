# Bolt-max initial interleaving grid

Initial widths: [4, 8, 16]; input sizes are field-element counts. Subsequent WHIR folds remain factor 8 (three variables). Each width and input size gets an independent exhaustive Canopy height search for X and Y. Both schemes use the same width within each comparison.

256-bit prime field, BLAKE3, Bolt-max degree 8 / compression 8, 128-bit per-event target conditional on prime-field LDPC distance 0.013. Canopy RS block lengths: systematic k/4 and parity k/4; total k/2, 40% of Bolt's encoded rows. Heights start at 3 for systematic and 1 for parity. Here k=N/width. Public-H constrained WHIR adaptation; Spark/Mulperm preprocessing excluded.

100 deterministic size samples per height; sizes include commitment roots and omit serializer framing. Positive savings mean Canopy is smaller.

| log2 input | width | h X / Y | Merkle MiB | Canopy MiB | saved MiB | saved % | auth saved % |
|---:|---:|:---:|---:|---:|---:|---:|---:|
| 18 | 4 | 16 / 14 | 3.889993 | 3.890054 | -0.000061 | -0.002 | -0.007 |
| 18 | 8 | 15 / 13 | 5.895351 | 5.895412 | -0.000061 | -0.001 | -0.018 |
| 18 | 16 | 14 / 12 | 8.572146 | 8.572207 | -0.000061 | -0.001 | -0.214 |
| 22 | 4 | 3 / 18 | 6.458806 | 5.416003 | 1.042803 | 16.145 | 33.523 |
| 22 | 8 | 3 / 17 | 8.329189 | 7.768090 | 0.561100 | 6.737 | 22.402 |
| 22 | 16 | 3 / 16 | 12.749962 | 12.571994 | 0.177968 | 1.396 | 9.287 |
| 26 | 4 | 3 / 22 | 9.409323 | 6.090703 | 3.318620 | 35.269 | 59.188 |
| 26 | 8 | 3 / 21 | 11.136554 | 8.405315 | 2.731238 | 24.525 | 54.873 |
| 26 | 16 | 3 / 20 | 15.507876 | 13.365482 | 2.142394 | 13.815 | 49.259 |
| 30 | 4 | 3 / 26 | 12.360618 | 6.635433 | 5.725185 | 46.318 | 70.373 |
| 30 | 8 | 3 / 25 | 14.178936 | 9.060604 | 5.118332 | 36.098 | 68.231 |
| 30 | 16 | 3 / 24 | 18.411580 | 13.888279 | 4.523300 | 24.568 | 65.849 |

Largest percentage saving: width 4, input 2^30, 46.318%. Largest byte saving: width 4, input 2^30, 5.725185 MiB.

## Smallest proofs within each input size

| log2 input | best Merkle width | Merkle MiB | best Canopy width | Canopy MiB | independently optimized saving % |
|---:|---:|---:|---:|---:|---:|
| 18 | 4 | 3.889993 | 4 | 3.890054 | -0.002 |
| 22 | 4 | 6.458806 | 4 | 5.416003 | 16.145 |
| 26 | 4 | 9.409323 | 4 | 6.090703 | 35.269 |
| 30 | 4 | 12.360618 | 4 | 6.635433 | 46.318 |

## Executable timings at input 2^18

2 warmups and 10 measured repetitions per scheme and width, 8 Rayon threads. Every proof verified; encoding included, setup and input generation excluded. Additional timings at `2^22` are in the [combined comparison](comparison.md) and [detailed prover profile](prover_profile.md); `2^26` and `2^30` have size estimates only.

| width | scheme | commitment median s | opening median s | total prover median s | verify median s | actual proof MiB |
|---:|---|---:|---:|---:|---:|---:|
| 4 | bolt_max_merkle | 0.016515 | 0.069122 | 0.085725 | 0.069490 | 3.890205 |
| 4 | bolt_max_canopy_equal_blocks | 0.016776 | 0.069290 | 0.086014 | 0.069833 | 3.888191 |
| 8 | bolt_max_merkle | 0.013288 | 0.073404 | 0.087696 | 0.087447 | 5.893288 |
| 8 | bolt_max_canopy_equal_blocks | 0.013939 | 0.074066 | 0.087506 | 0.087556 | 5.895546 |
| 16 | bolt_max_merkle | 0.011957 | 0.073729 | 0.085948 | 0.098066 | 8.571953 |
| 16 | bolt_max_canopy_equal_blocks | 0.011942 | 0.074358 | 0.086742 | 0.098526 | 8.572472 |

Each width subdirectory contains all height candidates, detailed proof components, per-WHIR-oracle costs, and raw timings. The earlier width-128 experiment is preserved separately.
