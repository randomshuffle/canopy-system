#!/usr/bin/env python3
"""Prepare, execute, resume and summarize the fixed five-configuration AWS campaign.

Standard library only. No SSH, instance provisioning, or credentials are needed
for preparation. Execution is explicit and only supported on the target Linux host.
"""
import argparse
import csv
import gzip
import hashlib
import json
import os
import platform
import resource
import shlex
import signal
import statistics
import subprocess
import sys
import tarfile
import time
from pathlib import Path
from public_metadata import public_metadata, public_tar_member, public_text

ROOT = Path(__file__).resolve().parents[1]
CONFIG = ROOT / 'experiments/aws-r5/config.json'
SOURCE_DIRS = ('src', 'examples', 'scripts', 'vendor', 'experiments')
BINS = ('canopy-system', 'examples/gf256_bolt_suite', 'examples/gf256_bolt_split',
        'examples/whir_fiat_shamir', 'examples/aws_field_check')


def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b''):
            h.update(chunk)
    return h.hexdigest()


def require_writable_campaign_output(out):
    """Published evidence and archived runs are inputs, never rerun destinations."""
    out = out.resolve()
    published = [ROOT / 'results' / name for name in (
        'aws_i4i_optimized', 'aws_i4i_n30_eta1',
        'aws_i4i_bolt_n22_recheck_20260917',
        'aws_i4i_bolt_n24_recheck_20260917', 'paper_bcgm_m10')]
    protected = [ROOT / 'archive', *published]
    if out in (ROOT, ROOT / 'results') or any(
            out == path or path in out.parents for path in protected):
        raise ValueError(f'{out}: retained evidence; select a fresh output directory')


def source_files():
    files = [ROOT / x for x in ('Cargo.toml', 'Cargo.lock')]
    for directory in SOURCE_DIRS:
        files.extend(p for p in (ROOT / directory).rglob('*') if p.is_file()
                     and not p.is_symlink() and p.suffix in ('.rs', '.py', '.toml', '.json')
                     and not any(x in p.parts for x in ('.git', 'target', '__pycache__')))
    return sorted(set(files))


def snapshot():
    return {str(p.relative_to(ROOT)): digest(p) for p in source_files()}


def save(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + '.tmp')
    tmp.write_text(json.dumps(public_metadata(data), indent=2) + '\n')
    tmp.replace(path)


def read_csv(path):
    with path.open() as f:
        return list(csv.DictReader(f))


def write_csv(path, rows):
    if not rows:
        raise ValueError(f'no rows for {path}')
    fields = list(dict.fromkeys(k for row in rows for k in row))
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open('w', newline='') as f:
        w = csv.DictWriter(f, fieldnames=fields)
        w.writeheader()
        w.writerows(rows)


def preparation_commands(cfg, bins, out):
    commands = []
    for n, etas in cfg['grid'].items():
        inputs = ','.join(map(str, etas))
        folder = out / 'estimates' / f'n{n}'
        w, b, c = (folder / x for x in ('whir', 'shared', 'quarter'))
        commands += [
            [str(bins / BINS[0]), 'paper-estimate', '--logs', n, '--inputs', inputs,
             '--samples', str(cfg['proof_samples']), '--seed', str(cfg['whir_proof_seed']),
             '--canopy-codeword-log-reduction', '1', '--output-dir', str(w)],
            [str(bins / BINS[3]), '--input-dir', str(w), '--output', str(w / 'fiat_shamir.csv')],
            [str(bins / BINS[1]), 'estimate', '--logs', n, '--inputs', inputs,
             '--samples', str(cfg['proof_samples']), '--width', '4', '--output', str(b)],
            [str(bins / BINS[1]), 'account', '--input', str(b / 'proof_sizes.csv'),
             '--output', str(b / 'fiat_shamir.csv')],
            [str(bins / BINS[2]), 'estimate', '--logs', n, '--inputs', inputs,
             '--samples', str(cfg['proof_samples']), '--width', '4', '--block-divisor', '4',
             '--output', str(c)],
        ]
    return commands


def timing_jobs(cfg, bins, out, only_log=None):
    jobs = []
    for n, etas in cfg['grid'].items():
        if only_log is not None and int(n) != only_log:
            continue
        for eta in etas:
            estimates = out / 'estimates' / f'n{n}'
            common = ['--warmups', str(cfg['warmups']), '--repetitions', str(cfg['repetitions'])]
            for family in ('whir', 'bolt'):
                name = f'{family}_n{n}_eta{eta}'
                dest = out / 'timings' / name
                if family == 'whir':
                    cmd = [str(bins / BINS[0]), 'paper-time', '--log-length', n, '--inputs', str(eta),
                           *common, '--seed', str(cfg['whir_timing_seed']),
                           '--canopy-codeword-log-reduction', '1', '--output-dir', str(dest)]
                else:
                    cmd = [str(bins / BINS[2]), 'time', '--log', n, '--inputs', str(eta), *common,
                           '--estimates', str(estimates / 'quarter'), '--shared-estimates', str(estimates / 'shared'),
                           '--paper-only', '--shared-merkle-baseline', '--width', '4', '--output', str(dest)]
                jobs.append(dict(name=name, log_input=int(n), input_oracles=eta, family=family,
                                 command=cmd, output=str(dest)))
    return jobs


def meminfo():
    if not Path('/proc/meminfo').exists():
        return {}
    return {line.split(':')[0]: int(line.split()[1]) * 1024
            for line in Path('/proc/meminfo').read_text().splitlines()}


def apply_cpu_affinity(cfg):
    """Restrict this process and every build/test/benchmark child to our CPUs."""
    if sys.platform == 'linux' and cfg.get('cpu_affinity') is not None:
        requested = set(cfg['cpu_affinity'])
        assert len(requested) == cfg['threads'], 'CPU allocation must match worker count'
        assert requested <= os.sched_getaffinity(0), 'Requested CPUs are outside the allowed set'
        os.sched_setaffinity(0, requested)
        assert os.sched_getaffinity(0) == requested


def launch(command, folder, cfg, resume=True):
    """One sequential process; checkpoint identity and peak RSS, stop on pressure."""
    folder.mkdir(parents=True, exist_ok=True)
    identity = public_metadata(dict(command=command, binary_sha256=digest(Path(command[0])),
                    source_sha256=snapshot(), configuration=cfg))
    fingerprint = hashlib.sha256(json.dumps(identity, sort_keys=True).encode()).hexdigest()
    record = folder / 'process.json'
    if record.exists():
        old = json.loads(record.read_text())
        if old['fingerprint'] != fingerprint:
            raise RuntimeError(f'{folder}: existing run has different source, binary or settings; use a new output directory')
        if old['status'] == 'complete' and resume:
            print(f'Already complete: {folder}', flush=True)
            return
        raise RuntimeError(f'{folder}: previous run incomplete; retain it and use a new output directory')
    report = dict(**identity, fingerprint=fingerprint, status='running', started_unix=time.time(),
                  cpu_affinity=sorted(os.sched_getaffinity(0)) if sys.platform == 'linux' else None)
    save(record, report)
    limit = int(cfg['memory_limit_gib'] * 2**30)
    linux = sys.platform == 'linux'
    def limits():
        if linux:
            resource.setrlimit(resource.RLIMIT_AS, (limit, limit))
    reason = None
    print(public_text(shlex.join(command)), flush=True)
    with (folder / 'run.log').open('w') as log:
        child = subprocess.Popen(command, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT,
                                 env={**os.environ, 'RAYON_NUM_THREADS': str(cfg['threads'])},
                                 start_new_session=True, preexec_fn=limits)
        peak = 0
        try:
            while True:
                pid, status, usage = os.wait4(child.pid, os.WNOHANG)
                if pid:
                    child.returncode = os.waitstatus_to_exitcode(status)
                    break
                if linux:
                    info = meminfo()
                    if info.get('MemAvailable', limit) < cfg['minimum_available_gib'] * 2**30:
                        reason = 'memory_pressure'
                    if info.get('SwapTotal', 0) - info.get('SwapFree', 0) > 0:
                        reason = 'swap_detected'
                    try:
                        status_text = Path(f'/proc/{child.pid}/status').read_text()
                        rss = next(int(line.split()[1])*1024 for line in status_text.splitlines() if line.startswith('VmRSS:'))
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
                time.sleep(0.5)
        except KeyboardInterrupt:
            reason = 'interrupted'
            try:
                os.killpg(child.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            _, status, usage = os.wait4(child.pid, 0)
            child.returncode = os.waitstatus_to_exitcode(status)
    log_path = folder / 'run.log'
    log_path.write_text(public_text(log_path.read_text()))
    peak = max(peak, usage.ru_maxrss * (1 if sys.platform == 'darwin' else 1024))
    report.update(status=reason or ('complete' if child.returncode == 0 else 'failed'),
                  exit_code=child.returncode, wall_seconds=time.time()-report['started_unix'],
                  peak_rss_gib=peak/2**30, user_seconds=usage.ru_utime, system_seconds=usage.ru_stime)
    save(record, report)
    if report['status'] != 'complete':
        raise RuntimeError(f"Stopped: {folder}: {report['status']}; see run.log. No timing is accepted for this job.")
    print(f"Completed in {report['wall_seconds']:.1f}s; peak RSS {peak/2**30:.3f} GiB", flush=True)


def proof_report(cfg, out):
    rows = []
    for n, etas in cfg['grid'].items():
        folder = out / 'estimates' / f'n{n}'
        whir_fs = {(r['input_oracles'], r['scheme']): r for r in read_csv(folder / 'whir/fiat_shamir.csv')}
        for r in read_csv(folder / 'whir/hybrid_proof_sizes.csv'):
            if r['scheme'] not in cfg['configurations'][:3]:
                continue
            fs = whir_fs[r['input_oracles'], r['scheme']]
            if r['scheme'] == 'canopy_schwartz_zippel':
                assert int(r['aligned_codeword_log']) == int(n)
                assert int(r['canopy_codeword_log_reduction']) == 1
            assert float(r['algebraic_rbr_union_bound_bits']) >= 128
            rows.append(dict(log_input=n, input_oracles=r['input_oracles'], scheme=r['scheme'],
                             proof_bytes=r['mean_total_bytes'], verifier_auth_hashes=r['verifier_auth_hashes'],
                             verifier_hash_to_field=r['verifier_hash_to_field'],
                             **{k: fs[k] for k in ('verifier_hashes','fs_finalizations','fs_absorb_bytes','fs_squeeze_bytes','fs_proof_bytes')},
                             proof_samples=cfg['proof_samples'], timing_scope='component-composed'))
        shared_fs = {(r['input_oracles'],r['scheme'],r['layout']): r for r in read_csv(folder / 'shared/fiat_shamir.csv')}
        shared = [r for r in read_csv(folder / 'shared/proof_sizes.csv') if not r['systematic_h']]
        assert len(shared) == len(etas)
        for r in shared:
            fs = shared_fs[r['input_oracles'],r['scheme'],r['layout']]
            rows.append(dict(log_input=n, input_oracles=r['input_oracles'], scheme='bolt_merkle_shared',
                             proof_bytes=r['total_bytes'], verifier_auth_hashes=r['verifier_auth_hashes'],
                             verifier_hash_to_field=r['verifier_hash_to_field'],
                             **{k: fs[k] for k in ('verifier_hashes','fs_finalizations','fs_absorb_bytes','fs_squeeze_bytes','fs_proof_bytes')},
                             proof_samples=cfg['proof_samples'], timing_scope='serialized public-H adapter'))
        for r in read_csv(folder / 'quarter/proof_sizes.csv'):
            if r['scheme'] != 'bolt_canopy_split_quarter':
                continue
            assert r['delayed_systematic'] == 'false'
            assert int(r['systematic_rs_length']) == int(r['parity_rs_length']) == 2**int(n)//16
            rows.append(dict(log_input=n, input_oracles=r['input_oracles'], scheme=r['scheme'],
                             proof_bytes=r['total_bytes'], systematic_h=r['systematic_h'], parity_h=r['parity_h'],
                             **{k:r[k] for k in ('verifier_hashes','verifier_auth_hashes','verifier_hash_to_field',
                                                'fs_finalizations','fs_absorb_bytes','fs_squeeze_bytes','fs_proof_bytes')},
                             proof_samples=cfg['proof_samples'], timing_scope='serialized public-H adapter'))
    expected = {(str(n), str(eta), scheme) for n, etas in cfg['grid'].items() for eta in etas for scheme in cfg['configurations']}
    assert len(rows) == len(expected)
    assert {(r['log_input'], r['input_oracles'], r['scheme']) for r in rows} == expected
    for r in rows:
        assert abs(float(r['verifier_hashes']) - sum(float(r[k]) for k in ('verifier_auth_hashes','verifier_hash_to_field','fs_finalizations'))) < 1e-6
        r['proof_kib'] = float(r['proof_bytes']) / 1024
    write_csv(out / 'proof_sizes_and_hashes.csv', rows)
    return rows


def timing_report(cfg, bins, out):
    proofs = proof_report(cfg, out)
    summaries = {}
    for job in timing_jobs(cfg, bins, out):
        folder = Path(job['output'])
        record = folder / 'process.json'
        if not record.exists() or json.loads(record.read_text())['status'] != 'complete':
            continue
        process = json.loads(record.read_text())
        filename = 'prover_timing_samples.csv' if job['family'] == 'whir' else 'prover_timings.csv'
        groups = {}
        for row in read_csv(folder / filename):
            groups.setdefault(row['scheme'], []).append(row)
        assert set(groups) == set(cfg['configurations'][:3] if job['family']=='whir' else cfg['configurations'][3:])
        if job['family'] == 'whir':
            method = json.loads((folder / 'timing_method.json').read_text())
            assert method['version'] == 'concurrent-commitments-v2', 'Stale WHIR timing method'
            assert method['rayon_threads'] == cfg['threads']
            stages = {(r['scheme'],r['stage']):float(r['mean_seconds'])
                      for r in read_csv(folder / 'prover_timing_stages.csv')}
            half, _, canopy = cfg['configurations'][:3]
            assert stages[half,'initial_rs_encoding'] == stages[canopy,'outer_rs_encoding'], 'Base RS sample must be identical'
            for scheme, samples in groups.items():
                names = ('outer_rs_encoding','canopy_bottom_forest','canopy_hash_to_field',
                         'canopy_rs_encoding','canopy_codeword_merkle_tree','canopy_commitment_other') if scheme == canopy else ('initial_rs_encoding','initial_merkle_tree')
                measured = statistics.mean(float(r['commitment_seconds']) for r in samples)
                assert abs(measured-sum(stages[scheme,name] for name in names)) < 1e-7 * max(1, measured), 'Commitment stage mismatch'
                assert all(abs(float(r['prover_seconds'])-float(r['commitment_seconds'])-float(r['opening_seconds'])) < 1e-7 * max(1,float(r['prover_seconds'])) for r in samples), 'Prover phase mismatch'
        for scheme, samples in groups.items():
            assert len(samples) == cfg['repetitions']
            if job['family'] == 'bolt':
                assert all(r['proof_verified'] == 'true' for r in samples)
                assert all(int(r['rayon_threads']) == cfg['threads'] for r in samples)
            total_key = 'prover_seconds' if job['family'] == 'whir' else 'total_prover_seconds'
            totals = [float(r[total_key]) for r in samples]
            summary = dict(timing_status='measured', repetitions=len(samples),
                           median_prover_seconds=statistics.median(totals), mean_prover_seconds=statistics.mean(totals),
                           min_prover_seconds=min(totals), max_prover_seconds=max(totals),
                           mean_commitment_seconds=statistics.mean(float(r['commitment_seconds']) for r in samples),
                           mean_opening_seconds=statistics.mean(float(r['opening_seconds']) for r in samples),
                           process_peak_rss_gib=process['peak_rss_gib'], process_wall_seconds=process['wall_seconds'])
            if job['family'] == 'bolt':
                summary['mean_serialized_proof_bytes'] = statistics.mean(int(r['proof_bytes']) for r in samples)
            summaries[str(job['log_input']), str(job['input_oracles']), scheme] = summary
    for row in proofs:
        row.update(summaries.get((row['log_input'],row['input_oracles'],row['scheme']), {'timing_status':'not_measured'}))
    write_csv(out / 'comparison.csv', proofs)
    subprocess.run([sys.executable, str(ROOT / 'scripts/report_iop_round_commitments.py'),
                    '--input', str(out / 'comparison.csv'),
                    '--output', str(out / 'comparison_iop_rounds.csv')], check=True)
    print(f'{len(summaries)}/{len(proofs)} points have complete timing samples')


def preflight(cfg, bins, out):
    assert sys.platform == 'linux' and platform.machine() == 'x86_64', 'Run AWS timings on Linux x86_64'
    assert len(os.sched_getaffinity(0)) >= cfg['threads'], 'Fewer usable CPUs than requested workers'
    if cfg.get('cpu_affinity') is not None:
        assert os.sched_getaffinity(0) == set(cfg['cpu_affinity']), 'CPU allocation mismatch'
    memory = meminfo()
    assert memory['SwapTotal'] == 0, 'Disable swap before this in-memory campaign'
    assert memory['MemAvailable'] > cfg['memory_limit_gib'] * 2**30, f"Host needs more than {cfg['memory_limit_gib']} GiB available at start"
    backend = json.loads(subprocess.check_output([str(bins / BINS[4]), '--require-intel'], text=True))
    receipt = json.loads((bins.parent / 'native-build.json').read_text())
    assert receipt['source_sha256'] == snapshot(), 'Sources changed since the native build'
    assert receipt['binaries'] == {b:digest(bins/b) for b in BINS}, 'Binary changed since the native build'
    assert receipt['rust_toolchain'] == cfg['rust_toolchain']
    assert receipt['features'] == [] and receipt['rustflags'] == '-C target-cpu=native'
    data = dict(platform=platform.platform(), cpu_affinity=sorted(os.sched_getaffinity(0)),
                lscpu=subprocess.check_output(['lscpu'], text=True), memory=memory, backend=backend,
                rustc=subprocess.check_output(['rustc', f"+{cfg['rust_toolchain']}", '-vV'], text=True),
                binaries={b:digest(bins/b) for b in BINS}, source_sha256=snapshot(), configuration=cfg)
    path = out / 'host.json'
    if path.exists():
        previous = json.loads(path.read_text())
        for key in ('binaries','source_sha256','configuration','cpu_affinity'):
            assert previous[key] == data[key], f'Campaign provenance changed: {key}; use a fresh output directory'
    else:
        save(path, data)


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('action', choices=('plan','build','prepare','run','report','bundle'))
    p.add_argument('--config', type=Path, default=CONFIG)
    p.add_argument('--output', type=Path, default=ROOT/'results/aws_r5')
    p.add_argument('--bin-dir', type=Path, default=ROOT/'target/aws-native/release')
    p.add_argument('--only-log', type=int, help='Pilot or continue at just one requested N')
    p.add_argument('--family', choices=('whir','bolt'), help='Rerun one family while preserving separately sourced results for the other')
    p.add_argument('--offline', action='store_true', help='Cargo uses cached dependencies only')
    a = p.parse_args()
    cfg = json.loads(a.config.read_text())
    out, bins = a.output.resolve(), a.bin_dir.resolve()
    try:
        require_writable_campaign_output(out)
    except ValueError as error:
        p.error(str(error))
    if a.action in ('build', 'prepare', 'run'):
        apply_cpu_affinity(cfg)
    out.mkdir(parents=True, exist_ok=True)
    if a.action == 'plan':
        jobs = timing_jobs(cfg, bins, out)
        save(out/'plan.json', dict(configuration=cfg, points=5*sum(map(len,cfg['grid'].values())),
             source_sha256=snapshot(), preparation_commands=preparation_commands(cfg,bins,out), timing_jobs=jobs))
        print(f'Prepared {len(jobs)} sequential jobs, {5*sum(map(len,cfg["grid"].values()))} configuration points')
    elif a.action == 'build':
        flags = ['--locked'] + (['--offline'] if a.offline else [])
        env = {**os.environ, 'RUSTFLAGS':'-C target-cpu=native',
               'RAYON_NUM_THREADS':str(cfg['threads']), 'CARGO_BUILD_JOBS':str(cfg['threads'])}
        cargo = ['cargo', f"+{cfg['rust_toolchain']}"]
        target = ['--target-dir', str(bins.parent)]
        subprocess.run(cargo+['build','--release',*flags,*target,'--bin','canopy-system',
                       '--example','gf256_bolt_suite','--example','gf256_bolt_split',
                       '--example','whir_fiat_shamir','--example','aws_field_check'],cwd=ROOT,env=env,check=True)
        for test in ('binary_field::tests', 'additive_rs::tests', 'bolt::tests',
                     'executable_whir::tests', 'timing_commitments_share_encoding_and_reconcile',
                     'digest_injection_is_inverted_by_field_serialization',
                     'canopy_verifier_work_excludes_digest_conversion_hashes',
                     'quarter_blocks_bind_geometry_and_claims',
                     'parallel_arithmetic_preserves_batched_proof_transcript'):
            subprocess.run(cargo+['test','--release',*flags,*target,'--lib',test],cwd=ROOT,env=env,check=True)
        subprocess.run([str(bins/BINS[4])]+(['--require-intel'] if sys.platform=='linux' else []),check=True)
        save(bins.parent / 'native-build.json', dict(source_sha256=snapshot(),
             binaries={b:digest(bins/b) for b in BINS}, rust_toolchain=cfg['rust_toolchain'],
             features=[], rustflags=env['RUSTFLAGS'],
             validation='field, additive FFT, Bolt contractions, quarter-block binding, parallel transcript equivalence, WHIR concurrent roots and timing reconciliation tests passed'))
    elif a.action == 'prepare':
        for i, cmd in enumerate(preparation_commands(cfg,bins,out)):
            launch(cmd, out/'preparation'/f'{i:02}',cfg)
        proof_report(cfg,out)
    elif a.action == 'run':
        preflight(cfg,bins,out)
        proof_report(cfg,out)
        for job in timing_jobs(cfg,bins,out,a.only_log):
            if a.family is not None and job['family'] != a.family:
                continue
            launch(job['command'],Path(job['output']),cfg)
            timing_report(cfg,bins,out)
    elif a.action == 'report':
        timing_report(cfg,bins,out)
    elif a.action == 'bundle':
        path = out/'canopy-aws-source.tar.gz'
        files = source_files()+[ROOT/'README.md',ROOT/'LICENSE',ROOT/'experiments/aws-r5/README.md',
                              ROOT/'docs/experiment_bolt_max.md', ROOT/'docs/anonymization.md',
                              ROOT/'results/README.md', ROOT/'results/paper_bcgm_m10/README.md',
                              ROOT/'results/paper_bcgm_m10/proof_sizes_and_hashes.csv',
                              ROOT/'vendor/whir/LICENSE-APACHE',ROOT/'vendor/whir/LICENSE-MIT',
                              ROOT/'vendor/whir/README.md']
        with path.open('wb') as raw, gzip.GzipFile(fileobj=raw, mode='wb', filename='', mtime=0) as gz:
            with tarfile.open(fileobj=gz, mode='w') as tar:
                for file in sorted(set(files)):
                    tar.add(file, arcname='canopy-system/'+str(file.relative_to(ROOT)),
                            recursive=False, filter=public_tar_member)
        save(out/'bundle.json',dict(archive=str(path),sha256=digest(path),sources=snapshot()))
        print(path)


if __name__ == '__main__':
    main()
