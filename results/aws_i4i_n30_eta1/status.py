import json, subprocess, time
from pathlib import Path
base=Path(__file__).resolve().parent
root=base/'canopy-system'
out=root/'results/aws_i4i_n30_eta1'
now=time.time()
budgets={'whir':2191.4,'bolt':690.5}
complete=[]; active=[]; errors=[]; remaining=0
for family,points in [('whir',3),('bolt',2)]:
    folder=out/'timings'/f'{family}_n30_eta1'
    p=folder/'process.json'
    r=json.loads(p.read_text()) if p.exists() else {'status':'not_started'}
    if r['status']=='complete':
        complete.append(dict(family=family,points=points,wall_seconds=r['wall_seconds'],peak_rss_gib=r['peak_rss_gib']))
    elif r['status']=='running':
        elapsed=now-r['started_unix']
        remaining+=max(0,budgets[family]-elapsed)
        log=folder/'run.log'
        active.append(dict(family=family,elapsed_seconds=elapsed,tail=log.read_text().splitlines()[-4:] if log.exists() else []))
    elif r['status']=='not_started': remaining+=budgets[family]
    else: errors.append(dict(family=family,status=r['status']))
ps=subprocess.check_output(['ps','-eo','pid,rss,pcpu,args'],text=True)
processes=[]
for line in ps.splitlines():
    if str((root/'target/aws-native/release').resolve()) in line and (' paper-time ' in line or ' time ' in line):
        pid=int(line.split()[0]); proc=Path(f'/proc/{pid}')
        status=(proc/'status').read_text() if proc.exists() else ''
        fields={x.split(':',1)[0]:x.split(':',1)[1].strip() for x in status.splitlines() if ':' in x}
        processes.append(dict(pid=pid,rss_gib=int(line.split()[1])/2**20,cpu_percent=float(line.split()[2]),allowed_cpus=fields.get('Cpus_allowed_list'),threads=fields.get('Threads')))
mem={x.split(':')[0]:int(x.split()[1])*1024 for x in Path('/proc/meminfo').read_text().splitlines()}
code=int((base/'campaign.exit').read_text()) if (base/'campaign.exit').exists() else None
phase=(base/'phase').read_text().strip() if (base/'phase').exists() else 'not_started'
log=base/'campaign.log'
print(json.dumps(dict(timestamp=now,campaign='gf256-five-configs-i4i-n30-eta1-direct-digest',phase=phase,campaign_exit=code,
 completed_points=sum(x['points'] for x in complete),total_points=5,rough_remaining_seconds=remaining,
 estimate_scope='remaining timing jobs only; build/preparation are additional',active=active,complete=complete,errors=errors,
 available_ram_gib=mem['MemAvailable']/2**30,processes=processes,campaign_tail=log.read_text().splitlines()[-6:] if log.exists() else []),indent=2))
