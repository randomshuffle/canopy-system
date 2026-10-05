# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 23 | - | 10 | 0.3768 | 0.4707 | 0.8475 | 0.8220 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 10 | 0.7902 | 1.0434 | 1.8336 | 1.6480 | 2.163x | 2.163x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 10 | 0.5310 | 0.5083 | 1.0394 | 1.0031 | 1.226x | 1.226x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.1761 | 0.1451 | 20.8% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0393 | 0.0389 | 4.6% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0122 | 0.0118 | 1.4% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.3375 | 0.3187 | 39.8% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.1419 | 0.1320 | 16.7% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0020 | 0.0016 | 0.2% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1427 | 0.1351 | 16.8% |
| 0 | 1 | merkle_whir_batched_after_second | separately_measured_component_adjustment | -0.0042 | 0.0000 | -0.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.4913 | 0.4516 | 26.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0786 | 0.0708 | 4.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0221 | 0.0163 | 1.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.7116 | 0.5875 | 38.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.2366 | 0.2119 | 12.9% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0028 | 0.0016 | 0.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.2906 | 0.2628 | 15.9% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.1472 | 0.1474 | 14.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0366 | 0.0311 | 3.5% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0003 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0242 | 0.0177 | 2.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0169 | 0.0154 | 1.6% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1122 | 0.1094 | 10.8% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0249 | 0.0244 | 2.4% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.3238 | 0.2893 | 31.2% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0122 | 0.0118 | 1.2% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.3411 | 0.3343 | 32.8% |
