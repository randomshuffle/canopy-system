#!/usr/bin/env python3
"""Add N=2^18 using the completed campaign's identical benchmark binaries."""
import csv
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess
import time

ROOT = Path(__file__).resolve().parent
OUT = ROOT / 'measurements'
CPUS = list(range(16)) + list(range(48, 64))
os.sched_setaffinity(0, CPUS)
ENV = dict(os.environ, RAYON_NUM_THREADS='32')
completion = OUT / 'complete.json'
deadline = time.monotonic() + 1800
while not completion.exists():
    if time.monotonic() > deadline:
        raise RuntimeError('Main campaign did not complete within 30 minutes')
    time.sleep(5)
complete = json.loads(completion.read_text())
assert complete['status'] == 'complete'


def run(name, command, expected_hash):
    digest = hashlib.sha256(Path(command[0]).read_bytes()).hexdigest()
    assert digest == expected_hash
    print(f'{name}: {command}', flush=True)
    start = time.time()
    record = dict(command=command, started_unix=start, status='running',
                  threads=32, cpu_affinity=CPUS, binary_sha256=digest)
    record_path = OUT / (name + '.json')
    record_path.write_text(json.dumps(record, indent=2) + '\n')
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
    record_path.write_text(json.dumps(record, indent=2) + '\n')
    assert process.returncode == 0, (name, process.returncode)
    print(f'{name}: completed in {record["wall_seconds"]:.2f}s', flush=True)


run('timing_n18', [str(OUT / 'bin/accumulation_one_fold'), '--logs', '18',
                   '--warmups', '1', '--repetitions', '3', '--seed', '91',
                   '--vary-inputs', '--output', str(OUT / 'timing_samples_n18.csv')],
    complete['timing_binary_sha256'])
run('trace_n18', [str(OUT / 'bin/accumulation_one_fold_fs'), '--logs', '18',
                  '--warmups', '0', '--repetitions', '3', '--seed', '92',
                  '--vary-inputs', '--output', str(OUT / 'hash_validation_n18.csv')],
    complete['trace_binary_sha256'])
with (OUT / 'timing_samples_n18.csv').open() as f:
    timings = list(csv.DictReader(f))
with (OUT / 'hash_validation_n18.csv').open() as f:
    traces = list(csv.DictReader(f))
assert len(timings) == len(traces) == 6
key = lambda r: (r['log_n'], r['scheme'], r['input_seed'])
traces = {key(r): r for r in traces}
for row in timings:
    other = traces[key(row)]
    for field in ('verified', 'actual_auth_hashes', 'proof_bytes', 'queries_0', 'queries_1'):
        assert row[field] == other[field], (key(row), field)
    assert row['verified'] == 'true'
    assert int(other['fs_finalizations']) == 18 + (4 if row['scheme'] == 'merkle_same_code_batch' else 9)
(OUT / 'complete_n18.json').write_text(json.dumps(dict(status='complete',
    verified_measured_runs=6, input_seeds=[92, 93, 94], repetitions=3,
    warmups_per_scheme_and_size=1), indent=2) + '\n')
print('All six N=2^18 measured runs verified; instrumentation matched.', flush=True)
