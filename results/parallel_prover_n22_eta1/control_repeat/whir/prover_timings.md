# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 23 | - | 10 | 0.3534 | 0.4439 | 0.7973 | 0.7972 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 10 | 0.7227 | 0.8983 | 1.6210 | 1.5238 | 2.033x | 2.033x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 10 | 0.5019 | 0.4824 | 0.9844 | 0.9798 | 1.235x | 1.235x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.1473 | 0.1600 | 18.5% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0374 | 0.0370 | 4.7% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0121 | 0.0121 | 1.5% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.3161 | 0.3156 | 39.6% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.1427 | 0.1326 | 17.9% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0026 | 0.0017 | 0.3% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1433 | 0.1315 | 18.0% |
| 0 | 1 | merkle_whir_batched_after_second | separately_measured_component_adjustment | -0.0041 | 0.0000 | -0.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.4023 | 0.4124 | 24.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0731 | 0.0713 | 4.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0161 | 0.0126 | 1.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.6496 | 0.5672 | 40.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.2183 | 0.2159 | 13.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0016 | 0.0016 | 0.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.2600 | 0.2562 | 16.0% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.1494 | 0.1471 | 15.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0310 | 0.0308 | 3.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0003 | 0.0004 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0182 | 0.0181 | 1.8% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0153 | 0.0153 | 1.6% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1081 | 0.1080 | 11.0% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0249 | 0.0249 | 2.5% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.2957 | 0.3012 | 30.0% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0121 | 0.0121 | 1.2% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.3293 | 0.3220 | 33.5% |
