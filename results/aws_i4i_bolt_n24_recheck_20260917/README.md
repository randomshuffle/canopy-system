# Bolt N=2^24 AWS timing recheck

Completed on the existing AWS i4i.24xlarge with 32 Rayon workers pinned to CPUs `0-15,48-63`.
The same validated native release binaries as the N=2^22 recheck were used: GF(2^256), BLAKE3, w=4, the faster shared-tail Merkle baseline, and separate systematic/parity Canopy layers with block lengths k/4. Direct digest conversion uses no extra rehash.

Each eta in {1,2,4,8} used one discarded warmup per configuration and four measured passes. Measured order alternated Canopy/Merkle, Merkle/Canopy, Canopy/Merkle, Merkle/Canopy. All 32 measured serialized public-H adapter proofs verified; Spark/Mulperm is excluded.

## Reported means

Commitment follows the paper convention: C(N,eta) = eta times mean C(N,1) for the same scheme. Opening is the mean measured for that eta, and total is their sum. The eta>1 totals are derived from measured components. All values below are seconds.

| eta | Merkle commit | Merkle open | Merkle total | Canopy commit | Canopy open | Canopy total | Canopy overhead |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 0.435430 | 1.188547 | 1.623976 | 0.496650 | 1.274097 | 1.770747 | 9.04% |
| 2 | 0.870859 | 2.332356 | 3.203215 | 0.993300 | 2.527744 | 3.521044 | 9.92% |
| 4 | 1.741719 | 4.733777 | 6.475496 | 1.986600 | 5.050800 | 7.037400 | 8.68% |
| 8 | 3.483438 | 8.922127 | 12.405565 | 3.973200 | 9.567117 | 13.540318 | 9.15% |

## Raw measured totals

These use each workload's actual measured commitments without eta=1 rescaling. Ranges span the four measured passes and are not confidence intervals.

| eta | Merkle mean (s) | Merkle range (s) | Canopy mean (s) | Canopy range (s) |
|---:|---:|---:|---:|---:|
| 1 | 1.623976 | 1.613395 to 1.631702 | 1.770747 | 1.759538 to 1.787370 |
| 2 | 3.190972 | 3.162190 to 3.210457 | 3.529531 | 3.518062 to 3.555107 |
| 4 | 6.476283 | 6.465767 to 6.496438 | 7.063668 | 7.050130 to 7.072947 |
| 8 | 12.330705 | 12.282301 to 12.362577 | 13.508906 | 13.450763 to 13.574929 |

Canopy was slower in 16 of the 16 paired measured passes.

Timing jobs took 373.36 seconds including warmups, setup, and verification; proof/hash parameter preparation is additional. Peak RSS was 13.64 GiB.

Files: [summary](remote/results/summary_iop_rounds.csv), [raw samples](remote/results/raw_samples.csv), [stage means](remote/results/stage_summary.csv), [raw stages](remote/results/raw_stages.csv), and [validation receipt](remote/results/completion-validation.json). Configuration, host/build provenance, proof/hash estimates, and run logs are retained under `remote/`. Historical data and manuscript figures remain separate.
