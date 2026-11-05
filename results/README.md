# Reference results

The public result set is the completed **October 28, 2026 r7a.32xlarge campaign**.
It covers black-box Bolt, WHIR, and steady-state RS accumulation over GF(2²⁵⁶).

| Artifact | Contents |
|---|---|
| [Bolt and WHIR comparison](r7a/snark_comparison.csv) | 135 measured configurations, raw and sequential-round timing totals, expected proof bytes, and verifier hashes. |
| [RS accumulation comparison](r7a/accumulation_comparison.csv) | 24 configurations at `N=2^24`; fresh commitment, reduction, and total prover costs. |
| [Bolt/WHIR proof and hash models](r7a/proof_sizes_and_hashes.csv) | 75 deterministic model points, shared by both thread counts. |
| [Parameter searches](r7a/estimates) | WHIR schedules, Bolt X/Y height candidates, and accumulation height candidates and selected reductions. |
| [Raw measurements](r7a/timings) | 78 completed jobs with raw samples, stage profiles, verification results, commands, and process receipts. |
| [Provenance](r7a/provenance) | Original source/binary hashes, host configuration, validation receipt, and the selection manifest. |
| [Measured source snapshot](r7a/source.tar.gz) | The source bundle used for the historical native build. |

See the [dataset notes](r7a/README.md) for the exact scope and accounting.
Experiment-specific instructions are in [Bolt](../experiments/bolt/README.md),
[WHIR](../experiments/whir/README.md), and
[RS accumulation](../experiments/rs-accumulation/README.md).

```sh
# Check file integrity and independently rebuild all 159 reported method points.
python3 scripts/validate_results.py

# Regenerate tables into a new directory; the reference files are read-only inputs.
python3 scripts/reproduce.py report --input results/r7a --output results/reproduce/report
```

Older machine campaigns, white-box Bolt variants, early accumulation workloads,
incomplete reruns, duplicate downloads, and manuscript scratch files are omitted.
Their timings are not mixed with the retained results. The current reproduction
code records its own source/binary identities; historical hashes continue to
refer to the original measured build.
