"""Shared I/O and provenance helpers for the current experiments."""
import csv
import hashlib
import json
from pathlib import Path

from public_metadata import public_metadata

ROOT = Path(__file__).resolve().parents[1]
SOURCE_DIRS = ('src', 'examples', 'scripts', 'tests', 'vendor', 'experiments')


def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b''):
            h.update(chunk)
    return h.hexdigest()


def require_writable_campaign_output(out):
    """Reject reference evidence, source directories, and their ancestors."""
    out = out.resolve()
    reference = ROOT / 'results/r7a'
    if out == reference or reference in out.parents or out in reference.parents:
        raise ValueError(f'{out}: retained evidence; select a fresh output directory')
    if ROOT in out.parents and ROOT / 'results' not in out.parents:
        raise ValueError(f'{out}: source directory; write under results/ or outside the checkout')



def source_files():
    files = [ROOT / x for x in ('Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml')]
    for directory in SOURCE_DIRS:
        files.extend(p for p in (ROOT / directory).rglob('*') if p.is_file()
                     and not p.is_symlink() and p.suffix in ('.rs', '.py', '.toml', '.json')
                     and not any(x in p.parts for x in ('.git', 'target', '__pycache__')))
    return sorted(set(files))


def snapshot():
    return {str(p.relative_to(ROOT)): digest(p) for p in source_files()}


def save(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + '.tmp')
    tmp.write_text(json.dumps(public_metadata(data), indent=2) + '\n')
    tmp.replace(path)


def read_csv(path):
    with path.open() as f:
        return list(csv.DictReader(f))


def write_csv(path, rows):
    if not rows:
        raise ValueError(f'no rows for {path}')
    fields = list(dict.fromkeys(k for row in rows for k in row))
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open('w', newline='') as f:
        w = csv.DictWriter(f, fieldnames=fields)
        w.writeheader()
        w.writerows(rows)



def meminfo():
    if not Path('/proc/meminfo').exists():
        return {}
    return {line.split(':')[0]: int(line.split()[1]) * 1024
            for line in Path('/proc/meminfo').read_text().splitlines()}

