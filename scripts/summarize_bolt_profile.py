#!/usr/bin/env python3
"""Validate and summarize the disjoint Bolt prover stage measurements (stdlib only)."""

import argparse
import csv
import math
import statistics as stats
from collections import defaultdict
from pathlib import Path


GROUPS = [
    ("commitment", "Bolt sparse syndrome U=HX", ["sparse syndrome U=HX"]),
    ("commitment", "Bolt parity RS encoding", ["parity RS encoding Y=RS(U)"]),
    ("commitment", "X/Y Merkle trees or bottom forests", ["row Merkle tree or forest"]),
    ("commitment", "Canopy hash-to-field", ["Canopy hash-to-field"]),
    ("commitment", "Canopy RS encode and oracle commitment", ["Canopy WHIR configuration", "Canopy RS encode and WHIR oracle commitment", "Canopy constant RS commitment"]),
    ("commitment", "Root binding and residual", ["Merkle root transcript", "unassigned phase work and timing overhead"]),
    ("opening", "Bolt partial evaluation g", ["partial evaluation g"]),
    ("opening", "Bolt column projections", ["column projections x=Xr and u=Ur"]),
    ("opening", "Common WHIR initial RS commitments", ["WHIR configuration", "common WHIR RS encode and oracle commitment"]),
    ("opening", "Syndrome evaluation and query sampling", ["syndrome evaluation sigma", "sample row queries"]),
    ("opening", "Copy and serialize queried row values", ["copy and serialize queried row values"]),
    ("opening", "Collect and serialize authentication paths", ["collect and serialize row authentication paths"]),
    ("opening", "Project and batch queried rows", ["project queried rows and sample batching challenges", "systematic query covector"]),
    ("opening", "Parity query covector / transpose RS FFT", ["parity query covector and transpose RS FFT"]),
    ("opening", "Public H transpose covector", ["public H transpose covector"]),
    ("opening", "Common WHIR constrained proofs", ["common WHIR constrained proof"]),
    ("opening", "Canopy claims, OOD and constant opening", ["Canopy ancestor claims", "Canopy SEP batching covector", "Canopy OOD evaluations", "Canopy constant opening"]),
    ("opening", "Canopy WHIR SEP proof", ["Canopy WHIR SEP proof"]),
    ("opening", "Residual opening work", ["unassigned phase work and timing overhead"]),
]
LOOKUP = {(phase, stage): i for i, (phase, _, names) in enumerate(GROUPS) for stage in names}


def read(path):
    with path.open() as file:
        return list(csv.DictReader(file))


def write_csv(path, rows):
    with path.open("w", newline="") as file:
        writer = csv.DictWriter(file, fieldnames=rows[0].keys())
        writer.writeheader()
        writer.writerows(rows)


def summarize(root, report_file="summary.md"):
    all_components, all_runs, profiles = [], [], {}
    directories = {}
    schemes, layout = None, None
    for n in (18, 22):
        for width in (4, 8, 16):
            candidates = [root / f"n{n}" / f"width_{width}",
                          root / f"width_{width}" if n == 18 else root / f"timing_n22/width_{width}"]
            directory = next(d for d in candidates if (d / "prover_timings.csv").exists())
            directories[n, width] = directory.relative_to(root).as_posix()
            runs = read(directory / "prover_timings.csv")
            current_schemes = ("bolt_max_merkle", next(r["scheme"] for r in runs if r["scheme"] != "bolt_max_merkle"))
            current_layout = runs[0].get("canopy_layout", "quarter_each")
            if schemes is None:
                schemes, layout = current_schemes, current_layout
            assert schemes == current_schemes and layout == current_layout
            assert all(r.get("canopy_layout", "quarter_each") == layout for r in runs)
            stages = read(directory / "prover_stage_timings.csv")
            by_run = defaultdict(list)
            for stage in stages:
                assert int(stage["log_input"]) == n and int(stage["interleaving"]) == width
                by_run[(stage["scheme"], int(stage["iteration"]))].append(stage)
                assert (stage["phase"], stage["stage"]) in LOOKUP, stage
            assert len(by_run) == len(runs)
            grouped = {s: [r for r in runs if r["scheme"] == s] for s in schemes}
            assert len(grouped[schemes[0]]) == len(grouped[schemes[1]]) > 1
            counts = {s: len(grouped[s]) for s in schemes}
            for run in runs:
                assert run["proof_verified"] == "true"
                key = (run["scheme"], int(run["iteration"]))
                entries = by_run[key]
                assert len({(p["phase"], p["part"], p["stage"]) for p in entries}) == len(entries)
                for phase in ("commitment", "opening"):
                    actual = sum(float(p["seconds"]) for p in entries if p["phase"] == phase)
                    assert math.isclose(actual, float(run[phase + "_seconds"]), abs_tol=1e-9, rel_tol=0)
                total = sum(float(p["seconds"]) for p in entries)
                assert math.isclose(total, float(run["total_prover_seconds"]), abs_tol=1e-9, rel_tol=0)
            amounts = [[0.0, 0.0] for _ in GROUPS]
            for stage in stages:
                i = LOOKUP[(stage["phase"], stage["stage"])]
                j = schemes.index(stage["scheme"])
                amounts[i][j] += 1000 * float(stage["seconds"]) / counts[stage["scheme"]]
            for (phase, name, _), (merkle, canopy) in zip(GROUPS, amounts):
                all_components.append(dict(log_input=n, interleaving=width, phase=phase, component=name,
                                           merkle_mean_ms=merkle, canopy_mean_ms=canopy, difference_ms=canopy-merkle))
            for scheme, group in grouped.items():
                values = [1000 * float(r["total_prover_seconds"]) for r in group]
                for j, phase in enumerate(("commitment", "opening")):
                    mean = stats.mean(1000 * float(r[phase + "_seconds"]) for r in group)
                    component_total = sum(amounts[i][schemes.index(scheme)] for i, g in enumerate(GROUPS) if g[0] == phase)
                    assert math.isclose(mean, component_total, abs_tol=1e-6, rel_tol=0)
                all_runs.append(dict(log_input=n, interleaving=width, scheme=scheme, repetitions=len(group),
                                     mean_ms=stats.mean(values), median_ms=stats.median(values),
                                     minimum_ms=min(values), maximum_ms=max(values), sample_sd_ms=stats.stdev(values),
                                     commitment_mean_ms=stats.mean(1000*float(r["commitment_seconds"]) for r in group),
                                     opening_mean_ms=stats.mean(1000*float(r["opening_seconds"]) for r in group)))
            paired = [{int(r["iteration"]): 1000*float(r["total_prover_seconds"]) for r in grouped[s]} for s in schemes]
            assert paired[0].keys() == paired[1].keys()
            differences = [paired[1][i] - paired[0][i] for i in paired[0]]
            profiles[n, width] = (amounts, grouped, differences)

    write_csv(root / "stage_comparison.csv", all_components)
    write_csv(root / "total_comparison.csv", all_runs)
    divisors = {"quarter_each": (4, 16), "equal_blocks": (4, 4),
                "equal_half": (2, 2), "equal_full": (1, 1)}
    x_divisor, y_divisor = divisors[layout]
    def block_label(divisor):
        return "k" if divisor == 1 else f"k/{divisor}"
    geometry = f"Canopy block lengths {block_label(x_divisor)} for X and {block_label(y_divisor)} for Y"
    measured_count = sum(r["repetitions"] for r in all_runs)
    report = ["# Bolt-max prover time breakdown\n",
              f"Both variants use the same prime-field Bolt-max encoder and common constrained WHIR proofs. Canopy changes the two initial row commitments and adds its own RS/WHIR SEP layer. Parameters: 256-bit field, BLAKE3, initial widths 4/8/16, subsequent WHIR factor 8, {geometry}, proof-size-optimal heights. The 128-bit per-event target is conditional on prime-field LDPC distance 0.013; this is the public-H adaptation and excludes Spark/Mulperm preprocessing.\n",
              f"Fresh profiles use two warmups and ten measured repetitions for each scheme, width, and input size on 8 Rayon threads. All {measured_count} measured proofs verified. Execution order alternates within each width. Raw data and per-part stage tables are linked below. Earlier timing files are preserved.\n",
              "Component tables use **arithmetic mean milliseconds**, so they add to the mean commitment/opening/total times. Median totals and observed variation are shown separately. Stage clocks are disjoint wall-clock intervals; parallel-worker CPU time is not summed. Shared stages execute the same algorithms; measured differences may reflect cache, allocation, and run-to-run variation. These local measurements do not establish speedups at unmeasured input sizes.\n",
              "## Total comparison\n",
              "| Input | Width | Merkle mean ms | Canopy mean ms | Mean overhead | Merkle median ms | Canopy median ms | Paired delta mean ± sample SD ms |",
              "|---:|---:|---:|---:|---:|---:|---:|---:|"]
    for (n, width), (_, grouped, differences) in profiles.items():
        records = [next(r for r in all_runs if r["log_input"] == n and r["interleaving"] == width and r["scheme"] == s) for s in schemes]
        m, c = records
        report.append(f"| 2^{n} | {width} | {m['mean_ms']:.3f} | {c['mean_ms']:.3f} | {100*(c['mean_ms']/m['mean_ms']-1):+.2f}% | {m['median_ms']:.3f} | {c['median_ms']:.3f} | {stats.mean(differences):+.3f} ± {stats.stdev(differences):.3f} |")
    report.append("\nThe paired spread is the sample standard deviation of Canopy-minus-Merkle times for matching repetition indices, not a confidence interval. Each configuration uses its independently selected heights, with message sizes listed below. Full-height constant endpoints add very little cryptographic work; small differences in their total time should not be interpreted as intrinsic speed benefits.\n")
    for (n, width), (amounts, grouped, _) in profiles.items():
        directory = directories[n, width]
        report.append(f"\n## Input 2^{n}, width {width}\n\n[Per-part stage report]({directory}/prover_stage_timings.md) · [Raw stages]({directory}/prover_stage_timings.csv) · [Raw totals]({directory}/prover_timings.csv)\n")
        original = 1 << n
        k = original // width
        canopy = grouped[schemes[1]][0]
        mx, my = k >> int(canopy["systematic_h"]), (k // 4) >> int(canopy["parity_h"])
        systematic_code, parity_code = k // x_divisor, k // y_divisor
        report.append(f"Workload sizes in field elements: Bolt encodes all **{original:,}** original elements, with a parity RS message of {original//8:,}. The common WHIR initial RS messages have {k:,} and {k//8:,} elements. Canopy's RS messages have **{mx:,}** (X) and **{my:,}** (Y), with fixed codeword lengths {systematic_code:,} and {parity_code:,}. A one-element message uses the constant endpoint. These RS stage times are not a comparison against full-input RS encoding.\n")
        report.append("| Phase | Component | Merkle mean ms | Canopy mean ms | Difference ms |\n|---|---|---:|---:|---:|")
        for phase in ("commitment", "opening"):
            sums = [0.0, 0.0]
            for (p, name, _), values in zip(GROUPS, amounts):
                if p != phase:
                    continue
                m, c = values
                sums[0] += m; sums[1] += c
                report.append(f"| {phase} | {name} | {m:.3f} | {c:.3f} | {c-m:+.3f} |")
            report.append(f"| **{phase}** | **Subtotal** | **{sums[0]:.3f}** | **{sums[1]:.3f}** | **{sums[1]-sums[0]:+.3f}** |")
        sums = [sum(values[j] for values in amounts) for j in (0, 1)]
        report.append(f"| **Total** | | **{sums[0]:.3f}** | **{sums[1]:.3f}** | **{sums[1]-sums[0]:+.3f}** |")
    report.extend(["\n## Measurement boundaries\n",
                   "Commitment includes Bolt syndrome formation and parity RS encoding. Opening includes the common proofs as well as extra Canopy work. Setup, input generation, verification, proof-object extraction, and final cleanup outside the existing phase timers are excluded. Residual time captures unassigned operations, allocation/drop, and timer bookkeeping inside those boundaries.\n",
                   "RS encoding plus WHIR oracle commitment is one native call, including the FFT, matrix hashing, allocation, and transcript binding. WHIR proof time includes its internal sumchecks, recursive oracles, query openings, and terminal messages; those internals are not individually timed here. Any part using a constant endpoint contributes no nonconstant RS FFT or WHIR SEP proof.\n",
                   "Rebuild this combined report using `python3 scripts/summarize_bolt_profile.py`; regenerate the underlying profiles with the commands in `docs/experiment_bolt_max.md`. Input sizes 2^26 and 2^30 have proof-size estimates only.\n"])
    audit = root.parent / "bolt_encoder_audit/n22_w4/summary.md"
    if audit.exists():
        report.append("\nSee the [full-input RS encoder audit](../bolt_encoder_audit/n22_w4/summary.md) for a comparison with Bolt at the same input size. The paper's 3–4x number concerns optimized binary-field commitment time; the current 256-bit-prime implementation is a different performance setting.\n")
    (root / report_file).write_text("\n".join(report))
    print(f"Validated {sum(r['repetitions'] for r in all_runs)} measured proofs and every stage/phase total; wrote {root / report_file}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input-dir", type=Path, default=Path("results/bolt_max_canopy_prover_profile"))
    parser.add_argument("--report-file", default="summary.md")
    args = parser.parse_args()
    summarize(args.input_dir, args.report_file)
