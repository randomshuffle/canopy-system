# Bolt-max prover time breakdown

Both variants use the same prime-field Bolt-max encoder and common constrained WHIR proofs. Canopy changes the two initial row commitments and adds its own RS/WHIR SEP layer. Parameters: 256-bit field, BLAKE3, initial widths 4/8/16, subsequent WHIR factor 8, separate quarter-length Canopy RS layers, proof-size-optimal heights. The 128-bit per-event target is conditional on prime-field LDPC distance 0.013; this is the public-H adaptation and excludes Spark/Mulperm preprocessing.

Fresh profiles use two warmups and ten measured repetitions for each scheme, width, and input size on 8 Rayon threads. All 120 measured proofs verified. Execution order alternates within each width. Raw data and per-part stage tables are linked below. Earlier timing files are preserved.

Component tables use **arithmetic mean milliseconds**, so they add to the mean commitment/opening/total times. Median totals and observed variation are shown separately. Stage clocks are disjoint wall-clock intervals; parallel-worker CPU time is not summed. Shared stages execute the same algorithms; measured differences may reflect cache, allocation, and run-to-run variation. These local measurements do not establish speedups at unmeasured input sizes.

## Total comparison

| Input | Width | Merkle mean ms | Canopy mean ms | Mean overhead | Merkle median ms | Canopy median ms | Paired delta mean ± sample SD ms |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 2^18 | 4 | 93.914 | 93.975 | +0.06% | 92.549 | 94.295 | +0.060 ± 5.476 |
| 2^18 | 8 | 84.941 | 85.956 | +1.20% | 84.852 | 85.688 | +1.015 ± 3.063 |
| 2^18 | 16 | 89.067 | 91.730 | +2.99% | 89.964 | 89.953 | +2.664 ± 8.100 |
| 2^22 | 4 | 1102.343 | 1177.991 | +6.86% | 1098.309 | 1171.434 | +75.648 ± 52.429 |
| 2^22 | 8 | 701.937 | 722.314 | +2.90% | 682.158 | 719.465 | +20.377 ± 51.728 |
| 2^22 | 16 | 618.430 | 650.046 | +5.11% | 610.452 | 649.550 | +31.617 ± 42.906 |

The paired spread is the sample standard deviation of Canopy-minus-Merkle times for matching repetition indices, not a confidence interval. At 2^18 both Canopy parts are constant/full-height endpoints; their added cryptographic work is tiny, and small measured total differences should not be treated as an intrinsic speed benefit. At 2^22 the systematic Canopy height is 3, while parity uses its full-height constant endpoint.


## Input 2^18, width 4

[Per-part stage report](n18/width_4/prover_stage_timings.md) · [Raw stages](n18/width_4/prover_stage_timings.csv) · [Raw totals](n18/width_4/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **262,144** original elements, with a parity RS message of 32,768. The common WHIR initial RS messages have 65,536 and 8,192 elements. Canopy's RS messages have **1** (X) and **1** (Y), with fixed codeword lengths 16,384 and 4,096. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 4.125 | 4.137 | +0.011 |
| commitment | Bolt parity RS encoding | 4.371 | 4.588 | +0.217 |
| commitment | X/Y Merkle trees or bottom forests | 10.424 | 10.943 | +0.519 |
| commitment | Canopy hash-to-field | 0.000 | 0.001 | +0.001 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 0.011 | +0.011 |
| commitment | Root binding and residual | 0.002 | 0.003 | +0.000 |
| **commitment** | **Subtotal** | **18.922** | **19.683** | **+0.760** |
| opening | Bolt partial evaluation g | 6.218 | 6.324 | +0.106 |
| opening | Bolt column projections | 1.600 | 1.714 | +0.114 |
| opening | Common WHIR initial RS commitments | 9.385 | 9.162 | -0.224 |
| opening | Syndrome evaluation and query sampling | 6.496 | 6.556 | +0.060 |
| opening | Copy and serialize queried row values | 15.890 | 15.317 | -0.573 |
| opening | Collect and serialize authentication paths | 4.481 | 4.431 | -0.050 |
| opening | Project and batch queried rows | 2.860 | 2.853 | -0.007 |
| opening | Parity query covector / transpose RS FFT | 1.646 | 1.865 | +0.219 |
| opening | Public H transpose covector | 3.355 | 3.483 | +0.129 |
| opening | Common WHIR constrained proofs | 22.991 | 22.512 | -0.479 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 0.021 | +0.021 |
| opening | Canopy WHIR SEP proof | 0.000 | 0.000 | +0.000 |
| opening | Residual opening work | 0.070 | 0.054 | -0.016 |
| **opening** | **Subtotal** | **74.992** | **74.292** | **-0.700** |
| **Total** | | **93.914** | **93.975** | **+0.060** |

## Input 2^18, width 8

[Per-part stage report](n18/width_8/prover_stage_timings.md) · [Raw stages](n18/width_8/prover_stage_timings.csv) · [Raw totals](n18/width_8/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **262,144** original elements, with a parity RS message of 32,768. The common WHIR initial RS messages have 32,768 and 4,096 elements. Canopy's RS messages have **1** (X) and **1** (Y), with fixed codeword lengths 8,192 and 2,048. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 3.494 | 3.733 | +0.239 |
| commitment | Bolt parity RS encoding | 3.561 | 3.556 | -0.005 |
| commitment | X/Y Merkle trees or bottom forests | 6.847 | 7.070 | +0.223 |
| commitment | Canopy hash-to-field | 0.000 | 0.001 | +0.001 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 0.006 | +0.006 |
| commitment | Root binding and residual | 0.002 | 0.001 | -0.001 |
| **commitment** | **Subtotal** | **13.904** | **14.367** | **+0.463** |
| opening | Bolt partial evaluation g | 5.696 | 5.827 | +0.130 |
| opening | Bolt column projections | 1.324 | 1.385 | +0.061 |
| opening | Common WHIR initial RS commitments | 4.236 | 4.381 | +0.145 |
| opening | Syndrome evaluation and query sampling | 7.737 | 7.675 | -0.063 |
| opening | Copy and serialize queried row values | 29.080 | 29.406 | +0.327 |
| opening | Collect and serialize authentication paths | 2.777 | 2.799 | +0.022 |
| opening | Project and batch queried rows | 4.364 | 4.485 | +0.121 |
| opening | Parity query covector / transpose RS FFT | 1.120 | 1.062 | -0.059 |
| opening | Public H transpose covector | 1.599 | 1.581 | -0.018 |
| opening | Common WHIR constrained proofs | 13.094 | 12.949 | -0.146 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 0.023 | +0.023 |
| opening | Canopy WHIR SEP proof | 0.000 | 0.000 | +0.000 |
| opening | Residual opening work | 0.010 | 0.017 | +0.007 |
| **opening** | **Subtotal** | **71.037** | **71.589** | **+0.552** |
| **Total** | | **84.941** | **85.956** | **+1.015** |

## Input 2^18, width 16

[Per-part stage report](n18/width_16/prover_stage_timings.md) · [Raw stages](n18/width_16/prover_stage_timings.csv) · [Raw totals](n18/width_16/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **262,144** original elements, with a parity RS message of 32,768. The common WHIR initial RS messages have 16,384 and 2,048 elements. Canopy's RS messages have **1** (X) and **1** (Y), with fixed codeword lengths 4,096 and 1,024. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 3.555 | 3.767 | +0.212 |
| commitment | Bolt parity RS encoding | 3.207 | 3.653 | +0.447 |
| commitment | X/Y Merkle trees or bottom forests | 6.029 | 6.243 | +0.214 |
| commitment | Canopy hash-to-field | 0.000 | 0.001 | +0.001 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 0.005 | +0.005 |
| commitment | Root binding and residual | 0.002 | 0.001 | -0.001 |
| **commitment** | **Subtotal** | **12.793** | **13.671** | **+0.878** |
| opening | Bolt partial evaluation g | 6.225 | 6.410 | +0.184 |
| opening | Bolt column projections | 1.388 | 1.454 | +0.066 |
| opening | Common WHIR initial RS commitments | 2.426 | 2.516 | +0.090 |
| opening | Syndrome evaluation and query sampling | 0.204 | 0.205 | +0.001 |
| opening | Copy and serialize queried row values | 48.548 | 49.218 | +0.670 |
| opening | Collect and serialize authentication paths | 1.472 | 1.477 | +0.006 |
| opening | Project and batch queried rows | 6.335 | 6.500 | +0.165 |
| opening | Parity query covector / transpose RS FFT | 0.796 | 1.136 | +0.340 |
| opening | Public H transpose covector | 0.777 | 0.785 | +0.007 |
| opening | Common WHIR constrained proofs | 8.097 | 8.289 | +0.192 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 0.017 | +0.017 |
| opening | Canopy WHIR SEP proof | 0.000 | 0.000 | +0.000 |
| opening | Residual opening work | 0.006 | 0.053 | +0.046 |
| **opening** | **Subtotal** | **76.273** | **78.059** | **+1.786** |
| **Total** | | **89.067** | **91.730** | **+2.664** |

## Input 2^22, width 4

[Per-part stage report](n22/width_4/prover_stage_timings.md) · [Raw stages](n22/width_4/prover_stage_timings.csv) · [Raw totals](n22/width_4/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **4,194,304** original elements, with a parity RS message of 524,288. The common WHIR initial RS messages have 1,048,576 and 131,072 elements. Canopy's RS messages have **131,072** (X) and **1** (Y), with fixed codeword lengths 262,144 and 65,536. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 165.950 | 159.675 | -6.274 |
| commitment | Bolt parity RS encoding | 74.067 | 68.736 | -5.331 |
| commitment | X/Y Merkle trees or bottom forests | 128.818 | 130.072 | +1.254 |
| commitment | Canopy hash-to-field | 0.000 | 25.027 | +25.027 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 14.793 | +14.793 |
| commitment | Root binding and residual | 0.003 | 0.002 | -0.001 |
| **commitment** | **Subtotal** | **368.838** | **398.305** | **+29.467** |
| opening | Bolt partial evaluation g | 94.257 | 96.661 | +2.404 |
| opening | Bolt column projections | 17.703 | 19.849 | +2.147 |
| opening | Common WHIR initial RS commitments | 164.515 | 171.787 | +7.272 |
| opening | Syndrome evaluation and query sampling | 10.592 | 10.816 | +0.225 |
| opening | Copy and serialize queried row values | 19.252 | 17.536 | -1.716 |
| opening | Collect and serialize authentication paths | 13.825 | 8.910 | -4.915 |
| opening | Project and batch queried rows | 5.619 | 5.686 | +0.067 |
| opening | Parity query covector / transpose RS FFT | 16.663 | 18.007 | +1.345 |
| opening | Public H transpose covector | 127.095 | 135.832 | +8.737 |
| opening | Common WHIR constrained proofs | 262.853 | 265.844 | +2.990 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 1.882 | +1.882 |
| opening | Canopy WHIR SEP proof | 0.000 | 25.217 | +25.217 |
| opening | Residual opening work | 1.130 | 1.658 | +0.528 |
| **opening** | **Subtotal** | **733.504** | **779.685** | **+46.181** |
| **Total** | | **1102.343** | **1177.991** | **+75.648** |

## Input 2^22, width 8

[Per-part stage report](n22/width_8/prover_stage_timings.md) · [Raw stages](n22/width_8/prover_stage_timings.csv) · [Raw totals](n22/width_8/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **4,194,304** original elements, with a parity RS message of 524,288. The common WHIR initial RS messages have 524,288 and 65,536 elements. Canopy's RS messages have **65,536** (X) and **1** (Y), with fixed codeword lengths 131,072 and 32,768. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 135.644 | 133.070 | -2.573 |
| commitment | Bolt parity RS encoding | 64.219 | 58.151 | -6.069 |
| commitment | X/Y Merkle trees or bottom forests | 94.460 | 92.847 | -1.613 |
| commitment | Canopy hash-to-field | 0.000 | 12.235 | +12.235 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 7.829 | +7.829 |
| commitment | Root binding and residual | 0.005 | 0.002 | -0.003 |
| **commitment** | **Subtotal** | **294.328** | **304.134** | **+9.806** |
| opening | Bolt partial evaluation g | 93.391 | 92.414 | -0.977 |
| opening | Bolt column projections | 18.312 | 19.208 | +0.895 |
| opening | Common WHIR initial RS commitments | 69.205 | 69.056 | -0.149 |
| opening | Syndrome evaluation and query sampling | 8.628 | 8.677 | +0.049 |
| opening | Copy and serialize queried row values | 30.829 | 32.002 | +1.173 |
| opening | Collect and serialize authentication paths | 10.184 | 6.667 | -3.517 |
| opening | Project and batch queried rows | 6.048 | 6.125 | +0.077 |
| opening | Parity query covector / transpose RS FFT | 8.106 | 8.337 | +0.231 |
| opening | Public H transpose covector | 35.914 | 35.207 | -0.706 |
| opening | Common WHIR constrained proofs | 126.431 | 124.957 | -1.474 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 1.287 | +1.287 |
| opening | Canopy WHIR SEP proof | 0.000 | 13.458 | +13.458 |
| opening | Residual opening work | 0.560 | 0.784 | +0.223 |
| **opening** | **Subtotal** | **407.609** | **418.180** | **+10.570** |
| **Total** | | **701.937** | **722.314** | **+20.377** |

## Input 2^22, width 16

[Per-part stage report](n22/width_16/prover_stage_timings.md) · [Raw stages](n22/width_16/prover_stage_timings.csv) · [Raw totals](n22/width_16/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **4,194,304** original elements, with a parity RS message of 524,288. The common WHIR initial RS messages have 262,144 and 32,768 elements. Canopy's RS messages have **32,768** (X) and **1** (Y), with fixed codeword lengths 65,536 and 16,384. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 109.623 | 114.176 | +4.553 |
| commitment | Bolt parity RS encoding | 57.444 | 57.975 | +0.531 |
| commitment | X/Y Merkle trees or bottom forests | 78.673 | 79.361 | +0.688 |
| commitment | Canopy hash-to-field | 0.000 | 6.098 | +6.098 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 3.833 | +3.833 |
| commitment | Root binding and residual | 0.002 | 0.002 | -0.000 |
| **commitment** | **Subtotal** | **245.743** | **261.444** | **+15.702** |
| opening | Bolt partial evaluation g | 149.253 | 153.735 | +4.482 |
| opening | Bolt column projections | 18.269 | 19.466 | +1.196 |
| opening | Common WHIR initial RS commitments | 33.696 | 34.286 | +0.590 |
| opening | Syndrome evaluation and query sampling | 7.716 | 7.736 | +0.020 |
| opening | Copy and serialize queried row values | 59.322 | 61.560 | +2.238 |
| opening | Collect and serialize authentication paths | 8.024 | 5.809 | -2.215 |
| opening | Project and batch queried rows | 8.690 | 9.067 | +0.376 |
| opening | Parity query covector / transpose RS FFT | 4.482 | 4.511 | +0.029 |
| opening | Public H transpose covector | 14.350 | 14.634 | +0.284 |
| opening | Common WHIR constrained proofs | 68.300 | 67.628 | -0.672 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 1.083 | +1.083 |
| opening | Canopy WHIR SEP proof | 0.000 | 8.660 | +8.660 |
| opening | Residual opening work | 0.585 | 0.428 | -0.157 |
| **opening** | **Subtotal** | **372.687** | **388.602** | **+15.915** |
| **Total** | | **618.430** | **650.046** | **+31.617** |

## Measurement boundaries

Commitment includes Bolt syndrome formation and parity RS encoding. Opening includes the common proofs as well as extra Canopy work. Setup, input generation, verification, proof-object extraction, and final cleanup outside the existing phase timers are excluded. Residual time captures unassigned operations, allocation/drop, and timer bookkeeping inside those boundaries.

RS encoding plus WHIR oracle commitment is one native call, including the FFT, matrix hashing, allocation, and transcript binding. WHIR proof time includes its internal sumchecks, recursive oracles, query openings, and terminal messages; those internals are not individually timed here. The constant parity layer contributes no nonconstant RS FFT or WHIR SEP proof.

Rebuild this combined report using `python3 scripts/summarize_bolt_profile.py`; regenerate the underlying profiles with the commands in `docs/experiment_bolt_max.md`. Input sizes 2^26 and 2^30 have proof-size estimates only.


See the [full-input RS encoder audit](../bolt_encoder_audit/n22_w4/summary.md) for a comparison with Bolt at the same input size. The paper's 3–4x number concerns optimized binary-field commitment time; the current 256-bit-prime implementation is a different performance setting.
