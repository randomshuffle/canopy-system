# WHIR

This experiment compares three batched proximity arguments over GF(2²⁵⁶):

| Method | Construction | `scheme` |
|---|---|---|
| Merkle, rate 1/2 | Each input executes its first transition; the resulting claims share a recursive WHIR proof. | `merkle_whir_batched_after_second` |
| Merkle, rate 1/4 | The same batching strategy with a longer initial RS codeword. | `merkle_whir_batched_rate_quarter_after_second` |
| Canopy–WHIR | Rate-1/2 input codeword of length `2N`, Canopy codeword of length `N`; batch the Canopy SEP and first-transition claims. | `canopy_schwartz_zippel` |

`N` is the message length in field elements; `eta` is the input count. The
Canopy construction batches `2 eta` source claims with Schwartz–Zippel batching.
It uses additive Cantor-subspace RS encoding, 32-byte field elements,
BLAKE3-256 commitments, a BLAKE3 XOF transcript, and no proof of work.

RS list decoding uses BCGM26 with `m=10`, radius
`1 - 1.05 sqrt(rate)`, and affine MCA error
`10.5^7 × n_RS^2 / (3 × rate^(3/2) × |F|)`. A multilinear fold incurs one
such error per coordinate. The local budget is 136 bits for a composed
128-bit target. Unique-decoding paths retain their stronger bounds.

## Results and timing scope

The three WHIR schemes account for 81 rows of
[`snark_comparison.csv`](../../results/r7a/snark_comparison.csv).
[`estimates/n22/whir/`](../../results/r7a/estimates/n22/whir) and the corresponding
`n24`, `n26`, `n28`, and `n30` directories retain parameter grids, selected
schedules, proof-component breakdowns, and Fiat–Shamir ledgers.

**The timing harness composes separately measured, verifier-checked native
WHIR components.** It does not serialize a complete Canopy–WHIR argument.
It shares the measured base RS encoding between half-rate Merkle and Canopy,
scales first-transition opening work by `eta`, and measures batched recursive
tails at the requested `eta`. The headline comparison uses sequential
commitment cost `C(N,eta)=eta × C(N,1)`; raw batch commitment timings are retained.

Proof-size searches optimize folds of three or four variables and a terminal
message of at most `2^10` coefficients. Timings use three-variable folds and
Canopy height four, giving a Canopy message of `N/8` and a shared tail at rate
1/8. The model's chosen schedule and the timed schedule are distinct and
recorded separately. Raw samples, stage timings, and `timing_method.json` are
under [`timings/`](../../results/r7a/timings), in `t*_whir_*` directories.

## Small local reproduction

Run from the repository root after the native release build in the main README.

```sh
target/release/canopy-system paper-estimate \
  --logs 18 --inputs 1,2 --samples 100 \
  --canopy-codeword-log-reduction 1 \
  --output-dir results/reproduce/whir/estimates

target/release/examples/whir_fiat_shamir \
  --input-dir results/reproduce/whir/estimates \
  --output results/reproduce/whir/estimates/fiat_shamir.csv

RAYON_NUM_THREADS=1 target/release/canopy-system paper-time \
  --log-length 18 --inputs 1 --warmups 0 --repetitions 1 \
  --canopy-codeword-log-reduction 1 \
  --output-dir results/reproduce/whir/timings
```

Use fresh output paths for each run. The `N=2^18` pilot verifies the component
measurement workflow. To reproduce the recorded model seeds and full grid, use
`python3 scripts/reproduce.py estimates`; the full Linux timing campaign is
`python3 scripts/reproduce.py run`.

## Implementation

- [`paper_experiments.rs`](../../src/paper_experiments.rs): parameter search,
  proof-size accounting, and composed timing harness.
- [`paper_fiat_shamir.rs`](../../src/paper_fiat_shamir.rs): transcript ledger.
- [`executable_whir.rs`](../../src/executable_whir.rs): native WHIR bridge.
- [`canopy.rs`](../../src/canopy.rs): Canopy commitments and the SEP reduction.
- [`vendor/whir/`](../../vendor/whir): vendored native protocol implementation.
