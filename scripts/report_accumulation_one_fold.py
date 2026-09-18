#!/usr/bin/env python3
"""Report matched-layout, one-fold accumulation timings and verifier hashes."""
import csv
import hashlib
import itertools
import json
import math
import statistics
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'results/accumulation_one_fold'
BASE = 'merkle_same_code_batch'
CANOPY = 'canopy_one_fold_three_claims'


def occupied(m, q):
    return 1.0 if m == 1 else -m * math.expm1(q * math.log1p(-1 / m))


def native_auth(log_leaves, q):
    # Native matrix verification hashes repeated leaves before the Merkle
    # verifier deduplicates paths: exactly q leaf hashes, merged parents.
    return q + sum(occupied(2**j, q) for j in range(log_leaves))


# Exhaustive validation of this counting convention on a small tree.
counts = []
for sample in itertools.product(range(4), repeat=3):
    nodes = set(sample)
    count = len(sample)
    for _ in range(2):
        nodes = {i // 2 for i in nodes}
        count += len(nodes)
    counts.append(count)
assert abs(statistics.mean(counts) - native_auth(2, 3)) < 1e-12
q0 = math.ceil(136 / -math.log2(1.05 * math.sqrt(0.5)))
q1 = math.ceil(136 / -math.log2(1.05 * math.sqrt(0.125)))
assert (q0, q1) == (317, 96)

hash_rows = []
for n in range(18, 31, 2):
    base_auth = 2 * native_auth(n - 2, q0)
    bottom = occupied(2**(n - 2), q0) + occupied(2**(n - 3), q0)
    layer = 3 * native_auth(n - 3, q1)
    row = dict(log_n=n, baseline_auth_hashes=base_auth, baseline_fs=n + 4,
               baseline_total_hashes=base_auth + n + 4,
               canopy_bottom_hashes=bottom, canopy_layer_auth_hashes=layer,
               canopy_fs=n + 9, canopy_total_hashes=bottom + layer + n + 9)
    row['canopy_hash_reduction_percent'] = 100 * (1 - row['canopy_total_hashes'] / row['baseline_total_hashes'])
    hash_rows.append(row)
with (OUT / 'hash_estimates.csv').open('w') as f:
    writer = csv.DictWriter(f, fieldnames=list(hash_rows[0]))
    writer.writeheader()
    writer.writerows(hash_rows)

raw = list(csv.DictReader((OUT / 'timing_samples.csv').open()))
assert all(r['verified'] == 'true' for r in raw)
assert all(int(r['queries_0']) == q0 and int(r['queries_1']) == q1 for r in raw)
summary = []
for n in [18, 20, 22]:
    for scheme in [BASE, CANOPY]:
        group = [r for r in raw if int(r['log_n']) == n and r['scheme'] == scheme]
        assert len(group) == 5
        row = dict(log_n=n, scheme=scheme, repetitions=len(group))
        for key in [k for k in raw[0] if k.endswith('_seconds')]:
            values = [float(r[key]) for r in group]
            row['median_' + key] = statistics.median(values)
            row['mean_' + key] = statistics.mean(values)
        row['min_step_seconds'] = min(float(r['step_seconds']) for r in group)
        row['max_step_seconds'] = max(float(r['step_seconds']) for r in group)
        row['observed_auth_hashes'] = statistics.mean(int(r['actual_auth_hashes']) for r in group)
        row['expected_fs_finalizations'] = n + (4 if scheme == BASE else 9)
        summary.append(row)
with (OUT / 'timing_summary.csv').open('w') as f:
    writer = csv.DictWriter(f, fieldnames=list(summary[0]))
    writer.writeheader()
    writer.writerows(summary)

# Trace instrumentation is kept out of the timing binary.
trace_path = OUT / 'fs_validation.csv'
assert trace_path.exists(), 'Run the instrumented small validation first'
traces = list(csv.DictReader(trace_path.open()))
for row in traces:
    expected = int(row['log_n']) + (4 if row['scheme'] == BASE else 9)
    assert int(row['fs_finalizations']) == expected, (row, expected)
    assert row['verified'] == 'true'

lines = [
    '# One-fold Canopy accumulation versus plain Merkle accumulation', '',
    'The incoming Canopy claim is retained unchanged. The old Canopy-committed RS0 '
    'SEP is reduced by exactly one native WHIR fold. This leaves the folded-polynomial '
    'RS1 claim and the Canopy-opening RS1 claim. Those two claims and the previous '
    'RS1 accumulator are batched into one new RS1 accumulator. This is a three-source '
    'batch inside a two-component accumulator state.', '',
    'The plain baseline batches an incoming RS0 SEP and the previous RS0 accumulator '
    'into one new RS0 accumulator, retaining the same code, message length, and rate. '
    'Both schemes use full batched sumcheck to a common evaluation point, followed '
    'by same-code commitment and consistency queries. This resets the SEP functional '
    'to a bounded-size description each step. Neither scheme proves the outgoing '
    'accumulator to a terminal WHIR proof.', '',
    '## Parameters and alignment', '',
    '| Quantity | RS0 | RS1 |',
    '|---|---:|---:|',
    '| Message length (field elements) | N | N/8 |',
    '| Codeword length (field elements) | 2N | N |',
    '| Rate | 1/2 | 1/8 |',
    '| Field elements per native Merkle leaf | 8 | 8 |',
    '| Merkle leaves | N/4 | N/8 |',
    '| Query draws per source | 317 | 96 |', '',
    'RS1 is taken directly from `native.round_configs[0].irs_committer`, the second '
    'oracle in the repository’s WHIR implementation. The example asserts equality '
    'of Canopy and native RS1 message length, codeword length, rate, and row layout. '
    'The domain is the same registered Cantor-subspace domain. Canopy uses h=4 in '
    'field-element units: its eight-element row hash is followed by one binary '
    'parent layer, giving N/8 subtree digests. Those digests map invertibly into '
    'GF(2^256), with no hash-to-field invocation.', '',
    'All measurements use GF(2^256), BLAKE3-256, eight Rayon workers, a 136-bit local '
    'Johnson query budget (the repository’s convention for a 128-bit composed '
    'target), and no proof of work. The first fold has three sumcheck rounds. '
    'The headline experiment uses WHIR’s interleaved code/row metric, matching the '
    'existing experiments; the earlier scalar-leaf primitive table is a different '
    'layout and workload.', '',
    '## Prover times', '',
    'Measured on the local Apple M2 desktop, with one warmup and five measurements '
    'per scheme and size. Scheme order alternates each repetition. Entries are '
    'medians in milliseconds. The incoming-commitment-inclusive column is the '
    'median of paired per-run sums, not the sum of independently taken medians.', '',
    '| N | Scheme | Accumulation step | Step range | Incoming commitment | Step + incoming commitment |',
    '|---|---|---:|---:|---:|---:|',
]
for row in summary:
    name = 'Plain Merkle batch' if row['scheme'] == BASE else 'Canopy, one fold'
    lines.append(f"| 2^{row['log_n']} | {name} | {1000*row['median_step_seconds']:.2f} | "
                 f"{1000*row['min_step_seconds']:.2f}–{1000*row['max_step_seconds']:.2f} | "
                 f"{1000*row['median_incoming_commit_seconds']:.2f} | "
                 f"{1000*row['median_with_incoming_commit_seconds']:.2f} |")
lines += ['',
    'The step starts with cached input commitments, codewords, and witnesses. It '
    'includes input-functional materialization, the fold and its new RS encoding '
    '(Canopy only), Canopy bottom openings and resulting SEP construction, full '
    'batched sumcheck, linear combinations, the outgoing Merkle commitment and '
    'OOD evaluation, source openings, and outgoing-claim construction. Both '
    'same-code batches combine cached codewords directly, avoiding a redundant '
    'RS encoding. The folded-polynomial commitment still requires its encoding.', '',
    'Input generation, public setup, computation of already-public input sigmas, '
    'verification, and final decision of the accumulator are excluded. The '
    'incoming-commitment column includes its RS encoding and commitment; Canopy '
    'also includes its layer OOD evaluation. The fresh input SEP functional is '
    'one multilinear evaluation. The existing accumulator uses the same compact '
    'functional shape produced by a prior step: an MLE term, one OOD term, and '
    'randomly batched column-wise query terms. Folded and Canopy-opening claims '
    'are the actual claims produced during the measured execution.', '',
    'The N=2^22 runs had substantial desktop timing variation; overlapping ranges '
    'limit any speedup conclusion. Commitment-inclusive costs are distinct from '
    'accumulation-step costs. No large-N prover-time extrapolation is made.', '',
    '## Expected verifier hashes per accumulation step', '',
    'Counts include authentication hashes and Fiat–Shamir BLAKE3-XOF finalizations. '
    'They are complete hash invocations, not BLAKE3 compression calls or verifier '
    'wall time. The output accumulator is retained, so its tree is not opened. '
    'The incoming Canopy commitment is simply carried forward, so its paths are '
    'not checked in this step. Final-decider work is excluded.', '',
    '| N | Plain Merkle batch | Canopy, one fold | Hash reduction |',
    '|---|---:|---:|---:|',
]
for r in hash_rows:
    lines.append(f"| 2^{r['log_n']} | {r['baseline_total_hashes']:,.0f} | {r['canopy_total_hashes']:,.0f} | {r['canopy_hash_reduction_percent']:.1f}% |")
lines += ['',
    'Let D(M,q)=M[1−(1−1/M)^q] be expected occupied nodes for sampling with '
    'replacement. Native WHIR hashes all q queried rows, including duplicates, '
    'then merges parent paths. Thus H(M,q)=q+sum_{j=1}^{log2 M} D(M/2^j,q). '
    'The formulas are:', '',
    '- Plain: 2 H(N/4,317) + log2(N)+4.',
    '- Canopy: D(N/4,317)+D(N/8,317) + 3 H(N/8,96) + log2(N)+9.', '',
    'The first two Canopy terms are its deduplicated bottom row and parent hashes. '
    'The three H terms are the separately committed folded, Canopy, and old '
    'accumulator sources. Shared query indices do not merge paths across distinct roots.', '',
    'The exact expectation formula was checked against exhaustive small-tree '
    'enumeration. Every execution checks native authentication counts against '
    'the sampled path geometry. Fiat–Shamir counts were independently recorded '
    'in a separate instrumented verifier run. Deterministic timing inputs have '
    'one fixed query sample per size; hash tables above use the exact expectation '
    'rather than that single sample.', '',
    '## Validation and scope', '',
    'Every warmup and measured run replayed the fold and batch verification and '
    'checked the derived output SEP against the output witness. Small validation '
    'runs additionally re-encoded the output message, checked its Merkle root '
    'and codeword, and rejected modified sumcheck/OOD messages, modified Canopy '
    'bottom values, and modified Merkle authentication data. Thus the cached '
    'codeword combination was checked against fresh RS encoding.', '',
    'This is an executable one-step cost prototype, not a recursive SNARK or a '
    'production accumulator interface. The verifier materializes covectors for '
    'the correctness checks; a succinct implementation would evaluate their '
    'compact descriptions directly. That difference does not affect the reported '
    'hash counts. The timing instance repeats the same valid Canopy input in the '
    'old and incoming positions: it is bound twice and the incoming one is '
    'carried unchanged. A production wrapper must check equality of the complete '
    'incoming and outgoing pending instance, and the final decider must check '
    'both state components. A formal extraction proof for this composition is '
    'separate from these measurements.', '',
    'The initial exploratory re-encoding run is retained in '
    '`reencoded_output_samples.csv` and `initial_reencoded_output.rs`. It is not '
    'used in the headline tables. Earlier scalar-layout primitives remain in '
    '`../accumulation_pending_reduction/` as preliminary work.', '',
    '## Reproduction', '', '```sh',
    'cargo build --offline --release --example accumulation_one_fold',
    'RAYON_NUM_THREADS=8 target/release/examples/accumulation_one_fold --logs 10 --warmups 0 --repetitions 1 --check-tampering --output results/accumulation_one_fold/optimized_smoke.csv',
    'python3 scripts/measure_gf256_run.py --record-dir results/accumulation_one_fold/optimized_run --threads 8 -- target/release/examples/accumulation_one_fold --logs 18,20,22 --warmups 1 --repetitions 5 --output results/accumulation_one_fold/timing_samples.csv',
    'cargo build --offline --release --features fs-accounting --example accumulation_one_fold',
    'RAYON_NUM_THREADS=8 target/release/examples/accumulation_one_fold --logs 10,18 --warmups 0 --repetitions 1 --check-tampering --output results/accumulation_one_fold/fs_validation.csv',
    'python3 scripts/report_accumulation_one_fold.py', '```', '',
    'Raw timing samples, stage means/medians, the exact command, executable hash, '
    'process RSS, instrumented validation, and expected hash counts accompany '
    'this report. The timed process used about 2.37 GiB peak RSS, including setup '
    'and verification. No AWS jobs or manuscript files were changed.', '',
]
(OUT / 'README.md').write_text('\n'.join(lines))
files = ['examples/accumulation_one_fold.rs', 'scripts/report_accumulation_one_fold.py',
         'src/additive_rs.rs', 'src/binary_field.rs', 'src/canopy.rs',
         'src/executable_whir.rs', 'vendor/whir/src/protocols/whir/config.rs',
         'vendor/whir/src/protocols/irs_commit.rs', 'Cargo.lock']
(OUT / 'source_hashes.json').write_text(json.dumps({p: hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in files}, indent=2)+'\n')
print(OUT / 'README.md')
