#!/usr/bin/env python3
"""Summarize measured Bolt process memory and the larger-input timing probe."""

import csv
import json
import math
from pathlib import Path
import statistics as stats


ROOT = Path("results/bolt_max_canopy_memory")
DIRECTORIES = ("n22_w4_equal_full_rusage", "n24_w4_equal_full", "n24_w4_equal_half")


def main():
    comparisons = []
    measured_proofs = 0
    for name in DIRECTORIES:
        directory = ROOT / name
        memory = json.loads((directory / "process_memory.json").read_text())
        assert memory["exit_code"] == 0 and memory["peak_rss_bytes"] > memory["input_bytes"]
        assert memory["warmups"] == 2 and memory["repetitions"] == 10 and memory["rayon_threads"] == 8
        with (directory / "prover_timings.csv").open() as file:
            runs = list(csv.DictReader(file))
        with (directory / "prover_stage_timings.csv").open() as file:
            stages = list(csv.DictReader(file))
        assert len(runs) == 20
        for run in runs:
            assert run["proof_verified"] == "true"
            for phase in ("commitment", "opening"):
                total = sum(float(s["seconds"]) for s in stages if s["scheme"] == run["scheme"]
                            and s["iteration"] == run["iteration"] and s["phase"] == phase)
                assert math.isclose(total, float(run[phase + "_seconds"]), abs_tol=1e-9, rel_tol=0)
            assert math.isclose(float(run["commitment_seconds"]) + float(run["opening_seconds"]),
                                float(run["total_prover_seconds"]), abs_tol=1e-9, rel_tol=0)
        measured_proofs += len(runs)
        with (directory / "timing_height_search/proof_sizes.csv").open() as file:
            size = next(csv.DictReader(file))
        m = [float(r["total_prover_seconds"]) for r in runs if r["scheme"] == "bolt_max_merkle"]
        c = [float(r["total_prover_seconds"]) for r in runs if r["scheme"] != "bolt_max_merkle"]
        for run in runs:
            if run["scheme"] != "bolt_max_merkle":
                assert run["systematic_h"] == size["systematic_h"] and run["parity_h"] == size["parity_h"]
        comparisons.append(dict(directory=name, log_input=memory["log_input"], interleaving=4,
                                canopy_layout=memory["canopy_layout"], peak_rss_gib=memory["peak_rss_gib"],
                                input_mib=memory["input_bytes"]/2**20, systematic_h=size["systematic_h"],
                                parity_h=size["parity_h"], merkle_proof_mib=float(size["merkle_total_bytes"])/2**20,
                                canopy_proof_mib=float(size["canopy_total_bytes"])/2**20,
                                proof_saved_percent=100*(1-float(size["canopy_over_merkle"])),
                                merkle_median_seconds=stats.median(m), canopy_median_seconds=stats.median(c),
                                overhead_percent=100*(stats.median(c)/stats.median(m)-1),
                                merkle_mean_seconds=stats.mean(m), canopy_mean_seconds=stats.mean(c),
                                merkle_sample_sd_seconds=stats.stdev(m), canopy_sample_sd_seconds=stats.stdev(c)))
    with (ROOT / "comparison.csv").open("w", newline="") as file:
        writer = csv.DictWriter(file, fieldnames=comparisons[0].keys())
        writer.writeheader()
        writer.writerows(comparisons)
    report = ["# Bolt-max memory and larger-input experiment\n",
              "This machine has 16 GiB of physical RAM. The original timing runs did not collect process memory. These fresh runs measure peak resident set size through `getrusage(RUSAGE_CHILDREN)` in a separate parent process. On macOS the reported maximum RSS is in bytes. Each child runs Merkle and Canopy sequentially; this is the maximum over the entire process, including height search, public-H setup, input generation, proving, verification, and allocator-retained memory. It is not the sum of simultaneous provers or a separate peak for each scheme.\n",
              "Parameters: width 4, 256-bit prime field, BLAKE3, Bolt-max, unchanged conditional distance assumption 0.013 and 128-bit per-event target. Both Canopy RS blocks have length k/2 or k. Heights minimize proof size using 100 samples per candidate. Public H is checked directly, excluding Spark/Mulperm preprocessing.\n",
              f"Each configuration uses 2 warmups, 10 measured repetitions per scheme, and 8 Rayon threads. All {measured_proofs} measured proofs verified, and every commitment/opening stage total reconciles. Prover medians below include encoding, commitment construction, and opening; setup and verification are excluded from prover timings but included in the memory measurement.\n",
              "## Measured memory\n",
              "| Input | Both Canopy blocks | Input alone MiB | Whole-process peak RSS GiB |",
              "|---:|---|---:|---:|"]
    for r in comparisons:
        block = "k" if r["canopy_layout"] == "equal-full" else "k/2"
        report.append(f"| 2^{r['log_input']} | {block} | {r['input_mib']:.0f} | {r['peak_rss_gib']:.3f} |")
    n22 = comparisons[0]
    n24 = next(r for r in comparisons if r["log_input"] == 24 and r["canopy_layout"] == "equal-full")
    report.append(f"\nIncreasing N by 4 increases peak RSS by {n24['peak_rss_gib']/n22['peak_rss_gib']:.2f}x. "
                  f"A linear projection from the measured 2^24 full-block run puts 2^26 at approximately {4*n24['peak_rss_gib']:.1f} GiB for this executable alone, "
                  f"leaving about {16-4*n24['peak_rss_gib']:.1f} GiB for the OS and all other applications on this machine. "
                  "That offers little headroom for the current desktop workload. This is an extrapolation, not a measured 2^26 run. "
                  "The input vector alone at 2^30 would require 32 GiB. Input 2^18 is excluded from future default Bolt grids and timings; historical results remain available.\n")
    report.extend(["## Proof size and prover time\n",
                   "| Input | Both blocks | h_X / h_Y | Merkle MiB | Canopy MiB | Proof saved % | Merkle median s | Canopy median s | Time overhead % |",
                   "|---:|---|:---:|---:|---:|---:|---:|---:|---:|"])
    for r in comparisons:
        block = "k" if r["canopy_layout"] == "equal-full" else "k/2"
        report.append(f"| 2^{r['log_input']} | {block} | {r['systematic_h']} / {r['parity_h']} | {r['merkle_proof_mib']:.6f} | {r['canopy_proof_mib']:.6f} | {r['proof_saved_percent']:.2f} | {r['merkle_median_seconds']:.6f} | {r['canopy_median_seconds']:.6f} | {r['overhead_percent']:+.2f} |")
    report.extend(["\n## Measurement data\n",
                   "Raw process resource accounting, individual timing repetitions, and disjoint stage tables are preserved below. Compare each Canopy timing with its accompanying Merkle baseline; measurements include run-to-run variation.\n"])
    for r in comparisons:
        d = r["directory"]
        report.append(f"- **{d}:** [process memory]({d}/process_memory.json), [timings]({d}/prover_timings.md), [stage breakdown and variation]({d}/prover_stage_timings.md), [selected heights and sizes]({d}/timing_height_search/summary.md).")
    report.append("\nReproduce with `archive/scripts/measure_bolt_memory.py` and rebuild this report with `python3 archive/scripts/summarize_bolt_memory.py`. "
                  "The initial `/usr/bin/time -l` probe could not collect memory because its clock-rate sysctl is restricted in this environment; "
                  "the reported measurements use the successful child-process accounting method instead.\n")
    (ROOT / "summary.md").write_text("\n".join(report))
    print(f"Validated {measured_proofs} verified proofs and stage totals; wrote {ROOT / 'summary.md'}")


if __name__ == "__main__":
    main()
