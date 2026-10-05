import csv, hashlib, json, os, shutil
from pathlib import Path

base = Path(__file__).resolve().parent
root = base / 'canopy-system'
prior = Path(os.environ['PRIOR_CAMPAIGN_DIR']).resolve() / 'canopy-system'
src = prior / 'results/aws_i4i_optimized'
out = root / 'archive/results/aws_i4i_fair'
cfg = json.loads((root / 'experiments/aws-r5/config.json').read_text())
old_cfg = json.loads((prior / 'experiments/aws-r5/config.json').read_text())
for key in set(cfg) | set(old_cfg):
    if key == 'campaign': continue
    if key == 'grid':
        assert {n:sorted(etas) for n,etas in cfg[key].items()} == {n:sorted(etas) for n,etas in old_cfg[key].items()}
    else: assert cfg[key] == old_cfg[key], key
old_build = json.loads((src / 'native-build.json').read_text())
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
allowed = {'experiments/aws-r5/config.json','scripts/aws_benchmark.py','src/canopy.rs','src/executable_whir.rs','src/paper_experiments.rs'}
changed = [name for name,h in old_build['source_sha256'].items() if sha(root/name) != h]
assert set(changed) == allowed, changed
for name in ('src/bolt.rs','src/bolt_experiment.rs','src/bolt_batching.rs','src/bolt_multi.rs','src/bolt_split.rs','src/additive_rs.rs','src/binary_field.rs','src/merkle.rs'):
    assert sha(root/name) == old_build['source_sha256'][name], name
out.mkdir(parents=True,exist_ok=True)
shutil.copytree(src/'estimates',out/'estimates')
for name in ('proof_sizes_and_hashes.csv','native-build.json','host.json','plan.json'):
    shutil.copy2(src/name,out/('prior-'+name))
imports=[]
for n,etas in cfg['grid'].items():
    for eta in etas:
        job = f'bolt_n{n}_eta{eta}'
        folder = src/'timings'/job
        record = json.loads((folder/'process.json').read_text())
        assert record['status'] == 'complete'
        assert record['cpu_affinity'] == cfg['cpu_affinity']
        assert record['configuration']['threads'] == 32
        assert record['configuration']['repetitions'] == 1 and record['configuration']['warmups'] == 0
        rows = list(csv.DictReader((folder/'prover_timings.csv').open()))
        assert {r['scheme'] for r in rows} == set(cfg['configurations'][3:]) and len(rows) == 2
        assert all(r['proof_verified']=='true' and r['rayon_threads']=='32' and r['iteration']=='0' for r in rows)
        dest = out/'timings'/job
        shutil.copytree(folder,dest)
        imports.append({'job':job,'source':str(folder),'files_sha256':{p.name:sha(p) for p in dest.iterdir() if p.is_file()}})
(out/'imported-bolt-provenance.json').write_text(json.dumps({'prior_source':str(src),'changed_source_files':changed,'imports':imports,'note':'28 unchanged Bolt timing points retained; all 42 WHIR points remeasured with symmetric concurrent commitments. Original Bolt process records and build receipt are preserved.'},indent=2)+'\n')
print('Imported 28 verified single-pass Bolt points and unchanged proof/hash estimates.',flush=True)
