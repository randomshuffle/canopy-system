# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 23 | - | 3 | 1.3508 | 1.1093 | 2.4601 | 2.5045 | 1.000x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 3 | 2.2634 | 1.9926 | 4.2560 | 4.2432 | 1.730x | 1.730x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 3 | 0.7618 | 1.2249 | 1.9867 | 1.9961 | 0.808x | 0.808x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.1675 | 0.1681 | 6.8% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.4109 | 0.4296 | 16.7% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0686 | 0.0614 | 2.8% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.1529 | 0.1535 | 6.2% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0092 | 0.0092 | 0.4% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 1.1978 | 1.1926 | 48.7% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.4531 | 0.4488 | 18.4% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.2450 | 0.2438 | 5.8% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.7250 | 0.7156 | 17.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0686 | 0.0614 | 1.6% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.2962 | 0.2970 | 7.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0097 | 0.0094 | 0.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 1.9672 | 1.9691 | 46.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.9443 | 0.9460 | 22.2% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.2210 | 0.2194 | 11.1% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0313 | 0.0316 | 1.6% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0290 | 0.0297 | 1.5% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0058 | 0.0058 | 0.3% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1695 | 0.1656 | 8.5% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0937 | 0.0949 | 4.7% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.9007 | 0.9130 | 45.3% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0092 | 0.0092 | 0.5% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 0.5262 | 0.5213 | 26.5% |
