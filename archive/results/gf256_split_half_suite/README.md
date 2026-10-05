# Independent Bolt batches with k/2 Canopy RS blocks

Both Canopy RS block lengths are fixed at k/2=N/8, with k=N/4 systematic rows. Width is 4; inputs are eta independent length-N polynomials. The field, security assumptions, hashes, query counts, and public-H scope match the existing batched suite.

For each nonconstant Canopy layer, its tail batches eta code-switch claims with eta corresponding Canopy claims. A full-height constant layer is opened directly, leaving eta code-switch claims in that tail. Both tails share a domain-separated Fiat–Shamir transcript but use distinct challenges. Each side optimizes its own height and join length. The full-height constant-layer alternatives are included. The search permits several authenticated folds of at most three variables; all selected configurations for N>=2^20 join after one fold per side, with h_X=2 and h_Y=3. At N=2^18, the systematic height is 5 for eta=1 and the full height 16 for eta=2,4,8; the parity height is 3 except for eta=8, where it is 2. Native full cross matrices are retained within each batch.

The Merkle baseline is optimized in both shared-tail and split-tail form; the smaller-proof configuration is the baseline at each point. The 100-query-sample proof estimates include verifier authentication, hash-to-field, and Fiat–Shamir finalizations, with transcript bytes separately. Fiat–Shamir adds no proof bytes.

The selected Merkle baseline and Canopy configuration were rerun in alternating order, with one warmup and five repetitions per (N,eta), eight Rayon threads. All 144 proofs were verified. Timing includes commitment and opening; verification/input generation/setup are excluded. The paper reports the selected Merkle baseline and split Canopy; the WHIR experiment retains its existing dataset.

All 12 full transcript traces match the model. Maximum mean serialized-size deviation from the sampled estimate: 0.115%.

## N = 2^22

| eta | Chosen baseline | Baseline proof KiB | Split Canopy proof KiB | Saving | Baseline s | Split Canopy s |
|---:|---|---:|---:|---:|---:|---:|
| 1 | bolt_merkle_shared | 6455.01 | 4763.28 | 26.2% | 0.568 | 0.732 |
| 2 | bolt_merkle_shared | 12885.63 | 9402.67 | 27.0% | 1.178 | 1.502 |
| 4 | bolt_merkle_shared | 25823.42 | 18741.82 | 27.4% | 2.546 | 3.568 |
| 8 | bolt_merkle_shared | 51857.09 | 37552.73 | 27.6% | 5.576 | 8.052 |

Comparisons against block length k use the retained prior split-suite measurements; the baseline and k/2 timings are fresh matched runs. Proof sizes and verifier hashes are reoptimized on the full grid.

The paper exports `paper_proof_sizes.csv` and `paper_timing_summary.csv` contain 56 size/hash rows and 24 timing rows for its two Bolt configurations.

Files: `comparison.csv` contains all 28 headline comparisons; `all_proof_sizes.csv` retains the two Merkle architectures and split Canopy with verifier hashes and FS bytes; `proof_sizes/independent_height_search.csv` retains every side candidate; `timing_summary.csv` and `timing_stage_summary.csv` contain medians and component means; raw repetitions and process RSS are retained in the timing directories.

Reproduce: build `gf256_bolt_split` in release mode without `fs-accounting`, run `estimate --block-divisor 2 --output archive/results/gf256_split_half_suite/proof_sizes`, then `python3 archive/scripts/run_gf256_split_half_timings.py`. Separately build with `--features fs-accounting` and run `gf256_bolt_split validate --input archive/results/gf256_split_half_suite/proof_sizes/proof_sizes.csv --output archive/results/gf256_split_half_suite/fiat_shamir_validation.csv`. Finally run `python3 archive/scripts/summarize_gf256_split_half_suite.py`. Do not run other compute-heavy jobs concurrently with timing.
