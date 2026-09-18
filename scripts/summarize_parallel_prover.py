#!/usr/bin/env python3
"""Compare the saved pre/post parallel prover runs at N=2^22, eta=1."""
import collections
import csv
import json
import statistics
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'results/parallel_prover_n22_eta1'
SCHEMES = {
    'merkle_whir_batched_after_second': 'WHIR rate 1/2',
    'merkle_whir_batched_rate_quarter_after_second': 'WHIR rate 1/4',
    'canopy_schwartz_zippel': 'Canopy–WHIR half length',
    'bolt_merkle_shared': 'Bolt + Merkle',
    'bolt_canopy_split_quarter': 'Bolt + Canopy k/4',
}


def read(path):
    with path.open() as f:
        return list(csv.DictReader(f))


def write(path, rows):
    with path.open('w', newline='') as f:
        writer = csv.DictWriter(f, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)


def summary(version):
    result = {}
    for family in ['whir', 'bolt']:
        folder = OUT / version / family
        process = json.loads((folder / 'process.json').read_text())
        assert process['exit_code'] == 0 and process['threads'] == 8
        rows = read(folder / 'prover_timings.csv')
        for scheme in dict.fromkeys(r['scheme'] for r in rows):
            samples = [r for r in rows if r['scheme'] == scheme]
            if family == 'whir':
                assert len(samples) == 1 and int(samples[0]['repetitions']) == 10
                r = samples[0]
                result[scheme] = {k: float(r[k]) for k in [
                    'mean_commitment_seconds', 'mean_opening_seconds', 'mean_prover_seconds',
                    'median_prover_seconds', 'min_prover_seconds', 'max_prover_seconds']}
            else:
                assert len(samples) == 10
                assert all(r['proof_verified'] == 'true' and r['rayon_threads'] == '8' for r in samples)
                result[scheme] = {
                    'mean_commitment_seconds': statistics.mean(float(r['commitment_seconds']) for r in samples),
                    'mean_opening_seconds': statistics.mean(float(r['opening_seconds']) for r in samples),
                    'mean_prover_seconds': statistics.mean(float(r['total_prover_seconds']) for r in samples),
                    'median_prover_seconds': statistics.median(float(r['total_prover_seconds']) for r in samples),
                    'min_prover_seconds': min(float(r['total_prover_seconds']) for r in samples),
                    'max_prover_seconds': max(float(r['total_prover_seconds']) for r in samples),
                    'proof_bytes': int(samples[0]['proof_bytes']),
                }
                assert len({r['proof_bytes'] for r in samples}) == 1
    assert set(result) == set(SCHEMES)
    return result


def stages(version, scheme):
    # Sum repeated occurrences within each proof, then average the ten proofs.
    grouped = collections.defaultdict(lambda: collections.defaultdict(float))
    for r in read(OUT / version / 'bolt/prover_timing_stages.csv'):
        if r['scheme'] == scheme:
            grouped[r['phase'], r['part'], r['stage']][r['iteration']] += float(r['seconds'])
    result = {k: sum(v.values()) / 10 for k, v in grouped.items()}
    return result


before, after = summary('before'), summary('after')
rows = []
for scheme in SCHEMES:
    row = dict(scheme=scheme, log_input=22, input_oracles=1, threads=8, repetitions=10)
    for k, v in before[scheme].items():
        row['before_' + k] = v
        row['after_' + k] = after[scheme][k]
    if 'proof_bytes' in before[scheme]:
        assert before[scheme]['proof_bytes'] == after[scheme]['proof_bytes']
    row['mean_total_speedup'] = before[scheme]['mean_prover_seconds'] / after[scheme]['mean_prover_seconds']
    row['mean_total_reduction_percent'] = 100 * (1 - 1 / row['mean_total_speedup'])
    rows.append(row)
# Use a common schema for the component-composed WHIR and serialized Bolt rows.
fields = list(dict.fromkeys(k for row in rows for k in row))
write(OUT / 'comparison.csv', [{k: r.get(k, '') for k in fields} for r in rows])
stage_rows = []
for scheme in ['bolt_merkle_shared', 'bolt_canopy_split_quarter']:
    a, b = stages('before', scheme), stages('after', scheme)
    assert abs(sum(a.values()) - before[scheme]['mean_prover_seconds']) < 1e-6
    assert abs(sum(b.values()) - after[scheme]['mean_prover_seconds']) < 1e-6
    for phase, part, stage in sorted(a.keys() | b.keys()):
        key = phase, part, stage
        stage_rows.append(dict(scheme=scheme, phase=phase, part=part, stage=stage,
            before_seconds=a.get(key, 0), after_seconds=b.get(key, 0)))
write(OUT / 'bolt_stage_comparison.csv', stage_rows)

lines = [
    '# Parallel prover comparison', '',
    'Fresh local controls and optimized runs at N=2^22, eta=1, eight Rayon workers, '
    'GF(2^256), BLAKE3. Both use release builds, two warmups and ten measured repetitions. '
    'Runs execute sequentially, with no concurrent benchmark or build. '
    'All cells below are arithmetic mean seconds. Encoding is included in commitment; '
    'opening includes Fiat–Shamir. Input generation, public setup and verification are excluded.', '',
    '| Configuration | Commit before → after | Open before → after | Total before → after | Speedup |',
    '|---|---:|---:|---:|---:|',
]
for r in rows:
    def pair(phase):
        return f"{r['before_mean_' + phase + '_seconds']:.4f} → {r['after_mean_' + phase + '_seconds']:.4f}"
    lines.append(f"| {SCHEMES[r['scheme']]} | {pair('commitment')} | {pair('opening')} | {pair('prover')} | {r['mean_total_speedup']:.2f}× |")
lines += ['', 'WHIR retains the component-composed timing scope: measured and verified native components, '
          'not a fully serialized Exp3 proof. Bolt retains the serialized, verifier-checked public-H adapter '
          '(no Spark/Mulperm). These are sequential before/after samples, not confidence bounds. '
          'Historical local results and the running AWS campaign remain separately identified. '
          'The machine is an eight-core Apple M2 with 16 GiB RAM.', '',
          'An initial FFT version used 8192-element sweep thresholds and 1024-element tasks; '
          'it was slower locally and is retained in `initial_fine_grain/`. The selected version uses '
          '65536-element sweep thresholds and 4096-element tasks, and inlines the small butterfly loop.', '',
          'The parallel Bolt stages improve substantially, but the full local timing result is mixed: '
          'Bolt opening means fall about 19–20%, while commitment means rise. Merkle Bolt total improves '
          'about 6%; Canopy Bolt mean total is essentially unchanged (its median improves about 7%). '
          'The three WHIR mean totals rise about 9–18%, with median increases about 4–6%. '
          'This does not demonstrate a local WHIR speedup or establish how the changes will scale on AWS.', '',
          '## Changes', '',
          '- Compute g using contiguous row blocks, with a local accumulator per task and a parallel reduction.',
          '- Gather H-transpose outputs independently using cached reverse degree-8 adjacency with u32 indices. '
          'Duplicate edges are preserved. The new cache adds 32k bytes (32 MiB at this input and w=4), '
          'plus a transient k-byte degree array during untimed setup.',
          '- Use the existing thresholded parallel WHIR dot product in the Bolt adapter.',
          '- Evaluate the cross matrix in parallel, borrow dense covectors, and move them into the native proof; '
          'write transcript messages in the original row-major order.',
          '- Parallelize forward/transposed FFT butterflies, basis conversion dependency blocks, and coset accumulation. '
          'Small sweeps remain serial to limit scheduling overhead.', '',
          '## Validation', '',
          'All 83 library tests passed, including new serial-reference conversion tests, large-domain Horner/transpose '
          'checks, duplicate-edge tests, and byte-identical batched proof transcripts at one and four workers. '
          'Every measured/warmup Bolt proof verified, and serialized sizes match the fresh control exactly. '
          'Proof parameters, messages and hash accounting are unchanged.', '',
          '## Bolt stages', '',
          '| Method | Stage | Before (s) | After (s) |', '|---|---|---:|---:|']
for scheme in ['bolt_merkle_shared', 'bolt_canopy_split_quarter']:
    for stage in ['partial evaluation g', 'public H transpose covector', 'distinct-functional cross evaluations']:
        selected = [r for r in stage_rows if r['scheme'] == scheme and r['stage'] == stage]
        lines.append(f"| {SCHEMES[scheme]} | {stage} | {sum(r['before_seconds'] for r in selected):.5f} | {sum(r['after_seconds'] for r in selected):.5f} |")
if (OUT / 'control_repeat/bolt/process.json').exists():
    repeat = summary('control_repeat')
    control_rows = []
    lines += ['', '## Repeat control and variability', '',
              'The unchanged original binaries were rerun after the optimized measurements, with another '
              'two warmups and ten repetitions per configuration. This checks local drift; the primary '
              'table retains the original same-session control rather than selecting whichever baseline '
              'makes the optimization look best.', '',
              '| Configuration | Original mean | Optimized mean | Repeated original mean | Optimized median (range) |',
              '|---|---:|---:|---:|---:|']
    for scheme, label in SCHEMES.items():
        r = after[scheme]
        lines.append(f"| {label} | {before[scheme]['mean_prover_seconds']:.4f} | {r['mean_prover_seconds']:.4f} | {repeat[scheme]['mean_prover_seconds']:.4f} | {r['median_prover_seconds']:.4f} ({r['min_prover_seconds']:.4f}–{r['max_prover_seconds']:.4f}) |")
        control_rows.append(dict(scheme=scheme, original_mean_seconds=before[scheme]['mean_prover_seconds'],
                                 optimized_mean_seconds=r['mean_prover_seconds'],
                                 repeated_original_mean_seconds=repeat[scheme]['mean_prover_seconds']))
    write(OUT / 'repeat_control.csv', control_rows)
if (OUT / 'memory_during_control.json').exists():
    records = json.loads((OUT / 'memory_during_control.json').read_text())
    a, b = records[0], records[-1]
    seconds = b['time'] - a['time']
    swap_mib = (b['vm_stat']['Swapins'] - a['vm_stat']['Swapins']) * 16384 / 2**20
    compressed_gib = (b['vm_stat']['Compressions'] - a['vm_stat']['Compressions']) * 16384 / 2**30
    lines += ['', f'A {seconds:.1f}-second system-wide sample near the end of the repeat control recorded '
              f'{compressed_gib:.2f} GiB of page compression and {swap_mib:.1f} MiB of swap-ins. '
              'These OS counters are not attributable solely to the prover; they show this was not an isolated, '
              'swap-free timing environment. No thermal/performance warning was reported. '
              'Interpret small differences and outlier-sensitive means cautiously. Raw counters are in '
              '`memory_during_control.json`.']
lines += ['', 'Raw samples, stages, exact commands, binary hashes, wall time and process RSS are in '
          '`before/{whir,bolt}/` and `after/{whir,bolt}/`. Frozen binaries and relevant source snapshots are retained '
          'under each version. `comparison.csv` also includes total medians and ranges. '
          'Regenerate this report with `python3 scripts/summarize_parallel_prover.py`.', '']
(OUT / 'README.md').write_text('\n'.join(lines))
print('\n'.join(lines[:12]))
