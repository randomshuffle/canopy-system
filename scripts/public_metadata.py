"""Remove local checkout/home paths from metadata intended for publication."""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def public_text(text):
    """Keep repository-relative paths and hide the current user's home directory."""
    return text.replace(str(ROOT), '.').replace(str(Path.home()), '~')


def public_metadata(value):
    """Sanitize recorded values without changing commands used for execution."""
    if isinstance(value, str):
        return public_text(value)
    if isinstance(value, list):
        return [public_metadata(item) for item in value]
    if isinstance(value, tuple):
        return tuple(public_metadata(item) for item in value)
    if isinstance(value, dict):
        return {public_metadata(key): public_metadata(item) for key, item in value.items()}
    return value


def public_tar_member(member):
    """Exclude filesystem ownership, timestamps, and extended attributes."""
    member.uid = member.gid = member.mtime = 0
    member.uname = member.gname = ''
    member.pax_headers = {}
    return member
