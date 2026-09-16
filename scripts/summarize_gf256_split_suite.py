#!/usr/bin/env python3
"""Summarize the independent-tail Bolt study and export the selected paper comparison."""
import csv
import hashlib
import json
import statistics
from collections import defaultdict
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT/'results/gf256_split_suite'
OLD = ROOT/'results/gf256_batched_suite/bolt'
def read(p):
    with p.open() as f:
        return list(csv.DictReader(f))
def write(name, rows):
    assert rows
    with (OUT/name).open('w', newline='') as f:
        w = csv.DictWriter(f, fieldnames=list(rows[0])); w.writeheader(); w.writerows(rows)
split = read(OUT/'proof_sizes/proof_sizes.csv')
shared = read(OLD/'best_proof_parameters.csv')
assert len(split) == len(shared) == 56
fields = list(split[0])
proof = split.copy()
for s in shared:
    r = {k:s.get(k,'') for k in fields}
    r['scheme'] = s['scheme'].replace('_batched','_shared')
    r['systematic_target_log'] = r['parity_target_log'] = s['aligned_log']
    if 'canopy' in r['scheme']:
        assert s['layout'] == 'equal_full'
        r['systematic_rs_length'] = r['parity_rs_length'] = str(1 << (int(s['log_input'])-2))
    else:
        r['systematic_rs_length'] = r['parity_rs_length'] = '0'
    proof.append(r)
proof.sort(key=lambda r:(int(r['log_input']), int(r['input_oracles']),r['scheme']))
write('all_proof_sizes.csv', proof)
by = {(int(r['log_input']), int(r['input_oracles']), r['scheme']):r for r in proof}
assert len(by) == 112
comp = []
for n in [18,20,22,24,26,28,30]:
    for eta in [1,2,4,8]:
        old = by[n,eta,'bolt_canopy_shared']; new = by[n,eta,'bolt_canopy_split']
        m = min((by[n,eta,'bolt_merkle_shared'], by[n,eta,'bolt_merkle_split']), key=lambda r:float(r['total_bytes']))
        comp.append(dict(log_input=n,input_oracles=eta,baseline_scheme=m['scheme'],
            baseline_bytes=float(m['total_bytes']),shared_canopy_bytes=float(old['total_bytes']),split_canopy_bytes=float(new['total_bytes']),
            canopy_proof_reduction_vs_shared_percent=100*(1-float(new['total_bytes'])/float(old['total_bytes'])),
            canopy_proof_reduction_vs_baseline_percent=100*(1-float(new['total_bytes'])/float(m['total_bytes'])),
            baseline_verifier_hashes=float(m['verifier_hashes']),split_canopy_verifier_hashes=float(new['verifier_hashes']),
            canopy_verifier_hash_reduction_percent=100*(1-float(new['verifier_hashes'])/float(m['verifier_hashes'])),
            systematic_h=int(new['systematic_h']),parity_h=int(new['parity_h']),
            systematic_target_log=int(new['systematic_target_log']),parity_target_log=int(new['parity_target_log'])))
write('comparison.csv',comp)
samples=[];stages=[];processes=[]
for d in sorted(OUT.glob('timing_n*_eta*')):
    if not (d/'process.json').exists(): continue
    process=json.loads((d/'process.json').read_text())
    assert process['exit_code']==0
    processes.append(process)
    samples.extend(read(d/'prover_timings.csv'))
    stages.extend(read(d/'prover_timing_stages.csv'))
assert len(processes)==12 and len(samples)==240
assert all(r['proof_verified']=='true' and r['rayon_threads']=='8' for r in samples)
grouped=defaultdict(list)
for r in samples: grouped[int(r['log_input']),int(r['input_oracles']),r['scheme']].append(r)
assert len(grouped)==48
summary=[]
for (n,eta,scheme),rr in sorted(grouped.items()):
    assert len(rr)==5
    values=[float(r['total_prover_seconds']) for r in rr]
    for r in rr:
        assert abs(float(r['commitment_seconds'])+float(r['opening_seconds'])-float(r['total_prover_seconds']))<1e-8
    summary.append(dict(log_input=n,input_oracles=eta,scheme=scheme,
        median_total_seconds=statistics.median(values),mean_total_seconds=statistics.mean(values),
        mean_commitment_seconds=statistics.mean(float(r['commitment_seconds']) for r in rr),
        mean_opening_seconds=statistics.mean(float(r['opening_seconds']) for r in rr),
        mean_verification_seconds=statistics.mean(float(r['verification_seconds']) for r in rr),
        min_total_seconds=min(values),max_total_seconds=max(values),
        mean_serialized_bytes=statistics.mean(int(r['proof_bytes']) for r in rr)))
write('timing_summary.csv',summary);write('timing_samples.csv',samples)
write('timing_stages.csv',stages)
stage_iterations=defaultdict(float)
for r in stages:
    stage_iterations[int(r['log_input']),int(r['input_oracles']),r['scheme'],r['phase'],r['part'],r['stage'],int(r['iteration'])]+=float(r['seconds'])
stage_groups=defaultdict(list)
for (*key, iteration), value in stage_iterations.items():
    stage_groups[tuple(key)].append(value)
stage_summary=[dict(log_input=n,input_oracles=eta,scheme=scheme,phase=phase,part=part,stage=stage,mean_seconds=statistics.mean(v)) for (n,eta,scheme,phase,part,stage),v in sorted(stage_groups.items())]
write('timing_stage_summary.csv',stage_summary)
for r in summary:
    for phase,column in [('commitment','mean_commitment_seconds'),('opening','mean_opening_seconds')]:
        total=sum(x['mean_seconds'] for x in stage_summary if all(x[k]==r[k] for k in ['log_input','input_oracles','scheme']) and x['phase']==phase)
        assert abs(total-r[column]) < 1e-8, (r,phase,total)
timing={(r['log_input'],r['input_oracles'],r['scheme']):r for r in summary}
for r in comp:
    n,eta=r['log_input'],r['input_oracles']
    for name,scheme in [('baseline',r['baseline_scheme']),('shared_canopy','bolt_canopy_shared'),('split_canopy','bolt_canopy_split')]:
        t=timing.get((n,eta,scheme),{})
        r[name+'_median_seconds']=t.get('median_total_seconds','')
    r['split_canopy_time_overhead_percent']=(100*(r['split_canopy_median_seconds']/r['baseline_median_seconds']-1)) if n<=22 else ''
    r['split_canopy_time_change_vs_shared_percent']=(100*(r['split_canopy_median_seconds']/r['shared_canopy_median_seconds']-1)) if n<=22 else ''
write('comparison.csv',comp)
# Stable paper exports contain only the selected Merkle baseline and split Canopy.
paper_keys={(r['log_input'],r['input_oracles'],r['baseline_scheme']) for r in comp}
paper_keys.update((r['log_input'],r['input_oracles'],'bolt_canopy_split') for r in comp)
paper_proofs=[r for r in proof if (int(r['log_input']),int(r['input_oracles']),r['scheme']) in paper_keys]
paper_timings=[r for r in summary if (r['log_input'],r['input_oracles'],r['scheme']) in paper_keys]
assert len(paper_proofs)==56 and len(paper_timings)==24
for r in paper_timings:
    expected=by[r['log_input'],r['input_oracles'],r['scheme']]
    actual=grouped[r['log_input'],r['input_oracles'],r['scheme']]
    assert all(s[k]==expected[k] for s in actual for k in ['systematic_h','parity_h'])
write('paper_proof_sizes.csv',paper_proofs)
write('paper_timing_summary.csv',paper_timings)
checks=read(OUT/'fiat_shamir_validation.csv')
assert len(checks)==12 and all(r['trace_matches_model']=='true' and r['proof_verified']=='true' for r in checks)
max_error=max(abs(r['mean_serialized_bytes']/float(by[r['log_input'],r['input_oracles'],r['scheme']]['total_bytes'])-1) for r in summary)
# Provenance includes source identities and all run-level resource records.
paths=['src/bolt_split.rs','src/bolt_multi.rs','src/bolt_batching.rs','src/bolt_fiat_shamir.rs','src/bolt_experiment.rs','examples/gf256_bolt_split.rs','scripts/summarize_gf256_split_suite.py','scripts/run_gf256_split_timings.py','Cargo.lock']
manifest=dict(field='GF(2^256)',hash='BLAKE3',width=4,block_lengths='k for both Canopy layers',seed=0x424f4c5443414e31,
    security_bits=128,query_security_bits='132+log2(eta)',whir_local_security_bits=136,
    fold_limit=3,join_constraint='each side after at least one transition; selected configurations all use exactly one',
    source_sha256={p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in paths},
    timing_samples=240,verified_warmups=48,fs_validations=12,max_relative_size_error=max_error,
    processes=processes)
(OUT/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
lines=['# Independent systematic/parity Bolt batches','',
    'Both Canopy RS block lengths are fixed at k=N/4. Width is 4; inputs are eta independent length-N polynomials. The field, security assumptions, hashes, query counts, and public-H scope match the existing batched suite.', '',
    'The systematic tail batches eta code-switch claims with eta systematic Canopy claims. The parity tail independently batches eta sketch claims with eta parity Canopy claims. Both tails share a domain-separated Fiat–Shamir transcript but use distinct challenges. Each side optimizes its own height and join length. The full-height constant-layer alternatives are included. The search permits several authenticated folds of at most three variables; every selected configuration joins after one fold. Native full cross matrices are retained within each batch.', '',
    'The Merkle baseline is optimized in both shared-tail and split-tail form; the smaller-proof configuration is the baseline at each point. The 100-query-sample proof estimates include verifier authentication, hash-to-field, and Fiat–Shamir finalizations, with transcript bytes separately. Fiat–Shamir adds no proof bytes.', '',
    'All four configurations were rerun in rotating order, with one warmup and five repetitions per (N,eta), eight Rayon threads. All 288 proofs were verified. Timing includes commitment and opening; verification/input generation/setup are excluded. The paper reports the selected Merkle baseline and split Canopy; the WHIR experiment retains its existing dataset.', '',
    f'All 12 full transcript traces match the model. Maximum mean serialized-size deviation from the sampled estimate: {100*max_error:.3f}%.', '',
    '## N = 2^22', '',
    '| eta | Chosen baseline | Baseline proof KiB | Split Canopy proof KiB | Saving | Baseline s | Split Canopy s |',
    '|---:|---|---:|---:|---:|---:|---:|']
for r in comp:
    if r['log_input']==22:
        lines.append(f"| {r['input_oracles']} | {r['baseline_scheme']} | {r['baseline_bytes']/1024:.2f} | {r['split_canopy_bytes']/1024:.2f} | {r['canopy_proof_reduction_vs_baseline_percent']:.1f}% | {r['baseline_median_seconds']:.3f} | {r['split_canopy_median_seconds']:.3f} |")
lines+=['', 'The paper exports `paper_proof_sizes.csv` and `paper_timing_summary.csv` contain 56 size/hash rows and 24 timing rows for its two Bolt configurations.\n\nFiles: `comparison.csv` contains all 28 headline comparisons; `all_proof_sizes.csv` retains all four architectures with verifier hashes and FS bytes; `proof_sizes/independent_height_search.csv` retains every side candidate; `timing_summary.csv` and `timing_stage_summary.csv` contain medians and component means; raw repetitions and process RSS are retained in the timing directories.', '',
    'Reproduce: build `gf256_bolt_split` in release mode without `fs-accounting`, run its `estimate` command, then `python3 scripts/run_gf256_split_timings.py`. Separately build with `--features fs-accounting` and run `gf256_bolt_split validate`. Finally run `python3 scripts/summarize_gf256_split_suite.py`. Do not run other compute-heavy jobs concurrently with timing.', '']
(OUT/'README.md').write_text('\n'.join(lines))
print('\n'.join(lines[-13:]))
