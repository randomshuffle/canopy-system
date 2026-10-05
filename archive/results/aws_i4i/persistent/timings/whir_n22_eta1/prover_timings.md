# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 23 | - | 3 | 0.3400 | 0.3491 | 0.6891 | 0.6777 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 3 | 0.5701 | 0.5936 | 1.1638 | 1.1812 | 1.689x | 1.689x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 3 | 0.4415 | 0.4186 | 0.8601 | 0.8463 | 1.248x | 1.248x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.1144 | 0.1150 | 16.6% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0400 | 0.0394 | 5.8% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0023 | 0.0023 | 0.3% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.3000 | 0.2994 | 43.5% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.1255 | 0.1261 | 18.2% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0009 | 0.0009 | 0.1% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1060 | 0.1089 | 15.4% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.1539 | 0.1780 | 13.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0758 | 0.0748 | 6.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0024 | 0.0024 | 0.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.4944 | 0.4946 | 42.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.1998 | 0.1989 | 17.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0009 | 0.0009 | 0.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.2367 | 0.2367 | 20.3% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.1408 | 0.1417 | 16.4% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0231 | 0.0233 | 2.7% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0204 | 0.0205 | 2.4% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0021 | 0.0021 | 0.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0950 | 0.0943 | 11.0% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0538 | 0.0539 | 6.3% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.2216 | 0.2067 | 25.8% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0023 | 0.0023 | 0.3% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.3010 | 0.3014 | 35.0% |
