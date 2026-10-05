# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 27 | - | 3 | 24.3511 | 21.6544 | 46.0055 | 45.6617 | 1.000x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 28 | - | 3 | 40.7809 | 32.4352 | 73.2161 | 72.0072 | 1.591x | 1.591x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 27 | 4 | 3 | 14.5258 | 23.1872 | 37.7130 | 37.8689 | 0.820x | 0.820x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 2.9154 | 2.9200 | 6.3% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 9.3254 | 9.2554 | 20.3% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 1.2264 | 1.2266 | 2.7% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 2.3213 | 2.3293 | 5.0% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.1138 | 0.1110 | 0.2% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 22.0298 | 21.8699 | 47.9% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 8.0735 | 8.0756 | 17.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 4.5363 | 4.5505 | 6.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 11.0053 | 11.0158 | 15.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 1.2264 | 1.2266 | 1.7% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 4.5517 | 4.5565 | 6.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.1447 | 0.1067 | 0.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 36.2292 | 35.1528 | 49.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 15.5226 | 15.6181 | 21.2% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 3.7941 | 3.7850 | 10.1% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.6345 | 0.6359 | 1.7% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0004 | 0.0004 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.4006 | 0.4016 | 1.1% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0819 | 0.0818 | 0.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 3.4348 | 3.4146 | 9.1% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 1.8724 | 1.8577 | 5.0% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 17.4065 | 17.3944 | 46.2% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.1138 | 0.1110 | 0.3% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 9.9740 | 9.9004 | 26.4% |
