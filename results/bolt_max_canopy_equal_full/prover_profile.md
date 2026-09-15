# Bolt-max prover time breakdown

Both variants use the same prime-field Bolt-max encoder and common constrained WHIR proofs. Canopy changes the two initial row commitments and adds its own RS/WHIR SEP layer. Parameters: 256-bit field, BLAKE3, initial widths 4/8/16, subsequent WHIR factor 8, Canopy block lengths k for X and k for Y, proof-size-optimal heights. The 128-bit per-event target is conditional on prime-field LDPC distance 0.013; this is the public-H adaptation and excludes Spark/Mulperm preprocessing.

Fresh profiles use two warmups and ten measured repetitions for each scheme, width, and input size on 8 Rayon threads. All 120 measured proofs verified. Execution order alternates within each width. Raw data and per-part stage tables are linked below. Earlier timing files are preserved.

Component tables use **arithmetic mean milliseconds**, so they add to the mean commitment/opening/total times. Median totals and observed variation are shown separately. Stage clocks are disjoint wall-clock intervals; parallel-worker CPU time is not summed. Shared stages execute the same algorithms; measured differences may reflect cache, allocation, and run-to-run variation. These local measurements do not establish speedups at unmeasured input sizes.

## Total comparison

| Input | Width | Merkle mean ms | Canopy mean ms | Mean overhead | Merkle median ms | Canopy median ms | Paired delta mean ± sample SD ms |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 2^18 | 4 | 102.537 | 105.349 | +2.74% | 89.645 | 103.800 | +2.811 ± 31.201 |
| 2^18 | 8 | 87.703 | 87.597 | -0.12% | 87.530 | 87.317 | -0.105 ± 4.035 |
| 2^18 | 16 | 95.999 | 91.034 | -5.17% | 90.263 | 90.926 | -4.965 ± 17.360 |
| 2^22 | 4 | 1071.840 | 1282.884 | +19.69% | 1046.714 | 1271.845 | +211.044 ± 66.248 |
| 2^22 | 8 | 708.945 | 814.237 | +14.85% | 698.818 | 811.479 | +105.292 ± 17.128 |
| 2^22 | 16 | 674.131 | 742.857 | +10.19% | 643.311 | 695.762 | +68.726 ± 158.548 |

The paired spread is the sample standard deviation of Canopy-minus-Merkle times for matching repetition indices, not a confidence interval. Each configuration uses its independently selected heights, with message sizes listed below. Full-height constant endpoints add very little cryptographic work; small differences in their total time should not be interpreted as intrinsic speed benefits.


## Input 2^18, width 4

[Per-part stage report](width_4/prover_stage_timings.md) · [Raw stages](width_4/prover_stage_timings.csv) · [Raw totals](width_4/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **262,144** original elements, with a parity RS message of 32,768. The common WHIR initial RS messages have 65,536 and 8,192 elements. Canopy's RS messages have **32,768** (X) and **1** (Y), with fixed codeword lengths 65,536 and 65,536. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 4.408 | 3.773 | -0.635 |
| commitment | Bolt parity RS encoding | 5.126 | 4.126 | -1.000 |
| commitment | X/Y Merkle trees or bottom forests | 12.051 | 7.929 | -4.122 |
| commitment | Canopy hash-to-field | 0.000 | 6.115 | +6.115 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 3.643 | +3.643 |
| commitment | Root binding and residual | 0.002 | 0.001 | -0.001 |
| **commitment** | **Subtotal** | **21.588** | **25.589** | **+4.001** |
| opening | Bolt partial evaluation g | 7.076 | 6.043 | -1.033 |
| opening | Bolt column projections | 1.649 | 1.521 | -0.129 |
| opening | Common WHIR initial RS commitments | 9.474 | 8.778 | -0.695 |
| opening | Syndrome evaluation and query sampling | 6.650 | 6.367 | -0.283 |
| opening | Copy and serialize queried row values | 16.761 | 15.574 | -1.188 |
| opening | Collect and serialize authentication paths | 4.754 | 1.878 | -2.876 |
| opening | Project and batch queried rows | 3.105 | 2.822 | -0.284 |
| opening | Parity query covector / transpose RS FFT | 2.340 | 1.623 | -0.717 |
| opening | Public H transpose covector | 4.085 | 3.315 | -0.769 |
| opening | Common WHIR constrained proofs | 24.976 | 21.602 | -3.373 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 1.146 | +1.146 |
| opening | Canopy WHIR SEP proof | 0.000 | 8.970 | +8.970 |
| opening | Residual opening work | 0.080 | 0.121 | +0.041 |
| **opening** | **Subtotal** | **80.950** | **79.760** | **-1.189** |
| **Total** | | **102.537** | **105.349** | **+2.811** |

## Input 2^18, width 8

[Per-part stage report](width_8/prover_stage_timings.md) · [Raw stages](width_8/prover_stage_timings.csv) · [Raw totals](width_8/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **262,144** original elements, with a parity RS message of 32,768. The common WHIR initial RS messages have 32,768 and 4,096 elements. Canopy's RS messages have **1** (X) and **1** (Y), with fixed codeword lengths 32,768 and 32,768. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 3.605 | 3.691 | +0.086 |
| commitment | Bolt parity RS encoding | 3.498 | 3.308 | -0.190 |
| commitment | X/Y Merkle trees or bottom forests | 6.800 | 7.018 | +0.218 |
| commitment | Canopy hash-to-field | 0.000 | 0.001 | +0.001 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 0.006 | +0.006 |
| commitment | Root binding and residual | 0.002 | 0.001 | -0.001 |
| **commitment** | **Subtotal** | **13.905** | **14.026** | **+0.121** |
| opening | Bolt partial evaluation g | 6.029 | 5.926 | -0.103 |
| opening | Bolt column projections | 1.427 | 1.342 | -0.085 |
| opening | Common WHIR initial RS commitments | 4.325 | 4.407 | +0.082 |
| opening | Syndrome evaluation and query sampling | 7.905 | 7.837 | -0.068 |
| opening | Copy and serialize queried row values | 30.250 | 30.324 | +0.075 |
| opening | Collect and serialize authentication paths | 2.978 | 2.916 | -0.062 |
| opening | Project and batch queried rows | 4.386 | 4.388 | +0.001 |
| opening | Parity query covector / transpose RS FFT | 0.991 | 1.139 | +0.148 |
| opening | Public H transpose covector | 1.592 | 1.609 | +0.018 |
| opening | Common WHIR constrained proofs | 13.884 | 13.602 | -0.281 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 0.026 | +0.026 |
| opening | Canopy WHIR SEP proof | 0.000 | 0.000 | +0.000 |
| opening | Residual opening work | 0.032 | 0.056 | +0.024 |
| **opening** | **Subtotal** | **73.798** | **73.572** | **-0.226** |
| **Total** | | **87.703** | **87.597** | **-0.105** |

## Input 2^18, width 16

[Per-part stage report](width_16/prover_stage_timings.md) · [Raw stages](width_16/prover_stage_timings.csv) · [Raw totals](width_16/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **262,144** original elements, with a parity RS message of 32,768. The common WHIR initial RS messages have 16,384 and 2,048 elements. Canopy's RS messages have **1** (X) and **1** (Y), with fixed codeword lengths 16,384 and 16,384. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 4.024 | 3.657 | -0.367 |
| commitment | Bolt parity RS encoding | 3.391 | 3.469 | +0.077 |
| commitment | X/Y Merkle trees or bottom forests | 9.767 | 5.872 | -3.896 |
| commitment | Canopy hash-to-field | 0.000 | 0.001 | +0.001 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 0.007 | +0.007 |
| commitment | Root binding and residual | 0.002 | 0.001 | -0.001 |
| **commitment** | **Subtotal** | **17.185** | **13.007** | **-4.177** |
| opening | Bolt partial evaluation g | 6.418 | 6.400 | -0.018 |
| opening | Bolt column projections | 2.350 | 1.372 | -0.979 |
| opening | Common WHIR initial RS commitments | 2.571 | 2.643 | +0.072 |
| opening | Syndrome evaluation and query sampling | 0.222 | 0.205 | -0.017 |
| opening | Copy and serialize queried row values | 48.538 | 49.153 | +0.615 |
| opening | Collect and serialize authentication paths | 1.637 | 1.611 | -0.026 |
| opening | Project and batch queried rows | 6.761 | 6.214 | -0.546 |
| opening | Parity query covector / transpose RS FFT | 0.787 | 0.825 | +0.038 |
| opening | Public H transpose covector | 0.765 | 0.786 | +0.021 |
| opening | Common WHIR constrained proofs | 8.754 | 8.766 | +0.011 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 0.019 | +0.019 |
| opening | Canopy WHIR SEP proof | 0.000 | 0.000 | +0.000 |
| opening | Residual opening work | 0.010 | 0.033 | +0.022 |
| **opening** | **Subtotal** | **78.814** | **78.027** | **-0.788** |
| **Total** | | **95.999** | **91.034** | **-4.965** |

## Input 2^22, width 4

[Per-part stage report](timing_n22/width_4/prover_stage_timings.md) · [Raw stages](timing_n22/width_4/prover_stage_timings.csv) · [Raw totals](timing_n22/width_4/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **4,194,304** original elements, with a parity RS message of 524,288. The common WHIR initial RS messages have 1,048,576 and 131,072 elements. Canopy's RS messages have **524,288** (X) and **1** (Y), with fixed codeword lengths 1,048,576 and 1,048,576. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 148.336 | 150.537 | +2.202 |
| commitment | Bolt parity RS encoding | 64.326 | 64.247 | -0.079 |
| commitment | X/Y Merkle trees or bottom forests | 125.008 | 107.475 | -17.533 |
| commitment | Canopy hash-to-field | 0.000 | 96.950 | +96.950 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 59.297 | +59.297 |
| commitment | Root binding and residual | 0.003 | 0.002 | -0.001 |
| **commitment** | **Subtotal** | **337.673** | **478.509** | **+140.836** |
| opening | Bolt partial evaluation g | 94.175 | 94.285 | +0.110 |
| opening | Bolt column projections | 17.358 | 17.887 | +0.529 |
| opening | Common WHIR initial RS commitments | 154.070 | 155.597 | +1.527 |
| opening | Syndrome evaluation and query sampling | 10.473 | 10.571 | +0.098 |
| opening | Copy and serialize queried row values | 16.778 | 16.844 | +0.065 |
| opening | Collect and serialize authentication paths | 14.064 | 3.142 | -10.922 |
| opening | Project and batch queried rows | 5.830 | 5.584 | -0.246 |
| opening | Parity query covector / transpose RS FFT | 16.123 | 15.700 | -0.423 |
| opening | Public H transpose covector | 144.458 | 130.827 | -13.631 |
| opening | Common WHIR constrained proofs | 259.670 | 257.067 | -2.603 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 5.588 | +5.588 |
| opening | Canopy WHIR SEP proof | 0.000 | 89.509 | +89.509 |
| opening | Residual opening work | 1.167 | 1.774 | +0.607 |
| **opening** | **Subtotal** | **734.167** | **804.375** | **+70.208** |
| **Total** | | **1071.840** | **1282.884** | **+211.044** |

## Input 2^22, width 8

[Per-part stage report](timing_n22/width_8/prover_stage_timings.md) · [Raw stages](timing_n22/width_8/prover_stage_timings.csv) · [Raw totals](timing_n22/width_8/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **4,194,304** original elements, with a parity RS message of 524,288. The common WHIR initial RS messages have 524,288 and 65,536 elements. Canopy's RS messages have **262,144** (X) and **1** (Y), with fixed codeword lengths 524,288 and 524,288. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 136.775 | 134.744 | -2.031 |
| commitment | Bolt parity RS encoding | 60.444 | 60.194 | -0.250 |
| commitment | X/Y Merkle trees or bottom forests | 95.241 | 87.415 | -7.826 |
| commitment | Canopy hash-to-field | 0.000 | 48.562 | +48.562 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 30.528 | +30.528 |
| commitment | Root binding and residual | 0.003 | 0.002 | -0.001 |
| **commitment** | **Subtotal** | **292.463** | **361.445** | **+68.981** |
| opening | Bolt partial evaluation g | 97.658 | 97.640 | -0.018 |
| opening | Bolt column projections | 18.179 | 17.903 | -0.276 |
| opening | Common WHIR initial RS commitments | 67.833 | 66.392 | -1.442 |
| opening | Syndrome evaluation and query sampling | 8.719 | 8.647 | -0.073 |
| opening | Copy and serialize queried row values | 30.992 | 31.114 | +0.122 |
| opening | Collect and serialize authentication paths | 10.696 | 2.907 | -7.789 |
| opening | Project and batch queried rows | 6.286 | 6.160 | -0.126 |
| opening | Parity query covector / transpose RS FFT | 8.148 | 8.215 | +0.067 |
| opening | Public H transpose covector | 39.874 | 39.808 | -0.066 |
| opening | Common WHIR constrained proofs | 127.381 | 127.579 | +0.198 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 3.289 | +3.289 |
| opening | Canopy WHIR SEP proof | 0.000 | 42.183 | +42.183 |
| opening | Residual opening work | 0.715 | 0.954 | +0.240 |
| **opening** | **Subtotal** | **416.482** | **452.793** | **+36.310** |
| **Total** | | **708.945** | **814.237** | **+105.292** |

## Input 2^22, width 16

[Per-part stage report](timing_n22/width_16/prover_stage_timings.md) · [Raw stages](timing_n22/width_16/prover_stage_timings.csv) · [Raw totals](timing_n22/width_16/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **4,194,304** original elements, with a parity RS message of 524,288. The common WHIR initial RS messages have 262,144 and 32,768 elements. Canopy's RS messages have **131,072** (X) and **1** (Y), with fixed codeword lengths 262,144 and 262,144. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 143.130 | 140.000 | -3.131 |
| commitment | Bolt parity RS encoding | 64.467 | 64.072 | -0.395 |
| commitment | X/Y Merkle trees or bottom forests | 82.095 | 77.300 | -4.795 |
| commitment | Canopy hash-to-field | 0.000 | 24.433 | +24.433 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 15.347 | +15.347 |
| commitment | Root binding and residual | 0.003 | 0.002 | -0.001 |
| **commitment** | **Subtotal** | **289.694** | **321.153** | **+31.459** |
| opening | Bolt partial evaluation g | 157.567 | 161.658 | +4.091 |
| opening | Bolt column projections | 18.522 | 20.282 | +1.760 |
| opening | Common WHIR initial RS commitments | 34.888 | 37.843 | +2.955 |
| opening | Syndrome evaluation and query sampling | 7.838 | 7.988 | +0.150 |
| opening | Copy and serialize queried row values | 63.522 | 64.695 | +1.173 |
| opening | Collect and serialize authentication paths | 7.844 | 2.852 | -4.992 |
| opening | Project and batch queried rows | 8.725 | 8.803 | +0.078 |
| opening | Parity query covector / transpose RS FFT | 4.765 | 4.937 | +0.172 |
| opening | Public H transpose covector | 14.505 | 15.798 | +1.293 |
| opening | Common WHIR constrained proofs | 65.895 | 69.326 | +3.431 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 1.891 | +1.891 |
| opening | Canopy WHIR SEP proof | 0.000 | 24.883 | +24.883 |
| opening | Residual opening work | 0.366 | 0.748 | +0.382 |
| **opening** | **Subtotal** | **384.437** | **421.703** | **+37.267** |
| **Total** | | **674.131** | **742.857** | **+68.726** |

## Measurement boundaries

Commitment includes Bolt syndrome formation and parity RS encoding. Opening includes the common proofs as well as extra Canopy work. Setup, input generation, verification, proof-object extraction, and final cleanup outside the existing phase timers are excluded. Residual time captures unassigned operations, allocation/drop, and timer bookkeeping inside those boundaries.

RS encoding plus WHIR oracle commitment is one native call, including the FFT, matrix hashing, allocation, and transcript binding. WHIR proof time includes its internal sumchecks, recursive oracles, query openings, and terminal messages; those internals are not individually timed here. Any part using a constant endpoint contributes no nonconstant RS FFT or WHIR SEP proof.

Rebuild this combined report using `python3 scripts/summarize_bolt_profile.py`; regenerate the underlying profiles with the commands in `docs/experiment_bolt_max.md`. Input sizes 2^26 and 2^30 have proof-size estimates only.


See the [full-input RS encoder audit](../bolt_encoder_audit/n22_w4/summary.md) for a comparison with Bolt at the same input size. The paper's 3–4x number concerns optimized binary-field commitment time; the current 256-bit-prime implementation is a different performance setting.
