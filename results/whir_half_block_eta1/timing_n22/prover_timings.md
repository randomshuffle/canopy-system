# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 0.3350 | 0.3932 | 0.7281 | 0.7308 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 0.5977 | 0.8319 | 1.4296 | 1.4675 | 1.963x | 1.963x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 0.4661 | 0.4325 | 0.8985 | 0.9025 | 1.234x | 1.234x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.1229 | 0.1213 | 16.9% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0356 | 0.0353 | 4.9% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0116 | 0.0116 | 1.6% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.2994 | 0.3001 | 41.1% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.1342 | 0.1333 | 18.4% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0016 | 0.0016 | 0.2% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1229 | 0.1227 | 16.9% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.3454 | 0.3701 | 24.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0692 | 0.0693 | 4.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0127 | 0.0117 | 0.9% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.5285 | 0.5222 | 37.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.2051 | 0.2041 | 14.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0015 | 0.0015 | 0.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.2672 | 0.2708 | 18.7% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.1408 | 0.1405 | 15.7% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0302 | 0.0301 | 3.4% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0003 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0185 | 0.0178 | 2.1% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0150 | 0.0148 | 1.7% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1027 | 0.1024 | 11.4% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0238 | 0.0237 | 2.6% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.2560 | 0.2534 | 28.5% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0116 | 0.0116 | 1.3% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.2997 | 0.2975 | 33.3% |
