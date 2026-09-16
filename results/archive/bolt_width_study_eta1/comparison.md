# GF(2^256): WHIR/Canopy and Bolt-max

The active suite has exactly three WHIR methods and two Bolt methods, all batched. Over all 28 WHIR size/polynomial-count combinations, rate-1/4 WHIR has the smallest proof. The verifier totals include authentication, H2F, and Fiat–Shamir. The tables pair these costs rather than collapsing them into a single score.

All schemes use **GF(2^256), 32-byte elements, BLAKE3-256, eight Rayon threads, and a 128-bit soundness target**. Timings are from Apple M2 with 16 GiB RAM, one warmup and five measured repetitions. Medians headline total prover time; additive breakdowns use arithmetic means. Commitment plus opening equals total. Input generation, public setup, and verification are excluded.

**Timing boundaries differ between suites.** WHIR/Canopy retains the established component-based harness: independently measured work is scaled for independent polynomials, recursive native WHIR components are executed and verified, and a separately measured suffix is subtracted to model the paper terminal cap. It is not a serialized end-to-end Canopy proof. Bolt timings execute complete serialized, verifier-checked proofs of the public-H adapter. Bolt still excludes Spark/Mulperm, and assumes LDPC distance gamma=0.013.

Khatam uses the one-and-a-half Johnson radius 1−(1−Δ)^(1/3). For the outer systematic test we use max(γ/3, 1−(1−γ+ε)^(1/3)−3η), with ε=η=2^-32 and assumed γ=0.013; the parity test retains its stronger 1/4 radius. The per-challenge Khatam exceptional probability is at most 2^-192. This is a conditional Bolt adaptation, not a new proof of this adapter. [Khatam, Theorem 1](https://eprint.iacr.org/2024/1843.pdf). Native RS-WHIR parameters retain their existing security bounds.

Authentication counts are mean BLAKE3 leaf and binary-parent hash calls for the selected proof parameters. They are not the number of digests transmitted and not BLAKE3 compression-block counts. Different leaf widths hash different byte lengths. WHIR size models deduplicate query rows; Bolt models the executable native verifier, which hashes repeated transmitted rows before deduplication. The outer forest verifier hashes paired siblings twice; those calls are included. Received roots and authentication digests themselves cost no hash call.

Verifier hashes include authentication, H2F, and Fiat–Shamir hash/XOF finalizations. Consecutive squeezes from the same XOF reader count once. FS absorbed and squeezed bytes are reported separately in [the hash breakdown](verifier_hashes.md); these are not compression-call counts. No proof of work is used. Fiat–Shamir adds zero proof bytes: challenges are derived, and all prover messages were already counted. Existing timing samples include the FS work performed by their executed components and are unchanged. WHIR FS is an explicit composed transcript model matching its three-coefficient communication model; Bolt FS is checked against real verifier traces. Cached public protocol/session identifier construction is setup, while their 96-byte absorption is included. See [accounting details](fiat_shamir/README.md).

The earlier WHIR setup used a 128-bit per-event budget, which gave a 124.5–127.2-bit algebraic union bound across these workloads. This run reserves a 136-bit local budget and validates the composed 128-bit bound. Earlier provisional WHIR numbers are superseded.

## WHIR proof sizes

Each cell is proof size (KiB) / verifier hashes. Sizes include roots and all modeled proof components. Each row is the total for t polynomials of 2^n field elements each. The size search optimizes ordered folds in {3,4}, terminal messages of at most 2^10 coefficients. Experiment 3 aligns h with its first fold. The local algebraic/query budget is 136 bits; all 84 selected rows meet the composed 128-bit union-bound check. Means use 100 deterministic query samples.

### t=1

| Input | WHIR batched 1/2 | WHIR batched 1/4 | Canopy Exp. 3 |
|---|---|---|---|
| 2^18 | 237.80 / 4,251 | 157.52 / 2,901 | 206.42 / 3,026 |
| 2^20 | 280.49 / 5,624 | 186.08 / 3,856 | 230.50 / 3,833 |
| 2^22 | 329.04 / 6,597 | 219.78 / 4,330 | 259.26 / 4,149 |
| 2^24 | 370.86 / 8,072 | 248.80 / 5,462 | 283.87 / 5,139 |
| 2^26 | 415.49 / 9,587 | 279.18 / 5,696 | 309.45 / 5,423 |
| 2^28 | 460.53 / 10,458 | 309.33 / 6,720 | 335.29 / 6,309 |
| 2^30 | 503.68 / 11,023 | 338.30 / 7,312 | 360.32 / 6,776 |

### t=2

| Input | WHIR batched 1/2 | WHIR batched 1/4 | Canopy Exp. 3 |
|---|---|---|---|
| 2^18 | 431.24 / 7,962 | 273.26 / 5,290 | 371.38 / 5,545 |
| 2^20 | 500.24 / 10,172 | 315.41 / 6,678 | 404.05 / 6,625 |
| 2^22 | 574.66 / 11,971 | 362.59 / 7,584 | 442.10 / 7,235 |
| 2^24 | 642.16 / 14,267 | 405.59 / 9,163 | 475.53 / 8,507 |
| 2^26 | 712.45 / 16,603 | 449.37 / 10,030 | 509.86 / 9,072 |
| 2^28 | 783.21 / 18,298 | 493.75 / 11,509 | 544.57 / 10,241 |
| 2^30 | 852.75 / 20,085 | 537.19 / 12,565 | 578.66 / 10,998 |

### t=4

| Input | WHIR batched 1/2 | WHIR batched 1/4 | Canopy Exp. 3 |
|---|---|---|---|
| 2^18 | 818.26 / 15,383 | 504.82 / 10,066 | 700.61 / 10,564 |
| 2^20 | 938.67 / 19,229 | 574.46 / 12,333 | 751.62 / 12,217 |
| 2^22 | 1065.62 / 22,706 | 648.86 / 14,109 | 807.33 / 13,393 |
| 2^24 | 1184.42 / 26,644 | 718.88 / 16,552 | 858.56 / 15,235 |
| 2^26 | 1306.37 / 30,633 | 790.00 / 18,294 | 910.73 / 16,370 |
| 2^28 | 1428.73 / 33,979 | 861.74 / 20,649 | 963.70 / 18,123 |
| 2^30 | 1550.04 / 37,422 | 932.35 / 22,574 | 1014.88 / 19,427 |

### t=8

| Input | WHIR batched 1/2 | WHIR batched 1/4 | Canopy Exp. 3 |
|---|---|---|---|
| 2^18 | 1592.73 / 30,226 | 968.09 / 19,620 | 1359.42 / 20,616 |
| 2^20 | 1816.50 / 37,376 | 1092.26 / 23,631 | 1447.24 / 23,426 |
| 2^22 | 2045.64 / 44,116 | 1221.50 / 27,161 | 1538.56 / 25,735 |
| 2^24 | 2268.67 / 51,389 | 1345.86 / 31,344 | 1625.03 / 28,702 |
| 2^26 | 2493.82 / 58,680 | 1472.27 / 34,855 | 1712.76 / 30,974 |
| 2^28 | 2719.15 / 65,320 | 1596.26 / 38,881 | 1800.10 / 33,829 |
| 2^30 | 2944.82 / 72,103 | 1721.48 / 42,554 | 1887.77 / 36,300 |

[Selected parameters and full proof components](whir/proof_sizes/hybrid_proof_sizes.csv) · [All fold/height candidates](whir/proof_sizes/hybrid_parameter_grid.csv) · [Per-oracle queries](whir/proof_sizes/oracle_query_breakdown.csv) · [RBR events](whir/proof_sizes/rbr_security_breakdown.csv).

## WHIR prover times

Median total seconds. The timing harness uses a 136-bit local budget, uniform fold 3, Experiment 3 height 4. Consequently these times do not claim to measure every proof-size-optimal mixed folding schedule above.

| Input 2^18; t | WHIR batched 1/2 | WHIR batched 1/4 | Canopy Exp. 3 |
|---|---|---|---|
| 1 | 0.0529 | 0.0752 | 0.0642 |
| 2 | 0.0906 | 0.1374 | 0.1079 |
| 4 | 0.1760 | 0.2574 | 0.1913 |
| 8 | 0.3526 | 0.5070 | 0.4230 |

| Input 2^20; t | WHIR batched 1/2 | WHIR batched 1/4 | Canopy Exp. 3 |
|---|---|---|---|
| 1 | 0.2079 | 0.3371 | 0.2700 |
| 2 | 0.3687 | 0.6265 | 0.4774 |
| 4 | 0.6863 | 1.1113 | 0.9558 |
| 8 | 1.3589 | 2.2068 | 2.0967 |

| Input 2^22; t | WHIR batched 1/2 | WHIR batched 1/4 | Canopy Exp. 3 |
|---|---|---|---|
| 1 | 0.9129 | 1.2974 | 1.2102 |
| 2 | 1.7609 | 2.4188 | 2.1848 |
| 4 | 3.3947 | 4.6709 | 4.0680 |
| 8 | 6.8096 | 9.3054 | 9.6710 |

The WHIR substage breakdown includes a signed adjustment for separately measured components (2^20, WHIR batched 1/4; up to 7.12% of total). Commitment/opening/total samples are unchanged. The detailed breakdown shows each adjustment.

[Mean commitment/opening, medians and ranges](whir/timing_summary.csv) · [Detailed stages](whir/timing_stages.csv). The detailed report below shows the additive phase breakdown.

## Bolt proof sizes

Each method selects its smallest proof across widths {4,8,16}; Canopy also searches both-codeword lengths k/4,k/2,k, with k=N/w. Nonconstant layers enforce h_Y=h_X-2 and search every compatible join length; full-height constant layers are also candidates. Merkle batching searches its own join length.

Bolt already proposes batching its two RS proximity proofs (Remark 4.2); Canopy adds its commitment-layer claims to that batch. Batched proofs include the commitments and openings needed to align x=Xr and u=Ur, then prove the aligned claims with one native WHIR tail. Functional cross evaluations use a full matrix (p(p-1) fields); this implementation does not claim Experiment 3's compressed 2p-3-field encoding. The two external tests receive 132-bit local budgets and WHIR uses 136 bits to leave composition margin.

| Input | Bolt + Merkle | Bolt + Canopy |
|---|---|---|
| 2^18 | 3902.52 / 96,565 | 3805.29 / 87,245 |
| 2^20 | 5127.26 / 135,613 | 4634.03 / 105,474 |
| 2^22 | 6455.01 / 178,108 | 4989.43 / 113,226 |
| 2^24 | 7841.29 / 222,315 | 5139.38 / 116,851 |
| 2^26 | 9250.87 / 267,289 | 5227.44 / 119,277 |
| 2^28 | 10654.85 / 312,211 | 5289.71 / 121,198 |
| 2^30 | 12074.96 / 357,540 | 5359.36 / 123,307 |

Entries are KiB / verifier hashes. Parameters:

| Input | Method | Width, block length, heights, join |
|---|---|---|
| 2^18 | Bolt + Merkle | w=4, join=2^13 |
| 2^18 | Bolt + Canopy | w=4, k, h=3/1, join=2^13 |
| 2^20 | Bolt + Merkle | w=4, join=2^15 |
| 2^20 | Bolt + Canopy | w=4, k, h=3/1, join=2^15 |
| 2^22 | Bolt + Merkle | w=4, join=2^17 |
| 2^22 | Bolt + Canopy | w=4, k, h=3/1, join=2^17 |
| 2^24 | Bolt + Merkle | w=4, join=2^19 |
| 2^24 | Bolt + Canopy | w=4, k, h=3/1, join=2^19 |
| 2^26 | Bolt + Merkle | w=4, join=2^21 |
| 2^26 | Bolt + Canopy | w=4, k, h=3/1, join=2^21 |
| 2^28 | Bolt + Merkle | w=4, join=2^23 |
| 2^28 | Bolt + Canopy | w=4, k, h=3/1, join=2^23 |
| 2^30 | Bolt + Merkle | w=4, join=2^25 |
| 2^30 | Bolt + Canopy | w=4, k, h=3/1, join=2^25 |

[All widths/layouts](bolt/proof_sizes/proof_sizes.csv) · [Batched height/join search](bolt/proof_sizes/batching_height_search.csv) · [Selected global parameters](bolt/best_proof_parameters.csv) · [Protocol and limitations](../../docs/experiment_bolt_max.md).

[Canopy savings against Merkle at each matching width](bolt/matched_width_proof_comparison.csv).

## Bolt prover times

Median total seconds at each method's proof-size-selected width/layout/heights above; the complete timing grid includes all widths and layouts.

| Input | Bolt + Merkle | Bolt + Canopy |
|---|---|---|
| 2^18 | 0.0516 | 0.0631 |
| 2^20 | 0.1390 | 0.1812 |
| 2^22 | 0.6517 | 0.8205 |

Additive means at those same configurations:

| Input | Method | Commit s | Opening s | Total s | Actual KiB / modeled verifier hashes |
|---|---|---|---|---|---|
| 2^18 | Bolt + Merkle | 0.0149 | 0.0369 | 0.0518 | 3905.20 / 96,565 |
| 2^18 | Bolt + Canopy | 0.0183 | 0.0443 | 0.0626 | 3803.66 / 87,245 |
| 2^20 | Bolt + Merkle | 0.0490 | 0.0905 | 0.1395 | 5126.21 / 135,613 |
| 2^20 | Bolt + Canopy | 0.0702 | 0.1113 | 0.1815 | 4638.20 / 105,474 |
| 2^22 | Bolt + Merkle | 0.3390 | 0.3946 | 0.7336 | 6451.68 / 178,108 |
| 2^22 | Bolt + Canopy | 0.3540 | 0.5009 | 0.8549 | 4988.54 / 113,226 |

[All timing configurations and observed proof bytes](bolt/timing_summary.csv) · [Raw repetitions](bolt/timing_samples.csv) · [Raw stage measurements](bolt/timing_stages.csv). Actual proofs use one transcript’s query realization and include framing; size estimates average sampled queries and omit framing.

## Memory and validation

Peak RSS covers each complete process, including setup, verification, and retained allocator memory across sequential schemes. It is not an individual prover-only allocation bound.

| Process | Peak GiB | Wall s |
|---|---|---|
| WHIR n=18 | 0.853 | 11.5 |
| Bolt n=18, w=4 | 0.136 | 3.0 |
| Bolt n=18, w=8 | 0.115 | 2.6 |
| Bolt n=18, w=16 | 0.102 | 2.2 |
| WHIR n=20 | 2.767 | 52.9 |
| Bolt n=20, w=4 | 0.334 | 6.0 |
| Bolt n=20, w=8 | 0.235 | 5.2 |
| Bolt n=20, w=16 | 0.230 | 5.8 |
| WHIR n=22 | 6.570 | 249.2 |
| Bolt n=22, w=4 | 1.094 | 24.7 |
| Bolt n=22, w=8 | 0.637 | 15.6 |
| Bolt n=22, w=16 | 0.474 | 14.1 |

Validation: 84 WHIR proof-size rows, 84 Bolt proof-size rows, 180 WHIR component timing samples, and 180 serialized verified Bolt timing samples. Bolt raw timing stages reconcile to phase totals; WHIR substages reconcile after the explicit signed component adjustment described below. All proof-size components reconcile to their totals. The 72-test suite passes, including the complete folding-grid composition bound, execution with the increased budget, and exhaustive small-tree authentication accounting. Native WHIR supports zero-polynomial verification without division by zero. Binary arithmetic, additive RS and its transpose, Bolt row checks, batching alignment, tamper rejection, and small-instance serialized byte accounting are tested.

[Verifier hash breakdown](verifier_hashes.md) · [Detailed additive time breakdowns](timing_breakdown.md) · [Reproduction commands and executable hashes](run_manifest.json). Historical prime-field measurements remain in their original directories.
