# Canopy

Experiments comparing **Canopy and Merkle commitments** in Bolt, WHIR, and
Reed–Solomon accumulation. The implementation uses Rust, GF(2²⁵⁶), additive
Reed–Solomon encoding, and BLAKE3-256.

| Experiment | What is compared | Start here |
|---|---|---|
| **Bolt** | The shared-tail Merkle baseline and black-box Canopy commitments for Bolt's input oracles. | [Protocol, results, and reproduction](experiments/bolt/README.md) |
| **WHIR** | Batched Merkle WHIR at rates 1/2 and 1/4, and Canopy–WHIR with a half-length Canopy codeword. | [Protocol, results, and reproduction](experiments/whir/README.md) |
| **RS accumulation** | A steady-state Canopy `ifold` + `iacc` step and Merkle accumulation at rates 1/2 and 1/4. | [Protocol, results, and reproduction](experiments/rs-accumulation/README.md) |

The [reference results](results/README.md) contain the completed October 28,
2026 campaign: **135 Bolt/WHIR configurations and 24 accumulation configurations**,
with raw samples, parameter searches, and build provenance. Superseded
campaigns and exploratory command-line interfaces have been removed.

## Getting started

Local reproduction supports macOS and Linux. You need Rust **1.92.0**,
Python **3.10+**, Git, and a C compiler/linker.
Python runners use the standard library. The first build needs network access;
subsequent builds can use `--offline`. Rust dependencies are locked in
[`Cargo.lock`](Cargo.lock), WHIR is vendored, and Spongefish is pinned to a Git
revision.

```sh
rustup toolchain install 1.92.0 --profile minimal --component rustfmt

# Check the retained files and reconstruct all reported results from raw samples.
python3 scripts/validate_results.py

# Regenerate all proof-size/hash estimates; no large input polynomials are allocated.
python3 scripts/reproduce.py estimates --output results/reproduce/estimates
```

The estimator command builds the executables, runs the 75 Bolt/WHIR model
points and the accumulation height search, then checks every exported summary
field against the reference. Commands, logs, source/binary hashes, and the
comparison receipt are saved in the output directory. Use a new directory for
each run; generated output under `results/reproduce/` is ignored by Git.

For a small, verifier-checked timing run on your own machine, build the
executables and follow one of the three experiment guides above:

```sh
RUSTFLAGS="-C target-cpu=native" cargo build --locked --release --bin canopy-system --examples
cargo test --locked --lib
```

The guides include small pilots (`N=2^16` for Bolt and accumulation,
`N=2^18` for WHIR). They exercise the full measurement path at a smaller size; their timings are not the reference measurements.

## Reproducing the measured campaign

The reference host was an **AWS r7a.32xlarge**, with an AMD EPYC 9R14, 1 TiB RAM,
and native release builds. Jobs ran sequentially with either **1 or 32 Rayon
workers**, pinned to CPUs `0` or `0–31`. The full runner requires Linux x86_64,
`numactl`, `lscpu`, disabled swap, and enough RAM for the large workloads. It
checks field arithmetic, protocol tests, CPU affinity, and memory before accepting
measurements.

```sh
# Inspect the exact 78 jobs before running the memory-intensive campaign.
python3 scripts/reproduce.py plan --output results/reproduce/campaign
python3 scripts/reproduce.py run --output results/reproduce/campaign
```

[`experiments/r7a.json`](experiments/r7a.json) records the completed grid and
per-job sampling policy:

| Family | 32 workers | 1 worker |
|---|---|---|
| Bolt and WHIR | `N=2^22, 2^24, 2^26`, `eta=1,2,4,8`; `N=2^28`, `eta=1,2`; `N=2^30`, `eta=1` | `N=2^22, 2^24, 2^26`, `eta=1,2,4,8` |
| RS accumulation | `N=2^24`, fresh arity `1,2,4,8`, one prior accumulator per relation | Same grid |

The saved campaign has mixed sampling: 108 configurations use zero warmups
and one measurement; 51 use one warmup and two measurements. The runner
reproduces those choices. For repeated performance studies, copy the config,
set `warmups` and `repetitions`, clear `sampling_by_job`, and pass `--config`.
CPU affinity and memory limits are also explicit configuration fields.

Successful jobs resume only when their source, executable, and configuration
fingerprints match. Failed or interrupted jobs require inspection and a fresh
output directory. Timing values depend on the machine and measurement conditions;
proof/hash models use deterministic samples.

To rebuild the tables from the retained raw measurements without compiling Rust:

```sh
python3 scripts/reproduce.py report \
  --input results/r7a --output results/reproduce/report
```

## Reading the results

- **Proof size** is a model expectation over 100 deterministic query samples
  for Bolt/WHIR, or an analytic expectation for accumulation. Models exclude
  serializer framing. Measured serialized sizes are retained where available.
- **Verifier hashes** count authentication and Fiat–Shamir hash/XOF
  finalizations. Direct conversion of a 32-byte digest to GF(2²⁵⁶) adds no hash.
- **Bolt/WHIR prover time** reports commitment as
  `C(N, eta) = eta × C(N, 1)` and adds the measured opening at that `eta`.
  The CSV also preserves raw commitment and total timings. WHIR timings compose
  separately measured, verified native components; Bolt verifies complete
  serialized public-`H` arguments and excludes Spark/Mulperm preprocessing.
- **Accumulation prover time** includes fresh commitments plus `ifold + iacc`
  (or the Merkle control's reduction). Prior commitments, verification, and the
  final membership decider are excluded. Both accumulator claims are retained
  for the next step.

This is a research implementation. Bolt/WHIR use a 128-bit composed target
with a 136-bit local WHIR budget; Bolt also assumes code distance
`gamma=0.013`. Accumulation uses a 128-bit round-by-round target and records
local soundness diagnostics, without claiming a proved end-to-end 128-bit
accumulation theorem. The experiment guides explain these boundaries.

## Repository map

| Path | Contents |
|---|---|
| [`src/`](src) | Field arithmetic, additive RS, commitments, protocol implementations, estimators, and tests. |
| [`examples/`](examples) | Bolt and accumulation CLIs, WHIR transcript accounting, and arithmetic/encoder checks. |
| [`scripts/`](scripts) | Reproduction, result validation, and accumulation profiling/reporting. |
| [`experiments/`](experiments) | The three experiment guides and the reference campaign configuration. |
| [`results/r7a/`](results/r7a) | Curated reference data, raw samples, and the original measured source snapshot. |
| [`vendor/whir/`](vendor/whir) | WHIR dependency with the experiment's local modifications and upstream licenses. |

First-party code is [MIT licensed](LICENSE). The vendored WHIR code retains
its [MIT](vendor/whir/LICENSE-MIT) and [Apache-2.0](vendor/whir/LICENSE-APACHE)
license files. See [public distribution notes](docs/publication.md) for the
scope of this cleanup and the distinction between files and Git history.
