# Section 8 accumulation reductions

The [RS accumulation experiment](../experiments/rs-accumulation/README.md)
benchmarks the Canopy `ifold` and Merkle `iacc` reductions at fixed codeword
lengths. This document specifies their statements, accounting, and tests.
The source relation is `R_cast = CR[SEP, Canopy]`.

In a steady-state step, `ell_cast` counts **fresh** Canopy claims. One prior
Canopy accumulator also enters `ifold`. It compiles the batching IOR on these
`ell_cast + 1` inputs with Canopy, giving the IR

```text
CR[SEP, Canopy]^ell_cast × CR[SEP, Canopy]
  -> CR[SEP, Canopy] × CR[SEP, MT]^(ell_cast + 1).
```

The output Canopy SEP claim is retained. Each input's Canopy opening adds one
Merkle-committed SEP claim about that input's Canopy layer. Apart from field
elements, the proof contains only the bottom Canopy paths for all `ell_cast + 1`
input oracles, including the prior Canopy accumulator. These obligations are
returned by the reduction rather than discharged by an extra IOPP.

`iacc` compiles the same batching IOR with ordinary Merkle trees:

```text
CR[SEP, MT]^(ell_cast + 1) × CR[SEP, MT]^ell_acc -> CR[SEP, MT].
```

Its `ell_cast + 1` residual sources are precisely the obligations returned by `ifold`;
its `ell_acc` additional sources are previously accumulated claims in the same
RS code. The combined accumulation step retains the batched Canopy claim and
the output Merkle SEP claim. A final membership decider is outside the measured
step. With the reference `ell_acc = 1`, the retained state is a Canopy claim
and a Merkle claim. The next step reuses both: `ifold` has `ell_cast + 1`
sources and `iacc` has `ell_cast + 2`.

## Geometry and parameter search

`N = 2^log_n` is the original message length in **field elements**. Each Merkle
row contains `w = 2^log_leaf_width` field elements. The default row width is
eight, configurable through `--log-leaf-width`. The cast code has rate 1/2,
hence block length `2N` field elements and `2N / w` Merkle rows. At height `h`,
its Canopy layer message has `2N / 2^h` field elements. The layer's codeword has
`N` field elements (`N / w` rows), half the input block length, and rate
`2^(1-h)`. Height uses the field-element convention; the bottom-path parent
depth is `h - log2(w)`.

The eight-element layout matches the existing WHIR experiments: a
three-variable fold consumes eight evaluations from each queried row. The
Section 8 batching reductions also support scalar leaves, but retain width
eight for this comparison. One row hash processes 256 bytes and counts as one
hash invocation.

Both reductions use GF(2^256), 32-byte field elements, BLAKE3-256, additive RS,
the Johnson query radius `1 - 1.05 sqrt(rate)`, and no proof of work. The local
query/security budget is configurable with `--security-bits` and defaults to
128 bits, targeting round-by-round errors without a union-bound margin for the
number of protocol rounds. The estimator records local
query-security and degree-scaled affine
mutual-correlated-agreement diagnostics and excludes heights failing its local
budget. These diagnostics do not certify composed soundness for batching by
powers of one challenge; the benchmark does not claim a proved end-to-end
128-bit accumulation theorem.

For each `(log_n, ell_cast, ell_acc)`, the estimator searches the admissible
Canopy heights and minimizes **expected verifier hashes for ifold + iacc**.
The objective includes authentication and Fiat–Shamir finalizations. It accounts
for duplicate random queries and shared nodes within multipaths. The selected
height and every height candidate are saved. Height is chosen from estimates
before measuring timings; it is not tuned to the sampled prover times.

The two Merkle controls use the original message length `N`, row width, and
`ell_cast + ell_acc` source claims. One uses rate 1/2, the other rate 1/4.
They perform the Merkle compilation of the same batching IOR. Their output
commitments preserve their input code parameters. A shrinking WHIR fold is not
part of this experiment.

## Measurement and accounting

The timing executable prepares valid input instances, commits them, executes
the reductions, and verifies each resulting proof. Initial witness generation,
public code setup, and input claim preparation are outside reduction time.
Incoming commitments are timed separately. The reduction includes its output
commitment and the computation of its returned claims. Verification and the
final membership decider are excluded from prover time.

The commitment-inclusive comparison charges only the `ell_cast` **fresh**
input oracles. For Canopy its total is fresh Canopy commitment time plus
`ifold` and `iacc`. Each regular Merkle total is fresh Merkle commitment time
plus that control's batching reduction. Prior Canopy and Merkle commitments are
already available to a steady-state step. Preparing representative prior
instances for an independent benchmark repetition is measured separately and
excluded from this total. `iacc` reuses the committed layer witnesses returned
by `ifold` and has zero fresh commitment sources.

New timing rows record `fresh_commit_sources`, `fresh_commit_seconds`, and
`prior_commit_seconds`, with
`incoming_commit_seconds = fresh_commit_seconds + prior_commit_seconds`.
The fresh commitment phases separate RS encoding, Merkle leaves, Merkle
parents, digest-to-field conversion, OOD evaluation, and remaining commitment
work. Their six durations sum to `fresh_commit_seconds`. Each measured
`total_prover_seconds` is formed as `fresh_commit_seconds + step_seconds`
before computing statistics or paired Canopy totals. Fresh commitment time
does not include initial witness generation or public SEP claim preparation.

Fresh cast inputs use single multilinear-evaluation claims (`Form::Mle`).
The additional prior Canopy accumulator and prior Merkle accumulators use
compact `Form::Batch` claims with an MLE term, an out-of-domain term, and
column-wise RS query terms, as produced by the batching reduction. The prior
Canopy claim uses the full cast code and has its own Canopy commitment and
layer OOD data. In the timing driver, these representative valid prior claims
and their public sigmas are prepared independently outside the reduction
timers; timing repetitions are independent steps. Each source has a distinct
deterministic message, and each repetition changes the input seed. All
`ell_cast + 1` residual sources of `iacc` use the actual compact claims returned
by that repetition's `ifold`. Separate tests carry both actual accumulator
outputs across consecutive steps.

The current prover represents `Form::Mle` sources implicitly in the shared
batching prefix: it retains the point and an equality factor, evaluates the
actual message halves for each round coefficient, and folds only the message.
This division-free path also applies to fresh MLE sources in the regular
Merkle controls. Carried `Form::Batch` claims and Canopy residual forms remain
dense; both public verifiers retain the generic dense evaluation path.
Initial message cloning and native fold shrinking are unchanged. The
untimed input-claim preparation still materializes dense forms to compute
public sigmas; the avoided fresh-form memory is timed sumcheck scratch.
The representation change preserves round polynomials, Fiat–Shamir challenges,
proof bytes, query counts, and verifier hash accounting, including the
coefficient semantics for incorrect input sigmas. The current tests compare the dense and implicit prefixes directly.

The raw CSV contains one row per scheme and measured repetition. `ifold` and
`iacc` rows share a repetition and input seed, so the reporting script computes
paired totals and then computes mean, median, minimum, and maximum. A total
median is the median of the paired sums. Warmup runs do not enter these
statistics. The report fails on an unverified measured sample.

New estimates, height-search rows, and timing rows record
`workload=steady_state_v2`. Reports check source counts for every scheme and
require all supplied CSVs to use the same workload. They record the workload
in summaries and provenance. A missing workload column means the historical
`legacy_fresh_only_v1` format, whose `ifold` omitted the prior Canopy input;
legacy rows cannot be combined with corrected estimates or timings. The
campaign runner requires the corrected tag, source counts, and fresh-cost
columns before accepting child results. With the expanded columns, report
provenance records `cost_scope=fresh_commit_plus_reduction_v1`. Older CSVs
remain supported, with fresh-only and commitment-inclusive costs unavailable;
their all-input incoming time is not a substitute for fresh-only cost.

Verifier hashes count complete authentication hash and Fiat–Shamir hash/XOF
invocations, rather than BLAKE3 compression blocks. Direct injective conversion
of a 256-bit digest to GF(2^256) costs zero hashes. Model expectations and
sampled verifier counts remain distinct in the report.

Expected proof payload bytes exclude public input instances, fresh commitment
roots and OOD values attached to returned instances, and serializer framing.
Field elements and authentication hashes have separate payload columns. Actual
`proof_bytes` include native vector framing; measured payload is
`proof_bytes - proof_framing_bytes = proof_field_bytes + proof_hash_bytes`.
`proof_hint_bytes` is the serialized hint storage containing opened rows,
authentication material, and framing, so it is not an additional payload term.
Fresh output wire data and actual serialized total are reported separately.
Returned SEP descriptors and challenges have no extra wire bytes. For `ifold`, authentication
payload consists only of bottom Canopy paths. For `ifold + iacc`, proof bytes
sum the two proofs; output-instance bytes sum both reductions' fresh output
data, including the intermediate `ifold` output. These output data occupy 64
bytes for `ifold` and 32 bytes for `iacc`; they are not just the final accumulator
size.

The report reads hardware and run metadata only from supplied companion
provenance. It does not infer a machine from a previous campaign. Raw source
CSVs are preserved and their SHA-256 checksums are recorded. Paths intended for
publication are sanitized with the repository's public-metadata helper.

## Correctness tests

The protocol tests in `src/section8_tests.rs` and
`src/section8_batch_tests.rs` verify proofs using only explicit public input
and output statements. They separately decide the returned SEP relations by
checking the full witness, fresh RS encoding, and fresh commitment root.
Verification derives native Merkle commitments from the declared public roots;
there is no separate cached commitment that can disagree with those roots.

Coverage includes scalar and packed leaves, several heights and arities,
zero and constant messages, repeated sources, and three consecutive
accumulation steps using both actual previous outputs. Each step passes the
previous Canopy root, layer OOD data, compact Batch form, and sigma into
`ifold`, and the previous Merkle root, Batch form, and sigma into `iacc`.
The carried witnesses are reused directly, and fresh output encoding and root
checks decide both returned relations. Tests reject tampering with carried
claims, replacing them by stale valid accumulators, cross-wiring the Canopy
and Merkle roots, and corrupting a carried input's opening. Adversarial cases
modify every sumcheck coefficient and final source evaluation, roots, OOD
values, each source's rows and authentication digests, public covectors and
sigmas, and source order/arity. Tests also exercise parameter/input replay,
truncated or appended proof streams, and incorrect vector lengths. A separate
test confirms that a changed Canopy bottom hash creates a false deferred SEP
claim which cannot be accumulated successfully.

`src/section8_prefix_tests.rs` compares dense and implicit prover prefixes for
several arities and RS layouts, mixed MLE/Batch/Canopy forms, zero and constant
messages, and Boolean MLE coordinates. It checks identical prefix outputs,
proof bytes, and subsequent sponge challenges. For incorrect input sigmas, the
two prover paths still agree and the unchanged dense verifier rejects; a separate
test forces a zero equality factor to check the division-free update. Both ordinary and hash-accounting builds exercise these checks.

`src/section8_estimator.rs` pins the query budgets and selected heights for the
benchmark grid and checks expected authentication costs against all repeated
three-query patterns in a small packed forest.

```sh
cargo test --locked --lib
cargo test --locked --release --lib section8
cargo test --locked --features fs-accounting --lib section8
```

Release tests require ordinary verification errors for tampered proofs. Debug
tests permit only the reference transcript's explicit interaction-pattern
mismatch assertion; unrelated panics fail the tests. The hash-accounting run
checks actual Fiat–Shamir finalizations as well as the protocol results.

## Reproduction

The [experiment guide](../experiments/rs-accumulation/README.md) includes a
small verified pilot and reporting commands. The shared reproduction runner
uses the reference N=2^24 grid in [r7a.json](../experiments/r7a.json).
The [profiling guide](experiment_section8_profile.md) explains the exclusive
prover phases and how to collect them in fresh processes.

The current source and reference dataset use `steady_state_v2`. Older CSV
formats remain readable by the reporters for diagnostics, with their workload
and cost scope explicitly labeled. Superseded measurements are not included in
the public result selection.
