# canopy-system

Rust implementation and experiments comparing Merkle and Canopy commitments
inside batched WHIR and Bolt-max proximity arguments. The current paper suite
contains **five configurations**, all using **GF(2^256)**, 32-byte field elements,
**BLAKE3-256**, and a **128-bit soundness target**. Reed–Solomon encoding uses
additive FFTs on Cantor-subspace domains, including conversion from monomial
coefficients to the FFT basis. Fiat–Shamir uses a BLAKE3 XOF; there is no proof of work.

## Direct digest-to-field conversion

Canopy now interprets each 32-byte subtree digest directly as a GF(2^256)
field element. This invertible mapping invokes no hash. The corrected AWS
report is [comparison_iop_rounds_direct_digest.csv](results/aws_i4i_optimized/comparison_iop_rounds_direct_digest.csv):
verifier hashes include authentication and Fiat–Shamir only. Proof-size models
and all recorded prover times are unchanged; Canopy timings still include the
obsolete rehash and have not been rerun. Earlier CSVs retain their original
hash accounting for provenance.

## Prepared AWS campaign

The [fairness audit and reporting policy](results/aws_i4i_fair/README.md) records
the implementation audit. Since input oracles are committed in different IOP
rounds, all five reported commitment times use `eta * commitment(N, eta=1)`.
Opening times remain measured at the requested eta; total time is recomputed.
The original sequential-round report is `comparison_iop_rounds.csv`; use the
corrected direct-digest report above for current hash comparisons. Original raw
concurrent commitment measurements are retained separately. Sequential-round
scaling changes timing totals only; the direct-digest correction changes hashes.

The [AWS experiment plan](experiments/aws-r5/README.md) fixes the next campaign
to the two batched WHIR baselines, half-length Canopy–WHIR, the shared-tail
Bolt–Merkle baseline, and Bolt–Canopy with both blocks `k/4` (`w=4`). It contains
70 points: `N=2^22,2^24,2^26` at `eta=1,2,4,8`, plus `N=2^28` at `eta=1,2`.
Use 32 pinned workers on i4i.24xlarge (CPUs `0-15,48-63`), zero warmups and one measured pass per point. The other experiment can use CPUs `16-47,64-95`.
The plan includes the native Intel field backend, correctness checks, resource
records, resumable jobs, and proof-size/verifier-hash accounting. The [optimized AWS campaign](results/aws_i4i_optimized/README.md) restarts with all parallel changes; the [original campaign](results/aws_i4i/README.md) was stopped with its partial results preserved. The older paper
snapshot below is retained separately.

## Latest local experiments: eta = 1

The [parallel prover rerun](results/parallel_prover_n22_eta1/README.md) compares all five
AWS configurations locally at `N=2^22, eta=1`, with eight workers and fresh
before/after controls. It parallelizes Bolt contractions, cross evaluations
and dot products, plus large additive-FFT sweeps; proof parameters are unchanged.

The [half-length WHIR update](results/whir_half_block_eta1/README.md) uses a
Canopy RS block of length `N`, half the rate-1/2 base oracle's `2N`. It reports
proof sizes and verifier hashes for `N=2^18,2^20,...,2^30`, and fresh prover
times only at `N=2^22`. The two batched WHIR baselines remain the controls.
At `N=2^30`, the new Canopy proof is 410.49 KiB with 7,844 verifier hashes;
at `N=2^22`, its measured median prover time is 0.903 s.

The [Bolt k/2 and k/4 rerun](results/bolt_half_quarter_shared_baseline_eta1/README.md)
fixes the baseline to the paper's faster **shared-tail Merkle** configuration.
Canopy uses `k/2` blocks for both parts or `k/4` blocks for both parts, with
independent height optimization and separate systematic/parity WHIR batches.
There are no independent Canopy folds before joining. Proof sizes and verifier
hashes cover the full seven-size grid. All three configurations were timed
together at `N=2^22, eta=1`, using two warmups and ten measured repetitions.
These studies retain the paper snapshot below and provide separate raw data,
reproduction commands, and provenance; no new large-input timing estimates
have been extrapolated.

## The five configurations

| Configuration | Commitments and batching | CSV scheme |
|---|---|---|
| WHIR, rate 1/2 | Merkle commitments; run each input's first transition separately, then batch the resulting claims into one recursive WHIR proof. | `merkle_whir_batched_after_second` |
| WHIR, rate 1/4 | The same batched Merkle baseline with starting RS rate 1/4. | `merkle_whir_batched_rate_quarter_after_second` |
| Canopy + WHIR (Experiment 3) | Starting rate 1/2; batch Canopy SEP claims with the claims from the first WHIR transition, giving `2 eta` source claims in one recursive proof. | `canopy_schwartz_zippel` |
| Bolt + Merkle | Bolt-max with `w=4`; retain the paper's shared WHIR tail, with join message `k/8`. | `bolt_merkle_shared` |
| Bolt + Canopy + WHIR | Bolt-max with `w=4`; both Canopy RS blocks have length `k/2`, where `k=N/4`. Systematic and parity claims use independently optimized Canopy heights and separate recursive WHIR batches. | `bolt_canopy_split` |

Here `N` is the number of field elements **per input polynomial**, and
`eta` is the number of inputs. Each nonconstant Bolt Canopy layer batches its
`eta` SEP claims with that side's `eta` code-switching claims. Full-height
constant layers are opened directly. Matching is required only within each
side; the two sides need not have equal message lengths.

For WHIR, proof sizes optimize folding schedules with three or four variables
per fold and a terminal message of at most `2^10` coefficients. Executable
WHIR timings use three-variable folds. Experiment 3's selected initial fold
is three variables and its aligned Canopy height is four. For Bolt at
`N >= 2^20`, the selected Canopy heights are `(h_X,h_Y)=(2,3)`, with message
lengths `(k/4,k/32)`. The [Bolt protocol notes](docs/experiment_bolt_max.md)
record the small-input exceptions, alignment, and batching details.

## Workloads and measurement conventions

| Quantity | Input sizes `N` | Input counts `eta` |
|---|---|---|
| Proof sizes and verifier hashes | `2^18, 2^20, 2^22, 2^24, 2^26, 2^28, 2^30` | `1, 2, 4, 8` |
| Measured prover times | `2^18, 2^20, 2^22` | `1, 2, 4, 8` |
| Estimated prover times | `2^24, 2^26, 2^28, 2^30` | `1, 2, 4, 8` |

- **Proof size:** mean over 100 deterministic query samples with merged
  authentication paths. Includes roots, authentication digests, queried field
  values, and non-oracle messages. Serializer framing is excluded from the
  size model; Bolt timing records also retain actual serialized proof bytes.
- **Verifier hashes:** authentication + hash-to-field + Fiat–Shamir hash/XOF
  finalizations. These are complete invocations, not BLAKE3 compression calls
  or a verifier-time estimate. FS adds zero proof bytes. Transcript byte counts
  remain in the CSV data; they are omitted from the paper tables.
- **Prover time:** commitment plus opening, excluding input generation,
  public setup, and verification. Commitment covers initial encoding and
  commitment construction; opening includes evaluations, batching, recursive
  commitments, and query openings. Measured components include their FS work.
- **Timing procedure:** MacBook Air M2, 16 GiB RAM, eight Rayon threads,
  Rust 1.92.0 release builds, one warmup, and five measured repetitions.
  Totals are medians; commitment/opening breakdowns are means and need not
  sum to the median total.

WHIR/Canopy timings compose separately measured, verifier-checked native WHIR
components; they do not execute one serialized end-to-end Canopy argument.
Bolt timings generate and verify complete serialized arguments for the
public-`H` adapter, excluding Spark/Mulperm preprocessing and proofs.

The soundness target is conditional on the stated protocol assumptions.
WHIR uses Johnson parameters and a 136-bit local budget to allow composition
at 128 bits. Bolt assumes ensemble distance `gamma=0.013`; its systematic
proximity radius uses the better of `gamma/3` and Khatam's one-and-a-half
Johnson bound with slack. The parity radius remains `1/4`. See the
[Bolt soundness notes](docs/experiment_bolt_max.md#arithmetic-and-soundness)
and [Fiat–Shamir accounting](results/gf256_batched_suite/fiat_shamir/README.md).

## Comparison at N = 2^30, eta = 1

**All times below are estimates in minutes**, extrapolated from the measured
`N=2^22` stages. Commitment/opening are projected means; totals are projected
medians. Proof sizes and hash counts use each configuration's optimized parameters.

| Configuration | Proof (KiB) | Verifier hashes (10³) | Commit (min) | Open (min) | Total (min) |
|---|---:|---:|---:|---:|---:|
| WHIR, rate 1/2 | 503.68 | 11.02 | 2.69 | 3.19 | 5.62 |
| WHIR, rate 1/4 | 338.30 | 7.31 | 3.73 | 4.43 | 7.97 |
| Canopy + WHIR (Exp. 3) | 360.32 | 6.78 | 3.75 | 3.47 | 7.38 |
| Bolt + Merkle | 12074.96 | 357.54 | 0.99 | 1.82 | 2.77 |
| Bolt + Canopy + WHIR | 5315.15 | 118.67 | 1.48 | 2.40 | 3.79 |

The timing model scales linear passes with `N`, FFT stages with `N log2(M)`
for per-column coefficient count `M`, and recursive WHIR work across the
additional rounds. It assumes comparable eight-thread throughput with enough
resident RAM and no paging. A single `N=2^30` input occupies **32 GiB** before
codewords, trees, and temporary buffers, so these are not measurements on the
16 GiB benchmark machine. The [estimation report](results/n30_prover_time_estimates/README.md)
contains the stage model, a smaller-size backtest, and scaling sensitivity.

## Results and paper artifacts

| Artifact | Contents |
|---|---|
| [WHIR results](results/gf256_batched_suite/whir/) | Three configurations: [84 proof/hash rows](results/gf256_batched_suite/whir/proof_sizes/hybrid_proof_sizes.csv), [36 measured timing rows](results/gf256_batched_suite/whir/timing_summary.csv), stage breakdowns, and raw repetitions. |
| [Bolt results](results/gf256_split_half_suite/README.md) | Two configurations: [56 proof/hash rows](results/gf256_split_half_suite/paper_proof_sizes.csv), [24 measured timing rows](results/gf256_split_half_suite/paper_timing_summary.csv), independent height searches, verified proofs, and process memory records. |
| [Timing estimates](results/n30_prover_time_estimates/prover_time_estimates.csv) | 80 estimated timing points covering the four larger sizes, all input counts, and all five configurations. |
| [WHIR report](results/gf256_batched_suite/comparison.md) | Human-readable WHIR comparisons. Its Bolt section describes the earlier shared-Canopy experiment; use the split-half Bolt results above for the current paper. |

The sibling [Canopy manuscript](../canopy/sections/impl.tex) consumes copies
of these data through its [figure/table generator](../canopy/figures/experiments/render.py).
Table 1 uses measured `N=2^22` times in seconds; Table 2 uses estimated `N=2^30`
times in minutes. Both fix `eta=1` and report proof size, verifier hashes,
and commitment/opening/total time.

Timing figures use a **linear relative-time scale**, dividing by rate-1/2
Merkle WHIR or Bolt + Merkle at the same `(N,eta)`. Thus `1.4x` means 40%
slower. Measured segments are solid through `2^22`; estimates beyond it are
dashed. Proof-size and hash curves are solid, and no plots use shading.
The manuscript's `sources.json` records source checksums; its `plotted-results.csv`
retains absolute times and the plotted ratios. The manuscript is a separate
checkout and is not required to run this repository's experiments.

## Reproduce the current suite

Run from the repository root with Rust/Cargo and Python 3. The commands below
use the retained shared-Bolt baseline estimates and reference results under
`results/`; the split-half summarizer also reads the earlier full-block suite
for its auxiliary comparison. Run timings sequentially without other heavy
jobs. Omit `--offline` on the first build if dependencies are not cached.

Build ordinary timing binaries without diagnostic hash instrumentation:

```sh
cargo build --offline --release --bin canopy-system --example gf256_bolt_split
```

Regenerate the three WHIR configurations and record their timing processes:

```sh
python3 scripts/measure_gf256_run.py \
  --record-dir results/gf256_batched_suite/whir/proof_sizes --threads 8 -- \
  target/release/canopy-system paper-estimate --samples 100 \
  --output-dir results/gf256_batched_suite/whir/proof_sizes

for n in 18 20 22; do
  python3 scripts/measure_gf256_run.py \
    --record-dir "results/gf256_batched_suite/whir/timing_n${n}" --threads 8 -- \
    target/release/canopy-system paper-time --log-length "$n" \
    --warmups 1 --repetitions 5 --seed 73947765755993 \
    --output-dir "results/gf256_batched_suite/whir/timing_n${n}"
done
```

Regenerate and time the two selected Bolt configurations with `w=4`, `k/2` blocks:

```sh
python3 scripts/measure_gf256_run.py \
  --record-dir results/gf256_split_half_suite/proof_sizes --threads 8 -- \
  target/release/examples/gf256_bolt_split estimate \
  --width 4 --block-divisor 2 --samples 100 \
  --output results/gf256_split_half_suite/proof_sizes
python3 scripts/run_gf256_split_half_timings.py
```

The Bolt runner resumes completed cases. For fresh measurements, move its completed
`results/gf256_split_half_suite/timing_n*_eta*` directories to an archive before
rerunning. Each case uses the fixed shared-tail Merkle baseline and the split
Canopy configuration.

After timing, regenerate and validate transcript accounting, then rebuild reports:

```sh
cargo build --offline --release --features fs-accounting \
  --example fiat_shamir_accounting --example gf256_bolt_split
RAYON_NUM_THREADS=8 target/release/examples/fiat_shamir_accounting --validate
python3 scripts/summarize_gf256_suite.py
RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_split validate \
  --input results/gf256_split_half_suite/proof_sizes/proof_sizes.csv \
  --output results/gf256_split_half_suite/fiat_shamir_validation.csv
python3 scripts/summarize_gf256_split_half_suite.py
python3 scripts/estimate_n30_prover_times.py
```

The accounting checks execute additional proofs but do not replace saved timing
samples. Rebuild without `fs-accounting` before taking more timings. The final
estimation script only reads saved measurements and writes extrapolations;
it does not run large-input proofs. To refresh summaries from the existing
checked-in data, only the three Python summarization/estimation commands above
are needed.

## Code map

- [Field](src/binary_field.rs) and [additive RS](src/additive_rs.rs): GF(2^256),
  ARM PMULL/portable arithmetic, additive FFTs, and transposed encoding.
- [Merkle](src/merkle.rs) and [Canopy](src/canopy.rs): commitments,
  authentication paths, and the Canopy SEP reduction.
- [WHIR experiments](src/paper_experiments.rs), [executable bridge](src/executable_whir.rs),
  and [vendored WHIR](vendor/whir/): proof models and component timing.
- [Bolt encoder](src/bolt.rs), [split experiment](src/bolt_split.rs), and
  [batching](src/bolt_batching.rs): the current public-`H` adapter.
- [WHIR transcript model](src/paper_fiat_shamir.rs) and
  [Bolt transcript model](src/bolt_fiat_shamir.rs): verifier hash accounting.
- [CLI](src/main.rs) and [Bolt suite CLI](examples/gf256_bolt_split.rs):
  executable entry points. Library checks run with `cargo test --offline --lib`.

## Historical experiments

The [archived README](docs/archive/readme_legacy_experiments.md) preserves the
old standalone-commitment experiments, 128-bit-field/capacity-model results,
and their commands. The [192-bit field-switch experiment](docs/archive/experiment4_field_switch.md)
is retired. [Width-16 Bolt](results/gf256_split_w16_suite/README.md),
[full-block split Bolt](results/gf256_split_suite/README.md), and
[earlier unbatched comparisons](results/gf256_suite/comparison.md) remain as
historical results; they are not additional configurations in the paper suite.
