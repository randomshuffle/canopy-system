# GF(2^256) WHIR/Canopy timing backend

This isolated crate runs the same WHIR/Canopy component timing harness as the
prime-field crate. It does not compile Bolt or replace the main crate's field.
There is no 192-bit recursive field and no field switching.

The field is

- K = GF(2)[x]/(x^128 + x^7 + x^2 + x + 1);
- L = K[w]/(w^2 + w + x^121), of cardinality 2^256.

Elements have two 128-bit polynomial-basis limbs, encoded in 32 little-endian
bytes. Addition is XOR. On Apple Silicon, multiplication uses ARM PMULL; other
architectures use the portable reference kernel. The absolute trace of x^121
in K is one, making the quadratic extension irreducible. Tests check that
trace, multiplication against the portable kernel, inverses, Frobenius,
serialization, and field identities.

The encoder evaluates **monomial coefficients** on additive Cantor subspaces.
It converts to a novel polynomial basis internally, applies additive FFTs on
cosets, and returns the same row-major interleaving expected by WHIR. Tests
compare codeword entries with direct Horner evaluations. A transposed encoder
computes WHIR's weighted power sums; tests compare these with direct sums.
This retains the existing protocol's univariate semantics and avoids using a
multiplicative radix-2 domain in characteristic two.

The shared vendored WHIR changes:

- standard sumcheck avoids the unused division by two in the ZK mask path;
- binary Merkle rows use packed canonical serialization;
- RS opening metadata distinguishes additive from multiplicative domains;
- an optional additive power-sum hook complements the prime sparse NTT path.

The existing odd-characteristic and Bolt paths retain their field,
parameters, arithmetic, serialization, and claim points. This runner is a
non-ZK research benchmark; the reference's ZK protocol still excludes
characteristic two.

Run from the repository root:

```sh
cargo test --manifest-path experiments/binary-whir/Cargo.toml --lib --offline
cargo build --manifest-path experiments/binary-whir/Cargo.toml --release --offline
cargo build --release --offline
python3 scripts/measure_whir_field.py --field binary --output-dir results/whir_field_comparison_n22/binary
python3 scripts/measure_whir_field.py --field prime --output-dir results/whir_field_comparison_n22/prime
```

Both runs use n=22, t=1/2/4/8, BLAKE3-256, 128-bit Johnson parameters, no PoW,
eight Rayon threads, one warmup and five measured repetitions. Input values
and claim points are deterministic full-width field elements. Workloads,
heights and timing boundaries are shared. The unbatched Canopy height h=9
comes from the existing n=22 paper proof-size grid; batched Canopy uses h=4.
All executable folds remove three variables.

These are the existing paper's **component-composed prover timings**. The
harness verifies each executable WHIR component, scales independently
measured first transitions by t, executes batched tails and cross terms at
the actual t, and subtracts a separately measured suffix to approximate the
paper's terminal size of at most 2^10. They are not a measurement of one
serialized end-to-end Canopy proof. Setup, input generation, verification,
and final deallocation are excluded from prover totals. Commitment covers
initial RS plus Merkle/Canopy commitment; opening covers OOD evaluations,
subsequent commitments, openings and recursive proof work. Whole-process RSS
includes all of those excluded phases as well.

See the generated field-comparison report for results, raw samples, stage
breakdowns, exact commands and executable hashes. The older n=18 prime
results use a different workload size and integer-pattern inputs; use the
fresh n=22 prime run for field ratios.
