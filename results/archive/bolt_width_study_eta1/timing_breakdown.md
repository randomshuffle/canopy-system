# GF(2^256) prover-time breakdown

Arithmetic means in seconds; phase means add to total means. Every measured repetition remains in the raw CSV files. WHIR is a component estimator: separately measured tail subtraction and clamped duration residuals can make substages overlap. The signed separately_measured_component_adjustment reconciles these substages to the unchanged reported total; it is a measurement-model adjustment, not negative prover work. Its median is unavailable when recovered from saved stage aggregates. Bolt stages partition each measured execution directly and need no adjustment.

## WHIR: commitment and opening

| n | t | Scheme | Commit | Opening | Total | Median total |
|---|---|---|---|---|---|---|
| 18 | 1 | WHIR batched 1/2 | 0.02006 | 0.03158 | 0.05165 | 0.05290 |
| 18 | 1 | WHIR batched 1/4 | 0.04070 | 0.04780 | 0.08851 | 0.07515 |
| 18 | 1 | Canopy Exp. 3 | 0.03240 | 0.03558 | 0.06798 | 0.06423 |
| 18 | 2 | WHIR batched 1/2 | 0.04013 | 0.05440 | 0.09453 | 0.09059 |
| 18 | 2 | WHIR batched 1/4 | 0.08141 | 0.08249 | 0.16389 | 0.13740 |
| 18 | 2 | Canopy Exp. 3 | 0.06151 | 0.05400 | 0.11551 | 0.10789 |
| 18 | 4 | WHIR batched 1/2 | 0.08025 | 0.10048 | 0.18074 | 0.17603 |
| 18 | 4 | WHIR batched 1/4 | 0.16282 | 0.14713 | 0.30995 | 0.25740 |
| 18 | 4 | Canopy Exp. 3 | 0.11434 | 0.09939 | 0.21372 | 0.19133 |
| 18 | 8 | WHIR batched 1/2 | 0.16051 | 0.19981 | 0.36032 | 0.35265 |
| 18 | 8 | WHIR batched 1/4 | 0.32563 | 0.28726 | 0.61289 | 0.50697 |
| 18 | 8 | Canopy Exp. 3 | 0.22122 | 0.21729 | 0.43851 | 0.42301 |
| 20 | 1 | WHIR batched 1/2 | 0.09596 | 0.11897 | 0.21492 | 0.20787 |
| 20 | 1 | WHIR batched 1/4 | 0.15612 | 0.18905 | 0.34517 | 0.33707 |
| 20 | 1 | Canopy Exp. 3 | 0.14231 | 0.12994 | 0.27226 | 0.27004 |
| 20 | 2 | WHIR batched 1/2 | 0.19191 | 0.20360 | 0.39551 | 0.36868 |
| 20 | 2 | WHIR batched 1/4 | 0.31224 | 0.31364 | 0.62588 | 0.62648 |
| 20 | 2 | Canopy Exp. 3 | 0.26845 | 0.22617 | 0.49462 | 0.47741 |
| 20 | 4 | WHIR batched 1/2 | 0.38382 | 0.36800 | 0.75183 | 0.68633 |
| 20 | 4 | WHIR batched 1/4 | 0.62448 | 0.51723 | 1.14170 | 1.11135 |
| 20 | 4 | Canopy Exp. 3 | 0.57534 | 0.40165 | 0.97699 | 0.95584 |
| 20 | 8 | WHIR batched 1/2 | 0.76765 | 0.73861 | 1.50626 | 1.35889 |
| 20 | 8 | WHIR batched 1/4 | 1.24895 | 1.00505 | 2.25400 | 2.20675 |
| 20 | 8 | Canopy Exp. 3 | 1.23842 | 0.87774 | 2.11616 | 2.09674 |
| 22 | 1 | WHIR batched 1/2 | 0.45560 | 0.50219 | 0.95779 | 0.91291 |
| 22 | 1 | WHIR batched 1/4 | 0.63805 | 0.68816 | 1.32621 | 1.29744 |
| 22 | 1 | Canopy Exp. 3 | 0.63076 | 0.55014 | 1.18091 | 1.21024 |
| 22 | 2 | WHIR batched 1/2 | 0.91121 | 0.89324 | 1.80444 | 1.76095 |
| 22 | 2 | WHIR batched 1/4 | 1.27611 | 1.19075 | 2.46686 | 2.41878 |
| 22 | 2 | Canopy Exp. 3 | 1.23445 | 0.90728 | 2.14173 | 2.18477 |
| 22 | 4 | WHIR batched 1/2 | 1.82241 | 1.66684 | 3.48925 | 3.39471 |
| 22 | 4 | WHIR batched 1/4 | 2.55221 | 2.19459 | 4.74680 | 4.67085 |
| 22 | 4 | Canopy Exp. 3 | 2.39898 | 1.81667 | 4.21565 | 4.06798 |
| 22 | 8 | WHIR batched 1/2 | 3.64483 | 3.39655 | 7.04138 | 6.80956 |
| 22 | 8 | WHIR batched 1/4 | 5.10443 | 4.38585 | 9.49028 | 9.30538 |
| 22 | 8 | Canopy Exp. 3 | 5.98146 | 4.00836 | 9.98982 | 9.67103 |

## WHIR: measured stages

| n | t | Scheme | Stage | Mean seconds |
|---|---|---|---|---|
| 18 | 1 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.011086 |
| 18 | 1 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.000000 |
| 18 | 1 | WHIR batched 1/2 | initial_merkle_tree | 0.002972 |
| 18 | 1 | WHIR batched 1/2 | initial_ood_evaluations | 0.000906 |
| 18 | 1 | WHIR batched 1/2 | initial_rs_encoding | 0.017091 |
| 18 | 1 | WHIR batched 1/2 | recursive_tail_after_ood | 0.010077 |
| 18 | 1 | WHIR batched 1/2 | recursive_tail_ood_evaluations | 0.000229 |
| 18 | 1 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.009284 |
| 18 | 1 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.016849 |
| 18 | 1 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.000000 |
| 18 | 1 | WHIR batched 1/4 | initial_merkle_tree | 0.007201 |
| 18 | 1 | WHIR batched 1/4 | initial_ood_evaluations | 0.000991 |
| 18 | 1 | WHIR batched 1/4 | initial_rs_encoding | 0.033502 |
| 18 | 1 | WHIR batched 1/4 | recursive_tail_after_ood | 0.016231 |
| 18 | 1 | WHIR batched 1/4 | recursive_tail_ood_evaluations | 0.000231 |
| 18 | 1 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.013500 |
| 18 | 1 | Canopy Exp. 3 | batched_recursive_whir | 0.017847 |
| 18 | 1 | Canopy Exp. 3 | canopy_bottom_forest | 0.001984 |
| 18 | 1 | Canopy Exp. 3 | canopy_bottom_opening | 0.000091 |
| 18 | 1 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.002736 |
| 18 | 1 | Canopy Exp. 3 | canopy_commitment_other | 0.000001 |
| 18 | 1 | Canopy Exp. 3 | canopy_hash_to_field | 0.001098 |
| 18 | 1 | Canopy Exp. 3 | canopy_rs_encoding | 0.010928 |
| 18 | 1 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.002064 |
| 18 | 1 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.014676 |
| 18 | 1 | Canopy Exp. 3 | outer_ood_evaluations | 0.000906 |
| 18 | 1 | Canopy Exp. 3 | outer_rs_encoding | 0.015652 |
| 18 | 2 | WHIR batched 1/2 | batched_recursive_whir | 0.010158 |
| 18 | 2 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.022173 |
| 18 | 2 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.001689 |
| 18 | 2 | WHIR batched 1/2 | initial_merkle_tree | 0.005945 |
| 18 | 2 | WHIR batched 1/2 | initial_ood_evaluations | 0.001812 |
| 18 | 2 | WHIR batched 1/2 | initial_rs_encoding | 0.034183 |
| 18 | 2 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.018569 |
| 18 | 2 | WHIR batched 1/4 | batched_recursive_whir | 0.018115 |
| 18 | 2 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.033698 |
| 18 | 2 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.001689 |
| 18 | 2 | WHIR batched 1/4 | initial_merkle_tree | 0.014403 |
| 18 | 2 | WHIR batched 1/4 | initial_ood_evaluations | 0.001982 |
| 18 | 2 | WHIR batched 1/4 | initial_rs_encoding | 0.067005 |
| 18 | 2 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.027000 |
| 18 | 2 | Canopy Exp. 3 | batched_recursive_whir | 0.019560 |
| 18 | 2 | Canopy Exp. 3 | canopy_bottom_forest | 0.003991 |
| 18 | 2 | Canopy Exp. 3 | canopy_bottom_opening | 0.000172 |
| 18 | 2 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.004475 |
| 18 | 2 | Canopy Exp. 3 | canopy_commitment_other | 0.000001 |
| 18 | 2 | Canopy Exp. 3 | canopy_hash_to_field | 0.001959 |
| 18 | 2 | Canopy Exp. 3 | canopy_rs_encoding | 0.020941 |
| 18 | 2 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.003099 |
| 18 | 2 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.029352 |
| 18 | 2 | Canopy Exp. 3 | outer_ood_evaluations | 0.001812 |
| 18 | 2 | Canopy Exp. 3 | outer_rs_encoding | 0.030143 |
| 18 | 4 | WHIR batched 1/2 | batched_recursive_whir | 0.012322 |
| 18 | 4 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.044345 |
| 18 | 4 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.003053 |
| 18 | 4 | WHIR batched 1/2 | initial_merkle_tree | 0.011889 |
| 18 | 4 | WHIR batched 1/2 | initial_ood_evaluations | 0.003625 |
| 18 | 4 | WHIR batched 1/2 | initial_rs_encoding | 0.068365 |
| 18 | 4 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.037138 |
| 18 | 4 | WHIR batched 1/4 | batched_recursive_whir | 0.018719 |
| 18 | 4 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.067397 |
| 18 | 4 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.003053 |
| 18 | 4 | WHIR batched 1/4 | initial_merkle_tree | 0.028806 |
| 18 | 4 | WHIR batched 1/4 | initial_ood_evaluations | 0.003965 |
| 18 | 4 | WHIR batched 1/4 | initial_rs_encoding | 0.134010 |
| 18 | 4 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.054001 |
| 18 | 4 | Canopy Exp. 3 | batched_recursive_whir | 0.027239 |
| 18 | 4 | Canopy Exp. 3 | canopy_bottom_forest | 0.007115 |
| 18 | 4 | Canopy Exp. 3 | canopy_bottom_opening | 0.000309 |
| 18 | 4 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.008495 |
| 18 | 4 | Canopy Exp. 3 | canopy_commitment_other | 0.000001 |
| 18 | 4 | Canopy Exp. 3 | canopy_hash_to_field | 0.004003 |
| 18 | 4 | Canopy Exp. 3 | canopy_rs_encoding | 0.038991 |
| 18 | 4 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.009509 |
| 18 | 4 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.058705 |
| 18 | 4 | Canopy Exp. 3 | outer_ood_evaluations | 0.003625 |
| 18 | 4 | Canopy Exp. 3 | outer_rs_encoding | 0.055731 |
| 18 | 8 | WHIR batched 1/2 | batched_recursive_whir | 0.020391 |
| 18 | 8 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.088690 |
| 18 | 8 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.009200 |
| 18 | 8 | WHIR batched 1/2 | initial_merkle_tree | 0.023778 |
| 18 | 8 | WHIR batched 1/2 | initial_ood_evaluations | 0.007249 |
| 18 | 8 | WHIR batched 1/2 | initial_rs_encoding | 0.136731 |
| 18 | 8 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.074276 |
| 18 | 8 | WHIR batched 1/4 | batched_recursive_whir | 0.027332 |
| 18 | 8 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.134794 |
| 18 | 8 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.009200 |
| 18 | 8 | WHIR batched 1/4 | initial_merkle_tree | 0.057611 |
| 18 | 8 | WHIR batched 1/4 | initial_ood_evaluations | 0.007930 |
| 18 | 8 | WHIR batched 1/4 | initial_rs_encoding | 0.268019 |
| 18 | 8 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.108002 |
| 18 | 8 | Canopy Exp. 3 | batched_recursive_whir | 0.059218 |
| 18 | 8 | Canopy Exp. 3 | canopy_bottom_forest | 0.014341 |
| 18 | 8 | Canopy Exp. 3 | canopy_bottom_opening | 0.000296 |
| 18 | 8 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.016081 |
| 18 | 8 | Canopy Exp. 3 | canopy_commitment_other | 0.000001 |
| 18 | 8 | Canopy Exp. 3 | canopy_hash_to_field | 0.007749 |
| 18 | 8 | Canopy Exp. 3 | canopy_rs_encoding | 0.074835 |
| 18 | 8 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.033119 |
| 18 | 8 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.117409 |
| 18 | 8 | Canopy Exp. 3 | outer_ood_evaluations | 0.007249 |
| 18 | 8 | Canopy Exp. 3 | outer_rs_encoding | 0.108213 |
| 20 | 1 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.039431 |
| 20 | 1 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.000000 |
| 20 | 1 | WHIR batched 1/2 | initial_merkle_tree | 0.012240 |
| 20 | 1 | WHIR batched 1/2 | initial_ood_evaluations | 0.003192 |
| 20 | 1 | WHIR batched 1/2 | initial_rs_encoding | 0.083716 |
| 20 | 1 | WHIR batched 1/2 | recursive_tail_after_ood | 0.042539 |
| 20 | 1 | WHIR batched 1/2 | recursive_tail_ood_evaluations | 0.000548 |
| 20 | 1 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.033258 |
| 20 | 1 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.044698 |
| 20 | 1 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.000000 |
| 20 | 1 | WHIR batched 1/4 | initial_merkle_tree | 0.019871 |
| 20 | 1 | WHIR batched 1/4 | initial_ood_evaluations | 0.004700 |
| 20 | 1 | WHIR batched 1/4 | initial_rs_encoding | 0.136248 |
| 20 | 1 | WHIR batched 1/4 | recursive_tail_after_ood | 0.082681 |
| 20 | 1 | WHIR batched 1/4 | recursive_tail_ood_evaluations | 0.001360 |
| 20 | 1 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.075668 |
| 20 | 1 | Canopy Exp. 3 | batched_recursive_whir | 0.062485 |
| 20 | 1 | Canopy Exp. 3 | canopy_bottom_forest | 0.008097 |
| 20 | 1 | Canopy Exp. 3 | canopy_bottom_opening | 0.000471 |
| 20 | 1 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.010850 |
| 20 | 1 | Canopy Exp. 3 | canopy_commitment_other | 0.000001 |
| 20 | 1 | Canopy Exp. 3 | canopy_hash_to_field | 0.003971 |
| 20 | 1 | Canopy Exp. 3 | canopy_rs_encoding | 0.049306 |
| 20 | 1 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.006617 |
| 20 | 1 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.057178 |
| 20 | 1 | Canopy Exp. 3 | outer_ood_evaluations | 0.003192 |
| 20 | 1 | Canopy Exp. 3 | outer_rs_encoding | 0.070090 |
| 20 | 2 | WHIR batched 1/2 | batched_recursive_whir | 0.045478 |
| 20 | 2 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.078862 |
| 20 | 2 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.006362 |
| 20 | 2 | WHIR batched 1/2 | initial_merkle_tree | 0.024480 |
| 20 | 2 | WHIR batched 1/2 | initial_ood_evaluations | 0.006383 |
| 20 | 2 | WHIR batched 1/2 | initial_rs_encoding | 0.167431 |
| 20 | 2 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.066516 |
| 20 | 2 | WHIR batched 1/4 | batched_recursive_whir | 0.097262 |
| 20 | 2 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.089396 |
| 20 | 2 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.006362 |
| 20 | 2 | WHIR batched 1/4 | initial_merkle_tree | 0.039743 |
| 20 | 2 | WHIR batched 1/4 | initial_ood_evaluations | 0.009400 |
| 20 | 2 | WHIR batched 1/4 | initial_rs_encoding | 0.272496 |
| 20 | 2 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.151336 |
| 20 | 2 | Canopy Exp. 3 | batched_recursive_whir | 0.091578 |
| 20 | 2 | Canopy Exp. 3 | canopy_bottom_forest | 0.015115 |
| 20 | 2 | Canopy Exp. 3 | canopy_bottom_opening | 0.000598 |
| 20 | 2 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.019691 |
| 20 | 2 | Canopy Exp. 3 | canopy_commitment_other | 0.000001 |
| 20 | 2 | Canopy Exp. 3 | canopy_hash_to_field | 0.008627 |
| 20 | 2 | Canopy Exp. 3 | canopy_rs_encoding | 0.094148 |
| 20 | 2 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.013258 |
| 20 | 2 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.114355 |
| 20 | 2 | Canopy Exp. 3 | outer_ood_evaluations | 0.006383 |
| 20 | 2 | Canopy Exp. 3 | outer_rs_encoding | 0.130865 |
| 20 | 4 | WHIR batched 1/2 | batched_recursive_whir | 0.051500 |
| 20 | 4 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.157724 |
| 20 | 4 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.012981 |
| 20 | 4 | WHIR batched 1/2 | initial_merkle_tree | 0.048961 |
| 20 | 4 | WHIR batched 1/2 | initial_ood_evaluations | 0.012767 |
| 20 | 4 | WHIR batched 1/2 | initial_rs_encoding | 0.334863 |
| 20 | 4 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.133031 |
| 20 | 4 | WHIR batched 1/4 | batched_recursive_whir | 0.084220 |
| 20 | 4 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.178791 |
| 20 | 4 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.012981 |
| 20 | 4 | WHIR batched 1/4 | initial_merkle_tree | 0.079485 |
| 20 | 4 | WHIR batched 1/4 | initial_ood_evaluations | 0.018800 |
| 20 | 4 | WHIR batched 1/4 | initial_rs_encoding | 0.544991 |
| 20 | 4 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.302673 |
| 20 | 4 | Canopy Exp. 3 | batched_recursive_whir | 0.103174 |
| 20 | 4 | Canopy Exp. 3 | canopy_bottom_forest | 0.033616 |
| 20 | 4 | Canopy Exp. 3 | canopy_bottom_opening | 0.001037 |
| 20 | 4 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.039023 |
| 20 | 4 | Canopy Exp. 3 | canopy_commitment_other | 0.000002 |
| 20 | 4 | Canopy Exp. 3 | canopy_hash_to_field | 0.017128 |
| 20 | 4 | Canopy Exp. 3 | canopy_rs_encoding | 0.204958 |
| 20 | 4 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.055965 |
| 20 | 4 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.228711 |
| 20 | 4 | Canopy Exp. 3 | outer_ood_evaluations | 0.012767 |
| 20 | 4 | Canopy Exp. 3 | outer_rs_encoding | 0.280615 |
| 20 | 8 | WHIR batched 1/2 | batched_recursive_whir | 0.078750 |
| 20 | 8 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.315449 |
| 20 | 8 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.052815 |
| 20 | 8 | WHIR batched 1/2 | initial_merkle_tree | 0.097921 |
| 20 | 8 | WHIR batched 1/2 | initial_ood_evaluations | 0.025533 |
| 20 | 8 | WHIR batched 1/2 | initial_rs_encoding | 0.669725 |
| 20 | 8 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.266063 |
| 20 | 8 | WHIR batched 1/4 | batched_recursive_whir | 0.112180 |
| 20 | 8 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.357582 |
| 20 | 8 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.052815 |
| 20 | 8 | WHIR batched 1/4 | initial_merkle_tree | 0.158971 |
| 20 | 8 | WHIR batched 1/4 | initial_ood_evaluations | 0.037599 |
| 20 | 8 | WHIR batched 1/4 | initial_rs_encoding | 1.089983 |
| 20 | 8 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.605346 |
| 20 | 8 | Canopy Exp. 3 | batched_recursive_whir | 0.242179 |
| 20 | 8 | Canopy Exp. 3 | canopy_bottom_forest | 0.067269 |
| 20 | 8 | Canopy Exp. 3 | canopy_bottom_opening | 0.003196 |
| 20 | 8 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.089040 |
| 20 | 8 | Canopy Exp. 3 | canopy_commitment_other | 0.000006 |
| 20 | 8 | Canopy Exp. 3 | canopy_hash_to_field | 0.032861 |
| 20 | 8 | Canopy Exp. 3 | canopy_rs_encoding | 0.466884 |
| 20 | 8 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.149415 |
| 20 | 8 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.457421 |
| 20 | 8 | Canopy Exp. 3 | outer_ood_evaluations | 0.025533 |
| 20 | 8 | Canopy Exp. 3 | outer_rs_encoding | 0.582356 |
| 22 | 1 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.213496 |
| 22 | 1 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.000000 |
| 22 | 1 | WHIR batched 1/2 | initial_merkle_tree | 0.039417 |
| 22 | 1 | WHIR batched 1/2 | initial_ood_evaluations | 0.013106 |
| 22 | 1 | WHIR batched 1/2 | initial_rs_encoding | 0.416186 |
| 22 | 1 | WHIR batched 1/2 | recursive_tail_after_ood | 0.143411 |
| 22 | 1 | WHIR batched 1/2 | recursive_tail_ood_evaluations | 0.001745 |
| 22 | 1 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.130433 |
| 22 | 1 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.187981 |
| 22 | 1 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.000000 |
| 22 | 1 | WHIR batched 1/4 | initial_merkle_tree | 0.074593 |
| 22 | 1 | WHIR batched 1/4 | initial_ood_evaluations | 0.012185 |
| 22 | 1 | WHIR batched 1/4 | initial_rs_encoding | 0.563460 |
| 22 | 1 | WHIR batched 1/4 | recursive_tail_after_ood | 0.220729 |
| 22 | 1 | WHIR batched 1/4 | recursive_tail_ood_evaluations | 0.001733 |
| 22 | 1 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.265533 |
| 22 | 1 | Canopy Exp. 3 | batched_recursive_whir | 0.240302 |
| 22 | 1 | Canopy Exp. 3 | canopy_bottom_forest | 0.032774 |
| 22 | 1 | Canopy Exp. 3 | canopy_bottom_opening | 0.000265 |
| 22 | 1 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.035892 |
| 22 | 1 | Canopy Exp. 3 | canopy_commitment_other | 0.000001 |
| 22 | 1 | Canopy Exp. 3 | canopy_hash_to_field | 0.016514 |
| 22 | 1 | Canopy Exp. 3 | canopy_rs_encoding | 0.220911 |
| 22 | 1 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.024423 |
| 22 | 1 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.272045 |
| 22 | 1 | Canopy Exp. 3 | outer_ood_evaluations | 0.013106 |
| 22 | 1 | Canopy Exp. 3 | outer_rs_encoding | 0.324672 |
| 22 | 2 | WHIR batched 1/2 | batched_recursive_whir | 0.154029 |
| 22 | 2 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.426992 |
| 22 | 2 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.025138 |
| 22 | 2 | WHIR batched 1/2 | initial_merkle_tree | 0.078835 |
| 22 | 2 | WHIR batched 1/2 | initial_ood_evaluations | 0.026213 |
| 22 | 2 | WHIR batched 1/2 | initial_rs_encoding | 0.832372 |
| 22 | 2 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.260865 |
| 22 | 2 | WHIR batched 1/4 | batched_recursive_whir | 0.234216 |
| 22 | 2 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.375962 |
| 22 | 2 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.025138 |
| 22 | 2 | WHIR batched 1/4 | initial_merkle_tree | 0.149187 |
| 22 | 2 | WHIR batched 1/4 | initial_ood_evaluations | 0.024370 |
| 22 | 2 | WHIR batched 1/4 | initial_rs_encoding | 1.126920 |
| 22 | 2 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 0.531065 |
| 22 | 2 | Canopy Exp. 3 | batched_recursive_whir | 0.285697 |
| 22 | 2 | Canopy Exp. 3 | canopy_bottom_forest | 0.063212 |
| 22 | 2 | Canopy Exp. 3 | canopy_bottom_opening | 0.000362 |
| 22 | 2 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.071497 |
| 22 | 2 | Canopy Exp. 3 | canopy_commitment_other | 0.000001 |
| 22 | 2 | Canopy Exp. 3 | canopy_hash_to_field | 0.033129 |
| 22 | 2 | Canopy Exp. 3 | canopy_rs_encoding | 0.431927 |
| 22 | 2 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.050921 |
| 22 | 2 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 0.544089 |
| 22 | 2 | Canopy Exp. 3 | outer_ood_evaluations | 0.026213 |
| 22 | 2 | Canopy Exp. 3 | outer_rs_encoding | 0.634681 |
| 22 | 4 | WHIR batched 1/2 | batched_recursive_whir | 0.189360 |
| 22 | 4 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 0.853984 |
| 22 | 4 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.049339 |
| 22 | 4 | WHIR batched 1/2 | initial_merkle_tree | 0.157670 |
| 22 | 4 | WHIR batched 1/2 | initial_ood_evaluations | 0.052425 |
| 22 | 4 | WHIR batched 1/2 | initial_rs_encoding | 1.664744 |
| 22 | 4 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 0.521731 |
| 22 | 4 | WHIR batched 1/4 | batched_recursive_whir | 0.282454 |
| 22 | 4 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 0.751924 |
| 22 | 4 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.049339 |
| 22 | 4 | WHIR batched 1/4 | initial_merkle_tree | 0.298374 |
| 22 | 4 | WHIR batched 1/4 | initial_ood_evaluations | 0.048741 |
| 22 | 4 | WHIR batched 1/4 | initial_rs_encoding | 2.253840 |
| 22 | 4 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 1.062131 |
| 22 | 4 | Canopy Exp. 3 | batched_recursive_whir | 0.515925 |
| 22 | 4 | Canopy Exp. 3 | canopy_bottom_forest | 0.122237 |
| 22 | 4 | Canopy Exp. 3 | canopy_bottom_opening | 0.000458 |
| 22 | 4 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.133869 |
| 22 | 4 | Canopy Exp. 3 | canopy_commitment_other | 0.000003 |
| 22 | 4 | Canopy Exp. 3 | canopy_hash_to_field | 0.065508 |
| 22 | 4 | Canopy Exp. 3 | canopy_rs_encoding | 0.879659 |
| 22 | 4 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.159688 |
| 22 | 4 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 1.088178 |
| 22 | 4 | Canopy Exp. 3 | outer_ood_evaluations | 0.052425 |
| 22 | 4 | Canopy Exp. 3 | outer_rs_encoding | 1.197705 |
| 22 | 8 | WHIR batched 1/2 | batched_recursive_whir | 0.367079 |
| 22 | 8 | WHIR batched 1/2 | first_transition_residual_after_subtracting_tail | 1.707968 |
| 22 | 8 | WHIR batched 1/2 | generalized_sz_cross_terms | 0.173195 |
| 22 | 8 | WHIR batched 1/2 | initial_merkle_tree | 0.315339 |
| 22 | 8 | WHIR batched 1/2 | initial_ood_evaluations | 0.104850 |
| 22 | 8 | WHIR batched 1/2 | initial_rs_encoding | 3.329488 |
| 22 | 8 | WHIR batched 1/2 | second_oracle_rs_and_merkle_commitment | 1.043462 |
| 22 | 8 | WHIR batched 1/4 | batched_recursive_whir | 0.487065 |
| 22 | 8 | WHIR batched 1/4 | first_transition_residual_after_subtracting_tail | 1.503847 |
| 22 | 8 | WHIR batched 1/4 | generalized_sz_cross_terms | 0.173195 |
| 22 | 8 | WHIR batched 1/4 | initial_merkle_tree | 0.596747 |
| 22 | 8 | WHIR batched 1/4 | initial_ood_evaluations | 0.097481 |
| 22 | 8 | WHIR batched 1/4 | initial_rs_encoding | 4.507679 |
| 22 | 8 | WHIR batched 1/4 | second_oracle_rs_and_merkle_commitment | 2.124261 |
| 22 | 8 | Canopy Exp. 3 | batched_recursive_whir | 1.096335 |
| 22 | 8 | Canopy Exp. 3 | canopy_bottom_forest | 0.237333 |
| 22 | 8 | Canopy Exp. 3 | canopy_bottom_opening | 0.004088 |
| 22 | 8 | Canopy Exp. 3 | canopy_codeword_merkle_tree | 0.310840 |
| 22 | 8 | Canopy Exp. 3 | canopy_commitment_other | 0.000001 |
| 22 | 8 | Canopy Exp. 3 | canopy_hash_to_field | 0.132019 |
| 22 | 8 | Canopy Exp. 3 | canopy_rs_encoding | 2.884824 |
| 22 | 8 | Canopy Exp. 3 | generalized_sz_cross_terms | 0.626727 |
| 22 | 8 | Canopy Exp. 3 | outer_initial_whir_stage_after_ood | 2.176357 |
| 22 | 8 | Canopy Exp. 3 | outer_ood_evaluations | 0.104850 |
| 22 | 8 | Canopy Exp. 3 | outer_rs_encoding | 2.416443 |
| 20 | 1 | WHIR batched 1/4 | separately_measured_component_adjustment | -0.020059 |
| 20 | 2 | WHIR batched 1/4 | separately_measured_component_adjustment | -0.040118 |
| 20 | 4 | WHIR batched 1/4 | separately_measured_component_adjustment | -0.080237 |
| 20 | 8 | WHIR batched 1/4 | separately_measured_component_adjustment | -0.160474 |

## Bolt: all configurations

| n | w | Scheme | RS block | h X/Y | Commit | Opening | Total | Median total |
|---|---|---|---|---|---|---|---|---|
| 18 | 4 | Bolt + Canopy | k/4 | 16/14 | 0.01312 | 0.03685 | 0.04997 | 0.04968 |
| 18 | 4 | Bolt + Canopy | k | 3/1 | 0.01830 | 0.04434 | 0.06264 | 0.06309 |
| 18 | 4 | Bolt + Canopy | k/2 | 3/1 | 0.01672 | 0.04165 | 0.05837 | 0.05831 |
| 18 | 4 | Bolt + Merkle | - | - | 0.01491 | 0.03688 | 0.05180 | 0.05160 |
| 18 | 8 | Bolt + Canopy | k/4 | 15/13 | 0.00955 | 0.03624 | 0.04579 | 0.04568 |
| 18 | 8 | Bolt + Canopy | k | 3/1 | 0.01232 | 0.03909 | 0.05140 | 0.05123 |
| 18 | 8 | Bolt + Canopy | k/2 | 3/1 | 0.01161 | 0.03787 | 0.04948 | 0.04964 |
| 18 | 8 | Bolt + Merkle | - | - | 0.00983 | 0.03623 | 0.04606 | 0.04615 |
| 18 | 16 | Bolt + Canopy | k/4 | 14/12 | 0.00785 | 0.02757 | 0.03541 | 0.03508 |
| 18 | 16 | Bolt + Canopy | k | 3/1 | 0.00883 | 0.02923 | 0.03806 | 0.03798 |
| 18 | 16 | Bolt + Canopy | k/2 | 3/1 | 0.00871 | 0.02903 | 0.03774 | 0.03712 |
| 18 | 16 | Bolt + Merkle | - | - | 0.00796 | 0.02868 | 0.03665 | 0.03631 |
| 20 | 4 | Bolt + Canopy | k/4 | 3/1 | 0.06133 | 0.10015 | 0.16147 | 0.16069 |
| 20 | 4 | Bolt + Canopy | k | 3/1 | 0.07020 | 0.11135 | 0.18155 | 0.18118 |
| 20 | 4 | Bolt + Canopy | k/2 | 3/1 | 0.06298 | 0.10492 | 0.16790 | 0.16753 |
| 20 | 4 | Bolt + Merkle | - | - | 0.04895 | 0.09052 | 0.13947 | 0.13897 |
| 20 | 8 | Bolt + Canopy | k/4 | 17/15 | 0.04039 | 0.07378 | 0.11417 | 0.11324 |
| 20 | 8 | Bolt + Canopy | k | 3/1 | 0.05356 | 0.08781 | 0.14137 | 0.13809 |
| 20 | 8 | Bolt + Canopy | k/2 | 3/1 | 0.05048 | 0.08308 | 0.13356 | 0.12969 |
| 20 | 8 | Bolt + Merkle | - | - | 0.03994 | 0.07871 | 0.11864 | 0.11777 |
| 20 | 16 | Bolt + Canopy | k/4 | 16/14 | 0.04681 | 0.09201 | 0.13882 | 0.11943 |
| 20 | 16 | Bolt + Canopy | k | 3/1 | 0.05821 | 0.10144 | 0.15965 | 0.13583 |
| 20 | 16 | Bolt + Canopy | k/2 | 3/1 | 0.04305 | 0.09578 | 0.13882 | 0.14012 |
| 20 | 16 | Bolt + Merkle | - | - | 0.04107 | 0.08506 | 0.12613 | 0.12652 |
| 22 | 4 | Bolt + Canopy | k/4 | 3/1 | 0.28989 | 0.40202 | 0.69191 | 0.68808 |
| 22 | 4 | Bolt + Canopy | k | 3/1 | 0.35405 | 0.50088 | 0.85492 | 0.82050 |
| 22 | 4 | Bolt + Canopy | k/2 | 3/1 | 0.30436 | 0.42829 | 0.73265 | 0.73342 |
| 22 | 4 | Bolt + Merkle | - | - | 0.33900 | 0.39457 | 0.73357 | 0.65174 |
| 22 | 8 | Bolt + Canopy | k/4 | 3/1 | 0.20102 | 0.24941 | 0.45044 | 0.44518 |
| 22 | 8 | Bolt + Canopy | k | 3/1 | 0.23106 | 0.28230 | 0.51336 | 0.51322 |
| 22 | 8 | Bolt + Canopy | k/2 | 3/1 | 0.23920 | 0.26970 | 0.50890 | 0.48734 |
| 22 | 8 | Bolt + Merkle | - | - | 0.17463 | 0.25731 | 0.43194 | 0.42088 |
| 22 | 16 | Bolt + Canopy | k/4 | 3/1 | 0.18452 | 0.28191 | 0.46643 | 0.42984 |
| 22 | 16 | Bolt + Canopy | k | 3/1 | 0.19272 | 0.29907 | 0.49179 | 0.42614 |
| 22 | 16 | Bolt + Canopy | k/2 | 3/1 | 0.17577 | 0.24604 | 0.42180 | 0.42595 |
| 22 | 16 | Bolt + Merkle | - | - | 0.15653 | 0.23323 | 0.38976 | 0.38806 |

## Bolt: stages at the smallest-proof configurations

| n | Scheme | Phase | Part | Stage | Mean seconds |
|---|---|---|---|---|---|
| 18 | Bolt + Canopy | commitment | Bolt | parity RS encoding Y=RS(U) | 0.002200 |
| 18 | Bolt + Canopy | commitment | Bolt | sparse syndrome U=HX | 0.001420 |
| 18 | Bolt + Canopy | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.001963 |
| 18 | Bolt + Canopy | commitment | X | Canopy WHIR configuration | 0.000003 |
| 18 | Bolt + Canopy | commitment | X | Canopy hash-to-field | 0.001381 |
| 18 | Bolt + Canopy | commitment | X | row Merkle tree or forest | 0.006600 |
| 18 | Bolt + Canopy | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.001860 |
| 18 | Bolt + Canopy | commitment | Y | Canopy WHIR configuration | 0.000002 |
| 18 | Bolt + Canopy | commitment | Y | Canopy hash-to-field | 0.001358 |
| 18 | Bolt + Canopy | commitment | Y | row Merkle tree or forest | 0.001510 |
| 18 | Bolt + Canopy | commitment | other | unassigned phase work and timing overhead | 0.000003 |
| 18 | Bolt + Canopy | opening | Bolt | column projections x=Xr and u=Ur | 0.001034 |
| 18 | Bolt + Canopy | opening | Bolt | parity query covector and transpose RS FFT | 0.001050 |
| 18 | Bolt + Canopy | opening | Bolt | partial evaluation g | 0.003431 |
| 18 | Bolt + Canopy | opening | Bolt | project queried rows and sample batching challenges | 0.001036 |
| 18 | Bolt + Canopy | opening | Bolt | public H transpose covector | 0.000890 |
| 18 | Bolt + Canopy | opening | Bolt | sample row queries | 0.006283 |
| 18 | Bolt + Canopy | opening | Bolt | syndrome evaluation sigma | 0.000223 |
| 18 | Bolt + Canopy | opening | Bolt | systematic query covector | 0.000593 |
| 18 | Bolt + Canopy | opening | X | Canopy OOD evaluations | 0.000093 |
| 18 | Bolt + Canopy | opening | X | Canopy SEP batching covector | 0.000183 |
| 18 | Bolt + Canopy | opening | X | Canopy ancestor claims | 0.000083 |
| 18 | Bolt + Canopy | opening | X | authenticated WHIR alignment and constraint combination | 0.000226 |
| 18 | Bolt + Canopy | opening | X | collect and serialize row authentication paths | 0.003584 |
| 18 | Bolt + Canopy | opening | X | copy and serialize queried row values | 0.004285 |
| 18 | Bolt + Canopy | opening | Y | Canopy OOD evaluations | 0.000090 |
| 18 | Bolt + Canopy | opening | Y | Canopy SEP batching covector | 0.000011 |
| 18 | Bolt + Canopy | opening | Y | Canopy ancestor claims | 0.000003 |
| 18 | Bolt + Canopy | opening | Y | authenticated WHIR alignment and constraint combination | 0.000305 |
| 18 | Bolt + Canopy | opening | Y | collect and serialize row authentication paths | 0.000038 |
| 18 | Bolt + Canopy | opening | Y | copy and serialize queried row values | 0.000080 |
| 18 | Bolt + Canopy | opening | batched WHIR | distinct-functional cross evaluations | 0.002158 |
| 18 | Bolt + Canopy | opening | batched WHIR | shared recursive WHIR proof | 0.004232 |
| 18 | Bolt + Canopy | opening | common WHIR | WHIR configuration | 0.000003 |
| 18 | Bolt + Canopy | opening | other | unassigned phase work and timing overhead | 0.000684 |
| 18 | Bolt + Canopy | opening | u | authenticated WHIR alignment and constraint combination | 0.000352 |
| 18 | Bolt + Canopy | opening | u | common WHIR RS encode and oracle commitment | 0.002228 |
| 18 | Bolt + Canopy | opening | x | authenticated WHIR alignment and constraint combination | 0.006472 |
| 18 | Bolt + Canopy | opening | x | common WHIR RS encode and oracle commitment | 0.004687 |
| 18 | Bolt + Merkle | commitment | Bolt | parity RS encoding Y=RS(U) | 0.002364 |
| 18 | Bolt + Merkle | commitment | Bolt | sparse syndrome U=HX | 0.001560 |
| 18 | Bolt + Merkle | commitment | X | Merkle root transcript | 0.000000 |
| 18 | Bolt + Merkle | commitment | X | row Merkle tree or forest | 0.007760 |
| 18 | Bolt + Merkle | commitment | Y | Merkle root transcript | 0.000000 |
| 18 | Bolt + Merkle | commitment | Y | row Merkle tree or forest | 0.003228 |
| 18 | Bolt + Merkle | commitment | other | unassigned phase work and timing overhead | 0.000001 |
| 18 | Bolt + Merkle | opening | Bolt | column projections x=Xr and u=Ur | 0.000936 |
| 18 | Bolt + Merkle | opening | Bolt | parity query covector and transpose RS FFT | 0.001050 |
| 18 | Bolt + Merkle | opening | Bolt | partial evaluation g | 0.003425 |
| 18 | Bolt + Merkle | opening | Bolt | project queried rows and sample batching challenges | 0.000999 |
| 18 | Bolt + Merkle | opening | Bolt | public H transpose covector | 0.000853 |
| 18 | Bolt + Merkle | opening | Bolt | sample row queries | 0.006135 |
| 18 | Bolt + Merkle | opening | Bolt | syndrome evaluation sigma | 0.000220 |
| 18 | Bolt + Merkle | opening | Bolt | systematic query covector | 0.000539 |
| 18 | Bolt + Merkle | opening | X | collect and serialize row authentication paths | 0.004254 |
| 18 | Bolt + Merkle | opening | X | copy and serialize queried row values | 0.004193 |
| 18 | Bolt + Merkle | opening | Y | collect and serialize row authentication paths | 0.000176 |
| 18 | Bolt + Merkle | opening | Y | copy and serialize queried row values | 0.000081 |
| 18 | Bolt + Merkle | opening | batched WHIR | distinct-functional cross evaluations | 0.000389 |
| 18 | Bolt + Merkle | opening | batched WHIR | shared recursive WHIR proof | 0.002341 |
| 18 | Bolt + Merkle | opening | common WHIR | WHIR configuration | 0.000004 |
| 18 | Bolt + Merkle | opening | other | unassigned phase work and timing overhead | 0.000365 |
| 18 | Bolt + Merkle | opening | u | authenticated WHIR alignment and constraint combination | 0.000375 |
| 18 | Bolt + Merkle | opening | u | common WHIR RS encode and oracle commitment | 0.000956 |
| 18 | Bolt + Merkle | opening | x | authenticated WHIR alignment and constraint combination | 0.005210 |
| 18 | Bolt + Merkle | opening | x | common WHIR RS encode and oracle commitment | 0.004380 |
| 20 | Bolt + Canopy | commitment | Bolt | parity RS encoding Y=RS(U) | 0.008917 |
| 20 | Bolt + Canopy | commitment | Bolt | sparse syndrome U=HX | 0.010416 |
| 20 | Bolt + Canopy | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.006689 |
| 20 | Bolt + Canopy | commitment | X | Canopy WHIR configuration | 0.000004 |
| 20 | Bolt + Canopy | commitment | X | Canopy hash-to-field | 0.005195 |
| 20 | Bolt + Canopy | commitment | X | row Merkle tree or forest | 0.022264 |
| 20 | Bolt + Canopy | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.006632 |
| 20 | Bolt + Canopy | commitment | Y | Canopy WHIR configuration | 0.000003 |
| 20 | Bolt + Canopy | commitment | Y | Canopy hash-to-field | 0.005168 |
| 20 | Bolt + Canopy | commitment | Y | row Merkle tree or forest | 0.004911 |
| 20 | Bolt + Canopy | commitment | other | unassigned phase work and timing overhead | 0.000002 |
| 20 | Bolt + Canopy | opening | Bolt | column projections x=Xr and u=Ur | 0.002922 |
| 20 | Bolt + Canopy | opening | Bolt | parity query covector and transpose RS FFT | 0.002324 |
| 20 | Bolt + Canopy | opening | Bolt | partial evaluation g | 0.012717 |
| 20 | Bolt + Canopy | opening | Bolt | project queried rows and sample batching challenges | 0.000970 |
| 20 | Bolt + Canopy | opening | Bolt | public H transpose covector | 0.004132 |
| 20 | Bolt + Canopy | opening | Bolt | sample row queries | 0.006714 |
| 20 | Bolt + Canopy | opening | Bolt | syndrome evaluation sigma | 0.000541 |
| 20 | Bolt + Canopy | opening | Bolt | systematic query covector | 0.001180 |
| 20 | Bolt + Canopy | opening | X | Canopy OOD evaluations | 0.000225 |
| 20 | Bolt + Canopy | opening | X | Canopy SEP batching covector | 0.000341 |
| 20 | Bolt + Canopy | opening | X | Canopy ancestor claims | 0.000105 |
| 20 | Bolt + Canopy | opening | X | authenticated WHIR alignment and constraint combination | 0.000596 |
| 20 | Bolt + Canopy | opening | X | collect and serialize row authentication paths | 0.005983 |
| 20 | Bolt + Canopy | opening | X | copy and serialize queried row values | 0.004862 |
| 20 | Bolt + Canopy | opening | Y | Canopy OOD evaluations | 0.000190 |
| 20 | Bolt + Canopy | opening | Y | Canopy SEP batching covector | 0.000026 |
| 20 | Bolt + Canopy | opening | Y | Canopy ancestor claims | 0.000003 |
| 20 | Bolt + Canopy | opening | Y | authenticated WHIR alignment and constraint combination | 0.000634 |
| 20 | Bolt + Canopy | opening | Y | collect and serialize row authentication paths | 0.000038 |
| 20 | Bolt + Canopy | opening | Y | copy and serialize queried row values | 0.000079 |
| 20 | Bolt + Canopy | opening | batched WHIR | distinct-functional cross evaluations | 0.008147 |
| 20 | Bolt + Canopy | opening | batched WHIR | shared recursive WHIR proof | 0.012134 |
| 20 | Bolt + Canopy | opening | common WHIR | WHIR configuration | 0.000006 |
| 20 | Bolt + Canopy | opening | other | unassigned phase work and timing overhead | 0.002941 |
| 20 | Bolt + Canopy | opening | u | authenticated WHIR alignment and constraint combination | 0.000882 |
| 20 | Bolt + Canopy | opening | u | common WHIR RS encode and oracle commitment | 0.006956 |
| 20 | Bolt + Canopy | opening | x | authenticated WHIR alignment and constraint combination | 0.019587 |
| 20 | Bolt + Canopy | opening | x | common WHIR RS encode and oracle commitment | 0.016113 |
| 20 | Bolt + Merkle | commitment | Bolt | parity RS encoding Y=RS(U) | 0.008328 |
| 20 | Bolt + Merkle | commitment | Bolt | sparse syndrome U=HX | 0.010194 |
| 20 | Bolt + Merkle | commitment | X | Merkle root transcript | 0.000000 |
| 20 | Bolt + Merkle | commitment | X | row Merkle tree or forest | 0.023924 |
| 20 | Bolt + Merkle | commitment | Y | Merkle root transcript | 0.000001 |
| 20 | Bolt + Merkle | commitment | Y | row Merkle tree or forest | 0.006506 |
| 20 | Bolt + Merkle | commitment | other | unassigned phase work and timing overhead | 0.000001 |
| 20 | Bolt + Merkle | opening | Bolt | column projections x=Xr and u=Ur | 0.003032 |
| 20 | Bolt + Merkle | opening | Bolt | parity query covector and transpose RS FFT | 0.002393 |
| 20 | Bolt + Merkle | opening | Bolt | partial evaluation g | 0.013066 |
| 20 | Bolt + Merkle | opening | Bolt | project queried rows and sample batching challenges | 0.000964 |
| 20 | Bolt + Merkle | opening | Bolt | public H transpose covector | 0.004318 |
| 20 | Bolt + Merkle | opening | Bolt | sample row queries | 0.006727 |
| 20 | Bolt + Merkle | opening | Bolt | syndrome evaluation sigma | 0.000585 |
| 20 | Bolt + Merkle | opening | Bolt | systematic query covector | 0.001192 |
| 20 | Bolt + Merkle | opening | X | collect and serialize row authentication paths | 0.008329 |
| 20 | Bolt + Merkle | opening | X | copy and serialize queried row values | 0.004896 |
| 20 | Bolt + Merkle | opening | Y | collect and serialize row authentication paths | 0.000237 |
| 20 | Bolt + Merkle | opening | Y | copy and serialize queried row values | 0.000080 |
| 20 | Bolt + Merkle | opening | batched WHIR | distinct-functional cross evaluations | 0.001543 |
| 20 | Bolt + Merkle | opening | batched WHIR | shared recursive WHIR proof | 0.006087 |
| 20 | Bolt + Merkle | opening | common WHIR | WHIR configuration | 0.000006 |
| 20 | Bolt + Merkle | opening | other | unassigned phase work and timing overhead | 0.001222 |
| 20 | Bolt + Merkle | opening | u | authenticated WHIR alignment and constraint combination | 0.001043 |
| 20 | Bolt + Merkle | opening | u | common WHIR RS encode and oracle commitment | 0.002523 |
| 20 | Bolt + Merkle | opening | x | authenticated WHIR alignment and constraint combination | 0.015515 |
| 20 | Bolt + Merkle | opening | x | common WHIR RS encode and oracle commitment | 0.016758 |
| 22 | Bolt + Canopy | commitment | Bolt | parity RS encoding Y=RS(U) | 0.037686 |
| 22 | Bolt + Canopy | commitment | Bolt | sparse syndrome U=HX | 0.084850 |
| 22 | Bolt + Canopy | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.032591 |
| 22 | Bolt + Canopy | commitment | X | Canopy WHIR configuration | 0.000006 |
| 22 | Bolt + Canopy | commitment | X | Canopy hash-to-field | 0.021986 |
| 22 | Bolt + Canopy | commitment | X | row Merkle tree or forest | 0.099793 |
| 22 | Bolt + Canopy | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.034759 |
| 22 | Bolt + Canopy | commitment | Y | Canopy WHIR configuration | 0.000005 |
| 22 | Bolt + Canopy | commitment | Y | Canopy hash-to-field | 0.021529 |
| 22 | Bolt + Canopy | commitment | Y | row Merkle tree or forest | 0.020841 |
| 22 | Bolt + Canopy | commitment | other | unassigned phase work and timing overhead | 0.000002 |
| 22 | Bolt + Canopy | opening | Bolt | column projections x=Xr and u=Ur | 0.016292 |
| 22 | Bolt + Canopy | opening | Bolt | parity query covector and transpose RS FFT | 0.010117 |
| 22 | Bolt + Canopy | opening | Bolt | partial evaluation g | 0.057225 |
| 22 | Bolt + Canopy | opening | Bolt | project queried rows and sample batching challenges | 0.001005 |
| 22 | Bolt + Canopy | opening | Bolt | public H transpose covector | 0.056036 |
| 22 | Bolt + Canopy | opening | Bolt | sample row queries | 0.006768 |
| 22 | Bolt + Canopy | opening | Bolt | syndrome evaluation sigma | 0.002259 |
| 22 | Bolt + Canopy | opening | Bolt | systematic query covector | 0.004483 |
| 22 | Bolt + Canopy | opening | X | Canopy OOD evaluations | 0.000595 |
| 22 | Bolt + Canopy | opening | X | Canopy SEP batching covector | 0.000618 |
| 22 | Bolt + Canopy | opening | X | Canopy ancestor claims | 0.000147 |
| 22 | Bolt + Canopy | opening | X | authenticated WHIR alignment and constraint combination | 0.002713 |
| 22 | Bolt + Canopy | opening | X | collect and serialize row authentication paths | 0.008134 |
| 22 | Bolt + Canopy | opening | X | copy and serialize queried row values | 0.005365 |
| 22 | Bolt + Canopy | opening | Y | Canopy OOD evaluations | 0.000523 |
| 22 | Bolt + Canopy | opening | Y | Canopy SEP batching covector | 0.000177 |
| 22 | Bolt + Canopy | opening | Y | Canopy ancestor claims | 0.000005 |
| 22 | Bolt + Canopy | opening | Y | authenticated WHIR alignment and constraint combination | 0.002543 |
| 22 | Bolt + Canopy | opening | Y | collect and serialize row authentication paths | 0.000040 |
| 22 | Bolt + Canopy | opening | Y | copy and serialize queried row values | 0.000088 |
| 22 | Bolt + Canopy | opening | batched WHIR | distinct-functional cross evaluations | 0.035025 |
| 22 | Bolt + Canopy | opening | batched WHIR | shared recursive WHIR proof | 0.051215 |
| 22 | Bolt + Canopy | opening | common WHIR | WHIR configuration | 0.000008 |
| 22 | Bolt + Canopy | opening | other | unassigned phase work and timing overhead | 0.012957 |
| 22 | Bolt + Canopy | opening | u | authenticated WHIR alignment and constraint combination | 0.002965 |
| 22 | Bolt + Canopy | opening | u | common WHIR RS encode and oracle commitment | 0.038518 |
| 22 | Bolt + Canopy | opening | x | authenticated WHIR alignment and constraint combination | 0.094329 |
| 22 | Bolt + Canopy | opening | x | common WHIR RS encode and oracle commitment | 0.090727 |
| 22 | Bolt + Merkle | commitment | Bolt | parity RS encoding Y=RS(U) | 0.046287 |
| 22 | Bolt + Merkle | commitment | Bolt | sparse syndrome U=HX | 0.119650 |
| 22 | Bolt + Merkle | commitment | X | Merkle root transcript | 0.000000 |
| 22 | Bolt + Merkle | commitment | X | row Merkle tree or forest | 0.139047 |
| 22 | Bolt + Merkle | commitment | Y | Merkle root transcript | 0.000001 |
| 22 | Bolt + Merkle | commitment | Y | row Merkle tree or forest | 0.034013 |
| 22 | Bolt + Merkle | commitment | other | unassigned phase work and timing overhead | 0.000001 |
| 22 | Bolt + Merkle | opening | Bolt | column projections x=Xr and u=Ur | 0.019296 |
| 22 | Bolt + Merkle | opening | Bolt | parity query covector and transpose RS FFT | 0.011511 |
| 22 | Bolt + Merkle | opening | Bolt | partial evaluation g | 0.056393 |
| 22 | Bolt + Merkle | opening | Bolt | project queried rows and sample batching challenges | 0.001001 |
| 22 | Bolt + Merkle | opening | Bolt | public H transpose covector | 0.059400 |
| 22 | Bolt + Merkle | opening | Bolt | sample row queries | 0.007723 |
| 22 | Bolt + Merkle | opening | Bolt | syndrome evaluation sigma | 0.002980 |
| 22 | Bolt + Merkle | opening | Bolt | systematic query covector | 0.004953 |
| 22 | Bolt + Merkle | opening | X | collect and serialize row authentication paths | 0.013934 |
| 22 | Bolt + Merkle | opening | X | copy and serialize queried row values | 0.005655 |
| 22 | Bolt + Merkle | opening | Y | collect and serialize row authentication paths | 0.000348 |
| 22 | Bolt + Merkle | opening | Y | copy and serialize queried row values | 0.000087 |
| 22 | Bolt + Merkle | opening | batched WHIR | distinct-functional cross evaluations | 0.006458 |
| 22 | Bolt + Merkle | opening | batched WHIR | shared recursive WHIR proof | 0.019914 |
| 22 | Bolt + Merkle | opening | common WHIR | WHIR configuration | 0.000008 |
| 22 | Bolt + Merkle | opening | other | unassigned phase work and timing overhead | 0.006228 |
| 22 | Bolt + Merkle | opening | u | authenticated WHIR alignment and constraint combination | 0.003327 |
| 22 | Bolt + Merkle | opening | u | common WHIR RS encode and oracle commitment | 0.012877 |
| 22 | Bolt + Merkle | opening | x | authenticated WHIR alignment and constraint combination | 0.069900 |
| 22 | Bolt + Merkle | opening | x | common WHIR RS encode and oracle commitment | 0.092577 |
