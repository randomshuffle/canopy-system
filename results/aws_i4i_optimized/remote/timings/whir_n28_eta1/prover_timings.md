# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 29 | - | 1 | 36.3052 | 32.8328 | 69.1380 | 69.1380 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 30 | - | 1 | 58.8992 | 55.0610 | 113.9602 | 113.9602 | 1.648x | 1.648x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 29 | 4 | 1 | 48.1205 | 38.2164 | 86.3369 | 86.3369 | 1.249x | 1.249x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 9.7733 | 9.7733 | 14.1% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 2.2249 | 2.2249 | 3.2% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.3507 | 0.3507 | 0.5% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 34.0803 | 34.0803 | 49.3% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 10.9729 | 10.9729 | 15.9% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0458 | 0.0458 | 0.1% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 11.6901 | 11.6901 | 16.9% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 11.9211 | 11.9211 | 10.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 4.5456 | 4.5456 | 4.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.2952 | 0.2952 | 0.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 54.3536 | 54.3536 | 47.7% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 20.3508 | 20.3508 | 17.9% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0372 | 0.0372 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 22.4568 | 22.4568 | 19.7% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 11.9170 | 11.9170 | 13.8% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 1.7116 | 1.7116 | 2.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 1.1192 | 1.1192 | 1.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.2395 | 0.2395 | 0.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 10.7487 | 10.7487 | 12.4% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 4.4615 | 4.4615 | 5.2% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 21.4870 | 21.4870 | 24.9% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.3507 | 0.3507 | 0.4% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 34.3016 | 34.3016 | 39.7% |
