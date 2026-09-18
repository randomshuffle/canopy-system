# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 23 | - | 3 | 1.0271 | 1.1465 | 2.1736 | 2.1706 | 1.000x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 3 | 2.1063 | 1.8627 | 3.9689 | 3.9623 | 1.826x | 1.826x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 3 | 0.7540 | 1.2594 | 2.0134 | 2.0143 | 0.926x | 0.926x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.1688 | 0.1699 | 7.8% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.5381 | 0.5137 | 24.8% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0720 | 0.0788 | 3.3% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0445 | 0.0475 | 2.0% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0112 | 0.0098 | 0.5% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 0.9826 | 0.9809 | 45.2% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.3564 | 0.4399 | 16.4% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.2375 | 0.2363 | 6.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.6166 | 0.6226 | 15.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0720 | 0.0788 | 1.8% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.2170 | 0.2932 | 5.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0096 | 0.0095 | 0.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 1.8893 | 1.9485 | 47.6% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.9270 | 0.9386 | 23.4% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.2175 | 0.2212 | 10.8% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0338 | 0.0333 | 1.7% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0284 | 0.0284 | 1.4% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0059 | 0.0057 | 0.3% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1679 | 0.1650 | 8.3% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.1041 | 0.1025 | 5.2% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.9263 | 0.9400 | 46.0% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0112 | 0.0098 | 0.6% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 0.5180 | 0.5182 | 25.7% |
