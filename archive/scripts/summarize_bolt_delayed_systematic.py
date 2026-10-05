#!/usr/bin/env python3
"""Compare the half-rate systematic Canopy search with the retained Bolt suite."""
import csv
import hashlib
import json
import statistics
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'archive/results/bolt_delayed_systematic_eta1'
OLD = ROOT / 'archive/results/gf256_split_half_suite'
LOGS = list(range(18, 31, 2))


def read(path):
    with path.open() as f:
        return list(csv.DictReader(f))


def write(name, rows):
    fields = list(dict.fromkeys(k for row in rows for k in row))
    with (OUT / name).open('w', newline='') as f:
        writer = csv.DictWriter(f, fieldnames=fields)
        writer.writeheader()
        writer.writerows(rows)


new = read(OUT / 'proof_sizes/proof_sizes.csv')
strict = read(OUT / 'proof_sizes/strictly_delayed/proof_sizes.csv')
old = [r for r in read(OLD / 'paper_proof_sizes.csv') if r['input_oracles'] == '1']
grid = read(OUT / 'proof_sizes/systematic_join_search.csv')
assert len(new) == len(strict) == len(old) == 14
proofs = []
comparison = []
for n in LOGS:
    a = next(r for r in new if int(r['log_input']) == n and r['systematic_h'])
    d = next(r for r in strict if int(r['log_input']) == n and r['systematic_h'])
    o = next(r for r in old if int(r['log_input']) == n and r['systematic_h'])
    previous_m = next(r for r in old if int(r['log_input']) == n and not r['systematic_h'])
    split_m = next(r for r in new if int(r['log_input']) == n and not r['systematic_h'])
    m = min([previous_m, split_m], key=lambda r: float(r['total_bytes']))
    for r in [a, d]:
        assert r['systematic_h'] == '0' and r['delayed_systematic'] == 'true'
        assert int(r['systematic_rs_length']) == 2 ** (n - 1)
        for key in ['parity_h', 'parity_target_log', 'parity_rs_length',
                    'systematic_queries', 'parity_queries', 'external_row_bytes']:
            assert r[key] == o[key], (n, key)
        assert abs(float(r['verifier_hashes']) - sum(float(r[k]) for k in [
            'verifier_auth_hashes', 'verifier_hash_to_field', 'fs_finalizations'])) < 1e-6
    assert int(a['systematic_target_log']) == n - 2
    assert int(d['systematic_target_log']) == n - 5
    candidates = [r for r in grid if int(r['log_input']) == n]
    best = min(candidates, key=lambda r: float(r['total_bytes']))
    assert best['selected'] == 'true'
    assert abs(float(best['total_bytes']) - float(a['total_bytes'])) < 1e-6
    bounded = min((r for r in candidates if int(r['systematic_rs_length']) <= 2 ** (n - 3)),
                  key=lambda r: float(r['total_bytes']))
    for tag, row in [('merkle', m), ('previous_k_over_2', o),
                     ('half_rate_best', a), ('half_rate_strictly_delayed', d)]:
        proofs.append(dict(configuration=tag, **row))
    comparison.append(dict(
        log_input=n, input_oracles=1, baseline_scheme=m['scheme'],
        **{f'{label}_{column}': float(row[column])
           for label, row in [('merkle', m), ('previous', o), ('best', a), ('strict', d)]
           for column in ['total_bytes', 'verifier_hashes']},
        proof_saving_vs_previous_percent=100 * (1 - float(a['total_bytes']) / float(o['total_bytes'])),
        hash_saving_vs_previous_percent=100 * (1 - float(a['verifier_hashes']) / float(o['verifier_hashes'])),
        proof_saving_vs_merkle_percent=100 * (1 - float(a['total_bytes']) / float(m['total_bytes'])),
        hash_saving_vs_merkle_percent=100 * (1 - float(a['verifier_hashes']) / float(m['verifier_hashes'])),
        bounded_best_height=int(bounded['systematic_h']),
        bounded_best_folds=bounded['canopy_folds_before_join'],
        bounded_best_total_bytes=float(bounded['total_bytes']),
        bounded_best_verifier_hashes=float(bounded['verifier_hashes']),
        bounded_saving_vs_previous_percent=100 * (1 - float(bounded['total_bytes']) / float(o['total_bytes']))))
write('proof_sizes_with_hashes.csv', proofs)
write('comparison.csv', comparison)

summary = []
stage_summary = []
for directory in ['timing_n22', 'strict_timing_n22']:
    samples = read(OUT / directory / 'prover_timings.csv')
    assert len(samples) == (15 if directory == 'timing_n22' else 10)
    assert all(r['proof_verified'] == 'true' and r['rayon_threads'] == '8'
               and r['log_input'] == '22' and r['input_oracles'] == '1' for r in samples)
    grouped = defaultdict(list)
    for r in samples:
        grouped[r['scheme']].append(r)
    stage_iterations = defaultdict(float)
    for r in read(OUT / directory / 'prover_timing_stages.csv'):
        stage_iterations[r['scheme'], r['phase'], r['part'], r['stage'], r['iteration']] += float(r['seconds'])
    stage_groups = defaultdict(list)
    for (*key, iteration), value in stage_iterations.items():
        stage_groups[tuple(key)].append(value)
    for (scheme, phase, part, stage), values in sorted(stage_groups.items()):
        stage_summary.append(dict(run=directory, scheme=scheme, phase=phase, part=part,
                                  stage=stage, mean_seconds=statistics.mean(values)))
    for scheme, rr in sorted(grouped.items()):
        assert len(rr) == 5
        values = [float(r['total_prover_seconds']) for r in rr]
        for r in rr:
            assert abs(float(r['commitment_seconds']) + float(r['opening_seconds'])
                       - float(r['total_prover_seconds'])) < 1e-8
        tag = ('half_rate_strictly_delayed' if directory == 'strict_timing_n22'
               else 'half_rate_best') if 'delayed' in scheme else (
                   'previous_k_over_2' if 'canopy' in scheme else 'merkle')
        expected = next(r for r in proofs if r['configuration'] == tag and r['log_input'] == '22')
        s = dict(run=directory, configuration=tag, scheme=scheme, log_input=22, input_oracles=1,
                 mean_commitment_seconds=statistics.mean(float(r['commitment_seconds']) for r in rr),
                 mean_opening_seconds=statistics.mean(float(r['opening_seconds']) for r in rr),
                 median_total_seconds=statistics.median(values), mean_total_seconds=statistics.mean(values),
                 min_total_seconds=min(values), max_total_seconds=max(values),
                 mean_serialized_bytes=statistics.mean(int(r['proof_bytes']) for r in rr))
        s['relative_size_error'] = s['mean_serialized_bytes'] / float(expected['total_bytes']) - 1
        assert abs(s['relative_size_error']) < 0.002
        for phase in ['commitment', 'opening']:
            total = sum(r['mean_seconds'] for r in stage_summary
                        if r['run'] == directory and r['scheme'] == scheme and r['phase'] == phase)
            assert abs(total - s[f'mean_{phase}_seconds']) < 1e-8
        summary.append(s)
write('timing_summary.csv', summary)
write('timing_stage_summary.csv', stage_summary)

checks = read(OUT / 'fiat_shamir_validation.csv') + read(OUT / 'strict_fiat_shamir_validation.csv')
assert len(checks) == 8
assert all(r['proof_verified'] == r['trace_matches_model'] == 'true' for r in checks)
max_error = max(abs(r['relative_size_error']) for r in summary)
at = lambda tag, run='timing_n22': next(r for r in summary if r['configuration'] == tag and r['run'] == run)
best_time = at('half_rate_best')['median_total_seconds']
old_time = at('previous_k_over_2')['median_total_seconds']
last = comparison[-1]

lines = ['# Bolt: half-rate systematic Canopy and delayed joins, eta = 1', '',
    'The search keeps Bolt-max w=4, GF(2^256), BLAKE3, the existing 128-bit conditional soundness assumptions, and the selected parity configuration. The parity Canopy block is k/2, with k=N/4, and its height remains 3. Systematic Canopy starts at rate 1/2: at height h its message has k/2^h elements and its block has 2k/2^h. Thus this experiment releases the old fixed systematic block-length constraint.', '',
    'Each candidate folds systematic Canopy zero or more times before batching with the systematic Bolt code-switch claim. Each independent Canopy fold reduces the message by 8 and the block by 2. After r folds, the join message has k/2^(h+3r) coefficients and inverse rate 2^(1+2r). Both claims are authenticated before they enter the shared native matrix-batched WHIR tail. The parity tail stays separate. All added commitments, query openings, cross-matrix entries, and transcript operations are counted.', '',
    'The search includes every height from 0 through the full tree, all feasible joins after complete three-variable Canopy folds, and full-height constant layers opened directly. It minimizes proof bytes; it does not optimize prover time or search other initial Canopy rates. Proof estimates average 100 deterministic query samples. Verifier hashes count authentication, hash-to-field, and Fiat–Shamir hash/XOF finalizations; FS adds no proof bytes.', '',
    'For a fair baseline, the Merkle split-tail search also includes immediate joins, without the formerly required prefix transition. It slightly improves the Merkle proof at N=2^18 through 2^24 (at most 0.27%); the retained shared-tail baseline remains smaller at N=2^26 through 2^30. Timings below use the newly selected smaller-proof baseline. The boundary audit is in `proof_sizes/merkle_immediate_join_check.csv`.', '',
    '**Result:** every size selects h=0 and zero independent Canopy folds. This is an immediate join, with a systematic Canopy block of 2k=N/2, four times the previous k/2 block. Height zero encodes the hashes of individual systematic rows, eliminating their bottom authentication paths. The best candidate requiring a delayed join also uses h=0 and folds once, joining at message k/8 and rate 1/8. Its proof is slightly larger.', '',
    '## Proof size (KiB) / verifier hashes', '',
    '| N | Bolt + Merkle | Previous Canopy k/2 | Best half-rate Canopy: immediate | Best strictly delayed: one fold |',
    '|---|---:|---:|---:|---:|']
for r in comparison:
    lines.append('| 2^' + str(r['log_input']) + ' | ' + ' | '.join(
        f"{r[label + '_total_bytes'] / 1024:.2f} / {r[label + '_verifier_hashes']:,.0f}"
        for label in ['merkle', 'previous', 'best', 'strict']) + ' |')
lines += ['',
    f"At N=2^30 the unconstrained half-rate winner saves {last['proof_saving_vs_previous_percent']:.1f}% in proof bytes and {last['hash_saving_vs_previous_percent']:.1f}% in verifier hashes versus the previous Canopy configuration; versus Merkle the savings are {last['proof_saving_vs_merkle_percent']:.1f}% and {last['hash_saving_vs_merkle_percent']:.1f}%. This gain comes from allowing the larger systematic block, not from delaying its join.", '',
    '## Keeping the previous systematic block-length budget', '',
    'Restricting the new search to blocks at most k/2 removes the larger-block advantage. At N=2^22 the winner is still h=2 with an immediate join, reproducing the previous proof size and hash count. At N=2^30, h=2 with one independent Canopy fold becomes marginally better:', '',
    '| N=2^30, systematic block k/2 | Proof (KiB) | Verifier hashes |',
    '|---|---:|---:|',
    f"| Previous immediate join | {last['previous_total_bytes']/1024:.2f} | {last['previous_verifier_hashes']:,.0f} |",
    f"| One Canopy fold before joining | {last['bounded_best_total_bytes']/1024:.2f} | {last['bounded_best_verifier_hashes']:,.0f} |", '',
    f"That is a {last['bounded_saving_vs_previous_percent']:.3f}% proof saving. The full bounded comparison is retained in `comparison.csv`.", '',
    '## Measured prover time, N=2^22 and eta=1', '',
    'The first three configurations were measured in rotating order in one run, with one warmup and five repetitions each, using eight Rayon threads. The strictly delayed candidate was measured in a separate alternating run with a fresh Merkle control. All 30 warmup/measured proofs verified. Times include commitment and opening, including FS; input generation, public setup, and verification are excluded. These are complete serialized public-H adapter proofs; Spark/Mulperm remain outside the scope.', '',
    '| Configuration / run | Mean commit (s) | Mean open (s) | Median total (s) | Min–max total (s) |',
    '|---|---:|---:|---:|---:|']
labels = [('merkle', 'timing_n22', 'Bolt + Merkle'),
          ('previous_k_over_2', 'timing_n22', 'Previous Canopy k/2'),
          ('half_rate_best', 'timing_n22', 'Half-rate Canopy, immediate'),
          ('half_rate_strictly_delayed', 'strict_timing_n22', 'Half-rate Canopy, one fold / separate run'),
          ('merkle', 'strict_timing_n22', 'Merkle control / separate run')]
for tag, directory, label in labels:
    r = at(tag, directory)
    lines.append(f"| {label} | {r['mean_commitment_seconds']:.3f} | {r['mean_opening_seconds']:.3f} | {r['median_total_seconds']:.3f} | {r['min_total_seconds']:.3f}–{r['max_total_seconds']:.3f} |")
lines += ['',
    f"The proof-optimal half-rate configuration takes {100*(best_time/old_time-1):.1f}% more median prover time than the previous k/2 Canopy configuration in the matched run. Phase means need not sum to the median total. The one-fold result is from a separate run, so small timing differences should not be interpreted as a reliable ranking.", '',
    f"All eight full Fiat–Shamir traces match the model, covering both immediate and genuinely delayed configurations at N=2^18 and N=2^22 plus split Merkle controls. Maximum mean serialized-size deviation from the sampled model is {100*max_error:.3f}%. The model excludes small serializer framing overhead. Stage timings, raw repetitions, transcript byte counts, and process memory records are retained.", '',
    'The legacy `canopy_layout=equal_half` tag in raw new timing rows identifies the retained parity setup; `proof_sizes.csv` gives the actual systematic length and join geometry. For the two new candidates this length is 2k, not k/2.', '',
    '## Reproduce', '', '```sh',
    'cargo build --offline --release --example gf256_bolt_split',
    'python3 scripts/measure_gf256_run.py --record-dir archive/results/bolt_delayed_systematic_eta1/proof_sizes --threads 8 -- target/release/examples/gf256_bolt_split estimate-delayed --inputs 1 --samples 100 --reference archive/results/gf256_split_half_suite --output archive/results/bolt_delayed_systematic_eta1/proof_sizes',
    'python3 scripts/measure_gf256_run.py --record-dir archive/results/bolt_delayed_systematic_eta1/timing_n22 --threads 8 -- target/release/examples/gf256_bolt_split time --log 22 --inputs 1 --warmups 1 --repetitions 5 --width 4 --estimates archive/results/bolt_delayed_systematic_eta1/proof_sizes --alternate-estimates archive/results/gf256_split_half_suite/proof_sizes --paper-only --output archive/results/bolt_delayed_systematic_eta1/timing_n22',
    'python3 scripts/measure_gf256_run.py --record-dir archive/results/bolt_delayed_systematic_eta1/strict_timing_n22 --threads 8 -- target/release/examples/gf256_bolt_split time --log 22 --inputs 1 --warmups 1 --repetitions 5 --width 4 --estimates archive/results/bolt_delayed_systematic_eta1/proof_sizes/strictly_delayed --paper-only --output archive/results/bolt_delayed_systematic_eta1/strict_timing_n22',
    'cargo build --offline --release --features fs-accounting --example gf256_bolt_split',
    'RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_split validate --input archive/results/bolt_delayed_systematic_eta1/proof_sizes/proof_sizes.csv --output archive/results/bolt_delayed_systematic_eta1/fiat_shamir_validation.csv',
    'RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_split validate --input archive/results/bolt_delayed_systematic_eta1/proof_sizes/strictly_delayed/proof_sizes.csv --output archive/results/bolt_delayed_systematic_eta1/strict_fiat_shamir_validation.csv',
    'python3 archive/scripts/summarize_bolt_delayed_systematic.py', '```', '',
    'Use a build without `fs-accounting` for timings. The Merkle baseline is the smaller of the retained shared configuration and the split configuration including immediate joins. Earlier runs against the retained baseline are preserved in `before_merkle_boundary_audit/`. These results do not overwrite the manuscript data or extrapolate large-input timings.']
(OUT / 'README.md').write_text('\n'.join(lines) + '\n')
paths = ['src/bolt_delayed.rs', 'src/bolt_split.rs', 'src/bolt_batching.rs',
         'src/bolt_multi.rs', 'src/bolt_fiat_shamir.rs', 'src/bolt_experiment.rs',
         'examples/gf256_bolt_split.rs', 'archive/scripts/summarize_bolt_delayed_systematic.py',
         'archive/results/gf256_split_half_suite/paper_proof_sizes.csv',
         'archive/results/gf256_split_half_suite/proof_sizes/independent_height_search.csv', 'Cargo.lock']
manifest = dict(field='GF(2^256)', hash='BLAKE3', width=4, input_oracles=[1], log_inputs=LOGS,
    timing_log_input=22, systematic_initial_rate='1/2', parity_block_length='k/2', parity_height=3,
    security_bits=128, query_security_bits=132, whir_local_security_bits=136,
    distance_assumption_gamma=0.013, samples=100, independent_canopy_folding_variables=3,
    search_objective='proof bytes', search_candidates=len(grid),
    measured_proofs=25, verified_warmups=5, full_fs_trace_validations=8,
    max_mean_relative_size_error=max_error,
    source_sha256={p: hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in paths},
    processes={d: json.loads((OUT/d/'process.json').read_text())
               for d in ['proof_sizes', 'timing_n22', 'strict_timing_n22']})
assert all(p['exit_code'] == 0 for p in manifest['processes'].values())
(OUT / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
print('\n'.join(lines[12:24]))
print(f'Maximum mean serialized-size discrepancy: {100*max_error:.3f}%')
