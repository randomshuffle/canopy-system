#!/usr/bin/env python3
"""Check the curated reference manifest and reconstruct every comparison field."""
import json
import tempfile
from pathlib import Path

from benchmark_common import digest
from reproduce import CONFIG, REFERENCE, compare_table, report


def validate(reference=REFERENCE):
    if not __debug__:
        raise ValueError('run without Python -O so validation assertions remain enabled')
    manifest = json.loads((reference / 'checksums.json').read_text())
    actual = {str(p.relative_to(reference)) for p in reference.rglob('*')
              if p.is_file() and p != reference / 'checksums.json'}
    if actual != set(manifest):
        raise ValueError(f'manifest file set differs: missing={set(manifest)-actual}, extra={actual-set(manifest)}')
    for relative, expected in manifest.items():
        if digest(reference / relative) != expected:
            raise ValueError(f'checksum mismatch: {relative}')
    selected = json.loads((reference / 'provenance/retained-files.json').read_text())['files']
    for relative, record in selected.items():
        if digest(reference / relative) != record['sha256']:
            raise ValueError(f'original evidence changed: {relative}')
    cfg = json.loads(CONFIG.read_text())
    with tempfile.TemporaryDirectory(prefix='canopy-report-') as directory:
        output = Path(directory)
        count = report(cfg, reference, Path('.'), output)
        for filename in ('snark_comparison.csv', 'accumulation_comparison.csv', 'proof_sizes_and_hashes.csv'):
            keys = ('log_input', 'input_oracles', 'scheme')
            if filename != 'proof_sizes_and_hashes.csv':
                keys += ('threads',)
            compare_table(output / filename, reference / filename, keys)
    return len(manifest), count


if __name__ == '__main__':
    files, points = validate()
    print(f'Validated {files} files and all {points} reported method points from raw measurements.')
