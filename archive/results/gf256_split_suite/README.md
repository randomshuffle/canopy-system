# Independent systematic/parity Bolt batches

Both Canopy RS block lengths are fixed at k=N/4. Width is 4; inputs are eta independent length-N polynomials. The field, security assumptions, hashes, query counts, and public-H scope match the existing batched suite.

The systematic tail batches eta code-switch claims with eta systematic Canopy claims. The parity tail independently batches eta sketch claims with eta parity Canopy claims. Both tails share a domain-separated Fiat–Shamir transcript but use distinct challenges. Each side optimizes its own height and join length. The full-height constant-layer alternatives are included. The search permits several authenticated folds of at most three variables; every selected configuration joins after one fold. Native full cross matrices are retained within each batch.

The Merkle baseline is optimized in both shared-tail and split-tail form; the smaller-proof configuration is the baseline at each point. The 100-query-sample proof estimates include verifier authentication, hash-to-field, and Fiat–Shamir finalizations, with transcript bytes separately. Fiat–Shamir adds no proof bytes.

All four configurations were rerun in rotating order, with one warmup and five repetitions per (N,eta), eight Rayon threads. All 288 proofs were verified. Timing includes commitment and opening; verification/input generation/setup are excluded. The paper reports the selected Merkle baseline and split Canopy; the WHIR experiment retains its existing dataset.

All 12 full transcript traces match the model. Maximum mean serialized-size deviation from the sampled estimate: 0.100%.

## N = 2^22

| eta | Chosen baseline | Baseline proof KiB | Split Canopy proof KiB | Saving | Baseline s | Split Canopy s |
|---:|---|---:|---:|---:|---:|---:|
| 1 | bolt_merkle_shared | 6455.01 | 4143.60 | 35.8% | 0.541 | 0.881 |
| 2 | bolt_merkle_shared | 12885.63 | 8146.11 | 36.8% | 1.082 | 1.854 |
| 4 | bolt_merkle_shared | 25823.42 | 16199.65 | 37.3% | 2.427 | 4.500 |
| 8 | bolt_merkle_shared | 51857.09 | 32417.92 | 37.5% | 5.492 | 12.104 |

The paper exports `paper_proof_sizes.csv` and `paper_timing_summary.csv` contain 56 size/hash rows and 24 timing rows for its two Bolt configurations.

Files: `comparison.csv` contains all 28 headline comparisons; `all_proof_sizes.csv` retains all four architectures with verifier hashes and FS bytes; `proof_sizes/independent_height_search.csv` retains every side candidate; `timing_summary.csv` and `timing_stage_summary.csv` contain medians and component means; raw repetitions and process RSS are retained in the timing directories.

Reproduce: build `gf256_bolt_split` in release mode without `fs-accounting`, run its `estimate` command, then `python3 archive/scripts/run_gf256_split_timings.py`. Separately build with `--features fs-accounting` and run `gf256_bolt_split validate`. Finally run `python3 archive/scripts/summarize_gf256_split_suite.py`. Do not run other compute-heavy jobs concurrently with timing.
