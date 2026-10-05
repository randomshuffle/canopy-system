#!/usr/bin/env python3
"""Reproduce the balanced Bolt N=2^22 or N=2^24 timing recheck on a fresh output.

The experiment uses 32 pinned workers, one discarded warmup, four measured
passes per scheme and eta=1,2,4,8. Actual timings require Linux x86_64 with the
configured CPU allocation and sufficient RAM. Use --plan to inspect commands.
"""

import argparse
import json
import os
import platform
import shutil
import statistics
import subprocess
import sys
import time
from pathlib import Path

import aws_benchmark as aws
from public_metadata import public_metadata, public_text


ROOT = Path(__file__).resolve().parents[1]
SCHEMES = ['bolt_merkle_shared', 'bolt_canopy_split_quarter']
INPUTS = [1, 2, 4, 8]
REPETITIONS = 4


def summarize(cfg, jobs, out):
    """Validate every measured proof and retain the original recheck statistics."""
    samples, profiles, processes = [], [], []
    log_input = next(iter(cfg['grid']))
    for job in jobs:
        folder = Path(job['output'])
        record = json.loads((folder / 'process.json').read_text())
        assert record['status'] == 'complete' and record['exit_code'] == 0
        assert record['cpu_affinity'] == cfg['cpu_affinity']
        processes.append(record)
        rows = aws.read_csv(folder / 'prover_timings.csv')
        assert len(rows) == len(SCHEMES) * REPETITIONS
        assert {row['scheme'] for row in rows} == set(SCHEMES)
        for row in rows:
            assert row['proof_verified'] == 'true' and int(row['rayon_threads']) == cfg['threads']
            assert int(row['input_oracles']) == job['input_oracles']
            assert row['log_input'] == log_input and row['interleaving'] == '4'
            assert abs(float(row['total_prover_seconds']) - float(row['commitment_seconds'])
                       - float(row['opening_seconds'])) < 1e-8
        samples.extend(rows)
        profiles.extend(aws.read_csv(folder / 'prover_timing_stages.csv'))
    aws.write_csv(out / 'raw_samples.csv', samples)
    aws.write_csv(out / 'raw_stages.csv', profiles)
    summaries = []
    for eta in INPUTS:
        for scheme in SCHEMES:
            rows = [row for row in samples if int(row['input_oracles']) == eta
                    and row['scheme'] == scheme]
            assert len(rows) == REPETITIONS
            assert {int(row['iteration']) for row in rows} == set(range(REPETITIONS))
            summary = dict(log_input=int(log_input), input_oracles=eta, scheme=scheme,
                           repetitions=REPETITIONS, warmups=1,
                           systematic_h=rows[0]['systematic_h'], parity_h=rows[0]['parity_h'])
            for name in ('commitment_seconds', 'opening_seconds', 'total_prover_seconds'):
                values = [float(row[name]) for row in rows]
                for statistic, function in [('mean', statistics.mean), ('median', statistics.median),
                                            ('min', min), ('max', max), ('stdev', statistics.stdev)]:
                    summary[f'raw_{statistic}_{name}'] = function(values)
            anchor = [float(row['commitment_seconds']) for row in samples
                      if row['input_oracles'] == '1' and row['scheme'] == scheme]
            summary['mean_commitment_seconds'] = eta * statistics.mean(anchor)
            summary['mean_opening_seconds'] = summary['raw_mean_opening_seconds']
            summary['mean_prover_seconds'] = (summary['mean_commitment_seconds']
                                               + summary['mean_opening_seconds'])
            summary['commitment_timing_method'] = 'eta_times_mean_eta1_commitment_same_scheme'
            summary['timing_scope'] = 'serialized public-H adapter'
            summary['digest_conversion'] = 'direct_GF256_byte_bijection'
            summaries.append(summary)
    aws.write_csv(out / 'summary_iop_rounds.csv', summaries)
    stage_groups = {}
    stage_fields = ['input_oracles', 'scheme', 'phase', 'part', 'stage']
    for row in profiles:
        key = tuple(row[field] for field in stage_fields)
        by_iteration = stage_groups.setdefault(key, {})
        iteration = int(row['iteration'])
        by_iteration[iteration] = by_iteration.get(iteration, 0) + float(row['seconds'])
    assert all(set(by_iteration) == set(range(REPETITIONS))
               for by_iteration in stage_groups.values())
    stage_means = [dict(zip(stage_fields, key),
                        mean_seconds=statistics.mean(by_iteration.values()),
                        min_seconds=min(by_iteration.values()), max_seconds=max(by_iteration.values()))
                   for key, by_iteration in stage_groups.items()]
    aws.write_csv(out / 'stage_summary.csv', stage_means)
    validation = dict(
        completed_unix=time.time(), configurations=len(summaries),
        measured_samples=len(samples), verified_proofs=len(samples),
        warmups_per_configuration=1, repetitions=REPETITIONS,
        workers=cfg['threads'], cpu_affinity=cfg['cpu_affinity'],
        total_timing_job_wall_seconds=sum(process['wall_seconds'] for process in processes),
        peak_rss_gib=max(process['peak_rss_gib'] for process in processes),
        summary_sha256=aws.digest(out / 'summary_iop_rounds.csv'),
        note='Raw measurements retained. Reported commitments use eta times the mean eta=1 '
             'commitment; opening uses the measured eta-specific mean. Direct digest conversion, no rehash.')
    assert validation['configurations'] == 8 and validation['verified_proofs'] == 32
    aws.save(out / 'completion-validation.json', validation)
    return validation


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--log', type=int, choices=(22, 24), required=True)
    parser.add_argument('--output', type=Path, help='new or empty output directory')
    parser.add_argument('--bin-dir', type=Path, default=ROOT / 'target/aws-native/release')
    parser.add_argument('--skip-build', action='store_true',
                        help='reuse a native build with a matching native-build.json receipt')
    parser.add_argument('--offline', action='store_true', help='use cached Cargo dependencies')
    parser.add_argument('--plan', action='store_true',
                        help='print the configuration and commands without building or measuring')
    args = parser.parse_args()
    out = (args.output or ROOT / f'results/reproduce_bolt_n{args.log}').resolve()
    bins = args.bin_dir.resolve()
    try:
        aws.require_writable_campaign_output(out)
    except ValueError as error:
        parser.error(str(error))
    if out.exists() and (not out.is_dir() or any(out.iterdir())):
        parser.error(f'output must be new or empty: {out}')
    if not args.skip_build and bins.name != 'release':
        parser.error('--bin-dir must end in release when building')
    cfg = json.loads((ROOT / 'experiments/aws-r5/config-n30-eta1.json').read_text())
    cfg.update(campaign=f'gf256-bolt-n{args.log}-balanced-recheck-reproduction',
               grid={str(args.log): INPUTS}, warmups=1, repetitions=REPETITIONS,
               configurations=SCHEMES, memory_limit_gib=64)
    preparation = aws.preparation_commands(cfg, bins, out)[2:]
    jobs = [job for job in aws.timing_jobs(cfg, bins, out) if job['family'] == 'bolt']
    plan = dict(configuration=cfg, preparation_commands=preparation, jobs=jobs,
                measured_order=['Canopy, Merkle', 'Merkle, Canopy', 'Canopy, Merkle', 'Merkle, Canopy'],
                warmup_order='Merkle, Canopy', expected_measured_proofs=32)
    if args.plan:
        print(json.dumps(public_metadata(plan), indent=2))
        return
    if sys.platform != 'linux' or platform.machine() != 'x86_64':
        parser.error('timing rechecks require Linux x86_64; use --plan to inspect commands locally')

    out.mkdir(parents=True, exist_ok=True)
    aws.save(out / 'config.json', cfg)
    aws.save(out / 'plan.json', plan)
    receipt = dict(status='running', started_unix=time.time(), scope='32 measured serialized Bolt proofs',
                   configuration=cfg, source_sha256=aws.snapshot(), build_skipped=args.skip_build)
    aws.save(out / 'provenance.json', receipt)
    try:
        if not args.skip_build:
            command = [sys.executable, str(ROOT / 'scripts/aws_benchmark.py'), 'build',
                       '--config', str(out / 'config.json'), '--output', str(out), '--bin-dir', str(bins)]
            if args.offline:
                command.append('--offline')
            receipt['build_command'] = command
            aws.save(out / 'provenance.json', receipt)
            print(public_text('Building: ' + ' '.join(command)), flush=True)
            log = out / 'build.log'
            try:
                with log.open('w') as stream:
                    subprocess.run(command, cwd=ROOT, stdout=stream,
                                   stderr=subprocess.STDOUT, check=True)
            finally:
                log.write_text(public_text(log.read_text()))
        aws.apply_cpu_affinity(cfg)
        os.environ['RAYON_NUM_THREADS'] = str(cfg['threads'])
        aws.preflight(cfg, bins, out)
        shutil.copyfile(bins.parent / 'native-build.json', out / 'native-build.json')
        receipt['binaries'] = {name: aws.digest(bins / name) for name in aws.BINS}
        aws.save(out / 'provenance.json', receipt)
        for index, command in enumerate(preparation):
            aws.launch(command, out / 'preparation' / str(index), cfg)
        for job in jobs:
            aws.launch(job['command'], Path(job['output']), cfg)
        validation = summarize(cfg, jobs, out)
        receipt.update(status='complete', completion_validation_sha256=aws.digest(out / 'completion-validation.json'),
                       verified_proofs=validation['verified_proofs'], wall_seconds=time.time() - receipt['started_unix'])
        aws.save(out / 'provenance.json', receipt)
        print(public_text(f'Validated 32 measured proofs across eight points; results: {out}'), flush=True)
    except (Exception, KeyboardInterrupt) as error:
        receipt.update(status='interrupted' if isinstance(error, KeyboardInterrupt) else 'failed',
                       error=str(error), wall_seconds=time.time() - receipt['started_unix'])
        aws.save(out / 'provenance.json', receipt)
        raise


if __name__ == '__main__':
    main()
