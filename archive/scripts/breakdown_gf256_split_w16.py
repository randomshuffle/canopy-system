#!/usr/bin/env python3
"""Decompose the saved w=16 Merkle and Canopy k/2 proof estimates; no reruns."""
import csv
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'archive/results/gf256_split_w16_suite'
with (OUT / 'selected_proof_sizes.csv').open() as f:
    proofs = [r for r in csv.DictReader(f) if r['variant'] in ['bolt_merkle_shared', 'canopy_k_half']]
with (OUT / 'half/proof_sizes/independent_height_search.csv').open() as f:
    side_rows = list(csv.DictReader(f))
components = ['systematic_row_values_bytes', 'parity_row_values_bytes',
              'initial_xy_roots_bytes', 'systematic_auth_bytes', 'parity_auth_bytes',
              'bolt_g_and_sigma_bytes', 'whir_reductions_and_batches_bytes']
output = []
for p in proofs:
    n, eta, width = int(p['log_input']), int(p['input_oracles']), int(p['interleaving'])
    canopy = p['variant'] == 'canopy_k_half'
    sides = []
    for side in ['systematic', 'parity']:
        candidates = [r for r in side_rows if int(r['log_input']) == n
            and int(r['input_oracles']) == eta and r['side'] == side
            and r['canopy'] == str(canopy).lower()]
        # Merkle row values and outer authentication are independent of the
        # WHIR join; Canopy authentication uses the selected height.
        if canopy:
            candidates = [r for r in candidates if r['selected'] == 'true']
            assert len(candidates) == 1
        else:
            assert len({r['external_row_bytes'] for r in candidates}) == 1
            assert len({r['external_auth_bytes'] for r in candidates}) == 1
        sides.append(candidates[0])
    roots = 2 * 32 * eta
    g_sigma = (width + 1) * 32 * eta
    r = dict(log_input=n, input_oracles=eta, interleaving=width, variant=p['variant'],
        systematic_queries=int(p['systematic_queries']), parity_queries=int(p['parity_queries']),
        systematic_h=p['systematic_h'], parity_h=p['parity_h'],
        systematic_row_values_bytes=float(sides[0]['external_row_bytes']),
        parity_row_values_bytes=float(sides[1]['external_row_bytes']),
        initial_xy_roots_bytes=roots,
        systematic_auth_bytes=float(sides[0]['external_auth_bytes']) - (0 if canopy else roots / 2),
        parity_auth_bytes=float(sides[1]['external_auth_bytes']) - (0 if canopy else roots / 2),
        bolt_g_and_sigma_bytes=g_sigma,
        whir_reductions_and_batches_bytes=float(p['whir_and_claim_bytes']) - g_sigma - (roots if canopy else 0),
        total_bytes=float(p['total_bytes']), verifier_hashes=float(p['verifier_hashes']),
        fs_proof_bytes=int(p['fs_proof_bytes']), fs_absorb_bytes=float(p['fs_absorb_bytes']),
        fs_squeeze_bytes=float(p['fs_squeeze_bytes']),
        canopy_systematic_whir_bytes=float(sides[0]['whir_and_claim_bytes']) - roots / 2 if canopy else '',
        canopy_parity_whir_bytes=float(sides[1]['whir_and_claim_bytes']) - roots / 2 if canopy else '')
    assert abs(sum(r[k] for k in components) - r['total_bytes']) < 1e-6
    assert abs(sum(float(s['external_auth_bytes']) for s in sides) - float(p['external_auth_bytes'])) < 1e-6
    if canopy:
        assert abs(r['canopy_systematic_whir_bytes'] + r['canopy_parity_whir_bytes'] - r['whir_reductions_and_batches_bytes']) < 1e-6
    output.append(r)
assert len(output) == 56
with (OUT / 'proof_size_breakdown.csv').open('w', newline='') as f:
    w = csv.DictWriter(f, fieldnames=list(output[0]))
    w.writeheader()
    w.writerows(output)
by = {(r['log_input'], r['input_oracles'], r['variant']): r for r in output}
labels = ['Systematic X row values', 'Parity Y row values', 'Initial X/Y commitment roots',
          'Systematic row authentication', 'Parity row authentication', 'Bolt messages g and sigma',
          'WHIR reductions and batched proofs']
lines = ['# Proof-size breakdown: w=16, Merkle vs Canopy k/2', '',
    'These tables decompose the existing 100-query-sample estimates. No prover-time reruns are needed. All proof-size entries are KiB (1,024 bytes); verifier hashes use the existing hash/XOF-finalization unit.', '',
    'At N=2^22, k=N/16=2^18. Both Canopy RS blocks have length k/2=2^17. The optimized heights are h_X=2 and h_Y=3; the Merkle baseline has one shared WHIR batch and Canopy has separate systematic and parity batches.', '',
    '## N=2^22, eta=1', '', '| Component | Merkle KiB | Canopy k/2 KiB | Canopy minus Merkle KiB |',
    '|---|---:|---:|---:|']
m, c = [by[22, 1, v] for v in ['bolt_merkle_shared', 'canopy_k_half']]
for label, column in zip(labels + ['Total'], components + ['total_bytes']):
    lines.append(f"| {label} | {m[column]/1024:,.2f} | {c[column]/1024:,.2f} | {(c[column]-m[column])/1024:+,.2f} |")
lines += [f"| Verifier hashes (count) | {m['verifier_hashes']:,.0f} | {c['verifier_hashes']:,.0f} | {c['verifier_hashes']-m['verifier_hashes']:+,.0f} |", '',
    f"Systematic row values are the actual 16 field elements in each queried row, not authentication paths. Each row occupies 16 x 32 = 512 bytes. The {m['systematic_queries']:,} systematic queries contribute {m['systematic_row_values_bytes']/1024:,.2f} KiB and the {m['parity_queries']:,} parity queries contribute {m['parity_row_values_bytes']/1024:,.2f} KiB. These values are identical in both methods.", '',
    f"Canopy saves {((m['systematic_auth_bytes']+m['parity_auth_bytes'])-(c['systematic_auth_bytes']+c['parity_auth_bytes']))/1024:,.2f} KiB of outer authentication and adds {(c['whir_reductions_and_batches_bytes']-m['whir_reductions_and_batches_bytes'])/1024:,.2f} KiB of WHIR/batching communication, leaving a net saving of {(m['total_bytes']-c['total_bytes'])/1024:,.2f} KiB ({100*(1-c['total_bytes']/m['total_bytes']):.2f}%). The unchanged row values account for {100*(c['systematic_row_values_bytes']+c['parity_row_values_bytes'])/c['total_bytes']:.1f}% of the Canopy proof.", '',
    f"Canopy's WHIR subtotal is {c['canopy_systematic_whir_bytes']/1024:,.2f} KiB for the systematic side and {c['canopy_parity_whir_bytes']/1024:,.2f} KiB for the parity side. Merkle's single shared subtotal is {m['whir_reductions_and_batches_bytes']/1024:,.2f} KiB and cannot be assigned uniquely to one side. These subtotals include prefix reductions, recursive oracle openings/authentication, internal commitment roots, out-of-domain values, sumcheck/terminal messages, and native batching cross evaluations. They exclude the two initial X/Y roots, which have their own row above.", '',
    'Fiat-Shamir contributes zero proof bytes. Its absorbed/squeezed byte counts and verifier hash finalizations remain recorded separately in the CSV; challenges are derived by the verifier. The Bolt messages consist of w field elements in g and one field element sigma per input. Prover estimates exclude serializer framing, matching the existing report.', '',
    '## N=2^22, all input counts', '',
    '| eta | Method | Row values KiB | Initial roots KiB | Outer auth KiB | Bolt messages KiB | WHIR/batching KiB | Total KiB | Verifier hashes |',
    '|---:|---|---:|---:|---:|---:|---:|---:|---:|']
for eta in [1, 2, 4, 8]:
    for v, label in [('bolt_merkle_shared', 'Merkle'), ('canopy_k_half', 'Canopy k/2')]:
        r = by[22, eta, v]
        lines.append(f"| {eta} | {label} | {(r['systematic_row_values_bytes']+r['parity_row_values_bytes'])/1024:,.2f} | {r['initial_xy_roots_bytes']/1024:.2f} | {(r['systematic_auth_bytes']+r['parity_auth_bytes'])/1024:,.2f} | {r['bolt_g_and_sigma_bytes']/1024:.2f} | {r['whir_reductions_and_batches_bytes']/1024:,.2f} | {r['total_bytes']/1024:,.2f} | {r['verifier_hashes']:,.0f} |")
lines += ['', '`proof_size_breakdown.csv` contains the same decomposition for all 7 input lengths and all 4 input counts (56 rows). Every row is checked to sum exactly to the corresponding saved total, up to floating-point precision. The scripts retain the w=16 field width and selected Canopy heights explicitly.', '',
    'Reproduce with `python3 archive/scripts/breakdown_gf256_split_w16.py`.', '']
(OUT / 'proof_size_breakdown.md').write_text('\n'.join(lines))
print('\n'.join(lines[:22]))
