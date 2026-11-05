# Section 8 prover phase profiles

The profiling build adds exclusive timers to the same `ifold`, `iacc`, and
rate-1/2 and rate-1/4 Merkle reductions described in
[the Section 8 experiment](experiment_section8.md). It measures where prover
time is spent while preserving the protocol, selected-height objective, and
proof/hash accounting.

The corrected workload is `steady_state_v2`: `ifold` takes `ell_cast` fresh
MLE Canopy claims plus one prior compact Batch Canopy claim, returning
`ell_cast + 1` Merkle residual claims. `iacc` combines every residual with
`ell_acc` prior Merkle claims. Its source count is therefore
`ell_cast + 1 + ell_acc`, while each regular Merkle control still takes
`ell_cast + ell_acc` full-length claims. Timing repetitions prepare a valid
representative prior Canopy claim outside the reduction timer. Protocol tests
separately reuse both actual previous accumulator outputs across three steps.

The reduction decomposition has eighteen exclusive components: functional
materialization; four sumcheck phases; message and cached-codeword combinations;
seven output-commitment phases; authentication and returned-claim construction;
the final proof copy; and remaining prover work. Sumcheck, combination, output
commitment, and opening totals are parent measurements. Their exclusive children
sum to them, and the exclusive reduction entries sum to `step_seconds`. Add
parent totals or their children when comparing costs; adding both double counts.

Output commitment separates RS encoding, Merkle leaves, Merkle parents,
digest-to-field conversion, Canopy-layer OOD evaluation, output-polynomial OOD
evaluation, and remaining commitment work. Leaf time includes allocation,
canonical field serialization, and native `hash_rows` work. Digest-to-field
time measures direct conversion to GF(2^256), with zero cryptographic hashes.
`serialization_seconds` measures
only the final `p.proof()` copy, so it does not include all serialization work
performed during the reduction.

`sumcheck_transcript_seconds` includes transcript work plus control and timer
residuals. It is not a measurement of pure hash latency. Dense covector
materialization remains charged to `form_seconds`. The current prover keeps
`Form::Mle` points and equality factors implicit, so fresh MLE sources no longer
allocate or fold dense covectors inside the prover prefix. Carried `Form::Batch` sources and
`Form::Canopy` residuals retain the dense path; sumcheck setup takes ownership
of those buffers. Native per-round allocation shrinking after the first fold is unchanged. The public verifiers still materialize and evaluate
the generic dense forms. Untimed input-claim preparation also materializes
dense forms to compute public sigmas. This changes the prover representation while
preserving the coefficients, challenges, proof structure, queries, and hash
accounting.

Consequently, optimized `ifold` still has nonzero dense materialization for its
prior Batch claim, and `iacc` still materializes the returned Canopy forms and
prior Batch claims. A lower form timer does not mean that all public functionals
have become implicit.

The October 21 implementation borrows each input message for the first
sumcheck round and allocates only its first half-length folded witness. Fresh
MLE folding also computes the evaluations needed for the following round's
polynomial during the fold. Dense carried claims retain the native fold path.
Consequently the polynomial and fold timers move work between them; their sum
is the useful comparison across implementations.

For at least eight sources and a codeword of at least 65,536 field elements,
the prover combines cached codewords in 512-row tiles and hashes each completed
tile immediately. `codeword_leaf_fused=true` identifies this path in raw rows.
Its entire elapsed time is charged to `codeword_combination_seconds`, and
`merkle_leaf_seconds` excludes those output leaves. The encoded Canopy layer
still has separately timed leaves; `iacc` and regular Merkle output leaf timers
are zero on the fused path. Compare the sum of those
two columns with earlier runs; a zero separate leaf timer does not mean hashing
was eliminated. Fresh commitment leaves still have their own timer.

Output encoding is zero for the Merkle controls and `iacc`: they combine cached
input codewords and build a fresh Merkle commitment to the result. `ifold`
additionally encodes its Canopy layer. Encoding fresh inputs remains part of
incoming commitment preparation and is reported in a separate six-component
decomposition. Those components are incoming RS encoding, leaf hashing, parent
hashing, digest-to-field conversion, OOD evaluation, and remaining preparation
work. They sum to `incoming_commit_seconds` and are excluded from reduction time.

Commitment-inclusive totals add only the commitment cost of the `ell_cast`
fresh input oracles. Canopy totals are fresh Canopy commitments plus `ifold`
plus `iacc`; regular Merkle totals are fresh Merkle commitments plus their
batching reduction. Prior Canopy and Merkle commitments are already available
to the accumulation step, so independently preparing representative priors is
excluded from the total. `iacc` has zero fresh commitment sources and reuses
the committed Canopy layers passed from `ifold`.

Raw rows record `fresh_commit_sources`, `fresh_commit_seconds`, and
`prior_commit_seconds`. Six additional exclusive fields record fresh encoding,
leaves, parents, digest conversion, OOD evaluation, and remaining work. The
reporter checks that they sum to the fresh commitment time and that fresh plus
prior equals incoming time. It forms each
`total_prover_seconds = fresh_commit_seconds + step_seconds` before pairing
or computing statistics. These extra times overlap the all-input incoming
decomposition; they are not additional reduction phases. Fresh-phase
percentages divide by fresh commitment time. `percentage_denominator` in the
summary records the denominator used for every component.

The reporter rejects negative/nonfinite phases, unverified samples, duplicate
sample identities, wrong source counts, mixed workload versions, and failures
to reconcile phase sums. It groups by original
message length, `ell_cast`, `ell_acc`, height, row width, security budget, and
scheme. Means, medians, minima, and maxima are retained. Percentages divide a
component mean by its corresponding total mean. Paired `ifold + iacc` totals
sum matching repetition/input-seed samples before computing statistics.
The workload and source count are retained in the summary. Missing CSV workload
tags identify `legacy_fresh_only_v1`, whose `ifold` omits the prior Canopy.
Historical and corrected CSVs cannot be combined in one report.
Complete fresh-cost columns are optional when reading historical files. A
partial bundle or a mixture of files with and without these columns fails the
phase report; no fresh-only cost is inferred from an old all-input incoming
time. Provenance labels the expanded accounting
`cost_scope=fresh_commit_plus_reduction_v1` and older phase-only samples
`cost_scope=reduction_only_v1`.

The earlier shared allocation changes apply across the reductions:
already materialized dense covectors move by ownership into sumcheck setup;
the first unit-weight
RLC coefficient copies its input rather than multiplying every element by one;
`Form::Canopy` covector materialization consumes its equality buffer with
`into_vec` rather than cloning it; and
the Batch public covector's query term is accumulated in place. These changes
preserve the protocol, queries, and proof structure. The reference samples use these optimizations.

The equality-buffer change applies only when `Form::Canopy` is materialized;
it does not change Canopy-layer encoding. Fresh `ifold` inputs use MLE
claims, and the prior Canopy input uses `Form::Batch`; their measured
materialization or layer-construction time should not be attributed to that
copy removal.

## Collecting a profile

Run each configuration and scheme in a fresh process. The runner launches
children sequentially, records source/executable identities and resource
usage, and refuses an existing output directory. Each Canopy child executes
both reductions. These commands run a small local profile using only Python's
standard library:

```sh
RUSTFLAGS="-C target-cpu=native" cargo build --locked --release --example accumulation_section8

python3 scripts/profile_section8_campaign.py \
  --binary target/release/examples/accumulation_section8 \
  --output-dir results/reproduce/accumulation-profile/raw \
  --logs 16 --ell-cast 1,2 --ell-acc 1 \
  --log-leaf-width 3 --security-bits 128 --objective total \
  --threads 1 --warmups 0 --repetitions 1

python3 scripts/report_section8_profile.py \
  --timings results/reproduce/accumulation-profile/raw/profiles.csv \
  --provenance results/reproduce/accumulation-profile/raw/campaign.json \
  --output-dir results/reproduce/accumulation-profile/report --plot off
```

The report writes `summary.csv`, `README.md`, and `provenance.json`, preserving
raw source files and their SHA-256 checksums. Optional Matplotlib support
produces a stacked phase plot with `--plot on`. Hardware attribution comes
only from supplied provenance. A source snapshot does not establish which
sources produced an existing executable; rebuild it before collecting profiles.

The [reference campaign](../results/r7a/README.md) retains per-job stage CSVs
at N=2^24. Its measured samples use the same phase boundaries described above.
