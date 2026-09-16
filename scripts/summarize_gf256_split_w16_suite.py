#!/usr/bin/env python3
"""Summarize w=16 Bolt, comparing independent Canopy layers at k and k/2."""
import csv
import hashlib
import json
import statistics
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'results/gf256_split_w16_suite'
LOGS = [18, 20, 22, 24, 26, 28, 30]
ETAS = [1, 2, 4, 8]


def read(path):
    with path.open() as f:
        return list(csv.DictReader(f))


def write(name, rows):
    assert rows
    with (OUT / name).open('w', newline='') as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)


def key(r):
    return int(r['log_input']), int(r['input_oracles']), r['variant']


def variant(r):
    if r['scheme'] == 'bolt_canopy_split':
        return {'equal_full': 'canopy_k', 'equal_half': 'canopy_k_half'}[r['canopy_layout']]
    return r['scheme']


half = read(OUT / 'half/proof_sizes/proof_sizes.csv')
full = read(OUT / 'full/proof_sizes/proof_sizes.csv')
assert len(half) == len(full) == 56
shared = read(OUT / 'shared/proof_sizes/proof_sizes.csv')
fs = read(OUT / 'shared/fiat_shamir.csv')
shared_fs = {(r['log_input'], r['input_oracles'], r['scheme'], r['layout']): r for r in fs}
fields = list(half[0])
proofs = []
for label, source, divisor in [('canopy_k_half', half, 2), ('canopy_k', full, 1)]:
    for r in source:
        if r['scheme'] == 'bolt_merkle_split':
            if divisor == 1:
                match = next(x for x in half if (x['log_input'], x['input_oracles'], x['scheme']) == (r['log_input'], r['input_oracles'], r['scheme']))
                assert r == match
                continue
            v = r['scheme']
        else:
            v = label
            assert int(r['systematic_rs_length']) == (1 << int(r['log_input'])) // 16 // divisor
            assert r['systematic_rs_length'] == r['parity_rs_length']
        assert r['interleaving'] == '16'
        proofs.append({**r, 'variant': v})
for s in shared:
    if s['scheme'] != 'bolt_merkle_batched':
        continue
    merged = {**s, **shared_fs[s['log_input'], s['input_oracles'], s['scheme'], s['layout']]}
    r = {k: merged.get(k, '') for k in fields}
    r['scheme'] = 'bolt_merkle_shared'
    r['systematic_target_log'] = r['parity_target_log'] = s['aligned_log']
    r['systematic_rs_length'] = r['parity_rs_length'] = '0'
    r['variant'] = r['scheme']
    proofs.append(r)
proofs.sort(key=key)
assert len(proofs) == 112
by = {key(r): r for r in proofs}
assert len(by) == len(proofs)
for r in proofs:
    assert int(r['fs_proof_bytes']) == 0
    assert abs(float(r['verifier_hashes']) - sum(float(r[x]) for x in ['verifier_auth_hashes', 'verifier_hash_to_field', 'fs_finalizations'])) < 1e-6
write('all_proof_sizes.csv', proofs)

samples, stages, processes = [], [], []
for d in sorted(OUT.glob('timing_n*_eta*')):
    p = json.loads((d / 'process.json').read_text())
    assert p['exit_code'] == 0
    processes.append(p)
    for r in read(d / 'prover_timings.csv'):
        samples.append({**r, 'variant': variant(r)})
    for r in read(d / 'prover_timing_stages.csv'):
        stages.append({**r, 'variant': variant(r)})
assert len(processes) == 12 and len(samples) == 180
assert all(r['proof_verified'] == 'true' and r['rayon_threads'] == '8' and r['interleaving'] == '16' for r in samples)
groups = defaultdict(list)
for r in samples:
    groups[key(r)].append(r)
assert len(groups) == 36
summary = []
for (n, eta, v), rr in sorted(groups.items()):
    assert len(rr) == 5
    expected = by[n, eta, v]
    assert all(r[x] == expected[x] for r in rr for x in ['systematic_h', 'parity_h'])
    values = [float(r['total_prover_seconds']) for r in rr]
    for r in rr:
        assert abs(float(r['commitment_seconds']) + float(r['opening_seconds']) - float(r['total_prover_seconds'])) < 1e-8
    summary.append(dict(log_input=n, input_oracles=eta, interleaving=16, scheme=rr[0]['scheme'], variant=v,
        median_total_seconds=statistics.median(values), mean_total_seconds=statistics.mean(values),
        mean_commitment_seconds=statistics.mean(float(r['commitment_seconds']) for r in rr),
        mean_opening_seconds=statistics.mean(float(r['opening_seconds']) for r in rr),
        mean_verification_seconds=statistics.mean(float(r['verification_seconds']) for r in rr),
        min_total_seconds=min(values), max_total_seconds=max(values),
        mean_serialized_bytes=statistics.mean(int(r['proof_bytes']) for r in rr)))
write('timing_summary.csv', summary)
write('timing_samples.csv', samples)
write('timing_stages.csv', stages)
# Sum repeated stage labels within a proof before averaging across repetitions.
iterations = defaultdict(float)
for r in stages:
    iterations[*key(r), r['phase'], r['part'], r['stage'], int(r['iteration'])] += float(r['seconds'])
stage_groups = defaultdict(list)
for (*k, iteration), value in iterations.items():
    stage_groups[tuple(k)].append(value)
stage_summary = [dict(log_input=n, input_oracles=eta, interleaving=16, variant=v, phase=phase, part=part, stage=stage,
    mean_seconds=statistics.mean(values)) for (n, eta, v, phase, part, stage), values in sorted(stage_groups.items())]
write('timing_stage_summary.csv', stage_summary)
for r in summary:
    for phase, column in [('commitment', 'mean_commitment_seconds'), ('opening', 'mean_opening_seconds')]:
        total = sum(x['mean_seconds'] for x in stage_summary if key(x) == key(r) and x['phase'] == phase)
        assert abs(total - r[column]) < 1e-8
T = {key(r): r for r in summary}
comparison, selected = [], []
old_proofs = {(int(r['log_input']), int(r['input_oracles']), r['scheme']): r for r in read(ROOT / 'results/gf256_split_half_suite/paper_proof_sizes.csv')}
old_times = {(int(r['log_input']), int(r['input_oracles']), r['scheme']): r for r in read(ROOT / 'results/gf256_split_half_suite/paper_timing_summary.csv')}
for n in LOGS:
    for eta in ETAS:
        m = min((by[n, eta, 'bolt_merkle_shared'], by[n, eta, 'bolt_merkle_split']), key=lambda r: float(r['total_bytes']))
        f, h = by[n, eta, 'canopy_k'], by[n, eta, 'canopy_k_half']
        selected.extend([m, f, h])
        c = dict(log_input=n, input_oracles=eta, interleaving=16, baseline_scheme=m['scheme'])
        for label, row in [('baseline', m), ('canopy_k', f), ('canopy_k_half', h)]:
            c[label + '_proof_kib'] = float(row['total_bytes']) / 1024
            c[label + '_verifier_hashes'] = float(row['verifier_hashes'])
            c[label + '_fs_absorb_bytes'] = float(row['fs_absorb_bytes'])
            c[label + '_fs_squeeze_bytes'] = float(row['fs_squeeze_bytes'])
            c[label + '_systematic_h'] = row['systematic_h']
            c[label + '_parity_h'] = row['parity_h']
            c[label + '_median_seconds'] = T.get(key(row), {}).get('median_total_seconds', '')
            c[label + '_proof_reduction_percent'] = 100 * (1 - float(row['total_bytes']) / float(m['total_bytes']))
            c[label + '_hash_reduction_percent'] = 100 * (1 - float(row['verifier_hashes']) / float(m['verifier_hashes']))
            c[label + '_time_overhead_percent'] = 100 * (c[label + '_median_seconds'] / T[key(m)]['median_total_seconds'] - 1) if n <= 22 else ''
        c['half_vs_full_proof_change_percent'] = 100 * (float(h['total_bytes']) / float(f['total_bytes']) - 1)
        c['half_vs_full_time_change_percent'] = 100 * (T[key(h)]['median_total_seconds'] / T[key(f)]['median_total_seconds'] - 1) if n <= 22 else ''
        old = old_proofs[n, eta, 'bolt_canopy_split']
        c['w4_half_proof_kib'] = float(old['total_bytes']) / 1024
        c['w4_half_verifier_hashes'] = float(old['verifier_hashes'])
        c['w4_half_median_seconds'] = float(old_times[n, eta, 'bolt_canopy_split']['median_total_seconds']) if n <= 22 else ''
        comparison.append(c)
write('comparison.csv', comparison)
write('selected_proof_sizes.csv', sorted(selected, key=key))
write('selected_timing_summary.csv', summary)
checks = []
for name in ['half', 'full', 'shared']:
    rr = read(OUT / name / 'fiat_shamir_validation.csv')
    assert len(rr) == (6 if name == 'shared' else 12)
    assert all(r['trace_matches_model'] == 'true' and r['proof_verified'] == 'true' for r in rr)
    checks.extend(rr)
max_error = max(abs(r['mean_serialized_bytes'] / float(by[key(r)]['total_bytes']) - 1) for r in summary)
paths = ['src/bolt_split.rs', 'src/bolt_multi.rs', 'src/bolt_batching.rs', 'src/bolt_fiat_shamir.rs', 'src/bolt_experiment.rs', 'examples/gf256_bolt_split.rs', 'examples/gf256_bolt_suite.rs', 'scripts/summarize_gf256_split_w16_suite.py', 'scripts/run_gf256_split_w16_timings.py', 'scripts/validate_gf256_split_w16_suite.py', 'Cargo.lock']
manifest = dict(field='GF(2^256)', hash='BLAKE3', width=16, block_lengths=['k', 'k/2'], seed=0x424f4c5443414e31,
    security_bits=128, query_security_bits='132+log2(eta)', whir_local_security_bits=136, fold_limit=3,
    source_sha256={p: hashlib.sha256((ROOT / p).read_bytes()).hexdigest() for p in paths},
    timing_samples=180, verified_warmups=36, fs_validations=len(checks), max_relative_size_error=max_error,
    processes=processes)
(OUT / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
n22 = [r for r in comparison if r['log_input'] == 22]
proof_extra = [r['half_vs_full_proof_change_percent'] for r in n22]
time_saved = [-r['half_vs_full_time_change_percent'] for r in n22]
half_overhead = [r['canopy_k_half_time_overhead_percent'] for r in n22]
full_overhead = [r['canopy_k_time_overhead_percent'] for r in n22]
lines = ['# Bolt at w=16: independent Canopy layers with blocks k and k/2', '',
    f'At N=2^22, blocks k/2 are {min(time_saved):.1f}--{max(time_saved):.1f}% faster than blocks k, with {min(proof_extra):.1f}--{max(proof_extra):.1f}% larger proofs. Relative to the matching Merkle baseline, the prover overhead is {min(half_overhead):.1f}--{max(half_overhead):.1f}% for k/2 and {min(full_overhead):.1f}--{max(full_overhead):.1f}% for k. Full blocks give smaller proofs at every grid point. The shared WHIR tail is the smaller-proof Merkle baseline at every grid point.', '',
    'At N=2^22 the selected heights (h_X,h_Y) are (1,2) for blocks k and (2,3) for blocks k/2, for every eta. Wider rows reduce code-switch and Canopy RS message lengths at fixed N, but increase the number of field values communicated per queried row. Thus the w=16 prover is faster than the retained w=4 runs, while proofs are substantially larger.', '',
    'Each input has N field elements and k=N/16 systematic rows. The systematic and parity Canopy layers both use the indicated RS block length; their heights and WHIR join lengths are optimized independently. The field is GF(2^256), with BLAKE3 and the same conditional 128-bit target, gamma=0.013 distance assumption, Khatam systematic query radius, parity radius 1/4, and public-H scope as the previous experiment. Spark/Mulperm preprocessing and proofs are excluded.', '',
    'Proof sizes and verifier hashes are means over 100 deterministic query samples. The Merkle baseline is the smaller proof among the optimized shared and separate WHIR tails. Full-height constant Canopy layers are included and opened directly. Each nonconstant side batches eta Canopy SEP claims with eta corresponding code-switch claims, using native full cross matrices. Both sides use distinct challenges within one Fiat-Shamir transcript.', '',
    'At N=2^18, k=16,384 is below the systematic query budget, so every systematic row is opened. The size model and executable prover both account for this saturation.', '',
    'Timings use identical inputs, rotating the selected Merkle baseline, Canopy k/2, and Canopy k within each repetition. Each configuration uses one warmup and five measured repetitions, with eight Rayon threads. All 216 proofs are serialized and verified. Totals include commitment and opening, excluding input generation, public setup, and verification.', '',
    f'All 30 executed Fiat-Shamir verifier traces match the model. Maximum mean serialized-size deviation from the sampled estimate: {100 * max_error:.3f}%. Verifier hashes include authentication, hash-to-field, and Fiat-Shamir hash/XOF finalizations; transcript input/output bytes are separately available in the CSVs. Fiat-Shamir adds zero proof bytes.', '',
    '## N=2^22 comparison', '',
    '| eta | Method | Proof KiB | Verifier hashes | Commit s | Open s | Total s |',
    '|---:|---|---:|---:|---:|---:|---:|']
for c in comparison:
    if c['log_input'] != 22:
        continue
    eta = c['input_oracles']
    for label, v in [('Merkle', c['baseline_scheme']), ('Canopy k', 'canopy_k'), ('Canopy k/2', 'canopy_k_half')]:
        p, t = by[22, eta, v], T[22, eta, v]
        lines.append(f"| {eta} | {label} | {float(p['total_bytes']) / 1024:,.2f} | {float(p['verifier_hashes']):,.0f} | {t['mean_commitment_seconds']:.3f} | {t['mean_opening_seconds']:.3f} | {t['median_total_seconds']:.3f} |")
lines += ['', 'Commitment and opening are means; total time is the median, so the columns need not add exactly.', '',
    '## Full proof-size and verifier-hash grid', '',
    '| log2 N | eta | Merkle KiB / hashes | Canopy k KiB / hashes | Canopy k/2 KiB / hashes | Heights k; k/2 |',
    '|---:|---:|---:|---:|---:|---|']
for c in comparison:
    pairs = [f"{c[x + '_proof_kib']:,.2f} / {c[x + '_verifier_hashes']:,.0f}" for x in ['baseline', 'canopy_k', 'canopy_k_half']]
    heights = '; '.join(f"({c[x + '_systematic_h']},{c[x + '_parity_h']})" for x in ['canopy_k', 'canopy_k_half'])
    lines.append(f"| {c['log_input']} | {c['input_oracles']} | {' | '.join(pairs)} | {heights} |")
lines += ['', '## Full prover-time grid', '',
    '| log2 N | eta | Merkle s | Canopy k s | Canopy k/2 s |',
    '|---:|---:|---:|---:|---:|']
for c in comparison:
    if c['log_input'] <= 22:
        lines.append(f"| {c['log_input']} | {c['input_oracles']} | {c['baseline_median_seconds']:.3f} | {c['canopy_k_median_seconds']:.3f} | {c['canopy_k_half_median_seconds']:.3f} |")
lines += ['', '## Files and reproduction', '',
    '`comparison.csv` contains all 28 comparisons and retained w=4, k/2 results. Width comparisons use historical w=4 timings, not simultaneous reruns. `all_proof_sizes.csv` retains both Merkle architectures and both Canopy block lengths; `selected_proof_sizes.csv` exports the three headline methods. Timing samples, component breakdowns, process RSS, candidate searches, and source/binary identities are retained alongside the summaries.', '',
    'Build `gf256_bolt_split` and `gf256_bolt_suite` in release mode without `fs-accounting`. Run `gf256_bolt_split estimate --width 16 --block-divisor 1 --output results/gf256_split_w16_suite/full/proof_sizes` and again with divisor 2 and output `half/proof_sizes`. Run `gf256_bolt_suite estimate --width 16 --output results/gf256_split_w16_suite/shared/proof_sizes`, then `gf256_bolt_suite account --input .../shared/proof_sizes/proof_sizes.csv --output .../shared/fiat_shamir.csv`. Run `python3 scripts/run_gf256_split_w16_timings.py` sequentially, without concurrent compute-heavy work. Build separately with `--features fs-accounting` and run `python3 scripts/validate_gf256_split_w16_suite.py` to check both split variants and the Merkle-only rows of the shared estimates. Finally run `python3 scripts/summarize_gf256_split_w16_suite.py`.', '']
(OUT / 'README.md').write_text('\n'.join(lines))
print('\n'.join(lines[:28]))
