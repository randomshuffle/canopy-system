# Final experiment artifacts

Only the final five-configuration suite and its repeated checks live here.
Use the [repository reproduction guide](../README.md#reproduce-the-final-suite)
for fresh outputs and the [campaign configurations](../experiments/aws-r5/README.md)
for exact workload, hardware, and timing conventions.

| Directory | Canonical artifact | Scope |
|---|---|---|
| [paper_bcgm_m10](paper_bcgm_m10/README.md) | [proof_sizes_and_hashes.csv](paper_bcgm_m10/proof_sizes_and_hashes.csv) | Current BCGM m=10 proof/hash estimates, all 75 points; no new timings. |
| [aws_i4i_optimized](aws_i4i_optimized/README.md) | [comparison_iop_rounds_direct_digest.csv](aws_i4i_optimized/comparison_iop_rounds_direct_digest.csv) | Main 70-point measured campaign; hashes corrected for direct digest conversion, original Canopy timings include the removed rehash. |
| [aws_i4i_n30_eta1](aws_i4i_n30_eta1/README.md) | [remote/comparison_iop_rounds.csv](aws_i4i_n30_eta1/remote/comparison_iop_rounds.csv) | Five measured N=2^30, eta=1 points using direct digest conversion. |
| [aws_i4i_bolt_n22_recheck_20260917](aws_i4i_bolt_n22_recheck_20260917/README.md) | [remote/results/summary_iop_rounds.csv](aws_i4i_bolt_n22_recheck_20260917/remote/results/summary_iop_rounds.csv) | Balanced repeated Bolt timings at N=2^22 for eta=1,2,4,8. |
| [aws_i4i_bolt_n24_recheck_20260917](aws_i4i_bolt_n24_recheck_20260917/README.md) | [remote/results/summary_iop_rounds.csv](aws_i4i_bolt_n24_recheck_20260917/remote/results/summary_iop_rounds.csv) | Corresponding repeated Bolt timings at N=2^24. |

The current estimate table and historical timing records have separate
provenance. Use the current BCGM table for proof/hash values; the measured
campaigns retain their original parameter and source receipts. Commitment
reporting uses C(N, eta) = eta * C(N, 1); openings are measured at each eta.
For eta > 1 the reported total is derived from these components.

Raw CSVs, logs, validation receipts, source/binary hashes, and original source
bundles remain with each dataset. Bundles and scripts under `remote/` are
original-run snapshots. Old host launchers are in
[archive/launchers](../archive/launchers/README.md); run the current tools under
`scripts/` into fresh output directories. Successful reruns do not rewrite
published measurements.

Exploratory studies and the scheduling audit are indexed separately in
[archive](../archive/README.md). The implementation change to BCGM m=10 did not
rerun historical AWS timings. See each dataset's README for limitations.
