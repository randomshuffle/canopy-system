# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0272 | 0.0274 | 0.0546 | 0.0454 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.0337 | 0.0904 | 0.1241 | 0.0761 | 2.273x | 2.273x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.0346 | 0.0456 | 0.0802 | 0.0631 | 1.468x | 1.468x |
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0545 | 0.0487 | 0.1031 | 0.0852 | 1.000x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.0674 | 0.1680 | 0.2354 | 0.1404 | 2.282x | 2.282x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.0707 | 0.0432 | 0.1139 | 0.1058 | 1.104x | 1.104x |
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.1089 | 0.0902 | 0.1991 | 0.1613 | 1.000x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.1349 | 0.3231 | 0.4580 | 0.2659 | 2.300x | 2.300x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.1331 | 0.0973 | 0.2304 | 0.1978 | 1.157x | 1.157x |
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.2178 | 0.1756 | 0.3934 | 0.3184 | 1.000x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.2698 | 0.6476 | 0.9173 | 0.5679 | 2.332x | 2.332x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.2259 | 0.1661 | 0.3921 | 0.3888 | 0.997x | 0.997x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0099 | 0.0098 | 18.1% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0045 | 0.0027 | 8.2% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0009 | 0.0008 | 1.6% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0228 | 0.0154 | 41.7% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.0088 | 0.0088 | 16.1% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0002 | 0.0002 | 0.4% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0076 | 0.0074 | 13.9% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0603 | 0.0141 | 48.6% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0050 | 0.0049 | 4.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0012 | 0.0009 | 1.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.0287 | 0.0289 | 23.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.0155 | 0.0153 | 12.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0002 | 0.0002 | 0.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0132 | 0.0132 | 10.6% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.0331 | 0.0172 | 41.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0025 | 0.0020 | 3.1% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0001 | 0.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0029 | 0.0028 | 3.6% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0011 | 0.0010 | 1.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0113 | 0.0104 | 14.0% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0021 | 0.0022 | 2.6% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0093 | 0.0107 | 11.6% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0009 | 0.0008 | 1.1% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.0169 | 0.0158 | 21.1% |
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0104 | 0.0107 | 10.1% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0198 | 0.0196 | 19.2% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0016 | 0.0015 | 1.5% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0089 | 0.0055 | 8.6% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0017 | 0.0017 | 1.7% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0455 | 0.0308 | 44.1% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0152 | 0.0147 | 14.7% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0169 | 0.0173 | 7.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.1207 | 0.0282 | 51.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0016 | 0.0015 | 0.7% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0100 | 0.0098 | 4.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0024 | 0.0018 | 1.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.0574 | 0.0579 | 24.4% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0264 | 0.0265 | 11.2% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.0197 | 0.0197 | 17.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0040 | 0.0037 | 3.5% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0003 | 0.0003 | 0.2% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0049 | 0.0046 | 4.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0025 | 0.0021 | 2.2% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0269 | 0.0218 | 23.6% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0030 | 0.0030 | 2.6% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0186 | 0.0214 | 16.3% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0017 | 0.0017 | 1.5% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.0323 | 0.0298 | 28.4% |
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0125 | 0.0127 | 6.3% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0396 | 0.0391 | 19.9% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0042 | 0.0032 | 2.1% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0178 | 0.0109 | 9.0% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0034 | 0.0034 | 1.7% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0911 | 0.0617 | 45.7% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0304 | 0.0295 | 15.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0199 | 0.0196 | 4.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.2414 | 0.0565 | 52.7% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0042 | 0.0032 | 0.9% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0201 | 0.0196 | 4.4% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0049 | 0.0037 | 1.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.1148 | 0.1157 | 25.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0527 | 0.0530 | 11.5% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.0466 | 0.0272 | 20.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0077 | 0.0077 | 3.4% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0003 | 0.0003 | 0.1% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0094 | 0.0086 | 4.1% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0042 | 0.0041 | 1.8% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0528 | 0.0409 | 22.9% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0097 | 0.0093 | 4.2% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0372 | 0.0428 | 16.2% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0034 | 0.0034 | 1.5% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 0.0589 | 0.0580 | 25.6% |
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0188 | 0.0183 | 4.8% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0792 | 0.0782 | 20.1% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0099 | 0.0096 | 2.5% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0357 | 0.0218 | 9.1% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0069 | 0.0067 | 1.8% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 0.1821 | 0.1234 | 46.3% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0608 | 0.0590 | 15.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0397 | 0.0265 | 4.3% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.4828 | 0.1129 | 52.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0099 | 0.0096 | 1.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0401 | 0.0393 | 4.4% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0098 | 0.0074 | 1.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.2296 | 0.2315 | 25.0% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.1055 | 0.1059 | 11.5% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 0.0486 | 0.0482 | 12.4% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0144 | 0.0140 | 3.7% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.1% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0168 | 0.0167 | 4.3% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0076 | 0.0076 | 1.9% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0759 | 0.0728 | 19.4% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0359 | 0.0338 | 9.2% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0745 | 0.0855 | 19.0% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0069 | 0.0067 | 1.8% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 0.1113 | 0.1127 | 28.4% |
