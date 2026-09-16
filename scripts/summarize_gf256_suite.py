#!/usr/bin/env python3
"""Validate the complete GF(2^256) experiment matrix and generate its report."""
import csv, hashlib, json, statistics as st
from collections import defaultdict
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT/'results/gf256_batched_suite'
LOGS = [18,20,22,24,26,28,30]
TIMES = [18,20,22]
BOLT_WIDTH = 4
TS = [1,2,4,8]
WHIR = {
 'merkle_whir_batched_after_second':'WHIR batched 1/2',
 'merkle_whir_batched_rate_quarter_after_second':'WHIR batched 1/4',
 'canopy_schwartz_zippel':'Canopy Exp. 3',
}
BOLT = {'bolt_merkle_batched':'Bolt + Merkle','bolt_canopy_batched':'Bolt + Canopy'}
LAYOUT = {'equal_blocks':'k/4','equal_half':'k/2','equal_full':'k'}
def read(path):
 with path.open() as f:return list(csv.DictReader(f))
def write(path, rows):
 with path.open('w',newline='') as f:
  w=csv.DictWriter(f, fieldnames=list(rows[0]));w.writeheader();w.writerows(rows)
def table(lines, headers, rows):
 lines.extend(['| '+' | '.join(headers)+' |','|'+'|'.join(['---']*len(headers))+'|'])
 lines.extend('| '+' | '.join(map(str,r))+' |' for r in rows);lines.append('')
def f(r,key):return float(r[key])
FS_COLUMNS = ['fs_finalizations','fs_absorb_bytes','fs_squeeze_bytes','fs_proof_bytes','verifier_hashes']
def merge_fs(rows, name, keys):
 sidecar=read(OUT/'fiat_shamir'/name)
 index={tuple(r[k] for k in keys):r for r in sidecar}
 assert len(index)==len(rows)
 for r in rows:
  extra=index[tuple(r[k] for k in keys)]
  for k in FS_COLUMNS:r[k]=extra[k]
  assert f(r,'fs_proof_bytes')==0
  assert abs(f(r,'verifier_hashes')-sum(f(r,k) for k in ['verifier_auth_hashes','verifier_hash_to_field','fs_finalizations']))<1e-6
def bk(r):return (int(r['log_input']),int(r['input_oracles']),r['scheme'],r.get('layout',r.get('canopy_layout')))
def bchoice(r):
 return f"eta={r['input_oracles']}, w=4"+(f", {LAYOUT[r['layout']]}, h={r['systematic_h']}/{r['parity_h']}" if r['systematic_h'] else '')+(f", join=2^{r['aligned_log']}" if r['aligned_log'] else '')
wp = read(OUT/'whir/proof_sizes/hybrid_proof_sizes.csv')
merge_fs(wp,'whir.csv',['log_message_length','input_oracles','scheme'])
write(OUT/'whir/proof_sizes/hybrid_proof_sizes.csv',wp)
assert len(wp)==84
assert {(int(r['log_message_length']),int(r['input_oracles']),r['scheme']) for r in wp}=={(n,t,s) for n in LOGS for t in TS for s in WHIR}
assert all(r['meets_128_bit_rbr']=='true' for r in wp)
for r in wp:
 total=sum(f(r,c) for c in ['mean_hash_bytes','mean_ood_bytes','mean_sumcheck_bytes','mean_batching_bytes','mean_query_answer_bytes','mean_final_coefficient_bytes'])
 assert abs(total-f(r,'mean_total_bytes'))<1e-5
widx={(int(r['log_message_length']),int(r['input_oracles']),r['scheme']):r for r in wp}
qrows=read(OUT/'whir/proof_sizes/oracle_query_breakdown.csv')
qsum=defaultdict(lambda:defaultdict(float))
for r in qrows:
 k=(int(r['log_message_length']),int(r['input_oracles']),r['scheme'])
 for c in ['mean_auth_hashes','mean_answer_fields','verifier_leaf_hashes','verifier_parent_hashes','verifier_auth_hashes','verifier_hash_to_field']:qsum[k][c]+=f(r,c)
for k,r in widx.items():
 assert f(r,'algebraic_rbr_union_bound_bits')>=128
 for c in ['mean_auth_hashes','verifier_leaf_hashes','verifier_parent_hashes','verifier_auth_hashes','verifier_hash_to_field']:
  assert abs(qsum[k][c]-f(r,c))<1e-6,(k,c,qsum[k][c],f(r,c))
 assert abs(qsum[k]['mean_answer_fields']-f(r,'mean_query_answer_fields'))<1e-6,k
bp=read(OUT/'bolt/proof_sizes/proof_sizes.csv');assert len(bp)==112
merge_fs(bp,'bolt.csv',['log_input','input_oracles','interleaving','scheme','layout','systematic_h','parity_h','aligned_log'])
write(OUT/'bolt/proof_sizes/proof_sizes.csv',bp)
bidx={bk(r):r for r in bp};assert len(bidx)==112
assert all(int(r['interleaving'])==BOLT_WIDTH for r in bp)
assert {(int(r['log_input']),int(r['input_oracles'])) for r in bp}=={(n,t) for n in LOGS for t in TS}
for r in bp:assert abs(sum(f(r,k) for k in ['external_row_bytes','external_auth_bytes','whir_and_claim_bytes'])-f(r,'total_bytes'))<1e-5
for r in wp+bp:
 assert abs(f(r,'verifier_leaf_hashes')+f(r,'verifier_parent_hashes')-f(r,'verifier_auth_hashes'))<1e-6
 assert f(r,'verifier_hash_to_field')>=0
best={(n,t,s):min((r for r in bp if int(r['log_input'])==n and int(r['input_oracles'])==t and r['scheme']==s),key=lambda r:f(r,'total_bytes')) for n in LOGS for t in TS for s in BOLT}
write(OUT/'bolt/best_proof_parameters.csv',list(best.values()))
comparison=[]
for n,t in [(n,t) for n in LOGS for t in TS]:
 base=best[n,t,'bolt_merkle_batched']; canopy=best[n,t,'bolt_canopy_batched']
 comparison.append(dict(log_input=n,input_oracles=t,merkle_bytes=f(base,'total_bytes'),canopy_bytes=f(canopy,'total_bytes'),merkle_auth_hashes=f(base,'verifier_auth_hashes'),canopy_auth_hashes=f(canopy,'verifier_auth_hashes'),merkle_h2f=f(base,'verifier_hash_to_field'),canopy_h2f=f(canopy,'verifier_hash_to_field'),proof_saved_percent=100*(1-f(canopy,'total_bytes')/f(base,'total_bytes')),verifier_hashes_saved_percent=100*(1-f(canopy,'verifier_hashes')/f(base,'verifier_hashes')),merkle_verifier_hashes=f(base,'verifier_hashes'),canopy_verifier_hashes=f(canopy,'verifier_hashes'),authentication_hashes_saved_percent=100*(1-f(canopy,'verifier_auth_hashes')/f(base,'verifier_auth_hashes'))))
write(OUT/'bolt/comparison.csv',comparison)

improvements=[]
for r in bp:
 if not r['scheme'].startswith('bolt_canopy'):continue
 baseline=bidx[int(r['log_input']),int(r['input_oracles']),r['scheme'].replace('canopy','merkle'),'equal_blocks']
 improvements.append(dict(**r,matching_merkle_bytes=f(baseline,'total_bytes'),canopy_over_matching_merkle=f(r,'total_bytes')/f(baseline,'total_bytes'),saved_percent=100*(1-f(r,'total_bytes')/f(baseline,'total_bytes'))))
write(OUT/'bolt/matched_input_proof_comparison.csv',improvements)

wt=[];ws=[];bt=[];bs=[];processes=[]
for n in TIMES:
 d=OUT/f'whir/timing_n{n}'
 samples=read(d/'prover_timing_samples.csv');assert len(samples)==60
 by=defaultdict(list)
 for r in samples:
  assert r['field']=='GF(2^256), additive Cantor-subspace RS'
  assert abs(f(r,'commitment_seconds')+f(r,'opening_seconds')-f(r,'prover_seconds'))<1e-8
  by[int(r['input_oracles']),r['scheme']].append(r)
 assert set(by)=={(t,s) for t in TS for s in WHIR}
 for (t,s), rr in by.items():
  assert len(rr)==5
  wt.append(dict(log_input=n,input_polynomials=t,scheme=s,mean_commitment_seconds=st.mean(f(r,'commitment_seconds') for r in rr),mean_opening_seconds=st.mean(f(r,'opening_seconds') for r in rr),mean_total_seconds=st.mean(f(r,'prover_seconds') for r in rr),median_total_seconds=st.median(f(r,'prover_seconds') for r in rr),min_total_seconds=min(f(r,'prover_seconds') for r in rr),max_total_seconds=max(f(r,'prover_seconds') for r in rr)))
 for r in read(d/'prover_timing_stages.csv'):ws.append(dict(log_input=n,**r))
 processes.append(dict(case=f'WHIR n={n}',**json.loads((d/'process.json').read_text())))
 for eta in TS:
  d=OUT/f'bolt/timing_n{n}_eta{eta}'
  rr=read(d/'prover_timings.csv');assert len(rr)==10
  for r in rr:
   assert r['proof_verified']=='true'
   assert abs(f(r,'commitment_seconds')+f(r,'opening_seconds')-f(r,'total_prover_seconds'))<1e-8
  bt.extend(rr);bs.extend(read(d/'prover_timing_stages.csv'))
  processes.append(dict(case=f'Bolt n={n}, eta={eta}, w=4',**json.loads((d/'process.json').read_text())))
assert all(p['exit_code']==0 and p['threads']==8 for p in processes)
wti={(int(r['log_input']),int(r['input_polynomials']),r['scheme']):r for r in wt}
# Stage reconciliation uses arithmetic means, not sums of stage medians.
wg=defaultdict(float)
for r in ws:wg[int(r['log_input']),int(r['input_oracles']),r['scheme']]+=f(r,'mean_seconds')
# The component harness subtracts separately measured tails and clamps
# Duration residuals at zero. Preserve all raw measurements and make the
# resulting overlap visible as a signed mean adjustment, not prover work.
adjustments=[r for r in ws if r['stage']=='separately_measured_component_adjustment']
for k,v in list(wg.items()):
 delta=wti[k]['mean_total_seconds']-v
 if abs(delta)>1e-7:
  template=next(r for r in ws if (int(r['log_input']),int(r['input_oracles']),r['scheme'])==k)
  adjusted=dict(template,stage='separately_measured_component_adjustment',mean_seconds=delta,median_seconds='',percent_of_mean_total=100*delta/wti[k]['mean_total_seconds'])
  ws.append(adjusted);adjustments.append(adjusted);wg[k]+=delta
for k,v in wg.items():assert abs(v-wti[k]['mean_total_seconds'])<1e-7,(k,v,wti[k])
if adjustments:
 affected=sorted({(int(r['log_input']),r['scheme']) for r in adjustments})
 affected_text='; '.join(f"2^{n}, {WHIR[s]}" for n,s in affected)
 max_percent=max(abs(float(r['percent_of_mean_total'])) for r in adjustments)
 adjustment_note=f"The WHIR substage breakdown includes a signed adjustment for separately measured components ({affected_text}; up to {max_percent:.2f}% of total). Commitment/opening/total samples are unchanged. The detailed breakdown shows each adjustment."
else:
 adjustment_note="No signed WHIR component adjustment was needed in this run."
by=defaultdict(list)
for r in bt:by[bk(r)].append(r)
bsummary=[]
for k,rr in sorted(by.items()):
 assert len(rr)==5
 n,t,s,l=k
 r=dict(log_input=n,input_oracles=t,interleaving=BOLT_WIDTH,scheme=s,layout=l,systematic_h=rr[0]['systematic_h'],parity_h=rr[0]['parity_h'],aligned_log=bidx[k]['aligned_log'],mean_commitment_seconds=st.mean(f(r,'commitment_seconds') for r in rr),mean_opening_seconds=st.mean(f(r,'opening_seconds') for r in rr),mean_total_seconds=st.mean(f(r,'total_prover_seconds') for r in rr),median_total_seconds=st.median(f(r,'total_prover_seconds') for r in rr),min_total_seconds=min(f(r,'total_prover_seconds') for r in rr),max_total_seconds=max(f(r,'total_prover_seconds') for r in rr),mean_proof_bytes=st.mean(f(r,'proof_bytes') for r in rr),estimated_proof_bytes=f(bidx[k],'total_bytes'),verifier_auth_hashes=f(bidx[k],'verifier_auth_hashes'),verifier_hash_to_field=f(bidx[k],'verifier_hash_to_field'))
 r.update({c:bidx[k][c] for c in FS_COLUMNS})
 bsummary.append(r)
bti={bk(r):r for r in bsummary}
reconcile=defaultdict(float)
for r in bs:reconcile[bk(r),int(r['iteration']),r['phase']]+=f(r,'seconds')
for r in bt:
 for phase in ['commitment','opening']:assert abs(reconcile[bk(r),int(r['iteration']),phase]-f(r,phase+'_seconds'))<1e-7
write(OUT/'whir/timing_summary.csv',wt);write(OUT/'whir/timing_stages.csv',ws)
write(OUT/'bolt/timing_summary.csv',bsummary);write(OUT/'bolt/timing_samples.csv',bt);write(OUT/'bolt/timing_stages.csv',bs)
lines=['# GF(2^256): WHIR/Canopy and Bolt-max','',
 'All schemes use **GF(2^256), 32-byte elements, BLAKE3-256, eight Rayon threads, and a 128-bit soundness target**. Timings are from Apple M2 with 16 GiB RAM, one warmup and five measured repetitions. Medians headline total prover time; additive breakdowns use arithmetic means. Commitment plus opening equals total. Input generation, public setup, and verification are excluded.','',
 '**Timing boundaries differ between suites.** WHIR/Canopy retains the established component-based harness: independently measured work is scaled for independent polynomials, recursive native WHIR components are executed and verified, and a separately measured suffix is subtracted to model the paper terminal cap. It is not a serialized end-to-end Canopy proof. Bolt timings execute complete serialized, verifier-checked proofs of the public-H adapter. Bolt still excludes Spark/Mulperm, and assumes LDPC distance gamma=0.013.','',
 'Khatam uses the one-and-a-half Johnson radius 1−(1−Δ)^(1/3). For the outer systematic test we use max(γ/3, 1−(1−γ+ε)^(1/3)−3η), with ε=η=2^-32 and assumed γ=0.013; the parity test retains its stronger 1/4 radius. The per-challenge Khatam exceptional probability is at most 2^-192. This is a conditional Bolt adaptation, not a new proof of this adapter. [Khatam, Theorem 1](https://eprint.iacr.org/2024/1843.pdf). Native RS-WHIR parameters retain their existing security bounds.','',
 'Authentication counts are mean BLAKE3 leaf and binary-parent hash calls for the selected proof parameters. They are not the number of digests transmitted and not BLAKE3 compression-block counts. Different leaf widths hash different byte lengths. WHIR size models deduplicate query rows; Bolt models the executable native verifier, which hashes repeated transmitted rows before deduplication. The outer forest verifier hashes paired siblings twice; those calls are included. Received roots and authentication digests themselves cost no hash call.','',
 'Verifier hashes include authentication, H2F, and Fiat–Shamir hash/XOF finalizations. Consecutive squeezes from the same XOF reader count once. FS absorbed and squeezed bytes are reported separately in [the hash breakdown](verifier_hashes.md); these are not compression-call counts. No proof of work is used. Fiat–Shamir adds zero proof bytes: challenges are derived, and all prover messages were already counted. Existing timing samples include the FS work performed by their executed components and are unchanged. WHIR FS is an explicit composed transcript model matching its three-coefficient communication model; Bolt FS is checked against real verifier traces. Cached public protocol/session identifier construction is setup, while their 96-byte absorption is included. See [accounting details](fiat_shamir/README.md).','',
 'The earlier WHIR setup used a 128-bit per-event budget, which gave a 124.5–127.2-bit algebraic union bound across these workloads. This run reserves a 136-bit local budget and validates the composed 128-bit bound. Earlier provisional WHIR numbers are superseded.','',
 '## WHIR proof sizes','',
 'Each cell is proof size (KiB) / verifier hashes. Sizes include roots and all modeled proof components. Each row is the total for t polynomials of 2^n field elements each. The size search optimizes ordered folds in {3,4}, terminal messages of at most 2^10 coefficients. Experiment 3 aligns h with its first fold. The local algebraic/query budget is 136 bits; all 84 selected rows meet the composed 128-bit union-bound check. Means use 100 deterministic query samples.','']
lines[2:2]=['The active suite has exactly three WHIR methods and two Bolt methods, all batched. Over all 28 WHIR size/polynomial-count combinations, rate-1/4 WHIR has the smallest proof. The verifier totals include authentication, H2F, and Fiat–Shamir. The tables pair these costs rather than collapsing them into a single score.','']
for t in TS:
 lines += [f'### t={t}','']
 table(lines,['Input']+list(WHIR.values()),[[f'2^{n}']+[f"{f(widx[n,t,s],'mean_total_kib'):.2f} / {f(widx[n,t,s],'verifier_hashes'):,.0f}" for s in WHIR] for n in LOGS])
lines+=['[Selected parameters and full proof components](whir/proof_sizes/hybrid_proof_sizes.csv) · [All fold/height candidates](whir/proof_sizes/hybrid_parameter_grid.csv) · [Per-oracle queries](whir/proof_sizes/oracle_query_breakdown.csv) · [RBR events](whir/proof_sizes/rbr_security_breakdown.csv).','',
 '## WHIR prover times','',
 'Median total seconds. The timing harness uses a 136-bit local budget, uniform fold 3, Experiment 3 height 4. Consequently these times do not claim to measure every proof-size-optimal mixed folding schedule above.','']
for n in TIMES:
 table(lines,[f'Input 2^{n}; t']+list(WHIR.values()),[[t]+[f"{wti[n,t,s]['median_total_seconds']:.4f}" for s in WHIR] for t in TS])
lines+=[adjustment_note,'',
 '[Mean commitment/opening, medians and ranges](whir/timing_summary.csv) · [Detailed stages](whir/timing_stages.csv). The detailed report below shows the additive phase breakdown.','',
 '## Bolt proof sizes','',
 'Bolt fixes row width w=4. Each row covers eta independent inputs of N=2^n field elements each. Both methods batch across inputs: the Merkle baseline has 2 eta aligned source claims; nonconstant Canopy has 4 eta. One recursive WHIR tail is shared. Canopy searches both-layer RS lengths k/4, k/2, k and every compatible height/join; k=N/4 per input. Each method minimizes its proof size independently for each (N,eta).','',
 'Bolt already proposes batching its two RS proximity proofs (Remark 4.2); Canopy adds its commitment-layer claims to that batch. Functional cross evaluations use the existing native full matrix p(p-1), not the WHIR paper model’s compressed 2p-3 encoding. Each external test receives 132+log2(eta) bits to preserve its union-bound allocation across inputs; native WHIR retains 136 bits.','']
for t in TS:
 lines += [f'### eta={t}','']
 table(lines,['Input']+list(BOLT.values()),[[f'2^{n}']+[f"{f(best[n,t,s],'total_bytes')/1024:.2f} / {f(best[n,t,s],'verifier_hashes'):,.0f}" for s in BOLT] for n in LOGS])
lines+=['Entries are KiB / verifier hashes. Parameters:','']
table(lines,['Input','eta','Method','Block length, heights, join'],[[f'2^{n}',t,BOLT[s],bchoice(best[n,t,s])] for n in LOGS for t in TS for s in BOLT])
lines+=['[All batch sizes/layouts](bolt/proof_sizes/proof_sizes.csv) · [Batched height/join search](bolt/proof_sizes/batching_height_search.csv) · [Selected parameters](bolt/best_proof_parameters.csv) · [Protocol](../../docs/experiment_bolt_max.md).','',
 '[Canopy savings at each matching input count](bolt/matched_input_proof_comparison.csv).','',
 '## Bolt prover times','',
 'Median total seconds at the independently selected proof-minimizing parameters. Every timed proof commits to eta actual, distinct input polynomials and executes one shared recursive proof; these are not scaled single-input timings.','']
for n in TIMES:
 table(lines,[f'Input 2^{n}; eta']+list(BOLT.values()),[[t]+[f"{bti[bk(best[n,t,s])]['median_total_seconds']:.4f}" for s in BOLT] for t in TS])
lines+=['Additive means at those same configurations:','']
table(lines,['Input','eta','Method','Commit s','Opening s','Total s','Actual KiB / modeled verifier hashes'],[[f'2^{n}',t,BOLT[s]]+[f"{bti[bk(best[n,t,s])][c]:.4f}" for c in ['mean_commitment_seconds','mean_opening_seconds','mean_total_seconds']]+[f"{bti[bk(best[n,t,s])]['mean_proof_bytes']/1024:.2f} / {f(best[n,t,s],'verifier_hashes'):,.0f}"] for n in TIMES for t in TS for s in BOLT])
lines+=['[Timing summaries](bolt/timing_summary.csv) · [Raw repetitions](bolt/timing_samples.csv) · [Raw stage measurements](bolt/timing_stages.csv). Actual proofs use one transcript’s query realization and include framing; size estimates average sampled queries and omit framing.','',
 '## Memory and validation','',
 'Peak RSS covers each complete process, including setup, verification, and retained allocator memory across sequential schemes. It is not an individual prover-only allocation bound.','']
table(lines,['Process','Peak GiB','Wall s'],[[p['case'],f"{p['peak_rss_gib']:.3f}",f"{p['wall_seconds']:.1f}"] for p in processes])
lines+=['Validation: 84 WHIR proof-size rows, 112 Bolt proof-size rows, 180 WHIR component timing samples, and 120 serialized verified Bolt timing samples. Bolt raw timing stages reconcile to phase totals; WHIR substages reconcile after the explicit signed component adjustment described below. All proof-size components reconcile to their totals. The 73-test suite passes, including the complete folding-grid composition bound, execution with the increased budget, and exhaustive small-tree authentication accounting. Native WHIR supports zero-polynomial verification without division by zero. Binary arithmetic, additive RS and its transpose, Bolt row checks, batching alignment, tamper rejection, and small-instance serialized byte accounting are tested.','',
 '[Verifier hash breakdown](verifier_hashes.md) · [Detailed additive time breakdowns](timing_breakdown.md) · [Reproduction commands and executable hashes](run_manifest.json). Historical prime-field measurements remain in their original directories.','']
(OUT/'comparison.md').write_text('\n'.join(lines))
detail=['# GF(2^256) prover-time breakdown','', 'Arithmetic means in seconds; phase means add to total means. Every measured repetition remains in the raw CSV files. WHIR is a component estimator: separately measured tail subtraction and clamped duration residuals can make substages overlap. The signed separately_measured_component_adjustment reconciles these substages to the unchanged reported total; it is a measurement-model adjustment, not negative prover work. Its median is unavailable when recovered from saved stage aggregates. Bolt stages partition each measured execution directly and need no adjustment.','', '## WHIR: commitment and opening','']
table(detail,['n','t','Scheme','Commit','Opening','Total','Median total'],[[r['log_input'],r['input_polynomials'],WHIR[r['scheme']]]+[f"{r[c]:.5f}" for c in ['mean_commitment_seconds','mean_opening_seconds','mean_total_seconds','median_total_seconds']] for r in sorted(wt,key=lambda r:(r['log_input'],r['input_polynomials'],list(WHIR).index(r['scheme'])))])
detail+=['## WHIR: measured stages','']
table(detail,['n','t','Scheme','Stage','Mean seconds'],[[r['log_input'],r['input_oracles'],WHIR[r['scheme']],r['stage'],f"{f(r,'mean_seconds'):.6f}"] for r in ws])
detail+=['## Bolt: all configurations','']
table(detail,['n','eta','Scheme','RS block','h X/Y','Commit','Opening','Total','Median total'],[[r['log_input'],r['input_oracles'],BOLT[r['scheme']],LAYOUT[r['layout']] if 'canopy' in r['scheme'] else '-',f"{r['systematic_h']}/{r['parity_h']}" if r['systematic_h'] else '-']+[f"{r[c]:.5f}" for c in ['mean_commitment_seconds','mean_opening_seconds','mean_total_seconds','median_total_seconds']] for r in bsummary])
detail+=['## Bolt: stages at the smallest-proof configurations','']
sg=defaultdict(list)
for r in bs:
 if bk(r)==bk(best[int(r['log_input']),int(r['input_oracles']),r['scheme']]):sg[int(r['log_input']),int(r['input_oracles']),r['scheme'],r['phase'],r['part'],r['stage']].append(f(r,'seconds'))
table(detail,['n','eta','Scheme','Phase','Part','Stage','Mean seconds'],[[n,t,BOLT[s],phase,part,stage,f'{st.mean(v):.6f}'] for (n,t,s,phase,part,stage),v in sorted(sg.items())])
(OUT/'timing_breakdown.md').write_text('\n'.join(detail))
hashes=['# Verifier hash work','',
 'Verifier hashes = authentication (leaf + parent) + H2F + Fiat–Shamir hash/XOF finalizations. A long XOF stream counts as one finalization until new data are absorbed. Input/output bytes are reported separately; these totals are not compression calls or verifier-time estimates. FS adds zero proof bytes; no proof of work is used. Cached public protocol/session ID construction is setup, while their 96-byte absorption is included. WHIR uses a composed transcript model; Bolt is trace-validated. See [the accounting specification and validation](fiat_shamir/README.md).','', '## WHIR','']
components=['verifier_leaf_hashes','verifier_parent_hashes','verifier_auth_hashes','verifier_hash_to_field','fs_finalizations','verifier_hashes']
headers=['Leaf','Parent','Authentication','H2F','FS','Verifier hashes','FS absorb KiB','FS squeeze KiB']
def hash_values(r):return [f"{f(r,c):.2f}" for c in components]+[f"{f(r,c)/1024:.3f}" for c in ['fs_absorb_bytes','fs_squeeze_bytes']]
table(hashes,['log N','t','Scheme','KiB']+headers,[[r['log_message_length'],r['input_oracles'],WHIR[r['scheme']],f"{f(r,'mean_total_kib'):.2f}"]+hash_values(r) for r in wp])
hashes+=['## Bolt: input counts and layouts, w=4','']
table(hashes,['log N','eta','Scheme','RS blocks','h X/Y','KiB']+headers,[[r['log_input'],r['input_oracles'],BOLT[r['scheme']],LAYOUT[r['layout']] if 'canopy' in r['scheme'] else '-',f"{r['systematic_h']}/{r['parity_h']}",f"{f(r,'total_bytes')/1024:.2f}"]+hash_values(r) for r in bp])
(OUT/'verifier_hashes.md').write_text('\n'.join(hashes))
sources = list((ROOT/'src').glob('*.rs')) + list((ROOT/'vendor/whir/src').rglob('*.rs')) + [ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'examples/gf256_bolt_suite.rs',ROOT/'examples/fiat_shamir_accounting.rs'] + [ROOT/'scripts'/n for n in ['measure_gf256_run.py','run_gf256_timings.py','summarize_gf256_suite.py']]
manifest = dict(field='GF(2^256)', provenance_note='The retained WHIR proof estimates were generated before the transcript-only SHAKE-to-BLAKE3 change; Bolt estimates and timings were rerun for fixed w=4 and eta=1,2,4,8. Their deterministic query geometry and size/hash models are unaffected; all final timing runs use the BLAKE3 executable. The final Rust stage reporter additionally emits a signed adjustment after timing; this report-only change does not change measured work, and the summarizer applies it to the saved raw aggregates. Each process records its actual binary hash.', active_whir_schemes=list(WHIR), whir_local_security_bits=136, active_bolt_schemes=list(BOLT), bolt_width=4, bolt_input_oracles=TS, bolt_proximity=dict(source='https://eprint.iacr.org/2024/1843.pdf', assumption='Khatam distance preservation applied to the public-H Bolt adapter', gamma=0.013, epsilon=2**-32, eta=2**-32, systematic_radius=max(0.013/3,1-(1-0.013+2**-32)**(1/3)-3*2**-32), parity_radius=0.25, eta1_systematic_query_budget=20977, eta1_parity_query_budget=319, query_security_bits='132 + log2(input_oracles)', native_whir_security_bits=136), transcript='BLAKE3 XOF in both executable suites; WHIR previously used the native SHAKE128 default', whir_component_adjustments=adjustments, verifier_hash_metric='Authentication + H2F + Fiat-Shamir hash/XOF finalizations; FS bytes separate; no PoW', fiat_shamir=dict(command='cargo run --offline --release --features fs-accounting --example fiat_shamir_accounting -- --validate', validation='fiat_shamir/bolt_validation.csv', model='fiat_shamir/README.md', extra_proof_bytes=0, accounting_changes_timing_samples=False, accounting_sha256={str(p.relative_to(OUT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted((OUT/'fiat_shamir').glob('*')) if p.is_file()}), source_sha256={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sources}, estimates=[json.loads((OUT/'whir/proof_sizes/process.json').read_text()),json.loads((OUT/'bolt/proof_sizes/process.json').read_text())], timings=processes)
(OUT/'run_manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print('Validated all 300 timing samples and 196 selected proof-size rows.')
