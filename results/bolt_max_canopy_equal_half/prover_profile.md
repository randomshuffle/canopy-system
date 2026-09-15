# Bolt-max prover time breakdown

Both variants use the same prime-field Bolt-max encoder and common constrained WHIR proofs. Canopy changes the two initial row commitments and adds its own RS/WHIR SEP layer. Parameters: 256-bit field, BLAKE3, initial widths 4/8/16, subsequent WHIR factor 8, Canopy block lengths k/2 for X and k/2 for Y, proof-size-optimal heights. The 128-bit per-event target is conditional on prime-field LDPC distance 0.013; this is the public-H adaptation and excludes Spark/Mulperm preprocessing.

Fresh profiles use two warmups and ten measured repetitions for each scheme, width, and input size on 8 Rayon threads. All 120 measured proofs verified. Execution order alternates within each width. Raw data and per-part stage tables are linked below. Earlier timing files are preserved.

Component tables use **arithmetic mean milliseconds**, so they add to the mean commitment/opening/total times. Median totals and observed variation are shown separately. Stage clocks are disjoint wall-clock intervals; parallel-worker CPU time is not summed. Shared stages execute the same algorithms; measured differences may reflect cache, allocation, and run-to-run variation. These local measurements do not establish speedups at unmeasured input sizes.

## Total comparison

| Input | Width | Merkle mean ms | Canopy mean ms | Mean overhead | Merkle median ms | Canopy median ms | Paired delta mean ± sample SD ms |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 2^18 | 4 | 86.837 | 86.243 | -0.68% | 86.484 | 86.368 | -0.595 ± 2.249 |
| 2^18 | 8 | 86.957 | 86.399 | -0.64% | 86.193 | 85.236 | -0.558 ± 3.332 |
| 2^18 | 16 | 92.781 | 93.295 | +0.55% | 93.539 | 93.481 | +0.515 ± 6.351 |
| 2^22 | 4 | 1106.102 | 1119.904 | +1.25% | 1003.884 | 1116.004 | +13.803 ± 300.289 |
| 2^22 | 8 | 708.729 | 803.106 | +13.32% | 685.919 | 732.183 | +94.377 ± 171.826 |
| 2^22 | 16 | 605.904 | 641.298 | +5.84% | 602.992 | 636.205 | +35.394 ± 16.487 |

The paired spread is the sample standard deviation of Canopy-minus-Merkle times for matching repetition indices, not a confidence interval. Each configuration uses its independently selected heights, with message sizes listed below. Full-height constant endpoints add very little cryptographic work; small differences in their total time should not be interpreted as intrinsic speed benefits.


## Input 2^18, width 4

[Per-part stage report](width_4/prover_stage_timings.md) · [Raw stages](width_4/prover_stage_timings.csv) · [Raw totals](width_4/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **262,144** original elements, with a parity RS message of 32,768. The common WHIR initial RS messages have 65,536 and 8,192 elements. Canopy's RS messages have **1** (X) and **1** (Y), with fixed codeword lengths 32,768 and 32,768. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 3.564 | 3.612 | +0.048 |
| commitment | Bolt parity RS encoding | 3.616 | 3.557 | -0.059 |
| commitment | X/Y Merkle trees or bottom forests | 9.045 | 8.899 | -0.146 |
| commitment | Canopy hash-to-field | 0.000 | 0.001 | +0.001 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 0.007 | +0.007 |
| commitment | Root binding and residual | 0.002 | 0.001 | -0.001 |
| **commitment** | **Subtotal** | **16.226** | **16.077** | **-0.149** |
| opening | Bolt partial evaluation g | 6.150 | 6.150 | +0.000 |
| opening | Bolt column projections | 1.353 | 1.332 | -0.021 |
| opening | Common WHIR initial RS commitments | 8.675 | 8.369 | -0.306 |
| opening | Syndrome evaluation and query sampling | 6.399 | 6.423 | +0.025 |
| opening | Copy and serialize queried row values | 15.066 | 15.356 | +0.290 |
| opening | Collect and serialize authentication paths | 4.265 | 4.207 | -0.057 |
| opening | Project and batch queried rows | 2.867 | 2.789 | -0.077 |
| opening | Parity query covector / transpose RS FFT | 1.592 | 1.570 | -0.022 |
| opening | Public H transpose covector | 3.232 | 3.184 | -0.049 |
| opening | Common WHIR constrained proofs | 20.949 | 20.705 | -0.245 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 0.020 | +0.020 |
| opening | Canopy WHIR SEP proof | 0.000 | 0.000 | +0.000 |
| opening | Residual opening work | 0.063 | 0.061 | -0.003 |
| **opening** | **Subtotal** | **70.611** | **70.166** | **-0.445** |
| **Total** | | **86.837** | **86.243** | **-0.595** |

## Input 2^18, width 8

[Per-part stage report](width_8/prover_stage_timings.md) · [Raw stages](width_8/prover_stage_timings.csv) · [Raw totals](width_8/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **262,144** original elements, with a parity RS message of 32,768. The common WHIR initial RS messages have 32,768 and 4,096 elements. Canopy's RS messages have **1** (X) and **1** (Y), with fixed codeword lengths 16,384 and 16,384. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 3.742 | 3.637 | -0.105 |
| commitment | Bolt parity RS encoding | 3.617 | 3.384 | -0.233 |
| commitment | X/Y Merkle trees or bottom forests | 7.288 | 6.779 | -0.508 |
| commitment | Canopy hash-to-field | 0.000 | 0.001 | +0.001 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 0.006 | +0.006 |
| commitment | Root binding and residual | 0.002 | 0.001 | -0.000 |
| **commitment** | **Subtotal** | **14.649** | **13.809** | **-0.840** |
| opening | Bolt partial evaluation g | 5.961 | 6.012 | +0.051 |
| opening | Bolt column projections | 1.443 | 1.313 | -0.130 |
| opening | Common WHIR initial RS commitments | 4.382 | 4.329 | -0.054 |
| opening | Syndrome evaluation and query sampling | 7.862 | 7.857 | -0.005 |
| opening | Copy and serialize queried row values | 29.199 | 29.436 | +0.237 |
| opening | Collect and serialize authentication paths | 3.013 | 2.979 | -0.034 |
| opening | Project and batch queried rows | 4.348 | 4.331 | -0.018 |
| opening | Parity query covector / transpose RS FFT | 1.112 | 1.173 | +0.061 |
| opening | Public H transpose covector | 1.580 | 1.614 | +0.034 |
| opening | Common WHIR constrained proofs | 13.356 | 13.480 | +0.125 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 0.022 | +0.022 |
| opening | Canopy WHIR SEP proof | 0.000 | 0.000 | +0.000 |
| opening | Residual opening work | 0.053 | 0.045 | -0.008 |
| **opening** | **Subtotal** | **72.308** | **72.590** | **+0.282** |
| **Total** | | **86.957** | **86.399** | **-0.558** |

## Input 2^18, width 16

[Per-part stage report](width_16/prover_stage_timings.md) · [Raw stages](width_16/prover_stage_timings.csv) · [Raw totals](width_16/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **262,144** original elements, with a parity RS message of 32,768. The common WHIR initial RS messages have 16,384 and 2,048 elements. Canopy's RS messages have **1** (X) and **1** (Y), with fixed codeword lengths 8,192 and 8,192. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 3.953 | 4.365 | +0.412 |
| commitment | Bolt parity RS encoding | 3.545 | 3.504 | -0.041 |
| commitment | X/Y Merkle trees or bottom forests | 6.186 | 6.437 | +0.251 |
| commitment | Canopy hash-to-field | 0.000 | 0.001 | +0.001 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 0.009 | +0.009 |
| commitment | Root binding and residual | 0.002 | 0.003 | +0.001 |
| **commitment** | **Subtotal** | **13.686** | **14.319** | **+0.632** |
| opening | Bolt partial evaluation g | 6.687 | 6.976 | +0.289 |
| opening | Bolt column projections | 1.379 | 1.383 | +0.003 |
| opening | Common WHIR initial RS commitments | 2.701 | 2.803 | +0.101 |
| opening | Syndrome evaluation and query sampling | 0.205 | 0.212 | +0.008 |
| opening | Copy and serialize queried row values | 49.880 | 49.446 | -0.434 |
| opening | Collect and serialize authentication paths | 1.549 | 1.549 | -0.000 |
| opening | Project and batch queried rows | 6.224 | 6.182 | -0.042 |
| opening | Parity query covector / transpose RS FFT | 0.820 | 0.801 | -0.019 |
| opening | Public H transpose covector | 0.927 | 0.800 | -0.127 |
| opening | Common WHIR constrained proofs | 8.695 | 8.793 | +0.098 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 0.018 | +0.018 |
| opening | Canopy WHIR SEP proof | 0.000 | 0.000 | +0.000 |
| opening | Residual opening work | 0.027 | 0.014 | -0.013 |
| **opening** | **Subtotal** | **79.094** | **78.977** | **-0.118** |
| **Total** | | **92.781** | **93.295** | **+0.515** |

## Input 2^22, width 4

[Per-part stage report](timing_n22/width_4/prover_stage_timings.md) · [Raw stages](timing_n22/width_4/prover_stage_timings.csv) · [Raw totals](timing_n22/width_4/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **4,194,304** original elements, with a parity RS message of 524,288. The common WHIR initial RS messages have 1,048,576 and 131,072 elements. Canopy's RS messages have **262,144** (X) and **1** (Y), with fixed codeword lengths 524,288 and 524,288. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 145.407 | 141.887 | -3.520 |
| commitment | Bolt parity RS encoding | 65.400 | 64.393 | -1.007 |
| commitment | X/Y Merkle trees or bottom forests | 129.076 | 114.478 | -14.598 |
| commitment | Canopy hash-to-field | 0.000 | 48.453 | +48.453 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 30.564 | +30.564 |
| commitment | Root binding and residual | 0.003 | 0.002 | -0.001 |
| **commitment** | **Subtotal** | **339.885** | **399.776** | **+59.890** |
| opening | Bolt partial evaluation g | 103.745 | 92.092 | -11.653 |
| opening | Bolt column projections | 24.338 | 18.607 | -5.731 |
| opening | Common WHIR initial RS commitments | 156.645 | 149.855 | -6.789 |
| opening | Syndrome evaluation and query sampling | 10.433 | 10.329 | -0.104 |
| opening | Copy and serialize queried row values | 16.023 | 15.666 | -0.356 |
| opening | Collect and serialize authentication paths | 13.478 | 5.190 | -8.288 |
| opening | Project and batch queried rows | 5.262 | 5.381 | +0.119 |
| opening | Parity query covector / transpose RS FFT | 15.210 | 15.398 | +0.188 |
| opening | Public H transpose covector | 166.104 | 114.166 | -51.938 |
| opening | Common WHIR constrained proofs | 253.668 | 248.034 | -5.634 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 2.976 | +2.976 |
| opening | Canopy WHIR SEP proof | 0.000 | 41.026 | +41.026 |
| opening | Residual opening work | 1.311 | 1.409 | +0.097 |
| **opening** | **Subtotal** | **766.216** | **720.129** | **-46.088** |
| **Total** | | **1106.102** | **1119.904** | **+13.803** |

## Input 2^22, width 8

[Per-part stage report](timing_n22/width_8/prover_stage_timings.md) · [Raw stages](timing_n22/width_8/prover_stage_timings.csv) · [Raw totals](timing_n22/width_8/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **4,194,304** original elements, with a parity RS message of 524,288. The common WHIR initial RS messages have 524,288 and 65,536 elements. Canopy's RS messages have **131,072** (X) and **1** (Y), with fixed codeword lengths 262,144 and 262,144. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 130.614 | 146.984 | +16.371 |
| commitment | Bolt parity RS encoding | 60.209 | 59.695 | -0.515 |
| commitment | X/Y Merkle trees or bottom forests | 101.668 | 92.351 | -9.318 |
| commitment | Canopy hash-to-field | 0.000 | 24.558 | +24.558 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 14.349 | +14.349 |
| commitment | Root binding and residual | 0.002 | 0.002 | -0.000 |
| **commitment** | **Subtotal** | **292.494** | **337.939** | **+45.445** |
| opening | Bolt partial evaluation g | 92.919 | 92.226 | -0.693 |
| opening | Bolt column projections | 18.256 | 17.736 | -0.519 |
| opening | Common WHIR initial RS commitments | 66.433 | 68.032 | +1.599 |
| opening | Syndrome evaluation and query sampling | 8.575 | 8.737 | +0.162 |
| opening | Copy and serialize queried row values | 31.233 | 34.305 | +3.072 |
| opening | Collect and serialize authentication paths | 10.123 | 5.118 | -5.006 |
| opening | Project and batch queried rows | 6.051 | 6.331 | +0.280 |
| opening | Parity query covector / transpose RS FFT | 8.013 | 9.415 | +1.401 |
| opening | Public H transpose covector | 33.917 | 39.713 | +5.795 |
| opening | Common WHIR constrained proofs | 140.023 | 157.858 | +17.835 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 1.948 | +1.948 |
| opening | Canopy WHIR SEP proof | 0.000 | 22.913 | +22.913 |
| opening | Residual opening work | 0.692 | 0.837 | +0.145 |
| **opening** | **Subtotal** | **416.235** | **465.168** | **+48.932** |
| **Total** | | **708.729** | **803.106** | **+94.377** |

## Input 2^22, width 16

[Per-part stage report](timing_n22/width_16/prover_stage_timings.md) · [Raw stages](timing_n22/width_16/prover_stage_timings.csv) · [Raw totals](timing_n22/width_16/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **4,194,304** original elements, with a parity RS message of 524,288. The common WHIR initial RS messages have 262,144 and 32,768 elements. Canopy's RS messages have **65,536** (X) and **1** (Y), with fixed codeword lengths 131,072 and 131,072. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 105.533 | 108.223 | +2.689 |
| commitment | Bolt parity RS encoding | 54.875 | 55.077 | +0.202 |
| commitment | X/Y Merkle trees or bottom forests | 76.700 | 74.980 | -1.721 |
| commitment | Canopy hash-to-field | 0.000 | 12.218 | +12.218 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 7.451 | +7.451 |
| commitment | Root binding and residual | 0.002 | 0.002 | -0.000 |
| **commitment** | **Subtotal** | **237.111** | **257.951** | **+20.840** |
| opening | Bolt partial evaluation g | 147.929 | 148.646 | +0.717 |
| opening | Bolt column projections | 18.517 | 18.514 | -0.003 |
| opening | Common WHIR initial RS commitments | 33.139 | 34.283 | +1.144 |
| opening | Syndrome evaluation and query sampling | 7.677 | 7.764 | +0.087 |
| opening | Copy and serialize queried row values | 61.552 | 64.243 | +2.691 |
| opening | Collect and serialize authentication paths | 7.861 | 4.327 | -3.534 |
| opening | Project and batch queried rows | 8.604 | 8.472 | -0.132 |
| opening | Parity query covector / transpose RS FFT | 4.733 | 4.435 | -0.298 |
| opening | Public H transpose covector | 14.639 | 13.870 | -0.769 |
| opening | Common WHIR constrained proofs | 63.791 | 63.773 | -0.018 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 1.409 | +1.409 |
| opening | Canopy WHIR SEP proof | 0.000 | 13.174 | +13.174 |
| opening | Residual opening work | 0.351 | 0.438 | +0.087 |
| **opening** | **Subtotal** | **368.793** | **383.347** | **+14.554** |
| **Total** | | **605.904** | **641.298** | **+35.394** |

## Measurement boundaries

Commitment includes Bolt syndrome formation and parity RS encoding. Opening includes the common proofs as well as extra Canopy work. Setup, input generation, verification, proof-object extraction, and final cleanup outside the existing phase timers are excluded. Residual time captures unassigned operations, allocation/drop, and timer bookkeeping inside those boundaries.

RS encoding plus WHIR oracle commitment is one native call, including the FFT, matrix hashing, allocation, and transcript binding. WHIR proof time includes its internal sumchecks, recursive oracles, query openings, and terminal messages; those internals are not individually timed here. Any part using a constant endpoint contributes no nonconstant RS FFT or WHIR SEP proof.

Rebuild this combined report using `python3 scripts/summarize_bolt_profile.py`; regenerate the underlying profiles with the commands in `docs/experiment_bolt_max.md`. Input sizes 2^26 and 2^30 have proof-size estimates only.


See the [full-input RS encoder audit](../bolt_encoder_audit/n22_w4/summary.md) for a comparison with Bolt at the same input size. The paper's 3–4x number concerns optimized binary-field commitment time; the current 256-bit-prime implementation is a different performance setting.
