#!/usr/bin/env python3
"""Summarize measured batching primitives; does not estimate an RS0 reduction."""
import csv
import itertools
import json
import math
import statistics
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "results/accumulation_pending_reduction"


def occupied(nodes, queries):
    return 1.0 if nodes == 1 else -nodes * math.expm1(queries * math.log1p(-1 / nodes))


def auth_hashes(log_leaves, queries):
    return sum(occupied(2**level, queries) for level in range(log_leaves + 1))


# Exhaustively verify leaf-plus-distinct-parent counting on a small tree.
observed = []
for sample in itertools.product(range(4), repeat=3):
    nodes = set(sample)
    count = len(nodes)
    for _ in range(2):
        nodes = {i // 2 for i in nodes}
        count += len(nodes)
    observed.append(count)
assert abs(statistics.mean(observed) - auth_hashes(2, 3)) < 1e-12

hash_rows = []
for n in range(18, 31, 2):
    for label, sources, log_rate, log_message in (
        ("RS0 batch", 2, 1, n),
        ("RS1 batch, 2 sources ONLY", 2, 3, n - 3),
        ("RS1 batch, 3 sources ONLY", 3, 3, n - 3),
    ):
        q = math.ceil(136 / -math.log2(1.05 * 2 ** (-log_rate / 2)))
        auth = sources * auth_hashes(log_message + log_rate, q)
        fs = log_message + 4
        hash_rows.append(dict(log_N=n, component=label, queries=q,
                              authentication_hashes=auth, fs_finalizations=fs,
                              total_hashes=auth + fs))
(OUT / "hash_estimates.json").write_text(json.dumps(hash_rows, indent=2) + "\n")

lines = [
    "# Accumulation batching primitives — reduction choice pending", "",
    "These are component measurements, not an end-to-end Canopy accumulation benchmark. "
    "In particular, every RS1 row excludes the old RS0 claim's reduction, its "
    "Canopy paths, and Canopy-opening SEP construction. No Canopy speedup is claimed.", "",
    "Parameters: GF(2^256), BLAKE3-256, additive RS, scalar leaves and interleaving depth one, "
    "136-bit local query budget (targeting 128-bit composition), eight Rayon workers, "
    "one warmup and five measured repetitions. N is the RS0 message length. "
    "RS0 has block length 2N; RS1 has message N/8 and block length N.", "",
    "The local machine is the Apple M2 used by the repository's prior local runs. "
    "Measurements are sequential but the desktop was not isolated. Input generation, "
    "public FFT setup, input commitments, input-sigma generation, and verification "
    "are excluded from the step. Source commitments are regenerated before each trial "
    "and separately timed. The output commitment is included. These are native scalar "
    "RS measurements and cannot be substituted for the prior interleaved WHIR timings.", "",
    "Every source uses a representative compact covector containing one MLE, one "
    "out-of-domain evaluation, and q domain evaluation terms. Covector materialization "
    "is included and separately reported; fresh inputs or sparse Canopy claims may "
    "have cheaper forms. Cost therefore depends on the eventual SEP functional representation.", "",
    "The benchmark executes batched sumcheck to a common point, combines messages, "
    "encodes and commits the combination at the same rate and length, sends one OOD "
    "evaluation, and opens all source roots at common random indices. It replays "
    "sumcheck and Merkle verification and checks the outgoing witness's MLE and RS "
    "query constraints. It is not a complete accumulator API: outgoing SEP serialization, "
    "its scalar batching, decider, and adversarial security tests are not implemented. "
    "The small outgoing scalar computation is not timed; these remain cost estimates.", "",
    "## Measured component costs", "",
    "Medians in milliseconds; min–max is the range of the accounted total. Stage "
    "medians need not sum to the median total.", "",
    "| N | Component | Covector materialization | Sumcheck | Output commitment | Accounted total | Range |",
    "|---|---|---:|---:|---:|---:|---:|",
]
for file, label, offset in (
    ("baseline_rate_half.csv", "RS0 batch, 2 sources", 0),
    ("layer_rate_eighth_two_sources.csv", "RS1 batch, 2 sources ONLY", 3),
    ("layer_rate_eighth_three_sources.csv", "RS1 batch, 3 sources ONLY", 3),
):
    rows = list(csv.DictReader((OUT / file).open()))
    assert all(r["verified"] == "true" for r in rows)
    for log in sorted({int(r["log_message"]) for r in rows}):
        group = [r for r in rows if int(r["log_message"]) == log]
        assert len(group) == 5
        median = lambda key: 1000 * statistics.median(float(r[key]) for r in group)
        totals = [1000 * float(r["accounted_step_seconds"]) for r in group]
        lines.append(f"| 2^{log + offset} | {label} | {median('covector_seconds'):.2f} | "
                     f"{median('batch_sumcheck_seconds'):.2f} | {median('output_commit_seconds'):.2f} | "
                     f"{median('accounted_step_seconds'):.2f} | {min(totals):.2f}–{max(totals):.2f} |")
lines += ["", "## Expected verifier hash invocations", "",
          "These count leaf hashes, distinct parent hashes in merged paths, and "
          "Fiat–Shamir XOF finalizations. They do not count BLAKE3 compression calls "
          "or hash bytes. Outgoing commitments are retained, so their trees are not "
          "opened in this step. Digest-to-field conversion costs zero hashes.", "",
          "For q independent draws with replacement into M leaves, the exact expected "
          "count is H(M,q) = sum over levels j=0..log2(M) of "
          "(M/2^j) [1 − (1 − 2^j/M)^q]. With s source roots the authentication cost "
          "is s H(M,q). The modeled transcript adds log2(k)+4 finalizations. This "
          "transcript count is analytic, not instrumented. q=317 for RS0 and q=96 for RS1.", "",
          "| N | RS0 batch | RS1 two-source batch ONLY | RS1 three-source batch ONLY |",
          "|---|---:|---:|---:|"]
for n in range(18, 31, 2):
    values = [r["total_hashes"] for r in hash_rows if r["log_N"] == n]
    lines.append(f"| 2^{n} | " + " | ".join(f"{v:,.0f}" for v in values) + " |")
lines += ["", "## Protocol decision needed", "",
          "A Canopy opening reduces authentication of a query-answer relation to a "
          "committed RS1 SEP claim. It does not itself reduce SEP[RS0] to SEP[RS1]. "
          "For example, changing only the old SEP claim's sigma to a false value "
          "leaves all Canopy paths unchanged.", "",
          "One option is to fully discharge the old RS0 SEP with an IOPP, using "
          "Canopy for its initial oracle and ordinary Merkle commitments for its "
          "internal oracles. Then only its Canopy-opening obligation remains; "
          "batch that with the existing RS1 accumulator (two sources). Another "
          "option is one three-variable WHIR fold: it leaves a folded-polynomial "
          "RS1 obligation plus a Canopy RS1 obligation, so the existing accumulator "
          "makes three sources. The folded output and Canopy code must use the same "
          "domain/encoding/layout to batch, not just matching lengths and rates.", "",
          "Both choices can carry the new Canopy claim unchanged. The verifier must "
          "check that equality, and the final decider must check both accumulator "
          "components, including the last deferred claim. A formal extraction "
          "composition argument remains separate from this cost study.", "",
          "Reproduce from the repository root:", "", "```sh",
          "cargo build --offline --release --example accumulation_primitives",
          "RAYON_NUM_THREADS=8 target/release/examples/accumulation_primitives --logs 18,20,22 --output results/accumulation_pending_reduction/baseline_rate_half.csv",
          "RAYON_NUM_THREADS=8 target/release/examples/accumulation_primitives --logs 15,17,19 --log-inverse-rate 3 --sources 2 --output results/accumulation_pending_reduction/layer_rate_eighth_two_sources.csv",
          "RAYON_NUM_THREADS=8 target/release/examples/accumulation_primitives --logs 15,17,19 --log-inverse-rate 3 --sources 3 --output results/accumulation_pending_reduction/layer_rate_eighth_three_sources.csv",
          "python3 scripts/report_accumulation_primitives.py", "```", ""]
(OUT / "README.md").write_text("\n".join(lines))
print(OUT / "README.md")
