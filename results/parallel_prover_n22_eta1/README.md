# Parallel prover comparison

Fresh local controls and optimized runs at N=2^22, eta=1, eight Rayon workers, GF(2^256), BLAKE3. Both use release builds, two warmups and ten measured repetitions. Runs execute sequentially, with no concurrent benchmark or build. All cells below are arithmetic mean seconds. Encoding is included in commitment; opening includes Fiat–Shamir. Input generation, public setup and verification are excluded.

| Configuration | Commit before → after | Open before → after | Total before → after | Speedup |
|---|---:|---:|---:|---:|
| WHIR rate 1/2 | 0.3521 → 0.3768 | 0.4274 → 0.4707 | 0.7795 → 0.8475 | 0.92× |
| WHIR rate 1/4 | 0.6409 → 0.7902 | 0.9137 → 1.0434 | 1.5546 → 1.8336 | 0.85× |
| Canopy–WHIR half length | 0.4885 → 0.5310 | 0.4668 → 0.5083 | 0.9553 → 1.0394 | 0.92× |
| Bolt + Merkle | 0.2192 → 0.2529 | 0.3589 → 0.2913 | 0.5781 → 0.5442 | 1.06× |
| Bolt + Canopy k/4 | 0.2559 → 0.3334 | 0.3868 → 0.3102 | 0.6427 → 0.6436 | 1.00× |

WHIR retains the component-composed timing scope: measured and verified native components, not a fully serialized Exp3 proof. Bolt retains the serialized, verifier-checked public-H adapter (no Spark/Mulperm). These are sequential before/after samples, not confidence bounds. Historical local results and the running AWS campaign remain separately identified. The machine is an eight-core Apple M2 with 16 GiB RAM.

An initial FFT version used 8192-element sweep thresholds and 1024-element tasks; it was slower locally and is retained in `initial_fine_grain/`. The selected version uses 65536-element sweep thresholds and 4096-element tasks, and inlines the small butterfly loop.

The parallel Bolt stages improve substantially, but the full local timing result is mixed: Bolt opening means fall about 19–20%, while commitment means rise. Merkle Bolt total improves about 6%; Canopy Bolt mean total is essentially unchanged (its median improves about 7%). The three WHIR mean totals rise about 9–18%, with median increases about 4–6%. This does not demonstrate a local WHIR speedup or establish how the changes will scale on AWS.

## Changes

- Compute g using contiguous row blocks, with a local accumulator per task and a parallel reduction.
- Gather H-transpose outputs independently using cached reverse degree-8 adjacency with u32 indices. Duplicate edges are preserved. The new cache adds 32k bytes (32 MiB at this input and w=4), plus a transient k-byte degree array during untimed setup.
- Use the existing thresholded parallel WHIR dot product in the Bolt adapter.
- Evaluate the cross matrix in parallel, borrow dense covectors, and move them into the native proof; write transcript messages in the original row-major order.
- Parallelize forward/transposed FFT butterflies, basis conversion dependency blocks, and coset accumulation. Small sweeps remain serial to limit scheduling overhead.

## Validation

All 83 library tests passed, including new serial-reference conversion tests, large-domain Horner/transpose checks, duplicate-edge tests, and byte-identical batched proof transcripts at one and four workers. Every measured/warmup Bolt proof verified, and serialized sizes match the fresh control exactly. Proof parameters, messages and hash accounting are unchanged.

## Bolt stages

| Method | Stage | Before (s) | After (s) |
|---|---|---:|---:|
| Bolt + Merkle | partial evaluation g | 0.05630 | 0.01649 |
| Bolt + Merkle | public H transpose covector | 0.05706 | 0.00376 |
| Bolt + Merkle | distinct-functional cross evaluations | 0.00593 | 0.00095 |
| Bolt + Canopy k/4 | partial evaluation g | 0.05581 | 0.01646 |
| Bolt + Canopy k/4 | public H transpose covector | 0.05820 | 0.00442 |
| Bolt + Canopy k/4 | distinct-functional cross evaluations | 0.00744 | 0.00120 |

## Repeat control and variability

The unchanged original binaries were rerun after the optimized measurements, with another two warmups and ten repetitions per configuration. This checks local drift; the primary table retains the original same-session control rather than selecting whichever baseline makes the optimization look best.

| Configuration | Original mean | Optimized mean | Repeated original mean | Optimized median (range) |
|---|---:|---:|---:|---:|
| WHIR rate 1/2 | 0.7795 | 0.8475 | 0.7973 | 0.8220 (0.7128–1.2095) |
| WHIR rate 1/4 | 1.5546 | 1.8336 | 1.6210 | 1.6480 (1.3660–2.6838) |
| Canopy–WHIR half length | 0.9553 | 1.0394 | 0.9844 | 1.0031 (0.8848–1.3215) |
| Bolt + Merkle | 0.5781 | 0.5442 | 0.5895 | 0.5206 (0.4813–0.7027) |
| Bolt + Canopy k/4 | 0.6427 | 0.6436 | 0.6666 | 0.5978 (0.5116–0.9562) |

A 10.0-second system-wide sample near the end of the repeat control recorded 2.32 GiB of page compression and 62.9 MiB of swap-ins. These OS counters are not attributable solely to the prover; they show this was not an isolated, swap-free timing environment. No thermal/performance warning was reported. Interpret small differences and outlier-sensitive means cautiously. Raw counters are in `memory_during_control.json`.

Raw samples, stages, exact commands, binary hashes, wall time and process RSS are in `before/{whir,bolt}/` and `after/{whir,bolt}/`. Frozen binaries and relevant source snapshots are retained under each version. `comparison.csv` also includes total medians and ranges. Regenerate this report with `python3 scripts/summarize_parallel_prover.py`.
