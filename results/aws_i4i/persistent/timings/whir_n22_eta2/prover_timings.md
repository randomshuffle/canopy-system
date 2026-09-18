# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 23 | - | 3 | 0.6751 | 0.6142 | 1.2893 | 1.2935 | 1.000x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 3 | 1.1313 | 1.0966 | 2.2279 | 2.2223 | 1.728x | 1.728x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 3 | 0.5442 | 0.6474 | 1.1916 | 1.1915 | 0.924x | 0.924x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.1380 | 0.1381 | 10.7% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.1924 | 0.1923 | 14.9% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0553 | 0.0540 | 4.3% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0767 | 0.0770 | 6.0% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0047 | 0.0047 | 0.4% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 0.5983 | 0.6007 | 46.4% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.2238 | 0.2234 | 17.4% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.2150 | 0.2156 | 9.7% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.3473 | 0.3516 | 15.6% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0553 | 0.0540 | 2.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.1455 | 0.1458 | 6.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0047 | 0.0047 | 0.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.9858 | 0.9758 | 44.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.4743 | 0.4753 | 21.3% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.1616 | 0.1618 | 13.6% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0269 | 0.0249 | 2.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0232 | 0.0221 | 1.9% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0033 | 0.0034 | 0.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1184 | 0.1206 | 9.9% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0715 | 0.0676 | 6.0% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.4095 | 0.4067 | 34.4% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0047 | 0.0047 | 0.4% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.3724 | 0.3700 | 31.2% |
