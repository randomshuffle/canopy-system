#!/usr/bin/env python3
"""Report Section 8 reduction estimates and measured samples without altering them.

The two Canopy reductions are paired by repetition and input seed. Explicitly
measured fresh commitments are added per sample to form an inclusive total;
historical input preparation is never prorated to infer that cost.
Only Python's standard library is required.
"""

import argparse
import csv
import hashlib
import json
import math
import statistics
from collections import defaultdict
from pathlib import Path

from public_metadata import public_metadata, public_text
from section8_workload import single_workload, validate_metadata, validate_sources, workload_description, workload_name


SCHEMES = ("ifold", "iacc", "canopy_total", "merkle_rate_half", "merkle_rate_quarter")
KEY_FIELDS = ("log_n", "ell_cast", "ell_acc", "height", "log_leaf_width", "security_bits")
KEY_ALIASES = {
    "log_n": ("log_n", "log_N", "log_input"),
    "ell_cast": ("ell_cast", "cast_sources", "input_oracles"),
    "ell_acc": ("ell_acc", "acc_sources", "accumulators"),
    "height": ("height", "canopy_height", "h"),
    "log_leaf_width": ("log_leaf_width", "log_width"),
    "security_bits": ("security_bits", "local_security_bits", "security"),
}
METRIC_ALIASES = {
    "prover_seconds": ("step_seconds", "stage_seconds", "prover_seconds", "reduction_seconds"),
    "incoming_commit_seconds": ("incoming_commit_seconds", "input_commit_seconds"),
    "fresh_commit_seconds": ("fresh_commit_seconds",),
    "prior_commit_seconds": ("prior_commit_seconds",),
    "total_prover_seconds": ("total_prover_seconds",),
    "verifier_hashes": ("verifier_hashes", "total_verifier_hashes", "total_hashes"),
    "auth_hashes": ("auth_hashes", "verifier_auth_hashes", "authentication_hashes"),
    "fs_finalizations": ("fs_finalizations", "fiat_shamir_finalizations", "fiat_shamir_hashes"),
    "proof_bytes": ("proof_bytes", "reduction_proof_bytes", "proof_size_bytes", "proof_payload_bytes"),
    "proof_payload_bytes": ("proof_payload_bytes",),
    "proof_hint_bytes": ("proof_hint_bytes", "proof_hints_bytes", "hint_bytes"),
    "proof_field_bytes": ("proof_field_bytes", "field_proof_bytes", "field_bytes"),
    "proof_hash_bytes": ("proof_hash_bytes", "authentication_proof_bytes", "hash_bytes", "authentication_bytes"),
    "proof_framing_bytes": ("proof_framing_bytes", "framing_bytes"),
    "output_instance_bytes": ("output_instance_bytes", "output_bytes"),
    "serialized_total_bytes": ("serialized_total_bytes", "serialized_proof_bytes"),
}
ESTIMATE_METRICS = tuple(k for k in METRIC_ALIASES if not k.endswith("seconds"))
SECURITY_FIELDS = ("eligible", "security_scope", "query_security_bits", "affine_mca_security_bits", "batching_powers_degree", "degree_scaled_affine_mca_security_bits", "one_ood_security_bits", "sumcheck_security_bits", "finite_field_floor_bits")
LABELS = {
    "ifold": "ifold (Canopy)",
    "iacc": "iacc (Merkle)",
    "canopy_total": "ifold + iacc",
    "merkle_rate_half": "Merkle, rate 1/2",
    "merkle_rate_quarter": "Merkle, rate 1/4",
}
FRESH_COST_FIELDS = ("fresh_commit_sources", "fresh_commit_seconds", "prior_commit_seconds")


def validate_fresh_costs(row, raw):
    """Require explicit source counts before adding commitment time."""
    count = raw.get("fresh_commit_sources", "")
    present = (count != "", "fresh_commit_seconds" in row, "prior_commit_seconds" in row)
    if not any(present):
        if "total_prover_seconds" in row:
            raise ValueError("total_prover_seconds requires explicit fresh commitment measurements")
        return
    if not all(present):
        raise ValueError("Fresh commitment measurements require all of " + ", ".join(FRESH_COST_FIELDS))
    count = number(count, "fresh_commit_sources")
    expected = 0 if row["scheme"] == "iacc" else row["ell_cast"]
    if not count.is_integer() or count != expected:
        raise ValueError(f"{row['scheme']}: expected {expected} fresh_commit_sources, got {count!r}")
    row["fresh_commit_sources"] = int(count)
    if expected == 0 and row["fresh_commit_seconds"] != 0:
        raise ValueError("iacc has no fresh commitments; fresh_commit_seconds must be zero")
    if expected > 0 and row["fresh_commit_seconds"] <= 0:
        raise ValueError("Fresh commitments require a positive measured fresh_commit_seconds")
    if "prover_seconds" not in row:
        raise ValueError("Fresh commitment measurements require a reduction duration")
    total = row["prover_seconds"] + row["fresh_commit_seconds"]
    if "total_prover_seconds" in row and not math.isclose(row["total_prover_seconds"], total, rel_tol=1e-8, abs_tol=1e-9):
        raise ValueError("total_prover_seconds differs from fresh commitments plus reduction time")
    row["total_prover_seconds"] = total
    if "incoming_commit_seconds" in row:
        accounted = row["fresh_commit_seconds"] + row["prior_commit_seconds"]
        tolerance = 1e-9 + 1e-8 * max(accounted, row["incoming_commit_seconds"])
        if abs(accounted - row["incoming_commit_seconds"]) > tolerance:
            raise ValueError("Fresh and prior commitment times do not reconcile to incoming commitment time")


def first(row, names, default=""):
    return next((row[n] for n in names if row.get(n, "") != ""), default)


def number(value, context):
    result = float(value)
    if not math.isfinite(result) or result < 0:
        raise ValueError(f"{context}: expected a finite nonnegative number, got {value!r}")
    return result


def scheme_name(value):
    name = value.strip().lower().replace("-", "_").replace(" ", "_")
    aliases = {
        "canopy_ifold": "ifold", "ifold_canopy": "ifold", "canopy_iacc": "iacc", "iacc_merkle": "iacc",
        "merkle_half": "merkle_rate_half", "merkle_quarter": "merkle_rate_quarter",
        "baseline_half": "merkle_rate_half", "baseline_quarter": "merkle_rate_quarter",
        "ifold+iacc": "canopy_total",
    }
    name = aliases.get(name, name)
    if name not in SCHEMES:
        raise ValueError(f"Unrecognized Section 8 scheme {value!r}")
    return name


def normalized(row, estimate=False):
    result = {key: first(row, aliases) for key, aliases in KEY_ALIASES.items()}
    for key in KEY_FIELDS[:3]:
        if result[key] == "":
            raise ValueError(f"Missing grouping column {key!r}")
    for key, value in result.items():
        if value != "":
            numeric = number(value, key)
            if not numeric.is_integer():
                raise ValueError(f"{key}: expected an integer, got {value!r}")
            result[key] = int(numeric)
    result["scheme"] = scheme_name(first(row, ("scheme", "component", "reduction")))
    result["queries"] = first(row, ("queries", "query_draws_per_source", "query_count", "q"))
    result["source_count"] = first(row, ("source_count", "sources", "num_sources"))
    result["workload"] = workload_name(row.get("workload"))
    result["source_count"] = validate_sources(result["workload"], result["scheme"], result["ell_cast"], result["ell_acc"], result["source_count"])
    for key, aliases in METRIC_ALIASES.items():
        names = tuple("expected_" + n for n in aliases) + aliases if estimate else aliases
        value = first(row, names)
        if value != "":
            result[key] = number(value, key)
    if "verifier_hashes" not in result and "auth_hashes" in result and "fs_finalizations" in result:
        result["verifier_hashes"] = result["auth_hashes"] + result["fs_finalizations"]
    if "proof_payload_bytes" not in result and "proof_bytes" in result:
        if estimate:
            result["proof_payload_bytes"] = result["proof_bytes"]
        elif "proof_framing_bytes" in result:
            result["proof_payload_bytes"] = result["proof_bytes"] - result["proof_framing_bytes"]
            if result["proof_payload_bytes"] < 0:
                raise ValueError("Measured framing exceeds measured proof bytes")
    result["repetition"] = first(row, ("repetition", "sample", "trial", "rep"))
    result["input_seed"] = first(row, ("input_seed", "seed"))
    result["verified"] = first(row, ("verified", "verification_passed"))
    if estimate:
        result.update({key: row[key] for key in SECURITY_FIELDS if row.get(key, "") != ""})
    else:
        validate_fresh_costs(result, row)
    return result


def key_of(row, include_scheme=True):
    key = tuple(row[key] for key in KEY_FIELDS)
    return key + (row["scheme"],) if include_scheme else key


def csv_rows(path):
    with path.open(newline="") as stream:
        reader = csv.DictReader(stream)
        if reader.fieldnames is None:
            raise ValueError(f"{public_text(str(path))}: CSV has no header")
        return list(reader)


def resolve_csv(path, filename):
    resolved = path / filename if path.is_dir() else path
    if not resolved.is_file():
        raise ValueError(f"CSV does not exist: {public_text(str(resolved))}")
    return resolved.resolve()


def read_estimates(path):
    result = {}
    workloads = []
    for raw in csv_rows(path):
        row = normalized(raw, estimate=True)
        workloads.append(row)
        if first(raw, ("is_selected", "selected"), "true").lower() not in ("true", "1", "yes"):
            continue
        key = key_of(row)
        if key in result:
            raise ValueError(f"Duplicate estimate for {key}")
        result[key] = row
    if not result:
        raise ValueError("The estimates CSV contains no rows")
    single_workload(workloads)
    # The sum is an estimate of sequential work; returned-instance bytes sum
    # both reduction outputs, including the intermediate output of ifold.
    for key in sorted({key_of(row, False) for row in result.values()}, key=str):
        fold = result.get(key + ("ifold",))
        acc = result.get(key + ("iacc",))
        total_key = key + ("canopy_total",)
        if fold and acc and total_key not in result:
            total = {field: value for field, value in zip(KEY_FIELDS, key)}
            total.update(scheme="canopy_total", queries="", source_count="", workload=fold["workload"])
            for metric in ESTIMATE_METRICS:
                if metric in fold and metric in acc:
                    total[metric] = fold[metric] + acc[metric]
            if fold.get("security_scope") == acc.get("security_scope"):
                total["security_scope"] = fold.get("security_scope", "")
            total["eligible"] = str(all(str(row.get("eligible", "true")).lower() in ("true", "1", "yes") for row in (fold, acc))).lower()
            result[total_key] = total
    return result


def read_samples(paths):
    result = defaultdict(list)
    for path in paths:
        for raw in csv_rows(path):
            if first(raw, ("phase", "sample_type")).lower() == "warmup" or raw.get("is_warmup", "").lower() in ("true", "1"):
                continue
            row = normalized(raw)
            if row["scheme"] == "canopy_total":
                raise ValueError("Supply separate ifold and iacc samples; canopy_total is derived by this report")
            if row["verified"].lower() not in ("true", "1", "yes"):
                raise ValueError(f"Unverified timing sample for {key_of(row)}; verified=true is required")
            if "prover_seconds" not in row:
                raise ValueError(f"Timing sample has no reduction duration: {key_of(row)}")
            result[key_of(row)].append(row)
    if not result:
        raise ValueError("The timing CSVs contain no measured samples")
    single_workload(row for group in result.values() for row in group)
    for key, group in result.items():
        if len({"fresh_commit_sources" in row for row in group}) > 1:
            raise ValueError(f"Incomplete fresh commitment measurements within timing group {key}")
    notes = []
    for key in sorted({key_of(row, False) for group in result.values() for row in group}, key=str):
        folds = result.get(key + ("ifold",), [])
        accs = result.get(key + ("iacc",), [])
        if not folds or not accs:
            continue
        if key + ("canopy_total",) in result:
            raise ValueError("Supply separate ifold and iacc samples; canopy_total is derived by this report")
        if any(row["repetition"] == "" or row["input_seed"] == "" for row in folds + accs):
            notes.append(f"Unpaired timings for {key}: repetition and input_seed are required for totals.")
            continue
        fold_map = {(row["repetition"], row["input_seed"]): row for row in folds}
        acc_map = {(row["repetition"], row["input_seed"]): row for row in accs}
        if len(fold_map) != len(folds) or len(acc_map) != len(accs):
            raise ValueError(f"Duplicate repetition/input_seed for Canopy timings at {key}")
        if fold_map.keys() != acc_map.keys():
            notes.append(f"Partially paired timings for {key}: only matching repetition/input_seed pairs contribute to totals.")
        for identity in sorted(fold_map.keys() & acc_map.keys()):
            fold, acc = fold_map[identity], acc_map[identity]
            if ("fresh_commit_sources" in fold) != ("fresh_commit_sources" in acc):
                raise ValueError(f"Incomplete fresh commitment measurements for paired Canopy sample {key}/{identity}")
            total = {field: value for field, value in zip(KEY_FIELDS, key)}
            total.update(scheme="canopy_total", repetition=identity[0], input_seed=identity[1], verified="true", workload=fold["workload"])
            for metric in METRIC_ALIASES:
                if metric in fold and metric in acc:
                    total[metric] = fold[metric] + acc[metric]
            if "fresh_commit_sources" in fold:
                total["fresh_commit_sources"] = fold["fresh_commit_sources"] + acc["fresh_commit_sources"]
            result[key + ("canopy_total",)].append(total)
    return result, notes


def statistics_into(output, metric, samples):
    values = [row[metric] for row in samples if metric in row]
    if values:
        output[metric + "_samples"] = len(values)
        for name, function in (("mean", statistics.mean), ("median", statistics.median), ("min", min), ("max", max)):
            output[f"{name}_{metric}"] = function(values)


def summarize(estimates, samples):
    rows = []
    for key in sorted(estimates.keys() | samples.keys(), key=lambda k: tuple(-1 if x == "" else x for x in k[:-1]) + (SCHEMES.index(k[-1]),)):
        row = {field: value for field, value in zip(KEY_FIELDS + ("scheme",), key)}
        estimate = estimates.get(key)
        group = samples.get(key, [])
        row["workload"] = (estimate or group[0])["workload"]
        row["source_count"] = (estimate or group[0]).get("source_count", "")
        row.update(estimate_status="available" if estimate else "unavailable", timing_status="measured" if group else "unmeasured", timing_samples=len(group), verified_all="true" if group else "")
        if group:
            row["cost_scope"] = "fresh_commit_plus_reduction_v1" if "fresh_commit_sources" in group[0] else "reduction_only_v1"
            if "fresh_commit_sources" in group[0]:
                row["fresh_commit_sources"] = group[0]["fresh_commit_sources"]
        if estimate:
            row.update(queries=estimate.get("queries", ""), source_count=estimate.get("source_count", ""))
            row.update({field: estimate[field] for field in SECURITY_FIELDS if field in estimate})
            for metric in ESTIMATE_METRICS:
                if metric in estimate:
                    row["expected_" + metric] = estimate[metric]
        for metric in METRIC_ALIASES:
            statistics_into(row, metric, group)
        rows.append(row)
    return rows


def comparison_rows(summary):
    grouped = defaultdict(dict)
    # Merkle controls do not depend on height. Match a control by all other
    # parameters, while retaining the selected height on the Canopy row.
    baseline_fields = tuple(k for k in KEY_FIELDS if k != "height")
    for row in summary:
        grouped[tuple(row[k] for k in baseline_fields)][row["scheme"]] = row
    output = []
    for schemes in grouped.values():
        canopy = schemes.get("canopy_total")
        if canopy is None:
            continue
        for baseline_name in ("merkle_rate_half", "merkle_rate_quarter"):
            baseline = schemes.get(baseline_name)
            if baseline is None:
                continue
            row = {key: canopy[key] for key in KEY_FIELDS}
            row["workload"] = canopy["workload"]
            row["baseline"] = baseline_name
            for label, metric in (("prover", "median_prover_seconds"), ("total_prover", "median_total_prover_seconds"), ("expected_verifier_hashes", "expected_verifier_hashes"), ("expected_proof_bytes", "expected_proof_bytes"), ("measured_proof_payload_bytes", "median_proof_payload_bytes")):
                if metric in canopy and metric in baseline:
                    row["canopy_" + label] = canopy[metric]
                    row["baseline_" + label] = baseline[metric]
                    if canopy[metric] != 0:
                        row["baseline_over_canopy_" + label] = baseline[metric] / canopy[metric]
            output.append(row)
    return output


def write_csv(path, rows):
    fields = list(dict.fromkeys(key for row in rows for key in row))
    if not fields:
        fields = list(KEY_FIELDS) + ["baseline"]
    with path.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def formatted(value, digits=2):
    return "—" if value is None or value == "" else f"{float(value):,.{digits}f}"


def render_markdown(rows, comparisons, notes, provenance):
    lines = [
        "# Section 8: ifold and iacc", "",
        workload_description(provenance["workload"]), "",
        "The table reports each reduction and their sequential sum. Prover time is the median measured reduction time; the range is the minimum–maximum. Input commitment/preparation is reported separately and is excluded from reduction time. Every included timing sample passed verification.", "",
        "Expected verifier hashes and payload bytes come from the estimator. Measured hash counts and proof payload sizes come from sampled proofs; sampling fluctuations can change them. Hash counts include authentication plus Fiat–Shamir finalizations, with direct digest-to-field conversion costing zero hashes. They count complete BLAKE3 hash/XOF invocations, not compression blocks.", "",
        "Expected proof payload excludes public input instances, fresh commitment roots/OOD values attached to returned instances, and serializer framing. Sampled payload is actual proof bytes minus native framing. Hash payload in ifold consists only of its Canopy bottom paths. Actual framed proof bytes, framing, fresh output wire bytes, and serialized totals are separate columns in `summary.csv`. For the combined row, output bytes sum both reductions' fresh output data, including the intermediate ifold output; they are not the size of just the final accumulator.", "",
        "Merkle controls batch ell_cast + ell_acc sources at the original message size N and code rates 1/2 and 1/4. Their cost is compared to ifold + iacc. The Canopy block length is half the cast input block length. Height is chosen by minimizing expected total verifier hashes; the two reductions preserve their respective codeword lengths.", "",
        "| N | ell_cast | ell_acc | Height | Reduction | Samples | Prover median (ms) | Range (ms) | Input commit median (ms) | Expected verifier hashes | Sampled hashes median | Expected proof (KiB) | Sampled payload median (KiB) |",
        "|---|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|",
    ]
    parameter_sets = sorted({(row["log_leaf_width"], row["security_bits"]) for row in rows}, key=str)
    settings = "; ".join(f"row width {1 << int(width) if width != '' else 'unspecified'}, local security budget {bits if bits != '' else 'unspecified'} bits" for width, bits in parameter_sets)
    lines[2:2] = [f"N is the original message length in field elements. Recorded settings: {settings}.", ""]
    for row in rows:
        duration = row.get("median_prover_seconds")
        preparation = row.get("median_incoming_commit_seconds")
        expected_proof = row.get("expected_proof_bytes")
        measured_proof = row.get("median_proof_payload_bytes")
        span = "—"
        if "min_prover_seconds" in row:
            span = f"{1000 * row['min_prover_seconds']:,.2f}–{1000 * row['max_prover_seconds']:,.2f}"
        lines.append(f"| 2^{row['log_n']} | {row['ell_cast']} | {row['ell_acc']} | {row['height'] if row['height'] != '' else '—'} | {LABELS[row['scheme']]} | {row['timing_samples']} | {formatted(None if duration is None else 1000 * duration)} | {span} | {formatted(None if preparation is None else 1000 * preparation)} | {formatted(row.get('expected_verifier_hashes'), 1)} | {formatted(row.get('median_verifier_hashes'), 1)} | {formatted(None if expected_proof is None else expected_proof / 1024)} | {formatted(None if measured_proof is None else measured_proof / 1024)} |")
    lines += ["", "Canopy totals use matched repetition and input-seed pairs. Their means, medians, and ranges are calculated from each paired sum; they are not sums of separately aggregated medians."]
    has_fresh_costs = any("median_total_prover_seconds" in row for row in rows)
    if has_fresh_costs:
        lines += ["", "Fresh commitment plus reduction time uses only the explicitly measured ell_cast fresh input commitments. Prior accumulator commitments remain separate and are excluded. Per sample, Canopy total is fresh commitments + ifold + iacc; each control total is fresh commitments + its accumulation reduction. iacc contributes zero fresh commitments. The table shows medians and the minimum–maximum of those complete per-sample totals.", "",
                  "| N | ell_cast | ell_acc | Height | Reduction | Fresh sources | Fresh commit median (ms) | Prior commit median (ms), excluded | Reduction median (ms) | Total prover median (ms) | Total range (ms) |",
                  "|---|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|"]
        for row in rows:
            span = "—"
            if "min_total_prover_seconds" in row:
                span = f"{1000 * row['min_total_prover_seconds']:,.2f}–{1000 * row['max_total_prover_seconds']:,.2f}"
            costs = [formatted(None if row.get(metric) is None else 1000 * row[metric]) for metric in ("median_fresh_commit_seconds", "median_prior_commit_seconds", "median_prover_seconds", "median_total_prover_seconds")]
            lines.append(f"| 2^{row['log_n']} | {row['ell_cast']} | {row['ell_acc']} | {row['height'] if row['height'] != '' else '—'} | {LABELS[row['scheme']]} | {row.get('fresh_commit_sources', '—')} | " + " | ".join(costs) + f" | {span} |")
        lines += ["", "Missing fresh commitment measurements remain unavailable; historical incoming commitment time is never prorated or substituted. Fresh-commit and reduction medians are descriptive components; the total is aggregated after their per-sample addition."]
    else:
        lines += ["", "Fresh commitment costs were not recorded in these samples. Commitment-inclusive prover totals remain unavailable; incoming commitment time is not prorated to infer them."]
    ratio_header = "| N | ell_cast | ell_acc | Merkle control | Reduction prover ratio (Merkle / Canopy) |"
    ratio_separator = "|---|---:|---:|---|---:|"
    if has_fresh_costs:
        ratio_header += " Total prover ratio including fresh commits (Merkle / Canopy) |"
        ratio_separator += "---:|"
    lines += ["", ratio_header + " Hash ratio (Merkle / Canopy) | Expected proof ratio (Merkle / Canopy) |", ratio_separator + "---:|---:|"]
    for row in comparisons:
        entry = f"| 2^{row['log_n']} | {row['ell_cast']} | {row['ell_acc']} | {LABELS[row['baseline']]} | {formatted(row.get('baseline_over_canopy_prover'))} |"
        if has_fresh_costs:
            entry += f" {formatted(row.get('baseline_over_canopy_total_prover'))} |"
        lines.append(entry + f" {formatted(row.get('baseline_over_canopy_expected_verifier_hashes'))} | {formatted(row.get('baseline_over_canopy_expected_proof_bytes'))} |")
    lines += ["", "Ratios above one mean the Merkle control costs more. Timing ratios compare measured medians on the supplied runs; expected hash/proof ratios use the model.", "",
              "RS uses the Johnson radius `1 - 1.05 sqrt(rate)` and the recorded local security budget, with no proof of work. The estimator records local query and degree-scaled affine mutual-correlated-agreement diagnostics; it does not certify composed powers-batching soundness. This report does not add a final decider, a casting step, or initial witness generation. It does not substitute a shrinking WHIR fold for the batching IOR.", "",
              "Full mean/median/min/max statistics and separated proof fields, hint/hash payload, returned instances, and serialization sizes are in `summary.csv`. Sources and SHA-256 checksums are in `provenance.json`; raw input files are preserved."]
    if notes:
        lines += ["", "Data notes:", ""] + ["- " + public_text(note) for note in notes]
    if provenance["source_metadata"]:
        lines += ["", "Available source metadata:", "", "```json", json.dumps(provenance["source_metadata"], indent=2, sort_keys=True), "```"]
    else:
        lines += ["", "No companion hardware/run provenance was supplied. This report makes no hardware attribution."]
    return "\n".join(lines) + "\n"


def source_record(path):
    return {"path": public_text(str(path)), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}


def companion_metadata(paths, explicit_paths=()):
    result = []
    seen = set()
    allowed = {"workload", "cost_scope", "fresh_commit_scope", "prior_commit_scope", "proof_configuration", "hardware", "system", "platform", "machine", "cpu", "memory", "threads", "rayon_threads", "command", "argv", "git_commit", "rustc", "field", "hash", "warmups", "repetitions", "security_bits", "parameters", "measurement_scope", "binary_sha256", "started_unix", "wall_seconds", "exit_code", "peak_rss_bytes", "peak_rss_gib", "user_seconds", "system_seconds", "memory_boundary", "architecture", "os", "cores", "physical_memory_bytes", "cpu_brand"}
    candidates = list(explicit_paths)
    for source in paths:
        candidates.extend((source.with_suffix(".json"), source.with_suffix(".provenance.json"), source.parent / "provenance.json", source.parent / "process.json"))
    for path in candidates:
        if path in seen or not path.is_file():
            continue
        seen.add(path)
        metadata = json.loads(path.read_text())
        if not isinstance(metadata, dict):
            continue
        selected = {key: value for key, value in metadata.items() if key in allowed}
        if selected:
            result.append({"source": source_record(path), "metadata": public_metadata(selected)})
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--estimates", type=Path, required=True, help="estimates.csv or its directory")
    parser.add_argument("--timings", type=Path, nargs="+", required=True, help="one or more raw timing CSVs")
    parser.add_argument("--height-search", type=Path, help="optional height_search.csv retained in provenance")
    parser.add_argument("--provenance", type=Path, nargs="+", default=[], help="explicit run/hardware metadata JSON, such as the measurement wrapper's process.json")
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args()
    try:
        estimates_path = resolve_csv(args.estimates, "estimates.csv")
        timing_paths = [resolve_csv(path, "timings.csv") for path in args.timings]
        search_path = resolve_csv(args.height_search, "height_search.csv") if args.height_search else None
        metadata_paths = [path.resolve() for path in args.provenance]
        for path in metadata_paths:
            if not path.is_file():
                raise ValueError(f"Metadata JSON does not exist: {public_text(str(path))}")
        estimates = read_estimates(estimates_path)
        samples, notes = read_samples(timing_paths)
        workload_rows = list(estimates.values()) + [row for group in samples.values() for row in group]
        if search_path:
            search_rows = [normalized(row, estimate=True) for row in csv_rows(search_path)]
            if not search_rows:
                raise ValueError("The height-search CSV contains no rows")
            workload_rows.extend(search_rows)
        workload = single_workload(workload_rows)
        summary = summarize(estimates, samples)
        comparisons = comparison_rows(summary)
        paths = [estimates_path, *timing_paths] + ([search_path] if search_path else [])
        sources = {path.resolve() for path in paths + metadata_paths}
        for name in ("summary.csv", "comparison.csv", "README.md", "provenance.json"):
            if (args.output_dir / name).resolve() in sources:
                raise ValueError("Report output would overwrite a raw source file")
        metadata = companion_metadata(paths, metadata_paths)
        validate_metadata(workload, metadata)
        cost_scopes = {row["cost_scope"] for row in summary if "cost_scope" in row}
        provenance = {
            "workload": workload,
            "estimates": source_record(estimates_path), "timings": [source_record(path) for path in timing_paths],
            "height_search": source_record(search_path) if search_path else None,
            "source_metadata": metadata,
            "pairing": "same parameters, repetition, and input_seed; statistics of per-sample sums",
            "cost_scope": next(iter(cost_scopes)) if len(cost_scopes) == 1 else "mixed_fresh_cost_availability",
            "time_scope": "prover_seconds is reduction only; total_prover_seconds adds explicitly measured fresh commitments per sample; prior commitments and final decider excluded; missing fresh measurements remain unavailable",
            "proof_scope": "expected payload and measured proof_bytes minus proof_framing_bytes; public input instances and fresh output commitment/OOD data excluded",
            "reported_rows": len(summary), "comparisons": len(comparisons), "notes": public_metadata(notes),
        }
        args.output_dir.mkdir(parents=True, exist_ok=True)
        write_csv(args.output_dir / "summary.csv", summary)
        write_csv(args.output_dir / "comparison.csv", comparisons)
        (args.output_dir / "README.md").write_text(render_markdown(summary, comparisons, notes, provenance))
        (args.output_dir / "provenance.json").write_text(json.dumps(public_metadata(provenance), indent=2, sort_keys=True) + "\n")
    except (ValueError, OSError) as error:
        parser.error(str(error))
    print(f"Reported {len(summary)} reduction groups and {len(comparisons)} comparisons in {public_text(str(args.output_dir))}")


if __name__ == "__main__":
    main()
