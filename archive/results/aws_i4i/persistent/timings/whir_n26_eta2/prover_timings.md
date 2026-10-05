# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 27 | - | 3 | 12.1546 | 12.4072 | 24.5618 | 24.1720 | 1.000x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 28 | - | 3 | 20.0530 | 18.7254 | 38.7783 | 39.1655 | 1.579x | 1.579x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 27 | 4 | 3 | 10.1351 | 12.9313 | 23.0664 | 23.0288 | 0.939x | 0.939x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 2.5396 | 2.5217 | 10.3% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 4.6638 | 4.7353 | 19.0% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 1.1215 | 1.1211 | 4.6% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 1.1519 | 1.1513 | 4.7% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0741 | 0.0532 | 0.3% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 11.0026 | 10.7502 | 44.8% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 4.0083 | 4.0066 | 16.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 4.1337 | 4.1502 | 10.7% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 5.5635 | 5.5508 | 14.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 1.1215 | 1.1211 | 2.9% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 2.3062 | 2.3054 | 5.9% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0753 | 0.0533 | 0.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 17.7468 | 17.9253 | 45.8% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 7.8315 | 7.8446 | 20.2% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 2.9134 | 2.9183 | 12.6% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.5250 | 0.5315 | 2.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.3273 | 0.3280 | 1.4% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0417 | 0.0417 | 0.2% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 2.4537 | 2.4589 | 10.6% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 1.3119 | 1.2937 | 5.7% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 8.6317 | 8.6445 | 37.4% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0741 | 0.0532 | 0.3% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 6.7873 | 6.7793 | 29.4% |
