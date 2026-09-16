# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 0.4556 | 0.5022 | 0.9578 | 0.9129 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 0.6381 | 0.6882 | 1.3262 | 1.2974 | 1.385x | 1.385x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 0.6308 | 0.5501 | 1.1809 | 1.2102 | 1.233x | 1.233x |
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 0.9112 | 0.8932 | 1.8044 | 1.7609 | 1.000x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 1.2761 | 1.1908 | 2.4669 | 2.4188 | 1.367x | 1.367x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 1.2344 | 0.9073 | 2.1417 | 2.1848 | 1.187x | 1.187x |
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 1.8224 | 1.6668 | 3.4893 | 3.3947 | 1.000x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 2.5522 | 2.1946 | 4.7468 | 4.6709 | 1.360x | 1.360x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 2.3990 | 1.8167 | 4.2157 | 4.0680 | 1.208x | 1.208x |
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 3.6448 | 3.3966 | 7.0414 | 6.8096 | 1.000x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 5.1044 | 4.3858 | 9.4903 | 9.3054 | 1.348x | 1.348x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 5.9815 | 4.0084 | 9.9898 | 9.6710 | 1.419x | 1.419x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.2135 | 0.1406 | 22.3% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0394 | 0.0392 | 4.1% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0131 | 0.0127 | 1.4% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.4162 | 0.3551 | 43.5% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.1434 | 0.1328 | 15.0% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0017 | 0.0016 | 0.2% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1304 | 0.1290 | 13.6% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.1880 | 0.1832 | 14.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0746 | 0.0732 | 5.6% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0122 | 0.0118 | 0.9% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.5635 | 0.5715 | 42.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.2207 | 0.2082 | 16.6% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0017 | 0.0016 | 0.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.2655 | 0.2519 | 20.0% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.2403 | 0.2345 | 20.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0328 | 0.0309 | 2.8% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0003 | 0.0003 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0359 | 0.0353 | 3.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0165 | 0.0162 | 1.4% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.2209 | 0.2113 | 18.7% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0244 | 0.0242 | 2.1% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.2720 | 0.2105 | 23.0% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0131 | 0.0127 | 1.1% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.3247 | 0.3223 | 27.5% |
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.1540 | 0.1499 | 8.5% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.4270 | 0.2813 | 23.7% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0251 | 0.0254 | 1.4% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0788 | 0.0784 | 4.4% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0262 | 0.0253 | 1.5% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 0.8324 | 0.7102 | 46.1% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.2609 | 0.2581 | 14.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.2342 | 0.2370 | 9.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.3760 | 0.3664 | 15.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0251 | 0.0254 | 1.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.1492 | 0.1463 | 6.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0244 | 0.0236 | 1.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 1.1269 | 1.1430 | 45.7% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.5311 | 0.5038 | 21.5% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.2857 | 0.2759 | 13.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0632 | 0.0605 | 3.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0004 | 0.0004 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0715 | 0.0692 | 3.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0331 | 0.0304 | 1.5% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.4319 | 0.4242 | 20.2% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0509 | 0.0483 | 2.4% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.5441 | 0.4209 | 25.4% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0262 | 0.0253 | 1.2% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.6347 | 0.6394 | 29.6% |
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.1894 | 0.1801 | 5.4% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.8540 | 0.5625 | 24.5% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0493 | 0.0478 | 1.4% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.1577 | 0.1569 | 4.5% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0524 | 0.0507 | 1.5% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 1.6647 | 1.4204 | 47.7% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.5217 | 0.5161 | 15.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.2825 | 0.2693 | 6.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.7519 | 0.7327 | 15.8% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0493 | 0.0478 | 1.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.2984 | 0.2926 | 6.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0487 | 0.0472 | 1.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 2.2538 | 2.2861 | 47.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 1.0621 | 1.0077 | 22.4% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.5159 | 0.4444 | 12.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.1222 | 0.1209 | 2.9% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0005 | 0.0005 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.1339 | 0.1291 | 3.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0655 | 0.0612 | 1.6% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.8797 | 0.8059 | 20.9% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.1597 | 0.1599 | 3.8% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 1.0882 | 0.8419 | 25.8% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0524 | 0.0507 | 1.2% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 1.1977 | 1.1411 | 28.4% |
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.3671 | 0.3752 | 5.2% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 1.7080 | 1.1251 | 24.3% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.1732 | 0.1657 | 2.5% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.3153 | 0.3138 | 4.5% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.1049 | 0.1013 | 1.5% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 3.3295 | 2.8408 | 47.3% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 1.0435 | 1.0322 | 14.8% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.4871 | 0.4420 | 5.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 1.5038 | 1.4655 | 15.8% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.1732 | 0.1657 | 1.8% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.5967 | 0.5852 | 6.3% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0975 | 0.0944 | 1.0% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 4.5077 | 4.5721 | 47.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 2.1243 | 2.0153 | 22.4% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 1.0963 | 0.9969 | 11.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.2373 | 0.2296 | 2.4% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0041 | 0.0047 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.3108 | 0.2967 | 3.1% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.1320 | 0.1229 | 1.3% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 2.8848 | 3.1233 | 28.9% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.6267 | 0.5820 | 6.3% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 2.1764 | 1.6837 | 21.8% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.1049 | 0.1013 | 1.0% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 2.4164 | 2.4008 | 24.2% |
