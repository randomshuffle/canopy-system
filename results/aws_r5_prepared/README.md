# Prepared AWS campaign — no AWS timings yet

The approved [experiment plan](../../experiments/aws-r5/README.md) contains five methods and 70 configuration points, using 96 workers and one warmup plus three measured runs. N=2^22,2^24,2^26 use eta=1,2,4,8; N=2^28 uses eta=1,2. There are no N=2^30 jobs.

- `proof_sizes_and_hashes.csv`: all 70 hardware-independent proof-size and verifier-hash estimates, with 100 deterministic samples per selected point. Verifier totals include authentication, H2F and FS finalizations.
- `comparison.csv`: the same grid with `timing_status=not_measured` everywhere. No local timings are represented as AWS measurements.
- `estimates/`: raw proof estimates, optimized parameters and hash ledgers.
- `preparation/`: exact estimator commands, binary hashes, source snapshots, logs and process measurements from local preparation.
- `plan.json`: the full intended command list and approved configuration. Paths resolve to the current checkout; regenerate on the host after extraction.
- `canopy-aws-source.tar.gz`, `bundle.json`: source bundle and SHA-256/source manifest. Includes the current uncommitted code and vendor changes; dependencies are fetched with Cargo's locked versions on the host.
- `local_validation/`: ARM correctness/memory probes only. A one-repetition smoke run at N=2^22, eta=1 exercised all five methods and verified the serialized Bolt proofs. Field, additive-FFT and quarter-block binding checks passed. These are not campaign timing samples.

All 20 eta=1 proof/hash rows exactly match the previous half-length WHIR and shared-Merkle/quarter-length Bolt results. The native Intel backend still requires the mandatory build-time tests on AWS; it cannot be executed on the local ARM machine.

The preparation process snapshots retain the launcher version used when generating each result. The final bundle additionally includes launcher-only improvements to cancellation handling, build provenance checks and dependency-license packaging; no estimator/protocol code changed after these results were generated.
