#!/usr/bin/env python3
"""Validate equal k/4 Canopy blocks and compare against Merkle and prior k/16 parity."""

import argparse
import csv
import math
from pathlib import Path


def read(path):
    with path.open() as file:
        return list(csv.DictReader(file))


def write_csv(path, rows):
    with path.open("w", newline="") as file:
        writer = csv.DictWriter(file, fieldnames=rows[0].keys())
        writer.writeheader()
        writer.writerows(rows)


def compare(root, previous):
    sizes = read(root / "proof_sizes.csv")
    old = {(int(r["log_input"]), int(r["interleaving"])): r for r in read(previous / "proof_sizes.csv")}
    expected = {(n, w) for n in (18, 22, 26, 30) for w in (4, 8, 16)}
    assert {(int(r["log_input"]), int(r["interleaving"])) for r in sizes} == expected
    comparisons, parity_checks = [], []
    candidate_count = 0
    for width in (4, 8, 16):
        directory = root / f"width_{width}"
        heights = read(directory / "height_search.csv")
        details = read(directory / "proof_size_breakdown.csv")
        candidate_count += len(heights)
        for r in read(directory / "proof_sizes.csv"):
            n = int(r["log_input"])
            k = (1 << n) // width
            o = old[n, width]
            assert r["canopy_layout"] == "equal_blocks"
            assert int(r["canopy_rs_length"]) == k // 2
            for field in ("merkle_total_bytes", "merkle_authentication_bytes", "common_iopp_bytes", "opened_row_field_bytes", "seed", "samples", "systematic_queries", "parity_queries"):
                assert r[field] == o[field], (n, width, field)
            for part, key, rows, min_h in (("systematic", "systematic_h", k, 3), ("parity", "parity_h", k//4, 1)):
                candidates = [h for h in heights if int(h["log_input"]) == n and h["part"] == part]
                assert [int(h["h"]) for h in candidates] == list(range(min_h, rows.bit_length()))
                assert all(int(h["canopy_rs_length"]) == k//4 for h in candidates)
                selected = [h for h in candidates if h["selected"] == "true"]
                assert len(selected) == 1 and int(selected[0]["h"]) == int(r[key])
                assert float(selected[0]["authentication_bytes"]) == min(float(h["authentication_bytes"]) for h in candidates)
                if part == "parity":
                    other = min((h for h in candidates if int(h["canopy_dimension"]) > 1), key=lambda h: float(h["authentication_bytes"]))
                    parity_checks.append(dict(log_input=n, interleaving=width, selected_h=r[key],
                                              selected_message_elements=selected[0]["canopy_dimension"],
                                              selected_authentication_bytes=selected[0]["authentication_bytes"],
                                              best_nonconstant_h=other["h"],
                                              best_nonconstant_authentication_bytes=other["authentication_bytes"]))
            for scheme, field in (("bolt_max_merkle", "merkle_total_bytes"), ("bolt_max_canopy_equal_blocks", "canopy_total_bytes")):
                total = sum(float(d["total_bytes"]) for d in details if int(d["log_input"]) == n and d["scheme"] == scheme)
                assert math.isclose(total, float(r[field]), abs_tol=0.0001, rel_tol=0)
            comparisons.append(dict(log_input=n, interleaving=width,
                                    systematic_h=r["systematic_h"], parity_h=r["parity_h"], old_parity_h=o["parity_h"],
                                    merkle_bytes=r["merkle_total_bytes"], old_canopy_bytes=o["canopy_total_bytes"],
                                    new_canopy_bytes=r["canopy_total_bytes"],
                                    saved_vs_merkle_percent=100*(1-float(r["canopy_over_merkle"])),
                                    extra_saved_vs_old_bytes=float(o["canopy_total_bytes"])-float(r["canopy_total_bytes"])))
    comparisons.sort(key=lambda r: (r["log_input"], r["interleaving"]))
    parity_checks.sort(key=lambda r: (r["log_input"], r["interleaving"]))
    write_csv(root / "layout_comparison.csv", comparisons)
    write_csv(root / "parity_height_comparison.csv", parity_checks)
    same = all(r["extra_saved_vs_old_bytes"] == 0 for r in comparisons)
    constant = all(int(r["selected_message_elements"]) == 1 for r in parity_checks)
    report = ["# Equal Canopy block lengths: comparison with Bolt + Merkle\n",
              "Both Canopy RS codewords now have length **k/4**, with k=N/width. Their total is k/2 (40% of Bolt's encoded rows). The systematic rate is 2^(2-h_X); the parity rate is 2^(-h_Y). The search includes h_X=3..log2(k) and h_Y=1..log2(k/4), independently minimizing authentication proof size. Parameters remain the 256-bit prime field, BLAKE3, Bolt-max, and the 128-bit per-event target conditional on LDPC distance 0.013. Common constrained WHIR proofs use the public-H adaptation, excluding Spark/Mulperm preprocessing.\n",
              f"Validated all {len(comparisons)} input/width configurations, {candidate_count} height candidates, and 24 disjoint proof-size totals. There are 100 deterministic samples per height; full proof sizes include commitments and omit serializer framing. The Merkle baseline, common proof costs, row query counts, and seeds match the previous layout exactly.\n"]
    if same and constant:
        report.append("**Result: the enlarged parity block changes neither the selected heights nor the modeled proof sizes in this grid.** Parity still selects the full-height constant endpoint in every case. A length-k/4 constant codeword has the same proof payload as a length-k/16 constant codeword: one root and one field coefficient. Its commitment construction adds two equal-child hash steps; it does not materialize a larger array or run an RS FFT/WHIR proof. The systematic layer is unchanged.\n")
    report.extend(["## Full proof-size comparison\n",
                   "| Input | Width | h X / Y | Merkle MiB | Canopy MiB | Saved vs Merkle % | Additional saving vs old layout KiB |",
                   "|---:|---:|:---:|---:|---:|---:|---:|"])
    for r in comparisons:
        report.append(f"| 2^{r['log_input']} | {r['interleaving']} | {r['systematic_h']} / {r['parity_h']} | {float(r['merkle_bytes'])/2**20:.6f} | {float(r['new_canopy_bytes'])/2**20:.6f} | {r['saved_vs_merkle_percent']:.3f} | {r['extra_saved_vs_old_bytes']/1024:.6f} |")
    report.extend(["\n## Executable prover time\n",
                   "Two warmups and ten measured repetitions per scheme and configuration, 8 Rayon threads. All 120 measured proofs verified. Times below are fresh arithmetic means including encoding, commitment construction, and opening. Setup and verification are excluded. The [stage profile](prover_profile.md) includes median totals, paired variation, and individual stages. Small differences from prior timing runs are not evidence of a geometry-induced speedup.\n",
                   "| Input | Width | Merkle commit ms | Canopy commit ms | Merkle opening ms | Canopy opening ms | Merkle total ms | Canopy total ms | Overhead % |",
                   "|---:|---:|---:|---:|---:|---:|---:|---:|---:|"])
    times = read(root / "total_comparison.csv")
    assert sum(int(r["repetitions"]) for r in times) == 120
    for n in (18, 22):
        for w in (4, 8, 16):
            m = next(r for r in times if int(r["log_input"]) == n and int(r["interleaving"]) == w and r["scheme"] == "bolt_max_merkle")
            c = next(r for r in times if int(r["log_input"]) == n and int(r["interleaving"]) == w and r["scheme"] == "bolt_max_canopy_equal_blocks")
            report.append(f"| 2^{n} | {w} | {float(m['commitment_mean_ms']):.3f} | {float(c['commitment_mean_ms']):.3f} | {float(m['opening_mean_ms']):.3f} | {float(c['opening_mean_ms']):.3f} | {float(m['mean_ms']):.3f} | {float(c['mean_ms']):.3f} | {100*(float(c['mean_ms'])/float(m['mean_ms'])-1):+.2f} |")
    report.extend(["\n## Closest nonconstant parity choices\n",
                   "This shows that nonconstant parity layers were evaluated, including the newly valid heights 1 and 2. Sizes here include only parity authentication, its commitment root, and Canopy proof; row values and common Bolt proofs are excluded.\n",
                   "| Input | Width | Selected h | Selected parity KiB | Best nonconstant h | Nonconstant parity KiB |",
                   "|---:|---:|---:|---:|---:|---:|"])
    for r in parity_checks:
        report.append(f"| 2^{r['log_input']} | {r['interleaving']} | {r['selected_h']} | {float(r['selected_authentication_bytes'])/1024:.3f} | {r['best_nonconstant_h']} | {float(r['best_nonconstant_authentication_bytes'])/1024:.3f} |")
    report.append("\nEach width subdirectory contains the exhaustive height search and full proof breakdown. Input sizes 2^26 and 2^30 were estimated, not timed. Reproduce using the equal-block commands in `docs/experiment_bolt_max.md`, then `python3 scripts/compare_bolt_canopy_layouts.py`.\n")
    (root / "comparison.md").write_text("\n".join(report))
    print(f"Validated {candidate_count} candidates; unchanged sizes={same}, all parity messages constant={constant}. Wrote {root / 'comparison.md'}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input-dir", type=Path, default=Path("results/bolt_max_canopy_equal_blocks"))
    parser.add_argument("--previous-dir", type=Path, default=Path("results/bolt_max_canopy_quarter_width_grid"))
    args = parser.parse_args()
    compare(args.input_dir, args.previous_dir)
