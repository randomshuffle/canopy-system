# Bolt-max initial interleaving grid

Initial widths: [4, 8, 16]; input sizes are field-element counts. Subsequent WHIR folds remain factor 8 (three variables). Each width and input size gets an independent exhaustive Canopy height search for X and Y. Both schemes use the same width within each comparison.

256-bit prime field, BLAKE3, Bolt-max degree 8 / compression 8, 128-bit per-event target conditional on prime-field LDPC distance 0.013. Canopy RS block lengths: systematic k and parity k; total 2k, 160% of Bolt's encoded rows. Heights start at 1 for systematic and 0 for parity. Here k=N/width. Public-H constrained WHIR adaptation; Spark/Mulperm preprocessing excluded.

100 deterministic size samples per height; sizes include commitment roots and omit serializer framing. Positive savings mean Canopy is smaller.

| log2 input | width | h X / Y | Merkle MiB | Canopy MiB | saved MiB | saved % | auth saved % |
|---:|---:|:---:|---:|---:|---:|---:|---:|
| 18 | 4 | 1 / 14 | 3.889993 | 3.730012 | 0.159981 | 4.113 | 19.414 |
| 18 | 8 | 15 / 13 | 5.895351 | 5.895412 | -0.000061 | -0.001 | -0.018 |
| 18 | 16 | 14 / 12 | 8.572146 | 8.572207 | -0.000061 | -0.001 | -0.214 |
| 22 | 4 | 1 / 18 | 6.458806 | 4.304075 | 2.154731 | 33.361 | 69.269 |
| 22 | 8 | 1 / 17 | 8.329189 | 6.742484 | 1.586705 | 19.050 | 63.351 |
| 22 | 16 | 1 / 16 | 12.749962 | 11.692677 | 1.057285 | 8.292 | 55.174 |
| 26 | 4 | 1 / 22 | 9.409323 | 4.883449 | 4.525873 | 48.100 | 80.720 |
| 26 | 8 | 1 / 21 | 11.136554 | 7.212998 | 3.923555 | 35.231 | 78.827 |
| 26 | 16 | 1 / 20 | 15.507876 | 12.184808 | 3.323068 | 21.428 | 76.405 |
| 30 | 4 | 1 / 26 | 12.360618 | 5.433590 | 6.927028 | 56.041 | 85.146 |
| 30 | 8 | 1 / 25 | 14.178936 | 7.849236 | 6.329699 | 44.642 | 84.379 |
| 30 | 16 | 1 / 24 | 18.411580 | 12.686980 | 5.724599 | 31.092 | 83.337 |

Largest percentage saving: width 4, input 2^30, 56.041%. Largest byte saving: width 4, input 2^30, 6.927028 MiB.

## Smallest proofs within each input size

| log2 input | best Merkle width | Merkle MiB | best Canopy width | Canopy MiB | independently optimized saving % |
|---:|---:|---:|---:|---:|---:|
| 18 | 4 | 3.889993 | 4 | 3.730012 | 4.113 |
| 22 | 4 | 6.458806 | 4 | 4.304075 | 33.361 |
| 26 | 4 | 9.409323 | 4 | 4.883449 | 48.100 |
| 30 | 4 | 12.360618 | 4 | 5.433590 | 56.041 |

## Executable timings at input 2^18

2 warmups and 10 measured repetitions per scheme and width, 8 Rayon threads. Every proof verified; encoding included, setup and input generation excluded. Additional timings at `2^22` are in the [detailed prover profile](prover_profile.md) and [combined block-length comparison](../bolt_max_canopy_block_lengths/comparison.md); `2^26` and `2^30` have size estimates only.

| width | scheme | commitment median s | opening median s | total prover median s | verify median s | actual proof MiB |
|---:|---|---:|---:|---:|---:|---:|
| 4 | bolt_max_merkle | 0.017387 | 0.072147 | 0.089645 | 0.070747 | 3.890205 |
| 4 | bolt_max_canopy_equal_full | 0.024765 | 0.078551 | 0.103800 | 0.061298 | 3.730087 |
| 8 | bolt_max_merkle | 0.013850 | 0.073719 | 0.087530 | 0.088389 | 5.893288 |
| 8 | bolt_max_canopy_equal_full | 0.013821 | 0.073855 | 0.087317 | 0.087998 | 5.894478 |
| 16 | bolt_max_merkle | 0.012970 | 0.077371 | 0.090263 | 0.101569 | 8.571953 |
| 16 | bolt_max_canopy_equal_full | 0.012998 | 0.078051 | 0.090926 | 0.101526 | 8.573387 |

Each width subdirectory contains all height candidates, detailed proof components, per-WHIR-oracle costs, and raw timings. The earlier width-128 experiment is preserved separately.
