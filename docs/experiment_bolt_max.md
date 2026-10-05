# Final Bolt-max with GF(2^256), Canopy RS, and WHIR

The final suite uses GF(2^256), BLAKE3-256, degree-8/compression-8 all-one
Bolt-max, and rate-1/2 additive RS for the syndrome. Public sparse matrix H
follows the pinned upstream ensemble at revision
`3832e47b24e7b3e10525c9c5bcfc1cfe66d525f2`. Ensemble distance gamma=0.013
is an explicit assumption. The public-H adapter evaluates H directly;
Spark/Mulperm preprocessing and proofs are excluded.

The [final artifact index](../results/README.md) links the 75 current proof/hash
estimates, main measured campaign, N=2^30 extension, and repeated Bolt checks.
Timing includes commitment and opening; input generation, public setup, and
verification are outside reported prover time. Every measured Bolt argument
is serialized and verified.

## Geometry

For each of eta inputs, containing N field elements, use w=4 and k=N/4:

- X has k rows, each of width w.
- U=HX has k/8 rows.
- Y=RS(U) has k/4 rows.
- Both Canopy RS codewords have length k/4=N/16.
- Their messages have k/2^h_X and (k/4)/2^h_Y coefficients.
  Heights count Merkle levels above complete rows and are optimized independently.

The Merkle control uses a fixed shared WHIR tail with 2 eta source claims
and join message length k/8. The executable fixes it with
`--paper-only --shared-merkle-baseline`; the estimate fixes the Canopy blocks
with `--width 4 --block-divisor 4`.

Canopy uses separate recursive batches for the systematic and parity sides.
Each combines eta code-switch claims with its corresponding nonconstant
Canopy SEP claims immediately at the aligned message size. Full-height constant
layers are opened directly. There are no independent Canopy folds before joining
and no delayed systematic join. Each side's cross evaluations and OOD values
enter native multi-root WHIR with distinct transcript challenges.

Each source's public constraints are combined before batching. This adapter
supplies the full matrix of distinct-functional cross evaluations, including
OOD cross evaluations: p(p-1) functional cross fields for p sources. The
compressed 2p-3 coefficient encoding of the WHIR Canopy experiment is a different
construction; its savings are not applied to this Bolt adapter.

## Arithmetic and soundness

The canonical field and additive RS implementations are [binary_field.rs](../src/binary_field.rs) and [additive_rs.rs](../src/additive_rs.rs). The tower is K=GF(2)[x]/(x^128+x^7+x^2+x+1), then K[w]/(w^2+w+x^121). Elements occupy 32 bytes. Integer conversion has characteristic-two semantics; benchmark inputs and public evaluation points use full-width hash-derived values.

The systematic query radius is max(gamma/3, 1−(1−gamma+epsilon)^(1/3)−3eta), with epsilon=eta=2^-32. This applies [Khatam, Theorem 1](https://eprint.iacr.org/2024/1843.pdf) as an explicit proximity assumption for the Bolt adapter. The “one-and-a-half Johnson” radius is the cube-root expression, not 1.5 times the ordinary Johnson radius. Its per-challenge exceptional probability is at most 2^-192. It slightly improves gamma/3 here. The parity test retains its stronger distance 1/4 (the cube-root radius at distance 1/2 is about 0.2063). For eta inputs, both external tests receive 132+log2(eta) bits, preserving their joint union-bound allocation as the number of inputs grows. Here the Khatam slack is separate from the input-count notation. Native WHIR configurations receive a 136-bit local budget, leaving room for composition at the stated conditional 128-bit target. Initial x/u configurations use unique decoding; Canopy and the batched recursive tails use RS list decoding with the current BCGM26 m=10 bound. The implementation retains the `Johnson` regime name for this path. There is no proof of work. The outer distance assumption and the existing public-H scope remain material limitations.


The current RS bound is `(10.5^7 * n_RS^2) / (3 * rho^(3/2) * |F|)` per
affine challenge at radius `1 - (1 + 1/20) sqrt(rho)`; multilinear folds sum
the coordinate errors. The [BCGM estimate report](../results/paper_bcgm_m10/README.md)
records the update and its validation separately from historical timings.

## Reproduction and output

Use the [repository reproduction guide](../README.md#reproduce-the-final-suite)
for the local 75-point estimate runner, native Linux campaign, and balanced
N=2^22/N=2^24 rechecks. New runs use fresh output directories. The
[campaign guide](../experiments/aws-r5/README.md) describes resources, raw
records, and reporting. Timing builds omit `fs-accounting`.

Proof searches retain every side candidate and selected heights. Proof-size
models exclude serializer framing; timing rows also retain actual serialized
proof bytes. Reported commitment time is C(N,eta)=eta*C(N,1), while opening
is measured at the requested eta. Single-pass and repeated timing datasets
retain separate measurement designs.

## Verifier hash accounting

Canopy maps each 256-bit digest invertibly into GF(2^256), with zero
`verifier_hash_to_field` calls. Verifier totals count authentication and
Fiat-Shamir hash/XOF finalizations. Counts refer to complete invocations rather
than BLAKE3 compression blocks. Transcript absorbed/squeezed bytes are reported
separately, and Fiat-Shamir adds no proof bytes.

The main campaign's
[corrected report](../results/aws_i4i_optimized/comparison_iop_rounds_direct_digest.csv)
removes the obsolete verifier rehash count while preserving original proof sizes
and timings; its Canopy timings still contain that removed work. The N=2^30
extension and repeated Bolt checks use direct conversion throughout.

Earlier k/2, full-block, w=16, delayed-join, and prime-field studies are in
[archive](../archive/README.md), with the older detailed
[half-block protocol guide](../archive/docs/experiment_bolt_max_half_blocks.md)
and [prime-field notes](../archive/docs/experiment_bolt_max_prime.md).
