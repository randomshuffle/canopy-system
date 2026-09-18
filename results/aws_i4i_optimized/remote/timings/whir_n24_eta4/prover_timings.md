# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 25 | - | 1 | 7.5111 | 6.3849 | 13.8961 | 13.8961 | 1.000x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 26 | - | 1 | 13.2044 | 9.9001 | 23.1045 | 23.1045 | 1.663x | 1.663x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 25 | 4 | 1 | 5.9543 | 7.0383 | 12.9926 | 12.9926 | 0.935x | 0.935x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.8391 | 0.8391 | 6.0% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 2.5412 | 2.5412 | 18.3% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.3553 | 0.3553 | 2.6% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.5723 | 0.5723 | 4.1% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0751 | 0.0751 | 0.5% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 6.9388 | 6.9388 | 49.9% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 2.5743 | 2.5743 | 18.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 1.3264 | 1.3264 | 5.7% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 3.0917 | 3.0917 | 13.4% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.3553 | 0.3553 | 1.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 1.1366 | 1.1366 | 4.9% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0749 | 0.0749 | 0.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 12.0678 | 12.0678 | 52.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 5.0517 | 5.0517 | 21.9% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 1.1398 | 1.1398 | 8.8% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.1701 | 0.1701 | 1.3% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.1039 | 0.1039 | 0.8% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0603 | 0.0603 | 0.5% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 1.4685 | 1.4685 | 11.3% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.7223 | 0.7223 | 5.6% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 5.1009 | 5.1009 | 39.3% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0751 | 0.0751 | 0.6% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 4.1517 | 4.1517 | 32.0% |
