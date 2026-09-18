# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 27 | - | 3 | 6.2222 | 6.6405 | 12.8627 | 12.4725 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 28 | - | 3 | 10.0697 | 10.6849 | 20.7547 | 20.7856 | 1.614x | 1.614x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 27 | 4 | 3 | 8.0464 | 7.8931 | 15.9395 | 15.7944 | 1.239x | 1.239x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 2.2617 | 2.3066 | 17.6% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.5782 | 0.5753 | 4.5% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0377 | 0.0263 | 0.3% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 5.6441 | 5.2636 | 43.9% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 2.3389 | 2.3341 | 18.2% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0041 | 0.0040 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 1.9982 | 2.0019 | 15.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 2.8039 | 2.8297 | 13.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 1.1336 | 1.1359 | 5.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0560 | 0.0573 | 0.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 8.9362 | 8.9670 | 43.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 3.9463 | 3.9455 | 19.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0041 | 0.0040 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 3.8747 | 3.8881 | 18.7% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 2.4968 | 2.4921 | 15.7% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.4356 | 0.4348 | 2.7% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0001 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.2873 | 0.2878 | 1.8% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0216 | 0.0214 | 0.1% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 1.8278 | 1.8212 | 11.5% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 1.1078 | 1.1069 | 6.9% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 4.2507 | 4.2665 | 26.7% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0377 | 0.0263 | 0.2% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 5.4740 | 5.2706 | 34.3% |
