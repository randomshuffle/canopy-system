# Bolt-max width grid: timings at input 2^22

256-bit field, BLAKE3, 8 Rayon threads. One warmup and five measured repetitions per scheme and width; all 30 measured proofs verified. Each width uses its proof-size-optimal Canopy heights. Setup and random input generation are excluded; commitment includes encoding. Public-H verification, with Spark/Mulperm preprocessing excluded. The 128-bit per-event target remains conditional on prime-field LDPC distance 0.013.

| width | h X / Y | Merkle prover s | Canopy prover s | Canopy time change % | Merkle commit s | Canopy commit s | Merkle verify s | Canopy verify s | actual Merkle MiB | actual Canopy MiB |
|---:|:---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 4 | 3 / 18 | 1.001551 | 1.052741 | +5.111 | 0.320615 | 0.353287 | 0.243355 | 0.223608 | 6.460320 | 5.414429 |
| 8 | 3 / 17 | 0.664531 | 0.698453 | +5.105 | 0.274872 | 0.294680 | 0.162028 | 0.148208 | 8.324883 | 7.769562 |
| 16 | 3 / 16 | 0.599250 | 0.613633 | +2.400 | 0.235106 | 0.242088 | 0.177718 | 0.168987 | 12.752983 | 12.564270 |

Positive time change means Canopy is slower. These are local CPU measurements with five repetitions, not a speed guarantee. Serialized proof sizes differ slightly from size-model means because the transcript selects different queries and adds framing. Input sizes 2^26 and 2^30 were estimated, not timed. Raw measurements and separate commitment/opening times are in each width subdirectory.
