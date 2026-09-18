#!/usr/bin/env python3
"""Run one field-comparison process, recording its command, binary hash and RSS."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import resource
import subprocess
import sys
import time
from public_metadata import public_metadata, public_text

p=argparse.ArgumentParser(description=__doc__)
p.add_argument('--field',choices=['prime','binary'],required=True)
p.add_argument('--log-length',type=int,default=22)
p.add_argument('--warmups',type=int,default=1)
p.add_argument('--repetitions',type=int,default=5)
p.add_argument('--threads',type=int,default=8)
p.add_argument('--output-dir',type=Path,required=True)
a=p.parse_args()
if a.field == 'prime':
    p.error('The main executable now uses GF(2^256). Prime timings require the archived pre-migration revision; this command will not mislabel a binary-field run.')
root=Path(__file__).resolve().parents[1]
binary=root/('target/release/canopy-system' if a.field=='prime' else 'experiments/binary-whir/target/release/canopy-binary-whir')
command=[str(binary)]
if a.field=='prime': command+=['paper-time']
command+=['--log-length',str(a.log_length),'--warmups',str(a.warmups),'--repetitions',str(a.repetitions),'--seed',str(0x43414e4f5059),'--output-dir',str(a.output_dir.resolve())]
if a.field=='binary': command+=['--threads',str(a.threads)]
a.output_dir.mkdir(parents=True,exist_ok=True)
start=time.monotonic()
with (a.output_dir/'run.log').open('w') as log:
    process=subprocess.Popen(command,cwd=root,env={**os.environ,'RAYON_NUM_THREADS':str(a.threads)},stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True)
    for line in process.stdout:
        line=public_text(line)
        print(line,end='',flush=True);log.write(line);log.flush()
    code=process.wait()
usage=resource.getrusage(resource.RUSAGE_CHILDREN)
peak=usage.ru_maxrss*(1 if sys.platform=='darwin' else 1024)
report=dict(field=a.field,command=command,binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),platform=platform.platform(),machine=platform.machine(),threads=a.threads,seed=0x43414e4f5059,log_length=a.log_length,input_polynomials=[1,2,4,8],warmups=a.warmups,repetitions=a.repetitions,exit_code=code,wall_seconds=time.monotonic()-start,peak_rss_bytes=peak,peak_rss_gib=peak/2**30,user_seconds=usage.ru_utime,system_seconds=usage.ru_stime,major_faults=usage.ru_majflt,minor_faults=usage.ru_minflt,memory_boundary='Whole process, all schemes sequentially, including setup and verification; not a per-scheme prover peak.')
(a.output_dir/'process.json').write_text(json.dumps(public_metadata(report),indent=2)+'\n')
print(f'Field {a.field}: exit {code}, peak RSS {peak/2**30:.3f} GiB, wall {report["wall_seconds"]:.1f}s',flush=True)
sys.exit(code)
