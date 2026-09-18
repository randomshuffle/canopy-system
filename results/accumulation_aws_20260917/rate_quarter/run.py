#!/usr/bin/env python3
"""Measure the rate-1/4 plain Merkle extension using the original library sources."""
import csv
import hashlib
import json
import os
from pathlib import Path
import platform
import resource
import shutil
import subprocess
import time

VARIANT = Path(__file__).resolve().parent
ROOT = VARIANT.parent
OUT = VARIANT / 'measurements'
OUT.mkdir(exist_ok=True)
CPUS = list(range(16)) + list(range(48, 64))
os.sched_setaffinity(0, CPUS)
ENV = dict(os.environ, RUSTFLAGS='-C target-cpu=native',
           RAYON_NUM_THREADS='32', CARGO_BUILD_JOBS='32')
ENV['PATH'] = str(Path.home() / '.cargo/bin') + os.pathsep + ENV['PATH']


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def save(name, value):
    (OUT / name).write_text(json.dumps(value, indent=2) + '\n')


def capture(args):
    return subprocess.check_output(args, text=True, env=ENV).strip()


def run(name, command):
    print(f'{name}: {command}', flush=True)
    start = time.time()
    record = dict(command=command, started_unix=start, status='running',
                  threads=32, cpu_affinity=CPUS)
    if Path(command[0]).is_file():
        record['binary_sha256'] = digest(Path(command[0]))
    save(name + '.json', record)
    with (OUT / (name + '.log')).open('w') as log:
        def limit_memory():
            resource.setrlimit(resource.RLIMIT_AS, (128 * 2**30, 128 * 2**30))
        process = subprocess.Popen(command, cwd=ROOT, env=ENV, stdout=log,
                                   stderr=subprocess.STDOUT, preexec_fn=limit_memory)
        _, status, usage = os.wait4(process.pid, 0)
        process.returncode = os.waitstatus_to_exitcode(status)
    record.update(exit_code=process.returncode, wall_seconds=time.time()-start,
                  peak_rss_gib=usage.ru_maxrss / 2**20,
                  user_seconds=usage.ru_utime, system_seconds=usage.ru_stime,
                  status='complete' if process.returncode == 0 else 'failed')
    save(name + '.json', record)
    if process.returncode:
        raise RuntimeError((OUT / (name + '.log')).read_text()[-12000:])
    print(f'{name}: completed in {record["wall_seconds"]:.2f}s', flush=True)


manifest = json.loads((VARIANT / 'source_sha256.json').read_text())
example = 'examples/accumulation_one_fold.rs'
assert all(digest(ROOT / path) == expected for path, expected in manifest.items() if path != example)
shutil.copy2(ROOT / example, VARIANT / 'original_accumulation_one_fold.rs')
shutil.copy2(VARIANT / 'accumulation_one_fold.rs', ROOT / example)
assert all(digest(ROOT / path) == expected for path, expected in manifest.items())
save('host.json', dict(hostname='benchmark-host', platform=platform.platform(),
                      recorded_utc=capture(['date', '-u', '+%Y-%m-%dT%H:%M:%SZ']),
                      lscpu=capture(['lscpu']), meminfo=Path('/proc/meminfo').read_text(),
                      rustc=capture(['rustc', '--version']), cargo=capture(['cargo', '--version']),
                      threads=32, cpu_affinity=CPUS, rustflags=ENV['RUSTFLAGS'],
                      source_sha256=manifest))
run('build', ['cargo', 'build', '--locked', '--release', '--example',
              'accumulation_one_fold', '--example', 'aws_field_check'])
run('field_check', [str(ROOT / 'target/release/examples/aws_field_check'), '--require-intel'])
bins = OUT / 'bin'
bins.mkdir(exist_ok=True)
binary = bins / 'accumulation_one_fold'
shutil.copy2(ROOT / 'target/release/examples/accumulation_one_fold', binary)
common = ['--merkle-only', '--log-inverse-rate', '2']
run('smoke', [str(binary), *common, '--logs', '10', '--warmups', '0', '--repetitions', '1',
              '--check-tampering', '--output', str(OUT / 'smoke.csv')])
run('timing', [str(binary), *common, '--logs', '18,20,22,24', '--warmups', '1', '--repetitions', '3',
               '--seed', '91', '--vary-inputs', '--output', str(OUT / 'timing_samples.csv')])
run('build_instrumented', ['cargo', 'build', '--locked', '--release', '--features',
                           'fs-accounting', '--example', 'accumulation_one_fold'])
trace = bins / 'accumulation_one_fold_fs'
shutil.copy2(ROOT / 'target/release/examples/accumulation_one_fold', trace)
run('trace', [str(trace), *common, '--logs', '18,20,22,24', '--warmups', '0', '--repetitions', '3',
              '--seed', '92', '--vary-inputs', '--output', str(OUT / 'hash_validation.csv')])
with (OUT / 'timing_samples.csv').open() as f:
    timings = list(csv.DictReader(f))
with (OUT / 'hash_validation.csv').open() as f:
    traces = list(csv.DictReader(f))
assert len(timings) == len(traces) == 12
key = lambda r: (r['log_n'], r['scheme'], r['input_seed'])
traces = {key(r): r for r in traces}
for row in timings:
    other = traces[key(row)]
    for field in ('verified', 'actual_auth_hashes', 'proof_bytes', 'queries_0', 'queries_1'):
        assert row[field] == other[field], (key(row), field)
    assert row['scheme'] == 'merkle_same_code_batch_rate_quarter'
    assert row['verified'] == 'true' and int(row['queries_0']) == 147
    assert int(other['fs_finalizations']) == int(row['log_n']) + 4
save('complete.json', dict(status='complete', verified_measured_runs=12,
                           input_seeds=[92, 93, 94], repetitions=3, inverse_rate=4,
                           warmups_per_scheme_and_size=1, queries_per_source=147,
                           timing_binary_sha256=digest(binary), trace_binary_sha256=digest(trace)))
print('All 12 rate-1/4 runs verified; separate instrumentation matched proof sizes and authentication hashes.', flush=True)
