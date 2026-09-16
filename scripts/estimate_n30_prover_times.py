#!/usr/bin/env python3
"""Data-only, stage-wise N=2^30 extrapolation of the five paper configurations."""
import csv
import hashlib
import json
import statistics
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'results/n30_prover_time_estimates'
OUT.mkdir(parents=True, exist_ok=True)
WHIR = ROOT / 'results/gf256_batched_suite/whir'
BOLT = ROOT / 'results/gf256_split_half_suite'
SCHEMES = {
    'merkle_whir_batched_after_second': 'WHIR batched, rate 1/2',
    'merkle_whir_batched_rate_quarter_after_second': 'WHIR batched, rate 1/4',
    'canopy_schwartz_zippel': 'Canopy + WHIR, Exp. 3',
    'bolt_merkle_shared': 'Bolt + Merkle, w=4',
    'bolt_canopy_split': 'Bolt + Canopy, w=4, k/2',
}


def read(p):
    with p.open() as f:
        return list(csv.DictReader(f))


def write(name, rows):
    with (OUT / name).open('w', newline='') as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0]))
        w.writeheader()
        w.writerows(rows)


def key(r):
    return int(r['log_input']), int(r.get('input_polynomials', r.get('input_oracles'))), r['scheme']


summaries = {key(r): r for r in read(WHIR / 'timing_summary.csv') + read(BOLT / 'timing_summary.csv')}
repetitions = defaultdict(list)
raw_paths = [WHIR / f'timing_n{n}/prover_timing_samples.csv' for n in [18, 20, 22]]
for n, path in zip([18, 20, 22], raw_paths):
    for r in read(path):
        repetitions[n, int(r['input_oracles']), r['scheme']].append({
            phase: float(r[phase + '_seconds']) for phase in ['commitment', 'opening']})
raw_paths.append(BOLT / 'timing_samples.csv')
for r in read(raw_paths[-1]):
    repetitions[key(r)].append({phase: float(r[phase + '_seconds']) for phase in ['commitment', 'opening']})
for k, rr in repetitions.items():
    assert len(rr) == 5
    assert abs(statistics.median(sum(r.values()) for r in rr) - float(summaries[k]['median_total_seconds'])) < 1e-8

commit_stages = {
    'initial_rs_encoding', 'initial_merkle_tree', 'outer_rs_encoding',
    'canopy_bottom_forest', 'canopy_hash_to_field', 'canopy_rs_encoding',
    'canopy_codeword_merkle_tree', 'canopy_commitment_other',
}
stages = []
for r in read(WHIR / 'timing_stages.csv'):
    if r['stage'] == 'separately_measured_component_adjustment':
        continue  # Reconcile positive stage weights to the measured phase total below.
    stages.append(dict(log_input=int(r['log_input']), input_oracles=int(r['input_oracles']),
        scheme=r['scheme'], phase='commitment' if r['stage'] in commit_stages else 'opening',
        part='WHIR', stage=r['stage'], measured_seconds=float(r['mean_seconds'])))
for r in read(BOLT / 'timing_stage_summary.csv'):
    stages.append({**{k: v for k, v in r.items() if k != 'mean_seconds'}, 'log_input': int(r['log_input']), 'input_oracles': int(r['input_oracles']),
        'measured_seconds': float(r['mean_seconds'])})
phase_groups = defaultdict(list)
for r in stages:
    phase_groups[*key(r), r['phase']].append(r)
for (n, eta, scheme, phase), rr in phase_groups.items():
    measured = float(summaries[n, eta, scheme]['mean_' + phase + '_seconds'])
    total = sum(r['measured_seconds'] for r in rr)
    assert total > 0
    for r in rr:
        r['phase_reconciliation_factor'] = measured / total
        r['calibrated_seconds'] = r['measured_seconds'] * measured / total
    assert abs(sum(r['calibrated_seconds'] for r in rr) - measured) < 1e-8


def classification(r):
    """Return a scaling model, coefficient-dimension shift, and terminal log."""
    s, p, scheme = r['stage'], r['part'], r['scheme']
    if p == 'WHIR':
        if s in ['initial_rs_encoding', 'outer_rs_encoding']:
            return 'fft', 3, 0  # Eight interleaved columns, each with N/8 coefficients.
        if s in ['canopy_rs_encoding', 'second_oracle_rs_and_merkle_commitment']:
            return 'fft', 6, 0  # Message N/8, eight columns: N/64 coefficients.
        if s in ['first_transition_residual_after_subtracting_tail', 'outer_initial_whir_stage_after_ood']:
            return 'fft', 3, 0  # Mixed arithmetic/transposed-FFT work, FFT scaling proxy.
        if s in ['batched_recursive_whir', 'recursive_tail_after_ood']:
            return 'recursive_fft', 3, 10
        if s in ['canopy_bottom_opening', 'canopy_commitment_other']:
            return 'fixed', 0, 0
        return 'linear', 0, 0  # Hashing, evaluation and cross-evaluation passes.
    if s == 'parity RS encoding Y=RS(U)':
        return 'fft', 5, 0  # Each syndrome column contains N/32 coefficients.
    if s == 'Canopy RS encode and WHIR oracle commitment':
        return 'fft', (7 if p == 'X' else 10), 0
    if s == 'common WHIR RS encode and oracle commitment':
        shift = (4 if p == 'x' else 7) if scheme == 'bolt_canopy_split' else (5 if p == 'x' else 8)
        return 'fft', shift, 0
    if s == 'parity query covector and transpose RS FFT':
        return 'fft', 5, 0
    if s == 'authenticated WHIR alignment and constraint combination':
        return 'fft', {'source': 5, 'systematic tail': 4, 'parity tail': 7}[p], 0
    if s == 'shared recursive WHIR proof':
        return 'recursive_fft', {'batched WHIR': 5, 'systematic tail': 4, 'parity tail': 7}[p], 3
    if s == 'collect and serialize row authentication paths':
        return ('logarithmic', 2 if p == 'X' else 4, 0) if scheme == 'bolt_merkle_shared' else ('fixed', 0, 0)
    if s in ['project queried rows and sample batching challenges', 'sample row queries',
             'Canopy ancestor claims', 'copy and serialize queried row values', 'Merkle root transcript']:
        return 'fixed', 0, 0
    if s in ['Canopy WHIR configuration', 'WHIR configuration']:
        return 'logarithmic', 0, 0
    return 'linear', 0, 0


def work(n, model, shift=0, terminal=0):
    if model == 'fixed':
        return 1.0
    if model == 'logarithmic':
        return float(n - shift)
    if model == 'linear':
        return float(2 ** n)
    if model == 'fft':
        return float(2 ** n * (n - shift))
    assert model == 'recursive_fft'
    # Proxy for FFT work across the fixed three-variable WHIR schedule:
    # each round halves total codeword size; each column loses three variables.
    d, weight, total = n - shift, 1.0, 0.0
    while True:
        total += weight * max(d - min(3, d), 1)
        if d <= terminal:
            break
        d -= min(3, d)
        weight /= 2
    return 2 ** n * total


for r in stages:
    r['scaling_model'], r['dimension_shift'], r['terminal_log'] = classification(r)


def project(anchor, target, mode='stage'):
    result, details = defaultdict(lambda: defaultdict(float)), []
    for r in stages:
        if r['log_input'] != anchor:
            continue
        model, shift, terminal = r['scaling_model'], r['dimension_shift'], r['terminal_log']
        # Sensitivity only: remove the log factor from FFT/recursive costs.
        if mode == 'linear' and model in ['fft', 'recursive_fft']:
            model, shift, terminal = 'linear', 0, 0
        factor = work(target, model, shift, terminal) / work(anchor, model, shift, terminal)
        estimate = r['calibrated_seconds'] * factor
        k = r['input_oracles'], r['scheme']
        result[k][r['phase']] += estimate
        details.append({**r, 'target_log_input': target, 'scaling_factor': factor, 'estimated_seconds': estimate})
    return result, details


def projected_repetitions(anchor, eta, scheme, phases):
    # Raw phases are retained for WHIR even though its substage repetitions
    # were stored only as aggregates. Use the same phase-factor method for
    # both suites to project all five measured repetitions consistently.
    mean = summaries[anchor, eta, scheme]
    factors = {p: phases[p] / float(mean['mean_' + p + '_seconds']) for p in phases}
    return [sum(r[p] * factors[p] for p in phases) for r in repetitions[anchor, eta, scheme]]


primary, details = project(22, 30)
linear, _ = project(22, 30, 'linear')
anchor20, _ = project(20, 30)
backtest, _ = project(20, 22)
validation = []
for (eta, scheme), phases in backtest.items():
    actual = float(summaries[22, eta, scheme]['mean_total_seconds'])
    predicted = sum(phases.values())
    median_actual = float(summaries[22, eta, scheme]['median_total_seconds'])
    median_predicted = statistics.median(projected_repetitions(20, eta, scheme, phases))
    validation.append(dict(input_oracles=eta, scheme=scheme, anchor_log_input=20,
        target_log_input=22, predicted_total_seconds=predicted, measured_total_seconds=actual,
        predicted_median_seconds=median_predicted, measured_median_seconds=median_actual,
        mean_relative_error_percent=100 * (predicted / actual - 1),
        relative_error_percent=100 * (median_predicted / median_actual - 1)))
write('n20_to_n22_backtest.csv', validation)
rows = []
for scheme in SCHEMES:
    for eta in [1, 2, 4, 8]:
        p, q, a = primary[eta, scheme], linear[eta, scheme], anchor20[eta, scheme]
        projected = projected_repetitions(22, eta, scheme, p)
        median = statistics.median(projected)
        row = dict(scheme=scheme, label=SCHEMES[scheme], input_oracles=eta,
            target_log_input=30, anchor_log_input=22, anchor_mean_seconds=float(summaries[22, eta, scheme]['mean_total_seconds']),
            estimated_commitment_seconds=p['commitment'], estimated_opening_seconds=p['opening'],
            estimated_mean_total_seconds=sum(p.values()), estimated_total_seconds=median, estimated_total_minutes=median / 60,
            min_projected_repetition_seconds=min(projected), max_projected_repetition_seconds=max(projected),
            linear_sensitivity_seconds=statistics.median(projected_repetitions(22, eta, scheme, q)),
            n20_anchor_sensitivity_seconds=statistics.median(projected_repetitions(20, eta, scheme, a)),
            input_data_gib=32 * eta, measured_at_n30=False)
        assert row['estimated_total_seconds'] > 0
        rows.append(row)
assert len(rows) == 20
write('estimates.csv', rows)
write('stage_estimates.csv', details)
# The manuscript extends each measured curve from 2^22 to the remaining grid
# points using the identical calibrated work model and saved repetitions.
grid_rows = []
for target in [24, 26, 28, 30]:
    projected_phases, _ = project(22, target)
    for scheme in SCHEMES:
        for eta in [1, 2, 4, 8]:
            phases = projected_phases[eta, scheme]
            values = projected_repetitions(22, eta, scheme, phases)
            grid_rows.append(dict(scheme=scheme, input_oracles=eta,
                log_input=target, anchor_log_input=22, timing_kind='estimated',
                mean_commitment_seconds=phases['commitment'],
                mean_opening_seconds=phases['opening'],
                mean_total_seconds=sum(phases.values()),
                median_total_seconds=statistics.median(values)))
assert len(grid_rows) == 80
for r in rows:
    end = next(g for g in grid_rows if g['log_input'] == 30 and
               g['scheme'] == r['scheme'] and g['input_oracles'] == r['input_oracles'])
    assert abs(end['median_total_seconds'] - r['estimated_total_seconds']) < 1e-8
write('prover_time_estimates.csv', grid_rows)
by = {(r['scheme'], r['input_oracles']): r for r in rows}
abs_errors = sorted(abs(r['relative_error_percent']) for r in validation)
median_error = (abs_errors[9] + abs_errors[10]) / 2
lines = ['# Estimated prover times at N=2^30', '',
    '**These are extrapolations, not N=2^30 benchmarks.** All five configurations use GF(2^256), BLAKE3, eight Rayon threads, and eta input polynomials of N coefficients each. Bolt is restored to w=4 and both Canopy RS blocks k/2 (k=N/4), with independent systematic/parity batches. Canopy heights are (2,3), and the Merkle baseline uses its optimized shared batch. WHIR timings retain the measured three-variable folding schedule and Exp. 3 height 4.', '',
    '## Estimated median commitment + opening time (minutes)', '',
    '| Configuration | eta=1 | eta=2 | eta=4 | eta=8 |', '|---|---:|---:|---:|---:|']
for scheme, label in SCHEMES.items():
    lines.append('| ' + label + ' | ' + ' | '.join(f"{by[scheme, eta]['estimated_total_minutes']:.1f}" for eta in [1, 2, 4, 8]) + ' |')
lines += ['', '## Commitment/opening breakdown for eta=1', '',
    '| Configuration | Commitment min | Opening min | Total min |', '|---|---:|---:|---:|']
for scheme, label in SCHEMES.items():
    r = by[scheme, 1]
    lines.append(f"| {label} | {r['estimated_commitment_seconds']/60:.2f} | {r['estimated_opening_seconds']/60:.2f} | {r['estimated_total_minutes']:.2f} |")
lines += ['', 'Commitment and opening are projected means; totals are projected medians, so the displayed columns need not sum exactly.', '', '## Method and limits', '',
    'The main estimates scale measured mean stage costs at N=2^22, independently for each eta, to obtain commitment/opening growth factors. Those phase factors are applied to all five saved repetitions, and the headline is their median projected total. Therefore the measured batching cost at eta=8 is retained, rather than estimated by multiplying the eta=1 total by eight. Commitment and opening both count; input generation, setup, and verification are excluded.', '',
    'Hashing, sparse Bolt matrix operations, evaluations, dot products, dense covectors, and cross-evaluation work scale with N. The linear factor is 2^(30-22)=256. RS and transposed-FFT stages scale with N log2(M), where M is the per-column coefficient count inferred from the implementation. Combined RS/commitment and alignment stages use that same FFT proxy for their aggregate cost. Fixed query-copy work remains fixed; full Merkle path collection grows with tree height. Recursive WHIR work sums a geometric sequence of codeword sizes and decreasing coefficient dimensions over the three-variable schedule, so the extra rounds at N=2^30 are included. This is an approximate work model, not cycle-accurate simulation; basis conversion, cache behavior, and degree-dependent query-count changes are not separately fitted.', '',
    'Saved WHIR substages have an explicit signed adjustment from independently measured suffix subtraction. Positive stage weights are normalized to their observed commitment/opening means before extrapolation; no negative work is projected. The Bolt stage timings already partition the measured execution. Stage sums are checked against source phase means; the five source repetitions also reproduce every saved median.', '',
    f'As a local check, the same model predicts measured N=2^22 medians from N=2^20 with a median absolute error of {median_error:.1f}% and maximum absolute error of {max(abs_errors):.1f}%. This much shorter extrapolation is not evidence of a confidence interval at N=2^30. Per-configuration errors are in `n20_to_n22_backtest.csv`.', '',
    'The table assumes comparable per-thread throughput with sufficient resident RAM and no swapping. One input polynomial alone is 32 GiB; eta=8 inputs occupy 256 GiB before codewords, trees, covectors, and temporary buffers. The current 16 GiB machine cannot execute these workloads in memory. Memory bandwidth, paging, and a different larger-memory machine could materially change actual time; those penalties are not included.', '',
    'These preserve the existing experiment scopes: WHIR/Canopy is a composition of measured, verifier-checked components, whereas Bolt measurements are complete serialized public-H adapter proofs, excluding Spark/Mulperm preprocessing and proofs. Executed Fiat-Shamir work is already included in the source timers.', '',
    '## Scaling sensitivity (not confidence bounds)', '',
    '| Configuration | eta | Linear-only bulk model min | Main stage model min | Stage model from N=2^20 min |',
    '|---|---:|---:|---:|---:|']
for r in rows:
    lines.append(f"| {r['label']} | {r['input_oracles']} | {r['linear_sensitivity_seconds']/60:.1f} | {r['estimated_total_minutes']:.1f} | {r['n20_anchor_sensitivity_seconds']/60:.1f} |")
lines += ['', 'The linear-only column removes FFT log growth and extra-round growth while retaining fixed/logarithmic query work. The N=2^20-anchor column applies the main model to the smaller measured dataset. Their differences show sensitivity to scaling assumptions and observed throughput; they are not hard lower/upper bounds.', '',
    'Files: `estimates.csv` contains all 20 median totals, mean phase estimates, and projected repetition ranges; `prover_time_estimates.csv` supplies the 80 estimated timing points at N=2^24, 2^26, 2^28, and 2^30 for the manuscript plots. `stage_estimates.csv` contains every measured stage, reconciliation factor, scaling rule, and projected contribution. `manifest.json` records source-file hashes. No new large-input proofs were generated.', '',
    'Reproduce with `python3 scripts/estimate_n30_prover_times.py`.', '']
(OUT / 'README.md').write_text('\n'.join(lines))
paths = [WHIR / 'timing_summary.csv', WHIR / 'timing_stages.csv', BOLT / 'timing_summary.csv', BOLT / 'timing_stage_summary.csv', BOLT / 'paper_proof_sizes.csv', ROOT / 'scripts/estimate_n30_prover_times.py', ROOT / 'src/additive_rs.rs', ROOT / 'src/bolt_batching.rs', ROOT / 'src/paper_experiments.rs'] + raw_paths
(OUT / 'manifest.json').write_text(json.dumps(dict(target_log_input=30, plot_target_log_inputs=[24, 26, 28, 30], anchor_log_input=22,
    field='GF(2^256)', hash='BLAKE3', rayon_threads=8, bolt_width=4, bolt_canopy_blocks='k/2 for both',
    measured_at_n30=False, headline_statistic='median of phase-scaled saved repetitions', median_local_backtest_error_percent=median_error,
    max_local_backtest_error_percent=max(abs_errors),
    source_sha256={str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}), indent=2) + '\n')
print('\n'.join(lines[:26]))
print(f'Local backtest median/max absolute error: {median_error:.1f}% / {max(abs_errors):.1f}%')
