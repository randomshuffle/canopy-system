#!/usr/bin/env python3
"""Reproduce Bolt, WHIR and RS accumulation estimates, measurements and reports.

Estimates and reports run locally. Full timing campaigns require Linux x86_64.
Only Python's standard library is needed. Run without Python's -O flag: the
campaign uses assertions to enforce its measurement and provenance invariants.
"""
import argparse
import fcntl
import hashlib
import json
import math
import os
import platform
import resource
import shutil
import signal
import statistics
import subprocess
import sys
import time
from pathlib import Path

from benchmark_common import (ROOT, digest, meminfo, read_csv, save, snapshot,
                              write_csv, require_writable_campaign_output)
from public_metadata import public_metadata, public_text

CONFIG = ROOT / 'experiments/r7a.json'
OUTPUT = ROOT / 'results/reproduce/campaign'
REFERENCE = ROOT / 'results/r7a'
BINS = ('canopy-system', 'examples/gf256_bolt_black_box',
        'examples/whir_fiat_shamir', 'examples/accumulation_section8',
        'examples/aws_field_check')


def sampling_policy(cfg, name):
    policy = cfg.get('sampling_by_job', {}).get(name, cfg)
    warmups, repetitions = int(policy['warmups']), int(policy['repetitions'])
    if warmups < 0 or repetitions < 1:
        raise ValueError(f'invalid sampling policy for {name}')
    return dict(warmups=warmups, repetitions=repetitions)


def jobs(cfg, out, bins):
    result = []
    for n, etas in cfg['grid'].items():
        for eta in etas:
            for threads in cfg['threads']:
                if n not in cfg.get('grid_by_threads', {}).get(str(threads), cfg['grid']):
                    continue
                if eta not in cfg.get('grid_by_threads', {}).get(str(threads), cfg['grid'])[n]:
                    continue
                for family in ('whir', 'bolt'):
                    name = f't{threads}_{family}_n{n}_eta{eta}'
                    folder = out / 'timings' / name
                    policy = sampling_policy(cfg, name)
                    common = ['--warmups', str(policy['warmups']), '--repetitions', str(policy['repetitions'])]
                    if family == 'whir':
                        command = [str(bins / BINS[0]), 'paper-time', '--log-length', n,
                                   '--inputs', str(eta), *common, '--seed', str(cfg['whir_timing_seed']),
                                   '--canopy-codeword-log-reduction', '1', '--output-dir', str(folder)]
                    else:
                        command = [str(bins / BINS[1]), 'time', '--log', n, '--inputs', str(eta),
                                   *common, '--estimates', str(out / 'estimates' / f'n{n}' / 'bolt'),
                                   '--output', str(folder)]
                    result.append(dict(name=name, family=family, log_input=int(n), input_oracles=eta,
                                       threads=threads, points=3 if family == 'whir' else 2,
                                       output=str(folder), command=command, **policy))
    a = cfg['accumulation']
    accum = []
    for eta in a['ell_cast']:
        for threads in cfg['threads']:
            for scheme in ('canopy', 'merkle-half', 'merkle-quarter'):
                name = f't{threads}_accum_n{a["log_n"]}_eta{eta}_{scheme}'
                folder = out / 'timings' / name
                policy = sampling_policy(cfg, name)
                command = [str(bins / BINS[3]), 'time', '--logs', str(a['log_n']),
                           '--ell-cast', str(eta), '--ell-acc', str(a['ell_acc']),
                           '--log-leaf-width', str(a['log_leaf_width']),
                           '--security-bits', str(a['security_bits']), '--objective', a['objective'],
                           '--schemes', scheme, '--seed', str(a['seed']),
                           '--warmups', str(policy['warmups']), '--repetitions', str(policy['repetitions']),
                           '--output', str(folder / 'timings.csv')]
                accum.append(dict(name=name, family='accumulation', scheme=scheme,
                                  log_input=a['log_n'], input_oracles=eta, threads=threads,
                                  points=1, output=str(folder), command=command, **policy))
    # First obtain a matched small SNARK pilot at both thread counts, then all
    # short accumulation runs, then increase SNARK size. Pilots are real samples.
    return result[:4] + accum + result[4:]


def preparation(cfg, out, bins):
    result = []
    for n, etas in cfg['grid'].items():
        folder = out / 'estimates' / f'n{n}'
        inputs = ','.join(map(str, etas))
        commands = [
            ('whir', [str(bins / BINS[0]), 'paper-estimate', '--logs', n, '--inputs', inputs,
                      '--samples', str(cfg['proof_samples']), '--seed', str(cfg['whir_proof_seed']),
                      '--canopy-codeword-log-reduction', '1', '--output-dir', str(folder / 'whir')]),
            ('whir_fs', [str(bins / BINS[2]), '--input-dir', str(folder / 'whir'),
                         '--output', str(folder / 'whir/fiat_shamir.csv')]),
            ('bolt', [str(bins / BINS[1]), 'estimate', '--logs', n, '--inputs', inputs,
                      '--samples', str(cfg['proof_samples']), '--output', str(folder / 'bolt')]),
        ]
        for family, command in commands:
            result.append((f'estimate_n{n}_{family}', command))
    a = cfg['accumulation']
    result.append(('estimate_accumulation', [str(bins / BINS[3]), 'estimate',
                  '--logs', str(a['log_n']), '--ell-cast', ','.join(map(str, a['ell_cast'])),
                  '--ell-acc', str(a['ell_acc']), '--log-leaf-width', str(a['log_leaf_width']),
                  '--security-bits', str(a['security_bits']), '--objective', a['objective'],
                  '--output-dir', str(out / 'estimates/accumulation')]))
    return result


class Campaign:
    def __init__(self, cfg, out):
        self.cfg, self.out = cfg, out
        self.sources = snapshot()
        self.started = time.time()
        self.completed = []
        self.points = 0

    def status(self, phase, **kw):
        plan = jobs(self.cfg, self.out, Path('.'))
        save(self.out / 'status.json', dict(phase=phase, updated_unix=time.time(),
             started_unix=self.started, completed_timing_jobs=len(self.completed),
             completed_points=self.points, total_points=sum(j['points'] for j in plan),
             total_timing_jobs=len(plan), **kw))

    def run(self, name, command, threads=None, phase='preparation', folder=None, extra_env=None):
        assert snapshot() == self.sources, 'Sources changed during campaign'
        threads = threads or self.cfg['threads'][0]
        folder = folder or self.out / 'preparation' / name
        folder.mkdir(parents=True, exist_ok=True)
        cpus = self.cfg['cpu_affinity'][str(threads)]
        executable = Path(shutil.which(command[0]) or command[0])
        identity = public_metadata(dict(command=command, executable_sha256=digest(executable),
                     source_sha256=self.sources, config=self.cfg, threads=threads, affinity=cpus,
                     extra_environment=extra_env or {}))
        fingerprint = hashlib.sha256(json.dumps(identity, sort_keys=True).encode()).hexdigest()
        record = folder / 'process.json'
        if record.exists():
            old = json.loads(record.read_text())
            assert old['fingerprint'] == fingerprint, f'Changed job identity: {name}'
            assert old['status'] == 'complete', f'Incomplete job {name}; inspect before retrying'
            return
        assert set(cpus) <= os.sched_getaffinity(0)
        limit = self.cfg['memory_limit_gib'] * 2**30
        assert meminfo()['MemAvailable'] > self.cfg['minimum_available_gib'] * 2**30

        def child_setup():
            os.sched_setaffinity(0, cpus)
            resource.setrlimit(resource.RLIMIT_AS, (limit, limit))

        report = dict(identity=identity, fingerprint=fingerprint, status='running',
                      started_unix=time.time(), name=name, rayon_threads=threads, cpu_affinity=cpus)
        save(record, report)
        print(f'{phase}: {name}', flush=True)
        reason, peak, last_update = None, 0, 0
        with (folder / 'run.log').open('w') as log:
            child = subprocess.Popen(command, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT,
                    env={**os.environ, 'RAYON_NUM_THREADS': str(threads), **(extra_env or {})},
                    start_new_session=True, preexec_fn=child_setup)
            report['pid'] = child.pid
            save(record, report)
            try:
                while True:
                    pid, status, usage = os.wait4(child.pid, os.WNOHANG)
                    if pid:
                        child.returncode = os.waitstatus_to_exitcode(status)
                        break
                    memory = meminfo()
                    if memory['MemAvailable'] < self.cfg['minimum_available_gib'] * 2**30:
                        reason = 'memory_pressure'
                    if memory.get('SwapTotal', 0) - memory.get('SwapFree', 0) > 0:
                        reason = 'swap_detected'
                    rss = 0
                    try:
                        info = Path(f'/proc/{child.pid}/status').read_text()
                        rss = next(int(line.split()[1])*1024 for line in info.splitlines() if line.startswith('VmRSS:'))
                        peak = max(peak, rss)
                        if rss > limit:
                            reason = 'memory_limit'
                    except (FileNotFoundError, ProcessLookupError, StopIteration):
                        pass
                    if reason:
                        try:
                            os.killpg(child.pid, signal.SIGKILL)
                        except ProcessLookupError:
                            pass
                    if time.time() - last_update >= 15:
                        self.status(phase, active_job=name, pid=child.pid, threads=threads,
                                    elapsed_seconds=time.time()-report['started_unix'],
                                    rss_gib=rss/2**30, peak_rss_gib=peak/2**30,
                                    available_gib=memory['MemAvailable']/2**30, log=str(folder/'run.log'))
                        last_update = time.time()
                    time.sleep(0.5)
            except BaseException:
                # An interrupted controller must not leave a timing child running.
                try:
                    os.killpg(child.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                _, status, usage = os.wait4(child.pid, 0)
                child.returncode = os.waitstatus_to_exitcode(status)
                report.update(status='interrupted', exit_code=child.returncode,
                              wall_seconds=time.time()-report['started_unix'])
                save(record, report)
                raise
        log_path = folder / 'run.log'
        log_path.write_text(public_text(log_path.read_text(errors='replace')))
        peak = max(peak, usage.ru_maxrss * 1024)
        report.update(status=reason or ('complete' if child.returncode == 0 else 'failed'),
                      exit_code=child.returncode, wall_seconds=time.time()-report['started_unix'],
                      peak_rss_gib=peak/2**30, user_seconds=usage.ru_utime, system_seconds=usage.ru_stime)
        save(record, report)
        assert report['status'] == 'complete', f'{name}: {report["status"]}; inspect {folder}/run.log'
        print(f'Completed {name}: {report["wall_seconds"]:.1f}s, {peak/2**30:.2f} GiB peak', flush=True)


def proof_rows(cfg, out, destination=None):
    result = []
    for n, etas in cfg['grid'].items():
        folder = out / 'estimates' / f'n{n}'
        fs = {(r['input_oracles'], r['scheme']): r for r in read_csv(folder/'whir/fiat_shamir.csv')}
        for r in read_csv(folder/'whir/hybrid_proof_sizes.csv'):
            if r['scheme'] not in cfg['configurations'][:3]:
                continue
            assert float(r['algebraic_rbr_union_bound_bits']) >= 128
            if r['scheme'] == 'canopy_schwartz_zippel':
                assert int(r['aligned_codeword_log']) == int(n)
            f = fs[r['input_oracles'], r['scheme']]
            result.append(dict(log_input=int(n), input_oracles=int(r['input_oracles']), scheme=r['scheme'],
                               proof_bytes=float(r['mean_total_bytes']), verifier_auth_hashes=float(r['verifier_auth_hashes']),
                               fs_finalizations=int(f['fs_finalizations']), verifier_hashes=float(f['verifier_hashes']),
                               timing_scope='component-composed native WHIR'))
        for r in read_csv(folder/'bolt/proof_sizes.csv'):
            result.append(dict(log_input=int(n), input_oracles=int(r['input_oracles']), scheme=r['scheme'],
                               proof_bytes=float(r['total_bytes']), verifier_auth_hashes=float(r['verifier_auth_hashes']),
                               fs_finalizations=int(r['fs_finalizations']), verifier_hashes=float(r['verifier_hashes']),
                               systematic_h=r['systematic_h'], parity_h=r['parity_h'],
                               timing_scope='complete serialized public-H Bolt adapter'))
    expected = {(int(n), e, s) for n, es in cfg['grid'].items() for e in es for s in cfg['configurations']}
    assert len(result) == len(expected)
    assert {(r['log_input'], r['input_oracles'], r['scheme']) for r in result} == expected
    for r in result:
        assert math.isclose(r['verifier_hashes'], r['verifier_auth_hashes'] + r['fs_finalizations'], abs_tol=1e-6)
    # The retained table uses the same WHIR model and the black-box Bolt construction.
    reference = {(int(r['log_input']), int(r['input_oracles']), r['scheme']): r
                 for r in read_csv(REFERENCE / 'proof_sizes_and_hashes.csv')}
    for r in result:
        key = r['log_input'], r['input_oracles'], r['scheme']
        old = reference[key]
        for field in ('proof_bytes', 'verifier_hashes'):
            assert math.isclose(r[field], float(old[field]), rel_tol=1e-10, abs_tol=1e-5), (key, field, r[field], old[field])
    write_csv((destination or out)/'proof_sizes_and_hashes.csv', result)
    return result


def report(cfg, out, bins, destination=None):
    destination = destination or out
    proofs = {(r['log_input'], r['input_oracles'], r['scheme']): r
              for r in proof_rows(cfg, out, destination)}
    snark, accum, components = [], [], []
    estimates = {(int(r['ell_cast']), r['scheme']): r
                 for r in read_csv(out/'estimates/accumulation/estimates.csv')}
    mean = statistics.mean
    for job in jobs(cfg, out, bins):
        folder = Path(job['output'])
        if not (folder/'process.json').exists():
            continue
        process = json.loads((folder/'process.json').read_text())
        if process['status'] != 'complete':
            continue
        assert process['exit_code'] == 0, f'unsuccessful job: {job["name"]}'
        threads, n, eta = job['threads'], job['log_input'], job['input_oracles']
        command = process['identity']['command']
        warmups = int(command[command.index('--warmups') + 1])
        repetitions = int(command[command.index('--repetitions') + 1])
        assert (warmups, repetitions) == (job['warmups'], job['repetitions'])
        if job['family'] == 'accumulation':
            rows = read_csv(folder/'timings.csv')
            names = ['ifold', 'iacc'] if job['scheme'] == 'canopy' else ['merkle_rate_half' if job['scheme'] == 'merkle-half' else 'merkle_rate_quarter']
            assert {r['scheme'] for r in rows} == set(names)
            for name in names:
                selected = [r for r in rows if r['scheme'] == name]
                assert len(selected) == repetitions
                assert {int(r['repetition']) for r in selected} == set(range(repetitions))
                for r in selected:
                    assert r['verified'] == 'true' and r['workload'] == 'steady_state_v2'
                    expected_sources = eta + (1 if name == 'ifold' else 2 if name == 'iacc' else cfg['accumulation']['ell_acc'])
                    assert int(r['sources']) == expected_sources
                    assert int(r['queries']) == int(estimates[eta, name]['query_draws_per_source'])
                    assert int(r['fresh_commit_sources']) == (0 if name == 'iacc' else eta)
                    if name == 'iacc':
                        assert float(r['fresh_commit_seconds']) == 0
                    components.append(dict(threads=threads, **r))
            paired = [[r for r in rows if int(r['repetition']) == i] for i in range(repetitions)]
            for pair in paired:
                assert len({r['input_seed'] for r in pair}) == 1
            accum.append(dict(threads=threads, log_input=n, input_oracles=eta, scheme='canopy_ifold_iacc' if len(names)==2 else names[0],
                warmups=warmups, repetitions=repetitions, mean_commitment_seconds=mean(sum(float(r['fresh_commit_seconds']) for r in p) for p in paired),
                mean_accumulation_seconds=mean(sum(float(r['step_seconds']) for r in p) for p in paired),
                mean_total_prover_seconds=mean(sum(float(r['fresh_commit_seconds'])+float(r['step_seconds']) for r in p) for p in paired),
                proof_bytes=sum(float(estimates[eta, name]['reduction_proof_bytes']) for name in names),
                verifier_hashes=sum(float(estimates[eta, name]['total_verifier_hashes']) for name in names),
                measured_proof_bytes=mean(sum(int(r['proof_bytes']) for r in p) for p in paired),
                measured_verifier_hashes=mean(sum(int(r['auth_hashes'])+int(r['fs_finalizations']) for r in p) for p in paired),
                height=int(estimates[eta, names[0]]['height']), process_peak_rss_gib=process['peak_rss_gib']))
            continue
        filename = 'prover_timing_samples.csv' if job['family'] == 'whir' else 'prover_timings.csv'
        rows = read_csv(folder/filename)
        if job['family'] == 'whir':
            method = json.loads((folder/'timing_method.json').read_text())
            assert method['version'] == 'concurrent-commitments-v2' and method['rayon_threads'] == threads
        names = cfg['configurations'][:3] if job['family'] == 'whir' else cfg['configurations'][3:]
        assert {r['scheme'] for r in rows} == set(names)
        for name in names:
            selected = [r for r in rows if r['scheme'] == name]
            assert len(selected) == repetitions
            if job['family'] == 'bolt':
                assert all(r['proof_verified']=='true' and int(r['rayon_threads'])==threads for r in selected)
            total = 'prover_seconds' if job['family'] == 'whir' else 'total_prover_seconds'
            for r in selected:
                assert math.isclose(float(r[total]), float(r['commitment_seconds']) + float(r['opening_seconds']), rel_tol=1e-7)
            snark.append(dict(**proofs[n, eta, name], threads=threads, warmups=warmups, repetitions=len(selected),
                raw_mean_commitment_seconds=mean(float(r['commitment_seconds']) for r in selected),
                mean_opening_seconds=mean(float(r['opening_seconds']) for r in selected),
                raw_mean_total_prover_seconds=mean(float(r[total]) for r in selected),
                raw_min_total_prover_seconds=min(float(r[total]) for r in selected),
                raw_max_total_prover_seconds=max(float(r[total]) for r in selected),
                process_peak_rss_gib=process['peak_rss_gib']))
    anchors = {(r['threads'], r['log_input'], r['scheme']): r['raw_mean_commitment_seconds'] for r in snark if r['input_oracles']==1}
    for r in snark:
        r['mean_commitment_seconds'] = anchors[r['threads'], r['log_input'], r['scheme']] * r['input_oracles']
        r['mean_total_prover_seconds'] = r['mean_commitment_seconds'] + r['mean_opening_seconds']
        r['commitment_policy'] = 'eta_times_mean_eta1_same_N_scheme_threads'
    for filename, rows in [('snark_comparison.csv', snark), ('accumulation_comparison.csv', accum), ('accumulation_samples.csv', components)]:
        if rows:
            write_csv(destination/filename, rows)
    speedups = []
    for family, rows in [('snark', snark), ('accumulation', accum)]:
        serial = {(r['log_input'],r['input_oracles'],r['scheme']):r for r in rows if r['threads']==1}
        for r in rows:
            key = r['log_input'], r['input_oracles'], r['scheme']
            if r['threads'] != 32 or key not in serial:
                continue
            one = serial[key]
            step = 'mean_opening_seconds' if family=='snark' else 'mean_accumulation_seconds'
            speedups.append(dict(family=family, log_input=key[0], input_oracles=key[1], scheme=key[2],
                commitment_speedup=one['mean_commitment_seconds']/r['mean_commitment_seconds'],
                opening_or_accumulation_speedup=one[step]/r[step],
                total_speedup=one['mean_total_prover_seconds']/r['mean_total_prover_seconds']))
    if speedups:
        write_csv(destination/'thread_speedups.csv', speedups)
    return len(snark) + len(accum)


def execute(cfg, out, bins):
    assert sys.platform == 'linux' and platform.machine() == 'x86_64'
    assert meminfo()['SwapTotal'] == 0
    c = Campaign(cfg, out)
    plan = jobs(cfg, out, bins)
    expected_points = sum(j['points'] for j in plan)
    assert plan, 'empty campaign'
    save(out/'plan.json', dict(config=cfg, jobs=plan, source_sha256=c.sources))
    host = dict(platform=platform.platform(), lscpu=subprocess.check_output(['lscpu'],text=True),
                numa=subprocess.check_output(['numactl','--hardware'],text=True), memory=meminfo(),
                cpu_affinity=sorted(os.sched_getaffinity(0)), configuration=cfg)
    if not (out/'host.json').exists():
        save(out/'host.json', host)
    cargo = 'cargo'
    build = [cargo, f'+{cfg["rust_toolchain"]}', 'build', '--release', '--locked', '--target-dir', str(bins.parent), '--bin', BINS[0]]
    for b in BINS[1:]:
        build += ['--example', Path(b).name]
    env = {'RUSTFLAGS':'-C target-cpu=native', 'CARGO_BUILD_JOBS':str(max(cfg['threads']))}
    try:
        c.run('native_build', build, phase='build', extra_env=env)
        c.run('native_tests', [cargo, f'+{cfg["rust_toolchain"]}', 'test', '--release', '--locked', '--target-dir', str(bins.parent), '--lib', '--', '--test-threads=1'], phase='validation', extra_env=env)
        c.run('fs_model_test', [cargo, f'+{cfg["rust_toolchain"]}', 'test', '--release', '--locked', '--target-dir', str(bins.parent.with_name(bins.parent.name + '-accounting')), '--features', 'fs-accounting', '--lib', 'black_box_fs_model_matches_verifier', '--', '--nocapture'], phase='validation', extra_env=env)
        for threads in cfg['threads']:
            c.run(f'field_backend_t{threads}', [str(bins/BINS[4]), '--require-pclmul'], threads=threads, phase='validation')
        receipt = dict(source_sha256=c.sources, binaries={b:digest(bins/b) for b in BINS},
                       rust_toolchain=cfg['rust_toolchain'], rustflags=env['RUSTFLAGS'], features=[],
                       validation='all native library tests; instrumented black-box verifier trace; 16384 field basis products at both thread counts')
        save(out/'native-build.json', receipt)
        for name, command in preparation(cfg, out, bins):
            c.run(name, command)
        proof_rows(cfg, out)
        for job in plan:
            assert receipt['binaries'] == {b:digest(bins/b) for b in BINS}
            c.run(job['name'], job['command'], threads=job['threads'], phase='timing', folder=Path(job['output']))
            c.completed.append(job['name'])
            c.points = report(cfg, out, bins)
            c.status('timing', last_completed_job=job['name'])
        assert c.points == expected_points
        save(out/'checksums.json', {str(p.relative_to(out)): digest(p) for p in sorted(out.rglob('*'))
                                   if p.is_file() and p.name not in ('checksums.json','status.json','campaign.lock')})
        c.status('complete', elapsed_seconds=time.time()-c.started)
    except BaseException as error:
        c.status('failed', error=str(error))
        raise


def compare_table(actual_path, reference_path, keys):
    """Compare every exported field, tolerating only floating-point roundoff."""
    def indexed(path):
        rows = read_csv(path)
        result = {tuple(row[k] for k in keys): row for row in rows}
        if len(result) != len(rows):
            raise ValueError(f'duplicate rows in {path}')
        return result
    actual, reference = indexed(actual_path), indexed(reference_path)
    if actual.keys() != reference.keys():
        raise ValueError(f'configuration keys differ: {actual_path}')
    for key, row in actual.items():
        expected = reference[key]
        if row.keys() != expected.keys():
            raise ValueError(f'columns differ: {actual_path}')
        for field, value in row.items():
            if value == expected[field]:
                continue
            try:
                equal = math.isclose(float(value), float(expected[field]), rel_tol=1e-10, abs_tol=1e-6)
            except ValueError:
                equal = False
            if not equal:
                raise ValueError(f'{actual_path}: {key}, {field}: {value} != {expected[field]}')
    return len(actual)


def require_compiler(cfg):
    version = subprocess.check_output(['rustc', f'+{cfg["rust_toolchain"]}', '--version'], text=True).strip()
    if version.split()[1] != '1.92.0':
        raise ValueError(f'the reference toolchain is Rust 1.92.0; found {version}')
    return version


def estimate(cfg, out, bins, threads, skip_build=False):
    if out.exists() and any(out.iterdir()):
        raise ValueError(f'estimates need a new or empty output directory: {out}')
    out.mkdir(parents=True, exist_ok=True)
    env = {**os.environ, 'RAYON_NUM_THREADS': str(threads), 'CARGO_BUILD_JOBS': str(threads)}
    receipt = dict(status='running', config=cfg, source_sha256=snapshot(),
                   threads=threads, build_skipped=skip_build, commands=[])
    save(out/'provenance.json', receipt)
    try:
        if not skip_build:
            receipt['rustc'] = require_compiler(cfg)
            command = ['cargo', f'+{cfg["rust_toolchain"]}', 'build', '--locked', '--release',
                       '--target-dir', str(bins.parent), '--bin', BINS[0]]
            for binary in BINS[1:]:
                command += ['--example', Path(binary).name]
            receipt['build_command'] = command
            print('Building release estimators...', flush=True)
            subprocess.run(command, cwd=ROOT, env=env, check=True)
        receipt['binaries'] = {name: digest(bins/name) for name in BINS}
        for name, command in preparation(cfg, out, bins):
            print(f'Estimating {name}...', flush=True)
            log = out/'logs'/f'{name}.log'
            log.parent.mkdir(exist_ok=True)
            start = time.monotonic()
            with log.open('w') as stream:
                result = subprocess.run(command, cwd=ROOT, env=env, stdout=stream, stderr=subprocess.STDOUT)
            log.write_text(public_text(log.read_text()))
            receipt['commands'].append(dict(name=name, command=command, exit_code=result.returncode,
                                            seconds=time.monotonic()-start))
            save(out/'provenance.json', receipt)
            if result.returncode:
                raise RuntimeError(f'{name} failed; see {log}')
        proof_rows(cfg, out)
        snark_count = compare_table(out/'proof_sizes_and_hashes.csv', REFERENCE/'proof_sizes_and_hashes.csv',
                                   ('log_input', 'input_oracles', 'scheme'))
        accum_count = compare_table(out/'estimates/accumulation/estimates.csv',
                                   REFERENCE/'estimates/accumulation/estimates.csv',
                                   ('log_n', 'ell_cast', 'ell_acc', 'scheme'))
        compare_table(out/'estimates/accumulation/height_search.csv',
                      REFERENCE/'estimates/accumulation/height_search.csv',
                      ('log_n', 'ell_cast', 'ell_acc', 'height', 'scheme'))
        receipt.update(status='complete', snark_rows=snark_count, accumulation_rows=accum_count,
                       matches_reference=True, timings_rerun=False)
        save(out/'validation.json', dict(snark_rows=snark_count, accumulation_rows=accum_count,
                                       matches_reference=True, timings_rerun=False))
        print(f'All {snark_count} Bolt/WHIR rows and {accum_count} accumulation reduction rows match.', flush=True)
    except BaseException as error:
        receipt.update(status='failed', error=str(error))
        raise
    finally:
        save(out/'provenance.json', receipt)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('plan', 'estimates', 'run', 'report'))
    parser.add_argument('--config', type=Path, default=CONFIG)
    parser.add_argument('--output', type=Path, help='fresh output directory; reference data are protected')
    parser.add_argument('--input', type=Path, default=REFERENCE, help='raw campaign directory for report')
    parser.add_argument('--bin-dir', type=Path, help='release binary directory')
    parser.add_argument('--toolchain', help='Rustup alias for Rust 1.92.0 (default: pinned version)')
    parser.add_argument('--offline', action='store_true', help='use cached Cargo dependencies')
    parser.add_argument('--threads', type=int, default=min(8, os.cpu_count() or 1),
                        help='local estimator workers; timing counts come from the config')
    parser.add_argument('--skip-build', action='store_true', help='estimates only: reuse existing binaries')
    args = parser.parse_args()
    if not __debug__:
        parser.error('run without Python -O so validation assertions remain enabled')
    if args.threads < 1:
        parser.error('--threads must be positive')
    if args.skip_build and args.action != 'estimates':
        parser.error('--skip-build applies only to estimates')
    cfg = json.loads(args.config.read_text())
    if args.toolchain:
        cfg['rust_toolchain'] = args.toolchain
    if args.offline:
        os.environ['CARGO_NET_OFFLINE'] = 'true'
    default_output = OUTPUT if args.action in ('plan', 'run') else ROOT/'results/reproduce'/args.action
    out = (args.output or default_output).resolve()
    native = ROOT/'target/reproduce-native/release'
    bins = (args.bin_dir or (ROOT/'target/release' if args.action == 'estimates' else native)).resolve()
    try:
        require_writable_campaign_output(out)
        if bins.name != 'release' and not args.skip_build:
            raise ValueError('--bin-dir must end in release when building')
        if args.action == 'estimates':
            estimate(cfg, out, bins, args.threads, args.skip_build)
            return
        if args.action == 'report':
            if out.exists() and any(out.iterdir()):
                raise ValueError('reports need a new or empty output directory')
            count = report(cfg, args.input.resolve(), bins, out)
            print(f'Reported {count} verified method points in {public_text(str(out))}')
            return
        plan = jobs(cfg, out, bins)
        if args.action == 'plan':
            save(out/'plan.json', dict(config=cfg, jobs=plan, source_sha256=snapshot()))
            print(f'{len(plan)} sequential timing jobs; {sum(j["points"] for j in plan)} method points')
            return
        if sys.platform != 'linux' or platform.machine() != 'x86_64':
            raise ValueError('the full native campaign requires Linux x86_64; use estimates or individual local pilots')
        require_compiler(cfg)
        out.mkdir(parents=True, exist_ok=True)
        with (out/'campaign.lock').open('a') as lock:
            fcntl.flock(lock.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
            execute(cfg, out, bins)
    except (ValueError, FileNotFoundError) as error:
        parser.error(str(error))


if __name__ == '__main__':
    main()
