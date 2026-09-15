# Bolt-max with Canopy RS/WHIR

This experiment compares a prime-field Bolt-max adaptation with Merkle row
commitments against the same protocol with Canopy row commitments. Both paths
generate a transcript and verify it. Going forward, the default proof-size grid
is `2^22, 2^26, 2^30` and the default executable timing point is `2^22`.
The earlier `2^18` results are preserved for reproducibility and are no longer
part of the default grid.

## Memory and larger-input measurements

The [memory report](../results/bolt_max_canopy_memory/summary.md) measures whole-process
peak resident RAM at `2^22` and `2^24`, using the proof-size-best width 4 and
eight threads. The larger point compares both `k/2` and `k` Canopy blocks with
fresh Merkle runs. Memory includes setup and verification; the reported prover
times include encoding, commitment, and opening. Reproduce sequentially with:

```sh
python3 scripts/measure_bolt_memory.py --log-length 22 --interleaving 4 \
  --canopy-layout equal-full --warmups 2 --repetitions 10 \
  --output-dir results/bolt_max_canopy_memory/n22_w4_equal_full_rusage
for bolt_layout in equal-full equal-half; do
  python3 scripts/measure_bolt_memory.py --log-length 24 --interleaving 4 \
    --canopy-layout "$bolt_layout" --warmups 2 --repetitions 10 \
    --output-dir "results/bolt_max_canopy_memory/n24_w4_${bolt_layout//-/_}"
done
python3 scripts/summarize_bolt_memory.py
```

On macOS the memory wrapper obtains the completed child's peak RSS with
`getrusage`, whose `ru_maxrss` value is in bytes. The two protocols execute
sequentially in one child, so the result is the peak of the complete benchmark,
including retained allocator memory, rather than two separately measured peaks.

## Reproduce

The historical block-length sweep gives **both** Canopy RS codewords length `k/2` or `k`,
where `k=N/w`. It retains the earlier `k/4` results for comparison. All layouts
optimize the two heights independently for proof size, then measure prover time
at those heights. Reproduce the two enlarged layouts with:

```sh
cargo test --offline --lib bolt
cargo build --release --offline
for bolt_layout in equal-half equal-full; do
  bolt_results="results/bolt_max_canopy_${bolt_layout//-/_}"
  RAYON_NUM_THREADS=8 target/release/canopy-system bolt-grid \
    --canopy-layout "$bolt_layout" --interleavings 4,8,16 \
    --log-lengths 18,22,26,30 --timing-log-length 18 --samples 100 \
    --warmups 2 --repetitions 10 --output-dir "$bolt_results"
  for bolt_width in 4 8 16; do
    RAYON_NUM_THREADS=8 target/release/canopy-system bolt-time \
      --canopy-layout "$bolt_layout" --interleaving "$bolt_width" \
      --log-length 22 --warmups 2 --repetitions 10 --height-search-samples 100 \
      --output-dir "$bolt_results/timing_n22/width_$bolt_width"
  done
  python3 scripts/summarize_bolt_profile.py \
    --input-dir "$bolt_results" --report-file prover_profile.md
done
python3 scripts/compare_bolt_block_lengths.py
```

The [combined block-length report](../results/bolt_max_canopy_block_lengths/comparison.md)
compares full proof sizes, selected heights, and commitment/opening timings.
The commands run sequentially so benchmarks do not compete with each other.

The earlier variant gives **both** Canopy RS codewords block length `k/4`,
where `k=N/w` is the systematic row count. It is selected with
`--canopy-layout equal-blocks`; the original `quarter-each` layout remains
available and is still the default for old commands.

```sh
cargo run --release -- bolt-grid --canopy-layout equal-blocks \
  --log-lengths 18,22,26,30 --timing-log-length 18 \
  --interleavings 4,8,16 --samples 100 --warmups 2 --repetitions 10 \
  --output-dir results/bolt_max_canopy_equal_blocks
for bolt_width in 4 8 16; do
  target/release/canopy-system bolt-time --canopy-layout equal-blocks \
    --interleaving "$bolt_width" --log-length 22 --warmups 2 --repetitions 10 \
    --height-search-samples 100 \
    --output-dir "results/bolt_max_canopy_equal_blocks/timing_n22/width_$bolt_width"
done
python3 scripts/summarize_bolt_profile.py \
  --input-dir results/bolt_max_canopy_equal_blocks --report-file prover_profile.md
python3 scripts/compare_bolt_canopy_layouts.py
```

These commands search all four input sizes, then time and verify both schemes
at `2^18` and `2^22`. Each part's height minimizes its estimated authentication
proof bytes, including its Canopy proof, and timings use those selected heights.
All original size and timing outputs are preserved separately.

To reproduce the original `k/4, k/16` variant:

```sh
cargo test --lib bolt
cargo run --release -- bolt-grid --interleavings 4,8,16 --samples 100 \
  --log-lengths 18,22,26,30 --timing-log-length 18
```

The grid searches initial Bolt interleaving widths 4, 8, and 16 at every input
size, optimizing both Canopy heights for each pair. It writes combined size and
timing comparisons to `results/bolt_max_canopy_quarter_width_grid/summary.md`.
Each `width_*` subdirectory includes the full height search and proof breakdown.
It also executes and verifies both schemes at `2^18`, with one warmup and five
measured repetitions per width. Use `--timing-log-length`, `--warmups`, or
`--repetitions` to change those timing parameters.

Additional timings at `2^22`, where Canopy saves proof bytes, are saved under
`results/bolt_max_canopy_quarter_width_grid/timing_n22/`. Reproduce them with:

```sh
for bolt_width in 4 8 16; do
  cargo run --release -- bolt-time --interleaving "$bolt_width" \
    --log-length 22 --warmups 1 --repetitions 5 --height-search-samples 100 \
    --output-dir "results/bolt_max_canopy_quarter_width_grid/timing_n22/width_$bolt_width"
done
```

The original width-128 data remain in `results/bolt_max_canopy_quarter`.
Single-width commands retain 128 as their default and accept `--interleaving`:

```sh
cargo run --release -- bolt-estimate --samples 100
cargo run --release -- bolt-breakdown
cargo run --release -- bolt-time --log-length 18 --warmups 1 --repetitions 5 --height-search-samples 100
```

The output directory defaults to `results/bolt_max_canopy_quarter`. Use
`--output-dir` to select another directory. Estimation accepts `--log-lengths`
as a comma-separated list; timing accepts one `--log-length`. Large timing
points allocate the actual matrices, including 32 bytes per input element.
Both commands accept a deterministic `--seed` for input/setup and size sampling.

Outputs:

- `proof_sizes.csv`: full modeled proof size and common versus authentication costs.
- `height_search.csv`: every admissible height, including terminal constant messages.
- `summary.md`: selected parameters and comparison.
- `proof_size_breakdown.csv` and `.md`: disjoint hash/field components for both
  schemes, reconciled to the saved size estimates without rerunning height search.
- `whir_oracle_breakdown.csv`: per-oracle geometry, query draws, distinct rows,
  repeated row values, and authentication costs for each common/Canopy WHIR proof.
- `prover_timings.csv`: individual measurements and actual serialized proof bytes.
- `prover_timings.md`: medians, with commitment and opening time separated.
- `prover_stage_timings.csv`: disjoint wall-clock stages for every measured proof.
- `prover_stage_timings.md`: additive mean stage times, phase reconciliation,
  and total timing variation. Generated by `bolt-time` alongside the totals.
- `timing_height_search/`: parameters selected for the timing run, without
  overwriting the full size grid.

## Parameters and geometry

The field is the existing `Fr`, with prime modulus `2^256 - 43*2^32 + 1`.
This reuses the executable WHIR backend's prime field. The separate
[archived Experiment 4 field-switch model](archive/experiment4_field_switch.md) instead uses
`GF(2^256)` as its source field and `GF(2^192)` for recursion; that binary-tower
model is a proof-size/soundness estimator, not the arithmetic backend used for
these Bolt timings. A 256-bit prime field and `GF(2^256)` are different fields.
The Bolt RS backend uses multiplicative radix-2 FFT domains. The separate
[GF(2^256) WHIR/Canopy timing backend](../experiments/binary-whir/README.md) now
implements characteristic-two arithmetic and additive RS; it does not change
these Bolt experiments.

Field elements and BLAKE3 digests occupy 32 bytes. Merkle commitments, H2F, and
the Fiat-Shamir duplex use BLAKE3. The duplex uses Spongefish's existing generic
XOF construction with the repository's pinned BLAKE3 version. WHIR retains its
SHA3-derived static protocol/session identifiers; this does not change the
selected commitment or challenge hash.

Write the input length as `N=wk`, where `w` is the initial interleaving width.
The current grid searches `w=4,8,16`; the original experiment used `w=128`.
Bolt's systematic matrix `X` has `k` rows of width `w`. A public all-one,
degree-8, compression-8 sparse matrix `H`
produces `U=HX` with `k/8` rows. Each column of `U` is encoded by a rate-1/2
coefficient-basis multiplicative RS code, yielding parity matrix `Y` with
`k/4` rows. The combined encoded row count is `5k/4`.

Both baselines commit `X` and `Y` separately. In the original `quarter-each`
layout, the Canopy RS lengths are
**`k/4` and `k/16` field elements**, respectively, so their sum is exactly
`(5k/4)/4`. There is no rounding to the next power of two and no multiplication
of this length by the row width. A native WHIR leaf can pack several
of these RS symbols; that leaf packing does not change the RS block length.

For a part with `R` rows, height `h` means `h` binary levels **above row hashes**.
The Canopy message has `R/2^h` field elements and the code has rate `2^(2-h)`.
The search enumerates `3 <= h <= log2(R)` independently for the two parts and
minimizes total estimated proof bytes. At `h=log2(R)` the message is constant:
the prover opens its one coefficient, and the verifier recomputes its constant
RS commitment. This is a valid, degenerate Canopy endpoint, not an omitted proof.
The constant codeword is committed by repeated equal-child hashing, avoiding
allocation of a redundant constant array.

For the revised **`equal-blocks`** layout, both RS block lengths are `k/4`.
The systematic layer is unchanged. The parity part still has `R_Y=k/4`
committed rows, so its Canopy codeword now has length `R_Y`, and its message
has `R_Y/2^h_Y` elements. Its rate is `2^-h_Y` and its admissible heights are
`1 <= h_Y <= log2(R_Y)`. The systematic rate remains `2^(2-h_X)`, with
`3 <= h_X <= log2(k)`. The combined Canopy codeword length is `k/2`, or 40%
of Bolt's total encoded row count, implementing the user's larger parity block.
The verifier uses these exact lengths, including at constant endpoints. A
distinct transcript domain binds the revised layout; Merkle transcripts and
the original Canopy layout retain their existing domains. CSV files record
the layout; old size CSVs without that column default to `quarter-each`.

The **`equal-half`** and **`equal-full`** layouts expand both blocks further:

| Layout | Each RS block | Systematic rate | Parity rate | Minimum h_X / h_Y |
|---|---|---|---|---|
| `equal-blocks` | `k/4` | `2^(2-h_X)` | `2^(-h_Y)` | `3 / 1` |
| `equal-half` | `k/2` | `2^(1-h_X)` | `2^(-1-h_Y)` | `2 / 0` |
| `equal-full` | `k` | `2^(-h_X)` | `2^(-2-h_Y)` | `1 / 0` |

The maximum heights remain `log2(k)` and `log2(k/4)`. At height zero, row
hashes go directly through H2F into the Canopy RS message; no binary parent
hashes or bottom authentication paths are needed. All permitted heights have
RS rate at most `1/2`. The new layouts have separate transcript domains;
Merkle and existing Canopy domains retain their previous bytes. Neither the
Bolt encoder nor its external row query budgets change.

## Distance and soundness convention

The degree-8 all-one structure follows the `bolt-xor` encoder in
[bolt-rs at revision 3832e47](https://github.com/bcc-research/bolt-rs/tree/3832e47b24e7b3e10525c9c5bcfc1cfe66d525f2).
The paper's [Sections 6.1 and 7.3](https://eprint.iacr.org/2026/310.pdf)
establish the relevant distance for a binary LDPC ensemble. Here, **distance
`gamma=0.013` is an explicit assumption about the prime-field code**. The
experiment does not certify the sampled matrix's minimum distance or claim
that the binary-field theorem transfers to this prime field. The upstream
configuration-model graph retains repeated edges, which become repeated field
additions in this adaptation.

Following the paper's concrete query formulas, the separate row budgets are:

```
q_X = ceil(128 / -log2(1 - 0.013/3)) = 20431
q_Y = ceil(128 / -log2(3/4))         =   309
```

External row queries are uniform without replacement. If the budget reaches
the number of rows, all rows are read. This is at least as strong as the
with-replacement query-error bound. Native WHIR queries retain the reference
implementation's with-replacement sampling. Its current configuration transmits
repeated row values; only the Merkle authentication paths are merged.

The 128-bit label is a **per-event / round-by-round target**, as in the current
paper experiment convention; it is not a union-bound claim of 128 bits for
the entire Fiat-Shamir composition. No proof of work is used. Native WHIR
configurations must pass their security and zero-PoW checks. RS proximity proofs
use unique decoding, and Canopy SEP proofs use the proved Johnson regime.

## Executed protocol

The experiment instantiates the public-`H` / `H`-oracle-aided setting of Bolt.
It checks the final `H` evaluation directly. **Spark/Mulperm preprocessing or
unamortized sparse-PCS proofs are not included.** Verification therefore has
linear work in the public sparse matrix; these are not succinct-verifier timing
claims or reproductions of the published binary-field timing numbers.

1. Commit to the two encoded matrices with the selected row commitments.
2. At the fixed public benchmark evaluation point, send Bolt's width-`w`
   partial-evaluation vector `g`. Check its final evaluation against the claim.
3. Sample the interleaving challenge `r`, form `x=Xr` and `u=Ur`, and create
   independent rate-1/2 WHIR commitments to these two vectors.
4. Sample a syndrome evaluation point `s` and send `sigma=u_hat(s)`.
5. Sample separate systematic and parity row pools and authenticate the complete
   queried rows. Batch their projected values with fresh challenges sampled
   after the opening messages.
6. Prove three linear constraints on `x` in one WHIR invocation: its evaluation
   from `g`, its queried coordinates, and `<H^T eq(s), x> = sigma`.
7. Prove two constraints on `u` in another WHIR invocation: its evaluation
   `sigma` and the batched RS evaluations corresponding to queried parity rows.
   The transpose RS covector is computed with one FFT.
8. For each Canopy part, reconstruct forest ancestors from the opened rows,
   apply H2F, and prove their batched coordinate claims against the **original
   Canopy RS commitment**. The verifier reconstructs these claims independently.
9. Verify all WHIR final claims and consume the complete transcript and hints.

WHIR's linear-form interface incorporates the `H` relation and spot-check
sumchecks. This is an explicit prime-field protocol adaptation, not a port of
the upstream binary Ligerito transcript. Only the two initial Bolt matrix
commitments are replaced; internal WHIR commitments use BLAKE3 Merkle trees.
The scalar message and syndrome proofs are independent, as are the two Canopy
proofs. No cross-proof batching benefit is assumed.

The initial reduction takes an independent random linear combination of the
`w` columns, reducing `N` field elements to `k=N/w`. It uses `w` random field
coefficients, rather than a multilinear evaluation at `log2(w)` challenges.
Each queried row still transmits all `w` values: 128, 256, or 512 bytes in the
new grid. The public evaluation and transcript domain both bind the width.
The transcript version changed to bind this parameter; newly generated
width-128 transcripts can differ from the historical timing data even though
their proof-size model is unchanged.

Subsequent WHIR proofs fold three variables at a time (factor 8, fewer for
messages of size 2 or 4) and use the native terminal condition. The grid
optimizes the initial width and `h`; subsequent WHIR folding schedules stay
fixed. It does not borrow the earlier paper experiment's `2^10` terminal
cost subtraction. Size estimates mirror this executable configuration and its
two transmitted coefficients per ordinary sumcheck round.

## Accounting and validation

Both total size estimates include initial roots, complete queried rows, `g`,
`sigma`, both scalar constrained-RS proofs, and any Canopy proofs. The modeled
sizes count cryptographic hashes and field elements and omit serializer framing
(eight-byte vector lengths for WHIR row hints). Zero-PoW slots emit no nonces.
Actual transcript-plus-hint bytes are recorded separately in executable timings.

Both timing paths call the identical prime-field Bolt encoder and row hash
implementation. Commitment time includes encoding; opening time includes the
partial evaluation, projection, new RS commitments, public-H covector work,
row authentication, constrained proofs, and Canopy SEP proofs. Public setup
and input generation are excluded. All measured proofs are verified, and the
execution order alternates across repetitions. The verifier receives no input
matrix, syndrome witness, or Canopy message.

Detailed profiles for widths 4, 8, and 16 at `2^18` and `2^22` are saved under
`results/bolt_max_canopy_prover_profile/`, separately from the earlier timings.
They use two warmups and ten measured repetitions per scheme and configuration:

```sh
cargo build --release
for bolt_log_input in 22 18; do
  for bolt_width in 4 8 16; do
    target/release/canopy-system bolt-time --interleaving "$bolt_width" \
      --log-length "$bolt_log_input" --warmups 2 --repetitions 10 \
      --height-search-samples 100 \
      --output-dir "results/bolt_max_canopy_prover_profile/n${bolt_log_input}/width_${bolt_width}"
  done
done
python3 scripts/summarize_bolt_profile.py
```

The profile separates sparse syndrome formation from Bolt parity RS encoding;
row-tree/forest construction from Canopy H2F and RS commitments; the partial
evaluation and projection; common WHIR commitments and proofs; query sampling,
row values, authentication paths, and constraint covectors; and Canopy ancestor
claims, OOD evaluations, and WHIR SEP proofs. Canopy's constant parity endpoint
is listed separately from its nonconstant systematic RS/WHIR layer.

Each native RS-encode/WHIR-oracle commitment call is timed as a unit, including
its FFT, allocation, matrix hashing, and transcript binding. The profile does
not separate these internal costs or the internal rounds of a WHIR proof.
Component tables use arithmetic means so they sum to the mean total; medians
and observed variation are reported separately. Every run reconciles the sum
of its stages to the existing commitment/opening timers. Residual time covers
untimed operations and allocation/drop within those boundaries. Setup,
verification, proof-object extraction, and final cleanup outside those timers
remain excluded. Timer bookkeeping is included in the residual.

### RS workload sizes and the upstream speed comparison

The RS stages in a Bolt/Canopy proof do **not** encode the full original input.
For original input `N`, width `w`, and systematic height `h_X`:

| Workload | RS message elements | RS codeword elements |
|---|---:|---:|
| Full-input rate-1/2 RS baseline | `N` | `2N` |
| Bolt's parity RS, across all columns | `N/8` | `N/4` |
| Common message WHIR initial RS | `N/w` | `2N/w` |
| Common syndrome WHIR initial RS | `N/(8w)` | `N/(4w)` |
| Systematic Canopy initial RS | `N/(w*2^h_X)` | `N/(4w)` |

At `N=2^22, w=4, h_X=3`, the Canopy RS message has only `N/32=131072`
elements. Its commitment time cannot be compared directly with Bolt encoding
of all `N=4194304` original elements. The two common WHIR RS messages together
have `9N/32` elements. Their profiling entries also include oracle hashing.

A separate encoder-only audit uses the same full input, field, width, and
thread count for Bolt and two rate-1/2 RS implementations:

```sh
cargo run --release --example bolt_encoder_audit -- 22 4 2 5
```

Results: [`results/bolt_encoder_audit/n22_w4/summary.md`](../results/bolt_encoder_audit/n22_w4/summary.md).
The measured means were 235.7 ms for the current Bolt-max adaptation, 656.0 ms
for full-input RS using Bolt's arkworks column encoder, and 673.1 ms using the
WHIR RS backend: about a 2.8x encoding advantage for Bolt in this setting.
The Canopy-sized RS-only call was 17.1 ms in the same audit. Output allocation
is included; setup, input-buffer preparation, hashing, proof generation, and
output destruction are excluded. The four workloads run in rotating order.

The [paper's 3–4x benchmark](https://eprint.iacr.org/2026/310) concerns full
**commitment** time for `N=2^30` over `GF(2^32)`. The
[upstream implementation](https://github.com/bcc-research/bolt-rs) uses SIMD XOR,
prefetching, width 128, accelerated SHA256, and CPU/GPU overlap. The current
port uses generic 256-bit modular additions, CPU-only BLAKE3, and arkworks
multiplicative FFTs. That benchmark ratio is not a guaranteed encoding ratio
for this adaptation. No claim is made that the current prime-field sparse
encoder reproduces the performance engineering of the upstream XOR encoder.

Tests cover the sparse matrix degree counts, prime-field linearity, direct RS
evaluation, the transpose relation, exact quarter geometry, admissible height
search, both nonconstant and constant Canopy proofs, altered evaluation claims,
tampered commitments and rows, and exact accounting of repeated WHIR row values.
Round trips and independent public-evaluation checks cover widths 4, 8, 16,
and 128. The size breakdown must reconcile to each saved total at every width.
