#!/usr/bin/env python3
"""Reproduce the 75 final proof-size/hash estimates without large timing workloads.

Uses the two checked-in final campaign configurations and the current BCGM m=10
implementation. Outputs must be new or empty so published evidence is preserved.
"""

import argparse
import csv
import json
import os
import subprocess
import time
from pathlib import Path

import aws_benchmark as aws
from public_metadata import public_text


ROOT = Path(__file__).resolve().parents[1]
ESTIMATE_BINS = aws.BINS[:4]


def positive_int(value):
    number = int(value)
    if number < 1:
        raise argparse.ArgumentTypeError('must be a positive integer')
    return number


def configuration():
    paths = [ROOT / 'experiments/aws-r5/config.json',
             ROOT / 'experiments/aws-r5/config-n30-eta1.json']
    configs = [json.loads(path.read_text()) for path in paths]
    cfg = configs[0].copy()
    for key in ('rust_toolchain', 'field', 'hash', 'security_bits',
                'whir_local_security_bits', 'proof_samples', 'whir_proof_seed',
                'bolt_seed', 'bolt_width', 'bolt_distance_assumption',
                'whir_canopy_codeword_log_reduction', 'bolt_canopy_block_divisor',
                'delayed_systematic_join', 'configurations'):
        if configs[0][key] != configs[1][key]:
            raise ValueError(f'final campaign configurations disagree on {key}')
    grid = {}
    for config in configs:
        for log_input, inputs in config['grid'].items():
            grid.setdefault(int(log_input), set()).update(inputs)
    cfg['grid'] = {str(log_input): sorted(inputs)
                   for log_input, inputs in sorted(grid.items())}
    cfg['campaign'] = 'paper-bcgm-m10-estimates'
    return cfg, paths


def compare_report(path):
    """Compare deterministic report fields with the published current estimates."""
    reference = ROOT / 'results/paper_bcgm_m10/proof_sizes_and_hashes.csv'
    key_fields = ('log_input', 'input_oracles', 'scheme')

    def rows_by_key(csv_path):
        with csv_path.open(newline='') as stream:
            rows = list(csv.DictReader(stream))
        result = {tuple(row[field] for field in key_fields): row for row in rows}
        if len(result) != len(rows):
            raise ValueError(f'duplicate configuration rows in {csv_path}')
        return result

    actual, expected = rows_by_key(path), rows_by_key(reference)
    differences = []
    if actual.keys() != expected.keys():
        differences.append({'field': 'configuration_keys',
                            'missing': sorted(expected.keys() - actual.keys()),
                            'extra': sorted(actual.keys() - expected.keys())})
    for key in sorted(actual.keys() & expected.keys()):
        for field in sorted(set(actual[key]) | set(expected[key])):
            left, right = actual[key].get(field), expected[key].get(field)
            if left != right:
                differences.append({'configuration': key, 'field': field,
                                    'reproduced': left, 'published': right})
    return {'rows': len(actual), 'reference': str(reference),
            'reference_sha256': aws.digest(reference),
            'report_sha256': aws.digest(path),
            'matches_published_estimates': not differences,
            'differences': differences, 'timings_rerun': False}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, default=ROOT / 'results/reproduce_estimates',
                        help='new or empty output directory')
    parser.add_argument('--bin-dir', type=Path, default=ROOT / 'target/release',
                        help='release binary directory (with --skip-build, any directory)')
    parser.add_argument('--skip-build', action='store_true',
                        help='use existing binaries; their hashes are still recorded')
    parser.add_argument('--offline', action='store_true',
                        help='build using only cached Cargo dependencies')
    parser.add_argument('--threads', type=positive_int,
                        default=min(8, os.cpu_count() or 1))
    args = parser.parse_args()
    out, bins = args.output.resolve(), args.bin_dir.resolve()
    try:
        aws.require_writable_campaign_output(out)
    except ValueError as error:
        parser.error(str(error))
    if out.exists() and (not out.is_dir() or any(out.iterdir())):
        parser.error(f'output must be new or empty: {out}')
    if not args.skip_build and bins.name != 'release':
        parser.error('--bin-dir must end in release when building; use --skip-build for other paths')

    cfg, config_paths = configuration()
    cfg['threads'] = args.threads
    cfg.pop('cpu_affinity', None)
    env = {**os.environ, 'RAYON_NUM_THREADS': str(args.threads),
           'CARGO_BUILD_JOBS': str(args.threads)}
    toolchain = cfg['rust_toolchain']
    build_command = None
    if not args.skip_build:
        build_command = ['cargo', f'+{toolchain}', 'build', '--locked', '--release',
                         '--target-dir', str(bins.parent), '--bin', 'canopy-system',
                         '--example', 'gf256_bolt_suite', '--example', 'gf256_bolt_split',
                         '--example', 'whir_fiat_shamir']
        if args.offline:
            build_command.append('--offline')
        print(public_text('Building: ' + ' '.join(build_command)), flush=True)
        subprocess.run(build_command, cwd=ROOT, env=env, check=True)
    missing = [str(bins / name) for name in ESTIMATE_BINS
               if not (bins / name).is_file() or not os.access(bins / name, os.X_OK)]
    if missing:
        parser.error('missing executable(s): ' + ', '.join(missing))

    out.mkdir(parents=True, exist_ok=True)
    commands = aws.preparation_commands(cfg, bins, out)
    receipt = {
        'status': 'running', 'bound': 'BCGM26, Definition 9.1 and Theorem 9.2',
        'reference': 'https://eprint.iacr.org/2025/2051', 'm': 10,
        'affine_error': '(m+1/2)^7 * n_RS^2 / (3 * rho^(3/2) * |F|)',
        'multilinear_error': 'sum over degree-one coordinate challenges',
        'scope': 'Proof-size and verifier-hash estimates only; no timings.',
        'configuration': cfg, 'configuration_files': {str(path): aws.digest(path)
                                                     for path in config_paths},
        'source_sha256': aws.snapshot(), 'build_command': build_command,
        'build_skipped': args.skip_build,
        'environment': {key: env.get(key) for key in
                        ('RAYON_NUM_THREADS', 'CARGO_BUILD_JOBS', 'RUSTFLAGS')},
        'binaries': {name: aws.digest(bins / name) for name in ESTIMATE_BINS},
        'commands': commands, 'jobs': [],
    }
    if not args.skip_build:
        receipt['rustc'] = subprocess.check_output(
            ['rustc', f'+{toolchain}', '--version'], text=True).strip()
    aws.save(out / 'provenance.json', receipt)
    try:
        for index, command in enumerate(commands):
            log = out / 'logs' / f'{index:02}.log'
            log.parent.mkdir(exist_ok=True)
            print(public_text(f'[{index + 1}/{len(commands)}] ' + ' '.join(command)), flush=True)
            start = time.monotonic()
            with log.open('w') as stream:
                result = subprocess.run(command, cwd=ROOT, env=env, stdout=stream,
                                        stderr=subprocess.STDOUT)
            log.write_text(public_text(log.read_text()))
            receipt['jobs'].append({'command': command, 'log': str(log),
                                    'seconds': time.monotonic() - start,
                                    'exit_code': result.returncode})
            aws.save(out / 'provenance.json', receipt)
            if result.returncode:
                raise RuntimeError(f'estimator failed with exit {result.returncode}; see {log}')
        rows = aws.proof_report(cfg, out)
        validation = compare_report(out / 'proof_sizes_and_hashes.csv')
        aws.save(out / 'validation.json', validation)
        if not validation['matches_published_estimates']:
            raise RuntimeError(f'estimates differ from the published report; see {out / "validation.json"}')
        receipt.update(status='complete', rows=len(rows),
                       output_sha256=validation['report_sha256'])
        aws.save(out / 'provenance.json', receipt)
        print(public_text(f'Complete: {len(rows)} proof/hash rows in {out}'), flush=True)
        print('All fields match the published 75-point estimates.', flush=True)
    except (Exception, KeyboardInterrupt) as error:
        receipt.update(status='interrupted' if isinstance(error, KeyboardInterrupt) else 'failed',
                       error=str(error))
        aws.save(out / 'provenance.json', receipt)
        raise


if __name__ == '__main__':
    main()
