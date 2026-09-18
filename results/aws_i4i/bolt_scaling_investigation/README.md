# Bolt scaling investigation

Read-only investigation of the running AWS campaign. No prover code, parameters, or timing instrumentation was changed. Evidence: existing per-stage timers, source inspection, process accounting, and an eight-second /proc thread-counter sample. No competing benchmark or perf recording was launched.

## Measured serial bottlenecks

Mean seconds at N=2^22, eta=1. Each repeated stage is summed within a proof before averaging over repetitions; totals reconcile to the prover timer.

| Method / stage | Mac | AWS | AWS prover share |
|---|---:|---:|---:|
| bolt_merkle_shared / partial evaluation g | 0.0520 | 0.1358 | 22.4% |
| bolt_merkle_shared / public H transpose covector | 0.0515 | 0.1007 | 16.6% |
| bolt_canopy_split_quarter / partial evaluation g | 0.0588 | 0.1342 | 19.6% |
| bolt_canopy_split_quarter / public H transpose covector | 0.0503 | 0.1063 | 15.5% |

The two stages together use 39.0% of the AWS Merkle prover and 35.1% of the Canopy prover at eta=1. They explain most of the increase in opening time. The field backend is the validated PCLMULQDQ implementation; parallel WHIR is enabled by its default Cargo feature.

At eta=8 the batch grows and adds a large serial cross-evaluation matrix:
- bolt_merkle_shared: g 1.053 s, H transpose 0.786 s, cross-evaluations 1.386 s; combined 54.9% of 5.875 s mean prover time.
- bolt_canopy_split_quarter: g 1.016 s, H transpose 0.764 s, cross-evaluations 1.733 s; combined 52.4% of 6.701 s mean prover time.

## Source findings

- `src/bolt_multi.rs:59`: g is computed with a serial loop over columns and serial reductions over rows. The preceding eq-weight generation can be parallel, but the large matrix contraction itself is not. Both baseline and Canopy call this shared front.
- `src/bolt.rs:80`: H transpose uses one serial scatter-add loop over all degree-8 edges. The forward syndrome multiplication is parallel; its transpose is not.
- `src/bolt_experiment.rs:211`: the adapter defines a serial dot-product helper. The vendored WHIR already supplies a size-thresholded parallel dot product, but this helper does not use it.
- `src/bolt_batching.rs:268`: cross-evaluations use nested serial loops. Each off-diagonal pair calls Covector::from, allocating a full vector and materializing a covector that is already dense. At eta=8 there are 16 claims in each batch, hence 240 off-diagonal pairs. Baseline has one such batch; split Canopy has one per side, with different vector lengths.
- `src/bolt_multi.rs:310` and `src/bolt_split.rs:219`: inputs are committed sequentially; their opening fronts are also processed sequentially. Transcript order constrains writes, but much of the underlying arithmetic can be parallelized without reordering those writes.
- `src/additive_rs.rs:128`: the additive FFT parallelizes recursive subproblems, but the butterfly sweep within each recursion node is serial. Basis conversion likewise has a serial sweep before recursive joins. At w=4, outer column parallelism alone supplies only four tasks. This also limits WHIR scaling.
- `src/bolt_split.rs:811` and `src/bolt.rs:35`: input generation and public-H setup are serial and contribute to campaign wall time, although excluded from reported prover time.

## CPU configuration and observation

The live process had affinity to all 96 CPUs and 97 threads (96 workers plus the main thread). Its cgroup and all inspected ancestors had unlimited CPU quota (`cpu.max = max 100000`), and zero throttled periods/time. The process active during the sample was WHIR at N=2^26, eta=1; Bolt had just finished, so this sample must not be presented as a Bolt-only profile. Across eight one-second intervals, CPU-equivalent use was 13.84, 30.78, 12.34, 16.95, 19.18, 15.48, 3.70 and 1.01. The last interval had only the main thread busy. Completed Bolt N=2^22 jobs average 8.8–10.7 CPU equivalents across setup, prover and verification.

## Concrete improvements to validate separately

1. Compute g with row-block parallel reductions, accumulating all four columns per block, instead of four serial strided passes. Merely parallelizing four columns would still leave most CPUs unused.
2. Precompute the reverse degree-8 incidence lists of H, then compute H-transpose outputs independently in parallel. This avoids scatter races and avoids allocating a full output vector per worker. It adds approximately 64k bytes of setup storage with usize indices; at N=2^28, w=4, that is 4 GiB. Preserve duplicate edges so cancellation remains identical.
3. Materialize each dense covector once (or borrow its existing vector), parallelize independent off-diagonal evaluations or use the existing parallel dot-product implementation, and write results to the transcript in the original deterministic order. Use bounded/chunked work to control temporary memory.
4. Profile additive-FFT serial sweeps and large-vector allocations after these changes; do not assume removing the first bottlenecks guarantees a particular speedup.

The current AWS-vs-Mac timing gap within scalar stages is measured; its exact division between instruction throughput, cache behavior and NUMA placement has not been isolated. No specific hardware-level cause or post-optimization speedup is claimed. Any optimized campaign should be reported separately or rerun consistently for both Bolt methods; proof bytes, hashes and verification must remain unchanged.
