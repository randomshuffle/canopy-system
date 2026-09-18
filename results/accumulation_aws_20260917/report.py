#!/usr/bin/env python3
"""Summarize the three measured AWS runs; never substitute local timings."""
import csv
import json
from pathlib import Path
import statistics

ROOT = Path(__file__).resolve().parent
OUT = ROOT / 'measurements'
QUARTER_OUT = ROOT / 'rate_quarter/measurements'
with (OUT / 'timing_samples.csv').open() as f:
    raw = list(csv.DictReader(f))
with (OUT / 'timing_samples_n18.csv').open() as f:
    raw.extend(csv.DictReader(f))
with (QUARTER_OUT / 'timing_samples.csv').open() as f:
    raw.extend(csv.DictReader(f))
with (OUT / 'hash_validation.csv').open() as f:
    traces = {(r['log_n'], r['scheme'], r['input_seed']): r for r in csv.DictReader(f)}
with (OUT / 'hash_validation_n18.csv').open() as f:
    traces.update({(r['log_n'], r['scheme'], r['input_seed']): r for r in csv.DictReader(f)})
with (QUARTER_OUT / 'hash_validation.csv').open() as f:
    traces.update({(r['log_n'], r['scheme'], r['input_seed']): r for r in csv.DictReader(f)})
host = json.loads((OUT / 'host.json').read_text())
complete = json.loads((OUT / 'complete.json').read_text())
assert complete['status'] == 'complete'
assert json.loads((OUT / 'complete_n18.json').read_text())['status'] == 'complete'
assert json.loads((QUARTER_OUT / 'complete.json').read_text())['status'] == 'complete'
quarter_host = json.loads((QUARTER_OUT / 'host.json').read_text())
for key in ('hostname', 'threads', 'cpu_affinity', 'rustc', 'cargo', 'rustflags'):
    assert host[key] == quarter_host[key], key
assert all(quarter_host['source_sha256'][p] == h for p, h in host['source_sha256'].items()
           if p != 'examples/accumulation_one_fold.rs')
assert len(raw) == len(traces) == 36
BASE = 'merkle_same_code_batch'
QUARTER = 'merkle_same_code_batch_rate_quarter'
CANOPY = 'canopy_one_fold_three_claims'
NAMES = {BASE: 'Plain Merkle, rate 1/2', QUARTER: 'Plain Merkle, rate 1/4', CANOPY: 'Canopy, one fold'}
summary = []
for log in (18, 20, 22, 24):
    for scheme in (BASE, QUARTER, CANOPY):
        group = [r for r in raw if int(r['log_n']) == log and r['scheme'] == scheme]
        assert len(group) == 3 and {int(r['input_seed']) for r in group} == {92, 93, 94}
        row = dict(log_n=log, scheme=scheme, measured_runs=3)
        for r in group:
            t = traces[r['log_n'], r['scheme'], r['input_seed']]
            assert r['verified'] == t['verified'] == 'true'
            assert r['proof_bytes'] == t['proof_bytes']
            assert r['actual_auth_hashes'] == t['actual_auth_hashes']
            assert int(t['fs_finalizations']) == log + (9 if scheme == CANOPY else 4)
            assert int(r['queries_0']) == (147 if scheme == QUARTER else 317)
        for metric in [k for k in raw[0] if k.endswith('_seconds')] + ['proof_bytes', 'actual_auth_hashes']:
            vals = [float(r[metric]) for r in group]
            row['mean_' + metric] = statistics.mean(vals)
            row['stddev_' + metric] = statistics.stdev(vals)
            row['min_' + metric] = min(vals)
            row['max_' + metric] = max(vals)
        row['fs_finalizations'] = log + (9 if scheme == CANOPY else 4)
        row['mean_verifier_hashes'] = row['mean_actual_auth_hashes'] + row['fs_finalizations']
        summary.append(row)
with (ROOT / 'summary.csv').open('w', newline='') as f:
    writer = csv.DictWriter(f, fieldnames=list(summary[0]))
    writer.writeheader()
    writer.writerows(summary)
lines = [
    '# AWS one-fold accumulation experiment', '',
    'Host: `benchmark-host.invalid`.',
    f'Intel Xeon Platinum 8375C, 96 logical CPUs, {host["threads"]} Rayon workers pinned to CPUs 0–15 and 48–63 (16 physical cores on one NUMA node). Rust 1.92.0, release build with `-C target-cpu=native`; GF(2^256) uses PCLMULQDQ.', '',
    'Every entry is an arithmetic mean over three measured runs, using input seeds 92, 93, and 94. One discarded warmup per scheme and size uses seed 91. The original rate-1/2 Merkle and Canopy runs alternate order; the added rate-1/4 baseline was measured separately on the same idle host with the same worker affinity and library sources. Every proof and its resulting output SEP were verified. Input generation and verification are outside the prover timers.', '',
    'N denotes the RS0 message length. RS0 has rate 1/2 and codeword length 2N; RS1 has message length N/8, rate 1/8, and codeword length N. Width 8, Canopy height 4, 317 RS0 queries and 96 RS1 queries. GF(2^256), BLAKE3-256, 136-bit local query budget, no proof of work.', '',
    'The additional plain Merkle baseline keeps message length N and uses rate 1/4: codeword length 4N, N/2 Merkle leaves of width 8, and 147 queries per source at the same 136-bit local budget. It batches two claims into one using the same code, without a WHIR fold. Only the benchmark example gained configuration flags; all library/vendor sources match the original campaign.', '',
    '| N | Scheme | Step prover (s) | Incoming commitment (s) | Step + incoming (s) | Verifier hashes | Proof (KiB) |',
    '|---|---|---:|---:|---:|---:|---:|',
]
for r in summary:
    name = NAMES[r['scheme']]
    lines.append(f'| 2^{r["log_n"]} | {name} | {r["mean_step_seconds"]:.4f} | {r["mean_incoming_commit_seconds"]:.4f} | {r["mean_with_incoming_commit_seconds"]:.4f} | {r["mean_verifier_hashes"]:,.1f} | {r["mean_proof_bytes"]/1024:.2f} |')
lines += ['', '## Scope', '',
    'Step time starts with cached input commitments/codewords. It includes public-functional materialization, the old-claim fold and its encoding (Canopy), bottom openings, batched sumcheck, codeword combination, output Merkle commitment, OOD evaluation, source openings, and output SEP construction. Incoming commitment includes encoding and, for Canopy, the layer OOD evaluation. The new incoming Canopy claim is carried unchanged. The final accumulator decider and any recursive SNARK are excluded.', '',
    'Verifier hashes are observed authentication hash calls plus separately instrumented Fiat–Shamir XOF finalizations, averaged over the same three inputs. One XOF finalization counts as one hash; these are not BLAKE3 compression counts. Instrumented proof byte counts and authentication hashes agree exactly with the timing runs. Output trees are not opened in this step. Direct digest-to-field conversion uses zero hashes.', '',
    'Proof bytes are the actual native transcript payload: narg_string plus hints. They include repeated public roots/scalars used by this prototype to bind the instance, as well as output commitment messages and authentication data. They exclude outer transport framing, cached witness data, incoming commitment construction, and final-decider proofs. These are executable prototype sizes, not a minimized wire format.', '',
    'The cost prototype verifies dense covectors and does not implement a recursive SNARK or a production accumulator wrapper. Its old and incoming Canopy positions reuse the same valid instance for timing, as in the local experiment. The measured accumulation step still reduces only the old position. Three distinct seeds produce three different instances/query samples across runs.', '',
    '## Variation across the three runs', '',
    '| N | Scheme | Step sample SD (s) | Step range (s) | Proof range (bytes) |',
    '|---|---|---:|---:|---:|',
]
for r in summary:
    name = NAMES[r['scheme']]
    lines.append(f'| 2^{r["log_n"]} | {name} | {r["stddev_step_seconds"]:.4f} | {r["min_step_seconds"]:.4f}–{r["max_step_seconds"]:.4f} | {r["min_proof_bytes"]:.0f}–{r["max_proof_bytes"]:.0f} |')
process = json.loads((OUT / 'timing.json').read_text())
quarter_process = json.loads((QUARTER_OUT / 'timing.json').read_text())
lines += ['', '## Reproduction and evidence', '',
    f'Original main timing process: {process["wall_seconds"]:.2f}s wall time, {process["peak_rss_gib"]:.2f} GiB peak RSS. Rate-1/4 timing process: {quarter_process["wall_seconds"]:.2f}s wall time, {quarter_process["peak_rss_gib"]:.2f} GiB peak RSS.', '',
    '- `run_experiment.py` and `run_n18.py`: exact remote build, validation, and measurement commands. N=2^18 was added after the main campaign started, using the same binaries.',
    '- `source.tar.gz` and `source_sha256.json`: source snapshot used on EC2.',
    '- `measurements/host.json`: hardware, toolchain, affinity, and source hashes.',
    '- `measurements/timing_samples.csv` and `timing_samples_n18.csv`: all 24 measured samples and stage times.',
    '- `measurements/hash_validation.csv` and `hash_validation_n18.csv`: separately instrumented runs on the same inputs.',
    '- `measurements/smoke.csv`: small-instance tampering checks.',
    '- `measurements/complete.json` and `complete_n18.json`: binary hashes and completion checks.',
    '- `rate_quarter/run.py`, `accumulation_one_fold.rs`, and `source_sha256.json`: extension runner and exact source change.',
    '- `rate_quarter/measurements/`: 12 additional measured samples, matching hash instrumentation, host metadata, validation and build logs, and binary hashes.',
    '- `summary.csv`: means, sample standard deviations, and ranges for all metrics.', '',
    'Regenerate this report with `python3 results/accumulation_aws_20260917/report.py`.', '',
]
(ROOT / 'README.md').write_text('\n'.join(lines))
print('\n'.join(lines[:26]))
