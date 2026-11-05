#!/usr/bin/env python3
"""Report measured Section 8 prover phase profiles using only the standard library.

Matplotlib, when available, additionally renders a static stacked SVG. Parent
phase totals and exclusive child phases remain distinct to avoid double counting.
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


KEYS = ("log_n", "ell_cast", "ell_acc", "height", "log_leaf_width", "security_bits")
SCHEMES = ("ifold", "iacc", "merkle_rate_half", "merkle_rate_quarter", "canopy_total")
LABELS = {"ifold": "ifold", "iacc": "iacc", "merkle_rate_half": "Merkle 1/2", "merkle_rate_quarter": "Merkle 1/4", "canopy_total": "ifold + iacc"}
CHILDREN = {
    "sumcheck_seconds": ("sumcheck_setup_seconds", "sumcheck_polynomial_seconds", "sumcheck_fold_seconds", "sumcheck_transcript_seconds"),
    "combination_seconds": ("message_combination_seconds", "codeword_combination_seconds"),
    "output_commit_seconds": ("encoding_seconds", "merkle_leaf_seconds", "merkle_parent_seconds", "h2f_seconds", "canopy_ood_seconds", "output_ood_seconds", "output_commit_other_seconds"),
    "opening_seconds": ("opening_auth_seconds", "opening_claim_seconds"),
}
EXCLUSIVE = ("form_seconds", *CHILDREN["sumcheck_seconds"], *CHILDREN["combination_seconds"], *CHILDREN["output_commit_seconds"], *CHILDREN["opening_seconds"], "serialization_seconds", "prover_other_seconds")
INCOMING = ("incoming_encoding_seconds", "incoming_merkle_leaf_seconds", "incoming_merkle_parent_seconds", "incoming_h2f_seconds", "incoming_ood_seconds", "incoming_other_seconds")
COARSE = ("form_seconds", "sumcheck_seconds", "combination_seconds", "output_commit_seconds", "opening_seconds", "serialization_seconds", "prover_other_seconds")
METRICS = tuple(dict.fromkeys(("step_seconds", *EXCLUSIVE, *CHILDREN, "incoming_commit_seconds", *INCOMING)))
FRESH = ("fresh_encoding_seconds", "fresh_merkle_leaf_seconds", "fresh_merkle_parent_seconds", "fresh_h2f_seconds", "fresh_ood_seconds", "fresh_other_seconds")
FRESH_INPUT_COLUMNS = ("fresh_commit_sources", "fresh_commit_seconds", "prior_commit_seconds", *FRESH)
FRESH_METRICS = ("fresh_commit_seconds", "prior_commit_seconds", *FRESH, "total_prover_seconds")
COST_SCOPE = "fresh_commit_plus_reduction_v1"
REDUCTION_SCOPE = "reduction_only_v1"
PHASE_LABELS = {
    "step_seconds": "Reduction total", "incoming_commit_seconds": "Incoming commitment total",
    "form_seconds": "Input functional materialization", "sumcheck_seconds": "Sumcheck total",
    "sumcheck_setup_seconds": "Sumcheck / setup", "sumcheck_polynomial_seconds": "Sumcheck / round polynomials",
    "sumcheck_fold_seconds": "Sumcheck / folding", "sumcheck_transcript_seconds": "Sumcheck / transcript and control",
    "combination_seconds": "Combination total", "message_combination_seconds": "Combination / messages",
    "codeword_combination_seconds": "Combination / cached codewords (plus fused leaves when flagged)", "output_commit_seconds": "Output commitment total",
    "encoding_seconds": "Output / RS encoding", "merkle_leaf_seconds": "Output / Merkle leaves",
    "merkle_parent_seconds": "Output / Merkle parents", "h2f_seconds": "Output / digest-to-field conversion",
    "canopy_ood_seconds": "Output / Canopy layer OOD", "output_ood_seconds": "Output / polynomial OOD",
    "output_commit_other_seconds": "Output / remaining commitment work", "opening_seconds": "Opening total",
    "opening_auth_seconds": "Opening / authentication", "opening_claim_seconds": "Opening / returned claim",
    "serialization_seconds": "Final proof copy", "prover_other_seconds": "Other reduction work",
    "incoming_encoding_seconds": "Incoming / RS encoding", "incoming_merkle_leaf_seconds": "Incoming / Merkle leaves",
    "incoming_merkle_parent_seconds": "Incoming / Merkle parents", "incoming_h2f_seconds": "Incoming / digest-to-field conversion",
    "incoming_ood_seconds": "Incoming / OOD evaluation", "incoming_other_seconds": "Incoming / remaining work",
    "fresh_commit_seconds": "Fresh commitment total", "prior_commit_seconds": "Prior commitment preparation total",
    "fresh_encoding_seconds": "Fresh / RS encoding", "fresh_merkle_leaf_seconds": "Fresh / Merkle leaves",
    "fresh_merkle_parent_seconds": "Fresh / Merkle parents", "fresh_h2f_seconds": "Fresh / digest-to-field conversion",
    "fresh_ood_seconds": "Fresh / OOD evaluation", "fresh_other_seconds": "Fresh / remaining commitment work",
    "total_prover_seconds": "Fresh commitment + reduction total",
}


def numeric(value, context):
    number = float(value)
    if not math.isfinite(number) or number < 0:
        raise ValueError(f"{context}: expected a finite nonnegative duration, got {value!r}")
    return number


def scheme(value):
    name = value.lower().replace("-", "_")
    name = {"ifold_canopy": "ifold", "canopy_ifold": "ifold", "iacc_merkle": "iacc", "canopy_iacc": "iacc", "merkle_half": "merkle_rate_half", "merkle_quarter": "merkle_rate_quarter"}.get(name, name)
    if name not in SCHEMES[:-1]:
        raise ValueError(f"Unknown raw profiling scheme {value!r}")
    return name


def grouping_key(row):
    return tuple(row[key] for key in KEYS) + (row["scheme"],)


def sort_key(key):
    return key[:-1] + (SCHEMES.index(key[-1]),)


def metric_names(row):
    return METRICS + FRESH_METRICS if "fresh_commit_seconds" in row else METRICS


def cost_scope(groups):
    scopes = {COST_SCOPE if "fresh_commit_seconds" in row else REDUCTION_SCOPE for samples in groups.values() for row in samples}
    if len(scopes) != 1:
        raise ValueError("Cannot mix timing files with and without fresh-commitment accounting")
    return scopes.pop()


def reconcile(row, context):
    def check(parent, children):
        actual = row[parent]
        children_total = sum(row[name] for name in children)
        if not math.isclose(actual, children_total, rel_tol=1e-6, abs_tol=5e-9):
            raise ValueError(f"{context}: {parent}={actual:.12g} does not reconcile with its exclusive components={children_total:.12g}")
    for parent, children in CHILDREN.items():
        check(parent, children)
    check("step_seconds", EXCLUSIVE)
    check("step_seconds", COARSE)
    check("incoming_commit_seconds", INCOMING)
    if "fresh_commit_seconds" in row:
        check("fresh_commit_seconds", FRESH)
        check("incoming_commit_seconds", ("fresh_commit_seconds", "prior_commit_seconds"))
        check("total_prover_seconds", ("fresh_commit_seconds", "step_seconds"))


def read_samples(paths):
    groups = defaultdict(list)
    identities = set()
    for path in paths:
        with path.open(newline="") as stream:
            reader = csv.DictReader(stream)
            if reader.fieldnames is None:
                raise ValueError(f"{public_text(str(path))}: missing CSV header")
            for line, raw in enumerate(reader, start=2):
                if raw.get("phase", "").lower() == "warmup" or raw.get("is_warmup", "").lower() in ("true", "1"):
                    continue
                context = f"{public_text(str(path))}:{line}"
                if raw.get("verified", "").lower() not in ("true", "1", "yes"):
                    raise ValueError(f"{context}: unverified measured sample")
                row = {}
                for key in KEYS:
                    value = raw.get(key, raw.get("local_security_bits", "") if key == "security_bits" else "")
                    if value == "":
                        raise ValueError(f"{context}: missing grouping column {key}")
                    number = numeric(value, context + ": " + key)
                    if not number.is_integer():
                        raise ValueError(f"{context}: {key} must be an integer")
                    row[key] = int(number)
                row["scheme"] = scheme(raw.get("scheme", ""))
                row["workload"] = workload_name(raw.get("workload"))
                row["sources"] = validate_sources(row["workload"], row["scheme"], row["ell_cast"], row["ell_acc"], raw.get("sources"))
                for name in METRICS:
                    if raw.get(name, "") == "":
                        raise ValueError(f"{context}: missing phase column {name}; use the profiling executable")
                    row[name] = numeric(raw[name], context + ": " + name)
                fresh_columns = set(FRESH_INPUT_COLUMNS) & raw.keys()
                if fresh_columns:
                    if fresh_columns != set(FRESH_INPUT_COLUMNS) or any(raw.get(name, "") == "" for name in FRESH_INPUT_COLUMNS):
                        raise ValueError(f"{context}: incomplete fresh-commitment accounting columns")
                    count = numeric(raw["fresh_commit_sources"], context + ": fresh_commit_sources")
                    expected = 0 if row["scheme"] == "iacc" else row["ell_cast"]
                    if not count.is_integer() or count != expected:
                        raise ValueError(f"{context}: expected {expected} fresh committed sources, got {count}")
                    row["fresh_commit_sources"] = int(count)
                    for name in FRESH_METRICS[:-1]:
                        row[name] = numeric(raw[name], context + ": " + name)
                    if not expected and any(row[name] != 0 for name in ("fresh_commit_seconds", *FRESH)):
                        raise ValueError(f"{context}: iacc has no fresh commitment work")
                    row["total_prover_seconds"] = row["fresh_commit_seconds"] + row["step_seconds"]
                for name in ("repetition", "input_seed"):
                    if raw.get(name, "") == "":
                        raise ValueError(f"{context}: missing sample identity {name}")
                    row[name] = raw[name]
                identity = grouping_key(row) + (row["repetition"], row["input_seed"])
                if identity in identities:
                    raise ValueError(f"{context}: duplicate sample {identity}")
                identities.add(identity)
                reconcile(row, context)
                groups[grouping_key(row)].append(row)
    if not groups:
        raise ValueError("No measured profiling samples were supplied")
    single_workload(row for samples in groups.values() for row in samples)
    cost_scope(groups)
    notes = []
    parameters = {key[:-1] for key in groups}
    for key in sorted(parameters):
        fold = groups.get(key + ("ifold",), [])
        acc = groups.get(key + ("iacc",), [])
        if not fold or not acc:
            continue
        fold_map = {(r["repetition"], r["input_seed"]): r for r in fold}
        acc_map = {(r["repetition"], r["input_seed"]): r for r in acc}
        if fold_map.keys() != acc_map.keys():
            notes.append(f"At {key}, paired Canopy totals use only matching repetition/input_seed samples.")
        for identity in sorted(fold_map.keys() & acc_map.keys()):
            row = {field: value for field, value in zip(KEYS, key)}
            row.update(scheme="canopy_total", repetition=identity[0], input_seed=identity[1], workload=fold_map[identity]["workload"], sources="")
            row.update({name: fold_map[identity][name] + acc_map[identity][name] for name in metric_names(fold_map[identity])})
            if "fresh_commit_seconds" in row:
                row["fresh_commit_sources"] = fold_map[identity]["fresh_commit_sources"] + acc_map[identity]["fresh_commit_sources"]
            reconcile(row, "paired Canopy total")
            groups[key + ("canopy_total",)].append(row)
    return groups, notes


def phase_category(name):
    if name.startswith("fresh_"):
        return "fresh_commitment"
    if name == "prior_commit_seconds":
        return "prior_commitment"
    if name == "total_prover_seconds":
        return "commit_inclusive_total"
    if name.startswith("incoming_"):
        return "incoming"
    for parent, children in CHILDREN.items():
        if name == parent or name in children:
            return parent.removesuffix("_seconds")
    return {"step_seconds": "total", "form_seconds": "form", "serialization_seconds": "serialization", "prover_other_seconds": "other"}[name]


def summary_rows(groups):
    rows = []
    for key in sorted(groups, key=sort_key):
        samples = groups[key]
        for metric in metric_names(samples[0]):
            values = [row[metric] for row in samples]
            incoming = metric.startswith("incoming_")
            denominator = "incoming_commit_seconds" if incoming else "step_seconds"
            scope = "incoming_exclusive" if incoming else "exclusive"
            if metric in CHILDREN:
                scope = "parent"
            if metric in ("step_seconds", "incoming_commit_seconds"):
                scope = "incoming_total" if incoming else "total"
            if metric.startswith("fresh_"):
                denominator = "fresh_commit_seconds"
                scope = "fresh_total" if metric == denominator else "fresh_exclusive"
            elif metric == "prior_commit_seconds":
                denominator, scope = "incoming_commit_seconds", "prior_total"
            elif metric == "total_prover_seconds":
                denominator, scope = "total_prover_seconds", "commit_inclusive_total"
            total_mean = statistics.mean(row[denominator] for row in samples)
            fractions = [100 * row[metric] / row[denominator] if row[denominator] else 0.0 for row in samples]
            row = dict(zip(KEYS + ("scheme",), key))
            row.update(workload=samples[0]["workload"], sources=samples[0]["sources"])
            row.update(cost_scope=COST_SCOPE if "fresh_commit_seconds" in samples[0] else REDUCTION_SCOPE,
                       fresh_commit_sources=samples[0].get("fresh_commit_sources", ""), percentage_denominator=denominator)
            row.update(samples=len(samples), scope=scope, category=phase_category(metric), component=metric,
                       mean_seconds=statistics.mean(values), median_seconds=statistics.median(values), min_seconds=min(values), max_seconds=max(values),
                       mean_percent_of_total=100 * statistics.mean(values) / total_mean if total_mean else 0.0,
                       min_sample_percent=min(fractions), max_sample_percent=max(fractions))
            rows.append(row)
    return rows


def write_csv(path, rows):
    with path.open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]), lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def sample_summary(samples, metric):
    values = [row[metric] for row in samples]
    return f"{1000 * statistics.mean(values):,.3f} [{1000 * min(values):,.3f}–{1000 * max(values):,.3f}]"


def phase_cell(samples, metric, denominator):
    mean = statistics.mean(row[metric] for row in samples)
    total = statistics.mean(row[denominator] for row in samples)
    return sample_summary(samples, metric) + f"; {100 * mean / total if total else 0:.1f}%"


def render_markdown(groups, notes, plot_created, metadata):
    workload = single_workload(row for samples in groups.values() for row in samples)
    fresh_costs = cost_scope(groups) == COST_SCOPE
    lines = [
        "# Section 8 measured prover phase profiles", "",
        workload_description(workload), "",
        "Durations are measured means in milliseconds; brackets contain the measured minimum–maximum. Percentages divide a phase's mean by its corresponding mean total. Sample counts are shown explicitly. Paired ifold + iacc values sum matching repetition/input-seed samples before aggregation.", "",
        "N counts original message field elements. The reported reduction time excludes initial input preparation, incoming commitments, verification, and the final decider. Incoming commitments have their own decomposition below.", "",
        "The detailed phase tables are exclusive: their reduction entries sum to reduction time. Sumcheck, combination, output-commitment, and opening parent totals are retained in `summary.csv` with `scope=parent`; these overlap their child entries and must not be added again. Every sample passed verification and all phase sums were checked before reporting.", "",
        "Output RS encoding is zero for iacc and both Merkle controls because their output codewords are formed by combining cached input codewords. Input encodings are still charged to incoming commitment preparation. ifold freshly encodes its Canopy layer, which has half the original input codeword length. A zero output encoding time denotes this reuse boundary, not a zero encoding cost for preparing fresh inputs.", "",
        "Separate Merkle leaf time includes allocation, canonical serialization of field elements, and native `hash_rows` work. On rows flagged `codeword_leaf_fused=true`, base-codeword leaf hashing is inside `codeword_combination_seconds`; compare that column plus `merkle_leaf_seconds` across implementations. Ifold still has separately timed encoded-layer leaves. A missing historical flag means false. Digest-to-field time measures direct conversion into GF(2^256) and uses no cryptographic hash. Sumcheck transcript time includes transcript work plus control and timer residuals; it is not a pure hash-latency measurement. Final proof-copy time measures only `p.proof()` copying; it excludes the field serialization already charged to leaf hashing. Remaining output-commitment work and remaining reduction work are explicit exclusive phases.", "",
        "Dense covector materialization is charged to `form_seconds`. Sumcheck setup takes ownership of those buffers without materializing them again. The current prover borrows first-round messages and fuses fresh-MLE folding with the following polynomial evaluation; compare total sumcheck time across implementations because the polynomial/fold attribution has changed.", "",
        "| N | ell_cast | ell_acc | Width | Height | Reduction | Samples | Reduction mean [range] (ms) | Incoming mean [range] (ms) |",
        "|---|---:|---:|---:|---:|---|---:|---:|---:|",
    ]
    for key in sorted(groups, key=sort_key):
        n, cast, acc, height, width, _ = key[:-1]
        samples = groups[key]
        lines.append(f"| 2^{n} | {cast} | {acc} | {1 << width} | {height if key[-1] not in ('merkle_rate_half', 'merkle_rate_quarter') else '—'} | {LABELS[key[-1]]} | {len(samples)} | {sample_summary(samples, 'step_seconds')} | {sample_summary(samples, 'incoming_commit_seconds')} |")
    if fresh_costs:
        lines += ["", "Fresh commitment cost includes only ell_cast new input oracles. The prior Canopy and prior Merkle commitments are already available; their independently measured preparation remains diagnostic and is excluded from total prover cost. iacc reuses ifold's committed layer witnesses and has zero fresh commitments. `total_prover_seconds = fresh_commit_seconds + step_seconds` is formed for each sample before pairing and aggregation. The paired total is fresh Canopy commitment + ifold + iacc; each control total is fresh Merkle commitment + its reduction.", "",
                  "| N | ell_cast | ell_acc | Reduction | Fresh committed sources | Fresh commit mean [range] (ms) | Prior preparation mean [range] (ms) | Total prover mean [range] (ms) |",
                  "|---|---:|---:|---|---:|---:|---:|---:|"]
        for key in sorted(groups, key=sort_key):
            samples = groups[key]
            lines.append(f"| 2^{key[0]} | {key[1]} | {key[2]} | {LABELS[key[-1]]} | {samples[0]['fresh_commit_sources']} | {sample_summary(samples, 'fresh_commit_seconds')} | {sample_summary(samples, 'prior_commit_seconds')} | {sample_summary(samples, 'total_prover_seconds')} |")
    else:
        lines += ["", "Fresh-only commitment measurements are unavailable in these historical samples. Incoming commitments include prepared prior claims, so this report does not infer fresh commitment or total prover cost from them."]
    if plot_created:
        lines += ["", "![Mean exclusive prover phases](phase_profiles.svg)", "",
                  "The plot stacks the seven coarse exclusive groups; output commitment includes its seven child phases. Incoming commitments and paired Canopy totals are omitted from the plot to keep the four individual reductions comparable."]
    workloads = defaultdict(dict)
    for key, samples in groups.items():
        n, cast, acc, _, width, security = key[:-1]
        workload = (n, cast, acc, width, security)
        if key[-1] in workloads[workload]:
            raise ValueError(f"Multiple heights for the same workload/scheme {workload + (key[-1],)}; supply the selected-height samples")
        workloads[workload][key[-1]] = samples
    for (n, cast, acc, width, security), schemes in sorted(workloads.items()):
        order = [name for name in SCHEMES if name in schemes]
        lines += ["", f"## N = 2^{n}, ell_cast = {cast}, ell_acc = {acc}", "",
                  f"Row width {1 << width}; local security budget {security} bits. Cells give mean [minimum–maximum] milliseconds followed by the share of the corresponding total.", "",
                  "| Exclusive reduction phase | " + " | ".join(LABELS[name] for name in order) + " |",
                  "|---|" + "---:|" * len(order)]
        for metric in EXCLUSIVE:
            lines.append("| " + PHASE_LABELS[metric] + " | " + " | ".join(phase_cell(schemes[name], metric, "step_seconds") for name in order) + " |")
        lines += ["", "| Exclusive incoming commitment phase | " + " | ".join(LABELS[name] for name in order) + " |",
                  "|---|" + "---:|" * len(order)]
        for metric in INCOMING:
            lines.append("| " + PHASE_LABELS[metric] + " | " + " | ".join(phase_cell(schemes[name], metric, "incoming_commit_seconds") for name in order) + " |")
        if fresh_costs:
            lines += ["", "| Exclusive fresh commitment phase | " + " | ".join(LABELS[name] for name in order) + " |",
                      "|---|" + "---:|" * len(order)]
            for metric in FRESH:
                lines.append("| " + PHASE_LABELS[metric] + " | " + " | ".join(phase_cell(schemes[name], metric, "fresh_commit_seconds") for name in order) + " |")
    if notes:
        lines += ["", "Data notes:", ""] + ["- " + public_text(note) for note in notes]
    lines += ["", "Raw timing files are preserved. `summary.csv` retains every component's mean, median, minimum, maximum, and sample percentage range. `provenance.json` records raw-source SHA-256 checksums and supplied public run metadata."]
    if metadata:
        lines += ["", "## Supplied run metadata", "", "```json", json.dumps(metadata, indent=2, sort_keys=True), "```"]
    else:
        lines += ["", "No hardware/run provenance was supplied; this report makes no hardware attribution."]
    return "\n".join(lines) + "\n"


def render_plot(groups, destination, mode):
    if mode == "off":
        return False, None
    try:
        import matplotlib
        matplotlib.use("Agg")
        from matplotlib import pyplot as plt
    except ImportError:
        if mode == "on":
            raise ValueError("--plot on requires matplotlib; use --plot auto or off for a standard-library-only report")
        return False, "Matplotlib was unavailable; the CSV and Markdown report contain the full phase breakdown."
    by_size = defaultdict(list)
    for key in sorted(groups, key=sort_key):
        if key[-1] != "canopy_total":
            by_size[key[0]].append(key)
    maximum_bars = max(map(len, by_size.values()))
    figure, axes = plt.subplots(len(by_size), 1, figsize=(max(10, 0.65 * maximum_bars), 3.6 * len(by_size)), squeeze=False)
    colors = ["#465b8c", "#ad5c50", "#68946f", "#cfaa53", "#917caa", "#75a2ac", "#a5a5a5"]
    for index, (log_n, keys) in enumerate(sorted(by_size.items())):
        axis = axes[index, 0]
        bottom = [0.0] * len(keys)
        for component, color in zip(COARSE, colors):
            values = [1000 * statistics.mean(row[component] for row in groups[key]) for key in keys]
            axis.bar(range(len(keys)), values, bottom=bottom, width=0.8, label=PHASE_LABELS[component], color=color)
            bottom = [old + value for old, value in zip(bottom, values)]
        axis.set_title(f"N = 2^{log_n} field elements", loc="left")
        axis.set_ylabel("Mean reduction time (ms)")
        axis.set_xticks(range(len(keys)))
        axis.set_xticklabels([f"{LABELS[key[-1]]}\ncast={key[1]}, acc={key[2]}" for key in keys], rotation=35, ha="right", fontsize=8)
        for side in ("top", "right"):
            axis.spines[side].set_visible(False)
        axis.grid(axis="y", color="#dddddd", linewidth=0.5)
        axis.set_axisbelow(True)
    handles, labels = axes[0, 0].get_legend_handles_labels()
    figure.legend(handles, labels, loc="upper center", ncol=4, frameon=False, fontsize=9, bbox_to_anchor=(0.5, 0.99))
    figure.suptitle("Section 8 measured prover phases", y=1.01)
    figure.tight_layout(rect=(0, 0, 1, 0.94))
    figure.savefig(destination, format="svg", bbox_inches="tight", metadata={"Creator": "Section 8 phase profile reporter", "Date": None})
    plt.close(figure)
    return True, None


def source(path):
    return {"path": public_text(str(path.resolve())), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}


def read_metadata(paths):
    allowed = {"workload", "cost_scope", "hardware", "cpu", "machine", "platform", "os", "architecture", "cores", "memory", "memory_bytes", "physical_memory_bytes", "threads", "field", "hash", "command", "argv", "git_commit", "git_dirty", "binary_sha256", "rustc", "compiler", "features", "build_features", "source_stable_through_build", "started_unix", "wall_seconds", "exit_code", "peak_rss_bytes", "peak_rss_gib", "user_seconds", "system_seconds", "memory_boundary", "measurement_scope", "parameters", "status", "verified", "process_policy", "source_scope", "source_and_binary_stable", "profiles_sha256", "proof_configuration", "actual_rows", "expected_rows", "expected_jobs", "created_utc", "completed_utc"}
    output = []
    for path in paths:
        value = json.loads(path.read_text())
        if not isinstance(value, dict):
            raise ValueError(f"{public_text(str(path))}: provenance must be a JSON object")
        output.append({"source": source(path), "metadata": public_metadata({key: value[key] for key in value if key in allowed})})
    return output


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--timings", type=Path, nargs="+", required=True)
    parser.add_argument("--provenance", type=Path, nargs="+", default=[])
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--plot", choices=("auto", "on", "off"), default="auto")
    args = parser.parse_args()
    try:
        inputs = {path.resolve() for path in args.timings + args.provenance}
        for name in ("summary.csv", "README.md", "provenance.json", "phase_profiles.svg"):
            if (args.output_dir / name).resolve() in inputs:
                raise ValueError("The report would overwrite an input source")
        groups, notes = read_samples(args.timings)
        rows = summary_rows(groups)
        metadata = read_metadata(args.provenance)
        workload = single_workload(row for samples in groups.values() for row in samples)
        validate_metadata(workload, metadata)
        scope = cost_scope(groups)
        for record in metadata:
            for declared in (record["metadata"].get("cost_scope"), record["metadata"].get("proof_configuration", {}).get("cost_scope")):
                if declared and declared != scope:
                    raise ValueError(f"Supplied provenance cost scope {declared!r} differs from CSV cost scope {scope!r}")
        # Check the selected-height report layout before creating output files.
        render_markdown(groups, notes, False, metadata)
        args.output_dir.mkdir(parents=True, exist_ok=True)
        plot_created, plot_note = render_plot(groups, args.output_dir / "phase_profiles.svg", args.plot)
        if plot_note:
            notes.append(plot_note)
        write_csv(args.output_dir / "summary.csv", rows)
        (args.output_dir / "README.md").write_text(render_markdown(groups, notes, plot_created, metadata))
        provenance = {"workload": workload, "cost_scope": scope, "timings": [source(path) for path in args.timings], "source_metadata": metadata, "reported_groups": len(groups),
                      "phase_scope": "18 exclusive reduction components; 6 separate incoming components; optional 6 fresh-commitment components; parent phase totals overlap their children",
                      "total_prover_scope": "ell_cast fresh input commitments + reduction; excludes cached prior commitments, input claim preparation, verification and final decider" if scope == COST_SCOPE else "unavailable: historical incoming times do not separate fresh and prior commitments",
                      "percentage_method": "mean component / corresponding mean total; per-sample percentage ranges also retained",
                      "pairing": "matching parameters, repetition, and input_seed before aggregation", "reconciliation_tolerance": {"relative": 1e-6, "absolute_seconds": 5e-9},
                      "plot_created": plot_created, "notes": notes}
        (args.output_dir / "provenance.json").write_text(json.dumps(public_metadata(provenance), indent=2, sort_keys=True) + "\n")
    except (OSError, ValueError) as error:
        parser.error(str(error))
    print(f"Reported {len(groups)} measured reduction groups and {len(rows)} component summaries in {public_text(str(args.output_dir))}")


if __name__ == "__main__":
    main()
