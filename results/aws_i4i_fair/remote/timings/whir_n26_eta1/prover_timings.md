# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. All three schemes execute concurrent initial encodings and Merkle commitments at t and report batch wall times. The half-rate baseline and Exp3 share the exact measured base RS batch. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales single-input opening-front work by t for all three schemes, and executes batched tails at t. It does not serialize a complete Exp3 proof. See timing_method.json for the scheduling policy.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 27 | - | 1 | 7.8481 | 8.1283 | 15.9765 | 15.9765 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 28 | - | 1 | 13.8390 | 13.2468 | 27.0858 | 27.0858 | 1.695x | 1.695x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 27 | 4 | 1 | 10.6872 | 9.3868 | 20.0740 | 20.0740 | 1.256x | 1.256x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 2.4556 | 2.4556 | 15.4% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.5583 | 0.5583 | 3.5% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0741 | 0.0741 | 0.5% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 7.2898 | 7.2898 | 45.6% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 2.7963 | 2.7963 | 17.5% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0095 | 0.0095 | 0.1% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 2.7928 | 2.7928 | 17.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 3.0531 | 3.0531 | 11.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 1.1100 | 1.1100 | 4.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0740 | 0.0740 | 0.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 12.7290 | 12.7290 | 47.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 4.8805 | 4.8805 | 18.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0096 | 0.0096 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 5.2296 | 5.2296 | 19.3% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 3.0338 | 3.0338 | 15.1% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.4339 | 0.4339 | 2.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.2817 | 0.2817 | 1.4% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0603 | 0.0603 | 0.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 2.6214 | 2.6214 | 13.1% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 1.0631 | 1.0631 | 5.3% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 5.2158 | 5.2158 | 26.0% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0741 | 0.0741 | 0.4% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 7.2898 | 7.2898 | 36.3% |
