# Bolt

This experiment compares the **shared-tail Merkle baseline** with **black-box
Canopy commitments** for Bolt-max's input matrices. Both use GF(2²⁵⁶),
BLAKE3-256, width `w=4`, and `k=N/4` rows in the systematic matrix.

Canopy replaces only the initial X/Y commitments. Every subsequent Bolt IOPP
oracle remains Merkle-committed. The Canopy SEP obligations are proved
separately from Bolt's claims, batched within each X or Y family across inputs.
Both Canopy codewords have `N/16` field elements; the X/Y heights are optimized
independently for expected proof bytes, including full-height constant layers.
The shared Bolt tail joins at message length `k/8` in both methods.

This is the public-`H` adapter: `H` is public and its final evaluation is checked
directly. Spark/Mulperm preprocessing and proofs are outside the experiment.
The distance assumption is `gamma=0.013`; the systematic test uses the existing
general-code Khatam analysis, and parity proximity is analyzed separately.
Recursive RS proofs use the BCGM `m=10` bound and a 136-bit local budget.
Fiat–Shamir uses a BLAKE3 XOF with no proof of work.

## Results

Filter [`snark_comparison.csv`](../../results/r7a/snark_comparison.csv) on:

| Method | `scheme` |
|---|---|
| Merkle baseline | `bolt_merkle_shared` |
| Black-box Canopy | `bolt_canopy_black_box` |

The table contains 54 measured Bolt configurations across 1 and 32 workers.
Proof/hash estimates and all height candidates are in
[`results/r7a/estimates/`](../../results/r7a/estimates), under `n*/bolt/`.
Raw `timings/t*_bolt_*/prover_timings.csv` files contain serialized proof bytes,
verification status, commitment/opening times, and the actual sample count.
Stage profiles are retained beside them.

The headline commitment cost is `eta × mean C(N,1)` at the same worker count;
opening is measured at the requested `eta`. See the [main README](../../README.md)
for the full grid and reporting conventions.

## Small local reproduction

Run from the repository root after the native release build in the main README.
Use fresh output paths. `N=2^16` is a small functional pilot, not a paper datapoint.

```sh
target/release/examples/gf256_bolt_black_box estimate \
  --logs 16 --inputs 1,2 --samples 100 \
  --output results/reproduce/bolt/estimates

RAYON_NUM_THREADS=1 target/release/examples/gf256_bolt_black_box time \
  --log 16 --inputs 1 --warmups 0 --repetitions 1 \
  --estimates results/reproduce/bolt/estimates \
  --output results/reproduce/bolt/timings
```

Each timing repetition generates and verifies both complete serialized
arguments, alternating method order. Public setup, input generation, and
verification are excluded from the prover timer. The selected heights are read
from the estimator output before timing.

For the full reference grid, run `python3 scripts/reproduce.py estimates` or
`python3 scripts/reproduce.py run` as described in the main README. The campaign
records source/binary hashes and enforces one timing job at a time.

## Implementation

- [`bolt.rs`](../../src/bolt.rs): Bolt-max encoder and public `H` evaluation.
- [`bolt_black_box.rs`](../../src/bolt_black_box.rs): height search, transcript
  accounting, comparison harness, and tamper tests.
- [`bolt_multi.rs`](../../src/bolt_multi.rs) and
  [`bolt_batching.rs`](../../src/bolt_batching.rs): batched proofs and shared tail.
- [`bolt_experiment.rs`](../../src/bolt_experiment.rs): shared commitments,
  openings, and serialized public-`H` adapter primitives.

The retired white-box split-tail and delayed-join experiments are not part of
this comparison.
