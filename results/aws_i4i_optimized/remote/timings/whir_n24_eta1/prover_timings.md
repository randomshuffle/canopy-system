# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 25 | - | 1 | 1.8973 | 1.9732 | 3.8705 | 3.8705 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 26 | - | 1 | 3.3225 | 3.2028 | 6.5253 | 6.5253 | 1.686x | 1.686x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 25 | 4 | 1 | 2.6223 | 2.2962 | 4.9184 | 4.9184 | 1.271x | 1.271x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.6378 | 0.6378 | 16.5% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.1464 | 0.1464 | 3.8% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0188 | 0.0188 | 0.5% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 1.7509 | 1.7509 | 45.2% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.6656 | 0.6656 | 17.2% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0026 | 0.0026 | 0.1% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.6486 | 0.6486 | 16.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.7797 | 0.7797 | 11.9% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.2855 | 0.2855 | 4.4% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0223 | 0.0223 | 0.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 3.0370 | 3.0370 | 46.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 1.1479 | 1.1479 | 17.6% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0027 | 0.0027 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 1.2502 | 1.2502 | 19.2% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.7193 | 0.7193 | 14.6% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.1089 | 0.1089 | 2.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0724 | 0.0724 | 1.5% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0153 | 0.0153 | 0.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.6047 | 0.6047 | 12.3% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.2701 | 0.2701 | 5.5% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 1.2879 | 1.2879 | 26.2% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0188 | 0.0188 | 0.4% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 1.8211 | 1.8211 | 37.0% |
