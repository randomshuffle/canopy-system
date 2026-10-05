# Prepared N=2^30, eta=1 extension

Five configurations, 32 pinned workers, no warmup and one measured pass.
Only a plan and planning estimates have been generated; nothing has run on AWS.
Use `experiments/aws-r5/config-n30-eta1.json` with the runner's `--config` option
and a fresh output directory and native build. The direct digest injection is
included. The completed 70-point campaign and manuscript data are unchanged.

Forecast from the AWS N=2^26 and N=2^28 measurements:

- WHIR grouped job: about 402 GiB peak RSS and 36.5 minutes wall time.
- Bolt grouped job: about 154 GiB peak RSS and 11.5 minutes wall time.
- Jobs run sequentially: peak memory is the maximum, not the sum.
- Budget roughly 400–450 GiB for WHIR and 150–170 GiB for Bolt, and 45–60 minutes
  for the two measured jobs. Compilation/proof-parameter preparation and other
  workload contention can add time. One input alone occupies 32 GiB.
- The existing conservative preflight still requires over 640 GiB available and
  caps address space at 640 GiB; check live free memory before execution. CPU
  affinity does not reserve RAM or isolate memory bandwidth.
- These forecasts use the old rehash-inclusive measurements, so no speculative
  timing benefit from removing the rehash is claimed.

`planning_prover_estimates.csv` gives per-method prover costs; these exclude
setup, verification, input generation and extra component-harness work. The
provenance JSON records whole-job RSS/wall estimates separately.
