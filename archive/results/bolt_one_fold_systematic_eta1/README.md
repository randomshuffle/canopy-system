# Bolt + Canopy: join after exactly one systematic Canopy fold

The join round is fixed at one. Only the height is selected within that fixed-round slice of the saved search; h_X=0 is selected at every size. These points were already evaluated, so the proof/hash data are reused after verifying their geometry against every one-fold height candidate. The immediate-join control uses the identical height and initial block lengths.

Parameters: eta=1, Bolt-max w=4, GF(2^256), BLAKE3, and the retained conditional 128-bit soundness target. With k=N/4, systematic Canopy starts with message k and block 2k (rate 1/2). One three-variable WHIR fold reduces its message to k/8 and block to k; it then joins the systematic code-switch claim at rate 1/8. The parity setup stays at h_Y=3 and block k/2. Verifier hashes include authentication, hash-to-field, and Fiat–Shamir finalizations.

## Proof size and verifier hashes

| N | Immediate proof (KiB) | One-fold proof (KiB) | Increase | Immediate hashes | One-fold hashes |
|---|---:|---:|---:|---:|---:|
| 2^18 | 3199.23 | 3244.21 | 1.41% | 52,583 | 53,376 |
| 2^20 | 3309.63 | 3361.47 | 1.57% | 55,791 | 56,804 |
| 2^22 | 3411.82 | 3469.82 | 1.70% | 58,917 | 60,127 |
| 2^24 | 3516.61 | 3581.19 | 1.84% | 62,133 | 63,553 |
| 2^26 | 3633.06 | 3704.39 | 1.96% | 65,622 | 67,258 |
| 2^28 | 3740.34 | 3818.91 | 2.10% | 68,940 | 70,808 |
| 2^30 | 3850.30 | 3934.75 | 2.19% | 72,350 | 74,407 |

## Matched prover timings at N=2^22

One warmup and five measured repetitions per configuration, in rotating order, eight Rayon threads. All 18 complete serialized proofs verified. Commitment and opening include their FS work; input generation, public setup, and verification are excluded. Spark/Mulperm remain outside the public-H adapter scope. Phase columns are means; total is a median.

| Configuration | Commit (s) | Open (s) | Total (s) | Total min–max (s) |
|---|---:|---:|---:|---:|
| Bolt + Merkle | 0.212 | 0.389 | 0.601 | 0.591–0.610 |
| Canopy: immediate join | 0.433 | 0.523 | 0.955 | 0.951–0.967 |
| Canopy: one-fold join | 0.435 | 0.468 | 0.908 | 0.883–0.916 |

One fold reduces mean opening time by 10.5% and median total time by 4.9% relative to the immediate-join control in this matched run. Both variants perform the same commitment algorithm, so measured commitment differences reflect run variation.

## Systematic recursive opening components

| Component | Immediate (s) | One fold (s) |
|---|---:|---:|
| authenticated WHIR alignment and constraint combination | 0.052 | 0.147 |
| distinct-functional cross evaluations | 0.046 | 0.006 |
| shared recursive WHIR proof | 0.119 | 0.035 |

The smaller shared tail reduces cross-evaluation and recursive-proof work. The separate authenticated fold adds alignment work. Both effects are included in the opening total.

## Reproduce

```sh
python3 archive/scripts/report_bolt_one_fold.py --prepare
cargo build --offline --release --example gf256_bolt_split
python3 scripts/measure_gf256_run.py --record-dir archive/results/bolt_one_fold_systematic_eta1/timing_n22 --threads 8 -- target/release/examples/gf256_bolt_split time --log 22 --inputs 1 --warmups 1 --repetitions 5 --width 4 --estimates archive/results/bolt_one_fold_systematic_eta1/one_fold --alternate-estimates archive/results/bolt_one_fold_systematic_eta1/immediate_control --paper-only --output archive/results/bolt_one_fold_systematic_eta1/timing_n22
python3 archive/scripts/report_bolt_one_fold.py
```

The raw measurements and stage summaries are retained alongside the complete seven-size proof/hash grid. The prior study validated full FS traces for both join geometries at N=2^18 and N=2^22. No prover timing above N=2^22 is measured or extrapolated here.
