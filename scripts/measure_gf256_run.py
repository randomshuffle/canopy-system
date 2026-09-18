#!/usr/bin/env python3
"""Run one suite command, preserving exact executable identity, logs, and child RSS."""
import argparse, hashlib, json, os, platform, subprocess, sys, time
from pathlib import Path
from public_metadata import public_metadata, public_text
p = argparse.ArgumentParser(description=__doc__)
p.add_argument('--record-dir', type=Path, required=True)
p.add_argument('--threads', type=int, default=8)
p.add_argument('command', nargs=argparse.REMAINDER)
a = p.parse_args()
command = a.command[1:] if a.command[:1] == ['--'] else a.command
if not command: p.error('a command is required')
a.record_dir.mkdir(parents=True, exist_ok=True)
exe = Path(command[0]).resolve()
binary_hash = hashlib.sha256(exe.read_bytes()).hexdigest()
started = time.time()
with (a.record_dir/'run.log').open('w') as log:
    child = subprocess.Popen(command, env={**os.environ, 'RAYON_NUM_THREADS':str(a.threads)}, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    for line in child.stdout:
        line = public_text(line)
        print(line, end='', flush=True); log.write(line); log.flush()
    _, status, usage = os.wait4(child.pid, 0)
    child.returncode = os.waitstatus_to_exitcode(status)
peak = usage.ru_maxrss * (1 if sys.platform == 'darwin' else 1024)
report = dict(field='GF(2^256)', command=command, binary_sha256=binary_hash, threads=a.threads, platform=platform.platform(), started_unix=started, wall_seconds=time.time()-started, exit_code=child.returncode, peak_rss_bytes=peak, peak_rss_gib=peak/2**30, user_seconds=usage.ru_utime, system_seconds=usage.ru_stime, memory_boundary='Whole child process, including setup and verification; schemes run sequentially.')
(a.record_dir/'process.json').write_text(json.dumps(public_metadata(report),indent=2)+'\n')
print(f'Completed: exit={child.returncode}, wall={report["wall_seconds"]:.2f}s, peak={peak/2**30:.3f} GiB', flush=True)
sys.exit(child.returncode)
