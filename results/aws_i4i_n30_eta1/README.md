# N=2^30, eta=1 AWS extension

Status: completed successfully on 2026-09-17. All five configurations have one
measured timing sample; both serialized Bolt proofs verified. Native release
build, field/protocol tests, proof/hash preparation, and completion validation
passed. The downloaded comparison CSV matches the SHA-256 in the remote
validation receipt. Runs used 32 Rayon workers pinned to CPUs `0-15,48-63`, no
warmup, and one measured pass. Canopy uses the direct digest-to-field bijection,
with no extra rehash.

| Configuration | Proof (KiB) | Verifier hashes | Commit (s) | Open (s) | Total prover (s) |
|---|---:|---:|---:|---:|---:|
| WHIR, rate 1/2 | 503.68 | 11,022.73 | 153.18 | 137.64 | 290.83 |
| WHIR, rate 1/4 | 338.30 | 7,312.48 | 253.55 | 242.13 | 495.68 |
| WHIR + Canopy, half block | 410.49 | 7,526.64 | 202.95 | 160.85 | 363.80 |
| Bolt + Merkle, w=4 | 12,074.96 | 357,539.79 | 34.73 | 87.12 | 121.85 |
| Bolt + Canopy, w=4, k/4 | 6,002.74 | 116,768.93 | 41.45 | 96.41 | 137.86 |

Proof sizes and verifier hash counts are averages over 100 query samples.
Verifier hashes count authentication and Fiat-Shamir hash/XOF finalizations;
digest conversion contributes zero hashes. Timings are measurements, not
extrapolations. WHIR timings compose measured components; Bolt uses the
serialized public-H adapter, excluding Spark/Mulperm. Input generation,
setup, and verification are excluded from reported prover time but included in
the process wall time and peak memory below.

The grouped WHIR job took 2,059.39 seconds (34.32 minutes) and peaked at
401.16 GiB RSS. The grouped Bolt job took 704.47 seconds (11.74 minutes) and
peaked at 153.28 GiB RSS. Together the timing jobs took 46.06 minutes;
build and proof/hash preparation are additional. Peak memory is measured per
grouped process and cannot be attributed to an individual configuration.

Bolt + Canopy reduces proof size by 50.29% and verifier hashes by 67.34%
relative to Bolt + Merkle, with 13.13% greater total prover time. Its selected
systematic and parity heights are 3 and 4, respectively.

Results: [comparison CSV](remote/comparison_iop_rounds.csv),
[validation receipt](remote/completion-validation.json), and raw measurements
under [timings](remote/timings/). The earlier 70-point dataset remains separate.

Remote host: `benchmark-host.invalid`.
Remote directory: `.`.
Output under that directory: `canopy-system/results/aws_i4i_n30_eta1`.
Persistent tmux session: `canopy-aws-n30`.
Driver/status/validation scripts: `run.sh`, `status.py`, `validate.py`.
Campaign status: `phase`, `campaign.log`, `campaign.exit`.
The five-minute progress heartbeat is paused after completion.

The build cache is a private copy under `tmp/canopy-aws-i4i-n30-native-20260917`,
linked as `target/aws-native` in the new source snapshot. This avoids filling the
small root filesystem or modifying older campaign caches. Raw output remains in
the persistent campaign directory. Original campaigns and the user's other
experiments are untouched; only the assigned 32 CPUs are used.

At launch the host had about 738 GiB available and no swap. The existing runner
checks available memory before timing, enforces a 640 GiB address-space cap, and
stops on low available RAM or swap use. The data-only forecast is in
`../aws_i4i_n30_eta1_prepared/`; it predicts about 48 minutes for the measured jobs
plus native build/proof preparation. Those predictions are not measurements.

Completion was checked using all five measured rows, successful process exits,
and `completion-validation.json`. Complete results are synchronized into
`remote/` here. Do not run the legacy direct-digest correction on this dataset: these
binaries already use the direct mapping and report zero conversion hashes.
