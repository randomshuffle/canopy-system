"""Validate the downloaded repeat campaign and summarize its measured results."""
from pathlib import Path
import csv
import hashlib
import json

BASE = Path(__file__).resolve().parent
OUT = BASE / 'remote/results'
MERKLE = 'bolt_merkle_shared'
CANOPY = 'bolt_canopy_split_quarter'

receipt = json.loads((OUT / 'completion-validation.json').read_text())
assert (BASE / 'remote/campaign.exit').read_text().strip() == '0'
assert hashlib.sha256((OUT / 'summary_iop_rounds.csv').read_bytes()).hexdigest() == receipt['summary_sha256']
rows = list(csv.DictReader((OUT / 'summary_iop_rounds.csv').open()))
raw = list(csv.DictReader((OUT / 'raw_samples.csv').open()))
stages = list(csv.DictReader((OUT / 'raw_stages.csv').open()))
idx = {(int(r['input_oracles']), r['scheme']): r for r in rows}
assert len(idx) == len(rows) == 8
assert len(raw) == 32 and all(r['proof_verified'] == 'true' for r in raw)
for row in raw:
    assert row['log_input'] == '24' and row['rayon_threads'] == '32'
    for phase, key in [('commitment', 'commitment_seconds'), ('opening', 'opening_seconds')]:
        total = sum(float(s['seconds']) for s in stages if s['input_oracles'] == row['input_oracles']
                    and s['scheme'] == row['scheme'] and s['iteration'] == row['iteration'] and s['phase'] == phase)
        assert abs(total - float(row[key])) < 1e-7
paired = []
for eta in [1, 2, 4, 8]:
    for iteration in range(4):
        pair = [r for r in raw if int(r['input_oracles']) == eta and int(r['iteration']) == iteration]
        expected = [CANOPY, MERKLE] if iteration % 2 == 0 else [MERKLE, CANOPY]
        assert [r['scheme'] for r in pair] == expected
        times = {r['scheme']: float(r['total_prover_seconds']) for r in pair}
        paired.append(times[CANOPY] > times[MERKLE])

lines = ['# Bolt N=2^24 AWS timing recheck', '',
         'Completed on the existing AWS i4i.24xlarge with 32 Rayon workers pinned to CPUs `0-15,48-63`.',
         'The same validated native release binaries as the N=2^22 recheck were used: GF(2^256), BLAKE3, w=4, the faster shared-tail Merkle baseline, and separate systematic/parity Canopy layers with block lengths k/4. Direct digest conversion uses no extra rehash.', '',
         'Each eta in {1,2,4,8} used one discarded warmup per configuration and four measured passes. Measured order alternated Canopy/Merkle, Merkle/Canopy, Canopy/Merkle, Merkle/Canopy. All 32 measured serialized public-H adapter proofs verified; Spark/Mulperm is excluded.', '',
         '## Reported means', '',
         'Commitment follows the paper convention: C(N,eta) = eta times mean C(N,1) for the same scheme. Opening is the mean measured for that eta, and total is their sum. The eta>1 totals are derived from measured components. All values below are seconds.', '',
         '| eta | Merkle commit | Merkle open | Merkle total | Canopy commit | Canopy open | Canopy total | Canopy overhead |',
         '|---:|---:|---:|---:|---:|---:|---:|---:|']
for eta in [1, 2, 4, 8]:
    a, b = idx[eta, MERKLE], idx[eta, CANOPY]
    vals = [float(r[k]) for r in [a, b] for k in ['mean_commitment_seconds', 'mean_opening_seconds', 'mean_prover_seconds']]
    overhead = 100 * (vals[5] / vals[2] - 1)
    line = f'| {eta} | ' + ' | '.join(f'{x:.6f}' for x in vals) + f' | {overhead:.2f}% |'
    lines.append(line)
    print(line)

lines += ['', '## Raw measured totals', '',
          'These use each workload\'s actual measured commitments without eta=1 rescaling. Ranges span the four measured passes and are not confidence intervals.', '',
          '| eta | Merkle mean (s) | Merkle range (s) | Canopy mean (s) | Canopy range (s) |',
          '|---:|---:|---:|---:|---:|']
for eta in [1, 2, 4, 8]:
    fields = []
    for row in [idx[eta, MERKLE], idx[eta, CANOPY]]:
        fields.extend([f"{float(row['raw_mean_total_prover_seconds']):.6f}",
                       f"{float(row['raw_min_total_prover_seconds']):.6f} to {float(row['raw_max_total_prover_seconds']):.6f}"])
    lines.append(f'| {eta} | ' + ' | '.join(fields) + ' |')
lines += ['', f'Canopy was slower in {sum(paired)} of the 16 paired measured passes.', '',
          f"Timing jobs took {receipt['total_timing_job_wall_seconds']:.2f} seconds including warmups, setup, and verification; proof/hash parameter preparation is additional. Peak RSS was {receipt['peak_rss_gib']:.2f} GiB.", '',
          'Files: [summary](remote/results/summary_iop_rounds.csv), [raw samples](remote/results/raw_samples.csv), [stage means](remote/results/stage_summary.csv), [raw stages](remote/results/raw_stages.csv), and [validation receipt](remote/results/completion-validation.json). Configuration, host/build provenance, proof/hash estimates, and run logs are retained under `remote/`. Historical data and manuscript figures remain separate.', '']
(BASE / 'README.md').write_text('\n'.join(lines))
print(f'Validated 32 proofs, phase reconciliation, and balanced order. Canopy slower in {sum(paired)}/16 pairs.')
print(json.dumps(receipt, indent=2))
