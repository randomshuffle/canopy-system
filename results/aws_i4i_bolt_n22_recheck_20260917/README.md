# Bolt N=2^22 AWS timing recheck

Completed 2026-09-17 on the existing i4i.24xlarge, using 32 Rayon workers pinned to CPUs `0-15,48-63`.
The validated native release binaries from the N=2^30 campaign were reused; GF(2^256), BLAKE3, w=4, shared-tail Merkle baseline, and separate Canopy layers of block length k/4. The Canopy heights are (3,3). Direct digest-to-field conversion is used, without the removed rehash.

Each of eta=1,2,4,8 used one discarded warmup per configuration and four measured passes. The measured order was Canopy/Merkle, Merkle/Canopy, Canopy/Merkle, Merkle/Canopy. All 32 measured serialized public-H adapter proofs verified. Spark/Mulperm is excluded.

## Reported means

Commitment follows the existing sequential-IOP convention: C(N,eta) = eta times the mean C(N,1) for the same scheme. Opening is the mean measured for the requested eta. Total is their sum. Thus eta>1 reported totals are derived from measured components, not whole-run stopwatch samples. All numbers below are seconds.

| eta | Merkle commit | Merkle open | Merkle total | Canopy commit | Canopy open | Canopy total | Canopy overhead |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 0.093139 | 0.316019 | 0.409159 | 0.110851 | 0.344073 | 0.454924 | 11.19% |
| 2 | 0.186279 | 0.576838 | 0.763117 | 0.221702 | 0.609872 | 0.831574 | 8.97% |
| 4 | 0.372558 | 1.141259 | 1.513817 | 0.443404 | 1.220283 | 1.663686 | 9.90% |
| 8 | 0.745116 | 2.270773 | 3.015889 | 0.886808 | 2.418122 | 3.304929 | 9.58% |

## Raw measured totals

The raw totals below retain each workload's actual measured commitment, without eta=1 rescaling. Ranges show the four individual measured totals, not confidence intervals. Canopy was slower in all 16 paired measurements.

| eta | Merkle mean (s) | Merkle range (s) | Canopy mean (s) | Canopy range (s) |
|---:|---:|---:|---:|---:|
| 1 | 0.409159 | 0.386065–0.423519 | 0.454924 | 0.453817–0.456238 |
| 2 | 0.753370 | 0.717215–0.788910 | 0.824729 | 0.790313–0.854648 |
| 4 | 1.541400 | 1.532386–1.547003 | 1.673275 | 1.659301–1.685965 |
| 8 | 3.063779 | 3.047330–3.087673 | 3.320140 | 3.309278–3.329549 |

The earlier single-pass apparent speedup did not reproduce. These repeated measurements have a different run-order/warmup design and use direct digest conversion; historical measurements are preserved rather than overwritten.

The four timing jobs took 99.62 seconds including warmups, setup, and verification; proof/hash parameter preparation is additional. Peak RSS was 3.73 GiB.

Files: [reported summary](remote/results/summary_iop_rounds.csv), [raw timing samples](remote/results/raw_samples.csv), [stage means](remote/results/stage_summary.csv), [raw stages](remote/results/raw_stages.csv), [validation](remote/results/completion-validation.json), and per-job logs/build provenance in `remote/results/`. The earlier campaigns and manuscript have not been changed.


## Reproduce into a fresh directory

On Linux x86_64 with the configured CPU allocation, Rust 1.92.0 and cached or
fetchable dependencies:

```sh
python3 scripts/reproduce_bolt_recheck.py --log 22 --output results/reproduce_bolt_n22
```

The runner builds and validates native binaries, uses one discarded warmup and
four alternating measured passes at eta=1,2,4,8, verifies all 32 measured
serialized proofs, and writes raw samples, stage means, and the sequential-IOP
summary. It requires a fresh output directory and never replaces these stored
measurements. Add `--offline` after dependencies are cached, or `--plan` to
inspect the commands locally. `--skip-build` requires a native build receipt
matching current sources. See the [campaign guide](../../experiments/aws-r5/README.md)
for resources and provenance. Current-source reruns use BCGM m=10 parameters.
Original launchers are in [the archive](../../archive/launchers/README.md);
scripts under `remote/` are historical source snapshots.
