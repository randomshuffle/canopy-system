# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 21 | - | 5 | 0.0960 | 0.1190 | 0.2149 | 0.2079 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 22 | - | 5 | 0.1561 | 0.1890 | 0.3452 | 0.3371 | 1.606x | 1.606x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 21 | 4 | 5 | 0.1423 | 0.1299 | 0.2723 | 0.2700 | 1.267x | 1.267x |
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 21 | - | 5 | 0.1919 | 0.2036 | 0.3955 | 0.3687 | 1.000x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 22 | - | 5 | 0.3122 | 0.3136 | 0.6259 | 0.6265 | 1.582x | 1.582x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 21 | 4 | 5 | 0.2684 | 0.2262 | 0.4946 | 0.4774 | 1.251x | 1.251x |
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 21 | - | 5 | 0.3838 | 0.3680 | 0.7518 | 0.6863 | 1.000x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 22 | - | 5 | 0.6245 | 0.5172 | 1.1417 | 1.1113 | 1.519x | 1.519x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 21 | 4 | 5 | 0.5753 | 0.4017 | 0.9770 | 0.9558 | 1.299x | 1.299x |
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 21 | - | 5 | 0.7676 | 0.7386 | 1.5063 | 1.3589 | 1.000x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 22 | - | 5 | 1.2490 | 1.0050 | 2.2540 | 2.2068 | 1.496x | 1.496x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 21 | 4 | 5 | 1.2384 | 0.8777 | 2.1162 | 2.0967 | 1.405x | 1.405x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0394 | 0.0431 | 18.3% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0122 | 0.0110 | 5.7% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0032 | 0.0031 | 1.5% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0837 | 0.0746 | 39.0% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.0425 | 0.0409 | 19.8% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0005 | 0.0005 | 0.3% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0333 | 0.0307 | 15.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0447 | 0.0482 | 12.9% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0199 | 0.0188 | 5.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0047 | 0.0036 | 1.4% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.1362 | 0.1294 | 39.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.0827 | 0.0615 | 24.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0014 | 0.0005 | 0.4% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0757 | 0.0610 | 21.9% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.0625 | 0.0614 | 23.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0081 | 0.0079 | 3.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0005 | 0.0003 | 0.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0108 | 0.0105 | 4.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0040 | 0.0040 | 1.5% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0493 | 0.0485 | 18.1% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0066 | 0.0064 | 2.4% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0572 | 0.0533 | 21.0% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0032 | 0.0031 | 1.2% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.0701 | 0.0715 | 25.7% |
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0455 | 0.0391 | 11.5% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0789 | 0.0861 | 19.9% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0064 | 0.0061 | 1.6% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0245 | 0.0221 | 6.2% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0064 | 0.0062 | 1.6% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 0.1674 | 0.1492 | 42.3% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0665 | 0.0614 | 16.8% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0973 | 0.0696 | 15.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0894 | 0.0964 | 14.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0064 | 0.0061 | 1.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0397 | 0.0375 | 6.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0094 | 0.0072 | 1.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.2725 | 0.2588 | 43.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.1513 | 0.1220 | 24.2% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.0916 | 0.0782 | 18.5% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0151 | 0.0155 | 3.1% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0006 | 0.0004 | 0.1% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0197 | 0.0194 | 4.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0086 | 0.0080 | 1.7% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0941 | 0.0946 | 19.0% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0133 | 0.0124 | 2.7% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.1144 | 0.1066 | 23.1% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0064 | 0.0062 | 1.3% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.1309 | 0.1304 | 26.5% |
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0515 | 0.0528 | 6.9% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.1577 | 0.1722 | 21.0% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0130 | 0.0130 | 1.7% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0490 | 0.0441 | 6.5% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0128 | 0.0124 | 1.7% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 0.3349 | 0.2984 | 44.5% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1330 | 0.1229 | 17.7% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0842 | 0.0836 | 7.4% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.1788 | 0.1928 | 15.7% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0130 | 0.0130 | 1.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0795 | 0.0751 | 7.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0188 | 0.0145 | 1.6% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.5450 | 0.5177 | 47.7% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.3027 | 0.2440 | 26.5% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.1032 | 0.1000 | 10.6% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0336 | 0.0294 | 3.4% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0010 | 0.0009 | 0.1% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0390 | 0.0374 | 4.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0171 | 0.0160 | 1.8% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.2050 | 0.1921 | 21.0% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0560 | 0.0425 | 5.7% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.2287 | 0.2131 | 23.4% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0128 | 0.0124 | 1.3% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 0.2806 | 0.2545 | 28.7% |
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0787 | 0.0722 | 5.2% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.3154 | 0.3445 | 20.9% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0528 | 0.0486 | 3.5% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0979 | 0.0882 | 6.5% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0255 | 0.0247 | 1.7% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 0.6697 | 0.5967 | 44.5% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.2661 | 0.2458 | 17.7% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.1122 | 0.1068 | 5.0% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.3576 | 0.3856 | 15.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0528 | 0.0486 | 2.3% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.1590 | 0.1502 | 7.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0376 | 0.0289 | 1.7% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 1.0900 | 1.0354 | 48.4% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.6053 | 0.4880 | 26.9% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 0.2422 | 0.2389 | 11.4% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0673 | 0.0632 | 3.2% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0032 | 0.0022 | 0.2% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0890 | 0.0816 | 4.2% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0329 | 0.0329 | 1.6% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 0.4669 | 0.4668 | 22.1% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.1494 | 0.1463 | 7.1% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.4574 | 0.4263 | 21.6% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0255 | 0.0247 | 1.2% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 0.5824 | 0.5044 | 27.5% |
