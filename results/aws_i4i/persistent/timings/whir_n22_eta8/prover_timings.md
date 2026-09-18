# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 23 | - | 3 | 2.0398 | 2.4612 | 4.5010 | 4.5336 | 1.000x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 3 | 3.6026 | 3.2361 | 6.8388 | 6.8688 | 1.519x | 1.519x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 3 | 1.1317 | 2.7266 | 3.8584 | 3.8772 | 0.857x | 0.857x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.2239 | 0.2261 | 5.0% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 1.8049 | 1.8043 | 40.1% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.1009 | 0.0967 | 2.2% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.1012 | 0.0975 | 2.2% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0193 | 0.0192 | 0.4% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 1.9386 | 1.9494 | 43.1% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.3122 | 0.3133 | 6.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.3014 | 0.3009 | 4.4% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 2.0722 | 2.1262 | 30.3% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.1009 | 0.0967 | 1.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.1238 | 0.1243 | 1.8% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0198 | 0.0195 | 0.3% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 3.4789 | 3.4916 | 50.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.7418 | 0.7501 | 10.8% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 0.3598 | 0.3608 | 9.3% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0439 | 0.0444 | 1.1% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0003 | 0.0003 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0340 | 0.0339 | 0.9% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0109 | 0.0107 | 0.3% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 0.2460 | 0.2464 | 6.4% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.2349 | 0.2331 | 6.1% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 2.1123 | 2.1011 | 54.7% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0193 | 0.0192 | 0.5% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 0.7970 | 0.7952 | 20.7% |
