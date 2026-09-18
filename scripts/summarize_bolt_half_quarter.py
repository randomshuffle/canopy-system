#!/usr/bin/env python3
"""Report fixed shared-Merkle Bolt against separate k/2 and k/4 Canopy layers."""
import csv
import hashlib
import json
import statistics
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'results/bolt_half_quarter_shared_baseline_eta1'
LOGS = list(range(18, 31, 2))
MERKLE = 'bolt_merkle_shared'
HALF = 'bolt_canopy_split'
QUARTER = 'bolt_canopy_split_quarter'
LABELS = {MERKLE: 'Bolt + Merkle (shared tail)', HALF: 'Bolt + Canopy, k/2', QUARTER: 'Bolt + Canopy, k/4'}


def read(path):
    with path.open() as f:
        return list(csv.DictReader(f))


def write(name, rows):
    with (OUT / name).open('w', newline='') as f:
        writer = csv.DictWriter(f, fieldnames=list(dict.fromkeys(k for r in rows for k in r)))
        writer.writeheader()
        writer.writerows(rows)


proofs = []
fs = {r['log_input']: r for r in read(OUT / 'shared_baseline/fiat_shamir.csv') if r['scheme'] == 'bolt_merkle_batched'}
for r in read(OUT / 'shared_baseline/merkle_only.csv'):
    r.update({k: fs[r['log_input']][k] for k in ['fs_finalizations', 'fs_absorb_bytes', 'fs_squeeze_bytes', 'fs_proof_bytes', 'verifier_hashes']})
    r['scheme'] = MERKLE
    r['systematic_rs_length'] = r['parity_rs_length'] = '0'
    assert int(r['aligned_log']) == int(r['log_input']) - 5
    proofs.append(r)
for directory, divisor in [('half', 2), ('quarter', 4)]:
    for r in read(OUT / directory / 'proof_sizes.csv'):
        if not r['systematic_h']:
            continue
        assert r['delayed_systematic'] == 'false'
        k = 2 ** (int(r['log_input']) - 2)
        assert int(r['systematic_rs_length']) == int(r['parity_rs_length']) == k // divisor
        proofs.append(r)
assert len(proofs) == 21
old = {(r['log_input'], r['scheme']): r for r in read(ROOT / 'results/gf256_split_half_suite/paper_proof_sizes.csv') if r['input_oracles'] == '1'}
for r in proofs:
    assert r['input_oracles'] == '1' and r['interleaving'] == '4' and r['samples'] == '100'
    assert r['systematic_queries'] == '20977' and r['parity_queries'] == '319'
    assert abs(float(r['verifier_hashes']) - sum(float(r[k]) for k in ['verifier_auth_hashes', 'verifier_hash_to_field', 'fs_finalizations'])) < 1e-6
    assert float(r['fs_proof_bytes']) == 0
    if r['scheme'] in [MERKLE, HALF]:
        for field in ['total_bytes', 'verifier_hashes']:
            assert abs(float(r[field]) - float(old[r['log_input'], r['scheme']][field])) < 1e-6
proofs.sort(key=lambda r: (int(r['log_input']), r['scheme']))
write('proof_sizes_with_hashes.csv', proofs)
by = {(int(r['log_input']), r['scheme']): r for r in proofs}

samples = read(OUT / 'timing_n22/prover_timings.csv')
assert len(samples) == 30
assert {r['scheme'] for r in samples} == set(LABELS)
assert all(r['proof_verified'] == 'true' and r['rayon_threads'] == '8' and r['log_input'] == '22' and r['input_oracles'] == '1' for r in samples)
stage_iterations = defaultdict(float)
for r in read(OUT / 'timing_n22/prover_timing_stages.csv'):
    stage_iterations[r['scheme'], r['phase'], r['part'], r['stage'], r['iteration']] += float(r['seconds'])
stage_groups = defaultdict(list)
for (*key, iteration), value in stage_iterations.items():
    stage_groups[tuple(key)].append(value)
stages = [dict(scheme=s, phase=p, part=t, stage=k, mean_seconds=statistics.mean(v)) for (s, p, t, k), v in sorted(stage_groups.items())]
write('timing_stage_summary.csv', stages)
timing = []
for scheme in LABELS:
    rr = [r for r in samples if r['scheme'] == scheme]
    assert len(rr) == 10
    expected = by[22, scheme]
    assert all(r[k] == expected[k] for r in rr for k in ['systematic_h', 'parity_h'])
    total = [float(r['total_prover_seconds']) for r in rr]
    r = dict(scheme=scheme, log_input=22, input_oracles=1, repetitions=10,
             mean_commitment_seconds=statistics.mean(float(r['commitment_seconds']) for r in rr),
             mean_opening_seconds=statistics.mean(float(r['opening_seconds']) for r in rr),
             mean_total_seconds=statistics.mean(total), median_total_seconds=statistics.median(total),
             min_total_seconds=min(total), max_total_seconds=max(total),
             mean_serialized_bytes=statistics.mean(int(r['proof_bytes']) for r in rr))
    r['relative_size_error'] = r['mean_serialized_bytes'] / float(expected['total_bytes']) - 1
    assert abs(r['relative_size_error']) < 0.002
    assert abs(r['mean_commitment_seconds'] + r['mean_opening_seconds'] - r['mean_total_seconds']) < 1e-9
    for phase in ['commitment', 'opening']:
        assert abs(sum(x['mean_seconds'] for x in stages if x['scheme'] == scheme and x['phase'] == phase) - r[f'mean_{phase}_seconds']) < 1e-8
    timing.append(r)
write('timing_summary.csv', timing)
t = {r['scheme']: r for r in timing}
comparison = []
for n in LOGS:
    m = by[n, MERKLE]
    for scheme in [HALF, QUARTER]:
        r = by[n, scheme]
        comparison.append(dict(log_input=n, input_oracles=1, scheme=scheme,
            proof_saving_percent=100 * (1 - float(r['total_bytes']) / float(m['total_bytes'])),
            verifier_hash_saving_percent=100 * (1 - float(r['verifier_hashes']) / float(m['verifier_hashes'])),
            prover_time_overhead_percent=100 * (t[scheme]['median_total_seconds'] / t[MERKLE]['median_total_seconds'] - 1) if n == 22 else '',
            commitment_overhead_percent=100 * (t[scheme]['mean_commitment_seconds'] / t[MERKLE]['mean_commitment_seconds'] - 1) if n == 22 else '',
            opening_overhead_percent=100 * (t[scheme]['mean_opening_seconds'] / t[MERKLE]['mean_opening_seconds'] - 1) if n == 22 else ''))
write('comparison.csv', comparison)
checks = sum((read(OUT / f'fiat_shamir_validation_{part}.csv') for part in ['baseline', 'half', 'quarter']), [])
assert len(checks) == 6 and all(r['trace_matches_model'] == r['proof_verified'] == 'true' for r in checks)
max_error = max(abs(r['relative_size_error']) for r in timing)
processes = {d: json.loads((OUT / d / 'process.json').read_text()) for d in ['shared_baseline', 'half', 'quarter', 'timing_n22']}
assert all(r['exit_code'] == 0 for r in processes.values())
assert '--shared-merkle-baseline' in processes['timing_n22']['command']

lines = ['# Bolt: fixed shared Merkle baseline, Canopy blocks k/2 and k/4', '',
    'This rerun fixes the Merkle baseline to the paper shared-tail architecture, with join message k/8. It does not select the slower separate-tail Merkle variant by proof size. The two Canopy configurations use separate systematic/parity WHIR batches and equal RS block lengths k/2 or k/4 on the two sides. Each side optimizes its height for proof size independently, including full-height constant layers. There are no independent Canopy folds before batching: each nonconstant Canopy message joins its own code-switch claim at its original length.', '',
    'Parameters are w=4, k=N/4, eta=1, GF(2^256), BLAKE3, and the retained 128-bit conditional soundness target (Bolt distance gamma=0.013 remains an explicit assumption). The systematic and parity query counts remain 20,977 and 319. Proof sizes average 100 deterministic query samples; verifier hashes include authentication, hash-to-field, and Fiat–Shamir finalizations. Fiat–Shamir adds no proof bytes. The fresh Merkle and k/2 proof/hash grids reproduce the paper values exactly.', '',
    '## Proof size (KiB) / verifier hashes', '',
    '| N | Shared Merkle baseline | Canopy k/2 | Canopy k/4 |', '|---|---:|---:|---:|']
for n in LOGS:
    rows = [by[n, s] for s in LABELS]
    lines.append(f'| 2^{n} | ' + ' | '.join(f"{float(r['total_bytes'])/1024:.2f} / {float(r['verifier_hashes']):,.0f}" for r in rows) + ' |')
lines += ['', '## Fresh matched prover times at N=2^22', '',
    'All three configurations use the same release binary and identical inputs, run in rotating order with two warmups and ten measured repetitions per configuration, eight Rayon threads. No other benchmark or build ran during timing. All 36 warmup/measured serialized proofs verified. Times include commitment and opening, including FS; input generation, public setup, and verification are excluded. Spark/Mulperm remain outside the public-H adapter scope. Phase times are means and total times are medians.', '',
    '| Configuration | Commit (s) | Open (s) | Total (s) | Total min–max (s) | Total overhead vs Merkle |', '|---|---:|---:|---:|---:|---:|']
for scheme, label in LABELS.items():
    r = t[scheme]
    overhead = 100 * (r['median_total_seconds'] / t[MERKLE]['median_total_seconds'] - 1)
    lines.append(f"| {label} | {r['mean_commitment_seconds']:.3f} | {r['mean_opening_seconds']:.3f} | {r['median_total_seconds']:.3f} | {r['min_total_seconds']:.3f}–{r['max_total_seconds']:.3f} | {overhead:.1f}% |")
q_saving = 100 * (1 - t[QUARTER]['median_total_seconds'] / t[HALF]['median_total_seconds'])
q_proof = 100 * (float(by[22, QUARTER]['total_bytes']) / float(by[22, HALF]['total_bytes']) - 1)
lines += ['', f'At N=2^22, reducing both Canopy blocks from k/2 to k/4 saves {q_saving:.1f}% in median total prover time and increases proof size by {q_proof:.1f}%. All headline timing comparisons come from this one matched run, rather than mixing Table 1 timings with later measurements.', '',
    '## Selected heights', '', '| N | k/2: h_X, h_Y | k/4: h_X, h_Y |', '|---|---:|---:|']
for n in LOGS:
    a, b = by[n, HALF], by[n, QUARTER]
    lines.append(f"| 2^{n} | {a['systematic_h']}, {a['parity_h']} | {b['systematic_h']}, {b['parity_h']} |")
lines += ['', 'At N=2^18 the k/4 systematic layer is constant and opened directly (h_X=16). At N=2^22 the systematic messages are k/4 for k/2 blocks and k/8 for k/4 blocks. The parity message is k/32 in both cases at that size.', '',
    f"All six full FS transcript validations match the model, covering all three headline configurations at N=2^18 and N=2^22. Maximum mean serialized-size discrepancy is {100*max_error:.3f}%; the size model excludes small serializer framing. Timing-process peak RSS was {processes['timing_n22']['peak_rss_gib']:.3f} GiB, including setup and verification with schemes run sequentially.", '',
    '## Reproduce', '', '```sh',
    'cargo build --offline --release --example gf256_bolt_suite --example gf256_bolt_split',
    'RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_suite estimate --inputs 1 --samples 100 --width 4 --output results/bolt_half_quarter_shared_baseline_eta1/shared_baseline',
    'target/release/examples/gf256_bolt_suite account --input results/bolt_half_quarter_shared_baseline_eta1/shared_baseline/proof_sizes.csv --output results/bolt_half_quarter_shared_baseline_eta1/shared_baseline/fiat_shamir.csv',
    'RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_split estimate --inputs 1 --samples 100 --width 4 --block-divisor 2 --output results/bolt_half_quarter_shared_baseline_eta1/half',
    'RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_split estimate --inputs 1 --samples 100 --width 4 --block-divisor 4 --output results/bolt_half_quarter_shared_baseline_eta1/quarter',
    'python3 scripts/measure_gf256_run.py --record-dir results/bolt_half_quarter_shared_baseline_eta1/timing_n22 --threads 8 -- target/release/examples/gf256_bolt_split time --log 22 --inputs 1 --warmups 2 --repetitions 10 --width 4 --estimates results/bolt_half_quarter_shared_baseline_eta1/half --alternate-estimates results/bolt_half_quarter_shared_baseline_eta1/quarter --shared-estimates results/bolt_half_quarter_shared_baseline_eta1/shared_baseline --paper-only --shared-merkle-baseline --output results/bolt_half_quarter_shared_baseline_eta1/timing_n22',
    'cargo build --offline --release --features fs-accounting --example gf256_bolt_suite --example gf256_bolt_split',
    'RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_suite validate --input results/bolt_half_quarter_shared_baseline_eta1/shared_baseline/merkle_only.csv --output results/bolt_half_quarter_shared_baseline_eta1/fiat_shamir_validation_baseline.csv',
    'RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_split validate --input results/bolt_half_quarter_shared_baseline_eta1/half/canopy_only.csv --output results/bolt_half_quarter_shared_baseline_eta1/fiat_shamir_validation_half.csv',
    'RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_split validate --input results/bolt_half_quarter_shared_baseline_eta1/quarter/canopy_only.csv --output results/bolt_half_quarter_shared_baseline_eta1/fiat_shamir_validation_quarter.csv',
    'python3 scripts/summarize_bolt_half_quarter.py', '```', '',
    '`merkle_only.csv` filters the shared estimate rows to `bolt_merkle_batched`; each `canopy_only.csv` filters its split estimates to rows with a systematic Canopy height. These validation inputs are retained. Build without `fs-accounting` before taking timings. No new larger-input times are measured or extrapolated, and the manuscript tables remain the saved paper snapshot.']
(OUT / 'README.md').write_text('\n'.join(lines) + '\n')
paths = ['src/bolt_split.rs', 'src/bolt_batching.rs', 'src/bolt_multi.rs', 'src/bolt_experiment.rs', 'src/bolt_fiat_shamir.rs',
         'examples/gf256_bolt_split.rs', 'examples/gf256_bolt_suite.rs', 'scripts/summarize_bolt_half_quarter.py',
         'results/gf256_split_half_suite/paper_proof_sizes.csv', 'Cargo.lock']
(OUT / 'manifest.json').write_text(json.dumps(dict(field='GF(2^256)', hash='BLAKE3', width=4, input_oracles=[1],
    log_inputs=LOGS, timing_log_input=22, canopy_block_divisors=[2,4], delayed_systematic=False,
    merkle_baseline='fixed shared-tail paper configuration; join k/8', soundness_bits=128,
    distance_assumption_gamma=0.013, samples=100, warmups=2, repetitions=10,
    verified_measured_proofs=30, verified_warmups=6, fs_trace_validations=6,
    max_mean_relative_serialized_size_error=max_error, processes=processes,
    source_sha256={p: hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in paths}), indent=2) + '\n')
print('\n'.join(lines[6:28]))
print(f'Quarter vs half: time saving {q_saving:.1f}%, proof increase {q_proof:.1f}% at N=2^22')
