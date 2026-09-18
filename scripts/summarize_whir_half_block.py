#!/usr/bin/env python3
"""Report the half-length Canopy WHIR rerun at eta=1 without replacing saved paper data."""
import csv
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'results/whir_half_block_eta1'
OLD = ROOT / 'results/gf256_batched_suite/whir'

def read(path):
    with path.open() as f:
        return list(csv.DictReader(f))

def write(path, rows):
    with path.open('w', newline='') as f:
        w=csv.DictWriter(f, fieldnames=list(rows[0]));w.writeheader();w.writerows(rows)

proof=read(OUT/'proof_sizes/hybrid_proof_sizes.csv')
fs={(r['log_message_length'],r['scheme']):r for r in read(OUT/'fiat_shamir.csv')}
old={(r['log_message_length'],r['scheme']):r for r in read(OLD/'proof_sizes/hybrid_proof_sizes.csv') if r['input_oracles']=='1'}
for r in proof:
    assert r['input_oracles']=='1'
    extra=fs[r['log_message_length'],r['scheme']]
    r.update({k:extra[k] for k in ['verifier_hashes','fs_finalizations','fs_absorb_bytes','fs_squeeze_bytes','fs_proof_bytes']})
    assert float(r['algebraic_rbr_union_bound_bits'])>=128
    assert abs(float(r['verifier_hashes'])-sum(float(r[k]) for k in ['verifier_auth_hashes','verifier_hash_to_field','fs_finalizations']))<1e-6
    if r['scheme']!='canopy_schwartz_zippel':
        for k in ['mean_total_bytes','verifier_hashes']:
            assert abs(float(r[k])-float(old[r['log_message_length'],r['scheme']][k]))<1e-6
    else:
        assert int(r['aligned_codeword_log'])==int(r['log_message_length'])
        assert int(r['canopy_codeword_log_reduction'])==1
assert len(proof)==21
write(OUT/'proof_sizes_with_hashes.csv',proof)

methods=[('merkle_whir_batched_after_second','WHIR, rate 1/2'),('merkle_whir_batched_rate_quarter_after_second','WHIR, rate 1/4'),('canopy_schwartz_zippel','Canopy, half-length')]
idx={(r['log_message_length'],r['scheme']):r for r in proof}
lines=['# WHIR with a half-length Canopy codeword, eta = 1','',
'All results use GF(2^256), BLAKE3, a composed 128-bit target, and one input polynomial. The base rate-1/2 oracle has 2N field elements; the Canopy RS codeword now has N. The selected initial fold remains three variables and h=4, so its message is N/8 and the aligned shared tail starts at rate 1/8 (previously 1/16). Both the Canopy and folded base oracle match in message and block length. No Bolt parameters are changed by this WHIR experiment.','',
'Proof sizes and verifier hashes cover all seven requested sizes using 100 deterministic query samples. Each method optimizes its folding schedule over {3,4}. Verifier hashes include authentication, H2F, and Fiat–Shamir finalizations. The two Merkle baselines reproduce the saved proof sizes and hash counts exactly.','',
'## Proof size (KiB) / verifier hashes','',
'| N | WHIR 1/2 | WHIR 1/4 | Canopy, prior full length | Canopy, half length |',
'|---|---:|---:|---:|---:|']
comparison=[]
for n in [18,20,22,24,26,28,30]:
    rows=[idx[str(n),methods[0][0]],idx[str(n),methods[1][0]],old[str(n),'canopy_schwartz_zippel'],idx[str(n),'canopy_schwartz_zippel']]
    lines.append('| 2^'+str(n)+' | '+' | '.join(f"{float(r['mean_total_kib']):.2f} / {float(r['verifier_hashes']):,.0f}" for r in rows)+' |')
    a,b=rows[2:]
    comparison.append(dict(log_input=n,input_oracles=1,full_length_proof_bytes=float(a['mean_total_bytes']),half_length_proof_bytes=float(b['mean_total_bytes']),proof_change_percent=100*(float(b['mean_total_bytes'])/float(a['mean_total_bytes'])-1),full_length_verifier_hashes=float(a['verifier_hashes']),half_length_verifier_hashes=float(b['verifier_hashes']),hash_change_percent=100*(float(b['verifier_hashes'])/float(a['verifier_hashes'])-1)))
write(OUT/'comparison.csv',comparison)

timing=read(OUT/'timing_n22/prover_timings.csv')
control=read(OUT/'full_length_control_n22/prover_timings.csv')
for rr in [timing,control]:
    assert len(rr)==3 and all(r['input_oracles']=='1' and r['repetitions']=='5' for r in rr)
lines+=['','## Measured prover time at N = 2^22, eta = 1','',
'One warmup and five measured repetitions per configuration, eight Rayon threads. Times include commitment and opening, excluding input generation, setup, and verification. These retain the component-composed timing scope: native WHIR components are verifier-checked, but this is not a single serialized end-to-end Canopy argument.','',
'| Configuration | Mean commit (s) | Mean open (s) | Median total (s) | Min–max total (s) |','|---|---:|---:|---:|---:|']
for scheme,label in methods:
    r=next(r for r in timing if r['scheme']==scheme)
    lines.append(f"| {label} | {float(r['mean_commitment_seconds']):.3f} | {float(r['mean_opening_seconds']):.3f} | {float(r['median_prover_seconds']):.3f} | {float(r['min_prover_seconds']):.3f}–{float(r['max_prover_seconds']):.3f} |")
full=next(r for r in control if r['scheme']=='canopy_schwartz_zippel')
half=next(r for r in timing if r['scheme']=='canopy_schwartz_zippel')
saving=100*(1-float(half['median_prover_seconds'])/float(full['median_prover_seconds']))
lines+=['',f"A fresh full-length control run measured Canopy at {float(full['median_prover_seconds']):.3f} s (range {float(full['min_prover_seconds']):.3f}–{float(full['max_prover_seconds']):.3f} s), versus {float(half['median_prover_seconds']):.3f} s for half length: {saving:.1f}% lower in these runs. The runs were sequential, not interleaved; this is a sample comparison, not a confidence bound. Both controls and raw stages are retained. Means of the phases need not sum to the median total.",'',
f"At N=2^30, halving the Canopy block raises proof size by {comparison[-1]['proof_change_percent']:.1f}% and verifier hashes by {comparison[-1]['hash_change_percent']:.1f}% relative to the previous full-length Canopy configuration. The higher rate saves encoding/commitment work but increases the query cost.",'',
'## Reproduce','',
'```sh',
'cargo build --offline --release --bin canopy-system --example whir_fiat_shamir',
'RAYON_NUM_THREADS=8 target/release/canopy-system paper-estimate --inputs 1 --samples 100 --canopy-codeword-log-reduction 1 --output-dir results/whir_half_block_eta1/proof_sizes',
'target/release/examples/whir_fiat_shamir --input-dir results/whir_half_block_eta1/proof_sizes --output results/whir_half_block_eta1/fiat_shamir.csv',
'python3 scripts/measure_gf256_run.py --record-dir results/whir_half_block_eta1/timing_n22 --threads 8 -- target/release/canopy-system paper-time --log-length 22 --inputs 1 --warmups 1 --repetitions 5 --seed 73947765755993 --canopy-codeword-log-reduction 1 --output-dir results/whir_half_block_eta1/timing_n22',
'# Repeat the timing command with reduction 0 and output/record directory full_length_control_n22 for the control.',
'python3 scripts/summarize_whir_half_block.py','```','',
'`proof_sizes_with_hashes.csv` has all 21 selected rows; `comparison.csv` compares old and new Canopy sizes/hashes. Raw timings, stage CSVs, process RSS, and binary identities are retained in `timing_n22/` and `full_length_control_n22/`. This rerun does not overwrite the manuscript data or extrapolate new larger-size timings.']
(OUT/'README.md').write_text('\n'.join(lines)+'\n')
paths=['src/paper_experiments.rs','src/paper_fiat_shamir.rs','src/main.rs','examples/whir_fiat_shamir.rs','scripts/summarize_whir_half_block.py','results/gf256_batched_suite/whir/proof_sizes/hybrid_proof_sizes.csv']
manifest=dict(field='GF(2^256)',hash='BLAKE3',input_oracles=[1],log_inputs=[18,20,22,24,26,28,30],timing_log_input=22,canopy_codeword_log_reduction=1,base_codeword_length='2N',canopy_codeword_length='N',samples=100,source_sha256={p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in paths},processes={d:json.loads((OUT/d/'process.json').read_text()) for d in ['proof_sizes','timing_n22','full_length_control_n22']})
(OUT/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print('\n'.join(lines[8:20]));print(f'Fresh control timing saving: {saving:.1f}%')
