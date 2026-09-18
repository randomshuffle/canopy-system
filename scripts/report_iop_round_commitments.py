#!/usr/bin/env python3
"""Report sequential-IOP commitment costs without altering raw timing samples.

For each (N, scheme), commitment(N, eta) = eta * commitment(N, 1).
Opening measurements and proof/hash estimates remain unchanged. The current
AWS campaign has one measured pass; derived totals are not independent samples.
"""
import argparse
import csv
import hashlib
import json
from pathlib import Path
from public_metadata import public_text

TOTAL_FIELDS = ('mean_prover_seconds', 'median_prover_seconds',
                'min_prover_seconds', 'max_prover_seconds')


def report_rows(rows):
    anchors = {(r['log_input'], r['scheme']): r for r in rows
               if r['input_oracles'] == '1' and r['timing_status'] == 'measured'}
    result = []
    for source in rows:
        row = dict(source)
        row['raw_timing_status'] = source['timing_status']
        for name in ('mean_commitment_seconds', *TOTAL_FIELDS):
            row['raw_' + name] = source.get(name, '')
        row['commitment_timing_method'] = 'eta_times_eta1_same_N_and_scheme'
        row['commitment_anchor_eta'] = '1'
        row['commitment_anchor_seconds'] = ''
        if source['timing_status'] == 'measured':
            assert int(source['repetitions']) == 1, 'This report expects the authorized one-pass campaign'
            anchor = anchors.get((source['log_input'], source['scheme']))
            if anchor is None:
                row['timing_status'] = 'awaiting_eta1_commitment_anchor'
                for name in ('mean_commitment_seconds', *TOTAL_FIELDS):
                    row[name] = ''
            else:
                assert int(anchor['repetitions']) == 1
                c1 = float(anchor['mean_commitment_seconds'])
                eta = int(source['input_oracles'])
                commitment = eta * c1
                opening = float(source['mean_opening_seconds'])
                row['commitment_anchor_seconds'] = c1
                row['mean_commitment_seconds'] = commitment
                for name in TOTAL_FIELDS:
                    row[name] = commitment + opening
                row['timing_status'] = 'measured' if eta == 1 else 'derived_from_measured_components'
        result.append(row)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--input', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    assert args.input.resolve() != args.output.resolve(), 'Preserve the original measured CSV'
    data = args.input.read_bytes()
    with args.input.open() as f:
        rows = list(csv.DictReader(f))
    output = report_rows(rows)
    fields = list(dict.fromkeys(k for r in output for k in r))
    args.output.parent.mkdir(parents=True, exist_ok=True)
    tmp = args.output.with_suffix('.csv.tmp')
    with tmp.open('w', newline='') as f:
        writer = csv.DictWriter(f, fieldnames=fields)
        writer.writeheader()
        writer.writerows(output)
    tmp.replace(args.output)
    provenance = {
        'policy': 'sequential IOP rounds: C(N, eta, scheme) = eta * C(N, 1, scheme)',
        'opening_policy': 'retain each measured opening time; total = scaled commitment + opening',
        'source': public_text(str(args.input.resolve())),
        'source_sha256': hashlib.sha256(data).hexdigest(),
        'reported_points': sum(r['timing_status'] in ('measured', 'derived_from_measured_components') for r in output),
        'note': 'All five configurations. Derived totals have one measured opening sample and an eta=1 commitment anchor; they are not independently timed full runs. Raw timing files and proof/hash columns are unchanged.',
    }
    args.output.with_suffix('.json').write_text(json.dumps(provenance, indent=2)+'\n')
    print(f"Reported {provenance['reported_points']}/{len(rows)} points using sequential-round commitments.")


if __name__ == '__main__':
    main()
