# One-fold Canopy accumulation versus plain Merkle accumulation

The incoming Canopy claim is retained unchanged. The old Canopy-committed RS0 SEP is reduced by exactly one native WHIR fold. This leaves the folded-polynomial RS1 claim and the Canopy-opening RS1 claim. Those two claims and the previous RS1 accumulator are batched into one new RS1 accumulator. This is a three-source batch inside a two-component accumulator state.

The plain baseline batches an incoming RS0 SEP and the previous RS0 accumulator into one new RS0 accumulator, retaining the same code, message length, and rate. Both schemes use full batched sumcheck to a common evaluation point, followed by same-code commitment and consistency queries. This resets the SEP functional to a bounded-size description each step. Neither scheme proves the outgoing accumulator to a terminal WHIR proof.

## Parameters and alignment

| Quantity | RS0 | RS1 |
|---|---:|---:|
| Message length (field elements) | N | N/8 |
| Codeword length (field elements) | 2N | N |
| Rate | 1/2 | 1/8 |
| Field elements per native Merkle leaf | 8 | 8 |
| Merkle leaves | N/4 | N/8 |
| Query draws per source | 317 | 96 |

RS1 is taken directly from `native.round_configs[0].irs_committer`, the second oracle in the repository’s WHIR implementation. The example asserts equality of Canopy and native RS1 message length, codeword length, rate, and row layout. The domain is the same registered Cantor-subspace domain. Canopy uses h=4 in field-element units: its eight-element row hash is followed by one binary parent layer, giving N/8 subtree digests. Those digests map invertibly into GF(2^256), with no hash-to-field invocation.

All measurements use GF(2^256), BLAKE3-256, eight Rayon workers, a 136-bit local Johnson query budget (the repository’s convention for a 128-bit composed target), and no proof of work. The first fold has three sumcheck rounds. The headline experiment uses WHIR’s interleaved code/row metric, matching the existing experiments; the earlier scalar-leaf primitive table is a different layout and workload.

## Prover times

Measured on the local Apple M2 desktop, with one warmup and five measurements per scheme and size. Scheme order alternates each repetition. Entries are medians in milliseconds. The incoming-commitment-inclusive column is the median of paired per-run sums, not the sum of independently taken medians.

| N | Scheme | Accumulation step | Step range | Incoming commitment | Step + incoming commitment |
|---|---|---:|---:|---:|---:|
| 2^18 | Plain Merkle batch | 23.59 | 23.37–24.06 | 17.21 | 40.87 |
| 2^18 | Canopy, one fold | 24.98 | 24.43–25.36 | 24.39 | 49.56 |
| 2^20 | Plain Merkle batch | 92.87 | 86.17–99.88 | 74.73 | 167.60 |
| 2^20 | Canopy, one fold | 91.55 | 88.86–94.57 | 102.32 | 193.92 |
| 2^22 | Plain Merkle batch | 492.32 | 372.63–827.35 | 364.56 | 853.00 |
| 2^22 | Canopy, one fold | 368.17 | 359.37–476.67 | 516.00 | 881.56 |

The step starts with cached input commitments, codewords, and witnesses. It includes input-functional materialization, the fold and its new RS encoding (Canopy only), Canopy bottom openings and resulting SEP construction, full batched sumcheck, linear combinations, the outgoing Merkle commitment and OOD evaluation, source openings, and outgoing-claim construction. Both same-code batches combine cached codewords directly, avoiding a redundant RS encoding. The folded-polynomial commitment still requires its encoding.

Input generation, public setup, computation of already-public input sigmas, verification, and final decision of the accumulator are excluded. The incoming-commitment column includes its RS encoding and commitment; Canopy also includes its layer OOD evaluation. The fresh input SEP functional is one multilinear evaluation. The existing accumulator uses the same compact functional shape produced by a prior step: an MLE term, one OOD term, and randomly batched column-wise query terms. Folded and Canopy-opening claims are the actual claims produced during the measured execution.

The N=2^22 runs had substantial desktop timing variation; overlapping ranges limit any speedup conclusion. Commitment-inclusive costs are distinct from accumulation-step costs. No large-N prover-time extrapolation is made.

## Expected verifier hashes per accumulation step

Counts include authentication hashes and Fiat–Shamir BLAKE3-XOF finalizations. They are complete hash invocations, not BLAKE3 compression calls or verifier wall time. The output accumulator is retained, so its tree is not opened. The incoming Canopy commitment is simply carried forward, so its paths are not checked in this step. Final-decider work is excluded.

| N | Plain Merkle batch | Canopy, one fold | Hash reduction |
|---|---:|---:|---:|
| 2^18 | 5,605 | 3,402 | 39.3% |
| 2^20 | 6,872 | 3,981 | 42.1% |
| 2^22 | 8,142 | 4,559 | 44.0% |
| 2^24 | 9,412 | 5,137 | 45.4% |
| 2^26 | 10,682 | 5,715 | 46.5% |
| 2^28 | 11,952 | 6,293 | 47.3% |
| 2^30 | 13,222 | 6,871 | 48.0% |

Let D(M,q)=M[1−(1−1/M)^q] be expected occupied nodes for sampling with replacement. Native WHIR hashes all q queried rows, including duplicates, then merges parent paths. Thus H(M,q)=q+sum_{j=1}^{log2 M} D(M/2^j,q). The formulas are:

- Plain: 2 H(N/4,317) + log2(N)+4.
- Canopy: D(N/4,317)+D(N/8,317) + 3 H(N/8,96) + log2(N)+9.

The first two Canopy terms are its deduplicated bottom row and parent hashes. The three H terms are the separately committed folded, Canopy, and old accumulator sources. Shared query indices do not merge paths across distinct roots.

The exact expectation formula was checked against exhaustive small-tree enumeration. Every execution checks native authentication counts against the sampled path geometry. Fiat–Shamir counts were independently recorded in a separate instrumented verifier run. Deterministic timing inputs have one fixed query sample per size; hash tables above use the exact expectation rather than that single sample.

## Validation and scope

Every warmup and measured run replayed the fold and batch verification and checked the derived output SEP against the output witness. Small validation runs additionally re-encoded the output message, checked its Merkle root and codeword, and rejected modified sumcheck/OOD messages, modified Canopy bottom values, and modified Merkle authentication data. Thus the cached codeword combination was checked against fresh RS encoding.

This is an executable one-step cost prototype, not a recursive SNARK or a production accumulator interface. The verifier materializes covectors for the correctness checks; a succinct implementation would evaluate their compact descriptions directly. That difference does not affect the reported hash counts. The timing instance repeats the same valid Canopy input in the old and incoming positions: it is bound twice and the incoming one is carried unchanged. A production wrapper must check equality of the complete incoming and outgoing pending instance, and the final decider must check both state components. A formal extraction proof for this composition is separate from these measurements.

The initial exploratory re-encoding run is retained in `reencoded_output_samples.csv` and `initial_reencoded_output.rs`. It is not used in the headline tables. Earlier scalar-layout primitives remain in `../accumulation_pending_reduction/` as preliminary work.

## Reproduction

```sh
cargo build --offline --release --example accumulation_one_fold
RAYON_NUM_THREADS=8 target/release/examples/accumulation_one_fold --logs 10 --warmups 0 --repetitions 1 --check-tampering --output archive/results/accumulation_one_fold/optimized_smoke.csv
python3 scripts/measure_gf256_run.py --record-dir archive/results/accumulation_one_fold/optimized_run --threads 8 -- target/release/examples/accumulation_one_fold --logs 18,20,22 --warmups 1 --repetitions 5 --output archive/results/accumulation_one_fold/timing_samples.csv
cargo build --offline --release --features fs-accounting --example accumulation_one_fold
RAYON_NUM_THREADS=8 target/release/examples/accumulation_one_fold --logs 10,18 --warmups 0 --repetitions 1 --check-tampering --output archive/results/accumulation_one_fold/fs_validation.csv
python3 archive/scripts/report_accumulation_one_fold.py
```

Raw timing samples, stage means/medians, the exact command, executable hash, process RSS, instrumented validation, and expected hash counts accompany this report. The timed process used about 2.37 GiB peak RSS, including setup and verification. No AWS jobs or manuscript files were changed.
