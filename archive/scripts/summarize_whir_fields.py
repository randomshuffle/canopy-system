#!/usr/bin/env python3
"""Compare matched prime/binary WHIR/Canopy component timings."""
import csv
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'archive/results/whir_field_comparison_n22'
labels={
 'merkle_whir_independent':'Merkle-WHIR, independent',
 'merkle_whir_batched_after_second':'Merkle-WHIR, batched, rate 1/2',
 'merkle_whir_batched_rate_quarter_after_second':'Merkle-WHIR, batched, rate 1/4',
 'canopy_unbatched':'Canopy, independent tails, h=9',
 'canopy_schwartz_zippel':'Canopy, batched tails, h=4',
}
def read(field,name):
 return list(csv.DictReader((OUT/field/name).open()))
data={field:{(int(r['input_oracles']),r['scheme']):r for r in read(field,'prover_timings.csv')} for field in ['prime','binary']}
process={f:json.loads((OUT/f/'process.json').read_text()) for f in data}
for p in process.values():
 assert p['exit_code']==0 and p['log_length']==22 and p['threads']==8
for key in ['seed','log_length','input_polynomials','warmups','repetitions','threads']:
 assert process['prime'][key]==process['binary'][key],key
rows=[]
for t in [1,2,4,8]:
 for scheme in labels:
  p=data['prime'][t,scheme];b=data['binary'][t,scheme]
  for k in ['log_input_length','log_initial_codeword_length','executable_folding_log','canopy_height','repetitions']:
   assert p[k]==b[k],(t,scheme,k)
  out=dict(input_polynomials=t,scheme=scheme)
  for phase in ['commitment','opening','prover']:
   x=float(p[f'mean_{phase}_seconds']);y=float(b[f'mean_{phase}_seconds'])
   out.update({f'prime_{phase}_seconds':x,f'binary_{phase}_seconds':y,f'binary_over_prime_{phase}':y/x})
  out['prime_median_seconds']=float(p['median_prover_seconds'])
  out['binary_median_seconds']=float(b['median_prover_seconds'])
  rows.append(out)
with (OUT/'comparison.csv').open('w') as f:
 w=csv.DictWriter(f,fieldnames=rows[0]);w.writeheader();w.writerows(rows)
lines=['# WHIR/Canopy: GF(2^256) versus the 256-bit prime','',
 'Matched fresh runs on **Apple M2, 16 GiB RAM**, at **2^22 field elements per polynomial**, **t=1,2,4,8**, eight Rayon threads, BLAKE3-256, 128-bit Johnson parameters, no PoW, one warmup and five measured repetitions. Both fields use 32-byte elements and deterministic full-width inputs and claim points.','',
 '**Timing boundary:** these reproduce the existing paper\'s component-composed prover benchmark. Commitment and opening are both included. Executable WHIR components are verified, while independent first transitions are measured once and scaled by t; batched tails and cross terms execute at their reported t. A separately measured native suffix is subtracted to model termination at at most 2^10 coefficients. These totals are not stopwatch measurements of a single end-to-end Canopy proof. Setup/input generation, verification and final deallocation are excluded.','',
 'The initial/recursive executable folding log is 3 (factor 8). The existing n=22 proof-size grid selects h=9 for independent Canopy; its additional Canopy WHIR tail is included. Batched Canopy aligns its two source types at h=4. Heights and geometry match across fields.','',
 '## Median total prover times','',
 '| t | Scheme | Prime s | GF(2^256) s | Binary/prime |',
 '|---:|---|---:|---:|---:|']
for r in rows:
 x=r['prime_median_seconds'];y=r['binary_median_seconds']
 lines.append(f"| {r['input_polynomials']} | {labels[r['scheme']]} | {x:.3f} | {y:.3f} | {y/x:.3f}x |")
lines+=['', 'A slower binary repetition noticeably affects the means. Medians retain all samples and reduce that influence; raw measurements are linked below. These are five-repeat measurements, not confidence bounds.','',
 '## Mean commitment + opening times','',
 '| t | Scheme | Prime commit s | Binary commit s | Prime opening s | Binary opening s | Prime total s | Binary total s | Binary/prime total |',
 '|---:|---|---:|---:|---:|---:|---:|---:|---:|']
for r in rows:
 lines.append(f"| {r['input_polynomials']} | {labels[r['scheme']]} | {r['prime_commitment_seconds']:.3f} | {r['binary_commitment_seconds']:.3f} | {r['prime_opening_seconds']:.3f} | {r['binary_opening_seconds']:.3f} | {r['prime_prover_seconds']:.3f} | {r['binary_prover_seconds']:.3f} | {r['binary_over_prime_prover']:.3f}x |")
lines+=['','## Canopy overhead within each field (mean totals)','',
 '| t | Canopy variant | Prime Canopy / corresponding Merkle | Binary Canopy / corresponding Merkle |',
 '|---:|---|---:|---:|']
for t in [1,2,4,8]:
 for canopy,base in [('canopy_unbatched','merkle_whir_independent'),('canopy_schwartz_zippel','merkle_whir_batched_after_second')]:
  ratios=[float(data[f][t,canopy]['mean_prover_seconds'])/float(data[f][t,base]['mean_prover_seconds']) for f in ['prime','binary']]
  lines.append(f'| {t} | {labels[canopy]} | {ratios[0]:.3f}x | {ratios[1]:.3f}x |')
lines+=['','## Stage comparison','',
 '| t | Scheme | Stage | Prime s | Binary s | Binary/prime |',
 '|---:|---|---|---:|---:|---:|']
stages={f:{(int(r['input_oracles']),r['scheme'],r['stage']):float(r['mean_seconds']) for r in read(f,'prover_timing_stages.csv')} for f in data}
for key,x in stages['prime'].items():
 y=stages['binary'][key]
 ratio=f'{y/x:.3f}x' if x else '—'
 lines.append(f'| {key[0]} | {labels[key[1]]} | {key[2]} | {x:.5f} | {y:.5f} | {ratio} |')
lines+=['','## Memory and reproducibility','',
 '| Field | Peak process RSS GiB | Process wall s |',
 '|---|---:|---:|']
for f,p in process.items():lines.append(f"| {f} | {p['peak_rss_gib']:.3f} | {p['wall_seconds']:.1f} |")
lines+=['','RSS covers the whole process, including setup and verification, with all five schemes executed sequentially; it is not a per-scheme prover peak.','',
 '- [Prime raw samples](prime/prover_timing_samples.csv), [stages](prime/prover_timing_stages.csv), [command and executable hash](prime/process.json).',
 '- [Binary raw samples](binary/prover_timing_samples.csv), [stages](binary/prover_timing_stages.csv), [command and executable hash](binary/process.json).',
 '- [Machine-readable comparison](comparison.csv).',
 '- [Field validation](field_validation.json), [machine](machine.json), and [matching dependency versions](common_dependencies.json).',
 '- [Backend and reproduction instructions](../../archive/experiments/binary-whir/README.md).','',
 'The prime field is p=2^256-43*2^32+1. The binary backend uses GF(2^128)[w]/(w^2+w+x^121), with the base polynomial x^128+x^7+x^2+x+1, PMULL multiplication and monomial RS evaluation via additive FFTs. WHIR query aggregation uses a transposed additive transform; the prime backend retains its sparse NTT optimization. This is a comparison of these implementations on this host, not a hardware-independent claim about field arithmetic.','',
 'The older 2^18 prime timings also used integer-pattern inputs and smaller claim points. The fresh 2^22 prime run is the denominator here; comparing directly with the old totals would mix input size, data and field changes. The 192-bit field-switch model is retired and its historical artifacts are archived. Bolt\'s field and experiment are unchanged.','']
(OUT/'comparison.md').write_text('\n'.join(lines))
for r in rows:
 if r['scheme'] in ['merkle_whir_batched_after_second','canopy_schwartz_zippel']:
  print(r['input_polynomials'],r['scheme'],f"prime={r['prime_prover_seconds']:.3f}s binary={r['binary_prover_seconds']:.3f}s ratio={r['binary_over_prime_prover']:.3f}")
