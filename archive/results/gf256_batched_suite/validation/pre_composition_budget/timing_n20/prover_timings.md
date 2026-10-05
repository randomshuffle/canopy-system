# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 21 | - | 5 | 0.0804 | 0.1121 | 0.1925 | 0.1867 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 22 | - | 5 | 0.1401 | 0.1717 | 0.3118 | 0.3062 | 1.619x | 1.619x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 21 | 4 | 5 | 0.1610 | 0.1242 | 0.2852 | 0.2551 | 1.482x | 1.482x |
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 21 | - | 5 | 0.1609 | 0.1921 | 0.3530 | 0.3439 | 1.000x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 22 | - | 5 | 0.2801 | 0.2922 | 0.5723 | 0.5729 | 1.621x | 1.621x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 21 | 4 | 5 | 0.2587 | 0.1779 | 0.4366 | 0.4462 | 1.237x | 1.237x |
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 21 | - | 5 | 0.3217 | 0.3500 | 0.6717 | 0.6488 | 1.000x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 22 | - | 5 | 0.5602 | 0.5315 | 1.0917 | 1.0811 | 1.625x | 1.625x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 21 | 4 | 5 | 0.4921 | 0.3283 | 0.8205 | 0.8238 | 1.221x | 1.221x |
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 21 | - | 5 | 0.6434 | 0.6911 | 1.3345 | 1.2939 | 1.000x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 22 | - | 5 | 1.1204 | 1.1029 | 2.2233 | 2.2316 | 1.666x | 1.666x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 21 | 4 | 5 | 0.9923 | 0.7385 | 1.7308 | 1.7763 | 1.297x | 1.297x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0399 | 0.0396 | 20.7% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0110 | 0.0102 | 5.7% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0032 | 0.0030 | 1.6% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0694 | 0.0670 | 36.0% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.0394 | 0.0385 | 20.5% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0005 | 0.0005 | 0.2% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0291 | 0.0284 | 15.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0543 | 0.0537 | 17.4% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0183 | 0.0182 | 5.9% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0032 | 0.0033 | 1.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.1218 | 0.1195 | 39.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.0582 | 0.0559 | 18.7% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0004 | 0.0004 | 0.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0556 | 0.0547 | 17.8% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.0614 | 0.0619 | 21.5% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0080 | 0.0077 | 2.8% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0012 | 0.0011 | 0.4% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0321 | 0.0101 | 11.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0038 | 0.0039 | 1.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0479 | 0.0456 | 16.8% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0129 | 0.0069 | 4.5% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0457 | 0.0472 | 16.0% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0032 | 0.0030 | 1.1% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.0691 | 0.0682 | 24.2% |
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0409 | 0.0406 | 11.6% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0799 | 0.0793 | 22.6% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0068 | 0.0069 | 1.9% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0221 | 0.0205 | 6.3% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0063 | 0.0060 | 1.8% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 0.1388 | 0.1340 | 39.3% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0583 | 0.0569 | 16.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0593 | 0.0585 | 10.4% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.1086 | 0.1073 | 19.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0068 | 0.0069 | 1.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0365 | 0.0364 | 6.4% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0064 | 0.0065 | 1.1% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.2436 | 0.2391 | 42.6% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.1112 | 0.1094 | 19.4% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.0676 | 0.0661 | 15.5% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0143 | 0.0143 | 3.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0007 | 0.0007 | 0.2% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0180 | 0.0180 | 4.1% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0075 | 0.0075 | 1.7% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0881 | 0.0887 | 20.2% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0120 | 0.0117 | 2.7% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0913 | 0.0944 | 20.9% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0063 | 0.0060 | 1.4% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.1308 | 0.1281 | 30.0% |
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0489 | 0.0489 | 7.3% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.1597 | 0.1586 | 23.8% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0123 | 0.0123 | 1.8% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0441 | 0.0409 | 6.6% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0126 | 0.0120 | 1.9% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 0.2776 | 0.2679 | 41.3% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1165 | 0.1138 | 17.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0669 | 0.0661 | 6.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.2173 | 0.2147 | 19.9% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0123 | 0.0123 | 1.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0730 | 0.0727 | 6.7% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0128 | 0.0131 | 1.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.4872 | 0.4781 | 44.6% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.2223 | 0.2187 | 20.4% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.0926 | 0.0927 | 11.3% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0279 | 0.0281 | 3.4% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0009 | 0.0010 | 0.1% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0345 | 0.0344 | 4.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0151 | 0.0151 | 1.8% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1721 | 0.1725 | 21.0% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0396 | 0.0388 | 4.8% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.1826 | 0.1888 | 22.3% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0126 | 0.0120 | 1.5% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 0.2424 | 0.2375 | 29.5% |
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0716 | 0.0718 | 5.4% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.3195 | 0.3171 | 23.9% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0417 | 0.0404 | 3.1% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0883 | 0.0818 | 6.6% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0253 | 0.0241 | 1.9% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 0.5552 | 0.5359 | 41.6% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.2330 | 0.2276 | 17.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.1564 | 0.0943 | 7.0% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.4345 | 0.4293 | 19.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0417 | 0.0404 | 1.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.1460 | 0.1454 | 6.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0256 | 0.0262 | 1.2% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.9744 | 0.9563 | 43.8% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.4447 | 0.4374 | 20.0% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 0.1997 | 0.2011 | 11.5% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0555 | 0.0560 | 3.2% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0014 | 0.0012 | 0.1% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0838 | 0.0784 | 4.8% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0307 | 0.0301 | 1.8% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 0.3429 | 0.3384 | 19.8% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.1470 | 0.1467 | 8.5% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.3653 | 0.3776 | 21.1% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0253 | 0.0241 | 1.5% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 0.4795 | 0.4821 | 27.7% |
