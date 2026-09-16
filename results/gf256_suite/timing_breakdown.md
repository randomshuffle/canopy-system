# GF(2^256) prover-time breakdown

Arithmetic means in seconds; phase means add to total means. Every measured repetition remains in the raw CSV files.

## WHIR: commitment and opening

| n | t | Scheme | Commit | Opening | Total | Median total |
|---|---|---|---|---|---|---|
| 18 | 1 | WHIR independent 1/2 | 0.02071 | 0.03148 | 0.05219 | 0.05272 |
| 18 | 1 | WHIR batched 1/2 | 0.02071 | 0.03148 | 0.05219 | 0.05272 |
| 18 | 1 | WHIR independent 1/4 | 0.03538 | 0.04713 | 0.08251 | 0.08167 |
| 18 | 1 | WHIR batched 1/4 | 0.03538 | 0.04713 | 0.08251 | 0.08167 |
| 18 | 1 | Canopy Exp. 2 | 0.02810 | 0.03165 | 0.05975 | 0.06128 |
| 18 | 1 | Canopy Exp. 3 | 0.03658 | 0.03422 | 0.07080 | 0.07209 |
| 18 | 2 | WHIR independent 1/2 | 0.04142 | 0.06296 | 0.10438 | 0.10544 |
| 18 | 2 | WHIR batched 1/2 | 0.04142 | 0.05422 | 0.09564 | 0.09703 |
| 18 | 2 | WHIR independent 1/4 | 0.07075 | 0.09426 | 0.16501 | 0.16333 |
| 18 | 2 | WHIR batched 1/4 | 0.07075 | 0.07743 | 0.14818 | 0.14670 |
| 18 | 2 | Canopy Exp. 2 | 0.05620 | 0.06330 | 0.11950 | 0.12256 |
| 18 | 2 | Canopy Exp. 3 | 0.06898 | 0.05124 | 0.12022 | 0.12294 |
| 18 | 4 | WHIR independent 1/2 | 0.08283 | 0.12592 | 0.20875 | 0.21087 |
| 18 | 4 | WHIR batched 1/2 | 0.08283 | 0.09806 | 0.18090 | 0.18466 |
| 18 | 4 | WHIR independent 1/4 | 0.14151 | 0.18852 | 0.33002 | 0.32667 |
| 18 | 4 | WHIR batched 1/4 | 0.14151 | 0.13941 | 0.28092 | 0.27700 |
| 18 | 4 | Canopy Exp. 2 | 0.11240 | 0.12660 | 0.23900 | 0.24513 |
| 18 | 4 | Canopy Exp. 3 | 0.12991 | 0.09481 | 0.22472 | 0.22398 |
| 18 | 8 | WHIR independent 1/2 | 0.16567 | 0.25184 | 0.41751 | 0.42174 |
| 18 | 8 | WHIR batched 1/2 | 0.16567 | 0.19477 | 0.36044 | 0.36722 |
| 18 | 8 | WHIR independent 1/4 | 0.28301 | 0.37703 | 0.66004 | 0.65334 |
| 18 | 8 | WHIR batched 1/4 | 0.28301 | 0.26853 | 0.55155 | 0.54141 |
| 18 | 8 | Canopy Exp. 2 | 0.22480 | 0.25320 | 0.47800 | 0.49025 |
| 18 | 8 | Canopy Exp. 3 | 0.24667 | 0.19958 | 0.44625 | 0.45370 |
| 20 | 1 | WHIR independent 1/2 | 0.08466 | 0.12173 | 0.20639 | 0.20715 |
| 20 | 1 | WHIR batched 1/2 | 0.08466 | 0.12173 | 0.20639 | 0.20715 |
| 20 | 1 | WHIR independent 1/4 | 0.15474 | 0.18319 | 0.33793 | 0.33695 |
| 20 | 1 | WHIR batched 1/4 | 0.15474 | 0.18319 | 0.33793 | 0.33695 |
| 20 | 1 | Canopy Exp. 2 | 0.13961 | 0.16495 | 0.30455 | 0.30460 |
| 20 | 1 | Canopy Exp. 3 | 0.15315 | 0.13181 | 0.28496 | 0.28591 |
| 20 | 2 | WHIR independent 1/2 | 0.16932 | 0.24346 | 0.41278 | 0.41430 |
| 20 | 2 | WHIR batched 1/2 | 0.16932 | 0.21548 | 0.38480 | 0.39021 |
| 20 | 2 | WHIR independent 1/4 | 0.30948 | 0.36638 | 0.67586 | 0.67390 |
| 20 | 2 | WHIR batched 1/4 | 0.30948 | 0.31191 | 0.62138 | 0.62717 |
| 20 | 2 | Canopy Exp. 2 | 0.27922 | 0.32989 | 0.60911 | 0.60920 |
| 20 | 2 | Canopy Exp. 3 | 0.28825 | 0.20940 | 0.49765 | 0.49732 |
| 20 | 4 | WHIR independent 1/2 | 0.33864 | 0.48692 | 0.82556 | 0.82860 |
| 20 | 4 | WHIR batched 1/2 | 0.33864 | 0.39245 | 0.73109 | 0.73200 |
| 20 | 4 | WHIR independent 1/4 | 0.61895 | 0.73276 | 1.35171 | 1.34780 |
| 20 | 4 | WHIR batched 1/4 | 0.61895 | 0.56829 | 1.18725 | 1.19394 |
| 20 | 4 | Canopy Exp. 2 | 0.55843 | 0.65978 | 1.21822 | 1.21840 |
| 20 | 4 | Canopy Exp. 3 | 0.56560 | 0.37923 | 0.94483 | 0.95099 |
| 20 | 8 | WHIR independent 1/2 | 0.67728 | 0.97384 | 1.65112 | 1.65720 |
| 20 | 8 | WHIR batched 1/2 | 0.67728 | 0.79206 | 1.46933 | 1.46565 |
| 20 | 8 | WHIR independent 1/4 | 1.23790 | 1.46552 | 2.70343 | 2.69559 |
| 20 | 8 | WHIR batched 1/4 | 1.23790 | 1.12491 | 2.36281 | 2.38373 |
| 20 | 8 | Canopy Exp. 2 | 1.11687 | 1.31957 | 2.43644 | 2.43679 |
| 20 | 8 | Canopy Exp. 3 | 1.22483 | 0.85752 | 2.08235 | 1.98549 |
| 22 | 1 | WHIR independent 1/2 | 0.48160 | 0.53349 | 1.01509 | 1.02001 |
| 22 | 1 | WHIR batched 1/2 | 0.48160 | 0.53349 | 1.01509 | 1.02001 |
| 22 | 1 | WHIR independent 1/4 | 0.87455 | 0.93235 | 1.80690 | 1.76135 |
| 22 | 1 | WHIR batched 1/4 | 0.87455 | 0.93235 | 1.80690 | 1.76135 |
| 22 | 1 | Canopy Exp. 2 | 0.76464 | 0.71038 | 1.47503 | 1.45906 |
| 22 | 1 | Canopy Exp. 3 | 0.87408 | 0.58163 | 1.45571 | 1.45132 |
| 22 | 2 | WHIR independent 1/2 | 0.96319 | 1.06699 | 2.03018 | 2.04002 |
| 22 | 2 | WHIR batched 1/2 | 0.96319 | 0.94449 | 1.90768 | 1.89301 |
| 22 | 2 | WHIR independent 1/4 | 1.74910 | 1.86470 | 3.61380 | 3.52269 |
| 22 | 2 | WHIR batched 1/4 | 1.74910 | 1.62797 | 3.37708 | 3.29656 |
| 22 | 2 | Canopy Exp. 2 | 1.52929 | 1.42077 | 2.95006 | 2.91811 |
| 22 | 2 | Canopy Exp. 3 | 1.67635 | 0.88652 | 2.56287 | 2.52939 |
| 22 | 4 | WHIR independent 1/2 | 1.92639 | 2.13398 | 4.06037 | 4.08003 |
| 22 | 4 | WHIR batched 1/2 | 1.92639 | 1.73485 | 3.66124 | 3.64157 |
| 22 | 4 | WHIR independent 1/4 | 3.49821 | 3.72940 | 7.22760 | 7.04539 |
| 22 | 4 | WHIR batched 1/4 | 3.49821 | 2.98570 | 6.48390 | 6.33250 |
| 22 | 4 | Canopy Exp. 2 | 3.05857 | 2.84154 | 5.90011 | 5.83622 |
| 22 | 4 | Canopy Exp. 3 | 3.24209 | 1.82281 | 5.06490 | 4.99412 |
| 22 | 8 | WHIR independent 1/2 | 3.85277 | 4.26796 | 8.12073 | 8.16006 |
| 22 | 8 | WHIR batched 1/2 | 3.85277 | 3.47500 | 7.32778 | 7.37562 |
| 22 | 8 | WHIR independent 1/4 | 6.99642 | 7.45879 | 14.45521 | 14.09077 |
| 22 | 8 | WHIR batched 1/4 | 6.99642 | 5.98068 | 12.97710 | 12.84076 |
| 22 | 8 | Canopy Exp. 2 | 6.11715 | 5.68307 | 11.80022 | 11.67245 |
| 22 | 8 | Canopy Exp. 3 | 8.53221 | 3.89685 | 12.42906 | 11.66249 |

## WHIR: measured stages

| n | t | Scheme | Stage | Mean seconds |
|---|---|---|---|---|
| 18 | 1 | WHIR independent 1/2 | initial_merkle_tree | 0.003045 |
| 18 | 1 | WHIR independent 1/2 | initial_ood_evaluations | 0.001020 |
| 18 | 1 | WHIR independent 1/2 | initial_rs_encoding | 0.017664 |
| 18 | 1 | WHIR independent 1/2 | whir_opening_proof_after_ood | 0.030460 |
| 18 | 1 | WHIR independent 1/4 | initial_merkle_tree | 0.005238 |
| 18 | 1 | WHIR independent 1/4 | initial_ood_evaluations | 0.001135 |
| 18 | 1 | WHIR independent 1/4 | initial_rs_encoding | 0.030138 |
| 18 | 1 | WHIR independent 1/4 | whir_opening_proof_after_ood | 0.045993 |
| 18 | 1 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.011164 |
| 18 | 1 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.000000 |
| 18 | 1 | WHIR batched 1/2 | initial_merkle_tree | 0.003045 |
| 18 | 1 | WHIR batched 1/2 | initial_ood_evaluations | 0.001020 |
| 18 | 1 | WHIR batched 1/2 | initial_rs_encoding | 0.017664 |
| 18 | 1 | WHIR batched 1/2 | recursive_tail_after_ood | 0.010862 |
| 18 | 1 | WHIR batched 1/2 | recursive_tail_ood_evaluations | 0.000218 |
| 18 | 1 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.008216 |
| 18 | 1 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.013255 |
| 18 | 1 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.000000 |
| 18 | 1 | WHIR batched 1/4 | initial_merkle_tree | 0.005238 |
| 18 | 1 | WHIR batched 1/4 | initial_ood_evaluations | 0.001135 |
| 18 | 1 | WHIR batched 1/4 | initial_rs_encoding | 0.030138 |
| 18 | 1 | WHIR batched 1/4 | recursive_tail_after_ood | 0.018240 |
| 18 | 1 | WHIR batched 1/4 | recursive_tail_ood_evaluations | 0.000200 |
| 18 | 1 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.014299 |
| 18 | 1 | Canopy Exp. 2 | canopy_bottom_forest | 0.002936 |
| 18 | 1 | Canopy Exp. 2 | canopy_bottom_opening | 0.000170 |
| 18 | 1 | Canopy Exp. 2 | canopy_claim_whirs | 0.000000 |
| 18 | 1 | Canopy Exp. 2 | canopy_codeword_merkle_tree | 0.003450 |
| 18 | 1 | Canopy Exp. 2 | canopy_commitment_other | 0.000001 |
| 18 | 1 | Canopy Exp. 2 | canopy_hash_to_field | 0.000027 |
| 18 | 1 | Canopy Exp. 2 | canopy_rs_encoding | 0.004022 |
| 18 | 1 | Canopy Exp. 2 | outer_initial_whir_stage_after_ood | 0.012370 |
| 18 | 1 | Canopy Exp. 2 | outer_ood_evaluations | 0.001020 |
| 18 | 1 | Canopy Exp. 2 | outer_rs_encoding | 0.017664 |
| 18 | 1 | Canopy Exp. 2 | outer_second_oracle_whirs | 0.018090 |
| 18 | 1 | Canopy Exp. 3 | batched_recursive_whir | 0.018741 |
| 18 | 1 | Canopy Exp. 3 | canopy_bottom_forest | 0.002324 |
| 18 | 1 | Canopy Exp. 3 | canopy_bottom_opening | 0.000358 |
| 18 | 1 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.002933 |
| 18 | 1 | Canopy Exp. 3 | canopy_commitment_other | 0.000001 |
| 18 | 1 | Canopy Exp. 3 | canopy_hash_to_field | 0.001186 |
| 18 | 1 | Canopy Exp. 3 | canopy_rs_encoding | 0.011699 |
| 18 | 1 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.001735 |
| 18 | 1 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.012370 |
| 18 | 1 | Canopy Exp. 3 | outer_ood_evaluations | 0.001020 |
| 18 | 1 | Canopy Exp. 3 | outer_rs_encoding | 0.018433 |
| 18 | 2 | WHIR independent 1/2 | initial_merkle_tree | 0.006089 |
| 18 | 2 | WHIR independent 1/2 | initial_ood_evaluations | 0.002040 |
| 18 | 2 | WHIR independent 1/2 | initial_rs_encoding | 0.035328 |
| 18 | 2 | WHIR independent 1/2 | whir_opening_proof_after_ood | 0.060921 |
| 18 | 2 | WHIR independent 1/4 | initial_merkle_tree | 0.010477 |
| 18 | 2 | WHIR independent 1/4 | initial_ood_evaluations | 0.002271 |
| 18 | 2 | WHIR independent 1/4 | initial_rs_encoding | 0.060277 |
| 18 | 2 | WHIR independent 1/4 | whir_opening_proof_after_ood | 0.091987 |
| 18 | 2 | WHIR batched 1/2 | batched_recursive_whir | 0.011871 |
| 18 | 2 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.022329 |
| 18 | 2 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.001547 |
| 18 | 2 | WHIR batched 1/2 | initial_merkle_tree | 0.006089 |
| 18 | 2 | WHIR batched 1/2 | initial_ood_evaluations | 0.002040 |
| 18 | 2 | WHIR batched 1/2 | initial_rs_encoding | 0.035328 |
| 18 | 2 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.016432 |
| 18 | 2 | WHIR batched 1/4 | batched_recursive_whir | 0.018503 |
| 18 | 2 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.026511 |
| 18 | 2 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.001547 |
| 18 | 2 | WHIR batched 1/4 | initial_merkle_tree | 0.010477 |
| 18 | 2 | WHIR batched 1/4 | initial_ood_evaluations | 0.002271 |
| 18 | 2 | WHIR batched 1/4 | initial_rs_encoding | 0.060277 |
| 18 | 2 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.028597 |
| 18 | 2 | Canopy Exp. 2 | canopy_bottom_forest | 0.005872 |
| 18 | 2 | Canopy Exp. 2 | canopy_bottom_opening | 0.000340 |
| 18 | 2 | Canopy Exp. 2 | canopy_claim_whirs | 0.000000 |
| 18 | 2 | Canopy Exp. 2 | canopy_codeword_merkle_tree | 0.006900 |
| 18 | 2 | Canopy Exp. 2 | canopy_commitment_other | 0.000002 |
| 18 | 2 | Canopy Exp. 2 | canopy_hash_to_field | 0.000054 |
| 18 | 2 | Canopy Exp. 2 | canopy_rs_encoding | 0.008043 |
| 18 | 2 | Canopy Exp. 2 | outer_initial_whir_stage_after_ood | 0.024740 |
| 18 | 2 | Canopy Exp. 2 | outer_ood_evaluations | 0.002040 |
| 18 | 2 | Canopy Exp. 2 | outer_rs_encoding | 0.035328 |
| 18 | 2 | Canopy Exp. 2 | outer_second_oracle_whirs | 0.036181 |
| 18 | 2 | Canopy Exp. 3 | batched_recursive_whir | 0.021210 |
| 18 | 2 | Canopy Exp. 3 | canopy_bottom_forest | 0.003909 |
| 18 | 2 | Canopy Exp. 3 | canopy_bottom_opening | 0.000288 |
| 18 | 2 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.004892 |
| 18 | 2 | Canopy Exp. 3 | canopy_commitment_other | 0.000002 |
| 18 | 2 | Canopy Exp. 3 | canopy_hash_to_field | 0.002093 |
| 18 | 2 | Canopy Exp. 3 | canopy_rs_encoding | 0.024113 |
| 18 | 2 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.002965 |
| 18 | 2 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.024740 |
| 18 | 2 | Canopy Exp. 3 | outer_ood_evaluations | 0.002040 |
| 18 | 2 | Canopy Exp. 3 | outer_rs_encoding | 0.033967 |
| 18 | 4 | WHIR independent 1/2 | initial_merkle_tree | 0.012178 |
| 18 | 4 | WHIR independent 1/2 | initial_ood_evaluations | 0.004080 |
| 18 | 4 | WHIR independent 1/2 | initial_rs_encoding | 0.070655 |
| 18 | 4 | WHIR independent 1/2 | whir_opening_proof_after_ood | 0.121841 |
| 18 | 4 | WHIR independent 1/4 | initial_merkle_tree | 0.020953 |
| 18 | 4 | WHIR independent 1/4 | initial_ood_evaluations | 0.004542 |
| 18 | 4 | WHIR independent 1/4 | initial_rs_encoding | 0.120553 |
| 18 | 4 | WHIR independent 1/4 | whir_opening_proof_after_ood | 0.183974 |
| 18 | 4 | WHIR batched 1/2 | batched_recursive_whir | 0.012924 |
| 18 | 4 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.044658 |
| 18 | 4 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.003537 |
| 18 | 4 | WHIR batched 1/2 | initial_merkle_tree | 0.012178 |
| 18 | 4 | WHIR batched 1/2 | initial_ood_evaluations | 0.004080 |
| 18 | 4 | WHIR batched 1/2 | initial_rs_encoding | 0.070655 |
| 18 | 4 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.032865 |
| 18 | 4 | WHIR batched 1/4 | batched_recursive_whir | 0.021118 |
| 18 | 4 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.053021 |
| 18 | 4 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.003537 |
| 18 | 4 | WHIR batched 1/4 | initial_merkle_tree | 0.020953 |
| 18 | 4 | WHIR batched 1/4 | initial_ood_evaluations | 0.004542 |
| 18 | 4 | WHIR batched 1/4 | initial_rs_encoding | 0.120553 |
| 18 | 4 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.057195 |
| 18 | 4 | Canopy Exp. 2 | canopy_bottom_forest | 0.011743 |
| 18 | 4 | Canopy Exp. 2 | canopy_bottom_opening | 0.000680 |
| 18 | 4 | Canopy Exp. 2 | canopy_claim_whirs | 0.000000 |
| 18 | 4 | Canopy Exp. 2 | canopy_codeword_merkle_tree | 0.013799 |
| 18 | 4 | Canopy Exp. 2 | canopy_commitment_other | 0.000005 |
| 18 | 4 | Canopy Exp. 2 | canopy_hash_to_field | 0.000108 |
| 18 | 4 | Canopy Exp. 2 | canopy_rs_encoding | 0.016086 |
| 18 | 4 | Canopy Exp. 2 | outer_initial_whir_stage_after_ood | 0.049480 |
| 18 | 4 | Canopy Exp. 2 | outer_ood_evaluations | 0.004080 |
| 18 | 4 | Canopy Exp. 2 | outer_rs_encoding | 0.070655 |
| 18 | 4 | Canopy Exp. 2 | outer_second_oracle_whirs | 0.072361 |
| 18 | 4 | Canopy Exp. 3 | batched_recursive_whir | 0.030707 |
| 18 | 4 | Canopy Exp. 3 | canopy_bottom_forest | 0.007619 |
| 18 | 4 | Canopy Exp. 3 | canopy_bottom_opening | 0.000472 |
| 18 | 4 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.009949 |
| 18 | 4 | Canopy Exp. 3 | canopy_commitment_other | 0.000002 |
| 18 | 4 | Canopy Exp. 3 | canopy_hash_to_field | 0.004165 |
| 18 | 4 | Canopy Exp. 3 | canopy_rs_encoding | 0.043983 |
| 18 | 4 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.010072 |
| 18 | 4 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.049480 |
| 18 | 4 | Canopy Exp. 3 | outer_ood_evaluations | 0.004080 |
| 18 | 4 | Canopy Exp. 3 | outer_rs_encoding | 0.064192 |
| 18 | 8 | WHIR independent 1/2 | initial_merkle_tree | 0.024357 |
| 18 | 8 | WHIR independent 1/2 | initial_ood_evaluations | 0.008160 |
| 18 | 8 | WHIR independent 1/2 | initial_rs_encoding | 0.141311 |
| 18 | 8 | WHIR independent 1/2 | whir_opening_proof_after_ood | 0.243682 |
| 18 | 8 | WHIR independent 1/4 | initial_merkle_tree | 0.041906 |
| 18 | 8 | WHIR independent 1/4 | initial_ood_evaluations | 0.009084 |
| 18 | 8 | WHIR independent 1/4 | initial_rs_encoding | 0.241106 |
| 18 | 8 | WHIR independent 1/4 | whir_opening_proof_after_ood | 0.367948 |
| 18 | 8 | WHIR batched 1/2 | batched_recursive_whir | 0.020178 |
| 18 | 8 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.089315 |
| 18 | 8 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.011390 |
| 18 | 8 | WHIR batched 1/2 | initial_merkle_tree | 0.024357 |
| 18 | 8 | WHIR batched 1/2 | initial_ood_evaluations | 0.008160 |
| 18 | 8 | WHIR batched 1/2 | initial_rs_encoding | 0.141311 |
| 18 | 8 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.065729 |
| 18 | 8 | WHIR batched 1/4 | batched_recursive_whir | 0.027628 |
| 18 | 8 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.106042 |
| 18 | 8 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.011390 |
| 18 | 8 | WHIR batched 1/4 | initial_merkle_tree | 0.041906 |
| 18 | 8 | WHIR batched 1/4 | initial_ood_evaluations | 0.009084 |
| 18 | 8 | WHIR batched 1/4 | initial_rs_encoding | 0.241106 |
| 18 | 8 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.114389 |
| 18 | 8 | Canopy Exp. 2 | canopy_bottom_forest | 0.023487 |
| 18 | 8 | Canopy Exp. 2 | canopy_bottom_opening | 0.001359 |
| 18 | 8 | Canopy Exp. 2 | canopy_claim_whirs | 0.000000 |
| 18 | 8 | Canopy Exp. 2 | canopy_codeword_merkle_tree | 0.027599 |
| 18 | 8 | Canopy Exp. 2 | canopy_commitment_other | 0.000009 |
| 18 | 8 | Canopy Exp. 2 | canopy_hash_to_field | 0.000217 |
| 18 | 8 | Canopy Exp. 2 | canopy_rs_encoding | 0.032173 |
| 18 | 8 | Canopy Exp. 2 | outer_initial_whir_stage_after_ood | 0.098959 |
| 18 | 8 | Canopy Exp. 2 | outer_ood_evaluations | 0.008160 |
| 18 | 8 | Canopy Exp. 2 | outer_rs_encoding | 0.141311 |
| 18 | 8 | Canopy Exp. 2 | outer_second_oracle_whirs | 0.144723 |
| 18 | 8 | Canopy Exp. 3 | batched_recursive_whir | 0.053319 |
| 18 | 8 | Canopy Exp. 3 | canopy_bottom_forest | 0.016152 |
| 18 | 8 | Canopy Exp. 3 | canopy_bottom_opening | 0.000267 |
| 18 | 8 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.017515 |
| 18 | 8 | Canopy Exp. 3 | canopy_commitment_other | 0.000001 |
| 18 | 8 | Canopy Exp. 3 | canopy_hash_to_field | 0.008218 |
| 18 | 8 | Canopy Exp. 3 | canopy_rs_encoding | 0.084330 |
| 18 | 8 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.038870 |
| 18 | 8 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.098959 |
| 18 | 8 | Canopy Exp. 3 | outer_ood_evaluations | 0.008160 |
| 18 | 8 | Canopy Exp. 3 | outer_rs_encoding | 0.120457 |
| 20 | 1 | WHIR independent 1/2 | initial_merkle_tree | 0.010761 |
| 20 | 1 | WHIR independent 1/2 | initial_ood_evaluations | 0.003909 |
| 20 | 1 | WHIR independent 1/2 | initial_rs_encoding | 0.073898 |
| 20 | 1 | WHIR independent 1/2 | whir_opening_proof_after_ood | 0.117821 |
| 20 | 1 | WHIR independent 1/4 | initial_merkle_tree | 0.020414 |
| 20 | 1 | WHIR independent 1/4 | initial_ood_evaluations | 0.003329 |
| 20 | 1 | WHIR independent 1/4 | initial_rs_encoding | 0.134324 |
| 20 | 1 | WHIR independent 1/4 | whir_opening_proof_after_ood | 0.179861 |
| 20 | 1 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.046864 |
| 20 | 1 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.000000 |
| 20 | 1 | WHIR batched 1/2 | initial_merkle_tree | 0.010761 |
| 20 | 1 | WHIR batched 1/2 | initial_ood_evaluations | 0.003909 |
| 20 | 1 | WHIR batched 1/2 | initial_rs_encoding | 0.073898 |
| 20 | 1 | WHIR batched 1/2 | recursive_tail_after_ood | 0.038852 |
| 20 | 1 | WHIR batched 1/2 | recursive_tail_ood_evaluations | 0.000655 |
| 20 | 1 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.031450 |
| 20 | 1 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.054495 |
| 20 | 1 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.000000 |
| 20 | 1 | WHIR batched 1/4 | initial_merkle_tree | 0.020414 |
| 20 | 1 | WHIR batched 1/4 | initial_ood_evaluations | 0.003329 |
| 20 | 1 | WHIR batched 1/4 | initial_rs_encoding | 0.134324 |
| 20 | 1 | WHIR batched 1/4 | recursive_tail_after_ood | 0.062476 |
| 20 | 1 | WHIR batched 1/4 | recursive_tail_ood_evaluations | 0.000498 |
| 20 | 1 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.062392 |
| 20 | 1 | Canopy Exp. 2 | canopy_bottom_forest | 0.009389 |
| 20 | 1 | Canopy Exp. 2 | canopy_bottom_opening | 0.000114 |
| 20 | 1 | Canopy Exp. 2 | canopy_claim_whirs | 0.043102 |
| 20 | 1 | Canopy Exp. 2 | canopy_codeword_merkle_tree | 0.012999 |
| 20 | 1 | Canopy Exp. 2 | canopy_commitment_other | 0.000001 |
| 20 | 1 | Canopy Exp. 2 | canopy_hash_to_field | 0.000577 |
| 20 | 1 | Canopy Exp. 2 | canopy_rs_encoding | 0.042744 |
| 20 | 1 | Canopy Exp. 2 | outer_initial_whir_stage_after_ood | 0.053684 |
| 20 | 1 | Canopy Exp. 2 | outer_ood_evaluations | 0.003909 |
| 20 | 1 | Canopy Exp. 2 | outer_rs_encoding | 0.073898 |
| 20 | 1 | Canopy Exp. 2 | outer_second_oracle_whirs | 0.064138 |
| 20 | 1 | Canopy Exp. 3 | batched_recursive_whir | 0.066414 |
| 20 | 1 | Canopy Exp. 3 | canopy_bottom_forest | 0.008444 |
| 20 | 1 | Canopy Exp. 3 | canopy_bottom_opening | 0.000580 |
| 20 | 1 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.010769 |
| 20 | 1 | Canopy Exp. 3 | canopy_commitment_other | 0.000001 |
| 20 | 1 | Canopy Exp. 3 | canopy_hash_to_field | 0.004099 |
| 20 | 1 | Canopy Exp. 3 | canopy_rs_encoding | 0.052270 |
| 20 | 1 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.007220 |
| 20 | 1 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.053684 |
| 20 | 1 | Canopy Exp. 3 | outer_ood_evaluations | 0.003909 |
| 20 | 1 | Canopy Exp. 3 | outer_rs_encoding | 0.077570 |
| 20 | 2 | WHIR independent 1/2 | initial_merkle_tree | 0.021522 |
| 20 | 2 | WHIR independent 1/2 | initial_ood_evaluations | 0.007818 |
| 20 | 2 | WHIR independent 1/2 | initial_rs_encoding | 0.147796 |
| 20 | 2 | WHIR independent 1/2 | whir_opening_proof_after_ood | 0.235642 |
| 20 | 2 | WHIR independent 1/4 | initial_merkle_tree | 0.040828 |
| 20 | 2 | WHIR independent 1/4 | initial_ood_evaluations | 0.006658 |
| 20 | 2 | WHIR independent 1/4 | initial_rs_encoding | 0.268648 |
| 20 | 2 | WHIR independent 1/4 | whir_opening_proof_after_ood | 0.359723 |
| 20 | 2 | WHIR batched 1/2 | batched_recursive_whir | 0.044186 |
| 20 | 2 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.093727 |
| 20 | 2 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.006853 |
| 20 | 2 | WHIR batched 1/2 | initial_merkle_tree | 0.021522 |
| 20 | 2 | WHIR batched 1/2 | initial_ood_evaluations | 0.007818 |
| 20 | 2 | WHIR batched 1/2 | initial_rs_encoding | 0.147796 |
| 20 | 2 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.062900 |
| 20 | 2 | WHIR batched 1/4 | batched_recursive_whir | 0.064623 |
| 20 | 2 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.108991 |
| 20 | 2 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.006853 |
| 20 | 2 | WHIR batched 1/4 | initial_merkle_tree | 0.040828 |
| 20 | 2 | WHIR batched 1/4 | initial_ood_evaluations | 0.006658 |
| 20 | 2 | WHIR batched 1/4 | initial_rs_encoding | 0.268648 |
| 20 | 2 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.124783 |
| 20 | 2 | Canopy Exp. 2 | canopy_bottom_forest | 0.018779 |
| 20 | 2 | Canopy Exp. 2 | canopy_bottom_opening | 0.000227 |
| 20 | 2 | Canopy Exp. 2 | canopy_claim_whirs | 0.086205 |
| 20 | 2 | Canopy Exp. 2 | canopy_codeword_merkle_tree | 0.025997 |
| 20 | 2 | Canopy Exp. 2 | canopy_commitment_other | 0.000003 |
| 20 | 2 | Canopy Exp. 2 | canopy_hash_to_field | 0.001153 |
| 20 | 2 | Canopy Exp. 2 | canopy_rs_encoding | 0.085489 |
| 20 | 2 | Canopy Exp. 2 | outer_initial_whir_stage_after_ood | 0.107367 |
| 20 | 2 | Canopy Exp. 2 | outer_ood_evaluations | 0.007818 |
| 20 | 2 | Canopy Exp. 2 | outer_rs_encoding | 0.147796 |
| 20 | 2 | Canopy Exp. 2 | outer_second_oracle_whirs | 0.128275 |
| 20 | 2 | Canopy Exp. 3 | batched_recursive_whir | 0.079218 |
| 20 | 2 | Canopy Exp. 3 | canopy_bottom_forest | 0.016213 |
| 20 | 2 | Canopy Exp. 3 | canopy_bottom_opening | 0.000951 |
| 20 | 2 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.020617 |
| 20 | 2 | Canopy Exp. 3 | canopy_commitment_other | 0.000001 |
| 20 | 2 | Canopy Exp. 3 | canopy_hash_to_field | 0.008435 |
| 20 | 2 | Canopy Exp. 3 | canopy_rs_encoding | 0.099227 |
| 20 | 2 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.014045 |
| 20 | 2 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.107367 |
| 20 | 2 | Canopy Exp. 3 | outer_ood_evaluations | 0.007818 |
| 20 | 2 | Canopy Exp. 3 | outer_rs_encoding | 0.143757 |
| 20 | 4 | WHIR independent 1/2 | initial_merkle_tree | 0.043045 |
| 20 | 4 | WHIR independent 1/2 | initial_ood_evaluations | 0.015636 |
| 20 | 4 | WHIR independent 1/2 | initial_rs_encoding | 0.295593 |
| 20 | 4 | WHIR independent 1/2 | whir_opening_proof_after_ood | 0.471284 |
| 20 | 4 | WHIR independent 1/4 | initial_merkle_tree | 0.081657 |
| 20 | 4 | WHIR independent 1/4 | initial_ood_evaluations | 0.013315 |
| 20 | 4 | WHIR independent 1/4 | initial_rs_encoding | 0.537295 |
| 20 | 4 | WHIR independent 1/4 | whir_opening_proof_after_ood | 0.719445 |
| 20 | 4 | WHIR batched 1/2 | batched_recursive_whir | 0.050013 |
| 20 | 4 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.187454 |
| 20 | 4 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.013548 |
| 20 | 4 | WHIR batched 1/2 | initial_merkle_tree | 0.043045 |
| 20 | 4 | WHIR batched 1/2 | initial_ood_evaluations | 0.015636 |
| 20 | 4 | WHIR batched 1/2 | initial_rs_encoding | 0.295593 |
| 20 | 4 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.125800 |
| 20 | 4 | WHIR batched 1/4 | batched_recursive_whir | 0.073883 |
| 20 | 4 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.217981 |
| 20 | 4 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.013548 |
| 20 | 4 | WHIR batched 1/4 | initial_merkle_tree | 0.081657 |
| 20 | 4 | WHIR batched 1/4 | initial_ood_evaluations | 0.013315 |
| 20 | 4 | WHIR batched 1/4 | initial_rs_encoding | 0.537295 |
| 20 | 4 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.249567 |
| 20 | 4 | Canopy Exp. 2 | canopy_bottom_forest | 0.037558 |
| 20 | 4 | Canopy Exp. 2 | canopy_bottom_opening | 0.000455 |
| 20 | 4 | Canopy Exp. 2 | canopy_claim_whirs | 0.172410 |
| 20 | 4 | Canopy Exp. 2 | canopy_codeword_merkle_tree | 0.051994 |
| 20 | 4 | Canopy Exp. 2 | canopy_commitment_other | 0.000005 |
| 20 | 4 | Canopy Exp. 2 | canopy_hash_to_field | 0.002307 |
| 20 | 4 | Canopy Exp. 2 | canopy_rs_encoding | 0.170977 |
| 20 | 4 | Canopy Exp. 2 | outer_initial_whir_stage_after_ood | 0.214734 |
| 20 | 4 | Canopy Exp. 2 | outer_ood_evaluations | 0.015636 |
| 20 | 4 | Canopy Exp. 2 | outer_rs_encoding | 0.295593 |
| 20 | 4 | Canopy Exp. 2 | outer_second_oracle_whirs | 0.256550 |
| 20 | 4 | Canopy Exp. 3 | batched_recursive_whir | 0.102776 |
| 20 | 4 | Canopy Exp. 3 | canopy_bottom_forest | 0.032009 |
| 20 | 4 | Canopy Exp. 3 | canopy_bottom_opening | 0.001015 |
| 20 | 4 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.039386 |
| 20 | 4 | Canopy Exp. 3 | canopy_commitment_other | 0.000001 |
| 20 | 4 | Canopy Exp. 3 | canopy_hash_to_field | 0.017285 |
| 20 | 4 | Canopy Exp. 3 | canopy_rs_encoding | 0.194549 |
| 20 | 4 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.045066 |
| 20 | 4 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.214734 |
| 20 | 4 | Canopy Exp. 3 | outer_ood_evaluations | 0.015636 |
| 20 | 4 | Canopy Exp. 3 | outer_rs_encoding | 0.282371 |
| 20 | 8 | WHIR independent 1/2 | initial_merkle_tree | 0.086089 |
| 20 | 8 | WHIR independent 1/2 | initial_ood_evaluations | 0.031272 |
| 20 | 8 | WHIR independent 1/2 | initial_rs_encoding | 0.591186 |
| 20 | 8 | WHIR independent 1/2 | whir_opening_proof_after_ood | 0.942569 |
| 20 | 8 | WHIR independent 1/4 | initial_merkle_tree | 0.163314 |
| 20 | 8 | WHIR independent 1/4 | initial_ood_evaluations | 0.026631 |
| 20 | 8 | WHIR independent 1/4 | initial_rs_encoding | 1.074590 |
| 20 | 8 | WHIR independent 1/4 | whir_opening_proof_after_ood | 1.438891 |
| 20 | 8 | WHIR batched 1/2 | batched_recursive_whir | 0.083015 |
| 20 | 8 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.374909 |
| 20 | 8 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.051261 |
| 20 | 8 | WHIR batched 1/2 | initial_merkle_tree | 0.086089 |
| 20 | 8 | WHIR batched 1/2 | initial_ood_evaluations | 0.031272 |
| 20 | 8 | WHIR batched 1/2 | initial_rs_encoding | 0.591186 |
| 20 | 8 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.251601 |
| 20 | 8 | WHIR batched 1/4 | batched_recursive_whir | 0.111921 |
| 20 | 8 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.435963 |
| 20 | 8 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.051261 |
| 20 | 8 | WHIR batched 1/4 | initial_merkle_tree | 0.163314 |
| 20 | 8 | WHIR batched 1/4 | initial_ood_evaluations | 0.026631 |
| 20 | 8 | WHIR batched 1/4 | initial_rs_encoding | 1.074590 |
| 20 | 8 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.499133 |
| 20 | 8 | Canopy Exp. 2 | canopy_bottom_forest | 0.075116 |
| 20 | 8 | Canopy Exp. 2 | canopy_bottom_opening | 0.000909 |
| 20 | 8 | Canopy Exp. 2 | canopy_claim_whirs | 0.344820 |
| 20 | 8 | Canopy Exp. 2 | canopy_codeword_merkle_tree | 0.103989 |
| 20 | 8 | Canopy Exp. 2 | canopy_commitment_other | 0.000011 |
| 20 | 8 | Canopy Exp. 2 | canopy_hash_to_field | 0.004614 |
| 20 | 8 | Canopy Exp. 2 | canopy_rs_encoding | 0.341955 |
| 20 | 8 | Canopy Exp. 2 | outer_initial_whir_stage_after_ood | 0.429468 |
| 20 | 8 | Canopy Exp. 2 | outer_ood_evaluations | 0.031272 |
| 20 | 8 | Canopy Exp. 2 | outer_rs_encoding | 0.591186 |
| 20 | 8 | Canopy Exp. 2 | outer_second_oracle_whirs | 0.513101 |
| 20 | 8 | Canopy Exp. 3 | batched_recursive_whir | 0.221598 |
| 20 | 8 | Canopy Exp. 3 | canopy_bottom_forest | 0.067166 |
| 20 | 8 | Canopy Exp. 3 | canopy_bottom_opening | 0.003454 |
| 20 | 8 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.087897 |
| 20 | 8 | Canopy Exp. 3 | canopy_commitment_other | 0.000002 |
| 20 | 8 | Canopy Exp. 3 | canopy_hash_to_field | 0.036504 |
| 20 | 8 | Canopy Exp. 3 | canopy_rs_encoding | 0.472759 |
| 20 | 8 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.171733 |
| 20 | 8 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.429468 |
| 20 | 8 | Canopy Exp. 3 | outer_ood_evaluations | 0.031272 |
| 20 | 8 | Canopy Exp. 3 | outer_rs_encoding | 0.560502 |
| 22 | 1 | WHIR independent 1/2 | initial_merkle_tree | 0.050110 |
| 22 | 1 | WHIR independent 1/2 | initial_ood_evaluations | 0.015112 |
| 22 | 1 | WHIR independent 1/2 | initial_rs_encoding | 0.431487 |
| 22 | 1 | WHIR independent 1/2 | whir_opening_proof_after_ood | 0.518383 |
| 22 | 1 | WHIR independent 1/4 | initial_merkle_tree | 0.094885 |
| 22 | 1 | WHIR independent 1/4 | initial_ood_evaluations | 0.020008 |
| 22 | 1 | WHIR independent 1/4 | initial_rs_encoding | 0.779667 |
| 22 | 1 | WHIR independent 1/4 | whir_opening_proof_after_ood | 0.912341 |
| 22 | 1 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.170668 |
| 22 | 1 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.000000 |
| 22 | 1 | WHIR batched 1/2 | initial_merkle_tree | 0.050110 |
| 22 | 1 | WHIR batched 1/2 | initial_ood_evaluations | 0.015112 |
| 22 | 1 | WHIR batched 1/2 | initial_rs_encoding | 0.431487 |
| 22 | 1 | WHIR batched 1/2 | recursive_tail_after_ood | 0.176354 |
| 22 | 1 | WHIR batched 1/2 | recursive_tail_ood_evaluations | 0.002210 |
| 22 | 1 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.169150 |
| 22 | 1 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.244224 |
| 22 | 1 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.000000 |
| 22 | 1 | WHIR batched 1/4 | initial_merkle_tree | 0.094885 |
| 22 | 1 | WHIR batched 1/4 | initial_ood_evaluations | 0.020008 |
| 22 | 1 | WHIR batched 1/4 | initial_rs_encoding | 0.779667 |
| 22 | 1 | WHIR batched 1/4 | recursive_tail_after_ood | 0.290982 |
| 22 | 1 | WHIR batched 1/4 | recursive_tail_ood_evaluations | 0.002080 |
| 22 | 1 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.375056 |
| 22 | 1 | Canopy Exp. 2 | canopy_bottom_forest | 0.049578 |
| 22 | 1 | Canopy Exp. 2 | canopy_bottom_opening | 0.000197 |
| 22 | 1 | Canopy Exp. 2 | canopy_claim_whirs | 0.176693 |
| 22 | 1 | Canopy Exp. 2 | canopy_codeword_merkle_tree | 0.052396 |
| 22 | 1 | Canopy Exp. 2 | canopy_commitment_other | 0.000001 |
| 22 | 1 | Canopy Exp. 2 | canopy_hash_to_field | 0.001185 |
| 22 | 1 | Canopy Exp. 2 | canopy_rs_encoding | 0.229996 |
| 22 | 1 | Canopy Exp. 2 | outer_initial_whir_stage_after_ood | 0.217751 |
| 22 | 1 | Canopy Exp. 2 | outer_ood_evaluations | 0.015112 |
| 22 | 1 | Canopy Exp. 2 | outer_rs_encoding | 0.431487 |
| 22 | 1 | Canopy Exp. 2 | outer_second_oracle_whirs | 0.300632 |
| 22 | 1 | Canopy Exp. 3 | batched_recursive_whir | 0.312861 |
| 22 | 1 | Canopy Exp. 3 | canopy_bottom_forest | 0.043955 |
| 22 | 1 | Canopy Exp. 3 | canopy_bottom_opening | 0.000176 |
| 22 | 1 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.049341 |
| 22 | 1 | Canopy Exp. 3 | canopy_commitment_other | 0.000002 |
| 22 | 1 | Canopy Exp. 3 | canopy_hash_to_field | 0.022147 |
| 22 | 1 | Canopy Exp. 3 | canopy_rs_encoding | 0.305743 |
| 22 | 1 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.035731 |
| 22 | 1 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.217751 |
| 22 | 1 | Canopy Exp. 3 | outer_ood_evaluations | 0.015112 |
| 22 | 1 | Canopy Exp. 3 | outer_rs_encoding | 0.452894 |
| 22 | 2 | WHIR independent 1/2 | initial_merkle_tree | 0.100219 |
| 22 | 2 | WHIR independent 1/2 | initial_ood_evaluations | 0.030225 |
| 22 | 2 | WHIR independent 1/2 | initial_rs_encoding | 0.862973 |
| 22 | 2 | WHIR independent 1/2 | whir_opening_proof_after_ood | 1.036765 |
| 22 | 2 | WHIR independent 1/4 | initial_merkle_tree | 0.189770 |
| 22 | 2 | WHIR independent 1/4 | initial_ood_evaluations | 0.040016 |
| 22 | 2 | WHIR independent 1/4 | initial_rs_encoding | 1.559334 |
| 22 | 2 | WHIR independent 1/4 | whir_opening_proof_after_ood | 1.824682 |
| 22 | 2 | WHIR batched 1/2 | batched_recursive_whir | 0.198243 |
| 22 | 2 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.341337 |
| 22 | 2 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.036385 |
| 22 | 2 | WHIR batched 1/2 | initial_merkle_tree | 0.100219 |
| 22 | 2 | WHIR batched 1/2 | initial_ood_evaluations | 0.030225 |
| 22 | 2 | WHIR batched 1/2 | initial_rs_encoding | 0.862973 |
| 22 | 2 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.338301 |
| 22 | 2 | WHIR batched 1/4 | batched_recursive_whir | 0.313014 |
| 22 | 2 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.488447 |
| 22 | 2 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.036385 |
| 22 | 2 | WHIR batched 1/4 | initial_merkle_tree | 0.189770 |
| 22 | 2 | WHIR batched 1/4 | initial_ood_evaluations | 0.040016 |
| 22 | 2 | WHIR batched 1/4 | initial_rs_encoding | 1.559334 |
| 22 | 2 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.750112 |
| 22 | 2 | Canopy Exp. 2 | canopy_bottom_forest | 0.099156 |
| 22 | 2 | Canopy Exp. 2 | canopy_bottom_opening | 0.000394 |
| 22 | 2 | Canopy Exp. 2 | canopy_claim_whirs | 0.353385 |
| 22 | 2 | Canopy Exp. 2 | canopy_codeword_merkle_tree | 0.104793 |
| 22 | 2 | Canopy Exp. 2 | canopy_commitment_other | 0.000003 |
| 22 | 2 | Canopy Exp. 2 | canopy_hash_to_field | 0.002371 |
| 22 | 2 | Canopy Exp. 2 | canopy_rs_encoding | 0.459991 |
| 22 | 2 | Canopy Exp. 2 | outer_initial_whir_stage_after_ood | 0.435501 |
| 22 | 2 | Canopy Exp. 2 | outer_ood_evaluations | 0.030225 |
| 22 | 2 | Canopy Exp. 2 | outer_rs_encoding | 0.862973 |
| 22 | 2 | Canopy Exp. 2 | outer_second_oracle_whirs | 0.601264 |
| 22 | 2 | Canopy Exp. 3 | batched_recursive_whir | 0.356475 |
| 22 | 2 | Canopy Exp. 3 | canopy_bottom_forest | 0.084943 |
| 22 | 2 | Canopy Exp. 3 | canopy_bottom_opening | 0.000429 |
| 22 | 2 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.097626 |
| 22 | 2 | Canopy Exp. 3 | canopy_commitment_other | 0.000001 |
| 22 | 2 | Canopy Exp. 3 | canopy_hash_to_field | 0.044507 |
| 22 | 2 | Canopy Exp. 3 | canopy_rs_encoding | 0.596582 |
| 22 | 2 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.063886 |
| 22 | 2 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.435501 |
| 22 | 2 | Canopy Exp. 3 | outer_ood_evaluations | 0.030225 |
| 22 | 2 | Canopy Exp. 3 | outer_rs_encoding | 0.852693 |
| 22 | 4 | WHIR independent 1/2 | initial_merkle_tree | 0.200439 |
| 22 | 4 | WHIR independent 1/2 | initial_ood_evaluations | 0.060450 |
| 22 | 4 | WHIR independent 1/2 | initial_rs_encoding | 1.725947 |
| 22 | 4 | WHIR independent 1/2 | whir_opening_proof_after_ood | 2.073530 |
| 22 | 4 | WHIR independent 1/4 | initial_merkle_tree | 0.379540 |
| 22 | 4 | WHIR independent 1/4 | initial_ood_evaluations | 0.080032 |
| 22 | 4 | WHIR independent 1/4 | initial_rs_encoding | 3.118668 |
| 22 | 4 | WHIR independent 1/4 | whir_opening_proof_after_ood | 3.649364 |
| 22 | 4 | WHIR batched 1/2 | batched_recursive_whir | 0.244944 |
| 22 | 4 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.682673 |
| 22 | 4 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.070182 |
| 22 | 4 | WHIR batched 1/2 | initial_merkle_tree | 0.200439 |
| 22 | 4 | WHIR batched 1/2 | initial_ood_evaluations | 0.060450 |
| 22 | 4 | WHIR batched 1/2 | initial_rs_encoding | 1.725947 |
| 22 | 4 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.676601 |
| 22 | 4 | WHIR batched 1/4 | batched_recursive_whir | 0.358364 |
| 22 | 4 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.976895 |
| 22 | 4 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.070182 |
| 22 | 4 | WHIR batched 1/4 | initial_merkle_tree | 0.379540 |
| 22 | 4 | WHIR batched 1/4 | initial_ood_evaluations | 0.080032 |
| 22 | 4 | WHIR batched 1/4 | initial_rs_encoding | 3.118668 |
| 22 | 4 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 1.500223 |
| 22 | 4 | Canopy Exp. 2 | canopy_bottom_forest | 0.198312 |
| 22 | 4 | Canopy Exp. 2 | canopy_bottom_opening | 0.000787 |
| 22 | 4 | Canopy Exp. 2 | canopy_claim_whirs | 0.706770 |
| 22 | 4 | Canopy Exp. 2 | canopy_codeword_merkle_tree | 0.209585 |
| 22 | 4 | Canopy Exp. 2 | canopy_commitment_other | 0.000006 |
| 22 | 4 | Canopy Exp. 2 | canopy_hash_to_field | 0.004742 |
| 22 | 4 | Canopy Exp. 2 | canopy_rs_encoding | 0.919982 |
| 22 | 4 | Canopy Exp. 2 | outer_initial_whir_stage_after_ood | 0.871002 |
| 22 | 4 | Canopy Exp. 2 | outer_ood_evaluations | 0.060450 |
| 22 | 4 | Canopy Exp. 2 | outer_rs_encoding | 1.725947 |
| 22 | 4 | Canopy Exp. 2 | outer_second_oracle_whirs | 1.202528 |
| 22 | 4 | Canopy Exp. 3 | batched_recursive_whir | 0.676595 |
| 22 | 4 | Canopy Exp. 3 | canopy_bottom_forest | 0.164669 |
| 22 | 4 | Canopy Exp. 3 | canopy_bottom_opening | 0.000595 |
| 22 | 4 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.183260 |
| 22 | 4 | Canopy Exp. 3 | canopy_commitment_other | 0.000002 |
| 22 | 4 | Canopy Exp. 3 | canopy_hash_to_field | 0.088543 |
| 22 | 4 | Canopy Exp. 3 | canopy_rs_encoding | 1.163342 |
| 22 | 4 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.214171 |
| 22 | 4 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.871002 |
| 22 | 4 | Canopy Exp. 3 | outer_ood_evaluations | 0.060450 |
| 22 | 4 | Canopy Exp. 3 | outer_rs_encoding | 1.642270 |
| 22 | 8 | WHIR independent 1/2 | initial_merkle_tree | 0.400878 |
| 22 | 8 | WHIR independent 1/2 | initial_ood_evaluations | 0.120899 |
| 22 | 8 | WHIR independent 1/2 | initial_rs_encoding | 3.451894 |
| 22 | 8 | WHIR independent 1/2 | whir_opening_proof_after_ood | 4.147061 |
| 22 | 8 | WHIR independent 1/4 | initial_merkle_tree | 0.759081 |
| 22 | 8 | WHIR independent 1/4 | initial_ood_evaluations | 0.160064 |
| 22 | 8 | WHIR independent 1/4 | initial_rs_encoding | 6.237335 |
| 22 | 8 | WHIR independent 1/4 | whir_opening_proof_after_ood | 7.298728 |
| 22 | 8 | WHIR batched 1/2 | batched_recursive_whir | 0.428084 |
| 22 | 8 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 1.365347 |
| 22 | 8 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.207471 |
| 22 | 8 | WHIR batched 1/2 | initial_merkle_tree | 0.400878 |
| 22 | 8 | WHIR batched 1/2 | initial_ood_evaluations | 0.120899 |
| 22 | 8 | WHIR batched 1/2 | initial_rs_encoding | 3.451894 |
| 22 | 8 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 1.353203 |
| 22 | 8 | WHIR batched 1/4 | batched_recursive_whir | 0.658913 |
| 22 | 8 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 1.953790 |
| 22 | 8 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.207471 |
| 22 | 8 | WHIR batched 1/4 | initial_merkle_tree | 0.759081 |
| 22 | 8 | WHIR batched 1/4 | initial_ood_evaluations | 0.160064 |
| 22 | 8 | WHIR batched 1/4 | initial_rs_encoding | 6.237335 |
| 22 | 8 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 3.000447 |
| 22 | 8 | Canopy Exp. 2 | canopy_bottom_forest | 0.396625 |
| 22 | 8 | Canopy Exp. 2 | canopy_bottom_opening | 0.001574 |
| 22 | 8 | Canopy Exp. 2 | canopy_claim_whirs | 1.413540 |
| 22 | 8 | Canopy Exp. 2 | canopy_codeword_merkle_tree | 0.419170 |
| 22 | 8 | Canopy Exp. 2 | canopy_commitment_other | 0.000012 |
| 22 | 8 | Canopy Exp. 2 | canopy_hash_to_field | 0.009483 |
| 22 | 8 | Canopy Exp. 2 | canopy_rs_encoding | 1.839964 |
| 22 | 8 | Canopy Exp. 2 | outer_initial_whir_stage_after_ood | 1.742005 |
| 22 | 8 | Canopy Exp. 2 | outer_ood_evaluations | 0.120899 |
| 22 | 8 | Canopy Exp. 2 | outer_rs_encoding | 3.451894 |
| 22 | 8 | Canopy Exp. 2 | outer_second_oracle_whirs | 2.405056 |
| 22 | 8 | Canopy Exp. 3 | batched_recursive_whir | 1.169467 |
| 22 | 8 | Canopy Exp. 3 | canopy_bottom_forest | 0.330541 |
| 22 | 8 | Canopy Exp. 3 | canopy_bottom_opening | 0.011506 |
| 22 | 8 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.347039 |
| 22 | 8 | Canopy Exp. 3 | canopy_commitment_other | 0.000002 |
| 22 | 8 | Canopy Exp. 3 | canopy_hash_to_field | 0.176970 |
| 22 | 8 | Canopy Exp. 3 | canopy_rs_encoding | 4.107950 |
| 22 | 8 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.852976 |
| 22 | 8 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 1.742005 |
| 22 | 8 | Canopy Exp. 3 | outer_ood_evaluations | 0.120899 |
| 22 | 8 | Canopy Exp. 3 | outer_rs_encoding | 3.569706 |

## Bolt: all configurations

| n | w | Scheme | RS block | h X/Y | Commit | Opening | Total | Median total |
|---|---|---|---|---|---|---|---|---|
| 18 | 4 | Canopy, batched | k/4 | 16/14 | 0.01883 | 0.04349 | 0.06232 | 0.05806 |
| 18 | 4 | Canopy, batched | k | 3/1 | 0.02217 | 0.06745 | 0.08962 | 0.07053 |
| 18 | 4 | Canopy, batched | k/2 | 3/1 | 0.01982 | 0.05032 | 0.07014 | 0.06560 |
| 18 | 4 | Canopy, unbatched | k/4 | 16/14 | 0.01554 | 0.04497 | 0.06050 | 0.06044 |
| 18 | 4 | Canopy, unbatched | k | 1/14 | 0.02287 | 0.05009 | 0.07296 | 0.07013 |
| 18 | 4 | Canopy, unbatched | k/2 | 16/14 | 0.01543 | 0.04617 | 0.06161 | 0.06026 |
| 18 | 4 | Merkle, batched | - | - | 0.01490 | 0.04220 | 0.05709 | 0.05643 |
| 18 | 4 | Merkle, unbatched | - | - | 0.01507 | 0.04550 | 0.06057 | 0.06059 |
| 18 | 8 | Canopy, batched | k/4 | 15/13 | 0.01088 | 0.03697 | 0.04784 | 0.04817 |
| 18 | 8 | Canopy, batched | k | 3/1 | 0.01417 | 0.04194 | 0.05611 | 0.05563 |
| 18 | 8 | Canopy, batched | k/2 | 3/1 | 0.01343 | 0.03981 | 0.05324 | 0.05270 |
| 18 | 8 | Canopy, unbatched | k/4 | 15/13 | 0.01067 | 0.04000 | 0.05067 | 0.05025 |
| 18 | 8 | Canopy, unbatched | k | 15/13 | 0.01339 | 0.04396 | 0.05735 | 0.05425 |
| 18 | 8 | Canopy, unbatched | k/2 | 15/13 | 0.01020 | 0.03934 | 0.04955 | 0.04823 |
| 18 | 8 | Merkle, batched | - | - | 0.01075 | 0.03736 | 0.04811 | 0.04890 |
| 18 | 8 | Merkle, unbatched | - | - | 0.01111 | 0.03947 | 0.05058 | 0.05120 |
| 18 | 16 | Canopy, batched | k/4 | 14/12 | 0.00880 | 0.02940 | 0.03820 | 0.03768 |
| 18 | 16 | Canopy, batched | k | 3/1 | 0.01010 | 0.03170 | 0.04180 | 0.04157 |
| 18 | 16 | Canopy, batched | k/2 | 3/1 | 0.01260 | 0.03151 | 0.04411 | 0.04307 |
| 18 | 16 | Canopy, unbatched | k/4 | 14/12 | 0.00939 | 0.03136 | 0.04075 | 0.03980 |
| 18 | 16 | Canopy, unbatched | k | 14/12 | 0.00977 | 0.03121 | 0.04098 | 0.04103 |
| 18 | 16 | Canopy, unbatched | k/2 | 14/12 | 0.01088 | 0.03086 | 0.04175 | 0.04258 |
| 18 | 16 | Merkle, batched | - | - | 0.01009 | 0.02909 | 0.03918 | 0.03896 |
| 18 | 16 | Merkle, unbatched | - | - | 0.00902 | 0.03142 | 0.04044 | 0.03892 |
| 20 | 4 | Canopy, batched | k/4 | 3/1 | 0.07571 | 0.12351 | 0.19923 | 0.20054 |
| 20 | 4 | Canopy, batched | k | 3/1 | 0.08956 | 0.14191 | 0.23147 | 0.23214 |
| 20 | 4 | Canopy, batched | k/2 | 3/1 | 0.08012 | 0.12850 | 0.20862 | 0.21123 |
| 20 | 4 | Canopy, unbatched | k/4 | 3/16 | 0.06928 | 0.12959 | 0.19888 | 0.19733 |
| 20 | 4 | Canopy, unbatched | k | 1/16 | 0.09006 | 0.13421 | 0.22427 | 0.22601 |
| 20 | 4 | Canopy, unbatched | k/2 | 2/16 | 0.07773 | 0.12805 | 0.20579 | 0.20579 |
| 20 | 4 | Merkle, batched | - | - | 0.05927 | 0.11186 | 0.17112 | 0.17125 |
| 20 | 4 | Merkle, unbatched | - | - | 0.06118 | 0.12256 | 0.18374 | 0.18317 |
| 20 | 8 | Canopy, batched | k/4 | 17/15 | 0.04931 | 0.08454 | 0.13384 | 0.13233 |
| 20 | 8 | Canopy, batched | k | 3/1 | 0.06054 | 0.10006 | 0.16060 | 0.15947 |
| 20 | 8 | Canopy, batched | k/2 | 3/1 | 0.05497 | 0.09328 | 0.14825 | 0.14798 |
| 20 | 8 | Canopy, unbatched | k/4 | 17/15 | 0.05008 | 0.09084 | 0.14093 | 0.13860 |
| 20 | 8 | Canopy, unbatched | k | 1/15 | 0.06228 | 0.09672 | 0.15900 | 0.15838 |
| 20 | 8 | Canopy, unbatched | k/2 | 2/15 | 0.05599 | 0.09274 | 0.14873 | 0.14818 |
| 20 | 8 | Merkle, batched | - | - | 0.04847 | 0.08476 | 0.13323 | 0.13340 |
| 20 | 8 | Merkle, unbatched | - | - | 0.05004 | 0.09101 | 0.14106 | 0.14102 |
| 20 | 16 | Canopy, batched | k/4 | 16/14 | 0.04051 | 0.08992 | 0.13044 | 0.12843 |
| 20 | 16 | Canopy, batched | k | 3/1 | 0.04665 | 0.09559 | 0.14224 | 0.13995 |
| 20 | 16 | Canopy, batched | k/2 | 3/1 | 0.04648 | 0.09514 | 0.14162 | 0.13890 |
| 20 | 16 | Canopy, unbatched | k/4 | 16/14 | 0.04051 | 0.09258 | 0.13309 | 0.13257 |
| 20 | 16 | Canopy, unbatched | k | 1/14 | 0.04879 | 0.09444 | 0.14323 | 0.14294 |
| 20 | 16 | Canopy, unbatched | k/2 | 16/14 | 0.04281 | 0.09474 | 0.13754 | 0.13668 |
| 20 | 16 | Merkle, batched | - | - | 0.04963 | 0.08962 | 0.13926 | 0.13156 |
| 20 | 16 | Merkle, unbatched | - | - | 0.04266 | 0.09232 | 0.13499 | 0.13333 |
| 22 | 4 | Canopy, batched | k/4 | 3/1 | 0.32013 | 0.45633 | 0.77646 | 0.77819 |
| 22 | 4 | Canopy, batched | k | 3/1 | 0.38457 | 0.54270 | 0.92727 | 0.92596 |
| 22 | 4 | Canopy, batched | k/2 | 3/1 | 0.34093 | 0.48840 | 0.82934 | 0.82642 |
| 22 | 4 | Canopy, unbatched | k/4 | 3/18 | 0.29743 | 0.47786 | 0.77528 | 0.77461 |
| 22 | 4 | Canopy, unbatched | k | 1/18 | 0.39146 | 0.52294 | 0.91441 | 0.91447 |
| 22 | 4 | Canopy, unbatched | k/2 | 2/18 | 0.32545 | 0.48801 | 0.81346 | 0.81201 |
| 22 | 4 | Merkle, batched | - | - | 0.26157 | 0.41492 | 0.67649 | 0.68159 |
| 22 | 4 | Merkle, unbatched | - | - | 0.26594 | 0.46063 | 0.72657 | 0.73634 |
| 22 | 8 | Canopy, batched | k/4 | 3/1 | 0.23889 | 0.30034 | 0.53923 | 0.53713 |
| 22 | 8 | Canopy, batched | k | 3/1 | 0.27432 | 0.40187 | 0.67620 | 0.60267 |
| 22 | 8 | Canopy, batched | k/2 | 3/1 | 0.24996 | 0.31287 | 0.56283 | 0.55975 |
| 22 | 8 | Canopy, unbatched | k/4 | 3/17 | 0.23170 | 0.33415 | 0.56585 | 0.53832 |
| 22 | 8 | Canopy, unbatched | k | 1/17 | 0.27053 | 0.33642 | 0.60696 | 0.60523 |
| 22 | 8 | Canopy, unbatched | k/2 | 2/17 | 0.23941 | 0.32700 | 0.56641 | 0.55495 |
| 22 | 8 | Merkle, batched | - | - | 0.21025 | 0.27839 | 0.48864 | 0.49001 |
| 22 | 8 | Merkle, unbatched | - | - | 0.20872 | 0.30448 | 0.51320 | 0.51536 |
| 22 | 16 | Canopy, batched | k/4 | 3/1 | 0.19709 | 0.27960 | 0.47669 | 0.47437 |
| 22 | 16 | Canopy, batched | k | 3/1 | 0.20680 | 0.29056 | 0.49736 | 0.49532 |
| 22 | 16 | Canopy, batched | k/2 | 3/1 | 0.20163 | 0.28254 | 0.48417 | 0.48033 |
| 22 | 16 | Canopy, unbatched | k/4 | 3/16 | 0.18676 | 0.27929 | 0.46605 | 0.46611 |
| 22 | 16 | Canopy, unbatched | k | 1/16 | 0.20995 | 0.28483 | 0.49478 | 0.49582 |
| 22 | 16 | Canopy, unbatched | k/2 | 2/16 | 0.19541 | 0.28234 | 0.47776 | 0.47960 |
| 22 | 16 | Merkle, batched | - | - | 0.17868 | 0.26601 | 0.44469 | 0.44161 |
| 22 | 16 | Merkle, unbatched | - | - | 0.17813 | 0.27795 | 0.45608 | 0.45730 |

## Bolt: stages at the smallest-proof configurations

| n | Scheme | Phase | Part | Stage | Mean seconds |
|---|---|---|---|---|---|
| 18 | Canopy, batched | commitment | Bolt | parity RS encoding Y=RS(U) | 0.003231 |
| 18 | Canopy, batched | commitment | Bolt | sparse syndrome U=HX | 0.001973 |
| 18 | Canopy, batched | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.002197 |
| 18 | Canopy, batched | commitment | X | Canopy WHIR configuration | 0.000003 |
| 18 | Canopy, batched | commitment | X | Canopy hash-to-field | 0.001506 |
| 18 | Canopy, batched | commitment | X | row Merkle tree or forest | 0.007952 |
| 18 | Canopy, batched | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.002119 |
| 18 | Canopy, batched | commitment | Y | Canopy WHIR configuration | 0.000003 |
| 18 | Canopy, batched | commitment | Y | Canopy hash-to-field | 0.001517 |
| 18 | Canopy, batched | commitment | Y | row Merkle tree or forest | 0.001666 |
| 18 | Canopy, batched | commitment | other | unassigned phase work and timing overhead | 0.000002 |
| 18 | Canopy, batched | opening | Bolt | column projections x=Xr and u=Ur | 0.001092 |
| 18 | Canopy, batched | opening | Bolt | parity query covector and transpose RS FFT | 0.003586 |
| 18 | Canopy, batched | opening | Bolt | partial evaluation g | 0.009774 |
| 18 | Canopy, batched | opening | Bolt | project queried rows and sample batching challenges | 0.001447 |
| 18 | Canopy, batched | opening | Bolt | public H transpose covector | 0.001536 |
| 18 | Canopy, batched | opening | Bolt | sample row queries | 0.007577 |
| 18 | Canopy, batched | opening | Bolt | syndrome evaluation sigma | 0.000251 |
| 18 | Canopy, batched | opening | Bolt | systematic query covector | 0.000748 |
| 18 | Canopy, batched | opening | X | Canopy OOD evaluations | 0.001495 |
| 18 | Canopy, batched | opening | X | Canopy SEP batching covector | 0.000271 |
| 18 | Canopy, batched | opening | X | Canopy ancestor claims | 0.000099 |
| 18 | Canopy, batched | opening | X | authenticated WHIR alignment and constraint combination | 0.000246 |
| 18 | Canopy, batched | opening | X | collect and serialize row authentication paths | 0.004061 |
| 18 | Canopy, batched | opening | X | copy and serialize queried row values | 0.005015 |
| 18 | Canopy, batched | opening | Y | Canopy OOD evaluations | 0.000869 |
| 18 | Canopy, batched | opening | Y | Canopy SEP batching covector | 0.000018 |
| 18 | Canopy, batched | opening | Y | Canopy ancestor claims | 0.000004 |
| 18 | Canopy, batched | opening | Y | authenticated WHIR alignment and constraint combination | 0.000284 |
| 18 | Canopy, batched | opening | Y | collect and serialize row authentication paths | 0.000045 |
| 18 | Canopy, batched | opening | Y | copy and serialize queried row values | 0.000116 |
| 18 | Canopy, batched | opening | batched WHIR | distinct-functional cross evaluations | 0.002453 |
| 18 | Canopy, batched | opening | batched WHIR | shared recursive WHIR proof | 0.005568 |
| 18 | Canopy, batched | opening | common WHIR | WHIR configuration | 0.000004 |
| 18 | Canopy, batched | opening | other | unassigned phase work and timing overhead | 0.001005 |
| 18 | Canopy, batched | opening | u | authenticated WHIR alignment and constraint combination | 0.000440 |
| 18 | Canopy, batched | opening | u | common WHIR RS encode and oracle commitment | 0.002719 |
| 18 | Canopy, batched | opening | x | authenticated WHIR alignment and constraint combination | 0.010218 |
| 18 | Canopy, batched | opening | x | common WHIR RS encode and oracle commitment | 0.006514 |
| 18 | Canopy, unbatched | commitment | Bolt | parity RS encoding Y=RS(U) | 0.002744 |
| 18 | Canopy, unbatched | commitment | Bolt | sparse syndrome U=HX | 0.001813 |
| 18 | Canopy, unbatched | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.002651 |
| 18 | Canopy, unbatched | commitment | X | Canopy WHIR configuration | 0.000004 |
| 18 | Canopy, unbatched | commitment | X | Canopy hash-to-field | 0.006066 |
| 18 | Canopy, unbatched | commitment | X | row Merkle tree or forest | 0.006544 |
| 18 | Canopy, unbatched | commitment | Y | Canopy constant RS commitment | 0.000004 |
| 18 | Canopy, unbatched | commitment | Y | Canopy hash-to-field | 0.000000 |
| 18 | Canopy, unbatched | commitment | Y | row Merkle tree or forest | 0.003039 |
| 18 | Canopy, unbatched | commitment | other | unassigned phase work and timing overhead | 0.000002 |
| 18 | Canopy, unbatched | opening | Bolt | column projections x=Xr and u=Ur | 0.001077 |
| 18 | Canopy, unbatched | opening | Bolt | parity query covector and transpose RS FFT | 0.001281 |
| 18 | Canopy, unbatched | opening | Bolt | partial evaluation g | 0.004060 |
| 18 | Canopy, unbatched | opening | Bolt | project queried rows and sample batching challenges | 0.001280 |
| 18 | Canopy, unbatched | opening | Bolt | public H transpose covector | 0.001099 |
| 18 | Canopy, unbatched | opening | Bolt | sample row queries | 0.007235 |
| 18 | Canopy, unbatched | opening | Bolt | syndrome evaluation sigma | 0.000252 |
| 18 | Canopy, unbatched | opening | Bolt | systematic query covector | 0.000656 |
| 18 | Canopy, unbatched | opening | X | Canopy OOD evaluations | 0.000226 |
| 18 | Canopy, unbatched | opening | X | Canopy SEP batching covector | 0.000483 |
| 18 | Canopy, unbatched | opening | X | Canopy WHIR SEP proof | 0.005365 |
| 18 | Canopy, unbatched | opening | X | Canopy ancestor claims | 0.000094 |
| 18 | Canopy, unbatched | opening | X | collect and serialize row authentication paths | 0.002058 |
| 18 | Canopy, unbatched | opening | X | copy and serialize queried row values | 0.004899 |
| 18 | Canopy, unbatched | opening | Y | Canopy ancestor claims | 0.000001 |
| 18 | Canopy, unbatched | opening | Y | Canopy constant opening | 0.000000 |
| 18 | Canopy, unbatched | opening | Y | collect and serialize row authentication paths | 0.000197 |
| 18 | Canopy, unbatched | opening | Y | copy and serialize queried row values | 0.000094 |
| 18 | Canopy, unbatched | opening | common WHIR | WHIR configuration | 0.000005 |
| 18 | Canopy, unbatched | opening | other | unassigned phase work and timing overhead | 0.000125 |
| 18 | Canopy, unbatched | opening | u | common WHIR RS encode and oracle commitment | 0.000979 |
| 18 | Canopy, unbatched | opening | u | common WHIR constrained proof | 0.002351 |
| 18 | Canopy, unbatched | opening | x | common WHIR RS encode and oracle commitment | 0.005079 |
| 18 | Canopy, unbatched | opening | x | common WHIR constrained proof | 0.011199 |
| 18 | Merkle, batched | commitment | Bolt | parity RS encoding Y=RS(U) | 0.002572 |
| 18 | Merkle, batched | commitment | Bolt | sparse syndrome U=HX | 0.001794 |
| 18 | Merkle, batched | commitment | X | Merkle root transcript | 0.000000 |
| 18 | Merkle, batched | commitment | X | row Merkle tree or forest | 0.008114 |
| 18 | Merkle, batched | commitment | Y | Merkle root transcript | 0.000001 |
| 18 | Merkle, batched | commitment | Y | row Merkle tree or forest | 0.002414 |
| 18 | Merkle, batched | commitment | other | unassigned phase work and timing overhead | 0.000002 |
| 18 | Merkle, batched | opening | Bolt | column projections x=Xr and u=Ur | 0.001130 |
| 18 | Merkle, batched | opening | Bolt | parity query covector and transpose RS FFT | 0.001217 |
| 18 | Merkle, batched | opening | Bolt | partial evaluation g | 0.004019 |
| 18 | Merkle, batched | opening | Bolt | project queried rows and sample batching challenges | 0.001156 |
| 18 | Merkle, batched | opening | Bolt | public H transpose covector | 0.001092 |
| 18 | Merkle, batched | opening | Bolt | sample row queries | 0.007139 |
| 18 | Merkle, batched | opening | Bolt | syndrome evaluation sigma | 0.000247 |
| 18 | Merkle, batched | opening | Bolt | systematic query covector | 0.000610 |
| 18 | Merkle, batched | opening | X | collect and serialize row authentication paths | 0.004734 |
| 18 | Merkle, batched | opening | X | copy and serialize queried row values | 0.005095 |
| 18 | Merkle, batched | opening | Y | collect and serialize row authentication paths | 0.000215 |
| 18 | Merkle, batched | opening | Y | copy and serialize queried row values | 0.000088 |
| 18 | Merkle, batched | opening | batched WHIR | distinct-functional cross evaluations | 0.000411 |
| 18 | Merkle, batched | opening | batched WHIR | shared recursive WHIR proof | 0.002571 |
| 18 | Merkle, batched | opening | common WHIR | WHIR configuration | 0.000005 |
| 18 | Merkle, batched | opening | other | unassigned phase work and timing overhead | 0.000352 |
| 18 | Merkle, batched | opening | u | authenticated WHIR alignment and constraint combination | 0.000399 |
| 18 | Merkle, batched | opening | u | common WHIR RS encode and oracle commitment | 0.000980 |
| 18 | Merkle, batched | opening | x | authenticated WHIR alignment and constraint combination | 0.005863 |
| 18 | Merkle, batched | opening | x | common WHIR RS encode and oracle commitment | 0.004874 |
| 18 | Merkle, unbatched | commitment | Bolt | parity RS encoding Y=RS(U) | 0.002779 |
| 18 | Merkle, unbatched | commitment | Bolt | sparse syndrome U=HX | 0.002057 |
| 18 | Merkle, unbatched | commitment | X | Merkle root transcript | 0.000000 |
| 18 | Merkle, unbatched | commitment | X | row Merkle tree or forest | 0.007889 |
| 18 | Merkle, unbatched | commitment | Y | Merkle root transcript | 0.000001 |
| 18 | Merkle, unbatched | commitment | Y | row Merkle tree or forest | 0.002337 |
| 18 | Merkle, unbatched | commitment | other | unassigned phase work and timing overhead | 0.000003 |
| 18 | Merkle, unbatched | opening | Bolt | column projections x=Xr and u=Ur | 0.001099 |
| 18 | Merkle, unbatched | opening | Bolt | parity query covector and transpose RS FFT | 0.001174 |
| 18 | Merkle, unbatched | opening | Bolt | partial evaluation g | 0.003796 |
| 18 | Merkle, unbatched | opening | Bolt | project queried rows and sample batching challenges | 0.001117 |
| 18 | Merkle, unbatched | opening | Bolt | public H transpose covector | 0.001174 |
| 18 | Merkle, unbatched | opening | Bolt | sample row queries | 0.007327 |
| 18 | Merkle, unbatched | opening | Bolt | syndrome evaluation sigma | 0.000240 |
| 18 | Merkle, unbatched | opening | Bolt | systematic query covector | 0.000618 |
| 18 | Merkle, unbatched | opening | X | collect and serialize row authentication paths | 0.004775 |
| 18 | Merkle, unbatched | opening | X | copy and serialize queried row values | 0.005065 |
| 18 | Merkle, unbatched | opening | Y | collect and serialize row authentication paths | 0.000230 |
| 18 | Merkle, unbatched | opening | Y | copy and serialize queried row values | 0.000089 |
| 18 | Merkle, unbatched | opening | common WHIR | WHIR configuration | 0.000005 |
| 18 | Merkle, unbatched | opening | other | unassigned phase work and timing overhead | 0.000061 |
| 18 | Merkle, unbatched | opening | u | common WHIR RS encode and oracle commitment | 0.000839 |
| 18 | Merkle, unbatched | opening | u | common WHIR constrained proof | 0.002164 |
| 18 | Merkle, unbatched | opening | x | common WHIR RS encode and oracle commitment | 0.004978 |
| 18 | Merkle, unbatched | opening | x | common WHIR constrained proof | 0.010747 |
| 20 | Canopy, batched | commitment | Bolt | parity RS encoding Y=RS(U) | 0.011188 |
| 20 | Canopy, batched | commitment | Bolt | sparse syndrome U=HX | 0.013436 |
| 20 | Canopy, batched | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.008656 |
| 20 | Canopy, batched | commitment | X | Canopy WHIR configuration | 0.000005 |
| 20 | Canopy, batched | commitment | X | Canopy hash-to-field | 0.006188 |
| 20 | Canopy, batched | commitment | X | row Merkle tree or forest | 0.028173 |
| 20 | Canopy, batched | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.008738 |
| 20 | Canopy, batched | commitment | Y | Canopy WHIR configuration | 0.000005 |
| 20 | Canopy, batched | commitment | Y | Canopy hash-to-field | 0.006545 |
| 20 | Canopy, batched | commitment | Y | row Merkle tree or forest | 0.006622 |
| 20 | Canopy, batched | commitment | other | unassigned phase work and timing overhead | 0.000002 |
| 20 | Canopy, batched | opening | Bolt | column projections x=Xr and u=Ur | 0.004400 |
| 20 | Canopy, batched | opening | Bolt | parity query covector and transpose RS FFT | 0.002927 |
| 20 | Canopy, batched | opening | Bolt | partial evaluation g | 0.015960 |
| 20 | Canopy, batched | opening | Bolt | project queried rows and sample batching challenges | 0.001162 |
| 20 | Canopy, batched | opening | Bolt | public H transpose covector | 0.006574 |
| 20 | Canopy, batched | opening | Bolt | sample row queries | 0.008541 |
| 20 | Canopy, batched | opening | Bolt | syndrome evaluation sigma | 0.000758 |
| 20 | Canopy, batched | opening | Bolt | systematic query covector | 0.001622 |
| 20 | Canopy, batched | opening | X | Canopy OOD evaluations | 0.000254 |
| 20 | Canopy, batched | opening | X | Canopy SEP batching covector | 0.000435 |
| 20 | Canopy, batched | opening | X | Canopy ancestor claims | 0.000113 |
| 20 | Canopy, batched | opening | X | authenticated WHIR alignment and constraint combination | 0.000900 |
| 20 | Canopy, batched | opening | X | collect and serialize row authentication paths | 0.006848 |
| 20 | Canopy, batched | opening | X | copy and serialize queried row values | 0.006348 |
| 20 | Canopy, batched | opening | Y | Canopy OOD evaluations | 0.000228 |
| 20 | Canopy, batched | opening | Y | Canopy SEP batching covector | 0.000022 |
| 20 | Canopy, batched | opening | Y | Canopy ancestor claims | 0.000004 |
| 20 | Canopy, batched | opening | Y | authenticated WHIR alignment and constraint combination | 0.000823 |
| 20 | Canopy, batched | opening | Y | collect and serialize row authentication paths | 0.000044 |
| 20 | Canopy, batched | opening | Y | copy and serialize queried row values | 0.000094 |
| 20 | Canopy, batched | opening | batched WHIR | distinct-functional cross evaluations | 0.010241 |
| 20 | Canopy, batched | opening | batched WHIR | shared recursive WHIR proof | 0.014818 |
| 20 | Canopy, batched | opening | common WHIR | WHIR configuration | 0.000007 |
| 20 | Canopy, batched | opening | other | unassigned phase work and timing overhead | 0.003785 |
| 20 | Canopy, batched | opening | u | authenticated WHIR alignment and constraint combination | 0.001313 |
| 20 | Canopy, batched | opening | u | common WHIR RS encode and oracle commitment | 0.009353 |
| 20 | Canopy, batched | opening | x | authenticated WHIR alignment and constraint combination | 0.024460 |
| 20 | Canopy, batched | opening | x | common WHIR RS encode and oracle commitment | 0.019877 |
| 20 | Canopy, unbatched | commitment | Bolt | parity RS encoding Y=RS(U) | 0.009968 |
| 20 | Canopy, unbatched | commitment | Bolt | sparse syndrome U=HX | 0.012083 |
| 20 | Canopy, unbatched | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.010686 |
| 20 | Canopy, unbatched | commitment | X | Canopy WHIR configuration | 0.000006 |
| 20 | Canopy, unbatched | commitment | X | Canopy hash-to-field | 0.025322 |
| 20 | Canopy, unbatched | commitment | X | row Merkle tree or forest | 0.024578 |
| 20 | Canopy, unbatched | commitment | Y | Canopy constant RS commitment | 0.000005 |
| 20 | Canopy, unbatched | commitment | Y | Canopy hash-to-field | 0.000000 |
| 20 | Canopy, unbatched | commitment | Y | row Merkle tree or forest | 0.007414 |
| 20 | Canopy, unbatched | commitment | other | unassigned phase work and timing overhead | 0.000002 |
| 20 | Canopy, unbatched | opening | Bolt | column projections x=Xr and u=Ur | 0.003973 |
| 20 | Canopy, unbatched | opening | Bolt | parity query covector and transpose RS FFT | 0.002807 |
| 20 | Canopy, unbatched | opening | Bolt | partial evaluation g | 0.015944 |
| 20 | Canopy, unbatched | opening | Bolt | project queried rows and sample batching challenges | 0.001141 |
| 20 | Canopy, unbatched | opening | Bolt | public H transpose covector | 0.005469 |
| 20 | Canopy, unbatched | opening | Bolt | sample row queries | 0.008473 |
| 20 | Canopy, unbatched | opening | Bolt | syndrome evaluation sigma | 0.000682 |
| 20 | Canopy, unbatched | opening | Bolt | systematic query covector | 0.001432 |
| 20 | Canopy, unbatched | opening | X | Canopy OOD evaluations | 0.000558 |
| 20 | Canopy, unbatched | opening | X | Canopy SEP batching covector | 0.000747 |
| 20 | Canopy, unbatched | opening | X | Canopy WHIR SEP proof | 0.016047 |
| 20 | Canopy, unbatched | opening | X | Canopy ancestor claims | 0.000126 |
| 20 | Canopy, unbatched | opening | X | collect and serialize row authentication paths | 0.002879 |
| 20 | Canopy, unbatched | opening | X | copy and serialize queried row values | 0.005976 |
| 20 | Canopy, unbatched | opening | Y | Canopy ancestor claims | 0.000001 |
| 20 | Canopy, unbatched | opening | Y | Canopy constant opening | 0.000000 |
| 20 | Canopy, unbatched | opening | Y | collect and serialize row authentication paths | 0.000259 |
| 20 | Canopy, unbatched | opening | Y | copy and serialize queried row values | 0.000095 |
| 20 | Canopy, unbatched | opening | common WHIR | WHIR configuration | 0.000008 |
| 20 | Canopy, unbatched | opening | other | unassigned phase work and timing overhead | 0.000668 |
| 20 | Canopy, unbatched | opening | u | common WHIR RS encode and oracle commitment | 0.003118 |
| 20 | Canopy, unbatched | opening | u | common WHIR constrained proof | 0.006112 |
| 20 | Canopy, unbatched | opening | x | common WHIR RS encode and oracle commitment | 0.019585 |
| 20 | Canopy, unbatched | opening | x | common WHIR constrained proof | 0.038104 |
| 20 | Merkle, batched | commitment | Bolt | parity RS encoding Y=RS(U) | 0.009780 |
| 20 | Merkle, batched | commitment | Bolt | sparse syndrome U=HX | 0.012531 |
| 20 | Merkle, batched | commitment | X | Merkle root transcript | 0.000000 |
| 20 | Merkle, batched | commitment | X | row Merkle tree or forest | 0.028874 |
| 20 | Merkle, batched | commitment | Y | Merkle root transcript | 0.000001 |
| 20 | Merkle, batched | commitment | Y | row Merkle tree or forest | 0.008079 |
| 20 | Merkle, batched | commitment | other | unassigned phase work and timing overhead | 0.000002 |
| 20 | Merkle, batched | opening | Bolt | column projections x=Xr and u=Ur | 0.003876 |
| 20 | Merkle, batched | opening | Bolt | parity query covector and transpose RS FFT | 0.002772 |
| 20 | Merkle, batched | opening | Bolt | partial evaluation g | 0.015685 |
| 20 | Merkle, batched | opening | Bolt | project queried rows and sample batching challenges | 0.001188 |
| 20 | Merkle, batched | opening | Bolt | public H transpose covector | 0.005755 |
| 20 | Merkle, batched | opening | Bolt | sample row queries | 0.008367 |
| 20 | Merkle, batched | opening | Bolt | syndrome evaluation sigma | 0.000704 |
| 20 | Merkle, batched | opening | Bolt | systematic query covector | 0.001527 |
| 20 | Merkle, batched | opening | X | collect and serialize row authentication paths | 0.010018 |
| 20 | Merkle, batched | opening | X | copy and serialize queried row values | 0.006026 |
| 20 | Merkle, batched | opening | Y | collect and serialize row authentication paths | 0.000286 |
| 20 | Merkle, batched | opening | Y | copy and serialize queried row values | 0.000094 |
| 20 | Merkle, batched | opening | batched WHIR | distinct-functional cross evaluations | 0.001632 |
| 20 | Merkle, batched | opening | batched WHIR | shared recursive WHIR proof | 0.007779 |
| 20 | Merkle, batched | opening | common WHIR | WHIR configuration | 0.000007 |
| 20 | Merkle, batched | opening | other | unassigned phase work and timing overhead | 0.002116 |
| 20 | Merkle, batched | opening | u | authenticated WHIR alignment and constraint combination | 0.001234 |
| 20 | Merkle, batched | opening | u | common WHIR RS encode and oracle commitment | 0.003090 |
| 20 | Merkle, batched | opening | x | authenticated WHIR alignment and constraint combination | 0.019469 |
| 20 | Merkle, batched | opening | x | common WHIR RS encode and oracle commitment | 0.020230 |
| 20 | Merkle, unbatched | commitment | Bolt | parity RS encoding Y=RS(U) | 0.010462 |
| 20 | Merkle, unbatched | commitment | Bolt | sparse syndrome U=HX | 0.013379 |
| 20 | Merkle, unbatched | commitment | X | Merkle root transcript | 0.000000 |
| 20 | Merkle, unbatched | commitment | X | row Merkle tree or forest | 0.029450 |
| 20 | Merkle, unbatched | commitment | Y | Merkle root transcript | 0.000001 |
| 20 | Merkle, unbatched | commitment | Y | row Merkle tree or forest | 0.007881 |
| 20 | Merkle, unbatched | commitment | other | unassigned phase work and timing overhead | 0.000002 |
| 20 | Merkle, unbatched | opening | Bolt | column projections x=Xr and u=Ur | 0.004292 |
| 20 | Merkle, unbatched | opening | Bolt | parity query covector and transpose RS FFT | 0.002791 |
| 20 | Merkle, unbatched | opening | Bolt | partial evaluation g | 0.016027 |
| 20 | Merkle, unbatched | opening | Bolt | project queried rows and sample batching challenges | 0.001193 |
| 20 | Merkle, unbatched | opening | Bolt | public H transpose covector | 0.005240 |
| 20 | Merkle, unbatched | opening | Bolt | sample row queries | 0.008443 |
| 20 | Merkle, unbatched | opening | Bolt | syndrome evaluation sigma | 0.000658 |
| 20 | Merkle, unbatched | opening | Bolt | systematic query covector | 0.001513 |
| 20 | Merkle, unbatched | opening | X | collect and serialize row authentication paths | 0.009563 |
| 20 | Merkle, unbatched | opening | X | copy and serialize queried row values | 0.005974 |
| 20 | Merkle, unbatched | opening | Y | collect and serialize row authentication paths | 0.000304 |
| 20 | Merkle, unbatched | opening | Y | copy and serialize queried row values | 0.000094 |
| 20 | Merkle, unbatched | opening | common WHIR | WHIR configuration | 0.000008 |
| 20 | Merkle, unbatched | opening | other | unassigned phase work and timing overhead | 0.000442 |
| 20 | Merkle, unbatched | opening | u | common WHIR RS encode and oracle commitment | 0.003220 |
| 20 | Merkle, unbatched | opening | u | common WHIR constrained proof | 0.006556 |
| 20 | Merkle, unbatched | opening | x | common WHIR RS encode and oracle commitment | 0.020022 |
| 20 | Merkle, unbatched | opening | x | common WHIR constrained proof | 0.036222 |
| 22 | Canopy, batched | commitment | Bolt | parity RS encoding Y=RS(U) | 0.044421 |
| 22 | Canopy, batched | commitment | Bolt | sparse syndrome U=HX | 0.070506 |
| 22 | Canopy, batched | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.038488 |
| 22 | Canopy, batched | commitment | X | Canopy WHIR configuration | 0.000007 |
| 22 | Canopy, batched | commitment | X | Canopy hash-to-field | 0.026916 |
| 22 | Canopy, batched | commitment | X | row Merkle tree or forest | 0.112622 |
| 22 | Canopy, batched | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.036446 |
| 22 | Canopy, batched | commitment | Y | Canopy WHIR configuration | 0.000008 |
| 22 | Canopy, batched | commitment | Y | Canopy hash-to-field | 0.028120 |
| 22 | Canopy, batched | commitment | Y | row Merkle tree or forest | 0.027032 |
| 22 | Canopy, batched | commitment | other | unassigned phase work and timing overhead | 0.000003 |
| 22 | Canopy, batched | opening | Bolt | column projections x=Xr and u=Ur | 0.014328 |
| 22 | Canopy, batched | opening | Bolt | parity query covector and transpose RS FFT | 0.012670 |
| 22 | Canopy, batched | opening | Bolt | partial evaluation g | 0.066646 |
| 22 | Canopy, batched | opening | Bolt | project queried rows and sample batching challenges | 0.001290 |
| 22 | Canopy, batched | opening | Bolt | public H transpose covector | 0.056974 |
| 22 | Canopy, batched | opening | Bolt | sample row queries | 0.009102 |
| 22 | Canopy, batched | opening | Bolt | syndrome evaluation sigma | 0.002838 |
| 22 | Canopy, batched | opening | Bolt | systematic query covector | 0.004602 |
| 22 | Canopy, batched | opening | X | Canopy OOD evaluations | 0.000659 |
| 22 | Canopy, batched | opening | X | Canopy SEP batching covector | 0.000794 |
| 22 | Canopy, batched | opening | X | Canopy ancestor claims | 0.000159 |
| 22 | Canopy, batched | opening | X | authenticated WHIR alignment and constraint combination | 0.003133 |
| 22 | Canopy, batched | opening | X | collect and serialize row authentication paths | 0.009375 |
| 22 | Canopy, batched | opening | X | copy and serialize queried row values | 0.006584 |
| 22 | Canopy, batched | opening | Y | Canopy OOD evaluations | 0.000590 |
| 22 | Canopy, batched | opening | Y | Canopy SEP batching covector | 0.000160 |
| 22 | Canopy, batched | opening | Y | Canopy ancestor claims | 0.000005 |
| 22 | Canopy, batched | opening | Y | authenticated WHIR alignment and constraint combination | 0.002923 |
| 22 | Canopy, batched | opening | Y | collect and serialize row authentication paths | 0.000049 |
| 22 | Canopy, batched | opening | Y | copy and serialize queried row values | 0.000109 |
| 22 | Canopy, batched | opening | batched WHIR | distinct-functional cross evaluations | 0.043256 |
| 22 | Canopy, batched | opening | batched WHIR | shared recursive WHIR proof | 0.056008 |
| 22 | Canopy, batched | opening | common WHIR | WHIR configuration | 0.000010 |
| 22 | Canopy, batched | opening | other | unassigned phase work and timing overhead | 0.012276 |
| 22 | Canopy, batched | opening | u | authenticated WHIR alignment and constraint combination | 0.003866 |
| 22 | Canopy, batched | opening | u | common WHIR RS encode and oracle commitment | 0.037961 |
| 22 | Canopy, batched | opening | x | authenticated WHIR alignment and constraint combination | 0.101463 |
| 22 | Canopy, batched | opening | x | common WHIR RS encode and oracle commitment | 0.094871 |
| 22 | Canopy, unbatched | commitment | Bolt | parity RS encoding Y=RS(U) | 0.041817 |
| 22 | Canopy, unbatched | commitment | Bolt | sparse syndrome U=HX | 0.072231 |
| 22 | Canopy, unbatched | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.045702 |
| 22 | Canopy, unbatched | commitment | X | Canopy WHIR configuration | 0.000007 |
| 22 | Canopy, unbatched | commitment | X | Canopy hash-to-field | 0.105551 |
| 22 | Canopy, unbatched | commitment | X | row Merkle tree or forest | 0.096829 |
| 22 | Canopy, unbatched | commitment | Y | Canopy constant RS commitment | 0.000007 |
| 22 | Canopy, unbatched | commitment | Y | Canopy hash-to-field | 0.000000 |
| 22 | Canopy, unbatched | commitment | Y | row Merkle tree or forest | 0.029313 |
| 22 | Canopy, unbatched | commitment | other | unassigned phase work and timing overhead | 0.000006 |
| 22 | Canopy, unbatched | opening | Bolt | column projections x=Xr and u=Ur | 0.016569 |
| 22 | Canopy, unbatched | opening | Bolt | parity query covector and transpose RS FFT | 0.011370 |
| 22 | Canopy, unbatched | opening | Bolt | partial evaluation g | 0.066008 |
| 22 | Canopy, unbatched | opening | Bolt | project queried rows and sample batching challenges | 0.001217 |
| 22 | Canopy, unbatched | opening | Bolt | public H transpose covector | 0.057451 |
| 22 | Canopy, unbatched | opening | Bolt | sample row queries | 0.008440 |
| 22 | Canopy, unbatched | opening | Bolt | syndrome evaluation sigma | 0.002548 |
| 22 | Canopy, unbatched | opening | Bolt | systematic query covector | 0.004283 |
| 22 | Canopy, unbatched | opening | X | Canopy OOD evaluations | 0.001924 |
| 22 | Canopy, unbatched | opening | X | Canopy SEP batching covector | 0.002214 |
| 22 | Canopy, unbatched | opening | X | Canopy WHIR SEP proof | 0.065103 |
| 22 | Canopy, unbatched | opening | X | Canopy ancestor claims | 0.000244 |
| 22 | Canopy, unbatched | opening | X | collect and serialize row authentication paths | 0.003357 |
| 22 | Canopy, unbatched | opening | X | copy and serialize queried row values | 0.006367 |
| 22 | Canopy, unbatched | opening | Y | Canopy ancestor claims | 0.000001 |
| 22 | Canopy, unbatched | opening | Y | Canopy constant opening | 0.000000 |
| 22 | Canopy, unbatched | opening | Y | collect and serialize row authentication paths | 0.000533 |
| 22 | Canopy, unbatched | opening | Y | copy and serialize queried row values | 0.000100 |
| 22 | Canopy, unbatched | opening | common WHIR | WHIR configuration | 0.000009 |
| 22 | Canopy, unbatched | opening | other | unassigned phase work and timing overhead | 0.002024 |
| 22 | Canopy, unbatched | opening | u | common WHIR RS encode and oracle commitment | 0.011562 |
| 22 | Canopy, unbatched | opening | u | common WHIR constrained proof | 0.020775 |
| 22 | Canopy, unbatched | opening | x | common WHIR RS encode and oracle commitment | 0.094223 |
| 22 | Canopy, unbatched | opening | x | common WHIR constrained proof | 0.146621 |
| 22 | Merkle, batched | commitment | Bolt | parity RS encoding Y=RS(U) | 0.041980 |
| 22 | Merkle, batched | commitment | Bolt | sparse syndrome U=HX | 0.069750 |
| 22 | Merkle, batched | commitment | X | Merkle root transcript | 0.000000 |
| 22 | Merkle, batched | commitment | X | row Merkle tree or forest | 0.117520 |
| 22 | Merkle, batched | commitment | Y | Merkle root transcript | 0.000001 |
| 22 | Merkle, batched | commitment | Y | row Merkle tree or forest | 0.032317 |
| 22 | Merkle, batched | commitment | other | unassigned phase work and timing overhead | 0.000002 |
| 22 | Merkle, batched | opening | Bolt | column projections x=Xr and u=Ur | 0.014317 |
| 22 | Merkle, batched | opening | Bolt | parity query covector and transpose RS FFT | 0.011850 |
| 22 | Merkle, batched | opening | Bolt | partial evaluation g | 0.066756 |
| 22 | Merkle, batched | opening | Bolt | project queried rows and sample batching challenges | 0.001311 |
| 22 | Merkle, batched | opening | Bolt | public H transpose covector | 0.057757 |
| 22 | Merkle, batched | opening | Bolt | sample row queries | 0.009117 |
| 22 | Merkle, batched | opening | Bolt | syndrome evaluation sigma | 0.002751 |
| 22 | Merkle, batched | opening | Bolt | systematic query covector | 0.004660 |
| 22 | Merkle, batched | opening | X | collect and serialize row authentication paths | 0.016303 |
| 22 | Merkle, batched | opening | X | copy and serialize queried row values | 0.006278 |
| 22 | Merkle, batched | opening | Y | collect and serialize row authentication paths | 0.000370 |
| 22 | Merkle, batched | opening | Y | copy and serialize queried row values | 0.000105 |
| 22 | Merkle, batched | opening | batched WHIR | distinct-functional cross evaluations | 0.007292 |
| 22 | Merkle, batched | opening | batched WHIR | shared recursive WHIR proof | 0.022994 |
| 22 | Merkle, batched | opening | common WHIR | WHIR configuration | 0.000014 |
| 22 | Merkle, batched | opening | other | unassigned phase work and timing overhead | 0.007817 |
| 22 | Merkle, batched | opening | u | authenticated WHIR alignment and constraint combination | 0.003592 |
| 22 | Merkle, batched | opening | u | common WHIR RS encode and oracle commitment | 0.011888 |
| 22 | Merkle, batched | opening | x | authenticated WHIR alignment and constraint combination | 0.074241 |
| 22 | Merkle, batched | opening | x | common WHIR RS encode and oracle commitment | 0.095501 |
| 22 | Merkle, unbatched | commitment | Bolt | parity RS encoding Y=RS(U) | 0.042807 |
| 22 | Merkle, unbatched | commitment | Bolt | sparse syndrome U=HX | 0.071843 |
| 22 | Merkle, unbatched | commitment | X | Merkle root transcript | 0.000000 |
| 22 | Merkle, unbatched | commitment | X | row Merkle tree or forest | 0.119753 |
| 22 | Merkle, unbatched | commitment | Y | Merkle root transcript | 0.000002 |
| 22 | Merkle, unbatched | commitment | Y | row Merkle tree or forest | 0.031527 |
| 22 | Merkle, unbatched | commitment | other | unassigned phase work and timing overhead | 0.000002 |
| 22 | Merkle, unbatched | opening | Bolt | column projections x=Xr and u=Ur | 0.014514 |
| 22 | Merkle, unbatched | opening | Bolt | parity query covector and transpose RS FFT | 0.012247 |
| 22 | Merkle, unbatched | opening | Bolt | partial evaluation g | 0.065646 |
| 22 | Merkle, unbatched | opening | Bolt | project queried rows and sample batching challenges | 0.001274 |
| 22 | Merkle, unbatched | opening | Bolt | public H transpose covector | 0.056363 |
| 22 | Merkle, unbatched | opening | Bolt | sample row queries | 0.009042 |
| 22 | Merkle, unbatched | opening | Bolt | syndrome evaluation sigma | 0.002887 |
| 22 | Merkle, unbatched | opening | Bolt | systematic query covector | 0.003787 |
| 22 | Merkle, unbatched | opening | X | collect and serialize row authentication paths | 0.016574 |
| 22 | Merkle, unbatched | opening | X | copy and serialize queried row values | 0.006332 |
| 22 | Merkle, unbatched | opening | Y | collect and serialize row authentication paths | 0.000395 |
| 22 | Merkle, unbatched | opening | Y | copy and serialize queried row values | 0.000117 |
| 22 | Merkle, unbatched | opening | common WHIR | WHIR configuration | 0.000010 |
| 22 | Merkle, unbatched | opening | other | unassigned phase work and timing overhead | 0.001371 |
| 22 | Merkle, unbatched | opening | u | common WHIR RS encode and oracle commitment | 0.011630 |
| 22 | Merkle, unbatched | opening | u | common WHIR constrained proof | 0.019581 |
| 22 | Merkle, unbatched | opening | x | common WHIR RS encode and oracle commitment | 0.094273 |
| 22 | Merkle, unbatched | opening | x | common WHIR constrained proof | 0.144589 |
