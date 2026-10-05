import json, subprocess, time
from pathlib import Path

base = Path(__file__).resolve().parent
root = base / 'canopy-system'
out = root / 'results/aws_i4i_optimized'
cfg = json.loads((root / 'experiments/aws-r5/config.json').read_text())
jobs = []
for n, etas in cfg['grid'].items():
    for eta in etas:
        for family, points in [('whir', 3), ('bolt', 2)]:
            folder = out / 'timings' / f'{family}_n{n}_eta{eta}'
            path = folder / 'process.json'
            record = json.loads(path.read_text()) if path.exists() else {'status': 'not_started'}
            jobs.append(dict(name=folder.name, n=int(n), eta=eta, family=family,
                             points=points, folder=folder, record=record))
complete = [j for j in jobs if j['record']['status'] == 'complete']
remaining, estimable, active, errors = 0, True, [], []
for j in jobs:
    r = j['record']
    if r['status'] == 'complete':
        continue
    anchors = [a for a in complete if a['family'] == j['family'] and a['eta'] == j['eta']]
    if anchors:
        a = max(anchors, key=lambda a: a['n'])
        predicted = a['record']['wall_seconds'] * 2**(j['n'] - a['n']) * j['n'] / a['n']
        elapsed = time.time() - r['started_unix'] if r['status'] == 'running' else 0
        remaining += max(0, predicted - elapsed)
    else:
        estimable = False
    if r['status'] == 'running':
        log = j['folder'] / 'run.log'
        active.append(dict(name=j['name'], elapsed_seconds=time.time() - r['started_unix'],
                           tail=log.read_text().splitlines()[-3:] if log.exists() else []))
    elif r['status'] != 'not_started':
        errors.append(dict(name=j['name'], status=r['status']))
mem = {line.split(':')[0]: int(line.split()[1]) * 1024 for line in Path('/proc/meminfo').read_text().splitlines()}
ps = subprocess.check_output(['ps', '-eo', 'pid,rss,pcpu,args'], text=True)
processes = [line.strip() for line in ps.splitlines()
             if str(root / 'target/aws-native/release') in line
             and (' paper-time ' in line or ' time ' in line)]
exit_path = base / 'campaign.exit'
exit_code = int(exit_path.read_text()) if exit_path.exists() else None
phase = 'complete' if len(complete) == len(jobs) and exit_code == 0 else (
    'stopped' if exit_code is not None else 'timing' if active or complete else 'building_or_validating')
log = base / 'campaign.log'
print(json.dumps(dict(timestamp=time.time(), campaign=cfg['campaign'], phase=phase,
    completed_points=sum(j['points'] for j in complete), completed_jobs=len(complete),
    total_points=sum(j['points'] for j in jobs), total_jobs=len(jobs),
    threads=cfg['threads'], cpu_affinity=cfg['cpu_affinity'], warmups=cfg['warmups'], repetitions=cfg['repetitions'],
    rough_remaining_seconds=remaining if estimable else None, active=active, errors=errors,
    available_ram_gib=mem['MemAvailable']/2**30, processes=processes, campaign_exit=exit_code,
    campaign_tail=log.read_text().splitlines()[-4:] if log.exists() else [],
    last_completed=[dict(name=j['name'], wall_seconds=j['record']['wall_seconds'],
                         peak_rss_gib=j['record']['peak_rss_gib']) for j in complete[-3:]]), indent=2))
