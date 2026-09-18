# canopy-system

Rust implementation and experiments comparing Merkle and Canopy commitments
inside batched WHIR and Bolt-max proximity arguments. The final suite contains
**five configurations**, all using **GF(2^256)**, 32-byte field elements,
**BLAKE3-256**, and a **128-bit soundness target**. Reed–Solomon encoding uses
additive FFTs on Cantor-subspace domains, including conversion from monomial
coefficients to the FFT basis. Fiat–Shamir uses a BLAKE3 XOF, with no proof of work.

Canopy interprets each 32-byte subtree digest directly as a GF(2^256) element.
This invertible conversion invokes no hash; current verifier hash counts include
only authentication and Fiat–Shamir finalizations.

## The final five configurations

Here `N` is the number of field elements per input polynomial, `eta` is the
number of inputs, and Bolt uses `w=4` with `k=N/4`.

| Configuration | Commitments and batching | CSV scheme |
|---|---|---|
| WHIR, rate 1/2 | Merkle commitments; execute each input's first transition separately, then batch the resulting claims into one recursive WHIR proof. | `merkle_whir_batched_after_second` |
| WHIR, rate 1/4 | The same batched Merkle baseline with starting RS rate 1/4. | `merkle_whir_batched_rate_quarter_after_second` |
| Canopy + WHIR, half-length block | Starting rate 1/2: the base oracle has length `2N`, and the Canopy RS codeword has length `N`. Batch the `eta` Canopy SEP claims with the `eta` first-transition WHIR claims into one recursive proof. | `canopy_schwartz_zippel` |
| Bolt + Merkle, shared tail | Bolt-max with `w=4`, the fixed shared WHIR tail, and join message length `k/8`. | `bolt_merkle_shared` |
| Bolt + Canopy, quarter-length blocks | Bolt-max with `w=4`; systematic and parity Canopy RS blocks both have length `k/4=N/16`. Optimize their heights independently and use a separate recursive WHIR batch for each side. | `bolt_canopy_split_quarter` |

For Canopy + WHIR, the timing harness uses a three-variable initial fold and
Canopy height four, giving a Canopy message of length `N/8` and a shared tail
starting at rate 1/8. The `2 eta` source claims use the existing
Schwartz–Zippel batching construction.

For Bolt + Canopy, each nonconstant Canopy claim joins its own side's
code-switch claim immediately at the aligned message size. There are no
independent Canopy folds before joining and no delayed systematic join.
Full-height constant layers are opened directly. The Merkle control always
uses the shared-tail baseline. The executable selection is fixed by
`--paper-only --shared-merkle-baseline`, with Canopy estimates generated using
`--width 4 --block-divisor 4`.

The configuration lists and workload grids are in
[config.json](experiments/aws-r5/config.json) and
[config-n30-eta1.json](experiments/aws-r5/config-n30-eta1.json).

## Measured results

The completed AWS experiments use an i4i.24xlarge with 32 Rayon workers pinned
to CPUs `0-15,48-63`, Rust 1.92.0, and native release builds. The main campaign
and its `N=2^30` extension use zero warmups and one measured pass per point.

| Dataset | Input sizes `N` | Input counts `eta` | Configuration points |
|---|---|---|---:|
| Main campaign | `2^22, 2^24, 2^26` | `1, 2, 4, 8` | 60 |
| Main campaign | `2^28` | `1, 2` | 10 |
| Large-input extension | `2^30` | `1` | 5 |

The following values come from the completed
[`N=2^30, eta=1` comparison](results/aws_i4i_n30_eta1/remote/comparison_iop_rounds.csv).
**Times are measured seconds.** Proof sizes and verifier hash counts are means
over 100 deterministic query samples.

| Configuration | Proof (KiB) | Verifier hashes | Commit (s) | Open (s) | Total prover (s) |
|---|---:|---:|---:|---:|---:|
| WHIR, rate 1/2 | 503.68 | 11,022.73 | 153.18 | 137.64 | 290.83 |
| WHIR, rate 1/4 | 338.30 | 7,312.48 | 253.55 | 242.13 | 495.68 |
| Canopy + WHIR, half-length block | 410.49 | 7,526.64 | 202.95 | 160.85 | 363.80 |
| Bolt + Merkle, shared tail | 12,074.96 | 357,539.79 | 34.73 | 87.12 | 121.85 |
| Bolt + Canopy, quarter-length blocks | 6,002.74 | 116,768.93 | 41.45 | 96.41 | 137.86 |

The large-input extension uses direct digest conversion in both the timed
binaries and the hash accounting. Its
[report](results/aws_i4i_n30_eta1/README.md) and
[validation receipt](results/aws_i4i_n30_eta1/remote/completion-validation.json)
record the completed measurements and proof checks.

Separate repeated Bolt checks at
[`N=2^22`](results/aws_i4i_bolt_n22_recheck_20260917/README.md) and
[`N=2^24`](results/aws_i4i_bolt_n24_recheck_20260917/README.md) cover all four
input counts using direct digest conversion, one discarded warmup, and four
measured passes with alternating scheme order. All 64 measured serialized
proofs verified. These repeated checks did not reproduce the earlier
single-pass apparent Canopy speedup; their results remain separate from the
original campaign.

## Measurement and accounting conventions

- **Proof size:** mean over 100 deterministic query samples with merged
  authentication paths, including roots, digests, queried field values, and
  non-oracle messages. The model excludes serializer framing; Bolt timing
  records additionally retain actual serialized proof bytes.
- **Verifier hashes:** authentication plus Fiat–Shamir hash/XOF finalizations.
  Direct digest-to-field conversion contributes zero hashes. Counts refer to
  complete invocations, not BLAKE3 compression calls. Fiat–Shamir adds no proof
  bytes; transcript byte counts remain in the CSVs.
- **Prover time:** commitment plus opening, including executed Fiat–Shamir work
  and excluding input generation, public setup, and verification. Those
  excluded phases still contribute to process wall time and peak RSS.
- **Sequential IOP rounds:** for all five configurations, reported commitment
  time is `C(N, eta) = eta * C(N, 1)` for the same scheme and input size.
  Opening time is measured at the requested `eta`; total time is their sum.
  For `eta > 1`, these totals are derived from measured components. Raw
  commitment measurements remain in the per-job data.
- **Timing scope:** WHIR/Canopy composes separately measured, verifier-checked
  native WHIR components. Bolt generates and verifies complete serialized
  arguments for the public-`H` adapter, excluding Spark/Mulperm preprocessing
  and proofs. Single-pass results do not characterize run-to-run variability.

WHIR proof-size searches optimize schedules with three or four variables per
fold and a terminal message of at most `2^10` coefficients. Executable timings
use three-variable folds. Proof-minimizing estimates and timed components
therefore have separately recorded parameters.

The composed 128-bit soundness target is conditional on the protocol
assumptions. WHIR uses Johnson parameters with a 136-bit local budget. Bolt
assumes ensemble distance `gamma=0.013` over GF(2^256) and retains separate
systematic and parity proximity analyses. See the
[protocol and soundness notes](docs/experiment_bolt_max.md#arithmetic-and-soundness)
for these assumptions and the public-`H` scope.

## Result files

| Artifact | Contents |
|---|---|
| [Main 70-point comparison](results/aws_i4i_optimized/comparison_iop_rounds_direct_digest.csv) | Final five configurations with sequential-round timing and corrected direct-digest hash accounting. The original Canopy timings still include the removed rehash; these timings have not been rerun. |
| [Measured `N=2^30` extension](results/aws_i4i_n30_eta1/remote/comparison_iop_rounds.csv) | Five measured points at `eta=1`, using direct digest conversion throughout. |
| [Bolt `N=2^22` recheck](results/aws_i4i_bolt_n22_recheck_20260917/remote/results/summary_iop_rounds.csv) | Repeated timing means for both Bolt configurations at all four input counts. |
| [Bolt `N=2^24` recheck](results/aws_i4i_bolt_n24_recheck_20260917/remote/results/summary_iop_rounds.csv) | The corresponding repeated checks at `N=2^24`. |
| [Reporting audit](results/aws_i4i_fair/README.md) | Commitment scheduling audit and the sequential-IOP reporting policy. |

Earlier full-length WHIR, Bolt `k/2` and full-block studies, and extrapolated
local timing tables remain under `results/` as historical data. Their geometry,
hash accounting, and hardware differ from the final suite. The
[archived README](docs/archive/readme_legacy_experiments.md) documents older
standalone-commitment and field experiments.

## Reproduce the final suite

Run from the repository root. The campaign runner requires Linux x86_64,
Rust 1.92.0, Cargo, Python 3.10+, and a C linker/build toolchain. Its supplied
configuration targets the CPU allocation above and requires sufficient memory
for the large workloads. The preflight checks enforce CPU affinity, native
field arithmetic, available RAM, and disabled swap. Use a fresh output directory
for new measurements; historical source and binary hashes describe their
original runs.

```sh
rustup toolchain install 1.92.0 --profile minimal

# Build, validate, prepare, measure, and report the 70-point grid.
set -e
campaign_config=experiments/aws-r5/config.json
campaign_output=results/reproduce_five_configs
for action in build plan prepare run report; do
  python3 scripts/aws_benchmark.py "$action" \
    --config "$campaign_config" --output "$campaign_output"
done
```

For the measured `N=2^30, eta=1` workload, repeat the same loop with
`campaign_config=experiments/aws-r5/config-n30-eta1.json` and a separate
`campaign_output=results/reproduce_n30_eta1`. Add `--offline` once dependencies
are cached. `run --only-log 22` can be used for the smaller-input pilot before
continuing the remaining jobs. Successful jobs with identical provenance are
resumable.

The runner applies the half-length WHIR codeword and quarter-length Bolt blocks
explicitly, generates the fixed shared-tail Merkle baseline, validates native
field/protocol behavior, and writes `proof_sizes_and_hashes.csv`, raw
`comparison.csv`, and the sequential-round `comparison_iop_rounds.csv`.
Fresh builds already use direct digest conversion; the legacy correction script
is only for the older 70-point dataset and must not be reapplied to these results.
See the [campaign guide](experiments/aws-r5/README.md) for resource limits,
per-job records, and source bundling.

## Code map

- [Field arithmetic](src/binary_field.rs) and [additive RS](src/additive_rs.rs):
  GF(2^256), ARM PMULL, Intel PCLMULQDQ, portable arithmetic, additive FFTs,
  and transposed encoding.
- [Merkle](src/merkle.rs) and [Canopy](src/canopy.rs): commitments,
  authentication paths, and the Canopy SEP reduction.
- [WHIR experiments](src/paper_experiments.rs),
  [executable bridge](src/executable_whir.rs), and [vendored WHIR](vendor/whir/):
  proof-size models and component timings.
- [Bolt encoder](src/bolt.rs), [split experiment](src/bolt_split.rs), and
  [batching](src/bolt_batching.rs): the public-`H` adapter and the selected
  shared-tail Merkle / quarter-block Canopy comparison.
- [WHIR transcript model](src/paper_fiat_shamir.rs) and
  [Bolt transcript model](src/bolt_fiat_shamir.rs): verifier hash accounting.
- [Campaign runner](scripts/aws_benchmark.py), [main CLI](src/main.rs), and
  [Bolt CLI](examples/gf256_bolt_split.rs): reproduction entry points.

Library checks run with `cargo test --offline --lib`. Timing builds omit the
diagnostic `fs-accounting` feature.
