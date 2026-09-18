# Bolt: half-rate systematic Canopy and delayed joins, eta = 1

The search keeps Bolt-max w=4, GF(2^256), BLAKE3, the existing 128-bit conditional soundness assumptions, and the selected parity configuration. The parity Canopy block is k/2, with k=N/4, and its height remains 3. Systematic Canopy starts at rate 1/2: at height h its message has k/2^h elements and its block has 2k/2^h. Thus this experiment releases the old fixed systematic block-length constraint.

Each candidate folds systematic Canopy zero or more times before batching with the systematic Bolt code-switch claim. Each independent Canopy fold reduces the message by 8 and the block by 2. After r folds, the join message has k/2^(h+3r) coefficients and inverse rate 2^(1+2r). Both claims are authenticated before they enter the shared native matrix-batched WHIR tail. The parity tail stays separate. All added commitments, query openings, cross-matrix entries, and transcript operations are counted.

The search includes every height from 0 through the full tree, all feasible joins after complete three-variable Canopy folds, and full-height constant layers opened directly. It minimizes proof bytes; it does not optimize prover time or search other initial Canopy rates. Proof estimates average 100 deterministic query samples. Verifier hashes count authentication, hash-to-field, and Fiat–Shamir hash/XOF finalizations; FS adds no proof bytes.

For a fair baseline, the Merkle split-tail search also includes immediate joins, without the formerly required prefix transition. It slightly improves the Merkle proof at N=2^18 through 2^24 (at most 0.27%); the retained shared-tail baseline remains smaller at N=2^26 through 2^30. Timings below use the newly selected smaller-proof baseline. The boundary audit is in `proof_sizes/merkle_immediate_join_check.csv`.

**Result:** every size selects h=0 and zero independent Canopy folds. This is an immediate join, with a systematic Canopy block of 2k=N/2, four times the previous k/2 block. Height zero encodes the hashes of individual systematic rows, eliminating their bottom authentication paths. The best candidate requiring a delayed join also uses h=0 and folds once, joining at message k/8 and rate 1/8. Its proof is slightly larger.

## Proof size (KiB) / verifier hashes

| N | Bolt + Merkle | Previous Canopy k/2 | Best half-rate Canopy: immediate | Best strictly delayed: one fold |
|---|---:|---:|---:|---:|
| 2^18 | 3892.06 / 96,869 | 3921.26 / 96,119 | 3199.23 / 52,583 | 3244.21 / 53,376 |
| 2^20 | 5122.91 / 135,937 | 4502.15 / 95,831 | 3309.63 / 55,791 | 3361.47 / 56,804 |
| 2^22 | 6446.50 / 178,290 | 4763.28 / 102,127 | 3411.82 / 58,917 | 3469.82 / 60,127 |
| 2^24 | 7839.57 / 222,580 | 4910.87 / 106,380 | 3516.61 / 62,133 | 3581.19 / 63,553 |
| 2^26 | 9250.87 / 267,289 | 5056.00 / 110,652 | 3633.06 / 65,622 | 3704.39 / 67,258 |
| 2^28 | 10654.85 / 312,211 | 5195.94 / 114,875 | 3740.34 / 68,940 | 3818.91 / 70,808 |
| 2^30 | 12074.96 / 357,540 | 5315.15 / 118,673 | 3850.30 / 72,350 | 3934.75 / 74,407 |

At N=2^30 the unconstrained half-rate winner saves 27.6% in proof bytes and 39.0% in verifier hashes versus the previous Canopy configuration; versus Merkle the savings are 68.1% and 79.8%. This gain comes from allowing the larger systematic block, not from delaying its join.

## Keeping the previous systematic block-length budget

Restricting the new search to blocks at most k/2 removes the larger-block advantage. At N=2^22 the winner is still h=2 with an immediate join, reproducing the previous proof size and hash count. At N=2^30, h=2 with one independent Canopy fold becomes marginally better:

| N=2^30, systematic block k/2 | Proof (KiB) | Verifier hashes |
|---|---:|---:|
| Previous immediate join | 5315.15 | 118,673 |
| One Canopy fold before joining | 5302.42 | 117,947 |

That is a 0.240% proof saving. The full bounded comparison is retained in `comparison.csv`.

## Measured prover time, N=2^22 and eta=1

The first three configurations were measured in rotating order in one run, with one warmup and five repetitions each, using eight Rayon threads. The strictly delayed candidate was measured in a separate alternating run with a fresh Merkle control. All 30 warmup/measured proofs verified. Times include commitment and opening, including FS; input generation, public setup, and verification are excluded. These are complete serialized public-H adapter proofs; Spark/Mulperm remain outside the scope.

| Configuration / run | Mean commit (s) | Mean open (s) | Median total (s) | Min–max total (s) |
|---|---:|---:|---:|---:|
| Bolt + Merkle | 0.211 | 0.409 | 0.621 | 0.608–0.631 |
| Previous Canopy k/2 | 0.273 | 0.413 | 0.682 | 0.673–0.698 |
| Half-rate Canopy, immediate | 0.434 | 0.538 | 0.965 | 0.964–0.992 |
| Half-rate Canopy, one fold / separate run | 0.444 | 0.485 | 0.928 | 0.916–0.947 |
| Merkle control / separate run | 0.209 | 0.404 | 0.613 | 0.598–0.628 |

The proof-optimal half-rate configuration takes 41.5% more median prover time than the previous k/2 Canopy configuration in the matched run. Phase means need not sum to the median total. The one-fold result is from a separate run, so small timing differences should not be interpreted as a reliable ranking.

All eight full Fiat–Shamir traces match the model, covering both immediate and genuinely delayed configurations at N=2^18 and N=2^22 plus split Merkle controls. Maximum mean serialized-size deviation from the sampled model is 0.051%. The model excludes small serializer framing overhead. Stage timings, raw repetitions, transcript byte counts, and process memory records are retained.

The legacy `canopy_layout=equal_half` tag in raw new timing rows identifies the retained parity setup; `proof_sizes.csv` gives the actual systematic length and join geometry. For the two new candidates this length is 2k, not k/2.

## Reproduce

```sh
cargo build --offline --release --example gf256_bolt_split
python3 scripts/measure_gf256_run.py --record-dir results/bolt_delayed_systematic_eta1/proof_sizes --threads 8 -- target/release/examples/gf256_bolt_split estimate-delayed --inputs 1 --samples 100 --reference results/gf256_split_half_suite --output results/bolt_delayed_systematic_eta1/proof_sizes
python3 scripts/measure_gf256_run.py --record-dir results/bolt_delayed_systematic_eta1/timing_n22 --threads 8 -- target/release/examples/gf256_bolt_split time --log 22 --inputs 1 --warmups 1 --repetitions 5 --width 4 --estimates results/bolt_delayed_systematic_eta1/proof_sizes --alternate-estimates results/gf256_split_half_suite/proof_sizes --paper-only --output results/bolt_delayed_systematic_eta1/timing_n22
python3 scripts/measure_gf256_run.py --record-dir results/bolt_delayed_systematic_eta1/strict_timing_n22 --threads 8 -- target/release/examples/gf256_bolt_split time --log 22 --inputs 1 --warmups 1 --repetitions 5 --width 4 --estimates results/bolt_delayed_systematic_eta1/proof_sizes/strictly_delayed --paper-only --output results/bolt_delayed_systematic_eta1/strict_timing_n22
cargo build --offline --release --features fs-accounting --example gf256_bolt_split
RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_split validate --input results/bolt_delayed_systematic_eta1/proof_sizes/proof_sizes.csv --output results/bolt_delayed_systematic_eta1/fiat_shamir_validation.csv
RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_split validate --input results/bolt_delayed_systematic_eta1/proof_sizes/strictly_delayed/proof_sizes.csv --output results/bolt_delayed_systematic_eta1/strict_fiat_shamir_validation.csv
python3 scripts/summarize_bolt_delayed_systematic.py
```

Use a build without `fs-accounting` for timings. The Merkle baseline is the smaller of the retained shared configuration and the split configuration including immediate joins. Earlier runs against the retained baseline are preserved in `before_merkle_boundary_audit/`. These results do not overwrite the manuscript data or extrapolate large-input timings.
