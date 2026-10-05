# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 25 | - | 3 | 5.7295 | 5.2151 | 10.9445 | 10.9842 | 1.000x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 26 | - | 3 | 9.3910 | 8.0210 | 17.4120 | 17.3537 | 1.591x | 1.591x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 25 | 4 | 3 | 3.4036 | 5.5726 | 8.9762 | 8.9804 | 0.820x | 0.820x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.7082 | 0.7034 | 6.5% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 2.2161 | 2.2523 | 20.2% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.3335 | 0.3351 | 3.0% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.5717 | 0.5706 | 5.2% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0339 | 0.0294 | 0.3% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 5.1577 | 5.1550 | 47.1% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 1.9233 | 1.9270 | 17.6% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 1.0905 | 1.0798 | 6.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 2.8134 | 2.8256 | 16.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.3335 | 0.3351 | 1.9% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 1.1391 | 1.1343 | 6.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0291 | 0.0291 | 0.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 8.2519 | 8.2403 | 47.4% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 3.7544 | 3.7365 | 21.6% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.9270 | 0.9254 | 10.3% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.1597 | 0.1593 | 1.8% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.1009 | 0.1017 | 1.1% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0213 | 0.0213 | 0.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.8147 | 0.8118 | 9.1% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.4793 | 0.4752 | 5.3% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 4.1321 | 4.1497 | 46.0% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0339 | 0.0294 | 0.4% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 2.3070 | 2.3118 | 25.7% |
