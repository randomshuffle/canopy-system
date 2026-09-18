# Five-configuration AWS campaign

Approved grid: **70 points**, on one **i4i.24xlarge**, **32 Rayon workers pinned to CPUs `0-15,48-63`**, **zero warmups and one measured pass per point**. Campaign jobs run sequentially. The user may run another experiment concurrently on CPUs `16-47,64-95`; memory and some cache/bandwidth resources remain shared. There are 28 executable jobs because the WHIR harness reports three methods together and the Bolt harness reports two.

| N (field elements per input) | eta (inputs) | Configuration points |
|---|---|---:|
| 2^22 | 1, 2, 4, 8 | 20 |
| 2^24 | 1, 2, 4, 8 | 20 |
| 2^26 | 1, 2, 4, 8 | 20 |
| 2^28 | 1, 2 | 10 |

**No 2^30 jobs.** All methods use GF(2^256), BLAKE3-256, and the existing composed 128-bit security target. Local WHIR subarguments use 136 bits. Commit and opening times both include executed Fiat–Shamir work; input generation, setup, and verification are excluded from reported prover time and included in process wall time/RSS.

The current headline commitment policy models sequential IOP rounds: for all
five schemes use C(N, eta) = eta * C(N, 1), retain measured openings, and recompute
totals. `comparison_iop_rounds.csv` is the reported comparison; `comparison.csv`
and per-job samples preserve raw measurements. This reporting policy supersedes
concurrent commitment timings for comparisons, without changing the active runs.

## Exactly five methods

1. **Batched WHIR, rate 1/2.** Independent first transitions, batching from the second oracle onward.
2. **Batched WHIR, rate 1/4.** Same batching convention.
3. **Batched Canopy–WHIR (Exp. 3), half-length Canopy codeword.** Base codeword has 2N field elements and the Canopy codeword has N. The timing harness uses first-fold log 3 and height 4, giving a Canopy message of N/8 and a shared tail starting at rate 1/8. The 2 eta claims use the existing distinct-functional Schwartz–Zippel batching construction.
4. **Bolt-max + Merkle, w=4, fixed shared WHIR tail.** This is the faster old paper baseline, pinned with `--paper-only --shared-merkle-baseline`; it is not replaced by the slightly smaller-proof split baseline. k=N/4 and the shared join message has length k/8.
5. **Bolt-max + Canopy, w=4, both blocks k/4=N/16.** Optimize the systematic and parity heights independently, retaining separate recursive batches for the two sides. Each Canopy claim joins its own side's code-switch claim immediately at its aligned message size. No delayed systematic join; no k/2 candidate.

Bolt retains the explicitly assumed distance gamma=0.013 over GF(2^256), the previously implemented Khatam general-linear-code proximity radius when better than gamma/3, and the existing separate parity distance analysis. It is the public-H adapter experiment and excludes Spark/Mulperm preprocessing and proofs. The raw estimator retains auxiliary baseline/layout candidates needed by its existing API; only the five methods above enter the headline CSV.

## Measurements and accounting

- **Proof size:** mean bytes and KiB from 100 deterministic query samples, merging authentication paths. Small serializer framing is excluded from the model. Bolt additionally records actual serialized proof sizes for every measured proof.
- **Verifier hashes:** authentication leaf/parent hashes + Fiat–Shamir hash/XOF finalizations. Digest-to-field conversion is a direct byte bijection and adds no hashes. Existing campaign snapshots predate this correction. This is a count of finalizations, not BLAKE3 compression calls. Component counts and transcript absorb/squeeze bytes remain in the CSV. Fiat–Shamir adds no proof bytes in this implementation.
- **Prover:** one raw timing sample per configuration; commitment/opening and total (the existing mean/median/min/max CSV fields coincide). Each Bolt proof is serialized and verified. WHIR retains the existing **component-composed timing harness**: native WHIR components are verified, all initial encodings and Merkle commitments run concurrently across eta, with the exact same measured half-rate base encoding charged to both WHIR 1/2 and Canopy. Opening-front components are scaled by eta for all three schemes, and batched tails run at eta. It does not serialize one complete Exp. 3 argument. Process wall time is not a prover-time sample.
- WHIR headline proof sizes retain the existing optimized folding schedules over {3,4}, terminal message at most 2^10. Its timing harness retains the fixed three-variable folding schedule. These are separately labelled methodologies, not a claim that the timed proof uses the proof-minimizing schedule.
- Bolt height choices are frozen by the proof search before timings. Its two schemes rotate execution order each repetition; WHIR component execution order is fixed by the existing harness.

## Build and run on the supplied AWS host

Requirements: Linux x86_64, Rust **1.92.0**, Cargo, C linker/build tools, Git, Python 3.10+, and network access for the initial locked dependency fetch. `Cargo.lock`, the vendored WHIR changes, and the pinned Spongefish git revision are included. There is no automatic instance provisioning or SSH in this runner.

From the extracted `canopy-system` directory:

```sh
# Install the pinned Rust toolchain if absent.
rustup toolchain install 1.92.0 --profile minimal

# Builds in target/aws-native; normal (uninstrumented) release binaries only.
# Uses -C target-cpu=native on the actual AWS processor.
python3 scripts/aws_benchmark.py build
python3 scripts/aws_benchmark.py plan
python3 scripts/aws_benchmark.py prepare

# Pilot: all five configurations at 2^22, eta=1,2,4,8; includes the final samples.
python3 scripts/aws_benchmark.py run --only-log 22

# Inspect pilot throughput/RSS, then continue. Completed identical jobs are skipped.
python3 scripts/aws_benchmark.py run
python3 scripts/aws_benchmark.py report
```

`run --only-log 24` (or 26, 28) can also advance by size. The jobs are ordered by increasing N and eta. A default output directory is `results/aws_r5`; `--output` selects a new campaign directory. Do not reuse a directory after changing binaries, sources, or settings.

Preparation has no large input allocations and can also run locally with `--bin-dir target/release`; the resulting `proof_sizes_and_hashes.csv` is hardware-independent. It has exactly 70 rows. Each family's raw geometry, candidate searches, and hash ledgers are retained under `estimates/n*/`.

To create a source bundle from the working tree, including uncommitted implementation changes:

```sh
python3 scripts/aws_benchmark.py bundle --output results/aws_r5_prepared
```

This produces `canopy-aws-source.tar.gz` and a SHA-256 manifest. It includes the benchmark sources, vendored Rust sources, scripts and locked dependency metadata; it excludes `.git`, compiled binaries, old result directories, SSH keys and unrelated files. Dependencies are fetched on the host by `cargo --locked`; this is not an offline dependency cache.

## Intel backend and validation

The field implementation now uses Intel PCLMULQDQ when compiled with that target feature. Portable builds still use the reference fallback. The native build runs the field tests, additive-FFT/transposed-FFT tests, serial-reference Bolt contraction tests, quarter-block geometry/binding tests, and the parallel proof-transcript equivalence test. `aws_field_check --require-intel` rejects a portable or FS-instrumented timing binary and checks all 16,384 basis products against the independent bit-serial multiplication/reduction reference. The campaign records the CPU topology, effective CPU affinity, memory, compiler version, source hashes and hashes of all five executables.

The Intel instructions cannot be executed on the local ARM machine. The x86 backend must pass these mandatory tests on AWS before timings are accepted. The ARM backend and local harness checks are independently testable now.

## CPU allocation, RAM, runtime and failures

AWS specifies 96 vCPUs / 48 physical cores and 768 GiB for i4i.24xlarge: https://docs.aws.amazon.com/ec2/latest/instancetypes/so.html . This restart uses 32 logical CPUs, both hardware threads of 16 physical cores on NUMA node 0. CPU affinity is enforced for builds, tests and benchmark children. The remaining 64 logical CPUs (`16-47,64-95`) are available for the user’s other experiment.

A current local WHIR probe at N=2^22, eta=2 peaks at 2.168 GiB after releasing unused front buffers before subsequent components. Linear scaling gives about **139 GiB** at N=2^28, eta=2. The retained Bolt half-block experiment's measured 1.307 GiB at N=2^22, eta=2 scales to about **84 GiB** at that point; this is only a planning proxy for the new quarter-block campaign. Other relevant measured peaks and their scope are kept with the local probes. Allocator behavior and 96-worker temporary storage can change these estimates; the pilot measures actual host usage before larger allocations.

The old stage projections sum to approximately **3.1 hours of prover work for four passes over this reduced grid**, at old eight-thread ARM throughput and old Canopy geometry. That is a planning reference, not an AWS forecast: it excludes setup, verification, duplicated component-harness work and memory effects. The new half-length WHIR and quarter-length Bolt geometry differ from that projection. We will use pilot wall times to update scheduling estimates; no 12x speedup is assumed from the CPU count.

The Linux runner requires swap disabled and at least 640 GiB available at campaign start, sets a 640 GiB address-space limit per process, and stops the campaign if available host RAM drops below 32 GiB or swapping is observed. It never substitutes an estimated timing for a failed measurement. Every job has a flushed log and a process record; successful identical jobs are resumable. Failed/interrupted jobs remain visible and require a new output directory to retry, preserving the original failure evidence.

`comparison.csv` contains all 70 proof/hash rows and any completed timing summaries. Unrun points explicitly say `not_measured`. `process_peak_rss_gib` is a **whole grouped-job peak**, including setup and verification, not per-method memory. All raw proof/timing/stage CSVs and process logs remain available.

The campaign now runs on the user-supplied i4i.24xlarge (Intel Xeon 8375C); the historical `experiments/aws-r5` path is retained. The grid is unchanged. The original four-pass campaign was stopped at 55 completed points at the user’s request; its partial results remain separate. This restart uses the parallel prover and one pass per point. Single-pass timings do not measure run-to-run variability.
