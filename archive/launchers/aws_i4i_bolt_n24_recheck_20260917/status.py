from pathlib import Path
import csv,json,time
p=Path(__file__).resolve().parent
print('phase:',(p/'phase').read_text().strip())
print('exit:',(p/'campaign.exit').read_text().strip() if (p/'campaign.exit').exists() else 'running')
for q in sorted((p/'results/timings').glob('*/process.json')):
 d=json.loads(q.read_text())
 print(q.parent.name,d['status'],'wall seconds',round(d.get('wall_seconds',time.time()-d['started_unix']),1),'peak GiB',d.get('peak_rss_gib'))
 if d['status']=='running':
  print('latest:', '; '.join((q.parent/'run.log').read_text().splitlines()[-2:]))
  f=q.parent/'prover_timings.csv'
  if f.exists(): print('saved measured samples:',len(list(csv.DictReader(f.open()))),'/ 8')
print('campaign log:', '; '.join((p/'campaign.log').read_text().splitlines()[-2:]))
