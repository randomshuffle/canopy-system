# canopy-system

Rust/arkworks implementation and concrete proof-size/prover-time comparison for
Merkle, Canopy, and executable WHIR commitments.

## Final paper experiments

The final configuration uses 128-bit soundness, BLAKE3-256, no proof of work,
and only the proved Johnson-bound RS proximity regime. All protocol field
elements use the FFT-friendly prime
`2^256 - 43 * 2^32 + 1`, whose two-adicity is 32. The executable WHIR adapter
is pinned to official WHIR commit
`c274cd5898d31af2cd9be1707a44ccad92c22eaa`.

The generated paper results are:

- [`results/paper/summary.md`](results/paper/summary.md): proof sizes for all
  three experiments and the largest-point oracle-query table;
- [`results/paper/oracle_query_breakdown.csv`](results/paper/oracle_query_breakdown.csv):
  every queried codeword in experiments 2 and 3;
- [`results/paper/prover_timings.md`](results/paper/prover_timings.md): executable
  `2^20` prover timings and stage breakdowns; and
- [`results/paper/experiment1_canopy_vs_merkle/`](results/paper/experiment1_canopy_vs_merkle/):
  complete Experiment 1 proof-size components and height search.

The matching capacity-bound results are in
[`results/paper_capacity/`](results/paper_capacity/). They use the same field,
128-bit target, grids, and zero-PoW policy, but rely on STIR Conjecture 5.6 and
must be labeled conjectural.
The batch-128-only diagnostic that permits a terminal polynomial of up to
`2^8` coefficients is in
[`results/paper_capacity_terminal8_batch128/`](results/paper_capacity_terminal8_batch128/).
The terminal-`2^8` Johnson- and capacity-bound Experiments 2 and 3 versus
independently optimized Merkle WHIR are in
[`results/paper_johnson_terminal8_fold3/`](results/paper_johnson_terminal8_fold3/)
and
[`results/paper_capacity_terminal8_fold3/`](results/paper_capacity_terminal8_fold3/).
These reports use no Canopy shortening, search every feasible Canopy height
independently in Experiment 2, set `h=f0+1=4` in Experiment 3, and use
three-variable folds throughout.

Regenerate them with:

```sh
cargo run --release -- paper-estimate --samples 100 --output-dir results/paper
cargo run --release -- paper-time --log-length 20 --warmups 3 --repetitions 20 --output-dir results/paper
cargo run --release -- paper-capacity-estimate --samples 100 --output-dir results/paper_capacity
cargo run --release -- paper-capacity-time --log-length 20 --warmups 3 --repetitions 20 --output-dir results/paper_capacity
cargo run --release -- paper-capacity-terminal8-estimate --samples 100 --output-dir results/paper_capacity_terminal8_batch128
cargo run --release -- paper-terminal8-hybrid-estimate --samples 100 --output-dir results/paper_johnson_terminal8_fold3
cargo run --release -- paper-capacity-terminal8-hybrid-estimate --samples 100 --output-dir results/paper_capacity_terminal8_fold3
```

The standard WHIR timing is one verifier-checked executable proof. The hybrid
timings are transparent sums of the measured stages in
`prover_timing_stages.csv`; the aligned tail proofs themselves are generated
and verifier-checked by the executable WHIR adapter, but a monolithic hybrid
transcript has not been implemented.
The current executable reference also hard-codes termination below `2^4`, so
the terminal-`2^8` Experiment 3 outputs are proof-size estimates rather than
executable prover-time measurements.

## Legacy exploratory results (superseded)

The material below records earlier parameter searches and is retained for
reproducibility. Its 128-bit fields, 100-bit target, and capacity-bound model
are not the final paper configuration above.

The original implementation compared:

- a BLAKE3-256 Merkle vector commitment with minimal batch multiproofs; and
- the Canopy vector commitment from the attached draft, using Reed-Solomon
  encoding and a concrete estimate for the WHIR argument that solves Canopy's
  output-committed SEP relation.

The current 128-bit-field benchmark outputs are checked into
[`results/`](results/). The full grid is in
[`results/summary.md`](results/summary.md), the original/half/quarter comparison
is in
[`results/codeword_length_n30_b128_f128/`](results/codeword_length_n30_b128_f128/),
and the matching `2^22` commitment profile is in
[`results/codeword_commitment_timings_f128.md`](results/codeword_commitment_timings_f128.md).
Earlier small-field experiments remain in their named subdirectories for
historical comparison; they do not use the current default soundness model.
The separate-commitment batching grid for `t=8,16,32` and its fixed timing
profile are in
[`results/separate_commitments/`](results/separate_commitments/). The matching
Canopy-quarter grid and profile are in
[`results/separate_commitments_quarter/`](results/separate_commitments_quarter/).
The `N=2^28`, 24-bit, factor-16 virtual-initial-oracle variant is in
[`results/separate_commitments_quarter_rs16_virtual_f24_n28/`](results/separate_commitments_quarter_rs16_virtual_f24_n28/).

## Main result

The input-vector and Canopy/WHIR fields are fixed at 128 bits. For `N=2^30`,
batch 128, searching all `h=1,...,15` selects `h=9`. Its mean proof is
70.34 KiB, versus 90.50 KiB for Merkle (0.777x): 58.23 KiB of hashes and
12.11 KiB of field elements. No proof of work or repeated sumcheck transcript
is used.

The configured WHIR target is 100 bits, matching the high-level
100-bit/roughly-128-bit-field choice in the WHIR reference implementation.
This is not a proved 100-bit no-PoW result under the capacity model: for the
`N=2^30`, batch-128 optimum, the smallest current RBR/list-adjusted
single-challenge estimate is 77.42 bits. The scalar terminal sumcheck itself
has 126.58 bits. See `whir_parameters.csv` for every component.

The best `h` values are:

| log2(vector length) | batch 64 | batch 128 | batch 256 | batch 512 | batch 1024 |
|---:|---:|---:|---:|---:|---:|
| 22 | 11 | 7 | 4 | 3 | 2 |
| 24 | 13 | 9 | 5 | 3 | 2 |
| 26 | 15 | 7 | 5 | 3 | 2 |
| 28 | 15 | 9 | 5 | 3 | 2 |
| 30 | 15 | 9 | 6 | 3 | 2 |

See the generated summary for KiB values and the hash/field split.

## Shortened-codeword experiment

With the input and Canopy-layer fields both fixed at 128 bits, shortening the
Canopy RS codeword changes its log length from `n` to `n-1` or `n-2`. The
message dimension remains `2^(n-h)`, so the inverse-rate exponent supplied to
WHIR changes from `h` to `h-1` or `h-2`.

For `N=2^30`, batch 128, and 100 random samples:

| codeword length | best h | proof KiB | hashes KiB | fields KiB | vs Merkle |
|---|---:|---:|---:|---:|---:|
| `N` | 9 | 70.34 | 58.23 | 12.11 | 0.777x |
| `N/2` | 10 | 71.70 | 57.67 | 14.03 | 0.792x |
| `N/4` | 11 | 72.75 | 60.76 | 11.98 | 0.804x |
| Merkle | - | 90.50 | 88.50 | 2.00 | 1.000x |

Shortening is still worse for opening size. The smaller oracle saves one or two
levels per Merkle path, but reducing the inverse-rate exponent increases the
number of WHIR queries and shifts the optimal height upward.

For `N=2^22`, commitment time uses the batch-128 proof-size-optimal heights
`h=7,7,8` for the three variants. Medians of 20 repetitions are:

| scheme | codeword | h | median commitment s | relative to Merkle |
|---|---|---:|---:|---:|
| Merkle | - | - | 0.2113 | 1.000x |
| Canopy original | `N` | 7 | 0.3838 | 1.816x |
| Canopy half | `N/2` | 7 | 0.2914 | 1.379x |
| Canopy quarter | `N/4` | 8 | 0.2491 | 1.179x |

Input-vector generation and deallocation are excluded from these timings.
The instrumented mean stage breakdown is:

| variant | bottom forest | hash-to-field | RS encoding | top tree |
|---|---:|---:|---:|---:|
| Original | 0.2080 s (53.9%) | 0.0049 s (1.3%) | 0.1267 s (32.8%) | 0.0465 s (12.0%) |
| Half | 0.2073 s (69.8%) | 0.0048 s (1.6%) | 0.0608 s (20.5%) | 0.0239 s (8.1%) |
| Quarter | 0.2078 s (82.4%) | 0.0025 s (1.0%) | 0.0297 s (11.8%) | 0.0123 s (4.9%) |

At `h=8`, quarter Canopy spends most remaining time on the bottom forest.
That forest performs 8,372,224 hashes, compared with 8,388,607 hashes for the
complete Merkle tree. Consequently, further shortening the RS codeword cannot
produce much commitment-time improvement without also changing the bottom
commitment construction or the selected `h`.

## Historical small-field experiments

The separate-commitment and 24/32/48/64-bit experiments below are retained as
historical outputs. They use the previous 128-bit target and small-field
repetition model, so their sizes and timing choices are not directly
comparable to the current 128-bit-field/100-bit-target default above.

### Separate-commitment batching experiment

This experiment initializes `t` vectors of length `N` and commits to every
vector separately. A batch opens the same `B` logical indices in all trees and
uses the same SEP-reduction challenge, so the resulting functionals align, but
every commitment sends its own Merkle authentication path. The Merkle baseline
is therefore exactly `t` independent batch proofs.

Canopy also sends `t` separate bottom paths. After the verifier's challenge,
the prover constructs several independently randomized linear combinations of
the `t` committed RS codewords and interleaves them in one new Merkle
commitment. The verifier opens the same consistency leaves in all old trees
and the new tree. New-oracle values are derived from the old openings rather
than serialized. The combined tree's authentication frontier is merged with
its first WHIR opening.

The number of independent combinations is
`ceil(128 / log2(|F|))`. The 32-bit layer field therefore uses five, providing
157.92 bits against cancellation; four would provide only 126.34 bits. Each
combination gets a parallel WHIR/SEP instance, while their interleaved oracle
roots and Merkle paths are shared. WHIR retains the capacity-bound estimator.
For the full-length codeword, the consistency check uses the RS minimum
distance `1-2^-h`, so it makes `ceil(128/h)` shared width-16 leaf queries.

The full default `N`, `B`, and `h` grid was run with 100 samples for each of
`t=8,16,32`. For `N=2^30` and `B=128` per commitment:

| t | total opened | best h | consistency q | Merkle KiB | Canopy KiB | Canopy/Merkle |
|---:|---:|---:|---:|---:|---:|---:|
| 8 | 1024 | 6 | 22 | 711.53 | 405.14 | 0.569x |
| 16 | 2048 | 5 | 26 | 1423.06 | 717.53 | 0.504x |
| 32 | 4096 | 5 | 26 | 2846.12 | 1336.89 | 0.470x |

The relative Canopy advantage improves with `t`: Merkle and Canopy's separate
paths scale with `t`, while the five parallel WHIR instances and their shared
paths do not. The complete result and per-height breakdown are in
[`results/separate_commitments/summary.md`](results/separate_commitments/summary.md)
and
[`results/separate_commitments/proof_sizes.csv`](results/separate_commitments/proof_sizes.csv).

For `N=2^18`, `t=16`, and `B=128` per commitment, the structural search
selects `h=3` and 43 consistency queries. The mean proof is 466.44 KiB for
Canopy versus 654.94 KiB for Merkle (0.712x). Means of 20 release-mode timing
repetitions are:

| scheme | h | combinations | mean s | median s | relative to Merkle mean |
|---|---:|---:|---:|---:|---:|
| Merkle separate | - | 0 | 0.2333 | 0.2234 | 1.000x |
| Canopy batched WHIR | 3 | 5 | 0.3415 | 0.3376 | 1.464x |

The Canopy time includes all 16 initial commitments, construction of five
linear combinations, and the new combined-oracle Merkle commitment, but not
WHIR execution. Its largest mean stages are the bottom forests at 0.2005 s
(58.7%) and hash-to-field at 0.0327 s (9.6%). See the
[`timing report`](results/separate_commitments/commitment_timings_n18_t16_b128.md)
and
[`stage profile`](results/separate_commitments/commitment_timings_n18_t16_b128_profile.csv).

### Canopy-quarter variant

The same experiment was also run with each Canopy RS codeword shortened to
`N/4`. The message dimension is unchanged, so the inverse-rate exponent is
`h-2`, the minimum relative distance is `1-2^{-(h-2)}`, and the consistency
check uses `ceil(128/(h-2))` shared leaf queries.

For `N=2^30` and `B=128` per commitment:

| t | total opened | best h | consistency q | Merkle KiB | Canopy-quarter KiB | Canopy-quarter/Merkle |
|---:|---:|---:|---:|---:|---:|---:|
| 8 | 1024 | 7 | 26 | 711.53 | 446.70 | 0.628x |
| 16 | 2048 | 7 | 26 | 1423.06 | 807.68 | 0.568x |
| 32 | 4096 | 7 | 26 | 2846.12 | 1529.64 | 0.537x |

At the fixed timing point `N=2^18`, `t=16`, and `B=128`, quarter Canopy
selects `h=5` and 43 consistency queries. Its mean proof is 539.94 KiB versus
654.94 KiB for Merkle (0.824x). Over 20 release-mode repetitions, its mean
commitment pipeline is 0.2635 s versus 0.2308 s for Merkle (1.141x); the median
times are 0.2610 s and 0.2246 s. Relative to the full-length Canopy profile,
quartering reduces mean prover commitment time by about 23%, but increases the
mean proof by about 16%.

The complete quarter result and per-height breakdown are in the generated
[`summary`](results/separate_commitments_quarter/summary.md) and
[`proof-size CSV`](results/separate_commitments_quarter/proof_sizes.csv). The
fixed-point measurements are in the
[`timing report`](results/separate_commitments_quarter/commitment_timings_n18_t16_b128.md)
and
[`stage profile`](results/separate_commitments_quarter/commitment_timings_n18_t16_b128_profile.csv).

### 24-bit, factor-16 virtual initial oracle

For `N=2^28`, quarter Canopy has a total RS codeword length of `2^26`. This
experiment models the Canopy/WHIR layer as `GF(2^24)` with a foldable additive
domain, and splits that codeword into 16 interleaved RS blocks of length
`2^22`. Each old commitment puts the 16 stripe values at one RS coordinate
in a Merkle leaf. Six independent challenges provide 144 bits against random-linear-
combination cancellation and batch all `16t` stripe claims into six parallel
WHIR instances with shared paths.

The old roots jointly bind WHIR's initial oracle. At an initial WHIR query the
prover opens one width-16 interleaving column in every old commitment, and the
verifier derives all six random linear combinations locally. The experiment
therefore has no separate combined-oracle root and no standalone consistency
test. Interleaving supplies the effective first width-16 fold, the additional
initial WHIR folding factor is one, and recursive WHIR folds use width 16. The
first old-oracle openings use WHIR's capacity-bound query count rather than an
exact-codeword minimum-distance assumption.

The proof-size optima over `h=3,...,15`, using 100 deterministic samples, are:

| B each | best h (`t=8/16/32`) | t=8 KiB (vs Merkle) | t=16 KiB (vs Merkle) | t=32 KiB (vs Merkle) |
|---:|---:|---:|---:|---:|
| 64 | 9 / 9 / 9 | 255.30 (0.751x) | 493.20 (0.725x) | 968.99 (0.713x) |
| 128 | 7 / 7 / 7 | 384.59 (0.594x) | 741.42 (0.572x) | 1455.08 (0.561x) |
| 256 | 6 / 6 / 5 | 587.55 (0.477x) | 1144.56 (0.465x) | 2257.92 (0.458x) |
| 512 | 5 / 5 / 5 | 919.24 (0.394x) | 1802.77 (0.386x) | 3569.83 (0.382x) |
| 1024 | 4 / 4 / 4 | 1465.27 (0.332x) | 2887.93 (0.327x) | 5733.26 (0.325x) |

Relative to the explicit combined-oracle protocol, virtual access is smaller
through most of the `t=8` grid and improves `t=16, B=128` from 747.82 KiB to
741.42 KiB. It becomes larger at high `t` and `B`, because every initial
capacity-bound WHIR query now authenticates all old trees rather than one
combined tree; this cost scales with `t`.

The three `h` entries are ordered by `t=8,16,32`. The complete per-height
accounting separates bottom authentication, virtual-initial authentication,
recursive WHIR hashes, virtual-initial stripe values, and WHIR field messages:
[`summary`](results/separate_commitments_quarter_rs16_virtual_f24_n28/summary.md),
[`proof-size CSV`](results/separate_commitments_quarter_rs16_virtual_f24_n28/proof_sizes.csv),
and
[`protocol parameters`](results/separate_commitments_quarter_rs16_virtual_f24_n28/protocol_parameters.csv).
The previous explicit combined-oracle/consistency result remains available for
[`comparison`](results/separate_commitments_quarter_rs16_f24_n28/summary.md),
and the apples-to-apples 32-bit virtual-initial grid is in
[`results/separate_commitments_quarter_rs16_virtual_f32_n28/`](results/separate_commitments_quarter_rs16_virtual_f32_n28/).

## Protocol choices

### Merkle

- BLAKE3 with 256-bit digests.
- The fixed 128-bit arkworks input field
  `p = 255211775190703847597530972673164705793`, with 16-byte elements and
  two-adicity 30.
- Domain separation between leaf and internal-node hashes.
- A true minimal frontier multiproof. If both children are opened, neither is
  sent as an authentication hash; all shared nodes, including the top layers,
  occur at most once.
- Opened values are included in the headline proof size. Verifier-generated
  indices are not serialized.

For a batch of `B` values and a multiproof frontier containing `A` hashes, the
reported size is `16 * B + 32 * A` bytes.

### Canopy

For a vector of length `2^n` and canopy depth `h`:

1. Construct only the bottom `h` Merkle layers.
2. Apply a 512-bit BLAKE3 XOF hash-to-field map to the `2^(n-h)` subtree roots.
3. Treat those field elements as polynomial coefficients and evaluate the
   resulting Reed-Solomon codeword on a radix-2 domain of size `2^(n-s)`, where
   `s=0` normally and `s=1,2` in the shortened-codeword experiments. The
   24-bit experiment instead distributes the coefficients across 16
   interleaved additive-domain RS blocks.
4. Commit to that codeword with a WHIR-compatible Merkle tree whose leaf holds
   16 field elements.
5. For an opening, authenticate the requested values only through the bottom
   `h` layers. The verifier deduplicates ancestor claims, samples `gamma`, and
   reduces them to one output-committed SEP instance.
6. Estimate the WHIR prover messages needed to establish that SEP instance.

The Rust implementation exposes the interaction as `open_bottom`,
`verify_bottom_and_reduce`, and `estimate_whir`. WHIR itself is intentionally
not implemented. The executable commitment/timing path and the default
structural estimator both use the fixed 128-bit field.

### WHIR

The estimator follows the capacity-bound (`ConjectureList`) parameter logic in
the November 2024 WHIR paper and its authors' arkworks reference code:

- nominal security target: 100 bits;
- input-vector and default Canopy/WHIR layer field: the fixed 128-bit prime
  `p = 255211775190703847597530972673164705793`, 16 bytes, two-adicity 30;
- legacy 24/32/48/64-bit layer profiles remain available for diagnostic
  comparisons, but the old repetition workaround is no longer applied;
- starting rate: `rho = 2^{-(h-r)}` for codeword log reduction `r`;
- capacity radius: `delta = 1 - rho - eta`, with `eta = rho/2`;
- mutual correlated agreement at capacity is assumed, with conjecture
  constants `c1 = c2 = c3 = 1`;
- folding log: 4 variables per round, so the folding factor is `2^4 = 16`;
- recursive WHIR terminates as soon as the message length is at most `2^8`;
- every recursive fold uses exactly four variables, so the actual terminal
  message has between `2^3` and `2^8` coefficients for these configurations;
- BLAKE3-256 Merkle commitments;
- every query opens one 16-field-element Merkle column;
- random WHIR queries are deduplicated and their Merkle paths use the same
  minimal frontier encoding as the baseline;
- no proof of work is used.

The 256-bit hash caps collision security at 128 bits, so it does not reduce the
100-bit nominal target. At inverse-rate exponent `r`, the capacity
assumption uses `rho = 2^-r`, `eta = rho/2`, and
`delta = 1 - rho - eta`. The number of queries is therefore
`ceil(100 / -log2(1-delta))`.

Soundness calculations use each field's actual cardinality `b = log2(|F|)`,
not its rounded storage width. For `s` OOD samples, current message-variable count
`m`, inverse-rate exponent `r`, and `L = m + 2*r + 1`, the estimator chooses the
smallest unrestricted `s` such that
`s*b + 1 - (2*L + m*s) >= 100`.

Every fold and the terminal check use one sumcheck transcript over the 128-bit
field. This removes the invalid small-field shortcut of repeating only scalar
sumcheck messages while reusing a recursive oracle that would actually depend
on a different folding challenge. Challenges are verifier randomness and are
not serialized.

For the specialized `N=2^30`, batch-128 optimum `h=9`, the OOD counts are
`2;2;2;2`, the per-oracle query counts are `12;9;7;6`, and there are four
four-variable folds plus a five-variable terminal sumcheck, for 21 verifier
challenges total.

This does not establish 100-bit end-to-end soundness without PoW. The
per-challenge folding estimates are `86.58;84.58;82.58;80.58` bits and the
recursive constraint-combination estimates are `80.78;79.13;77.42` bits. The
minimum is 77.42 bits. A field of roughly 151 bits would be needed to lift this
particular conservative analytic minimum to 100 bits while keeping one
challenge and no PoW; a 160-bit field is the natural byte-aligned choice.

The estimate counts OOD answers, each degree-2 sumcheck polynomial,
intermediate Merkle roots, queried oracle values, authentication hashes, and
final coefficients. The existing Canopy commitment root is not counted again.
Verifier randomness and verifier-generated query indices are not counted.

The capacity-bound mutual-correlated-agreement hypothesis is an explicit
benchmarking assumption requested for this comparison. These numbers must not
be interpreted as an unconditional security claim or a production-ready
parameter recommendation. In particular, `security_bits=100` in the CSV is a
parameter target, while `minimum_single_challenge_security_bits` records the
smaller RBR/list-adjusted analytic value.

## Running the benchmark

Build and test:

```sh
cargo test --all-targets
```

Regenerate the requested matrix:

```sh
cargo run --release -- estimate \
  --log-lengths 22,24,26,28,30 \
  --batch-sizes 64,128,256,512,1024 \
  --layer-field-bits 128 \
  --samples 100 \
  --output-dir results
```

Regenerate the shortened-codeword proof comparison:

```sh
cargo run --release -- estimate \
  --log-lengths 30 \
  --batch-sizes 128 \
  --layer-field-bits 128 \
  --codeword-log-reductions 0,1,2 \
  --samples 100 \
  --output-dir results/codeword_length_n30_b128_f128
```

Regenerate its commitment timing comparison:

```sh
cargo run --release -- commit-time \
  --log-length 22 \
  --codeword-log-reductions 0,1,2 \
  --heights 7,7,8 \
  --repetitions 20 \
  --output results/codeword_commitment_timings_f128.csv
```

Regenerate the separate-commitment batching grid:

```sh
cargo run --release -- estimate-separate \
  --commitment-counts 8,16,32 \
  --samples 100 \
  --output-dir results/separate_commitments
```

Regenerate the fixed height search and commitment profile:

```sh
cargo run --release -- estimate-separate \
  --log-lengths 18 \
  --batch-sizes 128 \
  --commitment-counts 16 \
  --output-dir results/separate_commitments/timing_height_search_n18_t16_b128

cargo run --release -- separate-commit-time \
  --log-length 18 \
  --commitment-count 16 \
  --batch-size 128 \
  --repetitions 20 \
  --output results/separate_commitments/commitment_timings_n18_t16_b128.csv
```

Regenerate the matching Canopy-quarter grid and fixed profile:

```sh
cargo run --release -- estimate-separate \
  --commitment-counts 8,16,32 \
  --codeword-log-reductions 2 \
  --samples 100 \
  --output-dir results/separate_commitments_quarter

cargo run --release -- estimate-separate \
  --log-lengths 18 \
  --batch-sizes 128 \
  --commitment-counts 16 \
  --codeword-log-reductions 2 \
  --output-dir results/separate_commitments_quarter/timing_height_search_n18_t16_b128

cargo run --release -- separate-commit-time \
  --log-length 18 \
  --commitment-count 16 \
  --batch-size 128 \
  --codeword-log-reduction 2 \
  --repetitions 20 \
  --output results/separate_commitments_quarter/commitment_timings_n18_t16_b128.csv
```

Regenerate the `N=2^28`, 24-bit, factor-16 interleaved-RS grid:

```sh
cargo run --release -- estimate-separate \
  --log-lengths 28 \
  --batch-sizes 64,128,256,512,1024 \
  --commitment-counts 8,16,32 \
  --layer-field-bits 24 \
  --codeword-log-reductions 2 \
  --rs-interleaving-factors 16 \
  --virtual-initial-oracle \
  --samples 100 \
  --output-dir results/separate_commitments_quarter_rs16_virtual_f24_n28
```

`separate-commit-time` automatically searches the default height grid
unless `--height` is supplied. It writes both the headline timing CSV and a
`*_profile.csv` stage breakdown.

The defaults are:

- vector lengths: `2^22, 2^24, 2^26, 2^28, 2^30`;
- batch sizes: `64, 128, 256, 512, 1024`;
- Canopy heights: every `h` from 1 through 15;
- Canopy-layer field width: 128 bits (override with `--layer-field-bits`);
- codeword log reduction: 0 (override with `--codeword-log-reductions`);
- 100 random samples from seed `0x43414e4f5059`.

Large-vector proof sizes are computed structurally in `O(B log N)` space and
do not allocate the vector or Merkle tree.

Run the real `2^22` commitments and front-end openings:

```sh
cargo run --release -- time \
  --log-length 22 \
  --batch-sizes 64,128,256,512,1024 \
  --repetitions 30 \
  --output results/timings.csv
```

The Merkle timing is end-to-end. Canopy commitment timing is also complete and
includes the bottom forest, RS encoding, and codeword Merkle commitment.
Canopy prover/verifier timing covers only the bottom opening and SEP reduction;
it excludes WHIR execution because WHIR is modeled only for proof size.

## Code map

- [`src/merkle.rs`](src/merkle.rs): commit/open/verify and minimal multiproofs.
- [`src/reed_solomon.rs`](src/reed_solomon.rs): arkworks radix-2 RS encoder.
- [`src/canopy.rs`](src/canopy.rs): Canopy commit and interactive SEP reduction.
- [`src/whir.rs`](src/whir.rs): capacity-bound WHIR parameters and size model.
- [`src/benchmark.rs`](src/benchmark.rs): random-sample matrix and real timings.
- [`src/separate_benchmark.rs`](src/separate_benchmark.rs): separate-commitment
  batching grid, virtual-initial and legacy consistency models, and commitment
  profile.
- [`src/main.rs`](src/main.rs): command-line interface.

## References

- Canopy draft supplied with this task, Section 3.
- [WHIR: Reed-Solomon Proximity Testing with Super-Fast Verification](https://eprint.iacr.org/2024/1586).
- [WHIR authors' arkworks implementation](https://github.com/worldfnd/whir),
  cross-checked against commit `0c0808d` from November 2024.
