#!/usr/bin/env python3
"""Fix the systematic Canopy join after one fold; prepare/run-summary phases."""
import argparse
import csv
import hashlib
import json
import statistics
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / 'archive/results/bolt_delayed_systematic_eta1'
OUT = ROOT / 'archive/results/bolt_one_fold_systematic_eta1'
ONE = 'bolt_canopy_one_fold_systematic'
ZERO = 'bolt_canopy_immediate_systematic'
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--prepare', action='store_true')
args = parser.parse_args()


def read(path):
    with path.open() as f:
        return list(csv.DictReader(f))


def write(name, rows):
    path = OUT / name
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open('w', newline='') as f:
        w = csv.DictWriter(f, fieldnames=list(dict.fromkeys(k for r in rows for k in r)))
        w.writeheader()
        w.writerows(rows)


grid = read(SOURCE / 'proof_sizes/systematic_join_search.csv')
delayed = read(SOURCE / 'proof_sizes/strictly_delayed/proof_sizes.csv')
immediate = read(SOURCE / 'proof_sizes/proof_sizes.csv')
prior_comparison = read(SOURCE / 'proof_sizes_with_hashes.csv')
one_rows, zero_rows, proof, heights = [], [], [], []
for n in range(18, 31, 2):
    candidates = [dict(r) for r in grid if int(r['log_input']) == n
                  and r['input_oracles'] == '1' and r['canopy_folds_before_join'] == '1']
    best = min(candidates, key=lambda r: float(r['total_bytes']))
    one = dict(next(r for r in delayed if int(r['log_input']) == n and r['systematic_h']))
    zero = dict(next(r for r in immediate if int(r['log_input']) == n and r['systematic_h']))
    # Reuse already evaluated points only after checking this fixed-round selection.
    for key in ['systematic_h', 'systematic_target_log', 'total_bytes', 'verifier_hashes']:
        assert abs(float(best[key]) - float(one[key])) < 1e-6
    assert int(one['systematic_target_log']) == n - 2 - int(one['systematic_h']) - 3
    assert one['systematic_h'] == zero['systematic_h'] == '0'
    assert int(zero['systematic_target_log']) == n - 2
    for key in ['systematic_rs_length', 'parity_rs_length', 'parity_h', 'parity_target_log']:
        assert one[key] == zero[key]
    for c in candidates:
        c['selected'] = str(c is best).lower()
    heights.extend(candidates)
    one['scheme'], zero['scheme'] = ONE, ZERO
    split_merkle = next(r for r in immediate if int(r['log_input']) == n and not r['systematic_h'])
    one_rows.extend([split_merkle, one])
    zero_rows.extend([split_merkle, zero])
    merkle = dict(next(r for r in prior_comparison if int(r['log_input']) == n and r['configuration'] == 'merkle'))
    merkle.pop('configuration')
    proof.extend([merkle, zero, one])
write('one_fold/proof_sizes.csv', one_rows)
write('immediate_control/proof_sizes.csv', zero_rows)
write('height_search.csv', heights)
write('proof_sizes_with_hashes.csv', proof)
if args.prepare:
    print('Prepared fixed one-fold geometry and an immediate-join control with identical heights/blocks.')
    raise SystemExit(0)

raw = read(OUT / 'timing_n22/prover_timings.csv')
assert len(raw) == 15
assert all(r['proof_verified'] == 'true' and r['rayon_threads'] == '8'
           and r['log_input'] == '22' and r['input_oracles'] == '1' for r in raw)
stages = defaultdict(float)
for r in read(OUT / 'timing_n22/prover_timing_stages.csv'):
    stages[r['scheme'], r['phase'], r['part'], r['stage']] += float(r['seconds']) / 5
stage_rows = [dict(scheme=s, phase=p, part=t, stage=k, mean_seconds=v)
              for (s, p, t, k), v in sorted(stages.items())]
write('timing_stage_summary.csv', stage_rows)
summary = []
for scheme in sorted({r['scheme'] for r in raw}):
    rows = [r for r in raw if r['scheme'] == scheme]
    assert len(rows) == 5
    total = [float(r['total_prover_seconds']) for r in rows]
    row = dict(scheme=scheme, mean_commitment_seconds=statistics.mean(float(r['commitment_seconds']) for r in rows),
               mean_opening_seconds=statistics.mean(float(r['opening_seconds']) for r in rows),
               mean_total_seconds=statistics.mean(total), median_total_seconds=statistics.median(total),
               min_total_seconds=min(total), max_total_seconds=max(total),
               mean_serialized_bytes=statistics.mean(int(r['proof_bytes']) for r in rows))
    expected = next(r for r in proof if r['scheme'] == scheme and r['log_input'] == '22')
    row['relative_size_error'] = row['mean_serialized_bytes'] / float(expected['total_bytes']) - 1
    assert abs(row['relative_size_error']) < 0.002
    assert abs(row['mean_commitment_seconds'] + row['mean_opening_seconds'] - row['mean_total_seconds']) < 1e-9
    for phase in ['commitment', 'opening']:
        assert abs(sum(v for (s, p, _, _), v in stages.items() if s == scheme and p == phase)
                   - row[f'mean_{phase}_seconds']) < 1e-8
    summary.append(row)
write('timing_summary.csv', summary)
t = {r['scheme']: r for r in summary}
opening_saving = 100 * (1 - t[ONE]['mean_opening_seconds'] / t[ZERO]['mean_opening_seconds'])
total_saving = 100 * (1 - t[ONE]['median_total_seconds'] / t[ZERO]['median_total_seconds'])
lines = ['# Bolt + Canopy: join after exactly one systematic Canopy fold', '',
    'The join round is fixed at one. Only the height is selected within that fixed-round slice of the saved search; h_X=0 is selected at every size. These points were already evaluated, so the proof/hash data are reused after verifying their geometry against every one-fold height candidate. The immediate-join control uses the identical height and initial block lengths.', '',
    'Parameters: eta=1, Bolt-max w=4, GF(2^256), BLAKE3, and the retained conditional 128-bit soundness target. With k=N/4, systematic Canopy starts with message k and block 2k (rate 1/2). One three-variable WHIR fold reduces its message to k/8 and block to k; it then joins the systematic code-switch claim at rate 1/8. The parity setup stays at h_Y=3 and block k/2. Verifier hashes include authentication, hash-to-field, and Fiat–Shamir finalizations.', '',
    '## Proof size and verifier hashes', '',
    '| N | Immediate proof (KiB) | One-fold proof (KiB) | Increase | Immediate hashes | One-fold hashes |',
    '|---|---:|---:|---:|---:|---:|']
for n in range(18, 31, 2):
    a = next(r for r in proof if r['log_input'] == str(n) and r['scheme'] == ZERO)
    b = next(r for r in proof if r['log_input'] == str(n) and r['scheme'] == ONE)
    lines.append(f"| 2^{n} | {float(a['total_bytes'])/1024:.2f} | {float(b['total_bytes'])/1024:.2f} | {100*(float(b['total_bytes'])/float(a['total_bytes'])-1):.2f}% | {float(a['verifier_hashes']):,.0f} | {float(b['verifier_hashes']):,.0f} |")
lines += ['', '## Matched prover timings at N=2^22', '',
    'One warmup and five measured repetitions per configuration, in rotating order, eight Rayon threads. All 18 complete serialized proofs verified. Commitment and opening include their FS work; input generation, public setup, and verification are excluded. Spark/Mulperm remain outside the public-H adapter scope. Phase columns are means; total is a median.', '',
    '| Configuration | Commit (s) | Open (s) | Total (s) | Total min–max (s) |',
    '|---|---:|---:|---:|---:|']
baseline = next(s for s in t if 'merkle' in s)
for scheme, label in [(baseline, 'Bolt + Merkle'), (ZERO, 'Canopy: immediate join'), (ONE, 'Canopy: one-fold join')]:
    r = t[scheme]
    lines.append(f"| {label} | {r['mean_commitment_seconds']:.3f} | {r['mean_opening_seconds']:.3f} | {r['median_total_seconds']:.3f} | {r['min_total_seconds']:.3f}–{r['max_total_seconds']:.3f} |")
lines += ['', f'One fold reduces mean opening time by {opening_saving:.1f}% and median total time by {total_saving:.1f}% relative to the immediate-join control in this matched run. Both variants perform the same commitment algorithm, so measured commitment differences reflect run variation.', '',
    '## Systematic recursive opening components', '',
    '| Component | Immediate (s) | One fold (s) |', '|---|---:|---:|']
for stage in ['authenticated WHIR alignment and constraint combination', 'distinct-functional cross evaluations', 'shared recursive WHIR proof']:
    lines.append(f"| {stage} | {stages[ZERO, 'opening', 'systematic tail', stage]:.3f} | {stages[ONE, 'opening', 'systematic tail', stage]:.3f} |")
lines += ['', 'The smaller shared tail reduces cross-evaluation and recursive-proof work. The separate authenticated fold adds alignment work. Both effects are included in the opening total.', '',
    '## Reproduce', '', '```sh',
    'python3 archive/scripts/report_bolt_one_fold.py --prepare',
    'cargo build --offline --release --example gf256_bolt_split',
    'python3 scripts/measure_gf256_run.py --record-dir archive/results/bolt_one_fold_systematic_eta1/timing_n22 --threads 8 -- target/release/examples/gf256_bolt_split time --log 22 --inputs 1 --warmups 1 --repetitions 5 --width 4 --estimates archive/results/bolt_one_fold_systematic_eta1/one_fold --alternate-estimates archive/results/bolt_one_fold_systematic_eta1/immediate_control --paper-only --output archive/results/bolt_one_fold_systematic_eta1/timing_n22',
    'python3 archive/scripts/report_bolt_one_fold.py', '```', '',
    'The raw measurements and stage summaries are retained alongside the complete seven-size proof/hash grid. The prior study validated full FS traces for both join geometries at N=2^18 and N=2^22. No prover timing above N=2^22 is measured or extrapolated here.']
(OUT / 'README.md').write_text('\n'.join(lines) + '\n')
paths = ['src/bolt_split.rs', 'src/bolt_batching.rs', 'src/bolt_delayed.rs', 'archive/scripts/report_bolt_one_fold.py',
         'archive/results/bolt_delayed_systematic_eta1/proof_sizes/systematic_join_search.csv',
         'archive/results/bolt_delayed_systematic_eta1/proof_sizes/proof_sizes.csv',
         'archive/results/bolt_delayed_systematic_eta1/proof_sizes/strictly_delayed/proof_sizes.csv']
process = json.loads((OUT / 'timing_n22/process.json').read_text())
assert process['exit_code'] == 0
(OUT / 'manifest.json').write_text(json.dumps(dict(fixed_canopy_folds_before_join=1,
    height=0, input_oracles=1, width=4, log_inputs=list(range(18,31,2)), timing_log_input=22,
    proof_hash_source='Reused fixed-one-fold slice of the validated prior search',
    verified_measured_proofs=15, verified_warmups=3, process=process,
    source_sha256={p: hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in paths}), indent=2) + '\n')
print(f'Matched opening reduction: {opening_saving:.1f}%; total reduction: {total_saving:.1f}%')
print('\n'.join(lines[20:34]))
