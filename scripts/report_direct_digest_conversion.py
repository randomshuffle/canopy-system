#!/usr/bin/env python3
"""Remove the obsolete Canopy rehash from reported verifier work.

Raw campaign data and all prover timings remain unchanged. This reports the
current direct GF(2^256) digest injection against previously sampled geometry;
it is not a new prover or verifier benchmark.
"""
import argparse
import csv
from decimal import Decimal
import hashlib
import json
from pathlib import Path
from public_metadata import public_text


def corrected_rows(rows):
    output = []
    for source in rows:
        row = dict(source)
        assert 'pre_direct_digest_verifier_hashes' not in row, 'Input is already corrected'
        old = Decimal(row['verifier_hashes'])
        auth = Decimal(row['verifier_auth_hashes'])
        h2f = Decimal(row['verifier_hash_to_field'])
        fs = Decimal(row['fs_finalizations'])
        assert abs(old - auth - h2f - fs) < Decimal('0.000001')
        row['pre_direct_digest_verifier_hashes'] = row['verifier_hashes']
        row['pre_direct_digest_verifier_hash_to_field'] = row['verifier_hash_to_field']
        row['verifier_hash_to_field'] = '0'
        row['verifier_hashes'] = str(auth + fs)
        row['digest_to_field_method'] = 'direct_GF256_byte_bijection'
        row['prover_timing_digest_map'] = (
            'measured_before_removing_rehash' if 'canopy' in row['scheme'] else 'not_applicable')
        output.append(row)
    return output


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--input', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    assert args.input.resolve() != args.output.resolve(), 'Preserve the original CSV'
    data = args.input.read_bytes()
    with args.input.open() as stream:
        rows = list(csv.DictReader(stream))
    output = corrected_rows(rows)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with args.output.open('w', newline='') as stream:
        writer = csv.DictWriter(stream, fieldnames=list(output[0]), lineterminator='\n')
        writer.writeheader()
        writer.writerows(output)
    args.output.with_suffix('.json').write_text(json.dumps({
        'source': public_text(str(args.input.resolve())),
        'source_sha256': hashlib.sha256(data).hexdigest(),
        'output_sha256': hashlib.sha256(args.output.read_bytes()).hexdigest(),
        'reported_points': len(output),
        'verifier_hashes': 'authentication + Fiat-Shamir finalizations; digest conversion is zero hashes',
        'proof_sizes': 'unchanged sampled query geometry and parameters',
        'prover_timings': 'unchanged AWS measurements, including the obsolete rehash for Canopy; no rerun',
        'previous_values': 'retained in pre_direct_digest_* columns',
    }, indent=2) + '\n')
    print(f'Corrected verifier hashes for {len(output)} points; proof sizes and all timing fields unchanged.')


if __name__ == '__main__':
    main()
