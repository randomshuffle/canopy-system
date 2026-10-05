# Paper estimates using BCGM m=10

All 75 paper proof-size and verifier-hash estimates have been regenerated with
BCGM26, full version Definition 9.1 and Theorem 9.2, using m=10. The native and
modeled affine error share one implementation in the vendored WHIR crate;
the estimator sums coordinate errors for multilinear folds.

The bound is `10.5^7 * n_RS^2 / (3 * rho^(3/2) * |F|)` per affine challenge,
at radius `1 - (1 + 1/20) sqrt(rho)`. These are RS list-decoding parameters;
Bolt's general-code test and the unique-decoding paths retain their bounds.

`proof_sizes_and_hashes.csv` is the 75-row export consumed by the paper.
`estimates/` contains the raw searches, selected configurations, soundness
breakdowns, and Fiat-Shamir accounting. `provenance.json` records exact commands,
source and binary hashes, the seed and sample count, and the toolchain.
`validation.json` compares every exported metric and selected geometry to the
historical datasets and records the minimum composed WHIR security.

Every exported proof/hash metric and all 825 checked geometry fields match the
historical values exactly. The minimum composed WHIR security is 132.785653 bits.
All 87 implementation tests and 110 vendored parameter tests pass.

No timings were rerun. The before/after native parameter audit checks 868
configurations. All 812 in the range covering the paper (three-variable folds,
log inverse rate at most 22, field size 2^256, targets 128 and 136) are identical.
Five configurations at much lower rates outside the paper's range acquire PoW;
these are recorded in `native-parameter-audit.json`. The audit source is saved as
`native-parameter-audit.rs` and uses the public native configuration constructor.
The historical AWS data remains unchanged.

To reproduce from the repository root on a local machine, install Rust 1.92.0
and run the estimate-only entry point. Cargo uses the checked-in lockfile; the
first build needs network access to fetch dependencies.

```sh
rustup toolchain install 1.92.0 --profile minimal
python3 scripts/reproduce_estimates.py --output results/reproduce_estimates
```

This runs only proof/hash estimation and reporting, using the final campaign
configuration plus its N=2^30 extension. It does not launch AWS timing jobs or
allocate full N-element proof workloads. The output must be new or empty; this
published dataset is never overwritten. The fresh `validation.json` compares
every output field with the stored 75-point report, and `provenance.json` records
the current source, configuration, binary hashes, commands and logs.

Use `--offline` once dependencies are cached, `--threads 4` to change local
parallelism, or `--skip-build --bin-dir target/release` to reuse an existing
uninstrumented build. `results/paper_bcgm_m10/regenerate.py` forwards the same
arguments to this entry point and uses the same fresh-output default. The paper's
external `figures/experiments/render.py` overlays these estimates on measured
timings; it is not required to regenerate the CSVs in this repository.
