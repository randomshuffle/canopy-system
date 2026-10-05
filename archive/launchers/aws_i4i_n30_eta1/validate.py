import csv, hashlib, json, time
from pathlib import Path
base = Path(__file__).resolve().parent
root = base / 'canopy-system'
out = root / 'results/aws_i4i_n30_eta1'
cfg = json.loads((root / 'experiments/aws-r5/config-n30-eta1.json').read_text())
rows = list(csv.DictReader((out / 'comparison_iop_rounds.csv').open()))
assert len(rows) == 5 and {r['scheme'] for r in rows} == set(cfg['configurations'])
for r in rows:
    assert (int(r['log_input']), int(r['input_oracles'])) == (30, 1)
    assert r['timing_status'] == 'measured' and int(r['repetitions']) == 1
    assert int(r['proof_samples']) == 100 and float(r['proof_bytes']) > 0
    assert float(r['verifier_hash_to_field']) == 0
    assert abs(float(r['verifier_hashes']) - float(r['verifier_auth_hashes']) - float(r['fs_finalizations'])) < 1e-6
    assert abs(float(r['mean_prover_seconds']) - float(r['mean_commitment_seconds']) - float(r['mean_opening_seconds'])) < 1e-7
for family in ['whir', 'bolt']:
    p = out / 'timings' / f'{family}_n30_eta1'
    rec = json.loads((p/'process.json').read_text())
    assert rec['status'] == 'complete' and rec['exit_code'] == 0
    assert rec['cpu_affinity'] == cfg['cpu_affinity']
    assert rec['configuration']['threads'] == 32
    assert rec['configuration']['warmups'] == 0 and rec['configuration']['repetitions'] == 1
    if family == 'bolt':
        samples = list(csv.DictReader((p/'prover_timings.csv').open()))
        assert len(samples) == 2 and all(r['proof_verified']=='true' for r in samples)
report = dict(validated_unix=time.time(), unique_grid_points=5, raw_timing_samples=5,
    verified_bolt_proofs=2, workers=32, cpu_affinity=cfg['cpu_affinity'], warmups=0,
    repetitions=1, digest_conversion='direct invertible GF256 byte mapping; no rehash',
    source_comparison_sha256=hashlib.sha256((out/'comparison_iop_rounds.csv').read_bytes()).hexdigest())
(out/'completion-validation.json').write_text(json.dumps(report,indent=2)+'\n')
print('Validated all five N=2^30 eta=1 points; both serialized Bolt proofs verified; zero digest conversion hashes.')
