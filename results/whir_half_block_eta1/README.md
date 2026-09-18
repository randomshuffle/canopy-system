# WHIR with a half-length Canopy codeword, eta = 1

All results use GF(2^256), BLAKE3, a composed 128-bit target, and one input polynomial. The base rate-1/2 oracle has 2N field elements; the Canopy RS codeword now has N. The selected initial fold remains three variables and h=4, so its message is N/8 and the aligned shared tail starts at rate 1/8 (previously 1/16). Both the Canopy and folded base oracle match in message and block length. No Bolt parameters are changed by this WHIR experiment.

Proof sizes and verifier hashes cover all seven requested sizes using 100 deterministic query samples. Each method optimizes its folding schedule over {3,4}. Verifier hashes include authentication, H2F, and Fiat–Shamir finalizations. The two Merkle baselines reproduce the saved proof sizes and hash counts exactly.

## Proof size (KiB) / verifier hashes

| N | WHIR 1/2 | WHIR 1/4 | Canopy, prior full length | Canopy, half length |
|---|---:|---:|---:|---:|
| 2^18 | 237.80 / 4,251 | 157.52 / 2,901 | 206.42 / 3,026 | 226.72 / 3,324 |
| 2^20 | 280.49 / 5,624 | 186.08 / 3,856 | 230.50 / 3,833 | 255.79 / 4,260 |
| 2^22 | 329.04 / 6,597 | 219.78 / 4,330 | 259.26 / 4,149 | 290.73 / 4,796 |
| 2^24 | 370.86 / 8,072 | 248.80 / 5,462 | 283.87 / 5,139 | 318.65 / 5,826 |
| 2^26 | 415.49 / 9,587 | 279.18 / 5,696 | 309.45 / 5,423 | 349.32 / 6,894 |
| 2^28 | 460.53 / 10,458 | 309.33 / 6,720 | 335.29 / 6,309 | 380.53 / 7,323 |
| 2^30 | 503.68 / 11,023 | 338.30 / 7,312 | 360.32 / 6,776 | 410.49 / 7,844 |

## Measured prover time at N = 2^22, eta = 1

One warmup and five measured repetitions per configuration, eight Rayon threads. Times include commitment and opening, excluding input generation, setup, and verification. These retain the component-composed timing scope: native WHIR components are verifier-checked, but this is not a single serialized end-to-end Canopy argument.

| Configuration | Mean commit (s) | Mean open (s) | Median total (s) | Min–max total (s) |
|---|---:|---:|---:|---:|
| WHIR, rate 1/2 | 0.335 | 0.393 | 0.731 | 0.717–0.737 |
| WHIR, rate 1/4 | 0.598 | 0.832 | 1.468 | 1.260–1.483 |
| Canopy, half-length | 0.466 | 0.432 | 0.903 | 0.882–0.903 |

A fresh full-length control run measured Canopy at 1.011 s (range 1.009–1.044 s), versus 0.903 s for half length: 10.8% lower in these runs. The runs were sequential, not interleaved; this is a sample comparison, not a confidence bound. Both controls and raw stages are retained. Means of the phases need not sum to the median total.

At N=2^30, halving the Canopy block raises proof size by 13.9% and verifier hashes by 15.8% relative to the previous full-length Canopy configuration. The higher rate saves encoding/commitment work but increases the query cost.

## Reproduce

```sh
cargo build --offline --release --bin canopy-system --example whir_fiat_shamir
RAYON_NUM_THREADS=8 target/release/canopy-system paper-estimate --inputs 1 --samples 100 --canopy-codeword-log-reduction 1 --output-dir results/whir_half_block_eta1/proof_sizes
target/release/examples/whir_fiat_shamir --input-dir results/whir_half_block_eta1/proof_sizes --output results/whir_half_block_eta1/fiat_shamir.csv
python3 scripts/measure_gf256_run.py --record-dir results/whir_half_block_eta1/timing_n22 --threads 8 -- target/release/canopy-system paper-time --log-length 22 --inputs 1 --warmups 1 --repetitions 5 --seed 73947765755993 --canopy-codeword-log-reduction 1 --output-dir results/whir_half_block_eta1/timing_n22
# Repeat the timing command with reduction 0 and output/record directory full_length_control_n22 for the control.
python3 scripts/summarize_whir_half_block.py
```

`proof_sizes_with_hashes.csv` has all 21 selected rows; `comparison.csv` compares old and new Canopy sizes/hashes. Raw timings, stage CSVs, process RSS, and binary identities are retained in `timing_n22/` and `full_length_control_n22/`. This rerun does not overwrite the manuscript data or extrapolate new larger-size timings.
