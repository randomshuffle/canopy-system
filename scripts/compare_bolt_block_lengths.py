#!/usr/bin/env python3
"""Validate and compare equal Canopy RS blocks of length k/4, k/2, and k."""

import argparse
import csv
import math
import os
from pathlib import Path


LAYOUTS = (("equal_blocks", 4, 3, 1), ("equal_half", 2, 2, 0), ("equal_full", 1, 1, 0))
CONFIGS = [(n, w) for n in (18, 22, 26, 30) for w in (4, 8, 16)]


def read(path):
    with path.open() as file:
        return list(csv.DictReader(file))


def write(path, rows):
    with path.open("w", newline="") as file:
        writer = csv.DictWriter(file, fieldnames=rows[0].keys())
        writer.writeheader()
        writer.writerows(rows)


def key(row):
    return int(row["log_input"]), int(row["interleaving"])


def label(divisor):
    return "k" if divisor == 1 else f"k/{divisor}"


def compare(roots, output):
    output.mkdir(parents=True, exist_ok=True)
    sizes, times, geometry, time_rows = {}, {}, [], []
    candidate_counts, verified_counts = {}, {}
    baseline_proofs = {}
    reference = {key(r): r for r in read(roots["equal_blocks"] / "proof_sizes.csv")}
    for layout, divisor, min_x, min_y in LAYOUTS:
        root = roots[layout]
        records = read(root / "proof_sizes.csv")
        assert len(records) == len(CONFIGS) and {key(r) for r in records} == set(CONFIGS)
        records = {key(r): r for r in records}
        candidate_counts[layout] = 0
        scheme = f"bolt_max_canopy_{layout}"
        for width in (4, 8, 16):
            directory = root / f"width_{width}"
            heights = read(directory / "height_search.csv")
            details = read(directory / "proof_size_breakdown.csv")
            candidate_counts[layout] += len(heights)
            for r in read(directory / "proof_sizes.csv"):
                n, w = key(r)
                assert w == width and r == records[n, w]
                k = (1 << n) // width
                assert r["canopy_layout"] == layout
                assert int(r["canopy_rs_length"]) == 2 * k // divisor
                assert int(r["target_rbr_bits"]) == 128 and float(r["assumed_ldpc_distance"]) == 0.013
                assert int(r["samples"]) == 100
                for field in ("merkle_total_bytes", "merkle_authentication_bytes", "common_iopp_bytes",
                              "opened_row_field_bytes", "seed", "samples", "systematic_queries", "parity_queries"):
                    assert r[field] == reference[n, w][field], (layout, n, w, field)
                assert int(r["systematic_queries"]) == min(k, 20431)
                assert int(r["parity_queries"]) == 309
                for part, h_field, rows, minimum in (("systematic", "systematic_h", k, min_x),
                                                     ("parity", "parity_h", k // 4, min_y)):
                    candidates = [h for h in heights if int(h["log_input"]) == n and h["part"] == part]
                    assert [int(h["h"]) for h in candidates] == list(range(minimum, rows.bit_length()))
                    assert all(int(h["canopy_rs_length"]) == k // divisor for h in candidates)
                    assert all(int(h["canopy_dimension"]) == rows >> int(h["h"]) for h in candidates)
                    selected = [h for h in candidates if h["selected"] == "true"]
                    assert len(selected) == 1 and int(selected[0]["h"]) == int(r[h_field])
                    assert float(selected[0]["authentication_bytes"]) == min(float(h["authentication_bytes"]) for h in candidates)
                    best_nonconstant = min((h for h in candidates if int(h["canopy_dimension"]) > 1),
                                           key=lambda h: float(h["authentication_bytes"]))
                    geometry.append(dict(canopy_layout=layout, log_input=n, interleaving=w, part=part,
                                         rs_length=k // divisor, selected_h=r[h_field],
                                         selected_message_elements=selected[0]["canopy_dimension"],
                                         selected_authentication_bytes=selected[0]["authentication_bytes"],
                                         best_nonconstant_h=best_nonconstant["h"],
                                         best_nonconstant_authentication_bytes=best_nonconstant["authentication_bytes"]))
                for s, field in (("bolt_max_merkle", "merkle_total_bytes"), (scheme, "canopy_total_bytes")):
                    total = sum(float(d["total_bytes"]) for d in details if int(d["log_input"]) == n and d["scheme"] == s)
                    assert math.isclose(total, float(r[field]), abs_tol=0.0001, rel_tol=0), (layout, n, w, s)
                sizes[layout, n, w] = r
        totals = read(root / "total_comparison.csv")
        assert len(totals) == 12
        verified_counts[layout] = 0
        for n in (18, 22):
            for w in (4, 8, 16):
                directory = root / (f"width_{w}" if n == 18 else f"timing_n22/width_{w}")
                selected = read(directory / "timing_height_search/proof_sizes.csv")
                assert selected == [records[n, w]]
                runs = read(directory / "prover_timings.csv")
                assert len(runs) == 20
                for run in runs:
                    assert key(run) == (n, w) and run["canopy_layout"] == layout
                    assert run["proof_verified"] == "true" and int(run["rayon_threads"]) == 8
                    assert math.isclose(float(run["commitment_seconds"]) + float(run["opening_seconds"]),
                                        float(run["total_prover_seconds"]), abs_tol=1e-9, rel_tol=0)
                    if run["scheme"] == scheme:
                        assert run["systematic_h"] == records[n, w]["systematic_h"]
                        assert run["parity_h"] == records[n, w]["parity_h"]
                    else:
                        assert run["scheme"] == "bolt_max_merkle"
                        baseline_key = (n, w, int(run["iteration"]))
                        baseline_proofs.setdefault(baseline_key, run["proof_bytes"])
                        assert run["proof_bytes"] == baseline_proofs[baseline_key]
                verified_counts[layout] += len(runs)
                m, c = [next(r for r in totals if key(r) == (n, w) and r["scheme"] == s)
                        for s in ("bolt_max_merkle", scheme)]
                assert int(m["repetitions"]) == int(c["repetitions"]) == 10
                times[layout, n, w] = (m, c)
                time_rows.append(dict(canopy_layout=layout, log_input=n, interleaving=w,
                                      merkle_median_ms=m["median_ms"], canopy_median_ms=c["median_ms"],
                                      median_overhead_percent=100 * (float(c["median_ms"]) / float(m["median_ms"]) - 1),
                                      merkle_commitment_mean_ms=m["commitment_mean_ms"], canopy_commitment_mean_ms=c["commitment_mean_ms"],
                                      merkle_opening_mean_ms=m["opening_mean_ms"], canopy_opening_mean_ms=c["opening_mean_ms"],
                                      merkle_total_mean_ms=m["mean_ms"], canopy_total_mean_ms=c["mean_ms"]))

    size_rows = []
    for n, w in CONFIGS:
        row = dict(log_input=n, interleaving=w, merkle_bytes=reference[n, w]["merkle_total_bytes"])
        for layout, _, _, _ in LAYOUTS:
            r = sizes[layout, n, w]
            row[f"{layout}_bytes"] = r["canopy_total_bytes"]
            row[f"{layout}_saved_percent"] = 100 * (1 - float(r["canopy_over_merkle"]))
        size_rows.append(row)
    write(output / "proof_size_comparison.csv", size_rows)
    write(output / "height_comparison.csv", geometry)
    write(output / "prover_time_comparison.csv", time_rows)
    fresh_candidates = sum(candidate_counts[l] for l in ("equal_half", "equal_full"))
    fresh_verified = sum(verified_counts[l] for l in ("equal_half", "equal_full"))
    report = ["# Bolt-max: Canopy RS block-length sweep\n",
              "Both Canopy RS blocks have length k/4, k/2, or k, where k=N/width is the number of systematic rows. Bolt's sketch matrix always has k/4 rows. Widths are 4, 8, and 16. Each layout independently minimizes proof size over both heights; executable timings use those selected heights.\n",
              "All variants use the same 256-bit prime field, BLAKE3, degree-8/compression-8 Bolt-max encoder, and common constrained WHIR proofs. The 128-bit per-event target is conditional on prime-field LDPC distance 0.013. Systematic/sketch query budgets remain min(k,20431) and 309. The public-H adaptation excludes Spark/Mulperm preprocessing.\n",
              f"Checked {fresh_candidates} new height candidates across 24 new configurations, plus {candidate_counts['equal_blocks']} existing k/4 candidates. All 72 disjoint proof-size totals reconcile. Sizes use 100 deterministic samples per height, include commitment roots, and omit serializer framing. The Merkle baseline, common proofs, row values, and sampling seeds are identical across layouts.\n",
              "## Full proof sizes\n",
              "MiB = 2^20 bytes. Savings are relative to Bolt + Merkle at the same input size and width.\n",
              "| Input | Width | Merkle MiB | k/4 MiB | k/2 MiB | k MiB | k/2 saved % | k saved % |",
              "|---:|---:|---:|---:|---:|---:|---:|---:|"]
    for r in size_rows:
        report.append(f"| 2^{r['log_input']} | {r['interleaving']} | {float(r['merkle_bytes'])/2**20:.6f} | {float(r['equal_blocks_bytes'])/2**20:.6f} | {float(r['equal_half_bytes'])/2**20:.6f} | {float(r['equal_full_bytes'])/2**20:.6f} | {r['equal_half_saved_percent']:.3f} | {r['equal_full_saved_percent']:.3f} |")
    largest = next(r for r in size_rows if r["log_input"] == 30 and r["interleaving"] == 4)
    if all(float(r["equal_full_bytes"]) <= min(float(r["equal_half_bytes"]), float(r["equal_blocks_bytes"])) for r in size_rows):
        report.append("\nLength k yields the smallest modeled proof in every configuration, allowing ties. "
                      f"At input 2^30, width 4, savings versus Merkle increase from {largest['equal_blocks_saved_percent']:.2f}% "
                      f"(k/4) to {largest['equal_half_saved_percent']:.2f}% (k/2) to {largest['equal_full_saved_percent']:.2f}% (k).\n")
    if all(int(sizes[l, n, w]["systematic_h"]) == min_x for l, _, min_x, _ in LAYOUTS for n, w in CONFIGS if n >= 22):
        report.append("For every input at least 2^22, the systematic height is 3, 2, or 1 as the block grows from k/4 to k/2 to k. "
                      "The selected systematic RS rate stays 1/2, while its message length grows from k/8 to k/4 to k/2. "
                      "This reduces bottom authentication paths and increases H2F, RS commitment, and WHIR proving work.\n")
    if all(int(g["selected_message_elements"]) == 1 for g in geometry if g["part"] == "parity"):
        report.append("The sketch layer chooses its full-height constant endpoint in all configurations, including after testing height zero in the enlarged layouts. "
                      "Its block enlargement adds only equal-child hash steps to commitment construction, without a materialized RS array or a WHIR SEP proof.\n")
    report.extend(["\n## Selected heights\n",
                   "Height counts binary levels above whole row hashes. At height zero the Canopy message contains one H2F value per row, with no bottom authentication paths. At full height the message is one field element; its constant RS commitment requires no FFT or WHIR SEP proof.\n",
                   "| Input | Width | k/4 h_X / h_Y | k/2 h_X / h_Y | k h_X / h_Y |",
                   "|---:|---:|:---:|:---:|:---:|"])
    for n, w in CONFIGS:
        hs = [f"{sizes[l,n,w]['systematic_h']} / {sizes[l,n,w]['parity_h']}" for l, _, _, _ in LAYOUTS]
        report.append(f"| 2^{n} | {w} | {' | '.join(hs)} |")
    report.extend(["\n## Total prover time\n",
                   f"The two enlarged layouts have {fresh_verified} freshly measured and verified proofs in total. Each configuration uses eight Rayon threads, two warmups, and ten measured repetitions per scheme; Merkle/Canopy execution order alternates. Layouts execute sequentially. The k/4 measurements are the earlier run (120 verified proofs), retained for context. Compare each Canopy time against its accompanying Merkle baseline; raw cross-run changes can include machine variation.\n",
                   "Median total times include encoding, commitment construction, and opening. Setup, input generation, verification, and cleanup outside the established timing boundaries are excluded. Input sizes 2^26 and 2^30 are estimated, not timed.\n",
                   "| Input | Width | Each block | Merkle median ms | Canopy median ms | Median overhead % |",
                   "|---:|---:|---|---:|---:|---:|"])
    for n in (18, 22):
        for w in (4, 8, 16):
            for layout, divisor, _, _ in LAYOUTS:
                m, c = times[layout, n, w]
                report.append(f"| 2^{n} | {w} | {label(divisor)} | {float(m['median_ms']):.3f} | {float(c['median_ms']):.3f} | {100*(float(c['median_ms'])/float(m['median_ms'])-1):+.2f} |")
    report.extend(["\n## Commitment and opening breakdown\n",
                   "Arithmetic mean milliseconds, so commitment plus opening adds to the mean total. Detailed profiles below include all disjoint stages and observed variation.\n",
                   "| Input | Width | Each block | Merkle commit | Canopy commit | Merkle opening | Canopy opening | Merkle total | Canopy total |",
                   "|---:|---:|---|---:|---:|---:|---:|---:|---:|"])
    for n in (18, 22):
        for w in (4, 8, 16):
            for layout, divisor, _, _ in LAYOUTS:
                m, c = times[layout, n, w]
                vals = [float(r[f]) for f in ("commitment_mean_ms", "opening_mean_ms", "mean_ms") for r in (m, c)]
                report.append(f"| 2^{n} | {w} | {label(divisor)} | " + " | ".join(f"{v:.3f}" for v in vals) + " |")
    report.append("\n## Data and reproduction\n")
    for layout, divisor, _, _ in LAYOUTS:
        relative = Path(os.path.relpath(roots[layout], output)).as_posix()
        report.append(f"- **{label(divisor)} each:** [stage profile]({relative}/prover_profile.md), [proof sizes]({relative}/proof_sizes.csv), exhaustive height searches and proof components in each `width_*` directory.")
    report.append("\nThe local [experiment notes](../../docs/experiment_bolt_max.md) contain reproduction commands and soundness assumptions. Rebuild this report with `python3 scripts/compare_bolt_block_lengths.py`.\n")
    (output / "comparison.md").write_text("\n".join(report))
    print(f"Validated new candidates={fresh_candidates}, fresh verified proofs={fresh_verified}, 72 proof component totals. Wrote {output / 'comparison.md'}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--quarter-dir", type=Path, default=Path("results/bolt_max_canopy_equal_blocks"))
    parser.add_argument("--half-dir", type=Path, default=Path("results/bolt_max_canopy_equal_half"))
    parser.add_argument("--full-dir", type=Path, default=Path("results/bolt_max_canopy_equal_full"))
    parser.add_argument("--output-dir", type=Path, default=Path("results/bolt_max_canopy_block_lengths"))
    args = parser.parse_args()
    compare(dict(equal_blocks=args.quarter_dir, equal_half=args.half_dir, equal_full=args.full_dir), args.output_dir)
