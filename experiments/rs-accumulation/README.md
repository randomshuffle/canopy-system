# Reed–Solomon accumulation

This experiment measures one steady-state accumulation step at fixed RS
codeword lengths. It compares **Canopy `ifold` + `iacc`** with ordinary Merkle
batching at rates **1/2 and 1/4**. The reference uses `N=2^24` field elements,
fresh arities `ell_cast=1,2,4,8`, and `ell_acc=1`.

Each step retains one Canopy SEP claim and one Merkle SEP claim:

```text
ell_cast fresh Canopy claims + previous Canopy accumulator
    -- ifold --> next Canopy accumulator + (ell_cast + 1) Merkle residuals

(ell_cast + 1) Merkle residuals + previous Merkle accumulator
    -- iacc --> next Merkle accumulator
```

Both prior accumulators participate in the next step. With `ell_acc=1`,
`ifold` has `ell_cast+1` sources and `iacc` has `ell_cast+2`. The Merkle controls
batch `ell_cast+1` full-length claims in their original code.

The field is GF(2²⁵⁶), with BLAKE3-256 and eight field elements per authenticated
leaf. The Canopy codeword has half the input codeword length. An analytic height
search minimizes expected **combined verifier hashes for `ifold + iacc`**;
heights are selected before timing. The target is 128-bit round-by-round
security with Johnson slack 1.05. The saved affine-MCA and degree-scaled
security diagnostics are local checks; they do not establish a proved
end-to-end 128-bit theorem for accumulation by powers of one challenge.

## Results and accounting

[`accumulation_comparison.csv`](../../results/r7a/accumulation_comparison.csv)
contains 24 measured configurations: three methods, four fresh arities, and
two worker counts. Its schemes are `canopy_ifold_iacc`, `merkle_rate_half`, and
`merkle_rate_quarter`. Selected estimates and all height candidates are in
[`estimates/accumulation/`](../../results/r7a/estimates/accumulation).

Total prover time is **fresh commitments + reduction**. Canopy reduction time
is the paired sum of `ifold` and `iacc` from the same repetition. Prior
commitments, input generation, public setup, verification, and the final
membership decider are excluded. Each measured reduction is verified. Independent
repetitions prepare representative valid prior Batch claims outside the timers;
protocol tests also carry both actual outputs across consecutive steps.

Expected proof bytes exclude serializer framing and returned output-instance
wire data. Measured serialized bytes and verifier hash counts are separate CSV
columns. The two reductions' fresh output data occupy 96 bytes in total;
these are additional to modeled proof payload, not the final accumulator size.
Raw reduction samples and exclusive phase profiles are in
[`timings/`](../../results/r7a/timings), under `t*_accum_n24_*`.

## Small local reproduction

Run from the repository root after the native release build in the main README.
Use fresh output paths for each run.

```sh
target/release/examples/accumulation_section8 estimate \
  --logs 16 --ell-cast 1,2 --ell-acc 1 \
  --log-leaf-width 3 --security-bits 128 \
  --output-dir results/reproduce/accumulation/estimates

RAYON_NUM_THREADS=1 target/release/examples/accumulation_section8 time \
  --logs 16 --ell-cast 1,2 --ell-acc 1 \
  --log-leaf-width 3 --security-bits 128 \
  --warmups 0 --repetitions 1 --check-tampering \
  --output results/reproduce/accumulation/timings.csv

python3 scripts/report_section8.py \
  --estimates results/reproduce/accumulation/estimates \
  --timings results/reproduce/accumulation/timings.csv \
  --output-dir results/reproduce/accumulation/report
```

The timing command runs all three methods, selects the optimized height, and
checks the proofs. The reporter pairs reduction rows, checks source counts
and fresh commitment costs, and rejects unverified samples. The small pilot
exercises the same recurrence as the reference, at `N=2^16`.

Use `python3 scripts/reproduce.py estimates` for the reference `N=2^24` height
search and `python3 scripts/reproduce.py run` for the full native campaign.
See the [protocol notes](../../docs/experiment_section8.md) for exact statement
and proof boundaries, and the [profiling guide](../../docs/experiment_section8_profile.md)
for exclusive phase measurements.

## Implementation

- [`section8.rs`](../../src/section8.rs): `ifold`, `iacc`, and Merkle controls.
- [`section8_estimator.rs`](../../src/section8_estimator.rs): height search and
  analytic proof/hash estimates.
- [`section8_codeword.rs`](../../src/section8_codeword.rs): combined codeword and
  leaf-hashing kernels.
- [`accumulation_section8.rs`](../../examples/accumulation_section8.rs): estimate
  and timing CLI.
- [`section8_tests.rs`](../../src/section8_tests.rs) and related test modules:
  public verification, tampering, and carried accumulator checks.
