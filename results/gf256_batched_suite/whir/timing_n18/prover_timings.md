# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0201 | 0.0316 | 0.0516 | 0.0529 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.0407 | 0.0478 | 0.0885 | 0.0752 | 1.714x | 1.714x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.0324 | 0.0356 | 0.0680 | 0.0642 | 1.316x | 1.316x |
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0401 | 0.0544 | 0.0945 | 0.0906 | 1.000x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.0814 | 0.0825 | 0.1639 | 0.1374 | 1.734x | 1.734x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.0615 | 0.0540 | 0.1155 | 0.1079 | 1.222x | 1.222x |
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0803 | 0.1005 | 0.1807 | 0.1760 | 1.000x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.1628 | 0.1471 | 0.3100 | 0.2574 | 1.715x | 1.715x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.1143 | 0.0994 | 0.2137 | 0.1913 | 1.182x | 1.182x |
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.1605 | 0.1998 | 0.3603 | 0.3526 | 1.000x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.3256 | 0.2873 | 0.6129 | 0.5070 | 1.701x | 1.701x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.2212 | 0.2173 | 0.4385 | 0.4230 | 1.217x | 1.217x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0111 | 0.0091 | 21.5% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0030 | 0.0026 | 5.8% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0009 | 0.0008 | 1.8% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0171 | 0.0156 | 33.1% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.0101 | 0.0094 | 19.5% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0002 | 0.0002 | 0.4% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0093 | 0.0074 | 18.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0168 | 0.0121 | 19.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0072 | 0.0058 | 8.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0010 | 0.0009 | 1.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.0335 | 0.0265 | 37.9% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.0162 | 0.0161 | 18.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0002 | 0.0002 | 0.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0135 | 0.0133 | 15.3% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.0178 | 0.0180 | 26.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0020 | 0.0020 | 2.9% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.1% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0027 | 0.0027 | 4.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0011 | 0.0011 | 1.6% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0109 | 0.0111 | 16.1% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0021 | 0.0020 | 3.0% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0147 | 0.0105 | 21.6% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0009 | 0.0008 | 1.3% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.0157 | 0.0155 | 23.0% |
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0102 | 0.0104 | 10.7% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0222 | 0.0182 | 23.5% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0017 | 0.0015 | 1.8% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0059 | 0.0053 | 6.3% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0018 | 0.0017 | 1.9% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0342 | 0.0311 | 36.2% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0186 | 0.0147 | 19.6% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0181 | 0.0176 | 11.1% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0337 | 0.0241 | 20.6% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0017 | 0.0015 | 1.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0144 | 0.0116 | 8.8% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0020 | 0.0019 | 1.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.0670 | 0.0529 | 40.9% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0270 | 0.0266 | 16.5% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.0196 | 0.0191 | 16.9% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0040 | 0.0037 | 3.5% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0001 | 0.1% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0045 | 0.0044 | 3.9% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0020 | 0.0019 | 1.7% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0209 | 0.0206 | 18.1% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0031 | 0.0028 | 2.7% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0294 | 0.0211 | 25.4% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0018 | 0.0017 | 1.6% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.0301 | 0.0290 | 26.1% |
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0123 | 0.0116 | 6.8% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0443 | 0.0365 | 24.5% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0031 | 0.0031 | 1.7% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0119 | 0.0106 | 6.6% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0036 | 0.0034 | 2.0% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0684 | 0.0623 | 37.8% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0371 | 0.0294 | 20.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0187 | 0.0187 | 6.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0674 | 0.0483 | 21.7% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0031 | 0.0031 | 1.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0288 | 0.0232 | 9.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0040 | 0.0038 | 1.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.1340 | 0.1058 | 43.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0540 | 0.0531 | 17.4% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.0272 | 0.0261 | 12.7% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0071 | 0.0069 | 3.3% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0003 | 0.0003 | 0.1% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0085 | 0.0081 | 4.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0040 | 0.0038 | 1.9% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0390 | 0.0381 | 18.2% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0095 | 0.0093 | 4.4% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0587 | 0.0421 | 27.5% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0036 | 0.0034 | 1.7% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 0.0557 | 0.0545 | 26.1% |
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0204 | 0.0184 | 5.7% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0887 | 0.0729 | 24.6% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0092 | 0.0092 | 2.6% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0238 | 0.0211 | 6.6% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0072 | 0.0067 | 2.0% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 0.1367 | 0.1245 | 37.9% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0743 | 0.0588 | 20.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0273 | 0.0274 | 4.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.1348 | 0.0965 | 22.0% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0092 | 0.0092 | 1.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0576 | 0.0465 | 9.4% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0079 | 0.0075 | 1.3% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.2680 | 0.2116 | 43.7% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.1080 | 0.1063 | 17.6% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 0.0592 | 0.0512 | 13.5% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0143 | 0.0137 | 3.3% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0003 | 0.0002 | 0.1% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0161 | 0.0161 | 3.7% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0077 | 0.0075 | 1.8% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0748 | 0.0735 | 17.1% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0331 | 0.0326 | 7.6% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.1174 | 0.0843 | 26.8% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0072 | 0.0067 | 1.7% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 0.1082 | 0.1094 | 24.7% |
