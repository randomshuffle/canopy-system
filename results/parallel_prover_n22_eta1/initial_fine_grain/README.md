# Parallel prover comparison

Fresh local controls and optimized runs at N=2^22, eta=1, eight Rayon workers, GF(2^256), BLAKE3. Both use release builds, two warmups and ten measured repetitions. Runs execute sequentially, with no concurrent benchmark or build. All cells below are arithmetic mean seconds. Encoding is included in commitment; opening includes Fiat–Shamir. Input generation, public setup and verification are excluded.

| Configuration | Commit before → after | Open before → after | Total before → after | Speedup |
|---|---:|---:|---:|---:|
| WHIR rate 1/2 | 0.3521 → 0.3974 | 0.4274 → 0.4952 | 0.7795 → 0.8926 | 0.87× |
| WHIR rate 1/4 | 0.6409 → 0.7792 | 0.9137 → 1.0805 | 1.5546 → 1.8597 | 0.84× |
| Canopy–WHIR half length | 0.4885 → 0.5490 | 0.4668 → 0.5328 | 0.9553 → 1.0819 | 0.88× |
| Bolt + Merkle | 0.2192 → 0.2382 | 0.3589 → 0.2745 | 0.5781 → 0.5127 | 1.13× |
| Bolt + Canopy k/4 | 0.2559 → 0.2806 | 0.3868 → 0.3079 | 0.6427 → 0.5884 | 1.09× |

WHIR retains the component-composed timing scope: measured and verified native components, not a fully serialized Exp3 proof. Bolt retains the serialized, verifier-checked public-H adapter (no Spark/Mulperm). These are sequential before/after samples, not confidence bounds. Historical local results and the running AWS campaign remain separately identified.

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
| Bolt + Merkle | partial evaluation g | 0.05630 | 0.01371 |
| Bolt + Merkle | public H transpose covector | 0.05706 | 0.00556 |
| Bolt + Merkle | distinct-functional cross evaluations | 0.00593 | 0.00086 |
| Bolt + Canopy k/4 | partial evaluation g | 0.05581 | 0.01403 |
| Bolt + Canopy k/4 | public H transpose covector | 0.05820 | 0.00432 |
| Bolt + Canopy k/4 | distinct-functional cross evaluations | 0.00744 | 0.00100 |

Raw samples, stages, exact commands, binary hashes, wall time and process RSS are in `before/{whir,bolt}/` and `after/{whir,bolt}/`. Frozen binaries and relevant source snapshots are retained under each version. `comparison.csv` also includes total medians and ranges. Regenerate this report with `python3 scripts/summarize_parallel_prover.py`.
