#!/usr/bin/env python3
"""Validate w=16 verifier Fiat-Shamir traces using binaries built with fs-accounting."""
import csv
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'results/gf256_split_w16_suite'
# Retain the exact selected shared-baseline rows used by the trace validator.
with (OUT / 'shared/proof_sizes/proof_sizes.csv').open() as f:
    rows = [r for r in csv.DictReader(f) if r['scheme'] == 'bolt_merkle_batched']
assert len(rows) == 28 and all(r['interleaving'] == '16' for r in rows)
with (OUT / 'shared/merkle_proof_sizes.csv').open('w', newline='') as f:
    w = csv.DictWriter(f, fieldnames=list(rows[0]))
    w.writeheader()
    w.writerows(rows)
for part in ['half', 'full', 'shared']:
    binary = ROOT / 'target/release/examples' / ('gf256_bolt_suite' if part == 'shared' else 'gf256_bolt_split')
    source = OUT / part / ('merkle_proof_sizes.csv' if part == 'shared' else 'proof_sizes/proof_sizes.csv')
    subprocess.run([
        sys.executable, str(ROOT / 'scripts/measure_gf256_run.py'),
        '--record-dir', str(OUT / part / 'fs_validation'), '--threads', '8', '--',
        str(binary), 'validate', '--input', str(source),
        '--output', str(OUT / part / 'fiat_shamir_validation.csv'),
    ], cwd=ROOT, check=True)
print('All 30 Fiat-Shamir verifier traces match.', flush=True)
