# GF(2^256): WHIR/Canopy and Bolt-max

All schemes use **GF(2^256), 32-byte elements, BLAKE3-256, eight Rayon threads, and a 128-bit soundness target**. Timings are from Apple M2 with 16 GiB RAM, one warmup and five measured repetitions. Medians headline total prover time; additive breakdowns use arithmetic means. Commitment plus opening equals total. Input generation, public setup, and verification are excluded.

**Timing boundaries differ between suites.** WHIR/Canopy retains the established component-based harness: independently measured work is scaled for independent polynomials, recursive native WHIR components are executed and verified, and a separately measured suffix is subtracted to model the paper terminal cap. It is not a serialized end-to-end Canopy proof. Bolt timings execute complete serialized, verifier-checked proofs of the public-H adapter. Bolt still excludes Spark/Mulperm, and assumes LDPC distance gamma=0.013.

At **2^22 with eight polynomials**, WHIR Experiment 3 has a 24.8% smaller proof than batched rate-1/2 WHIR (1452.7 vs 1932.7 KiB), with median time 11.662 vs 7.376 s. Batched rate-1/4 WHIR has the smallest proof (1154.0 KiB), at 12.841 s.

At **2^22 for Bolt**, the selected unbatched Canopy configuration reduces proof size by 32.9% with 24.2% median prover-time overhead (0.914 vs 0.736 s). Both Canopy blocks have length k and w=4. The batched Canopy variant has a larger proof at this point; batching becomes the smallest Canopy variant at 2^30.

## WHIR proof sizes

KiB, including roots and all modeled proof components. Each row is the total for t polynomials of 2^n field elements each. The size search optimizes ordered folds in {3,4}, terminal messages of at most 2^10 coefficients, and Experiment 2 heights. Experiment 3 aligns h with its first fold. All 168 selected rows meet the modeled 128-bit RBR check. Means use 100 deterministic query samples.

### t=1

| Input | WHIR independent 1/2 | WHIR batched 1/2 | WHIR independent 1/4 | WHIR batched 1/4 | Canopy Exp. 2 | Canopy Exp. 3 |
|---|---|---|---|---|---|---|
| 2^18 | 225.91 | 225.91 | 149.79 | 149.79 | 222.37 | 195.81 |
| 2^20 | 265.86 | 265.86 | 176.32 | 176.32 | 256.23 | 218.12 |
| 2^22 | 311.88 | 311.88 | 207.79 | 207.79 | 293.02 | 244.97 |
| 2^24 | 350.83 | 350.83 | 235.15 | 235.15 | 325.31 | 268.22 |
| 2^26 | 392.74 | 392.74 | 264.00 | 264.00 | 359.05 | 292.66 |
| 2^28 | 434.90 | 434.90 | 292.97 | 292.97 | 390.79 | 317.07 |
| 2^30 | 475.50 | 475.50 | 318.97 | 318.97 | 421.04 | 340.76 |

### t=2

| Input | WHIR independent 1/2 | WHIR batched 1/2 | WHIR independent 1/4 | WHIR batched 1/4 | Canopy Exp. 2 | Canopy Exp. 3 |
|---|---|---|---|---|---|---|
| 2^18 | 451.56 | 408.73 | 299.49 | 259.25 | 444.82 | 351.52 |
| 2^20 | 531.86 | 473.49 | 352.61 | 298.61 | 512.65 | 381.90 |
| 2^22 | 624.05 | 543.72 | 415.58 | 342.69 | 586.23 | 417.53 |
| 2^24 | 702.24 | 606.86 | 470.29 | 383.17 | 650.67 | 449.01 |
| 2^26 | 785.60 | 672.81 | 527.87 | 424.67 | 718.13 | 481.91 |
| 2^28 | 869.63 | 739.11 | 586.00 | 466.58 | 781.64 | 514.72 |
| 2^30 | 950.93 | 804.82 | 638.04 | 507.43 | 842.35 | 546.81 |

### t=4

| Input | WHIR independent 1/2 | WHIR batched 1/2 | WHIR independent 1/4 | WHIR batched 1/4 | Canopy Exp. 2 | Canopy Exp. 3 |
|---|---|---|---|---|---|---|
| 2^18 | 903.29 | 774.60 | 599.37 | 478.17 | 889.02 | 662.32 |
| 2^20 | 1063.83 | 887.65 | 705.00 | 543.22 | 1025.61 | 710.26 |
| 2^22 | 1248.45 | 1007.28 | 830.51 | 612.80 | 1172.38 | 762.48 |
| 2^24 | 1404.21 | 1118.59 | 940.11 | 679.05 | 1300.59 | 810.69 |
| 2^26 | 1571.32 | 1232.83 | 1056.22 | 746.24 | 1436.30 | 860.18 |
| 2^28 | 1739.32 | 1347.88 | 1171.70 | 813.89 | 1563.08 | 910.29 |
| 2^30 | 1901.76 | 1461.93 | 1276.10 | 880.21 | 1684.68 | 958.67 |

### t=8

| Input | WHIR independent 1/2 | WHIR batched 1/2 | WHIR independent 1/4 | WHIR batched 1/4 | Canopy Exp. 2 | Canopy Exp. 3 |
|---|---|---|---|---|---|---|
| 2^18 | 1807.33 | 1506.40 | 1198.74 | 916.29 | 1778.39 | 1284.51 |
| 2^20 | 2127.06 | 1716.51 | 1409.93 | 1032.64 | 2049.89 | 1366.83 |
| 2^22 | 2497.93 | 1932.73 | 1661.73 | 1154.05 | 2345.15 | 1452.72 |
| 2^24 | 2806.71 | 2141.44 | 1880.91 | 1271.07 | 2601.35 | 1534.37 |
| 2^26 | 3143.09 | 2353.24 | 2111.32 | 1389.76 | 2872.15 | 1617.32 |
| 2^28 | 3480.23 | 2564.70 | 2343.04 | 1506.91 | 3126.15 | 1699.70 |
| 2^30 | 3803.34 | 2776.88 | 2553.32 | 1624.38 | 3368.67 | 1782.59 |

[Selected parameters and full proof components](whir/proof_sizes/hybrid_proof_sizes.csv) · [All fold/height candidates](whir/proof_sizes/hybrid_parameter_grid.csv) · [Per-oracle queries](whir/proof_sizes/oracle_query_breakdown.csv) · [RBR events](whir/proof_sizes/rbr_security_breakdown.csv).

## WHIR prover times

Median total seconds. The timing harness uses uniform fold 3, Experiment 3 height 4, and an independent 32-sample height search for Experiment 2 under that executable schedule. Consequently these times do not claim to measure every proof-size-optimal mixed folding schedule above.

| Input 2^18; t | WHIR independent 1/2 | WHIR batched 1/2 | WHIR independent 1/4 | WHIR batched 1/4 | Canopy Exp. 2 | Canopy Exp. 3 |
|---|---|---|---|---|---|---|
| 1 | 0.0527 | 0.0527 | 0.0817 | 0.0817 | 0.0613 | 0.0721 |
| 2 | 0.1054 | 0.0970 | 0.1633 | 0.1467 | 0.1226 | 0.1229 |
| 4 | 0.2109 | 0.1847 | 0.3267 | 0.2770 | 0.2451 | 0.2240 |
| 8 | 0.4217 | 0.3672 | 0.6533 | 0.5414 | 0.4903 | 0.4537 |

| Input 2^20; t | WHIR independent 1/2 | WHIR batched 1/2 | WHIR independent 1/4 | WHIR batched 1/4 | Canopy Exp. 2 | Canopy Exp. 3 |
|---|---|---|---|---|---|---|
| 1 | 0.2071 | 0.2071 | 0.3369 | 0.3369 | 0.3046 | 0.2859 |
| 2 | 0.4143 | 0.3902 | 0.6739 | 0.6272 | 0.6092 | 0.4973 |
| 4 | 0.8286 | 0.7320 | 1.3478 | 1.1939 | 1.2184 | 0.9510 |
| 8 | 1.6572 | 1.4656 | 2.6956 | 2.3837 | 2.4368 | 1.9855 |

| Input 2^22; t | WHIR independent 1/2 | WHIR batched 1/2 | WHIR independent 1/4 | WHIR batched 1/4 | Canopy Exp. 2 | Canopy Exp. 3 |
|---|---|---|---|---|---|---|
| 1 | 1.0200 | 1.0200 | 1.7613 | 1.7613 | 1.4591 | 1.4513 |
| 2 | 2.0400 | 1.8930 | 3.5227 | 3.2966 | 2.9181 | 2.5294 |
| 4 | 4.0800 | 3.6416 | 7.0454 | 6.3325 | 5.8362 | 4.9941 |
| 8 | 8.1601 | 7.3756 | 14.0908 | 12.8408 | 11.6724 | 11.6625 |

[Mean commitment/opening, medians and ranges](whir/timing_summary.csv) · [Detailed stages](whir/timing_stages.csv). The detailed report below shows the additive phase breakdown.

## Bolt proof sizes

Each method selects its smallest proof across widths {4,8,16}; Canopy also searches both-codeword lengths k/4,k/2,k, with k=N/w. Unbatched heights vary independently. Batched nonconstant layers enforce h_Y=h_X-2 and search every compatible join length; full-height constant layers are also candidates. Merkle batching searches its own join length.

Batched proofs include the commitments and openings needed to align x=Xr and u=Ur, then prove the aligned claims with one native WHIR tail. Functional cross evaluations use a full matrix (p(p-1) fields); this implementation does not claim Experiment 3's compressed 2p-3-field encoding. The two external tests receive 132-bit local budgets and WHIR uses 136 bits to leave composition margin.

| Input | Merkle, unbatched | Merkle, batched | Canopy, unbatched | Canopy, batched |
|---|---|---|---|---|
| 2^18 | 4096.75 | 3914.12 | 3949.23 | 3817.10 |
| 2^20 | 5433.08 | 5143.59 | 4346.81 | 4650.74 |
| 2^22 | 6811.92 | 6476.14 | 4571.41 | 5008.46 |
| 2^24 | 8352.79 | 7869.06 | 4880.96 | 5159.36 |
| 2^26 | 9939.40 | 9283.46 | 5199.19 | 5247.55 |
| 2^28 | 11429.02 | 10693.09 | 5424.61 | 5309.82 |
| 2^30 | 13068.28 | 12118.93 | 5795.61 | 5379.19 |

All entries above are KiB. Parameters:

| Input | Method | Width, block length, heights, join |
|---|---|---|
| 2^18 | Merkle, unbatched | w=4 |
| 2^18 | Merkle, batched | w=4, join=2^13 |
| 2^18 | Canopy, unbatched | w=4, k, h=1/14 |
| 2^18 | Canopy, batched | w=4, k, h=3/1, join=2^13 |
| 2^20 | Merkle, unbatched | w=4 |
| 2^20 | Merkle, batched | w=4, join=2^15 |
| 2^20 | Canopy, unbatched | w=4, k, h=1/16 |
| 2^20 | Canopy, batched | w=4, k, h=3/1, join=2^15 |
| 2^22 | Merkle, unbatched | w=4 |
| 2^22 | Merkle, batched | w=4, join=2^17 |
| 2^22 | Canopy, unbatched | w=4, k, h=1/18 |
| 2^22 | Canopy, batched | w=4, k, h=3/1, join=2^17 |
| 2^24 | Merkle, unbatched | w=4 |
| 2^24 | Merkle, batched | w=4, join=2^19 |
| 2^24 | Canopy, unbatched | w=4, k, h=1/20 |
| 2^24 | Canopy, batched | w=4, k, h=3/1, join=2^19 |
| 2^26 | Merkle, unbatched | w=4 |
| 2^26 | Merkle, batched | w=4, join=2^21 |
| 2^26 | Canopy, unbatched | w=4, k, h=1/22 |
| 2^26 | Canopy, batched | w=4, k, h=3/1, join=2^21 |
| 2^28 | Merkle, unbatched | w=4 |
| 2^28 | Merkle, batched | w=4, join=2^23 |
| 2^28 | Canopy, unbatched | w=4, k, h=1/24 |
| 2^28 | Canopy, batched | w=4, k, h=3/1, join=2^23 |
| 2^30 | Merkle, unbatched | w=4 |
| 2^30 | Merkle, batched | w=4, join=2^25 |
| 2^30 | Canopy, unbatched | w=4, k, h=1/26 |
| 2^30 | Canopy, batched | w=4, k, h=3/1, join=2^25 |

[All widths/layouts/modes](bolt/proof_sizes/proof_sizes.csv) · [Batched height/join search](bolt/proof_sizes/batching_height_search.csv) · [Selected global parameters](bolt/best_proof_parameters.csv) · [Protocol and limitations](../../../docs/experiment_bolt_max.md).

[Canopy savings against Merkle at each matching width and batching mode](bolt/matched_width_proof_comparison.csv).

## Bolt prover times

Median total seconds at each method's proof-size-selected width/layout/heights above; the complete timing grid includes all widths and layouts.

| Input | Merkle, unbatched | Merkle, batched | Canopy, unbatched | Canopy, batched |
|---|---|---|---|---|
| 2^18 | 0.0606 | 0.0564 | 0.0701 | 0.0705 |
| 2^20 | 0.1832 | 0.1712 | 0.2260 | 0.2321 |
| 2^22 | 0.7363 | 0.6816 | 0.9145 | 0.9260 |

Additive means at those same configurations:

| Input | Method | Commit s | Opening s | Total s | Actual proof KiB |
|---|---|---|---|---|---|
| 2^18 | Merkle, unbatched | 0.0151 | 0.0455 | 0.0606 | 4095.73 |
| 2^18 | Merkle, batched | 0.0149 | 0.0422 | 0.0571 | 3915.83 |
| 2^18 | Canopy, unbatched | 0.0229 | 0.0501 | 0.0730 | 3952.02 |
| 2^18 | Canopy, batched | 0.0222 | 0.0675 | 0.0896 | 3816.47 |
| 2^20 | Merkle, unbatched | 0.0612 | 0.1226 | 0.1837 | 5431.84 |
| 2^20 | Merkle, batched | 0.0593 | 0.1119 | 0.1711 | 5141.99 |
| 2^20 | Canopy, unbatched | 0.0901 | 0.1342 | 0.2243 | 4343.78 |
| 2^20 | Canopy, batched | 0.0896 | 0.1419 | 0.2315 | 4656.82 |
| 2^22 | Merkle, unbatched | 0.2659 | 0.4606 | 0.7266 | 6816.55 |
| 2^22 | Merkle, batched | 0.2616 | 0.4149 | 0.6765 | 6479.46 |
| 2^22 | Canopy, unbatched | 0.3915 | 0.5229 | 0.9144 | 4572.41 |
| 2^22 | Canopy, batched | 0.3846 | 0.5427 | 0.9273 | 5010.29 |

[All timing configurations and observed proof bytes](bolt/timing_summary.csv) · [Raw repetitions](bolt/timing_samples.csv) · [Raw stage measurements](bolt/timing_stages.csv). Actual proofs use one transcript’s query realization and include framing; size estimates average sampled queries and omit framing.

## Memory and validation

Peak RSS covers each complete process, including setup, verification, and retained allocator memory across sequential schemes. It is not an individual prover-only allocation bound.

| Process | Peak GiB | Wall s |
|---|---|---|
| WHIR n=18 | 0.884 | 12.5 |
| Bolt n=18, w=4 | 0.135 | 6.4 |
| Bolt n=18, w=8 | 0.136 | 5.6 |
| Bolt n=18, w=16 | 0.126 | 4.6 |
| WHIR n=20 | 2.719 | 53.1 |
| Bolt n=20, w=4 | 0.378 | 14.1 |
| Bolt n=20, w=8 | 0.263 | 11.3 |
| Bolt n=20, w=16 | 0.233 | 11.3 |
| WHIR n=22 | 6.355 | 298.6 |
| Bolt n=22, w=4 | 1.131 | 48.9 |
| Bolt n=22, w=8 | 0.684 | 34.2 |
| Bolt n=22, w=16 | 0.533 | 29.5 |

Validation: 168 WHIR proof-size rows, 168 Bolt proof-size rows, 360 WHIR component timing samples, and 360 serialized verified Bolt timing samples. All timing stages reconcile to phase totals; all proof-size components reconcile to their totals. Native WHIR supports zero-polynomial verification without division by zero. Binary arithmetic, additive RS and its transpose, Bolt row checks, batching alignment, tamper rejection, and small-instance serialized byte accounting are tested.

[Detailed additive time breakdowns](timing_breakdown.md) · [Reproduction commands and executable hashes](run_manifest.json). Historical prime-field measurements remain in their original directories.
