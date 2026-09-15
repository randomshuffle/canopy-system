# Bolt-max initial interleaving grid

Initial widths: [4, 8, 16]; input sizes are field-element counts. Subsequent WHIR folds remain factor 8 (three variables). Each width and input size gets an independent exhaustive Canopy height search for X and Y. Both schemes use the same width within each comparison.

256-bit prime field, BLAKE3, Bolt-max degree 8 / compression 8, 128-bit per-event target conditional on prime-field LDPC distance 0.013. Canopy RS block lengths: systematic k/2 and parity k/2; total k, 80% of Bolt's encoded rows. Heights start at 2 for systematic and 0 for parity. Here k=N/width. Public-H constrained WHIR adaptation; Spark/Mulperm preprocessing excluded.

100 deterministic size samples per height; sizes include commitment roots and omit serializer framing. Positive savings mean Canopy is smaller.

| log2 input | width | h X / Y | Merkle MiB | Canopy MiB | saved MiB | saved % | auth saved % |
|---:|---:|:---:|---:|---:|---:|---:|---:|
| 18 | 4 | 16 / 14 | 3.889993 | 3.890054 | -0.000061 | -0.002 | -0.007 |
| 18 | 8 | 15 / 13 | 5.895351 | 5.895412 | -0.000061 | -0.001 | -0.018 |
| 18 | 16 | 14 / 12 | 8.572146 | 8.572207 | -0.000061 | -0.001 | -0.214 |
| 22 | 4 | 2 / 18 | 6.458806 | 4.881348 | 1.577458 | 24.423 | 50.711 |
| 22 | 8 | 2 / 17 | 8.329189 | 7.282004 | 1.047186 | 12.572 | 41.810 |
| 22 | 16 | 2 / 16 | 12.749962 | 12.186451 | 0.563511 | 4.420 | 29.407 |
| 26 | 4 | 2 / 22 | 9.409323 | 5.488057 | 3.921265 | 41.674 | 69.936 |
| 26 | 8 | 2 / 21 | 11.136554 | 7.815800 | 3.320753 | 29.819 | 66.716 |
| 26 | 16 | 2 / 20 | 15.507876 | 12.774793 | 2.733083 | 17.624 | 62.840 |
| 30 | 4 | 2 / 26 | 12.360618 | 6.029576 | 6.331043 | 51.219 | 77.820 |
| 30 | 8 | 2 / 25 | 14.178936 | 8.454864 | 5.724072 | 40.370 | 76.305 |
| 30 | 16 | 2 / 24 | 18.411580 | 13.292554 | 5.119026 | 27.803 | 74.522 |

Largest percentage saving: width 4, input 2^30, 51.219%. Largest byte saving: width 4, input 2^30, 6.331043 MiB.

## Smallest proofs within each input size

| log2 input | best Merkle width | Merkle MiB | best Canopy width | Canopy MiB | independently optimized saving % |
|---:|---:|---:|---:|---:|---:|
| 18 | 4 | 3.889993 | 4 | 3.890054 | -0.002 |
| 22 | 4 | 6.458806 | 4 | 4.881348 | 24.423 |
| 26 | 4 | 9.409323 | 4 | 5.488057 | 41.674 |
| 30 | 4 | 12.360618 | 4 | 6.029576 | 51.219 |

## Executable timings at input 2^18

2 warmups and 10 measured repetitions per scheme and width, 8 Rayon threads. Every proof verified; encoding included, setup and input generation excluded. Additional timings at `2^22` are in the [detailed prover profile](prover_profile.md) and [combined block-length comparison](../bolt_max_canopy_block_lengths/comparison.md); `2^26` and `2^30` have size estimates only.

| width | scheme | commitment median s | opening median s | total prover median s | verify median s | actual proof MiB |
|---:|---|---:|---:|---:|---:|---:|
| 4 | bolt_max_merkle | 0.015812 | 0.070297 | 0.086484 | 0.070179 | 3.890205 |
| 4 | bolt_max_canopy_equal_half | 0.016010 | 0.070320 | 0.086368 | 0.070323 | 3.889046 |
| 8 | bolt_max_merkle | 0.013859 | 0.072204 | 0.086193 | 0.088185 | 5.893288 |
| 8 | bolt_max_canopy_equal_half | 0.013814 | 0.071734 | 0.085236 | 0.087548 | 5.897102 |
| 16 | bolt_max_merkle | 0.013507 | 0.078367 | 0.093539 | 0.102083 | 8.571953 |
| 16 | bolt_max_canopy_equal_half | 0.013773 | 0.078597 | 0.093481 | 0.104084 | 8.572014 |

Each width subdirectory contains all height candidates, detailed proof components, per-WHIR-oracle costs, and raw timings. The earlier width-128 experiment is preserved separately.
