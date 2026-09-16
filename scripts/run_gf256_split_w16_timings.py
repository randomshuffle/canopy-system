#!/usr/bin/env python3
"""Time the selected w=16 Bolt baseline and both independent Canopy block lengths."""
import argparse
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'results/gf256_split_w16_suite'
p = argparse.ArgumentParser(description=__doc__)
p.add_argument('--binary', type=Path, default=ROOT / 'target/release/examples/gf256_bolt_split')
a = p.parse_args()
for n in [18, 20, 22]:
    for eta in [1, 2, 4, 8]:
        d = OUT / f'timing_n{n}_eta{eta}'
        if (d / 'process.json').exists() and json.loads((d / 'process.json').read_text())['exit_code'] == 0:
            print('Already complete:', d.name, flush=True)
            continue
        subprocess.run([
            sys.executable, str(ROOT / 'scripts/measure_gf256_run.py'),
            '--record-dir', str(d), '--threads', '8', '--', str(a.binary.resolve()),
            'time', '--width', '16', '--log', str(n), '--inputs', str(eta),
            '--warmups', '1', '--repetitions', '5', '--paper-only',
            '--estimates', str(OUT / 'half/proof_sizes'),
            '--alternate-estimates', str(OUT / 'full/proof_sizes'),
            '--shared-estimates', str(OUT / 'shared/proof_sizes'),
            '--output', str(d),
        ], cwd=ROOT, check=True)
print('All 180 measured proofs and 36 warmup proofs verified.', flush=True)
