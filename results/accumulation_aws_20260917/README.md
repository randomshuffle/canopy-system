# AWS one-fold accumulation experiment

Host: `benchmark-host.invalid`.
Intel Xeon Platinum 8375C, 96 logical CPUs, 32 Rayon workers pinned to CPUs 0–15 and 48–63 (16 physical cores on one NUMA node). Rust 1.92.0, release build with `-C target-cpu=native`; GF(2^256) uses PCLMULQDQ.

Every entry is an arithmetic mean over three measured runs, using input seeds 92, 93, and 94. One discarded warmup per scheme and size uses seed 91. The original rate-1/2 Merkle and Canopy runs alternate order; the added rate-1/4 baseline was measured separately on the same idle host with the same worker affinity and library sources. Every proof and its resulting output SEP were verified. Input generation and verification are outside the prover timers.

N denotes the RS0 message length. RS0 has rate 1/2 and codeword length 2N; RS1 has message length N/8, rate 1/8, and codeword length N. Width 8, Canopy height 4, 317 RS0 queries and 96 RS1 queries. GF(2^256), BLAKE3-256, 136-bit local query budget, no proof of work.

The additional plain Merkle baseline keeps message length N and uses rate 1/4: codeword length 4N, N/2 Merkle leaves of width 8, and 147 queries per source at the same 136-bit local budget. It batches two claims into one using the same code, without a WHIR fold. Only the benchmark example gained configuration flags; all library/vendor sources match the original campaign.

| N | Scheme | Step prover (s) | Incoming commitment (s) | Step + incoming (s) | Verifier hashes | Proof (KiB) |
|---|---|---:|---:|---:|---:|---:|
| 2^18 | Plain Merkle, rate 1/2 | 0.0279 | 0.0143 | 0.0422 | 5,560.0 | 293.52 |
| 2^18 | Plain Merkle, rate 1/4 | 0.0367 | 0.0289 | 0.0656 | 3,260.0 | 157.83 |
| 2^18 | Canopy, one fold | 0.0291 | 0.0233 | 0.0525 | 3,414.3 | 231.20 |
| 2^20 | Plain Merkle, rate 1/2 | 0.1662 | 0.0989 | 0.2651 | 6,842.0 | 333.58 |
| 2^20 | Plain Merkle, rate 1/4 | 0.1763 | 0.1733 | 0.3497 | 3,836.0 | 175.89 |
| 2^20 | Canopy, one fold | 0.1317 | 0.1205 | 0.2522 | 3,970.7 | 248.80 |
| 2^22 | Plain Merkle, rate 1/2 | 0.5925 | 0.4122 | 1.0046 | 8,135.3 | 374.06 |
| 2^22 | Plain Merkle, rate 1/4 | 0.6589 | 0.7762 | 1.4351 | 4,400.7 | 193.60 |
| 2^22 | Canopy, one fold | 0.4564 | 0.5654 | 1.0218 | 4,545.0 | 267.02 |
| 2^24 | Plain Merkle, rate 1/2 | 2.3584 | 1.8730 | 4.2314 | 9,412.0 | 414.02 |
| 2^24 | Plain Merkle, rate 1/4 | 2.7033 | 3.1969 | 5.9002 | 5,004.7 | 212.54 |
| 2^24 | Canopy, one fold | 2.0834 | 2.5119 | 4.5953 | 5,130.0 | 285.43 |

## Scope

Step time starts with cached input commitments/codewords. It includes public-functional materialization, the old-claim fold and its encoding (Canopy), bottom openings, batched sumcheck, codeword combination, output Merkle commitment, OOD evaluation, source openings, and output SEP construction. Incoming commitment includes encoding and, for Canopy, the layer OOD evaluation. The new incoming Canopy claim is carried unchanged. The final accumulator decider and any recursive SNARK are excluded.

Verifier hashes are observed authentication hash calls plus separately instrumented Fiat–Shamir XOF finalizations, averaged over the same three inputs. One XOF finalization counts as one hash; these are not BLAKE3 compression counts. Instrumented proof byte counts and authentication hashes agree exactly with the timing runs. Output trees are not opened in this step. Direct digest-to-field conversion uses zero hashes.

Proof bytes are the actual native transcript payload: narg_string plus hints. They include repeated public roots/scalars used by this prototype to bind the instance, as well as output commitment messages and authentication data. They exclude outer transport framing, cached witness data, incoming commitment construction, and final-decider proofs. These are executable prototype sizes, not a minimized wire format.

The cost prototype verifies dense covectors and does not implement a recursive SNARK or a production accumulator wrapper. Its old and incoming Canopy positions reuse the same valid instance for timing, as in the local experiment. The measured accumulation step still reduces only the old position. Three distinct seeds produce three different instances/query samples across runs.

## Variation across the three runs

| N | Scheme | Step sample SD (s) | Step range (s) | Proof range (bytes) |
|---|---|---:|---:|---:|
| 2^18 | Plain Merkle, rate 1/2 | 0.0019 | 0.0257–0.0290 | 298832–302416 |
| 2^18 | Plain Merkle, rate 1/4 | 0.0024 | 0.0344–0.0392 | 161552–161680 |
| 2^18 | Canopy, one fold | 0.0029 | 0.0273–0.0325 | 235384–237848 |
| 2^20 | Plain Merkle, rate 1/2 | 0.0066 | 0.1611–0.1736 | 340880–342928 |
| 2^20 | Plain Merkle, rate 1/4 | 0.0018 | 0.1746–0.1783 | 179024–181200 |
| 2^20 | Canopy, one fold | 0.0052 | 0.1263–0.1367 | 254680–254872 |
| 2^22 | Plain Merkle, rate 1/2 | 0.0032 | 0.5894–0.5957 | 382544–383312 |
| 2^22 | Plain Merkle, rate 1/4 | 0.0033 | 0.6552–0.6617 | 197840–198736 |
| 2^22 | Canopy, one fold | 0.0210 | 0.4322–0.4694 | 271384–274552 |
| 2^24 | Plain Merkle, rate 1/2 | 0.0106 | 2.3481–2.3692 | 422608–425232 |
| 2^24 | Plain Merkle, rate 1/4 | 0.0123 | 2.6915–2.7160 | 216144–219216 |
| 2^24 | Canopy, one fold | 0.0073 | 2.0756–2.0900 | 290488–294424 |

## Reproduction and evidence

Original main timing process: 75.70s wall time, 9.58 GiB peak RSS. Rate-1/4 timing process: 59.19s wall time, 13.28 GiB peak RSS.

- `run_experiment.py` and `run_n18.py`: exact remote build, validation, and measurement commands. N=2^18 was added after the main campaign started, using the same binaries.
- `source.tar.gz` and `source_sha256.json`: source snapshot used on EC2.
- `measurements/host.json`: hardware, toolchain, affinity, and source hashes.
- `measurements/timing_samples.csv` and `timing_samples_n18.csv`: all 24 measured samples and stage times.
- `measurements/hash_validation.csv` and `hash_validation_n18.csv`: separately instrumented runs on the same inputs.
- `measurements/smoke.csv`: small-instance tampering checks.
- `measurements/complete.json` and `complete_n18.json`: binary hashes and completion checks.
- `rate_quarter/run.py`, `accumulation_one_fold.rs`, and `source_sha256.json`: extension runner and exact source change.
- `rate_quarter/measurements/`: 12 additional measured samples, matching hash instrumentation, host metadata, validation and build logs, and binary hashes.
- `summary.csv`: means, sample standard deviations, and ranges for all metrics.

Regenerate this report with `python3 results/accumulation_aws_20260917/report.py`.
