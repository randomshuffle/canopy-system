# Main 70-point AWS campaign

Completed on an i4i.24xlarge with 32 Rayon workers pinned to CPUs
`0-15,48-63`, Rust 1.92.0, and native release builds. The five configurations
cover N=2^22,2^24,2^26 with eta=1,2,4,8 and N=2^28 with eta=1,2.
Each point has zero warmups and one measured pass.

Use [comparison_iop_rounds_direct_digest.csv](comparison_iop_rounds_direct_digest.csv)
for this campaign. It corrects the obsolete extra Canopy rehash in verifier
hash totals and preserves every original proof-size and timing value.
Canopy timings still include that rehash and have not been rerun.
[Current BCGM m=10 estimates](../paper_bcgm_m10/README.md) are recorded
separately; no new timing campaign accompanied that update.

Reported commitment time follows sequential IOP rounds:
C(N, eta) = eta * C(N, 1) for the same scheme and N. Opening is measured at eta,
and total is their sum. This supersedes the raw concurrent commitment comparison
that prompted the [historical scheduling audit](../../archive/results/aws_i4i_fair/README.md).
For eta > 1 these totals are derived from measured components.

WHIR times compose verifier-checked native components; Bolt times cover complete
serialized and verified public-H adapter arguments. Spark/Mulperm preprocessing
and proofs are excluded. A single timing pass does not measure run variability.
The repeated [N=2^22](../aws_i4i_bolt_n22_recheck_20260917/README.md) and
[N=2^24](../aws_i4i_bolt_n24_recheck_20260917/README.md) Bolt checks use direct
digest conversion and remain separate.

Raw measurements are in [remote/comparison.csv](remote/comparison.csv) and
`remote/timings/`. [completion-validation.json](completion-validation.json)
and [affinity-validation.json](affinity-validation.json) retain completion and
CPU-allocation checks. The source tarball and bundle manifest describe the
original run, including its original directory layout; they are not current
reproduction bundles.

Reproduce with the [main campaign guide](../../experiments/aws-r5/README.md)
into a fresh output directory. A current-source run uses direct digest
conversion throughout and current BCGM parameters. To regenerate only the
historical hash correction into a new file:

```sh
python3 scripts/report_direct_digest_conversion.py \
  --input results/aws_i4i_optimized/comparison_iop_rounds.csv \
  --output results/reproduce_main_direct_digest.csv
```

The [original campaign discussion](../../archive/docs/aws_i4i_optimized_history.md)
and [original host launchers](../../archive/launchers/aws_i4i_optimized/EXPERIMENT.md)
are archived for context.
