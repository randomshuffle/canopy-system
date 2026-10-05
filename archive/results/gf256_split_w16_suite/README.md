# Bolt at w=16: independent Canopy layers with blocks k and k/2

At N=2^22, blocks k/2 are 11.3--15.1% faster than blocks k, with 4.4--4.5% larger proofs. Relative to the matching Merkle baseline, the prover overhead is 7.6--14.4% for k/2 and 22.7--34.7% for k. Full blocks give smaller proofs at every grid point. The shared WHIR tail is the smaller-proof Merkle baseline at every grid point.

At N=2^22 the selected heights (h_X,h_Y) are (1,2) for blocks k and (2,3) for blocks k/2, for every eta. Wider rows reduce code-switch and Canopy RS message lengths at fixed N, but increase the number of field values communicated per queried row. Thus the w=16 prover is faster than the retained w=4 runs, while proofs are substantially larger.

Each input has N field elements and k=N/16 systematic rows. The systematic and parity Canopy layers both use the indicated RS block length; their heights and WHIR join lengths are optimized independently. The field is GF(2^256), with BLAKE3 and the same conditional 128-bit target, gamma=0.013 distance assumption, Khatam systematic query radius, parity radius 1/4, and public-H scope as the previous experiment. Spark/Mulperm preprocessing and proofs are excluded.

Proof sizes and verifier hashes are means over 100 deterministic query samples. The Merkle baseline is the smaller proof among the optimized shared and separate WHIR tails. Full-height constant Canopy layers are included and opened directly. Each nonconstant side batches eta Canopy SEP claims with eta corresponding code-switch claims, using native full cross matrices. Both sides use distinct challenges within one Fiat-Shamir transcript.

At N=2^18, k=16,384 is below the systematic query budget, so every systematic row is opened. The size model and executable prover both account for this saturation.

Timings use identical inputs, rotating the selected Merkle baseline, Canopy k/2, and Canopy k within each repetition. Each configuration uses one warmup and five measured repetitions, with eight Rayon threads. All 216 proofs are serialized and verified. Totals include commitment and opening, excluding input generation, public setup, and verification.

All 30 executed Fiat-Shamir verifier traces match the model. Maximum mean serialized-size deviation from the sampled estimate: 0.038%. Verifier hashes include authentication, hash-to-field, and Fiat-Shamir hash/XOF finalizations; transcript input/output bytes are separately available in the CSVs. Fiat-Shamir adds zero proof bytes.

## N=2^22 comparison

| eta | Method | Proof KiB | Verifier hashes | Commit s | Open s | Total s |
|---:|---|---:|---:|---:|---:|---:|
| 1 | Merkle | 13,113.68 | 135,614 | 0.151 | 0.241 | 0.392 |
| 1 | Canopy k | 11,959.78 | 78,676 | 0.190 | 0.304 | 0.486 |
| 1 | Canopy k/2 | 12,488.59 | 95,833 | 0.170 | 0.263 | 0.431 |
| 2 | Merkle | 26,319.36 | 271,330 | 0.320 | 0.494 | 0.814 |
| 2 | Canopy k | 23,937.20 | 155,443 | 0.382 | 0.623 | 0.999 |
| 2 | Canopy k/2 | 25,011.16 | 190,319 | 0.351 | 0.536 | 0.876 |
| 4 | Merkle | 52,909.59 | 544,252 | 0.882 | 0.996 | 1.602 |
| 4 | Canopy k | 48,063.70 | 309,892 | 0.927 | 1.517 | 2.100 |
| 4 | Canopy k/2 | 50,235.31 | 380,487 | 0.779 | 1.215 | 1.795 |
| 8 | Merkle | 106,457.62 | 1,093,186 | 1.369 | 2.053 | 3.366 |
| 8 | Canopy k | 96,674.29 | 620,766 | 1.604 | 2.978 | 4.534 |
| 8 | Canopy k/2 | 101,052.96 | 763,271 | 1.426 | 2.544 | 3.851 |

Commitment and opening are means; total time is the median, so the columns need not add exactly.

## Full proof-size and verifier-hash grid

| log2 N | eta | Merkle KiB / hashes | Canopy k KiB / hashes | Canopy k/2 KiB / hashes | Heights k; k/2 |
|---:|---:|---:|---:|---:|---|
| 18 | 1 | 8,707.64 / 54,573 | 8,709.96 / 54,097 | 8,728.22 / 55,658 | (3,2); (6,3) |
| 18 | 2 | 17,374.34 / 108,724 | 17,359.11 / 107,259 | 17,385.64 / 113,389 | (3,2); (14,2) |
| 18 | 4 | 34,711.31 / 217,040 | 34,638.20 / 225,417 | 34,682.57 / 225,518 | (14,2); (14,2) |
| 18 | 8 | 69,400.61 / 433,772 | 69,206.85 / 449,644 | 69,293.76 / 449,882 | (14,2); (14,2) |
| 20 | 1 | 11,888.85 / 96,564 | 11,685.70 / 72,184 | 11,907.65 / 96,120 | (1,3); (5,3) |
| 20 | 2 | 23,876.47 / 193,268 | 23,419.16 / 143,970 | 23,887.90 / 196,008 | (1,2); (16,3) |
| 20 | 4 | 48,015.33 / 387,670 | 47,040.37 / 287,503 | 47,999.81 / 392,408 | (1,2); (16,3) |
| 20 | 8 | 96,625.95 / 778,460 | 94,627.13 / 576,280 | 96,555.89 / 789,643 | (1,2); (16,2) |
| 22 | 1 | 13,113.68 / 135,614 | 11,959.78 / 78,676 | 12,488.59 / 95,833 | (1,2); (2,3) |
| 22 | 2 | 26,319.36 / 271,330 | 23,937.20 / 155,443 | 25,011.16 / 190,319 | (1,2); (2,3) |
| 22 | 4 | 52,909.59 / 544,252 | 48,063.70 / 309,892 | 50,235.31 / 380,487 | (1,2); (2,3) |
| 22 | 8 | 106,457.62 / 1,093,186 | 96,674.29 / 620,766 | 101,052.96 / 763,271 | (1,2); (2,3) |
| 24 | 1 | 14,441.13 / 178,100 | 12,129.80 / 82,803 | 12,750.15 / 102,142 | (1,3); (2,3) |
| 24 | 2 | 28,979.32 / 356,467 | 24,239.37 / 163,543 | 25,495.92 / 201,955 | (1,2); (2,3) |
| 24 | 4 | 58,251.90 / 715,243 | 48,626.19 / 325,009 | 51,171.08 / 402,872 | (1,2); (2,3) |
| 24 | 8 | 117,200.24 / 1,437,029 | 97,761.07 / 650,003 | 102,897.66 / 807,275 | (1,2); (2,3) |
| 26 | 1 | 15,828.20 / 222,332 | 12,261.96 / 86,737 | 12,898.01 / 106,405 | (1,3); (2,3) |
| 26 | 2 | 31,745.16 / 444,821 | 24,471.34 / 169,640 | 25,767.86 / 209,714 | (1,3); (2,3) |
| 26 | 4 | 63,796.58 / 892,526 | 49,064.89 / 336,438 | 51,691.55 / 417,622 | (1,3); (2,3) |
| 26 | 8 | 128,341.27 / 1,793,404 | 98,603.22 / 675,376 | 103,919.04 / 836,097 | (1,2); (2,3) |
| 28 | 1 | 17,236.80 / 267,274 | 12,387.47 / 90,593 | 13,042.30 / 110,650 | (1,3); (2,3) |
| 28 | 2 | 34,557.26 / 534,676 | 24,687.09 / 176,344 | 26,010.85 / 216,997 | (1,3); (2,3) |
| 28 | 4 | 69,433.30 / 1,072,769 | 49,460.54 / 348,817 | 52,132.86 / 431,003 | (1,3); (2,3) |
| 28 | 8 | 139,669.55 / 2,155,780 | 99,368.03 / 695,789 | 104,752.36 / 861,523 | (1,3); (2,3) |
| 30 | 1 | 18,640.84 / 312,199 | 12,523.43 / 94,715 | 13,181.88 / 113,925 | (1,3); (2,4) |
| 30 | 2 | 37,368.51 / 624,631 | 24,911.37 / 183,282 | 26,241.05 / 224,077 | (1,3); (2,3) |
| 30 | 4 | 75,078.47 / 1,253,410 | 49,861.21 / 361,377 | 52,545.42 / 443,858 | (1,3); (2,3) |
| 30 | 8 | 151,024.43 / 2,519,132 | 100,120.57 / 719,567 | 105,534.66 / 886,085 | (1,3); (2,3) |

## Full prover-time grid

| log2 N | eta | Merkle s | Canopy k s | Canopy k/2 s |
|---:|---:|---:|---:|---:|
| 18 | 1 | 0.038 | 0.042 | 0.040 |
| 18 | 2 | 0.079 | 0.085 | 0.087 |
| 18 | 4 | 0.160 | 0.178 | 0.172 |
| 18 | 8 | 0.324 | 0.358 | 0.355 |
| 20 | 1 | 0.119 | 0.139 | 0.126 |
| 20 | 2 | 0.241 | 0.305 | 0.257 |
| 20 | 4 | 0.482 | 0.612 | 0.535 |
| 20 | 8 | 0.998 | 1.303 | 1.033 |
| 22 | 1 | 0.392 | 0.486 | 0.431 |
| 22 | 2 | 0.814 | 0.999 | 0.876 |
| 22 | 4 | 1.602 | 2.100 | 1.795 |
| 22 | 8 | 3.366 | 4.534 | 3.851 |

## Files and reproduction

`comparison.csv` contains all 28 comparisons and retained w=4, k/2 results. Width comparisons use historical w=4 timings, not simultaneous reruns. `all_proof_sizes.csv` retains both Merkle architectures and both Canopy block lengths; `selected_proof_sizes.csv` exports the three headline methods. Timing samples, component breakdowns, process RSS, candidate searches, and source/binary identities are retained alongside the summaries.

Build `gf256_bolt_split` and `gf256_bolt_suite` in release mode without `fs-accounting`. Run `gf256_bolt_split estimate --width 16 --block-divisor 1 --output archive/results/gf256_split_w16_suite/full/proof_sizes` and again with divisor 2 and output `half/proof_sizes`. Run `gf256_bolt_suite estimate --width 16 --output archive/results/gf256_split_w16_suite/shared/proof_sizes`, then `gf256_bolt_suite account --input .../shared/proof_sizes/proof_sizes.csv --output .../shared/fiat_shamir.csv`. Run `python3 archive/scripts/run_gf256_split_w16_timings.py` sequentially, without concurrent compute-heavy work. Build separately with `--features fs-accounting` and run `python3 archive/scripts/validate_gf256_split_w16_suite.py` to check both split variants and the Merkle-only rows of the shared estimates. Finally run `python3 archive/scripts/summarize_gf256_split_w16_suite.py`.
