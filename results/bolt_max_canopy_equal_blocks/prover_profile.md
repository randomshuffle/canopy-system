# Bolt-max prover time breakdown

Both variants use the same prime-field Bolt-max encoder and common constrained WHIR proofs. Canopy changes the two initial row commitments and adds its own RS/WHIR SEP layer. Parameters: 256-bit field, BLAKE3, initial widths 4/8/16, subsequent WHIR factor 8, Canopy block lengths k/4 for both X and Y, proof-size-optimal heights. The 128-bit per-event target is conditional on prime-field LDPC distance 0.013; this is the public-H adaptation and excludes Spark/Mulperm preprocessing.

Fresh profiles use two warmups and ten measured repetitions for each scheme, width, and input size on 8 Rayon threads. All 120 measured proofs verified. Execution order alternates within each width. Raw data and per-part stage tables are linked below. Earlier timing files are preserved.

Component tables use **arithmetic mean milliseconds**, so they add to the mean commitment/opening/total times. Median totals and observed variation are shown separately. Stage clocks are disjoint wall-clock intervals; parallel-worker CPU time is not summed. Shared stages execute the same algorithms; measured differences may reflect cache, allocation, and run-to-run variation. These local measurements do not establish speedups at unmeasured input sizes.

## Total comparison

| Input | Width | Merkle mean ms | Canopy mean ms | Mean overhead | Merkle median ms | Canopy median ms | Paired delta mean ± sample SD ms |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 2^18 | 4 | 93.141 | 86.463 | -7.17% | 85.725 | 86.014 | -6.678 ± 18.142 |
| 2^18 | 8 | 89.351 | 90.836 | +1.66% | 87.696 | 87.506 | +1.484 ± 10.457 |
| 2^18 | 16 | 85.588 | 87.301 | +2.00% | 85.948 | 86.742 | +1.713 ± 3.219 |
| 2^22 | 4 | 977.014 | 1050.127 | +7.48% | 976.739 | 1027.973 | +73.114 ± 45.876 |
| 2^22 | 8 | 651.150 | 678.116 | +4.14% | 648.923 | 677.614 | +26.966 ± 9.219 |
| 2^22 | 16 | 598.537 | 611.665 | +2.19% | 593.683 | 607.721 | +13.128 ± 25.474 |

The paired spread is the sample standard deviation of Canopy-minus-Merkle times for matching repetition indices, not a confidence interval. Each configuration uses its independently selected heights, with message sizes listed below. Full-height constant endpoints add very little cryptographic work; small differences in their total time should not be interpreted as intrinsic speed benefits.


## Input 2^18, width 4

[Per-part stage report](width_4/prover_stage_timings.md) · [Raw stages](width_4/prover_stage_timings.csv) · [Raw totals](width_4/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **262,144** original elements, with a parity RS message of 32,768. The common WHIR initial RS messages have 65,536 and 8,192 elements. Canopy's RS messages have **1** (X) and **1** (Y), with fixed codeword lengths 16,384 and 16,384. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 3.883 | 3.581 | -0.302 |
| commitment | Bolt parity RS encoding | 3.896 | 3.689 | -0.207 |
| commitment | X/Y Merkle trees or bottom forests | 11.393 | 9.187 | -2.207 |
| commitment | Canopy hash-to-field | 0.000 | 0.001 | +0.001 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 0.009 | +0.009 |
| commitment | Root binding and residual | 0.002 | 0.001 | -0.001 |
| **commitment** | **Subtotal** | **19.174** | **16.468** | **-2.706** |
| opening | Bolt partial evaluation g | 6.769 | 5.998 | -0.772 |
| opening | Bolt column projections | 1.655 | 1.433 | -0.222 |
| opening | Common WHIR initial RS commitments | 8.956 | 8.523 | -0.433 |
| opening | Syndrome evaluation and query sampling | 6.361 | 6.274 | -0.087 |
| opening | Copy and serialize queried row values | 15.908 | 15.257 | -0.651 |
| opening | Collect and serialize authentication paths | 4.393 | 4.312 | -0.082 |
| opening | Project and batch queried rows | 3.176 | 2.684 | -0.493 |
| opening | Parity query covector / transpose RS FFT | 1.563 | 1.535 | -0.027 |
| opening | Public H transpose covector | 3.123 | 3.084 | -0.039 |
| opening | Common WHIR constrained proofs | 21.892 | 20.817 | -1.075 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 0.021 | +0.021 |
| opening | Canopy WHIR SEP proof | 0.000 | 0.000 | +0.000 |
| opening | Residual opening work | 0.171 | 0.059 | -0.112 |
| **opening** | **Subtotal** | **73.966** | **69.995** | **-3.971** |
| **Total** | | **93.141** | **86.463** | **-6.678** |

## Input 2^18, width 8

[Per-part stage report](width_8/prover_stage_timings.md) · [Raw stages](width_8/prover_stage_timings.csv) · [Raw totals](width_8/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **262,144** original elements, with a parity RS message of 32,768. The common WHIR initial RS messages have 32,768 and 4,096 elements. Canopy's RS messages have **1** (X) and **1** (Y), with fixed codeword lengths 8,192 and 8,192. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 4.830 | 3.660 | -1.171 |
| commitment | Bolt parity RS encoding | 3.811 | 3.669 | -0.142 |
| commitment | X/Y Merkle trees or bottom forests | 7.324 | 7.801 | +0.477 |
| commitment | Canopy hash-to-field | 0.000 | 0.001 | +0.001 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 0.007 | +0.007 |
| commitment | Root binding and residual | 0.001 | 0.001 | -0.000 |
| **commitment** | **Subtotal** | **15.967** | **15.139** | **-0.828** |
| opening | Bolt partial evaluation g | 5.908 | 5.969 | +0.061 |
| opening | Bolt column projections | 1.735 | 1.594 | -0.141 |
| opening | Common WHIR initial RS commitments | 4.496 | 5.542 | +1.046 |
| opening | Syndrome evaluation and query sampling | 7.878 | 8.312 | +0.434 |
| opening | Copy and serialize queried row values | 29.795 | 30.259 | +0.464 |
| opening | Collect and serialize authentication paths | 2.833 | 2.866 | +0.033 |
| opening | Project and batch queried rows | 4.379 | 4.371 | -0.007 |
| opening | Parity query covector / transpose RS FFT | 1.069 | 1.060 | -0.009 |
| opening | Public H transpose covector | 1.528 | 1.579 | +0.051 |
| opening | Common WHIR constrained proofs | 13.749 | 14.093 | +0.344 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 0.023 | +0.023 |
| opening | Canopy WHIR SEP proof | 0.000 | 0.000 | +0.000 |
| opening | Residual opening work | 0.015 | 0.030 | +0.014 |
| **opening** | **Subtotal** | **73.384** | **75.697** | **+2.313** |
| **Total** | | **89.351** | **90.836** | **+1.484** |

## Input 2^18, width 16

[Per-part stage report](width_16/prover_stage_timings.md) · [Raw stages](width_16/prover_stage_timings.csv) · [Raw totals](width_16/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **262,144** original elements, with a parity RS message of 32,768. The common WHIR initial RS messages have 16,384 and 2,048 elements. Canopy's RS messages have **1** (X) and **1** (Y), with fixed codeword lengths 4,096 and 4,096. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 3.352 | 3.383 | +0.032 |
| commitment | Bolt parity RS encoding | 3.072 | 3.291 | +0.219 |
| commitment | X/Y Merkle trees or bottom forests | 5.685 | 5.776 | +0.092 |
| commitment | Canopy hash-to-field | 0.000 | 0.001 | +0.001 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 0.005 | +0.005 |
| commitment | Root binding and residual | 0.001 | 0.001 | -0.001 |
| **commitment** | **Subtotal** | **12.110** | **12.458** | **+0.348** |
| opening | Bolt partial evaluation g | 5.816 | 5.958 | +0.142 |
| opening | Bolt column projections | 1.279 | 1.322 | +0.043 |
| opening | Common WHIR initial RS commitments | 2.377 | 2.458 | +0.081 |
| opening | Syndrome evaluation and query sampling | 0.201 | 0.204 | +0.003 |
| opening | Copy and serialize queried row values | 46.079 | 47.059 | +0.980 |
| opening | Collect and serialize authentication paths | 1.550 | 1.511 | -0.040 |
| opening | Project and batch queried rows | 6.161 | 6.270 | +0.109 |
| opening | Parity query covector / transpose RS FFT | 0.757 | 0.781 | +0.023 |
| opening | Public H transpose covector | 0.742 | 0.744 | +0.002 |
| opening | Common WHIR constrained proofs | 8.457 | 8.453 | -0.004 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 0.018 | +0.018 |
| opening | Canopy WHIR SEP proof | 0.000 | 0.000 | +0.000 |
| opening | Residual opening work | 0.058 | 0.065 | +0.008 |
| **opening** | **Subtotal** | **73.478** | **74.843** | **+1.365** |
| **Total** | | **85.588** | **87.301** | **+1.713** |

## Input 2^22, width 4

[Per-part stage report](timing_n22/width_4/prover_stage_timings.md) · [Raw stages](timing_n22/width_4/prover_stage_timings.csv) · [Raw totals](timing_n22/width_4/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **4,194,304** original elements, with a parity RS message of 524,288. The common WHIR initial RS messages have 1,048,576 and 131,072 elements. Canopy's RS messages have **131,072** (X) and **1** (Y), with fixed codeword lengths 262,144 and 262,144. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 135.741 | 138.118 | +2.377 |
| commitment | Bolt parity RS encoding | 60.052 | 60.052 | -0.000 |
| commitment | X/Y Merkle trees or bottom forests | 119.208 | 116.002 | -3.206 |
| commitment | Canopy hash-to-field | 0.000 | 24.169 | +24.169 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 14.189 | +14.189 |
| commitment | Root binding and residual | 0.002 | 0.002 | -0.001 |
| **commitment** | **Subtotal** | **315.003** | **352.532** | **+37.529** |
| opening | Bolt partial evaluation g | 91.089 | 92.125 | +1.036 |
| opening | Bolt column projections | 16.692 | 17.528 | +0.836 |
| opening | Common WHIR initial RS commitments | 144.357 | 146.577 | +2.219 |
| opening | Syndrome evaluation and query sampling | 10.244 | 10.276 | +0.032 |
| opening | Copy and serialize queried row values | 16.396 | 15.752 | -0.644 |
| opening | Collect and serialize authentication paths | 13.041 | 7.620 | -5.421 |
| opening | Project and batch queried rows | 5.122 | 5.254 | +0.132 |
| opening | Parity query covector / transpose RS FFT | 15.013 | 15.221 | +0.208 |
| opening | Public H transpose covector | 110.805 | 116.414 | +5.609 |
| opening | Common WHIR constrained proofs | 238.186 | 245.722 | +7.536 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 1.813 | +1.813 |
| opening | Canopy WHIR SEP proof | 0.000 | 22.082 | +22.082 |
| opening | Residual opening work | 1.066 | 1.212 | +0.147 |
| **opening** | **Subtotal** | **662.011** | **697.595** | **+35.585** |
| **Total** | | **977.014** | **1050.127** | **+73.114** |

## Input 2^22, width 8

[Per-part stage report](timing_n22/width_8/prover_stage_timings.md) · [Raw stages](timing_n22/width_8/prover_stage_timings.csv) · [Raw totals](timing_n22/width_8/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **4,194,304** original elements, with a parity RS message of 524,288. The common WHIR initial RS messages have 524,288 and 65,536 elements. Canopy's RS messages have **65,536** (X) and **1** (Y), with fixed codeword lengths 131,072 and 131,072. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 118.000 | 118.883 | +0.882 |
| commitment | Bolt parity RS encoding | 56.155 | 55.790 | -0.365 |
| commitment | X/Y Merkle trees or bottom forests | 89.564 | 87.753 | -1.811 |
| commitment | Canopy hash-to-field | 0.000 | 12.045 | +12.045 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 7.202 | +7.202 |
| commitment | Root binding and residual | 0.002 | 0.002 | -0.001 |
| **commitment** | **Subtotal** | **263.722** | **281.675** | **+17.952** |
| opening | Bolt partial evaluation g | 90.223 | 89.635 | -0.588 |
| opening | Bolt column projections | 17.463 | 17.139 | -0.324 |
| opening | Common WHIR initial RS commitments | 64.349 | 63.205 | -1.144 |
| opening | Syndrome evaluation and query sampling | 8.480 | 8.538 | +0.059 |
| opening | Copy and serialize queried row values | 30.944 | 30.989 | +0.045 |
| opening | Collect and serialize authentication paths | 9.978 | 6.367 | -3.611 |
| opening | Project and batch queried rows | 5.904 | 5.813 | -0.092 |
| opening | Parity query covector / transpose RS FFT | 7.798 | 7.784 | -0.013 |
| opening | Public H transpose covector | 31.230 | 31.211 | -0.019 |
| opening | Common WHIR constrained proofs | 120.431 | 120.907 | +0.476 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 1.351 | +1.351 |
| opening | Canopy WHIR SEP proof | 0.000 | 12.802 | +12.802 |
| opening | Residual opening work | 0.630 | 0.701 | +0.071 |
| **opening** | **Subtotal** | **387.428** | **396.441** | **+9.013** |
| **Total** | | **651.150** | **678.116** | **+26.966** |

## Input 2^22, width 16

[Per-part stage report](timing_n22/width_16/prover_stage_timings.md) · [Raw stages](timing_n22/width_16/prover_stage_timings.csv) · [Raw totals](timing_n22/width_16/prover_timings.csv)

Workload sizes in field elements: Bolt encodes all **4,194,304** original elements, with a parity RS message of 524,288. The common WHIR initial RS messages have 262,144 and 32,768 elements. Canopy's RS messages have **32,768** (X) and **1** (Y), with fixed codeword lengths 65,536 and 65,536. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.

| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |
|---|---|---:|---:|---:|
| commitment | Bolt sparse syndrome U=HX | 103.618 | 103.465 | -0.153 |
| commitment | Bolt parity RS encoding | 56.556 | 53.821 | -2.735 |
| commitment | X/Y Merkle trees or bottom forests | 75.122 | 74.615 | -0.507 |
| commitment | Canopy hash-to-field | 0.000 | 6.021 | +6.021 |
| commitment | Canopy RS encode and oracle commitment | 0.000 | 3.530 | +3.530 |
| commitment | Root binding and residual | 0.002 | 0.001 | -0.001 |
| **commitment** | **Subtotal** | **235.297** | **241.453** | **+6.156** |
| opening | Bolt partial evaluation g | 145.781 | 145.761 | -0.020 |
| opening | Bolt column projections | 17.810 | 17.713 | -0.098 |
| opening | Common WHIR initial RS commitments | 31.834 | 31.492 | -0.342 |
| opening | Syndrome evaluation and query sampling | 7.666 | 7.677 | +0.012 |
| opening | Copy and serialize queried row values | 64.157 | 62.158 | -1.999 |
| opening | Collect and serialize authentication paths | 7.825 | 5.620 | -2.205 |
| opening | Project and batch queried rows | 8.453 | 8.483 | +0.031 |
| opening | Parity query covector / transpose RS FFT | 4.165 | 4.416 | +0.251 |
| opening | Public H transpose covector | 13.173 | 13.308 | +0.135 |
| opening | Common WHIR constrained proofs | 62.023 | 63.661 | +1.638 |
| opening | Canopy claims, OOD and constant opening | 0.000 | 1.012 | +1.012 |
| opening | Canopy WHIR SEP proof | 0.000 | 8.460 | +8.460 |
| opening | Residual opening work | 0.354 | 0.450 | +0.096 |
| **opening** | **Subtotal** | **363.240** | **370.212** | **+6.972** |
| **Total** | | **598.537** | **611.665** | **+13.128** |

## Measurement boundaries

Commitment includes Bolt syndrome formation and parity RS encoding. Opening includes the common proofs as well as extra Canopy work. Setup, input generation, verification, proof-object extraction, and final cleanup outside the existing phase timers are excluded. Residual time captures unassigned operations, allocation/drop, and timer bookkeeping inside those boundaries.

RS encoding plus WHIR oracle commitment is one native call, including the FFT, matrix hashing, allocation, and transcript binding. WHIR proof time includes its internal sumchecks, recursive oracles, query openings, and terminal messages; those internals are not individually timed here. Any part using a constant endpoint contributes no nonconstant RS FFT or WHIR SEP proof.

Rebuild this combined report using `python3 scripts/summarize_bolt_profile.py`; regenerate the underlying profiles with the commands in `docs/experiment_bolt_max.md`. Input sizes 2^26 and 2^30 have proof-size estimates only.


See the [full-input RS encoder audit](../bolt_encoder_audit/n22_w4/summary.md) for a comparison with Bolt at the same input size. The paper's 3–4x number concerns optimized binary-field commitment time; the current 256-bit-prime implementation is a different performance setting.
