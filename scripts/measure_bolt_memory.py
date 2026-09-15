#!/usr/bin/env python3
"""Measure whole-process peak RSS for a verified Bolt/Canopy timing run."""

import argparse
import json
import os
from pathlib import Path
import resource
import subprocess
import sys
import time


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--log-length", type=int, required=True)
    parser.add_argument("--interleaving", type=int, default=4)
    parser.add_argument("--canopy-layout", default="equal-full")
    parser.add_argument("--warmups", type=int, default=2)
    parser.add_argument("--repetitions", type=int, default=10)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--binary", type=Path, default=Path("target/release/canopy-system"))
    args = parser.parse_args()
    args.output_dir.mkdir(parents=True, exist_ok=True)
    command = [str(args.binary.resolve()), "bolt-time", "--canopy-layout", args.canopy_layout,
               "--interleaving", str(args.interleaving), "--log-length", str(args.log_length),
               "--warmups", str(args.warmups), "--repetitions", str(args.repetitions),
               "--height-search-samples", "100", "--output-dir", str(args.output_dir)]
    # This fresh Python process launches only one child, so RUSAGE_CHILDREN's
    # maximum describes this benchmark. On Darwin maxrss is bytes; Linux uses KiB.
    if sys.platform not in ("darwin", "linux"):
        raise RuntimeError(f"Unsupported ru_maxrss units on {sys.platform}")
    start = time.monotonic()
    result = subprocess.run(command, env={**os.environ, "RAYON_NUM_THREADS": "8"})
    elapsed = time.monotonic() - start
    usage = resource.getrusage(resource.RUSAGE_CHILDREN)
    peak_bytes = int(usage.ru_maxrss) * (1 if sys.platform == "darwin" else 1024)
    report = dict(command=command, platform=sys.platform, exit_code=result.returncode,
                  log_input=args.log_length, interleaving=args.interleaving,
                  canopy_layout=args.canopy_layout, warmups=args.warmups,
                  repetitions=args.repetitions, rayon_threads=8,
                  input_bytes=32 * (1 << args.log_length), peak_rss_bytes=peak_bytes,
                  peak_rss_gib=peak_bytes / 2**30, wall_seconds=elapsed,
                  user_seconds=usage.ru_utime, system_seconds=usage.ru_stime,
                  major_faults=usage.ru_majflt, minor_faults=usage.ru_minflt,
                  boundary="Whole benchmark process: height search, public H setup, input generation, alternating Merkle/Canopy proving and verification; not prover-only or separate per-scheme peaks.")
    (args.output_dir / "process_memory.json").write_text(json.dumps(report, indent=2) + "\n")
    print(f"Peak RSS: {peak_bytes/2**30:.3f} GiB; wall time {elapsed:.2f}s; exit {result.returncode}", flush=True)
    raise SystemExit(result.returncode)


if __name__ == "__main__":
    main()
