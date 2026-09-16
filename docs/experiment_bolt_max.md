# Bolt-max with GF(2^256), Canopy RS, and WHIR

The active suite uses GF(2^256), BLAKE3-256, degree-8/compression-8 all-one Bolt-max, and rate-1/2 additive RS for the syndrome. The public sparse matrix H follows the pinned upstream ensemble at revision `3832e47b24e7b3e10525c9c5bcfc1cfe66d525f2`. Its ensemble distance gamma=0.013 is an explicit assumption in these measurements. No claim that a larger extension field improves that distance is needed.

The [suite report](../results/gf256_split_half_suite/README.md) contains proof sizes at input logs 18,20,22,24,26,28,30 and timings at logs 18,20,22. Each measured argument covers eta in {1,2,4,8} inputs of N field elements each. Both commitment and opening are included. Public-H setup, deterministic full-width input generation, and verification are outside prover time. Every measured Bolt proof is serialized and verified. H is evaluated directly: Spark/Mulperm preprocessing and proofs are excluded, as in the historical adapter.

## Geometry

For each of eta in {1,2,4,8} input polynomials, containing N field elements each, fix w=4 and k=N/4:

- X has k rows, each of width w.
- U=HX has k/8 rows.
- Y=RS(U) has k/4 rows.
- The two Canopy RS codewords each have fixed length k/2.
- Canopy messages have k/2^h_X and (k/4)/2^h_Y coefficients. Heights count Merkle levels above complete rows.

## Batched comparison

Bolt already batches its code-switch and syndrome RS proximity proofs (paper Remark 4.2). We compare shared and separate recursive proofs for the Merkle baseline and select the smaller proof; the shared configuration wins at every grid point, with 2 eta source claims and join message length k/8.

Canopy instead uses two recursive batches, one for each side. Each batches eta code-switch claims with eta corresponding nonconstant Canopy SEP claims. A full-height constant Canopy layer is opened directly, leaving eta code-switch claims in that batch. The two batches use distinct challenges within one domain-separated Fiat-Shamir transcript. The systematic and parity heights are optimized independently, requiring each nonconstant Canopy codeword to match only its corresponding WHIR oracle in message and block length.

The code-switch messages x and u reach their respective join lengths through authenticated WHIR transitions with folds of at most three variables. Each transition runs sumcheck, commits the reduced message, then queries and authenticates the previous oracle. The verifier derives the resulting linear constraint with the transposed additive RS map. These costs are included in both proof sizes and opening time. For N>=2^20, all selected Canopy configurations have (h_X,h_Y)=(2,3), message lengths (k/4,k/32), and join at each side's second WHIR oracle. At N=2^18, the selected height pairs are (5,3), (16,3), (16,3), (16,2) for eta=1,2,4,8; systematic height 16 is full height.

Each source's public constraints are combined before batching. The implementation supplies a full matrix of distinct-functional cross evaluations to native multi-root WHIR, including all OOD cross evaluations. This has the same shared-tail purpose as Experiment 3, but uses p(p-1) functional cross fields for p sources instead of its compressed 2p-3 coefficient encoding. No compressed-batching savings are claimed for this implementation.

## Arithmetic and soundness

The canonical field and additive RS implementations are [binary_field.rs](../src/binary_field.rs) and [additive_rs.rs](../src/additive_rs.rs). The tower is K=GF(2)[x]/(x^128+x^7+x^2+x+1), then K[w]/(w^2+w+x^121). Elements occupy 32 bytes. Integer conversion has characteristic-two semantics; benchmark inputs and public evaluation points use full-width hash-derived values.

The systematic query radius is max(gamma/3, 1−(1−gamma+epsilon)^(1/3)−3eta), with epsilon=eta=2^-32. This applies [Khatam, Theorem 1](https://eprint.iacr.org/2024/1843.pdf) as an explicit proximity assumption for the Bolt adapter. The “one-and-a-half Johnson” radius is the cube-root expression, not 1.5 times the ordinary Johnson radius. Its per-challenge exceptional probability is at most 2^-192. It slightly improves gamma/3 here. The parity test retains its stronger distance 1/4 (the cube-root radius at distance 1/2 is about 0.2063). For eta inputs, both external tests receive 132+log2(eta) bits, preserving their joint union-bound allocation as the number of inputs grows. Here the Khatam slack is separate from the input-count notation. Native WHIR configurations receive a 136-bit local budget, leaving room for composition at the stated conditional 128-bit target. Initial x/u configurations use unique decoding; Canopy and the batched recursive tails use Johnson decoding. There is no proof of work. The outer distance assumption and the existing public-H scope remain material limitations.

## Reproduction and output

```sh
cargo build --offline --release --example gf256_bolt_split
RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_split estimate \
  --width 4 --block-divisor 2 --samples 100 \
  --output results/gf256_split_half_suite/proof_sizes
python3 scripts/run_gf256_split_half_timings.py
cargo build --offline --release --features fs-accounting --example gf256_bolt_split
RAYON_NUM_THREADS=8 target/release/examples/gf256_bolt_split validate \
  --input results/gf256_split_half_suite/proof_sizes/proof_sizes.csv \
  --output results/gf256_split_half_suite/fiat_shamir_validation.csv
python3 scripts/summarize_gf256_split_half_suite.py
```

The suite runs timing commands for all three sizes and all four input counts, sequentially, alternating the selected Merkle and Canopy configurations. Each has one warmup and five measured repetitions; all 144 proofs are verified. Timing builds omit the diagnostic `fs-accounting` feature. `scripts/measure_gf256_run.py` records exact commands, executable hashes, wall time, and whole-process peak RSS. The search retains every side candidate in `proof_sizes/independent_height_search.csv`. Shared-baseline estimates are reused from the retained batched suite. `paper_proof_sizes.csv` and `paper_timing_summary.csv` export the two selected methods. Raw timing and stage CSV files preserve every measured repetition. Proof estimates exclude serializer framing; executable timing rows also record actual serialized bytes.

[Prover-time estimates at N=2^30](../results/n30_prover_time_estimates/README.md) extrapolate the saved w=4, k/2 stage timings and the three WHIR configurations. They assume comparable eight-thread throughput and sufficient RAM; they are not N=2^30 measurements. Reproduce with `python3 scripts/estimate_n30_prover_times.py`.

Historical prime-field measurements and commands are in [the archive](archive/experiment_bolt_max_prime.md).

## Verifier hash accounting

Every selected proof and height-search row reports leaf, parent, and total authentication hash calls, plus H2F calls separately. The outer forest verifier currently computes a parent for each child path, including both siblings; these repeated calls are counted. Native WHIR hashes all transmitted rows, then deduplicates paths and computes each parent once. Constant Canopy roots cost one leaf hash and log2(RS length) parent hashes. Received root/authentication digests do not themselves require hashing. Counts are means over the same sampled query sets used for proof sizes. A hash call is a complete BLAKE3 invocation, not a compression block; row widths affect its cost. The selected proof rows additionally report total verifier hashes = authentication + H2F + Fiat–Shamir hash/XOF finalizations, plus absorbed/squeezed FS bytes. The original height-search rows retain only authentication/H2F components. Fiat-Shamir adds zero proof bytes. All 12 selected Canopy configurations have [matching executed verifier traces](../results/gf256_split_half_suite/fiat_shamir_validation.csv); the Merkle accounting follows the [original trace-validated model](../results/gf256_batched_suite/fiat_shamir/README.md).

The [w=16 comparison](../results/gf256_split_w16_suite/README.md), [w=4 split suite with block length k](../results/gf256_split_suite/README.md) and [original shared-batch suite](../results/gf256_batched_suite/comparison.md) are retained for reproducibility. The previous suite including unbatched experiments is preserved in [historical results](../results/gf256_suite/comparison.md); the active timing runner emits only the two selected batched methods.
