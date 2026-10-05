# Corrected WHIR scheduling and parallelism audit

**Status: redundant follow-up intentionally stopped.** The final results are
`../../../results/aws_i4i_optimized/comparison_iop_rounds.csv` (70 completed points). Six new
WHIR timing points completed here before the stop; they are diagnostic only and
are not mixed into the final dataset. The eta=2 N=2^26 job was interrupted.

## Current reporting policy: sequential IOP rounds

The user's later clarification supersedes concurrent commitment timing for the
headline comparison. The eta input oracles are committed in different IOP rounds.
For **all five configurations**, report C(N, eta) = eta * C(N, 1), using the
same N and scheme's measured eta=1 commitment. Retain each eta's measured opening
time and set total = scaled commitment + opening. Proof sizes and verifier hashes
are unchanged. Apply this also to both Bolt schemes, even though their raw
commitment loops were already sequential.

The original run completed unchanged. The queued follow-up was subsequently
stopped because the sequential-round reporting policy made it unnecessary. Concurrent commitment
samples are retained as raw diagnostics; they are not the reported commitment
times. Use `comparison_iop_rounds.csv`, generated from raw `comparison.csv` by
`scripts/report_iop_round_commitments.py`. The report retains raw timing columns
and marks eta>1 totals as derived from measured components. Points without their
same-N/same-scheme eta=1 anchor remain pending. Backups from frozen remote builds
are processed locally after retrieval; the running binaries are not changed.

This campaign corrects an implementation asymmetry discovered in the 32-worker
AWS run. Previously the two Merkle-WHIR baselines multiplied a single-input
commitment sample by eta, while Canopy executed all eta commitments concurrently.
The previous eta > 1 WHIR commitment and total-time comparisons must not be used
as evidence of a Canopy speedup. Original data remain in `../../../results/aws_i4i_optimized/`.

## Correction

All three WHIR configurations now use concurrent initial encodings and concurrent
independent Merkle commitments on the same Rayon pool. Timers record batch wall
time, never the sum of overlapping task durations. Rate-1/2 WHIR and Canopy share
the exact same measured base RS encoding and codeword matrices. Each method is
charged this encoding cost once. The rate-1/4 baseline uses the same scheduler
with its own larger codewords. Canopy's layer uses the same batch encoder and
batch Merkle commitment helper as both baselines. The full baseline trees are
released before measuring Canopy's forests; Canopy matrices/trees are released
before measuring rate-1/4 commitments.

WHIR results include `timing_method.json`, version `concurrent-commitments-v2`.
The runner rejects older methods, different worker counts, different shared RS
samples, or commitment/total stage sums that do not reconcile.

## Audit of corresponding stages

| Work | WHIR baseline versus Canopy | Bolt baseline versus Canopy |
|---|---|---|
| Initial RS encoding | Same `encode_many_interleaved_with_whir`; concurrent inputs for both rates and Canopy | Same `BoltMaxEncoder::syndrome` and `bolt::encode_columns` |
| Base Merkle leaves and parents | Same WHIR BLAKE3 engine and parallel row hashing; both full trees and bottom forests parallelize parent work | Same `Layer::commit` and `MerkleLevels::commit_forest`, differing only in retained height |
| Canopy-layer RS/Merkle | Same concurrent encoder/Merkle helper as baselines | Same native WHIR committer used for code-switch oracles |
| Hash-to-field | Parallel root maps in WHIR Canopy and Bolt Canopy; Merkle baselines do not need H2F | Parallel map in the common X/Y `Layer::commit` |
| Opening fronts | Same native WHIR routines and same single-input-times-eta component model for all three methods | Both use `multi::prove_front`; serial transcript order for both |
| g, H transpose, projections, dot products | Native WHIR routines shared by all three methods | Identical shared parallel helpers for both methods |
| Distinct-functional cross evaluations | Same parallel `generalized_sz_cross_term_work` for all three methods | Same parallel `batching::prove_batch` and parallel dot helper |
| Recursive WHIR and additive FFT | Same native prover, GF(2^256) backend and FFT implementation | Same native prover and FFT implementation; shared versus split tails are the selected protocol designs |
| Resource limits | Same 32-worker pool and affinity | Same 32-worker pool and affinity |

No further one-sided parallelism optimization was found in these corresponding
paths. This is a source audit, not a claim of optimal scheduling: some loops and
transcript-dependent work remain serial for both sides. Different circuit sizes,
query counts and batch sizes can produce different utilization in shared code.

WHIR still composes measured components: single-input opening-front work is
scaled by eta for all three methods, recursive tails are native batch executions,
and native suffix timings are subtracted to match the paper's terminal cutoff.
Canopy bottom openings are measured explicitly as additional work. This is not
a full serialized Exp3 timing. Fixed stage order and a single timing sample do
not characterize cache/order effects or run-to-run variability. The user's other
experiment can share memory bandwidth despite separate CPU affinity.

## Validation and rerun

- Regression tests compare independently computed commitments to concurrent
  batches of four distinct inputs, with 1 and 4 workers, at rates 1/2, 1/4 and 1/8.
  Roots and their order must match exactly.
- The small executable timing regression covers eta=1 and eta=4, checks identical
  half-rate base-encoding samples and commitment/opening/total reconciliation.
- The full library suite and native AWS validation must pass before the rerun.
- Proof/hash estimates and protocol parameters do not change.
- Rerun all 42 WHIR points: N=2^22,2^24,2^26 with eta=1,2,4,8; N=2^28 with eta=1,2.
  N=2^26,eta=4 runs first. Retain the 28 Bolt points from the previous optimized
  32-worker campaign, with their original source/build/process provenance.
- Zero warmups, one measured pass. CPUs `0-15,48-63` only; the user's CPUs
  `16-47,64-95` remain available to their experiment. Jobs/builds do not overlap
  our previous campaign.

The original 70-point campaign completed successfully and is the final dataset.
This follow-up was stopped after N=2^26 eta=4 and eta=1 completed (six new WHIR
points); N=2^26 eta=2 was interrupted and excluded. Imported Bolt points retain
original provenance but are not new runs. All 85 local release tests and the
native AWS validation passed. The implementation audit and tests remain useful;
concurrent commitment measurements are not used for the sequential IOP model.

Remote directory: `.`.
Output: `canopy-system/archive/results/aws_i4i_fair`; local backups: `remote/`.
Source archive and checksums: `bundle.json`. Raw results are retained separately.
The progress heartbeat is paused. No further benchmark is scheduled.
