# Historical campaign discussion

This is the original campaign discussion. Current dataset and reproduction instructions are in [the final report](../../results/aws_i4i_optimized/README.md). Relative paths in the historical text describe the original layout.

# Parallel AWS campaign — one pass, 32 workers

## Digest conversion correction

The current paper uses
[`comparison_iop_rounds_direct_digest.csv`](comparison_iop_rounds_direct_digest.csv).
It removes the obsolete extra Canopy rehash from verifier hash totals while
preserving every proof-size and timing value in `comparison_iop_rounds.csv`.
Canopy timings still include that rehash; they have not been rerun. The original
campaign and sequential-round exports remain unchanged. Provenance and prior
hash totals are retained in the corrected CSV and its JSON sidecar.

Regenerate with:

```sh
python3 scripts/report_direct_digest_conversion.py \
  --input results/aws_i4i_optimized/comparison_iop_rounds.csv \
  --output results/aws_i4i_optimized/comparison_iop_rounds_direct_digest.csv
```


Status: complete, all 70 points with successful campaign exit. Results are backed
up locally and validated in `completion-validation.json`. Reported commitment
times use the sequential-IOP convention below. The redundant WHIR follow-up was stopped; its partial diagnostics remain
separate in `../aws_i4i_fair/`. This completed campaign is the final dataset.

## Current reported results

Use `comparison_iop_rounds.csv` for the comparison. As clarified by the user,
the eta commitments occur in separate IOP rounds. For all five schemes,
C(N, eta) = eta * C(N, 1), using the measured commitment at the same N and scheme.
Opening measurements and proof/hash estimates stay unchanged; reported totals
are recomputed. Raw measurements remain in `remote/comparison.csv` and individual
job files. The reporting script retains raw values and records the anchor and
input-file hash. Thus concurrent commitment scheduling in the raw WHIR harness
does not enter the headline comparison.

## Timing comparison caveat discovered during the run

The multi-polynomial WHIR commitment comparison needs correction before paper
use. `run_hybrid_iteration` measures one initial Merkle-WHIR commitment and the
timing harness multiplies it by eta for both WHIR baselines. Canopy instead
executes all eta initial encodings concurrently with
`encode_many_interleaved_with_whir` and commits them through the staged parallel
Canopy batch. This gives only Canopy an implementation optimization that is
equally applicable to the baselines. The scheduling mismatch affects eta > 1
baseline commitment and total-time comparisons; it does not establish a Canopy
protocol speedup. The baseline opening fronts are also scaled from single-input
measurements, so the harness remains a component-composed timing model.

At N=2^26, eta=4, the baseline initial RS time is 4 times a single-input sample:
30.1573 seconds. Canopy's actual concurrent initial RS time is 18.0688 seconds
for the same four base encodings. Baseline Merkle work is similarly scaled to
2.2621 seconds, while Canopy's concurrent bottom forests take 0.6692 seconds.
The additional Canopy H2F, layer RS and layer Merkle work takes 7.6103 seconds.
Thus the reported 32.4194 versus 26.3482 second commitments are not a fair
comparison of equally optimized implementations. The previously reported 5.7%
Canopy total-time advantage at this point is withdrawn pending correction.

Raw results are preserved. No corrected baseline timings have been measured
yet. Both WHIR baselines need the same concurrent commitment scheduling and
remeasurement before replacing these numbers. Proof-size/hash estimates and
the separately implemented Bolt timing harness are unaffected by this issue.

Restart of the five-configuration campaign on the same i4i.24xlarge, with parallel
g, cached gather H-transpose, dot products, cross evaluations, large additive-FFT
sweeps, and Bolt Canopy hash-to-field. Release build with `-C target-cpu=native`.

The user requested zero warmups and one measured pass per configuration point.
The grid remains 70 points: N=2^22,2^24,2^26 with eta=1,2,4,8, and N=2^28 with
eta=1,2. One timing sample does not characterize run-to-run variability.

The campaign, including builds and tests, uses 32 Rayon workers pinned to CPUs
`0-15,48-63`: both hardware threads of physical cores 0–15 on NUMA node 0.
The user's other experiment can use `16-47,64-95`, the remaining 32 physical
cores / 64 hardware threads. CPU affinity does not isolate memory bandwidth or
all other shared hardware resources.

All five configurations retain the existing field, hashes, security assumptions,
block lengths, heights and batching. WHIR timings remain component-composed;
Bolt timings cover serialized and verified public-H adapter proofs.

Hardware-independent proof/hash estimates are copied from the original campaign,
with parameters and checksums recorded. All prover timings restart from scratch
with the new binary; original results remain in `../aws_i4i/persistent/`.

Remote directory: `.`.
Output: `canopy-system/results/aws_i4i_optimized` under that directory.
Session: `canopy-aws-optimized`. Driver: `run.sh`. Status: `status.py`.
Parent log/exit marker: `campaign.log` and `campaign.exit`.
Local synchronized outputs: `remote/`.

The existing five-minute progress monitor follows this campaign. The instance
remains running after completion. The old campaign stopped at 55 completed
points; its active WHIR N=2^26, eta=8 job is marked interrupted and excluded.
