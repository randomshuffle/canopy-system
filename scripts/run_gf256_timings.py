#!/usr/bin/env python3
"""Run the requested timing matrix sequentially, resuming completed cases."""
import json, subprocess, sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'results/gf256_batched_suite'
assert (OUT/'bolt/proof_sizes/proof_sizes.csv').exists(), 'Run the Bolt parameter search first.'
assert (OUT/'whir/proof_sizes/hybrid_proof_sizes.csv').exists(), 'Run the WHIR parameter search first.'
commands=[]
for n in [18,20,22]:
 d=OUT/f'whir/timing_n{n}'
 commands.append((d,[str(ROOT/'target/release/canopy-system'),'paper-time','--log-length',str(n),'--warmups','1','--repetitions','5','--seed',str(0x43414e4f5059),'--output-dir',str(d)]))
for n in [18,20,22]:
 for eta in [1,2,4,8]:
  d=OUT/f'bolt/timing_n{n}_eta{eta}'
  commands.append((d,[str(ROOT/'target/release/examples/gf256_bolt_suite'),'time','--log',str(n),'--inputs',str(eta),'--warmups','1','--repetitions','5','--estimates',str(OUT/'bolt/proof_sizes'),'--output',str(d)]))
for d,command in commands:
 if (d/'process.json').exists() and json.loads((d/'process.json').read_text())['exit_code']==0:
  print('Already complete:',d.name,flush=True);continue
 subprocess.run([sys.executable,str(ROOT/'scripts/measure_gf256_run.py'),'--record-dir',str(d),'--threads','8','--',*command],cwd=ROOT,check=True)
print('Timing matrix complete. Regenerate FS accounting, then run summarize_gf256_suite.py.',flush=True)
