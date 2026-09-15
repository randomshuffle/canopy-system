# Bolt-max initial interleaving grid

Initial widths: [4, 8, 16]; input sizes are field-element counts. Subsequent WHIR folds remain factor 8 (three variables). Each width and input size gets an independent exhaustive Canopy height search for X and Y. Both schemes use the same width within each comparison.

256-bit prime field, BLAKE3, Bolt-max degree 8 / compression 8, 128-bit per-event target conditional on prime-field LDPC distance 0.013. Separate Canopy RS lengths are k/4 and k/16 where k=N/width, totaling exactly one quarter of Bolt's encoded rows. Public-H constrained WHIR adaptation; Spark/Mulperm preprocessing excluded.

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

1 warmups and 5 measured repetitions per scheme and width, 8 Rayon threads. Every proof verified; encoding included, setup and input generation excluded. The table below measures 2^18; additional 2^22 measurements are linked below. Inputs 2^26 and 2^30 have size estimates only.

| width | scheme | commitment median s | opening median s | total prover median s | verify median s | actual proof MiB |
|---:|---|---:|---:|---:|---:|---:|
| 4 | bolt_max_merkle | 0.020445 | 0.070763 | 0.090169 | 0.069053 | 3.890205 |
| 4 | bolt_max_canopy_quarter | 0.016848 | 0.069993 | 0.086841 | 0.069213 | 3.894875 |
| 8 | bolt_max_merkle | 0.013980 | 0.072346 | 0.086565 | 0.086823 | 5.893288 |
| 8 | bolt_max_canopy_quarter | 0.013355 | 0.072076 | 0.085265 | 0.086943 | 5.895851 |
| 16 | bolt_max_merkle | 0.013356 | 0.076450 | 0.088887 | 0.099381 | 8.571953 |
| 16 | bolt_max_canopy_quarter | 0.012125 | 0.076857 | 0.089134 | 0.099952 | 8.572655 |

Each width subdirectory contains all height candidates, detailed proof components, per-WHIR-oracle costs, and raw timings. The earlier width-128 experiment is preserved separately.

## Additional timings at input 2^22

See the [combined 2^22 timing report](timing_n22/summary.md), including prover, commitment, verification, and actual proof sizes for widths 4, 8, and 16. All 30 additional measured proofs verified.
