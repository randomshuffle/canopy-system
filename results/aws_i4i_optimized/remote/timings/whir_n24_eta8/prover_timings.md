# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 25 | - | 1 | 15.4022 | 12.3069 | 27.7091 | 27.7091 | 1.000x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 26 | - | 1 | 26.5120 | 18.8622 | 45.3742 | 45.3742 | 1.638x | 1.638x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 25 | 4 | 1 | 10.6647 | 14.6746 | 25.3394 | 25.3394 | 0.914x | 0.914x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 1.1393 | 1.1393 | 4.1% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 5.1674 | 5.1674 | 18.6% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.7344 | 0.7344 | 2.7% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 1.1346 | 1.1346 | 4.1% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.1510 | 0.1510 | 0.5% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 14.2676 | 14.2676 | 51.5% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 5.1147 | 5.1147 | 18.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 1.6231 | 1.6231 | 3.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 6.1825 | 6.1825 | 13.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.7344 | 0.7344 | 1.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 2.2399 | 2.2399 | 4.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.1768 | 0.1768 | 0.4% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 24.2721 | 24.2721 | 53.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 10.1454 | 10.1454 | 22.4% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 1.9654 | 1.9654 | 7.8% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.2388 | 0.2388 | 0.9% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.1463 | 0.1463 | 0.6% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.1199 | 0.1199 | 0.5% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 2.6487 | 2.6487 | 10.5% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 2.3079 | 2.3079 | 9.1% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 10.2501 | 10.2501 | 40.5% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.1510 | 0.1510 | 0.6% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 7.5111 | 7.5111 | 29.6% |
