# Bolt-max memory and larger-input experiment

This machine has 16 GiB of physical RAM. The original timing runs did not collect process memory. These fresh runs measure peak resident set size through `getrusage(RUSAGE_CHILDREN)` in a separate parent process. On macOS the reported maximum RSS is in bytes. Each child runs Merkle and Canopy sequentially; this is the maximum over the entire process, including height search, public-H setup, input generation, proving, verification, and allocator-retained memory. It is not the sum of simultaneous provers or a separate peak for each scheme.

Parameters: width 4, 256-bit prime field, BLAKE3, Bolt-max, unchanged conditional distance assumption 0.013 and 128-bit per-event target. Both Canopy RS blocks have length k/2 or k. Heights minimize proof size using 100 samples per candidate. Public H is checked directly, excluding Spark/Mulperm preprocessing.

Each configuration uses 2 warmups, 10 measured repetitions per scheme, and 8 Rayon threads. All 60 measured proofs verified, and every commitment/opening stage total reconciles. Prover medians below include encoding, commitment construction, and opening; setup and verification are excluded from prover timings but included in the memory measurement.

## Measured memory

| Input | Both Canopy blocks | Input alone MiB | Whole-process peak RSS GiB |
|---:|---|---:|---:|
| 2^22 | k | 128 | 1.011 |
| 2^24 | k | 512 | 2.898 |
| 2^24 | k/2 | 512 | 2.837 |

Increasing N by 4 increases peak RSS by 2.87x. A linear projection from the measured 2^24 full-block run puts 2^26 at approximately 11.6 GiB for this executable alone, leaving about 4.4 GiB for the OS and all other applications on this machine. That offers little headroom for the current desktop workload. This is an extrapolation, not a measured 2^26 run. The input vector alone at 2^30 would require 32 GiB. Input 2^18 is excluded from future default Bolt grids and timings; historical results remain available.

## Proof size and prover time

| Input | Both blocks | h_X / h_Y | Merkle MiB | Canopy MiB | Proof saved % | Merkle median s | Canopy median s | Time overhead % |
|---:|---|:---:|---:|---:|---:|---:|---:|---:|
| 2^22 | k | 1 / 18 | 6.458806 | 4.304075 | 33.36 | 1.009717 | 1.264002 | +25.18 |
| 2^24 | k | 1 / 20 | 7.912373 | 4.589244 | 42.00 | 4.606044 | 5.758267 | +25.02 |
| 2^24 | k/2 | 2 / 20 | 7.912373 | 5.179237 | 34.54 | 4.573365 | 5.138512 | +12.36 |

## Measurement data

Raw process resource accounting, individual timing repetitions, and disjoint stage tables are preserved below. Compare each Canopy timing with its accompanying Merkle baseline; measurements include run-to-run variation.

- **n22_w4_equal_full_rusage:** [process memory](n22_w4_equal_full_rusage/process_memory.json), [timings](n22_w4_equal_full_rusage/prover_timings.md), [stage breakdown and variation](n22_w4_equal_full_rusage/prover_stage_timings.md), [selected heights and sizes](n22_w4_equal_full_rusage/timing_height_search/summary.md).
- **n24_w4_equal_full:** [process memory](n24_w4_equal_full/process_memory.json), [timings](n24_w4_equal_full/prover_timings.md), [stage breakdown and variation](n24_w4_equal_full/prover_stage_timings.md), [selected heights and sizes](n24_w4_equal_full/timing_height_search/summary.md).
- **n24_w4_equal_half:** [process memory](n24_w4_equal_half/process_memory.json), [timings](n24_w4_equal_half/prover_timings.md), [stage breakdown and variation](n24_w4_equal_half/prover_stage_timings.md), [selected heights and sizes](n24_w4_equal_half/timing_height_search/summary.md).

Reproduce with `scripts/measure_bolt_memory.py` and rebuild this report with `python3 scripts/summarize_bolt_memory.py`. The initial `/usr/bin/time -l` probe could not collect memory because its clock-rate sysctl is restricted in this environment; the reported measurements use the successful child-process accounting method instead.
