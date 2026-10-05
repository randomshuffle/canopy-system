"""Recheck Bolt's small-N timing crossover with balanced measured run order."""
import csv
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import statistics
import time
import sys

BASE = Path(__file__).resolve().parent
ROOT = Path(os.environ.get('CANOPY_ROOT', '.')).resolve()
sys.path.insert(0, str(ROOT / 'scripts'))
spec = importlib.util.spec_from_file_location('aws_benchmark', ROOT / 'scripts/aws_benchmark.py')
aws = importlib.util.module_from_spec(spec)
spec.loader.exec_module(aws)
BINS = (ROOT / 'target/aws-native/release').resolve()
OUT = BASE / 'results'


def main():
    cfg = json.loads((ROOT / 'experiments/aws-r5/config-n30-eta1.json').read_text())
    cfg.update(campaign='gf256-bolt-n22-balanced-recheck-20260917',
               grid={'22': [1, 2, 4, 8]}, warmups=1, repetitions=4,
               configurations=['bolt_merkle_shared', 'bolt_canopy_split_quarter'],
               memory_limit_gib=64)
    aws.apply_cpu_affinity(cfg)
    os.environ['RAYON_NUM_THREADS'] = '32'
    aws.save(BASE / 'config.json', cfg)
    (BASE / 'phase').write_text('preflight\n')
    aws.preflight(cfg, BINS, OUT)
    shutil.copyfile(BINS.parent / 'native-build.json', OUT / 'native-build.json')
    jobs = [j for j in aws.timing_jobs(cfg, BINS, OUT) if j['family'] == 'bolt']
    prep = aws.preparation_commands(cfg, BINS, OUT)[2:]
    aws.save(OUT / 'plan.json', dict(configuration=cfg, jobs=jobs, preparation_commands=prep,
             measured_order=['Canopy, Merkle', 'Merkle, Canopy', 'Canopy, Merkle', 'Merkle, Canopy'],
             warmup_order='Merkle, Canopy', source_campaign=str(ROOT.parent)))
    (BASE / 'phase').write_text('preparation\n')
    for i, command in enumerate(prep):
        aws.launch(command, OUT / 'preparation' / str(i), cfg)
    (BASE / 'phase').write_text('timing\n')
    for job in jobs:
        aws.launch(job['command'], Path(job['output']), cfg)
    (BASE / 'phase').write_text('validation\n')
    samples = []
    profiles = []
    processes = []
    for job in jobs:
        folder = Path(job['output'])
        rec = json.loads((folder / 'process.json').read_text())
        assert rec['status'] == 'complete' and rec['exit_code'] == 0
        assert rec['cpu_affinity'] == cfg['cpu_affinity']
        processes.append(rec)
        rows = aws.read_csv(folder / 'prover_timings.csv')
        assert len(rows) == 8
        assert {r['scheme'] for r in rows} == set(cfg['configurations'])
        for row in rows:
            assert row['proof_verified'] == 'true' and row['rayon_threads'] == '32'
            assert int(row['input_oracles']) == job['input_oracles']
            assert row['log_input'] == '22' and row['interleaving'] == '4'
            assert abs(float(row['total_prover_seconds']) - float(row['commitment_seconds']) - float(row['opening_seconds'])) < 1e-8
        samples.extend(rows)
        profiles.extend(aws.read_csv(folder / 'prover_timing_stages.csv'))
    aws.write_csv(OUT / 'raw_samples.csv', samples)
    aws.write_csv(OUT / 'raw_stages.csv', profiles)
    summaries = []
    for eta in [1, 2, 4, 8]:
        for scheme in cfg['configurations']:
            rows = [r for r in samples if int(r['input_oracles']) == eta and r['scheme'] == scheme]
            assert {int(r['iteration']) for r in rows} == set(range(4))
            out = dict(log_input=22, input_oracles=eta, scheme=scheme, repetitions=4,
                       warmups=1, systematic_h=rows[0]['systematic_h'], parity_h=rows[0]['parity_h'])
            for name in ['commitment_seconds', 'opening_seconds', 'total_prover_seconds']:
                values = [float(r[name]) for r in rows]
                for stat, fn in [('mean', statistics.mean), ('median', statistics.median),
                                 ('min', min), ('max', max), ('stdev', statistics.stdev)]:
                    out[f'raw_{stat}_{name}'] = fn(values)
            anchor = [float(r['commitment_seconds']) for r in samples
                      if r['input_oracles'] == '1' and r['scheme'] == scheme]
            out['mean_commitment_seconds'] = eta * statistics.mean(anchor)
            out['mean_opening_seconds'] = out['raw_mean_opening_seconds']
            out['mean_prover_seconds'] = out['mean_commitment_seconds'] + out['mean_opening_seconds']
            out['commitment_timing_method'] = 'eta_times_mean_eta1_commitment_same_scheme'
            out['timing_scope'] = 'serialized public-H adapter'
            out['digest_conversion'] = 'direct_GF256_byte_bijection'
            summaries.append(out)
    aws.write_csv(OUT / 'summary_iop_rounds.csv', summaries)
    stage_groups = {}
    for row in profiles:
        key = tuple(row[k] for k in ['input_oracles', 'scheme', 'phase', 'part', 'stage'])
        by_iteration = stage_groups.setdefault(key, {})
        it = int(row['iteration'])
        by_iteration[it] = by_iteration.get(it, 0) + float(row['seconds'])
    stage_means = [dict(zip(['input_oracles', 'scheme', 'phase', 'part', 'stage'], key),
                        mean_seconds=statistics.mean(by_it.values()),
                        min_seconds=min(by_it.values()), max_seconds=max(by_it.values()))
                   for key, by_it in stage_groups.items()]
    assert all(set(by_it) == set(range(4)) for by_it in stage_groups.values())
    aws.write_csv(OUT / 'stage_summary.csv', stage_means)
    aws.save(OUT / 'completion-validation.json', dict(
        completed_unix=time.time(), configurations=8, measured_samples=32, verified_proofs=32,
        warmups_per_configuration=1, repetitions=4, workers=32, cpu_affinity=cfg['cpu_affinity'],
        total_timing_job_wall_seconds=sum(p['wall_seconds'] for p in processes),
        peak_rss_gib=max(p['peak_rss_gib'] for p in processes),
        summary_sha256=hashlib.sha256((OUT / 'summary_iop_rounds.csv').read_bytes()).hexdigest(),
        note='Raw measurements retained. Reported commitments use eta times the mean eta=1 commitment; opening uses the measured eta-specific mean. Direct digest conversion, no rehash.'))
    (BASE / 'phase').write_text('complete\n')
    print('Validated 32 measured proofs across all eight configuration points.', flush=True)


try:
    main()
except BaseException:
    (BASE / 'campaign.exit').write_text('1\n')
    raise
else:
    (BASE / 'campaign.exit').write_text('0\n')
