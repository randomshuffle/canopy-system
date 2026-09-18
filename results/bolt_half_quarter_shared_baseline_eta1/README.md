# Bolt: fixed shared Merkle baseline, Canopy blocks k/2 and k/4

This rerun fixes the Merkle baseline to the paper shared-tail architecture, with join message k/8. It does not select the slower separate-tail Merkle variant by proof size. The two Canopy configurations use separate systematic/parity WHIR batches and equal RS block lengths k/2 or k/4 on the two sides. Each side optimizes its height for proof size independently, including full-height constant layers. There are no independent Canopy folds before batching: each nonconstant Canopy message joins its own code-switch claim at its original length.

Parameters are w=4, k=N/4, eta=1, GF(2^256), BLAKE3, and the retained 128-bit conditional soundness target (Bolt distance gamma=0.013 remains an explicit assumption). The systematic and parity query counts remain 20,977 and 319. Proof sizes average 100 deterministic query samples; verifier hashes include authentication, hash-to-field, and Fiat–Shamir finalizations. Fiat–Shamir adds no proof bytes. The fresh Merkle and k/2 proof/hash grids reproduce the paper values exactly.

## Proof size (KiB) / verifier hashes

| N | Shared Merkle baseline | Canopy k/2 | Canopy k/4 |
|---|---:|---:|---:|
| 2^18 | 3902.52 / 96,565 | 3921.26 / 96,119 | 3941.29 / 98,354 |
| 2^20 | 5127.26 / 135,613 | 4502.15 / 95,831 | 4940.69 / 110,779 |
| 2^22 | 6455.01 / 178,108 | 4763.28 / 102,127 | 5365.96 / 120,619 |
| 2^24 | 7841.29 / 222,315 | 4910.87 / 106,380 | 5578.08 / 126,208 |
| 2^26 | 9250.87 / 267,289 | 5056.00 / 110,652 | 5741.03 / 130,881 |
| 2^28 | 10654.85 / 312,211 | 5195.94 / 114,875 | 5862.55 / 133,814 |
| 2^30 | 12074.96 / 357,540 | 5315.15 / 118,673 | 6002.74 / 138,059 |

## Fresh matched prover times at N=2^22

All three configurations use the same release binary and identical inputs, run in rotating order with two warmups and ten measured repetitions per configuration, eight Rayon threads. No other benchmark or build ran during timing. All 36 warmup/measured serialized proofs verified. Times include commitment and opening, including FS; input generation, public setup, and verification are excluded. Spark/Mulperm remain outside the public-H adapter scope. Phase times are means and total times are medians.

| Configuration | Commit (s) | Open (s) | Total (s) | Total min–max (s) | Total overhead vs Merkle |
|---|---:|---:|---:|---:|---:|
| Bolt + Merkle (shared tail) | 0.227 | 0.345 | 0.558 | 0.538–0.673 | 0.0% |
| Bolt + Canopy, k/2 | 0.282 | 0.436 | 0.711 | 0.699–0.770 | 27.5% |
| Bolt + Canopy, k/4 | 0.248 | 0.381 | 0.612 | 0.590–0.706 | 9.8% |

At N=2^22, reducing both Canopy blocks from k/2 to k/4 saves 13.9% in median total prover time and increases proof size by 12.7%. All headline timing comparisons come from this one matched run, rather than mixing Table 1 timings with later measurements.

## Selected heights

| N | k/2: h_X, h_Y | k/4: h_X, h_Y |
|---|---:|---:|
| 2^18 | 5, 3 | 16, 3 |
| 2^20 | 2, 3 | 3, 3 |
| 2^22 | 2, 3 | 3, 3 |
| 2^24 | 2, 3 | 3, 3 |
| 2^26 | 2, 3 | 3, 3 |
| 2^28 | 2, 3 | 3, 4 |
| 2^30 | 2, 3 | 3, 4 |

At N=2^18 the k/4 systematic layer is constant and opened directly (h_X=16). At N=2^22 the systematic messages are k/4 for k/2 blocks and k/8 for k/4 blocks. The parity message is k/32 in both cases at that size.

All six full FS transcript validations match the model, covering all three headline configurations at N=2^18 and N=2^22. Maximum mean serialized-size discrepancy is 0.030%; the size model excludes small serializer framing. Timing-process peak RSS was 0.956 GiB, including setup and verification with schemes run sequentially.

## Reproduce

```sh
cargo build --offline --release --example gf256_bolt_suite --example gf256_bolt_split
RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_suite estimate --inputs 1 --samples 100 --width 4 --output results/bolt_half_quarter_shared_baseline_eta1/shared_baseline
target/release/examples/gf256_bolt_suite account --input results/bolt_half_quarter_shared_baseline_eta1/shared_baseline/proof_sizes.csv --output results/bolt_half_quarter_shared_baseline_eta1/shared_baseline/fiat_shamir.csv
RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_split estimate --inputs 1 --samples 100 --width 4 --block-divisor 2 --output results/bolt_half_quarter_shared_baseline_eta1/half
RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_split estimate --inputs 1 --samples 100 --width 4 --block-divisor 4 --output results/bolt_half_quarter_shared_baseline_eta1/quarter
python3 scripts/measure_gf256_run.py --record-dir results/bolt_half_quarter_shared_baseline_eta1/timing_n22 --threads 8 -- target/release/examples/gf256_bolt_split time --log 22 --inputs 1 --warmups 2 --repetitions 10 --width 4 --estimates results/bolt_half_quarter_shared_baseline_eta1/half --alternate-estimates results/bolt_half_quarter_shared_baseline_eta1/quarter --shared-estimates results/bolt_half_quarter_shared_baseline_eta1/shared_baseline --paper-only --shared-merkle-baseline --output results/bolt_half_quarter_shared_baseline_eta1/timing_n22
cargo build --offline --release --features fs-accounting --example gf256_bolt_suite --example gf256_bolt_split
RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_suite validate --input results/bolt_half_quarter_shared_baseline_eta1/shared_baseline/merkle_only.csv --output results/bolt_half_quarter_shared_baseline_eta1/fiat_shamir_validation_baseline.csv
RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_split validate --input results/bolt_half_quarter_shared_baseline_eta1/half/canopy_only.csv --output results/bolt_half_quarter_shared_baseline_eta1/fiat_shamir_validation_half.csv
RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_split validate --input results/bolt_half_quarter_shared_baseline_eta1/quarter/canopy_only.csv --output results/bolt_half_quarter_shared_baseline_eta1/fiat_shamir_validation_quarter.csv
python3 scripts/summarize_bolt_half_quarter.py
```

`merkle_only.csv` filters the shared estimate rows to `bolt_merkle_batched`; each `canopy_only.csv` filters its split estimates to rows with a systematic Canopy height. These validation inputs are retained. Build without `fs-accounting` before taking timings. No new larger-input times are measured or extrapolated, and the manuscript tables remain the saved paper snapshot.
