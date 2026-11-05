#!/usr/bin/env python3
"""Profile Section 8 configurations in sequential, fresh child processes.

Each child measures one configuration and one scheme. Canopy emits both ifold
and iacc; the two ordinary Merkle controls run in separate children. The runner
preserves the binary's CSV schema and records whole-child resource usage.
"""

import argparse
import csv
import hashlib
import json
import math
import os
import platform
import signal
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

from public_metadata import public_metadata, public_text
from section8_workload import CURRENT_WORKLOAD, validate_sources, workload_name


ROOT = Path(__file__).resolve().parents[1]
SCHEMES = ("canopy", "merkle-half", "merkle-quarter")
OUTPUT_SCHEMES = {
    "canopy": {"ifold", "iacc"},
    "merkle-half": {"merkle_rate_half"},
    "merkle-quarter": {"merkle_rate_quarter"},
}
REQUIRED_COLUMNS = {
    "workload",
    "log_n", "ell_cast", "ell_acc", "height", "log_leaf_width",
    "security_bits", "scheme", "repetition", "input_seed", "sources",
    "queries", "step_seconds", "incoming_commit_seconds", "form_seconds",
    "sumcheck_seconds", "combination_seconds", "output_commit_seconds",
    "opening_seconds", "auth_hashes", "fs_finalizations", "proof_bytes",
    "proof_hint_bytes", "proof_field_bytes", "proof_hash_bytes",
    "proof_framing_bytes", "output_instance_bytes", "serialized_total_bytes",
    "verified",
    "fresh_commit_sources", "fresh_commit_seconds", "prior_commit_seconds",
    "fresh_encoding_seconds", "fresh_merkle_leaf_seconds", "fresh_merkle_parent_seconds",
    "fresh_h2f_seconds", "fresh_ood_seconds", "fresh_other_seconds",
}
FRESH_PHASES = ("fresh_encoding_seconds", "fresh_merkle_leaf_seconds", "fresh_merkle_parent_seconds", "fresh_h2f_seconds", "fresh_ood_seconds", "fresh_other_seconds")
COST_SCOPE = "fresh_commit_plus_reduction_v1"

def csv_integers(value):
    try:
        numbers = [int(part) for part in value.split(",")]
    except ValueError as error:
        raise argparse.ArgumentTypeError("expected comma-separated integers") from error
    if not numbers or len(numbers) != len(set(numbers)):
        raise argparse.ArgumentTypeError("expected a nonempty list without duplicates")
    return numbers


def csv_schemes(value):
    names = value.split(",")
    if not names or len(names) != len(set(names)) or any(name not in SCHEMES for name in names):
        raise argparse.ArgumentTypeError("expected distinct schemes from " + ",".join(SCHEMES))
    return names


def parser():
    result = argparse.ArgumentParser(description=__doc__)
    result.add_argument("--binary", type=Path, default=ROOT / "target/release/examples/accumulation_section8")
    result.add_argument("--output-dir", type=Path, required=True, help="new directory; existing directories are refused")
    result.add_argument("--logs", type=csv_integers, default=csv_integers("18,20,22"))
    result.add_argument("--ell-cast", type=csv_integers, default=csv_integers("1,2,4,8"))
    result.add_argument("--ell-acc", type=csv_integers, default=csv_integers("1"))
    result.add_argument("--schemes", type=csv_schemes, default=list(SCHEMES))
    result.add_argument("--threads", type=int, default=8)
    result.add_argument("--warmups", type=int, default=1)
    result.add_argument("--repetitions", type=int, default=2)
    result.add_argument("--seed", type=int, default=91)
    result.add_argument("--log-leaf-width", type=int, default=3)
    result.add_argument("--security-bits", type=int, default=128)
    result.add_argument("--objective", choices=("total", "ifold"), default="total")
    result.add_argument("--height", type=int, help="explicit eligible Canopy height instead of optimization")
    result.add_argument("--check-tampering", action="store_true")
    return result


def validate_arguments(args):
    if args.threads <= 0 or args.repetitions <= 0 or args.warmups < 0:
        raise ValueError("threads/repetitions must be positive and warmups nonnegative")
    if any(n < 1 or n > 32 for n in args.logs):
        raise ValueError("logs must be between 1 and 32")
    if any(n <= 0 for n in args.ell_cast) or any(n < 0 for n in args.ell_acc):
        raise ValueError("ell-cast must be positive and ell-acc nonnegative")
    if args.log_leaf_width < 0 or args.log_leaf_width > min(args.logs):
        raise ValueError("log-leaf-width must lie between zero and every message log")
    if args.security_bits <= 0 or not 0 <= args.seed < 2**64:
        raise ValueError("security-bits must be positive and seed must fit u64")
    if args.height is not None and args.height < 1:
        raise ValueError("height must be positive")
    if not hasattr(os, "wait4"):
        raise ValueError("this resource-measuring runner requires os.wait4")
    args.binary = args.binary.expanduser().resolve()
    args.output_dir = args.output_dir.expanduser().resolve()
    if not args.binary.is_file() or not os.access(args.binary, os.X_OK):
        raise ValueError(f"binary is missing or not executable: {args.binary}")
    if args.output_dir.exists():
        raise ValueError(f"refusing existing output directory: {args.output_dir}")


def digest(path):
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(block)
    return value.hexdigest()


def source_snapshot():
    paths = set()
    for folder in ("src", "examples", "vendor/whir/src"):
        paths.update((ROOT / folder).rglob("*.rs"))
    for relative in ("Cargo.toml", "Cargo.lock", "vendor/whir/Cargo.toml",
                     "vendor/whir/Cargo.lock", "scripts/profile_section8_campaign.py",
                     "scripts/public_metadata.py", "scripts/section8_workload.py",
                     "scripts/report_section8.py", "scripts/report_section8_profile.py"):
        path = ROOT / relative
        if path.is_file():
            paths.add(path)
    return {path.relative_to(ROOT).as_posix(): digest(path) for path in sorted(paths)}


def command_output(command):
    try:
        output = subprocess.run(command, cwd=ROOT, check=True, stdout=subprocess.PIPE,
                                stderr=subprocess.DEVNULL, text=True).stdout.strip()
        return public_text(output)
    except (OSError, subprocess.CalledProcessError):
        return None


def hardware():
    details = {"architecture": platform.machine(), "logical_cores": os.cpu_count()}
    if sys.platform == "darwin":
        details["cpu_brand"] = command_output(["sysctl", "-n", "machdep.cpu.brand_string"])
        for key, sysctl_name in (("physical_cores", "hw.physicalcpu"), ("physical_memory_bytes", "hw.memsize")):
            value = command_output(["sysctl", "-n", sysctl_name])
            details[key] = int(value) if value else None
    else:
        details["cpu_brand"] = platform.processor() or None
        try:
            details["physical_memory_bytes"] = os.sysconf("SC_PAGE_SIZE") * os.sysconf("SC_PHYS_PAGES")
        except (OSError, ValueError):
            details["physical_memory_bytes"] = None
    return details


def write_json(path, value):
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(json.dumps(public_metadata(value), indent=2, sort_keys=True) + "\n")
    temporary.replace(path)


def utc_now():
    return datetime.now(timezone.utc).isoformat()


def parameters(args):
    return {
        "logs": args.logs, "ell_cast": args.ell_cast, "ell_acc": args.ell_acc,
        "schemes": args.schemes, "log_leaf_width": args.log_leaf_width,
        "security_bits": args.security_bits, "height_objective": args.objective,
        "height_override": args.height, "seed": args.seed,
        "threads": args.threads, "warmups": args.warmups,
        "repetitions": args.repetitions, "check_tampering": args.check_tampering,
    }


def proof_configuration(args):
    return {
        "workload": CURRENT_WORKLOAD,
        "cost_scope": COST_SCOPE,
        "fresh_commit_sources": "ell_cast for ifold and each Merkle control; zero for iacc",
        "total_prover_scope": "ell_cast fresh commitments + ifold + iacc for Canopy; ell_cast fresh commitments + reduction for each Merkle control; excludes cached priors, claim preparation, verification and final decider",
        "ifold_sources": "ell_cast fresh Canopy + 1 prior Canopy",
        "ifold_residuals": "ell_cast + 1 Merkle SEP claims",
        "iacc_sources": "ell_cast + 1 residuals + ell_acc prior Merkle",
        "merkle_control_sources": "ell_cast fresh full-length + ell_acc prior full-length",
        "field": "GF(2^256)", "hash": "BLAKE3-256",
        "leaf_width_field_elements": 1 << args.log_leaf_width,
        "local_security_bits": args.security_bits,
        "security_scope": "local configuration budget; no composed-security certificate",
        "rs_regime": "Johnson bound",
        "johnson_radius": "1 - 1.05 sqrt(rate)",
        "canopy_input_rate": "1/2", "canopy_block_length_divisor": 2,
        "merkle_control_rates": {"merkle-half": "1/2", "merkle-quarter": "1/4"},
        "height_objective": "expected ifold+iacc verifier hashes" if args.objective == "total" else "expected ifold verifier hashes",
        "proof_boundary": "proof_bytes includes field messages and hints, excludes returned output-instance roots and Canopy layer OOD; serialized_total_bytes also includes those output-instance messages",
        "public_inputs": "roots and compact SEP statements transcript-bound as public inputs, excluded from proof bytes",
        "prior_claims": "one independently prepared valid compact Batch Canopy statement and ell_acc compact Batch Merkle statements outside timers; iacc additionally consumes actual ifold residual statements; cross-step reuse is exercised by protocol tests",
    }


def child_command(args, log_n, ell_cast, ell_acc, scheme, csv_path):
    command = [str(args.binary), "time", "--logs", str(log_n),
               "--ell-cast", str(ell_cast), "--ell-acc", str(ell_acc),
               "--schemes", scheme, "--log-leaf-width", str(args.log_leaf_width),
               "--security-bits", str(args.security_bits), "--objective", args.objective,
               "--seed", str(args.seed), "--warmups", str(args.warmups),
               "--repetitions", str(args.repetitions), "--output", str(csv_path)]
    if args.height is not None:
        command.extend(("--height", str(args.height)))
    if args.check_tampering:
        command.append("--check-tampering")
    return command


def canonical_scheme(value):
    name = value.lower().replace("-", "_")
    return {"ifold_canopy": "ifold", "canopy_ifold": "ifold",
            "iacc_merkle": "iacc", "canopy_iacc": "iacc",
            "merkle_half": "merkle_rate_half", "merkle_quarter": "merkle_rate_quarter"}.get(name, name)


def validate_csv(path, args, log_n, ell_cast, ell_acc, scheme, expected_header):
    with path.open(newline="") as stream:
        reader = csv.DictReader(stream)
        header = reader.fieldnames
        if header is None or len(header) != len(set(header)):
            raise ValueError("missing or duplicate CSV columns")
        missing = REQUIRED_COLUMNS - set(header)
        if missing:
            raise ValueError("missing required CSV columns: " + ", ".join(sorted(missing)))
        if expected_header is not None and header != expected_header:
            raise ValueError("CSV schema differs from an earlier child")
        rows = list(reader)
    expected_names = OUTPUT_SCHEMES[scheme]
    expected_ids = {(name, repetition) for name in expected_names for repetition in range(args.repetitions)}
    seen = set()
    heights = set()
    for row in rows:
        if None in row or any(value is None or value == "" for value in row.values()):
            raise ValueError("malformed or incomplete CSV row")
        if row["verified"].lower() not in ("true", "1"):
            raise ValueError("child emitted an unverified reduction")
        name = canonical_scheme(row["scheme"])
        repetition = int(row["repetition"])
        identity = (name, repetition)
        if identity not in expected_ids or identity in seen:
            raise ValueError(f"unexpected or duplicate sample: {identity}")
        seen.add(identity)
        for key, value in (("log_n", log_n), ("ell_cast", ell_cast), ("ell_acc", ell_acc),
                           ("log_leaf_width", args.log_leaf_width), ("security_bits", args.security_bits)):
            if int(row[key]) != value:
                raise ValueError(f"sample has wrong {key}")
        seed = (args.seed + args.warmups + repetition) % 2**64
        if int(row["input_seed"]) != seed:
            raise ValueError("sample has wrong input seed")
        if workload_name(row["workload"]) != CURRENT_WORKLOAD:
            raise ValueError("child must measure the steady_state_v2 workload")
        validate_sources(CURRENT_WORKLOAD, name, ell_cast, ell_acc, row["sources"])
        expected_fresh = 0 if name == "iacc" else ell_cast
        if int(row["fresh_commit_sources"]) != expected_fresh:
            raise ValueError("sample has wrong fresh commitment source count")
        height = int(row["height"])
        if scheme == "canopy":
            if height < 1 or (args.height is not None and height != args.height):
                raise ValueError("sample has wrong Canopy height")
            heights.add(height)
        elif height != 0:
            raise ValueError("Merkle sample has nonzero Canopy height")
        for key, value in row.items():
            if key.endswith("_seconds"):
                duration = float(value)
                if not math.isfinite(duration) or duration < 0:
                    raise ValueError(f"invalid duration in {key}")
        fresh = float(row["fresh_commit_seconds"])
        prior = float(row["prior_commit_seconds"])
        if not math.isclose(float(row["incoming_commit_seconds"]), fresh + prior, rel_tol=1e-6, abs_tol=5e-9):
            raise ValueError("incoming commitment time does not equal fresh plus prior time")
        if not math.isclose(fresh, sum(float(row[key]) for key in FRESH_PHASES), rel_tol=1e-6, abs_tol=5e-9):
            raise ValueError("fresh commitment phases do not reconcile")
        if not expected_fresh and (fresh != 0 or any(float(row[key]) != 0 for key in FRESH_PHASES)):
            raise ValueError("iacc has unexpected fresh commitment work")
        for key in ("queries", "auth_hashes", "fs_finalizations", "proof_bytes", "proof_hint_bytes",
                    "proof_field_bytes", "proof_hash_bytes", "proof_framing_bytes", "output_instance_bytes",
                    "serialized_total_bytes"):
            if int(row[key]) < 0:
                raise ValueError(f"negative {key}")
        if int(row["proof_bytes"]) != sum(int(row[key]) for key in ("proof_field_bytes", "proof_hash_bytes", "proof_framing_bytes")):
            raise ValueError("proof payload accounting does not reconcile")
        if int(row["serialized_total_bytes"]) != int(row["proof_bytes"]) + int(row["output_instance_bytes"]):
            raise ValueError("serialized proof accounting does not reconcile")
    if seen != expected_ids or len(heights) > 1:
        raise ValueError("missing samples or inconsistent optimized heights")
    return header, rows


def run_child(args, identity, log_n, ell_cast, ell_acc, scheme, expected_header):
    folder = args.output_dir / "jobs" / f"n{log_n}_cast{ell_cast}_acc{ell_acc}" / scheme
    folder.mkdir(parents=True)
    csv_path = folder / "timings.csv"
    command = child_command(args, log_n, ell_cast, ell_acc, scheme, csv_path)
    report = {
        **identity, "command": command, "cwd": str(ROOT),
        "configuration": {"log_n": log_n, "ell_cast": ell_cast, "ell_acc": ell_acc,
                          "scheme": scheme, "log_leaf_width": args.log_leaf_width,
                          "security_bits": args.security_bits, "threads": args.threads,
                          "warmups": args.warmups, "repetitions": args.repetitions,
                          "seed": args.seed, "height_objective": args.objective,
                          "height_override": args.height, "check_tampering": args.check_tampering},
        "status": "running", "started_unix": time.time(), "started_utc": utc_now(),
        "memory_boundary": "Whole fresh child, including initialization, input preparation, warmup, two reductions for Canopy, all measured repetitions, verification, and final witness checks; children run sequentially",
        "fault_boundary": "os.wait4 whole-child rusage; page faults are not a direct swap measurement",
    }
    record_path = folder / "process.json"
    write_json(record_path, report)
    print(public_text(f"Starting n={log_n} cast={ell_cast} acc={ell_acc} {scheme}: {' '.join(command)}"), flush=True)
    start = time.monotonic()
    child = None
    interrupted = False
    usage = None
    try:
        with (folder / "run.log").open("w") as log:
            child = subprocess.Popen(command, cwd=ROOT, env={**os.environ, "RAYON_NUM_THREADS": str(args.threads)},
                                     stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
            try:
                _, status, usage = os.wait4(child.pid, 0)
            except KeyboardInterrupt:
                interrupted = True
                try:
                    os.killpg(child.pid, signal.SIGTERM)
                except ProcessLookupError:
                    pass
                _, status, usage = os.wait4(child.pid, 0)
            child.returncode = os.waitstatus_to_exitcode(status)
        log_path = folder / "run.log"
        log_path.write_text(public_text(log_path.read_text(errors="replace")))
        peak = usage.ru_maxrss * (1 if sys.platform == "darwin" else 1024)
        report.update(
            exit_code=child.returncode, peak_rss_bytes=peak, peak_rss_gib=peak / 2**30,
            user_seconds=usage.ru_utime, system_seconds=usage.ru_stime,
            minor_page_faults=usage.ru_minflt, major_page_faults=usage.ru_majflt,
            swaps=usage.ru_nswap, voluntary_context_switches=usage.ru_nvcsw,
            involuntary_context_switches=usage.ru_nivcsw,
        )
        if interrupted:
            raise KeyboardInterrupt
        if child.returncode != 0:
            raise RuntimeError(f"child exited with status {child.returncode}; see {folder / 'run.log'}")
        if digest(args.binary) != identity["binary_sha256"] or source_snapshot() != identity["source_sha256"]:
            raise RuntimeError("binary or source changed during the profiling job; use a new campaign")
        header, rows = validate_csv(csv_path, args, log_n, ell_cast, ell_acc, scheme, expected_header)
        report.update(status="complete", verified=True, csv_rows=len(rows), csv_sha256=digest(csv_path),
                      source_and_binary_stable=True, selected_height=int(rows[0]["height"]))
        return header, rows, folder.relative_to(args.output_dir).as_posix()
    except KeyboardInterrupt:
        report.update(status="interrupted", error="interrupted by user")
        raise
    except (OSError, ValueError, RuntimeError) as error:
        report.update(status="failed", error=str(error), verified=False)
        raise
    finally:
        report.update(wall_seconds=time.monotonic() - start, completed_utc=utc_now())
        write_json(record_path, report)
        if usage is not None:
            print(f"Finished {scheme}: status={report['status']}, wall={report['wall_seconds']:.2f}s, "
                  f"peak={report['peak_rss_gib']:.3f}GiB, minflt={report['minor_page_faults']}, "
                  f"majflt={report['major_page_faults']}", flush=True)


def run_campaign(args, argv):
    args.output_dir.mkdir(parents=True, exist_ok=False)
    source_hashes = source_snapshot()
    identity = {
        "workload": CURRENT_WORKLOAD,
        "cost_scope": COST_SCOPE,
        "binary": str(args.binary), "binary_sha256": digest(args.binary),
        "binary_bytes": args.binary.stat().st_size, "binary_mtime_ns": args.binary.stat().st_mtime_ns,
        "source_sha256": source_hashes,
        "source_scope": "all src/examples/vendor/whir Rust sources, Cargo manifests/lockfiles, and Section 8 runner/reporters/workload-validator/sanitizer observed at campaign launch; matching build provenance is not inferred",
        "git_commit": command_output(["git", "rev-parse", "HEAD"]),
        "git_status": command_output(["git", "status", "--short"]),
        "rustc": command_output(["rustc", "--version"]),
        "platform": platform.platform(), "hardware": hardware(),
        "threads": args.threads, "field": "GF(2^256)", "hash": "BLAKE3-256",
        "proof_configuration": proof_configuration(args),
    }
    configurations = [(n, cast, acc) for n in args.logs for cast in args.ell_cast for acc in args.ell_acc]
    expected_rows = len(configurations) * args.repetitions * sum(len(OUTPUT_SCHEMES[name]) for name in args.schemes)
    campaign = {
        **identity, "command": [sys.executable, str(Path(__file__).resolve()), *argv],
        "parameters": parameters(args), "status": "running", "created_utc": utc_now(),
        "expected_jobs": len(configurations) * len(args.schemes), "expected_rows": expected_rows,
        "completed_jobs": [], "actual_rows": 0,
        "process_policy": "one fresh child per configuration/scheme; no concurrent benchmark children",
        "measurement_scope": "reduction phases and separate fresh input commitments; total prover cost excludes cached prior commitments; whole-child resources include setup, warmups and verification",
    }
    campaign_path = args.output_dir / "campaign.json"
    write_json(campaign_path, campaign)
    header = None
    csv_stream = None
    writer = None
    try:
        for log_n, ell_cast, ell_acc in configurations:
            for scheme in args.schemes:
                if digest(args.binary) != identity["binary_sha256"] or source_snapshot() != source_hashes:
                    raise RuntimeError("binary or source changed between profiling jobs; use a new campaign")
                job_header, rows, relative_folder = run_child(args, identity, log_n, ell_cast, ell_acc, scheme, header)
                if header is None:
                    header = job_header
                    csv_stream = (args.output_dir / "profiles.csv").open("x", newline="")
                    writer = csv.DictWriter(csv_stream, fieldnames=header)
                    writer.writeheader()
                writer.writerows(rows)
                csv_stream.flush()
                campaign["actual_rows"] += len(rows)
                campaign["completed_jobs"].append(relative_folder)
                write_json(campaign_path, campaign)
        if campaign["actual_rows"] != expected_rows:
            raise RuntimeError("collated row count differs from the requested campaign")
        campaign.update(status="complete", verified=True, csv_columns=header,
                        profiles_sha256=digest(args.output_dir / "profiles.csv"), source_and_binary_stable=True)
        print(public_text(f"Complete: {campaign['actual_rows']} verified rows in {args.output_dir / 'profiles.csv'}"), flush=True)
    except KeyboardInterrupt:
        campaign.update(status="interrupted", verified=False, error="interrupted by user")
        raise
    except (OSError, ValueError, RuntimeError) as error:
        campaign.update(status="failed", verified=False, error=str(error))
        raise
    finally:
        if csv_stream is not None:
            csv_stream.close()
        campaign["completed_utc"] = utc_now()
        write_json(campaign_path, campaign)


def main(argv=None):
    argv = list(sys.argv[1:] if argv is None else argv)
    arguments = parser()
    args = arguments.parse_args(argv)
    try:
        validate_arguments(args)
        run_campaign(args, argv)
    except KeyboardInterrupt:
        print("Profiling campaign interrupted; partial artifacts preserved.", file=sys.stderr)
        return 130
    except (OSError, ValueError, RuntimeError) as error:
        print(public_text(f"Profiling campaign failed: {error}"), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
