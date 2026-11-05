# October 28, 2026 reference campaign

Hardware: **AWS r7a.32xlarge**, AMD EPYC 9R14, 128 physical cores, 1 TiB RAM.
Measurements used **1 or 32 Rayon workers**, pinned to CPU `0` or CPUs `0–31`
on NUMA node 0, with default first-touch memory placement. The build used
Rust 1.92.0, locked dependencies, `-C target-cpu=native`, and no Fiat–Shamir
instrumentation. Jobs ran sequentially.

## Completed scope

| Family | 32-worker configurations | 1-worker configurations | Total |
|---|---:|---:|---:|
| WHIR: Merkle 1/2, Merkle 1/4, Canopy | 45 | 36 | 81 |
| Bolt: Merkle, black-box Canopy | 30 | 24 | 54 |
| RS accumulation: Canopy, Merkle 1/2, Merkle 1/4 | 12 | 12 | 24 |
| **Total** | **87** | **72** | **159** |

For Bolt/WHIR, the 32-worker grid is `N=2^22,2^24,2^26` at `eta=1,2,4,8`,
`N=2^28` at `eta=1,2`, and `N=2^30` at `eta=1`. Single-worker measurements
stop at `N=2^26`. Accumulation uses `N=2^24`, fresh arities `1,2,4,8`, and
one previous accumulator in each relation, at both worker counts.

The campaign changed its sampling policy while running. **108 configurations**
have zero warmups and one measurement; **51** have one warmup and two
measurements. Each comparison row and raw process command retains its actual
policy. [`experiments/r7a.json`](../../experiments/r7a.json) reconstructs the
completed 78-job workload, including these policies. It replaces the originally
planned 174-point/N=2^20 accumulation scope recorded in the historical config.

## Files and accounting

- [`snark_comparison.csv`](snark_comparison.csv): all Bolt/WHIR measurements.
  `raw_mean_*` fields preserve measured batch costs. Headline commitment is
  `eta × mean C(N,1)` for the same method and worker count; headline total adds
  the opening measured at that `eta`. These are derived timing totals.
- [`accumulation_comparison.csv`](accumulation_comparison.csv): fresh commitment
  plus paired `ifold+iacc` or the Merkle reduction. Prior commitments,
  verification, and a final membership decider are excluded.
- [`proof_sizes_and_hashes.csv`](proof_sizes_and_hashes.csv): 75 Bolt/WHIR
  model points, with authentication and Fiat–Shamir counts separated.
- [`estimates/`](estimates): selected parameters and full search outputs.
  Accumulation has four reduction rows per arity (`ifold`, `iacc`, and two
  controls); the report pairs `ifold+iacc` into one method.
- [`timings/`](timings): raw CSVs, logs, and `process.json` for all 78 jobs.
  WHIR jobs also retain their `timing_method.json`; Bolt rows retain complete
  serialized proof checks. Accumulation reductions are verified individually.

WHIR proof models optimize fold schedules separately from the fixed
three-variable timing schedule. WHIR timings compose verified components;
Bolt timings cover complete serialized public-`H` adapter arguments. Model
proof bytes exclude serializer framing. Accumulation's returned-instance wire
data are separate from modeled reduction proof payload.

## Provenance and integrity

The selected CSVs, logs, process receipts, and source archive are byte-for-byte
copies of the original completed campaign. Their original locations and SHA-256
hashes are recorded in [`provenance/retained-files.json`](provenance/retained-files.json).
Paths inside those files describe the original run, not the current layout.

[`checksums.json`](checksums.json) covers this curated directory. Run
`python3 scripts/validate_results.py` from the repository root to check its
contents and rebuild the comparison tables from raw samples. The validator
checks all fields against the retained tables, including the mixed sampling
policy and sequential commitment calculation.

[`provenance/native-build.json`](provenance/native-build.json) identifies the
historical sources and binaries; [`provenance/host.json`](provenance/host.json)
records the machine. [`provenance/original-validation.json`](provenance/original-validation.json)
is the original campaign receipt, before curation. Its references to full
download manifests and manuscript checks describe that historical snapshot;
the current validator covers only the retained public selection.

[`source.tar.gz`](source.tar.gz) preserves the original measured source tree
under `canopy-system/`, including upstream licenses. Extract it into a separate
directory to inspect or rebuild that implementation. The cleaned working tree
has fewer entry points and new reproduction tools, so its source and binary
hashes differ. Reproducing estimates with current sources checks numerical
agreement; it does not relabel old timings as a new measurement.
