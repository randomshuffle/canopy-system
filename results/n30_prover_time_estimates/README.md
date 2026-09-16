# Estimated prover times at N=2^30

**These are extrapolations, not N=2^30 benchmarks.** All five configurations use GF(2^256), BLAKE3, eight Rayon threads, and eta input polynomials of N coefficients each. Bolt is restored to w=4 and both Canopy RS blocks k/2 (k=N/4), with independent systematic/parity batches. Canopy heights are (2,3), and the Merkle baseline uses its optimized shared batch. WHIR timings retain the measured three-variable folding schedule and Exp. 3 height 4.

## Estimated median commitment + opening time (minutes)

| Configuration | eta=1 | eta=2 | eta=4 | eta=8 |
|---|---:|---:|---:|---:|
| WHIR batched, rate 1/2 | 5.6 | 10.7 | 20.5 | 41.0 |
| WHIR batched, rate 1/4 | 8.0 | 14.7 | 28.2 | 56.0 |
| Canopy + WHIR, Exp. 3 | 7.4 | 13.2 | 24.5 | 58.0 |
| Bolt + Merkle, w=4 | 2.8 | 5.7 | 12.3 | 26.8 |
| Bolt + Canopy, w=4, k/2 | 3.8 | 7.6 | 17.9 | 39.7 |

## Commitment/opening breakdown for eta=1

| Configuration | Commitment min | Opening min | Total min |
|---|---:|---:|---:|
| WHIR batched, rate 1/2 | 2.69 | 3.19 | 5.62 |
| WHIR batched, rate 1/4 | 3.73 | 4.43 | 7.97 |
| Canopy + WHIR, Exp. 3 | 3.75 | 3.47 | 7.38 |
| Bolt + Merkle, w=4 | 0.99 | 1.82 | 2.77 |
| Bolt + Canopy, w=4, k/2 | 1.48 | 2.40 | 3.79 |

Commitment and opening are projected means; totals are projected medians, so the displayed columns need not sum exactly.

## Method and limits

The main estimates scale measured mean stage costs at N=2^22, independently for each eta, to obtain commitment/opening growth factors. Those phase factors are applied to all five saved repetitions, and the headline is their median projected total. Therefore the measured batching cost at eta=8 is retained, rather than estimated by multiplying the eta=1 total by eight. Commitment and opening both count; input generation, setup, and verification are excluded.

Hashing, sparse Bolt matrix operations, evaluations, dot products, dense covectors, and cross-evaluation work scale with N. The linear factor is 2^(30-22)=256. RS and transposed-FFT stages scale with N log2(M), where M is the per-column coefficient count inferred from the implementation. Combined RS/commitment and alignment stages use that same FFT proxy for their aggregate cost. Fixed query-copy work remains fixed; full Merkle path collection grows with tree height. Recursive WHIR work sums a geometric sequence of codeword sizes and decreasing coefficient dimensions over the three-variable schedule, so the extra rounds at N=2^30 are included. This is an approximate work model, not cycle-accurate simulation; basis conversion, cache behavior, and degree-dependent query-count changes are not separately fitted.

Saved WHIR substages have an explicit signed adjustment from independently measured suffix subtraction. Positive stage weights are normalized to their observed commitment/opening means before extrapolation; no negative work is projected. The Bolt stage timings already partition the measured execution. Stage sums are checked against source phase means; the five source repetitions also reproduce every saved median.

As a local check, the same model predicts measured N=2^22 medians from N=2^20 with a median absolute error of 6.0% and maximum absolute error of 16.9%. This much shorter extrapolation is not evidence of a confidence interval at N=2^30. Per-configuration errors are in `n20_to_n22_backtest.csv`.

The table assumes comparable per-thread throughput with sufficient resident RAM and no swapping. One input polynomial alone is 32 GiB; eta=8 inputs occupy 256 GiB before codewords, trees, covectors, and temporary buffers. The current 16 GiB machine cannot execute these workloads in memory. Memory bandwidth, paging, and a different larger-memory machine could materially change actual time; those penalties are not included.

These preserve the existing experiment scopes: WHIR/Canopy is a composition of measured, verifier-checked components, whereas Bolt measurements are complete serialized public-H adapter proofs, excluding Spark/Mulperm preprocessing and proofs. Executed Fiat-Shamir work is already included in the source timers.

## Scaling sensitivity (not confidence bounds)

| Configuration | eta | Linear-only bulk model min | Main stage model min | Stage model from N=2^20 min |
|---|---:|---:|---:|---:|
| WHIR batched, rate 1/2 | 1 | 3.9 | 5.6 | 5.8 |
| WHIR batched, rate 1/2 | 2 | 7.5 | 10.7 | 10.1 |
| WHIR batched, rate 1/2 | 4 | 14.5 | 20.5 | 18.5 |
| WHIR batched, rate 1/2 | 8 | 29.1 | 41.0 | 36.2 |
| WHIR batched, rate 1/4 | 1 | 5.5 | 8.0 | 9.4 |
| WHIR batched, rate 1/4 | 2 | 10.3 | 14.7 | 17.2 |
| WHIR batched, rate 1/4 | 4 | 19.9 | 28.2 | 30.1 |
| WHIR batched, rate 1/4 | 8 | 39.7 | 56.0 | 59.2 |
| Canopy + WHIR, Exp. 3 | 1 | 5.2 | 7.4 | 7.4 |
| Canopy + WHIR, Exp. 3 | 2 | 9.3 | 13.2 | 13.0 |
| Canopy + WHIR, Exp. 3 | 4 | 17.4 | 24.5 | 25.3 |
| Canopy + WHIR, Exp. 3 | 8 | 41.2 | 58.0 | 55.4 |
| Bolt + Merkle, w=4 | 1 | 2.3 | 2.8 | 2.9 |
| Bolt + Merkle, w=4 | 2 | 4.8 | 5.7 | 5.8 |
| Bolt + Merkle, w=4 | 4 | 10.4 | 12.3 | 13.3 |
| Bolt + Merkle, w=4 | 8 | 22.7 | 26.8 | 25.1 |
| Bolt + Canopy, w=4, k/2 | 1 | 3.0 | 3.8 | 4.4 |
| Bolt + Canopy, w=4, k/2 | 2 | 6.2 | 7.6 | 8.6 |
| Bolt + Canopy, w=4, k/2 | 4 | 14.8 | 17.9 | 17.6 |
| Bolt + Canopy, w=4, k/2 | 8 | 33.4 | 39.7 | 37.1 |

The linear-only column removes FFT log growth and extra-round growth while retaining fixed/logarithmic query work. The N=2^20-anchor column applies the main model to the smaller measured dataset. Their differences show sensitivity to scaling assumptions and observed throughput; they are not hard lower/upper bounds.

Files: `estimates.csv` contains all 20 median totals, mean phase estimates, and projected repetition ranges; `prover_time_estimates.csv` supplies the 80 estimated timing points at N=2^24, 2^26, 2^28, and 2^30 for the manuscript plots. `stage_estimates.csv` contains every measured stage, reconciliation factor, scaling rule, and projected contribution. `manifest.json` records source-file hashes. No new large-input proofs were generated.

Reproduce with `python3 scripts/estimate_n30_prover_times.py`.
