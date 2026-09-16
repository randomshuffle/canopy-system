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

| n | eta | Scheme | RS block | h X/Y | Commit | Opening | Total | Median total |
|---|---|---|---|---|---|---|---|---|
| 18 | 1 | Bolt + Canopy | k | 3/1 | 0.01690 | 0.04401 | 0.06091 | 0.05872 |
| 18 | 1 | Bolt + Merkle | - | - | 0.01171 | 0.03604 | 0.04775 | 0.04775 |
| 18 | 2 | Bolt + Canopy | k | 3/1 | 0.03387 | 0.09236 | 0.12623 | 0.12615 |
| 18 | 2 | Bolt + Merkle | - | - | 0.02484 | 0.07478 | 0.09962 | 0.09869 |
| 18 | 4 | Bolt + Canopy | k | 3/1 | 0.06759 | 0.20053 | 0.26812 | 0.26770 |
| 18 | 4 | Bolt + Merkle | - | - | 0.04763 | 0.15182 | 0.19945 | 0.19920 |
| 18 | 8 | Bolt + Canopy | k | 3/1 | 0.13712 | 0.50698 | 0.64410 | 0.63720 |
| 18 | 8 | Bolt + Merkle | - | - | 0.10912 | 0.33585 | 0.44497 | 0.44031 |
| 20 | 1 | Bolt + Canopy | k | 3/1 | 0.06983 | 0.11571 | 0.18554 | 0.18408 |
| 20 | 1 | Bolt + Merkle | - | - | 0.04912 | 0.09470 | 0.14382 | 0.13978 |
| 20 | 2 | Bolt + Canopy | k | 3/1 | 0.13703 | 0.24910 | 0.38614 | 0.38499 |
| 20 | 2 | Bolt + Merkle | - | - | 0.09781 | 0.19006 | 0.28787 | 0.28472 |
| 20 | 4 | Bolt + Canopy | k | 3/1 | 0.27637 | 0.58000 | 0.85637 | 0.85695 |
| 20 | 4 | Bolt + Merkle | - | - | 0.19322 | 0.39801 | 0.59124 | 0.59067 |
| 20 | 8 | Bolt + Canopy | k | 3/1 | 0.55225 | 1.61357 | 2.16583 | 2.11134 |
| 20 | 8 | Bolt + Merkle | - | - | 0.40647 | 0.88095 | 1.28742 | 1.27326 |
| 22 | 1 | Bolt + Canopy | k | 3/1 | 0.29407 | 0.43340 | 0.72747 | 0.73122 |
| 22 | 1 | Bolt + Merkle | - | - | 0.20607 | 0.32295 | 0.52902 | 0.52716 |
| 22 | 2 | Bolt + Canopy | k | 3/1 | 0.60224 | 0.96779 | 1.57003 | 1.54976 |
| 22 | 2 | Bolt + Merkle | - | - | 0.41454 | 0.67981 | 1.09435 | 1.09404 |
| 22 | 4 | Bolt + Canopy | k | 3/1 | 1.21215 | 2.42703 | 3.63918 | 3.64556 |
| 22 | 4 | Bolt + Merkle | - | - | 0.84730 | 1.47088 | 2.31818 | 2.31373 |
| 22 | 8 | Bolt + Canopy | k | 3/1 | 2.72702 | 7.24598 | 9.97300 | 10.34931 |
| 22 | 8 | Bolt + Merkle | - | - | 1.78424 | 3.94239 | 5.72664 | 5.24628 |

## Bolt: stages at the smallest-proof configurations

| n | eta | Scheme | Phase | Part | Stage | Mean seconds |
|---|---|---|---|---|---|---|
| 18 | 1 | Bolt + Canopy | commitment | Bolt | parity RS encoding Y=RS(U) | 0.002092 |
| 18 | 1 | Bolt + Canopy | commitment | Bolt | sparse syndrome U=HX | 0.001262 |
| 18 | 1 | Bolt + Canopy | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.001840 |
| 18 | 1 | Bolt + Canopy | commitment | X | Canopy WHIR configuration | 0.000002 |
| 18 | 1 | Bolt + Canopy | commitment | X | Canopy hash-to-field | 0.001333 |
| 18 | 1 | Bolt + Canopy | commitment | X | row Merkle tree or forest | 0.005853 |
| 18 | 1 | Bolt + Canopy | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.001779 |
| 18 | 1 | Bolt + Canopy | commitment | Y | Canopy WHIR configuration | 0.000002 |
| 18 | 1 | Bolt + Canopy | commitment | Y | Canopy hash-to-field | 0.001326 |
| 18 | 1 | Bolt + Canopy | commitment | Y | row Merkle tree or forest | 0.001405 |
| 18 | 1 | Bolt + Canopy | commitment | other | unassigned phase work and timing overhead | 0.000003 |
| 18 | 1 | Bolt + Canopy | opening | Bolt | column projections x=Xr and u=Ur | 0.000885 |
| 18 | 1 | Bolt + Canopy | opening | Bolt | parity query covector and transpose RS FFT | 0.001051 |
| 18 | 1 | Bolt + Canopy | opening | Bolt | partial evaluation g | 0.003289 |
| 18 | 1 | Bolt + Canopy | opening | Bolt | project queried rows and sample batching challenges | 0.000972 |
| 18 | 1 | Bolt + Canopy | opening | Bolt | public H transpose covector | 0.000863 |
| 18 | 1 | Bolt + Canopy | opening | Bolt | sample row queries | 0.006020 |
| 18 | 1 | Bolt + Canopy | opening | Bolt | syndrome evaluation sigma | 0.000219 |
| 18 | 1 | Bolt + Canopy | opening | Bolt | systematic query covector | 0.000551 |
| 18 | 1 | Bolt + Canopy | opening | X | Canopy OOD evaluations | 0.000099 |
| 18 | 1 | Bolt + Canopy | opening | X | Canopy SEP batching covector | 0.000173 |
| 18 | 1 | Bolt + Canopy | opening | X | Canopy ancestor claims | 0.000081 |
| 18 | 1 | Bolt + Canopy | opening | X | collect and serialize row authentication paths | 0.003546 |
| 18 | 1 | Bolt + Canopy | opening | X | copy and serialize queried row values | 0.004193 |
| 18 | 1 | Bolt + Canopy | opening | Y | Canopy OOD evaluations | 0.000091 |
| 18 | 1 | Bolt + Canopy | opening | Y | Canopy SEP batching covector | 0.000012 |
| 18 | 1 | Bolt + Canopy | opening | Y | Canopy ancestor claims | 0.000003 |
| 18 | 1 | Bolt + Canopy | opening | Y | collect and serialize row authentication paths | 0.000037 |
| 18 | 1 | Bolt + Canopy | opening | Y | copy and serialize queried row values | 0.000092 |
| 18 | 1 | Bolt + Canopy | opening | batched WHIR | distinct-functional cross evaluations | 0.002010 |
| 18 | 1 | Bolt + Canopy | opening | batched WHIR | shared recursive WHIR proof | 0.003729 |
| 18 | 1 | Bolt + Canopy | opening | common WHIR | WHIR configuration | 0.000003 |
| 18 | 1 | Bolt + Canopy | opening | other | unassigned phase work and timing overhead | 0.002841 |
| 18 | 1 | Bolt + Canopy | opening | source | authenticated WHIR alignment and constraint combination | 0.001751 |
| 18 | 1 | Bolt + Canopy | opening | u | common WHIR RS encode and oracle commitment | 0.001940 |
| 18 | 1 | Bolt + Canopy | opening | x | common WHIR RS encode and oracle commitment | 0.004307 |
| 18 | 1 | Bolt + Merkle | commitment | Bolt | parity RS encoding Y=RS(U) | 0.001998 |
| 18 | 1 | Bolt + Merkle | commitment | Bolt | sparse syndrome U=HX | 0.001276 |
| 18 | 1 | Bolt + Merkle | commitment | X | Merkle root transcript | 0.000000 |
| 18 | 1 | Bolt + Merkle | commitment | X | row Merkle tree or forest | 0.006369 |
| 18 | 1 | Bolt + Merkle | commitment | Y | Merkle root transcript | 0.000000 |
| 18 | 1 | Bolt + Merkle | commitment | Y | row Merkle tree or forest | 0.002064 |
| 18 | 1 | Bolt + Merkle | commitment | other | unassigned phase work and timing overhead | 0.000002 |
| 18 | 1 | Bolt + Merkle | opening | Bolt | column projections x=Xr and u=Ur | 0.000864 |
| 18 | 1 | Bolt + Merkle | opening | Bolt | parity query covector and transpose RS FFT | 0.001018 |
| 18 | 1 | Bolt + Merkle | opening | Bolt | partial evaluation g | 0.003242 |
| 18 | 1 | Bolt + Merkle | opening | Bolt | project queried rows and sample batching challenges | 0.000955 |
| 18 | 1 | Bolt + Merkle | opening | Bolt | public H transpose covector | 0.000804 |
| 18 | 1 | Bolt + Merkle | opening | Bolt | sample row queries | 0.005977 |
| 18 | 1 | Bolt + Merkle | opening | Bolt | syndrome evaluation sigma | 0.000213 |
| 18 | 1 | Bolt + Merkle | opening | Bolt | systematic query covector | 0.000519 |
| 18 | 1 | Bolt + Merkle | opening | X | collect and serialize row authentication paths | 0.004103 |
| 18 | 1 | Bolt + Merkle | opening | X | copy and serialize queried row values | 0.004100 |
| 18 | 1 | Bolt + Merkle | opening | Y | collect and serialize row authentication paths | 0.000176 |
| 18 | 1 | Bolt + Merkle | opening | Y | copy and serialize queried row values | 0.000078 |
| 18 | 1 | Bolt + Merkle | opening | batched WHIR | distinct-functional cross evaluations | 0.000355 |
| 18 | 1 | Bolt + Merkle | opening | batched WHIR | shared recursive WHIR proof | 0.002209 |
| 18 | 1 | Bolt + Merkle | opening | common WHIR | WHIR configuration | 0.000003 |
| 18 | 1 | Bolt + Merkle | opening | other | unassigned phase work and timing overhead | 0.001228 |
| 18 | 1 | Bolt + Merkle | opening | source | authenticated WHIR alignment and constraint combination | 0.002660 |
| 18 | 1 | Bolt + Merkle | opening | u | common WHIR RS encode and oracle commitment | 0.000750 |
| 18 | 1 | Bolt + Merkle | opening | x | common WHIR RS encode and oracle commitment | 0.004125 |
| 18 | 2 | Bolt + Canopy | commitment | Bolt | parity RS encoding Y=RS(U) | 0.002063 |
| 18 | 2 | Bolt + Canopy | commitment | Bolt | sparse syndrome U=HX | 0.001364 |
| 18 | 2 | Bolt + Canopy | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.001893 |
| 18 | 2 | Bolt + Canopy | commitment | X | Canopy WHIR configuration | 0.000002 |
| 18 | 2 | Bolt + Canopy | commitment | X | Canopy hash-to-field | 0.001313 |
| 18 | 2 | Bolt + Canopy | commitment | X | row Merkle tree or forest | 0.005884 |
| 18 | 2 | Bolt + Canopy | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.001771 |
| 18 | 2 | Bolt + Canopy | commitment | Y | Canopy WHIR configuration | 0.000002 |
| 18 | 2 | Bolt + Canopy | commitment | Y | Canopy hash-to-field | 0.001306 |
| 18 | 2 | Bolt + Canopy | commitment | Y | row Merkle tree or forest | 0.001334 |
| 18 | 2 | Bolt + Canopy | commitment | other | unassigned phase work and timing overhead | 0.000008 |
| 18 | 2 | Bolt + Canopy | opening | Bolt | column projections x=Xr and u=Ur | 0.000847 |
| 18 | 2 | Bolt + Canopy | opening | Bolt | parity query covector and transpose RS FFT | 0.001027 |
| 18 | 2 | Bolt + Canopy | opening | Bolt | partial evaluation g | 0.003269 |
| 18 | 2 | Bolt + Canopy | opening | Bolt | project queried rows and sample batching challenges | 0.000987 |
| 18 | 2 | Bolt + Canopy | opening | Bolt | public H transpose covector | 0.000856 |
| 18 | 2 | Bolt + Canopy | opening | Bolt | sample row queries | 0.006060 |
| 18 | 2 | Bolt + Canopy | opening | Bolt | syndrome evaluation sigma | 0.000216 |
| 18 | 2 | Bolt + Canopy | opening | Bolt | systematic query covector | 0.000539 |
| 18 | 2 | Bolt + Canopy | opening | X | Canopy OOD evaluations | 0.000096 |
| 18 | 2 | Bolt + Canopy | opening | X | Canopy SEP batching covector | 0.000171 |
| 18 | 2 | Bolt + Canopy | opening | X | Canopy ancestor claims | 0.000078 |
| 18 | 2 | Bolt + Canopy | opening | X | collect and serialize row authentication paths | 0.003512 |
| 18 | 2 | Bolt + Canopy | opening | X | copy and serialize queried row values | 0.005150 |
| 18 | 2 | Bolt + Canopy | opening | Y | Canopy OOD evaluations | 0.000092 |
| 18 | 2 | Bolt + Canopy | opening | Y | Canopy SEP batching covector | 0.000011 |
| 18 | 2 | Bolt + Canopy | opening | Y | Canopy ancestor claims | 0.000004 |
| 18 | 2 | Bolt + Canopy | opening | Y | collect and serialize row authentication paths | 0.000036 |
| 18 | 2 | Bolt + Canopy | opening | Y | copy and serialize queried row values | 0.000078 |
| 18 | 2 | Bolt + Canopy | opening | batched WHIR | distinct-functional cross evaluations | 0.009388 |
| 18 | 2 | Bolt + Canopy | opening | batched WHIR | shared recursive WHIR proof | 0.005747 |
| 18 | 2 | Bolt + Canopy | opening | common WHIR | WHIR configuration | 0.000003 |
| 18 | 2 | Bolt + Canopy | opening | other | unassigned phase work and timing overhead | 0.003551 |
| 18 | 2 | Bolt + Canopy | opening | source | authenticated WHIR alignment and constraint combination | 0.001922 |
| 18 | 2 | Bolt + Canopy | opening | u | common WHIR RS encode and oracle commitment | 0.001957 |
| 18 | 2 | Bolt + Canopy | opening | x | common WHIR RS encode and oracle commitment | 0.004159 |
| 18 | 2 | Bolt + Merkle | commitment | Bolt | parity RS encoding Y=RS(U) | 0.002018 |
| 18 | 2 | Bolt + Merkle | commitment | Bolt | sparse syndrome U=HX | 0.001351 |
| 18 | 2 | Bolt + Merkle | commitment | X | Merkle root transcript | 0.000000 |
| 18 | 2 | Bolt + Merkle | commitment | X | row Merkle tree or forest | 0.006503 |
| 18 | 2 | Bolt + Merkle | commitment | Y | Merkle root transcript | 0.000000 |
| 18 | 2 | Bolt + Merkle | commitment | Y | row Merkle tree or forest | 0.002548 |
| 18 | 2 | Bolt + Merkle | commitment | other | unassigned phase work and timing overhead | 0.000003 |
| 18 | 2 | Bolt + Merkle | opening | Bolt | column projections x=Xr and u=Ur | 0.000854 |
| 18 | 2 | Bolt + Merkle | opening | Bolt | parity query covector and transpose RS FFT | 0.001027 |
| 18 | 2 | Bolt + Merkle | opening | Bolt | partial evaluation g | 0.003252 |
| 18 | 2 | Bolt + Merkle | opening | Bolt | project queried rows and sample batching challenges | 0.000977 |
| 18 | 2 | Bolt + Merkle | opening | Bolt | public H transpose covector | 0.000816 |
| 18 | 2 | Bolt + Merkle | opening | Bolt | sample row queries | 0.006113 |
| 18 | 2 | Bolt + Merkle | opening | Bolt | syndrome evaluation sigma | 0.000221 |
| 18 | 2 | Bolt + Merkle | opening | Bolt | systematic query covector | 0.000514 |
| 18 | 2 | Bolt + Merkle | opening | X | collect and serialize row authentication paths | 0.004174 |
| 18 | 2 | Bolt + Merkle | opening | X | copy and serialize queried row values | 0.004717 |
| 18 | 2 | Bolt + Merkle | opening | Y | collect and serialize row authentication paths | 0.000185 |
| 18 | 2 | Bolt + Merkle | opening | Y | copy and serialize queried row values | 0.000079 |
| 18 | 2 | Bolt + Merkle | opening | batched WHIR | distinct-functional cross evaluations | 0.002061 |
| 18 | 2 | Bolt + Merkle | opening | batched WHIR | shared recursive WHIR proof | 0.003106 |
| 18 | 2 | Bolt + Merkle | opening | common WHIR | WHIR configuration | 0.000004 |
| 18 | 2 | Bolt + Merkle | opening | other | unassigned phase work and timing overhead | 0.002819 |
| 18 | 2 | Bolt + Merkle | opening | source | authenticated WHIR alignment and constraint combination | 0.002698 |
| 18 | 2 | Bolt + Merkle | opening | u | common WHIR RS encode and oracle commitment | 0.000818 |
| 18 | 2 | Bolt + Merkle | opening | x | common WHIR RS encode and oracle commitment | 0.004251 |
| 18 | 4 | Bolt + Canopy | commitment | Bolt | parity RS encoding Y=RS(U) | 0.002039 |
| 18 | 4 | Bolt + Canopy | commitment | Bolt | sparse syndrome U=HX | 0.001382 |
| 18 | 4 | Bolt + Canopy | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.001986 |
| 18 | 4 | Bolt + Canopy | commitment | X | Canopy WHIR configuration | 0.000003 |
| 18 | 4 | Bolt + Canopy | commitment | X | Canopy hash-to-field | 0.001315 |
| 18 | 4 | Bolt + Canopy | commitment | X | row Merkle tree or forest | 0.005770 |
| 18 | 4 | Bolt + Canopy | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.001739 |
| 18 | 4 | Bolt + Canopy | commitment | Y | Canopy WHIR configuration | 0.000002 |
| 18 | 4 | Bolt + Canopy | commitment | Y | Canopy hash-to-field | 0.001308 |
| 18 | 4 | Bolt + Canopy | commitment | Y | row Merkle tree or forest | 0.001349 |
| 18 | 4 | Bolt + Canopy | commitment | other | unassigned phase work and timing overhead | 0.000014 |
| 18 | 4 | Bolt + Canopy | opening | Bolt | column projections x=Xr and u=Ur | 0.000829 |
| 18 | 4 | Bolt + Canopy | opening | Bolt | parity query covector and transpose RS FFT | 0.001013 |
| 18 | 4 | Bolt + Canopy | opening | Bolt | partial evaluation g | 0.003257 |
| 18 | 4 | Bolt + Canopy | opening | Bolt | project queried rows and sample batching challenges | 0.000977 |
| 18 | 4 | Bolt + Canopy | opening | Bolt | public H transpose covector | 0.000817 |
| 18 | 4 | Bolt + Canopy | opening | Bolt | sample row queries | 0.006094 |
| 18 | 4 | Bolt + Canopy | opening | Bolt | syndrome evaluation sigma | 0.000217 |
| 18 | 4 | Bolt + Canopy | opening | Bolt | systematic query covector | 0.000545 |
| 18 | 4 | Bolt + Canopy | opening | X | Canopy OOD evaluations | 0.000095 |
| 18 | 4 | Bolt + Canopy | opening | X | Canopy SEP batching covector | 0.000170 |
| 18 | 4 | Bolt + Canopy | opening | X | Canopy ancestor claims | 0.000078 |
| 18 | 4 | Bolt + Canopy | opening | X | collect and serialize row authentication paths | 0.003509 |
| 18 | 4 | Bolt + Canopy | opening | X | copy and serialize queried row values | 0.004679 |
| 18 | 4 | Bolt + Canopy | opening | Y | Canopy OOD evaluations | 0.000091 |
| 18 | 4 | Bolt + Canopy | opening | Y | Canopy SEP batching covector | 0.000011 |
| 18 | 4 | Bolt + Canopy | opening | Y | Canopy ancestor claims | 0.000004 |
| 18 | 4 | Bolt + Canopy | opening | Y | collect and serialize row authentication paths | 0.000036 |
| 18 | 4 | Bolt + Canopy | opening | Y | copy and serialize queried row values | 0.000078 |
| 18 | 4 | Bolt + Canopy | opening | batched WHIR | distinct-functional cross evaluations | 0.040235 |
| 18 | 4 | Bolt + Canopy | opening | batched WHIR | shared recursive WHIR proof | 0.011857 |
| 18 | 4 | Bolt + Canopy | opening | common WHIR | WHIR configuration | 0.000003 |
| 18 | 4 | Bolt + Canopy | opening | other | unassigned phase work and timing overhead | 0.006762 |
| 18 | 4 | Bolt + Canopy | opening | source | authenticated WHIR alignment and constraint combination | 0.001708 |
| 18 | 4 | Bolt + Canopy | opening | u | common WHIR RS encode and oracle commitment | 0.001951 |
| 18 | 4 | Bolt + Canopy | opening | x | common WHIR RS encode and oracle commitment | 0.004136 |
| 18 | 4 | Bolt + Merkle | commitment | Bolt | parity RS encoding Y=RS(U) | 0.002113 |
| 18 | 4 | Bolt + Merkle | commitment | Bolt | sparse syndrome U=HX | 0.001393 |
| 18 | 4 | Bolt + Merkle | commitment | X | Merkle root transcript | 0.000000 |
| 18 | 4 | Bolt + Merkle | commitment | X | row Merkle tree or forest | 0.006409 |
| 18 | 4 | Bolt + Merkle | commitment | Y | Merkle root transcript | 0.000000 |
| 18 | 4 | Bolt + Merkle | commitment | Y | row Merkle tree or forest | 0.001990 |
| 18 | 4 | Bolt + Merkle | commitment | other | unassigned phase work and timing overhead | 0.000005 |
| 18 | 4 | Bolt + Merkle | opening | Bolt | column projections x=Xr and u=Ur | 0.000841 |
| 18 | 4 | Bolt + Merkle | opening | Bolt | parity query covector and transpose RS FFT | 0.001019 |
| 18 | 4 | Bolt + Merkle | opening | Bolt | partial evaluation g | 0.003227 |
| 18 | 4 | Bolt + Merkle | opening | Bolt | project queried rows and sample batching challenges | 0.000973 |
| 18 | 4 | Bolt + Merkle | opening | Bolt | public H transpose covector | 0.000788 |
| 18 | 4 | Bolt + Merkle | opening | Bolt | sample row queries | 0.006074 |
| 18 | 4 | Bolt + Merkle | opening | Bolt | syndrome evaluation sigma | 0.000216 |
| 18 | 4 | Bolt + Merkle | opening | Bolt | systematic query covector | 0.000513 |
| 18 | 4 | Bolt + Merkle | opening | X | collect and serialize row authentication paths | 0.004074 |
| 18 | 4 | Bolt + Merkle | opening | X | copy and serialize queried row values | 0.004603 |
| 18 | 4 | Bolt + Merkle | opening | Y | collect and serialize row authentication paths | 0.000184 |
| 18 | 4 | Bolt + Merkle | opening | Y | copy and serialize queried row values | 0.000077 |
| 18 | 4 | Bolt + Merkle | opening | batched WHIR | distinct-functional cross evaluations | 0.009507 |
| 18 | 4 | Bolt + Merkle | opening | batched WHIR | shared recursive WHIR proof | 0.005195 |
| 18 | 4 | Bolt + Merkle | opening | common WHIR | WHIR configuration | 0.000004 |
| 18 | 4 | Bolt + Merkle | opening | other | unassigned phase work and timing overhead | 0.005398 |
| 18 | 4 | Bolt + Merkle | opening | source | authenticated WHIR alignment and constraint combination | 0.002683 |
| 18 | 4 | Bolt + Merkle | opening | u | common WHIR RS encode and oracle commitment | 0.000803 |
| 18 | 4 | Bolt + Merkle | opening | x | common WHIR RS encode and oracle commitment | 0.004169 |
| 18 | 8 | Bolt + Canopy | commitment | Bolt | parity RS encoding Y=RS(U) | 0.002083 |
| 18 | 8 | Bolt + Canopy | commitment | Bolt | sparse syndrome U=HX | 0.001423 |
| 18 | 8 | Bolt + Canopy | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.001878 |
| 18 | 8 | Bolt + Canopy | commitment | X | Canopy WHIR configuration | 0.000002 |
| 18 | 8 | Bolt + Canopy | commitment | X | Canopy hash-to-field | 0.001312 |
| 18 | 8 | Bolt + Canopy | commitment | X | row Merkle tree or forest | 0.005961 |
| 18 | 8 | Bolt + Canopy | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.001794 |
| 18 | 8 | Bolt + Canopy | commitment | Y | Canopy WHIR configuration | 0.000002 |
| 18 | 8 | Bolt + Canopy | commitment | Y | Canopy hash-to-field | 0.001313 |
| 18 | 8 | Bolt + Canopy | commitment | Y | row Merkle tree or forest | 0.001371 |
| 18 | 8 | Bolt + Canopy | commitment | other | unassigned phase work and timing overhead | 0.000019 |
| 18 | 8 | Bolt + Canopy | opening | Bolt | column projections x=Xr and u=Ur | 0.000893 |
| 18 | 8 | Bolt + Canopy | opening | Bolt | parity query covector and transpose RS FFT | 0.001025 |
| 18 | 8 | Bolt + Canopy | opening | Bolt | partial evaluation g | 0.003241 |
| 18 | 8 | Bolt + Canopy | opening | Bolt | project queried rows and sample batching challenges | 0.000993 |
| 18 | 8 | Bolt + Canopy | opening | Bolt | public H transpose covector | 0.000802 |
| 18 | 8 | Bolt + Canopy | opening | Bolt | sample row queries | 0.006208 |
| 18 | 8 | Bolt + Canopy | opening | Bolt | syndrome evaluation sigma | 0.000217 |
| 18 | 8 | Bolt + Canopy | opening | Bolt | systematic query covector | 0.000556 |
| 18 | 8 | Bolt + Canopy | opening | X | Canopy OOD evaluations | 0.000312 |
| 18 | 8 | Bolt + Canopy | opening | X | Canopy SEP batching covector | 0.000169 |
| 18 | 8 | Bolt + Canopy | opening | X | Canopy ancestor claims | 0.000076 |
| 18 | 8 | Bolt + Canopy | opening | X | collect and serialize row authentication paths | 0.003584 |
| 18 | 8 | Bolt + Canopy | opening | X | copy and serialize queried row values | 0.004610 |
| 18 | 8 | Bolt + Canopy | opening | Y | Canopy OOD evaluations | 0.000093 |
| 18 | 8 | Bolt + Canopy | opening | Y | Canopy SEP batching covector | 0.000013 |
| 18 | 8 | Bolt + Canopy | opening | Y | Canopy ancestor claims | 0.000004 |
| 18 | 8 | Bolt + Canopy | opening | Y | collect and serialize row authentication paths | 0.000039 |
| 18 | 8 | Bolt + Canopy | opening | Y | copy and serialize queried row values | 0.000083 |
| 18 | 8 | Bolt + Canopy | opening | batched WHIR | distinct-functional cross evaluations | 0.168455 |
| 18 | 8 | Bolt + Canopy | opening | batched WHIR | shared recursive WHIR proof | 0.033186 |
| 18 | 8 | Bolt + Canopy | opening | common WHIR | WHIR configuration | 0.000004 |
| 18 | 8 | Bolt + Canopy | opening | other | unassigned phase work and timing overhead | 0.014113 |
| 18 | 8 | Bolt + Canopy | opening | source | authenticated WHIR alignment and constraint combination | 0.001772 |
| 18 | 8 | Bolt + Canopy | opening | u | common WHIR RS encode and oracle commitment | 0.002038 |
| 18 | 8 | Bolt + Canopy | opening | x | common WHIR RS encode and oracle commitment | 0.004353 |
| 18 | 8 | Bolt + Merkle | commitment | Bolt | parity RS encoding Y=RS(U) | 0.002486 |
| 18 | 8 | Bolt + Merkle | commitment | Bolt | sparse syndrome U=HX | 0.001615 |
| 18 | 8 | Bolt + Merkle | commitment | X | Merkle root transcript | 0.000000 |
| 18 | 8 | Bolt + Merkle | commitment | X | row Merkle tree or forest | 0.007306 |
| 18 | 8 | Bolt + Merkle | commitment | Y | Merkle root transcript | 0.000000 |
| 18 | 8 | Bolt + Merkle | commitment | Y | row Merkle tree or forest | 0.002229 |
| 18 | 8 | Bolt + Merkle | commitment | other | unassigned phase work and timing overhead | 0.000014 |
| 18 | 8 | Bolt + Merkle | opening | Bolt | column projections x=Xr and u=Ur | 0.000879 |
| 18 | 8 | Bolt + Merkle | opening | Bolt | parity query covector and transpose RS FFT | 0.001032 |
| 18 | 8 | Bolt + Merkle | opening | Bolt | partial evaluation g | 0.003256 |
| 18 | 8 | Bolt + Merkle | opening | Bolt | project queried rows and sample batching challenges | 0.000990 |
| 18 | 8 | Bolt + Merkle | opening | Bolt | public H transpose covector | 0.000803 |
| 18 | 8 | Bolt + Merkle | opening | Bolt | sample row queries | 0.006194 |
| 18 | 8 | Bolt + Merkle | opening | Bolt | syndrome evaluation sigma | 0.000218 |
| 18 | 8 | Bolt + Merkle | opening | Bolt | systematic query covector | 0.000530 |
| 18 | 8 | Bolt + Merkle | opening | X | collect and serialize row authentication paths | 0.004206 |
| 18 | 8 | Bolt + Merkle | opening | X | copy and serialize queried row values | 0.004610 |
| 18 | 8 | Bolt + Merkle | opening | Y | collect and serialize row authentication paths | 0.000178 |
| 18 | 8 | Bolt + Merkle | opening | Y | copy and serialize queried row values | 0.000083 |
| 18 | 8 | Bolt + Merkle | opening | batched WHIR | distinct-functional cross evaluations | 0.040735 |
| 18 | 8 | Bolt + Merkle | opening | batched WHIR | shared recursive WHIR proof | 0.012935 |
| 18 | 8 | Bolt + Merkle | opening | common WHIR | WHIR configuration | 0.000004 |
| 18 | 8 | Bolt + Merkle | opening | other | unassigned phase work and timing overhead | 0.010316 |
| 18 | 8 | Bolt + Merkle | opening | source | authenticated WHIR alignment and constraint combination | 0.002888 |
| 18 | 8 | Bolt + Merkle | opening | u | common WHIR RS encode and oracle commitment | 0.000870 |
| 18 | 8 | Bolt + Merkle | opening | x | common WHIR RS encode and oracle commitment | 0.004355 |
| 20 | 1 | Bolt + Canopy | commitment | Bolt | parity RS encoding Y=RS(U) | 0.008546 |
| 20 | 1 | Bolt + Canopy | commitment | Bolt | sparse syndrome U=HX | 0.009720 |
| 20 | 1 | Bolt + Canopy | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.006908 |
| 20 | 1 | Bolt + Canopy | commitment | X | Canopy WHIR configuration | 0.000005 |
| 20 | 1 | Bolt + Canopy | commitment | X | Canopy hash-to-field | 0.005310 |
| 20 | 1 | Bolt + Canopy | commitment | X | row Merkle tree or forest | 0.022523 |
| 20 | 1 | Bolt + Canopy | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.006581 |
| 20 | 1 | Bolt + Canopy | commitment | Y | Canopy WHIR configuration | 0.000003 |
| 20 | 1 | Bolt + Canopy | commitment | Y | Canopy hash-to-field | 0.005247 |
| 20 | 1 | Bolt + Canopy | commitment | Y | row Merkle tree or forest | 0.004984 |
| 20 | 1 | Bolt + Canopy | commitment | other | unassigned phase work and timing overhead | 0.000005 |
| 20 | 1 | Bolt + Canopy | opening | Bolt | column projections x=Xr and u=Ur | 0.002998 |
| 20 | 1 | Bolt + Canopy | opening | Bolt | parity query covector and transpose RS FFT | 0.002392 |
| 20 | 1 | Bolt + Canopy | opening | Bolt | partial evaluation g | 0.012723 |
| 20 | 1 | Bolt + Canopy | opening | Bolt | project queried rows and sample batching challenges | 0.000959 |
| 20 | 1 | Bolt + Canopy | opening | Bolt | public H transpose covector | 0.004601 |
| 20 | 1 | Bolt + Canopy | opening | Bolt | sample row queries | 0.006832 |
| 20 | 1 | Bolt + Canopy | opening | Bolt | syndrome evaluation sigma | 0.000563 |
| 20 | 1 | Bolt + Canopy | opening | Bolt | systematic query covector | 0.001273 |
| 20 | 1 | Bolt + Canopy | opening | X | Canopy OOD evaluations | 0.000230 |
| 20 | 1 | Bolt + Canopy | opening | X | Canopy SEP batching covector | 0.000346 |
| 20 | 1 | Bolt + Canopy | opening | X | Canopy ancestor claims | 0.000112 |
| 20 | 1 | Bolt + Canopy | opening | X | collect and serialize row authentication paths | 0.006116 |
| 20 | 1 | Bolt + Canopy | opening | X | copy and serialize queried row values | 0.004959 |
| 20 | 1 | Bolt + Canopy | opening | Y | Canopy OOD evaluations | 0.000211 |
| 20 | 1 | Bolt + Canopy | opening | Y | Canopy SEP batching covector | 0.000034 |
| 20 | 1 | Bolt + Canopy | opening | Y | Canopy ancestor claims | 0.000003 |
| 20 | 1 | Bolt + Canopy | opening | Y | collect and serialize row authentication paths | 0.000037 |
| 20 | 1 | Bolt + Canopy | opening | Y | copy and serialize queried row values | 0.000081 |
| 20 | 1 | Bolt + Canopy | opening | batched WHIR | distinct-functional cross evaluations | 0.008195 |
| 20 | 1 | Bolt + Canopy | opening | batched WHIR | shared recursive WHIR proof | 0.012644 |
| 20 | 1 | Bolt + Canopy | opening | common WHIR | WHIR configuration | 0.000005 |
| 20 | 1 | Bolt + Canopy | opening | other | unassigned phase work and timing overhead | 0.004324 |
| 20 | 1 | Bolt + Canopy | opening | source | authenticated WHIR alignment and constraint combination | 0.005673 |
| 20 | 1 | Bolt + Canopy | opening | u | common WHIR RS encode and oracle commitment | 0.007126 |
| 20 | 1 | Bolt + Canopy | opening | x | common WHIR RS encode and oracle commitment | 0.016254 |
| 20 | 1 | Bolt + Merkle | commitment | Bolt | parity RS encoding Y=RS(U) | 0.008798 |
| 20 | 1 | Bolt + Merkle | commitment | Bolt | sparse syndrome U=HX | 0.009677 |
| 20 | 1 | Bolt + Merkle | commitment | X | Merkle root transcript | 0.000000 |
| 20 | 1 | Bolt + Merkle | commitment | X | row Merkle tree or forest | 0.024019 |
| 20 | 1 | Bolt + Merkle | commitment | Y | Merkle root transcript | 0.000000 |
| 20 | 1 | Bolt + Merkle | commitment | Y | row Merkle tree or forest | 0.006618 |
| 20 | 1 | Bolt + Merkle | commitment | other | unassigned phase work and timing overhead | 0.000008 |
| 20 | 1 | Bolt + Merkle | opening | Bolt | column projections x=Xr and u=Ur | 0.003084 |
| 20 | 1 | Bolt + Merkle | opening | Bolt | parity query covector and transpose RS FFT | 0.002520 |
| 20 | 1 | Bolt + Merkle | opening | Bolt | partial evaluation g | 0.013002 |
| 20 | 1 | Bolt + Merkle | opening | Bolt | project queried rows and sample batching challenges | 0.000985 |
| 20 | 1 | Bolt + Merkle | opening | Bolt | public H transpose covector | 0.004656 |
| 20 | 1 | Bolt + Merkle | opening | Bolt | sample row queries | 0.006823 |
| 20 | 1 | Bolt + Merkle | opening | Bolt | syndrome evaluation sigma | 0.000601 |
| 20 | 1 | Bolt + Merkle | opening | Bolt | systematic query covector | 0.001276 |
| 20 | 1 | Bolt + Merkle | opening | X | collect and serialize row authentication paths | 0.008381 |
| 20 | 1 | Bolt + Merkle | opening | X | copy and serialize queried row values | 0.005222 |
| 20 | 1 | Bolt + Merkle | opening | Y | collect and serialize row authentication paths | 0.000245 |
| 20 | 1 | Bolt + Merkle | opening | Y | copy and serialize queried row values | 0.000081 |
| 20 | 1 | Bolt + Merkle | opening | batched WHIR | distinct-functional cross evaluations | 0.001398 |
| 20 | 1 | Bolt + Merkle | opening | batched WHIR | shared recursive WHIR proof | 0.006219 |
| 20 | 1 | Bolt + Merkle | opening | common WHIR | WHIR configuration | 0.000007 |
| 20 | 1 | Bolt + Merkle | opening | other | unassigned phase work and timing overhead | 0.003121 |
| 20 | 1 | Bolt + Merkle | opening | source | authenticated WHIR alignment and constraint combination | 0.008645 |
| 20 | 1 | Bolt + Merkle | opening | u | common WHIR RS encode and oracle commitment | 0.002578 |
| 20 | 1 | Bolt + Merkle | opening | x | common WHIR RS encode and oracle commitment | 0.017212 |
| 20 | 2 | Bolt + Canopy | commitment | Bolt | parity RS encoding Y=RS(U) | 0.008170 |
| 20 | 2 | Bolt + Canopy | commitment | Bolt | sparse syndrome U=HX | 0.009699 |
| 20 | 2 | Bolt + Canopy | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.006822 |
| 20 | 2 | Bolt + Canopy | commitment | X | Canopy WHIR configuration | 0.000003 |
| 20 | 2 | Bolt + Canopy | commitment | X | Canopy hash-to-field | 0.005214 |
| 20 | 2 | Bolt + Canopy | commitment | X | row Merkle tree or forest | 0.022052 |
| 20 | 2 | Bolt + Canopy | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.006517 |
| 20 | 2 | Bolt + Canopy | commitment | Y | Canopy WHIR configuration | 0.000003 |
| 20 | 2 | Bolt + Canopy | commitment | Y | Canopy hash-to-field | 0.005196 |
| 20 | 2 | Bolt + Canopy | commitment | Y | row Merkle tree or forest | 0.004836 |
| 20 | 2 | Bolt + Canopy | commitment | other | unassigned phase work and timing overhead | 0.000008 |
| 20 | 2 | Bolt + Canopy | opening | Bolt | column projections x=Xr and u=Ur | 0.003001 |
| 20 | 2 | Bolt + Canopy | opening | Bolt | parity query covector and transpose RS FFT | 0.002343 |
| 20 | 2 | Bolt + Canopy | opening | Bolt | partial evaluation g | 0.012782 |
| 20 | 2 | Bolt + Canopy | opening | Bolt | project queried rows and sample batching challenges | 0.000969 |
| 20 | 2 | Bolt + Canopy | opening | Bolt | public H transpose covector | 0.004366 |
| 20 | 2 | Bolt + Canopy | opening | Bolt | sample row queries | 0.006836 |
| 20 | 2 | Bolt + Canopy | opening | Bolt | syndrome evaluation sigma | 0.000550 |
| 20 | 2 | Bolt + Canopy | opening | Bolt | systematic query covector | 0.001298 |
| 20 | 2 | Bolt + Canopy | opening | X | Canopy OOD evaluations | 0.000219 |
| 20 | 2 | Bolt + Canopy | opening | X | Canopy SEP batching covector | 0.000356 |
| 20 | 2 | Bolt + Canopy | opening | X | Canopy ancestor claims | 0.000105 |
| 20 | 2 | Bolt + Canopy | opening | X | collect and serialize row authentication paths | 0.006305 |
| 20 | 2 | Bolt + Canopy | opening | X | copy and serialize queried row values | 0.004956 |
| 20 | 2 | Bolt + Canopy | opening | Y | Canopy OOD evaluations | 0.000194 |
| 20 | 2 | Bolt + Canopy | opening | Y | Canopy SEP batching covector | 0.000032 |
| 20 | 2 | Bolt + Canopy | opening | Y | Canopy ancestor claims | 0.000003 |
| 20 | 2 | Bolt + Canopy | opening | Y | collect and serialize row authentication paths | 0.000037 |
| 20 | 2 | Bolt + Canopy | opening | Y | copy and serialize queried row values | 0.000081 |
| 20 | 2 | Bolt + Canopy | opening | batched WHIR | distinct-functional cross evaluations | 0.037537 |
| 20 | 2 | Bolt + Canopy | opening | batched WHIR | shared recursive WHIR proof | 0.019656 |
| 20 | 2 | Bolt + Canopy | opening | common WHIR | WHIR configuration | 0.000006 |
| 20 | 2 | Bolt + Canopy | opening | other | unassigned phase work and timing overhead | 0.010675 |
| 20 | 2 | Bolt + Canopy | opening | source | authenticated WHIR alignment and constraint combination | 0.005589 |
| 20 | 2 | Bolt + Canopy | opening | u | common WHIR RS encode and oracle commitment | 0.007032 |
| 20 | 2 | Bolt + Canopy | opening | x | common WHIR RS encode and oracle commitment | 0.016790 |
| 20 | 2 | Bolt + Merkle | commitment | Bolt | parity RS encoding Y=RS(U) | 0.008557 |
| 20 | 2 | Bolt + Merkle | commitment | Bolt | sparse syndrome U=HX | 0.009641 |
| 20 | 2 | Bolt + Merkle | commitment | X | Merkle root transcript | 0.000000 |
| 20 | 2 | Bolt + Merkle | commitment | X | row Merkle tree or forest | 0.024192 |
| 20 | 2 | Bolt + Merkle | commitment | Y | Merkle root transcript | 0.000000 |
| 20 | 2 | Bolt + Merkle | commitment | Y | row Merkle tree or forest | 0.006509 |
| 20 | 2 | Bolt + Merkle | commitment | other | unassigned phase work and timing overhead | 0.000011 |
| 20 | 2 | Bolt + Merkle | opening | Bolt | column projections x=Xr and u=Ur | 0.003097 |
| 20 | 2 | Bolt + Merkle | opening | Bolt | parity query covector and transpose RS FFT | 0.002459 |
| 20 | 2 | Bolt + Merkle | opening | Bolt | partial evaluation g | 0.012827 |
| 20 | 2 | Bolt + Merkle | opening | Bolt | project queried rows and sample batching challenges | 0.000975 |
| 20 | 2 | Bolt + Merkle | opening | Bolt | public H transpose covector | 0.004213 |
| 20 | 2 | Bolt + Merkle | opening | Bolt | sample row queries | 0.006892 |
| 20 | 2 | Bolt + Merkle | opening | Bolt | syndrome evaluation sigma | 0.000593 |
| 20 | 2 | Bolt + Merkle | opening | Bolt | systematic query covector | 0.001261 |
| 20 | 2 | Bolt + Merkle | opening | X | collect and serialize row authentication paths | 0.008485 |
| 20 | 2 | Bolt + Merkle | opening | X | copy and serialize queried row values | 0.004994 |
| 20 | 2 | Bolt + Merkle | opening | Y | collect and serialize row authentication paths | 0.000252 |
| 20 | 2 | Bolt + Merkle | opening | Y | copy and serialize queried row values | 0.000081 |
| 20 | 2 | Bolt + Merkle | opening | batched WHIR | distinct-functional cross evaluations | 0.008232 |
| 20 | 2 | Bolt + Merkle | opening | batched WHIR | shared recursive WHIR proof | 0.009132 |
| 20 | 2 | Bolt + Merkle | opening | common WHIR | WHIR configuration | 0.000006 |
| 20 | 2 | Bolt + Merkle | opening | other | unassigned phase work and timing overhead | 0.006182 |
| 20 | 2 | Bolt + Merkle | opening | source | authenticated WHIR alignment and constraint combination | 0.008518 |
| 20 | 2 | Bolt + Merkle | opening | u | common WHIR RS encode and oracle commitment | 0.002653 |
| 20 | 2 | Bolt + Merkle | opening | x | common WHIR RS encode and oracle commitment | 0.017433 |
| 20 | 4 | Bolt + Canopy | commitment | Bolt | parity RS encoding Y=RS(U) | 0.008300 |
| 20 | 4 | Bolt + Canopy | commitment | Bolt | sparse syndrome U=HX | 0.009723 |
| 20 | 4 | Bolt + Canopy | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.006706 |
| 20 | 4 | Bolt + Canopy | commitment | X | Canopy WHIR configuration | 0.000004 |
| 20 | 4 | Bolt + Canopy | commitment | X | Canopy hash-to-field | 0.005229 |
| 20 | 4 | Bolt + Canopy | commitment | X | row Merkle tree or forest | 0.022412 |
| 20 | 4 | Bolt + Canopy | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.006560 |
| 20 | 4 | Bolt + Canopy | commitment | Y | Canopy WHIR configuration | 0.000003 |
| 20 | 4 | Bolt + Canopy | commitment | Y | Canopy hash-to-field | 0.005235 |
| 20 | 4 | Bolt + Canopy | commitment | Y | row Merkle tree or forest | 0.004917 |
| 20 | 4 | Bolt + Canopy | commitment | other | unassigned phase work and timing overhead | 0.000015 |
| 20 | 4 | Bolt + Canopy | opening | Bolt | column projections x=Xr and u=Ur | 0.002968 |
| 20 | 4 | Bolt + Canopy | opening | Bolt | parity query covector and transpose RS FFT | 0.002289 |
| 20 | 4 | Bolt + Canopy | opening | Bolt | partial evaluation g | 0.012786 |
| 20 | 4 | Bolt + Canopy | opening | Bolt | project queried rows and sample batching challenges | 0.000968 |
| 20 | 4 | Bolt + Canopy | opening | Bolt | public H transpose covector | 0.004128 |
| 20 | 4 | Bolt + Canopy | opening | Bolt | sample row queries | 0.006886 |
| 20 | 4 | Bolt + Canopy | opening | Bolt | syndrome evaluation sigma | 0.000548 |
| 20 | 4 | Bolt + Canopy | opening | Bolt | systematic query covector | 0.001245 |
| 20 | 4 | Bolt + Canopy | opening | X | Canopy OOD evaluations | 0.000218 |
| 20 | 4 | Bolt + Canopy | opening | X | Canopy SEP batching covector | 0.000350 |
| 20 | 4 | Bolt + Canopy | opening | X | Canopy ancestor claims | 0.000095 |
| 20 | 4 | Bolt + Canopy | opening | X | collect and serialize row authentication paths | 0.005846 |
| 20 | 4 | Bolt + Canopy | opening | X | copy and serialize queried row values | 0.004991 |
| 20 | 4 | Bolt + Canopy | opening | Y | Canopy OOD evaluations | 0.000186 |
| 20 | 4 | Bolt + Canopy | opening | Y | Canopy SEP batching covector | 0.000028 |
| 20 | 4 | Bolt + Canopy | opening | Y | Canopy ancestor claims | 0.000003 |
| 20 | 4 | Bolt + Canopy | opening | Y | collect and serialize row authentication paths | 0.000038 |
| 20 | 4 | Bolt + Canopy | opening | Y | copy and serialize queried row values | 0.000088 |
| 20 | 4 | Bolt + Canopy | opening | batched WHIR | distinct-functional cross evaluations | 0.161145 |
| 20 | 4 | Bolt + Canopy | opening | batched WHIR | shared recursive WHIR proof | 0.042796 |
| 20 | 4 | Bolt + Canopy | opening | common WHIR | WHIR configuration | 0.000005 |
| 20 | 4 | Bolt + Canopy | opening | other | unassigned phase work and timing overhead | 0.017087 |
| 20 | 4 | Bolt + Canopy | opening | source | authenticated WHIR alignment and constraint combination | 0.005501 |
| 20 | 4 | Bolt + Canopy | opening | u | common WHIR RS encode and oracle commitment | 0.007062 |
| 20 | 4 | Bolt + Canopy | opening | x | common WHIR RS encode and oracle commitment | 0.017012 |
| 20 | 4 | Bolt + Merkle | commitment | Bolt | parity RS encoding Y=RS(U) | 0.008472 |
| 20 | 4 | Bolt + Merkle | commitment | Bolt | sparse syndrome U=HX | 0.009819 |
| 20 | 4 | Bolt + Merkle | commitment | X | Merkle root transcript | 0.000000 |
| 20 | 4 | Bolt + Merkle | commitment | X | row Merkle tree or forest | 0.023597 |
| 20 | 4 | Bolt + Merkle | commitment | Y | Merkle root transcript | 0.000000 |
| 20 | 4 | Bolt + Merkle | commitment | Y | row Merkle tree or forest | 0.006416 |
| 20 | 4 | Bolt + Merkle | commitment | other | unassigned phase work and timing overhead | 0.000009 |
| 20 | 4 | Bolt + Merkle | opening | Bolt | column projections x=Xr and u=Ur | 0.003093 |
| 20 | 4 | Bolt + Merkle | opening | Bolt | parity query covector and transpose RS FFT | 0.002360 |
| 20 | 4 | Bolt + Merkle | opening | Bolt | partial evaluation g | 0.012947 |
| 20 | 4 | Bolt + Merkle | opening | Bolt | project queried rows and sample batching challenges | 0.000979 |
| 20 | 4 | Bolt + Merkle | opening | Bolt | public H transpose covector | 0.004432 |
| 20 | 4 | Bolt + Merkle | opening | Bolt | sample row queries | 0.006887 |
| 20 | 4 | Bolt + Merkle | opening | Bolt | syndrome evaluation sigma | 0.000567 |
| 20 | 4 | Bolt + Merkle | opening | Bolt | systematic query covector | 0.001265 |
| 20 | 4 | Bolt + Merkle | opening | X | collect and serialize row authentication paths | 0.007939 |
| 20 | 4 | Bolt + Merkle | opening | X | copy and serialize queried row values | 0.005054 |
| 20 | 4 | Bolt + Merkle | opening | Y | collect and serialize row authentication paths | 0.000269 |
| 20 | 4 | Bolt + Merkle | opening | Y | copy and serialize queried row values | 0.000083 |
| 20 | 4 | Bolt + Merkle | opening | batched WHIR | distinct-functional cross evaluations | 0.037627 |
| 20 | 4 | Bolt + Merkle | opening | batched WHIR | shared recursive WHIR proof | 0.016497 |
| 20 | 4 | Bolt + Merkle | opening | common WHIR | WHIR configuration | 0.000006 |
| 20 | 4 | Bolt + Merkle | opening | other | unassigned phase work and timing overhead | 0.013809 |
| 20 | 4 | Bolt + Merkle | opening | source | authenticated WHIR alignment and constraint combination | 0.008270 |
| 20 | 4 | Bolt + Merkle | opening | u | common WHIR RS encode and oracle commitment | 0.002580 |
| 20 | 4 | Bolt + Merkle | opening | x | common WHIR RS encode and oracle commitment | 0.017518 |
| 20 | 8 | Bolt + Canopy | commitment | Bolt | parity RS encoding Y=RS(U) | 0.008056 |
| 20 | 8 | Bolt + Canopy | commitment | Bolt | sparse syndrome U=HX | 0.009804 |
| 20 | 8 | Bolt + Canopy | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.006918 |
| 20 | 8 | Bolt + Canopy | commitment | X | Canopy WHIR configuration | 0.000004 |
| 20 | 8 | Bolt + Canopy | commitment | X | Canopy hash-to-field | 0.005235 |
| 20 | 8 | Bolt + Canopy | commitment | X | row Merkle tree or forest | 0.022165 |
| 20 | 8 | Bolt + Canopy | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.006643 |
| 20 | 8 | Bolt + Canopy | commitment | Y | Canopy WHIR configuration | 0.000003 |
| 20 | 8 | Bolt + Canopy | commitment | Y | Canopy hash-to-field | 0.005230 |
| 20 | 8 | Bolt + Canopy | commitment | Y | row Merkle tree or forest | 0.004971 |
| 20 | 8 | Bolt + Canopy | commitment | other | unassigned phase work and timing overhead | 0.000025 |
| 20 | 8 | Bolt + Canopy | opening | Bolt | column projections x=Xr and u=Ur | 0.003040 |
| 20 | 8 | Bolt + Canopy | opening | Bolt | parity query covector and transpose RS FFT | 0.002378 |
| 20 | 8 | Bolt + Canopy | opening | Bolt | partial evaluation g | 0.012832 |
| 20 | 8 | Bolt + Canopy | opening | Bolt | project queried rows and sample batching challenges | 0.000987 |
| 20 | 8 | Bolt + Canopy | opening | Bolt | public H transpose covector | 0.004339 |
| 20 | 8 | Bolt + Canopy | opening | Bolt | sample row queries | 0.006940 |
| 20 | 8 | Bolt + Canopy | opening | Bolt | syndrome evaluation sigma | 0.000549 |
| 20 | 8 | Bolt + Canopy | opening | Bolt | systematic query covector | 0.001328 |
| 20 | 8 | Bolt + Canopy | opening | X | Canopy OOD evaluations | 0.000258 |
| 20 | 8 | Bolt + Canopy | opening | X | Canopy SEP batching covector | 0.000352 |
| 20 | 8 | Bolt + Canopy | opening | X | Canopy ancestor claims | 0.000104 |
| 20 | 8 | Bolt + Canopy | opening | X | collect and serialize row authentication paths | 0.005905 |
| 20 | 8 | Bolt + Canopy | opening | X | copy and serialize queried row values | 0.005411 |
| 20 | 8 | Bolt + Canopy | opening | Y | Canopy OOD evaluations | 0.000195 |
| 20 | 8 | Bolt + Canopy | opening | Y | Canopy SEP batching covector | 0.000029 |
| 20 | 8 | Bolt + Canopy | opening | Y | Canopy ancestor claims | 0.000008 |
| 20 | 8 | Bolt + Canopy | opening | Y | collect and serialize row authentication paths | 0.000049 |
| 20 | 8 | Bolt + Canopy | opening | Y | copy and serialize queried row values | 0.000104 |
| 20 | 8 | Bolt + Canopy | opening | batched WHIR | distinct-functional cross evaluations | 0.720301 |
| 20 | 8 | Bolt + Canopy | opening | batched WHIR | shared recursive WHIR proof | 0.124640 |
| 20 | 8 | Bolt + Canopy | opening | common WHIR | WHIR configuration | 0.000005 |
| 20 | 8 | Bolt + Canopy | opening | other | unassigned phase work and timing overhead | 0.034948 |
| 20 | 8 | Bolt + Canopy | opening | source | authenticated WHIR alignment and constraint combination | 0.005606 |
| 20 | 8 | Bolt + Canopy | opening | u | common WHIR RS encode and oracle commitment | 0.007196 |
| 20 | 8 | Bolt + Canopy | opening | x | common WHIR RS encode and oracle commitment | 0.017274 |
| 20 | 8 | Bolt + Merkle | commitment | Bolt | parity RS encoding Y=RS(U) | 0.008418 |
| 20 | 8 | Bolt + Merkle | commitment | Bolt | sparse syndrome U=HX | 0.012151 |
| 20 | 8 | Bolt + Merkle | commitment | X | Merkle root transcript | 0.000000 |
| 20 | 8 | Bolt + Merkle | commitment | X | row Merkle tree or forest | 0.023747 |
| 20 | 8 | Bolt + Merkle | commitment | Y | Merkle root transcript | 0.000000 |
| 20 | 8 | Bolt + Merkle | commitment | Y | row Merkle tree or forest | 0.006489 |
| 20 | 8 | Bolt + Merkle | commitment | other | unassigned phase work and timing overhead | 0.000020 |
| 20 | 8 | Bolt + Merkle | opening | Bolt | column projections x=Xr and u=Ur | 0.003037 |
| 20 | 8 | Bolt + Merkle | opening | Bolt | parity query covector and transpose RS FFT | 0.002411 |
| 20 | 8 | Bolt + Merkle | opening | Bolt | partial evaluation g | 0.012887 |
| 20 | 8 | Bolt + Merkle | opening | Bolt | project queried rows and sample batching challenges | 0.000987 |
| 20 | 8 | Bolt + Merkle | opening | Bolt | public H transpose covector | 0.004276 |
| 20 | 8 | Bolt + Merkle | opening | Bolt | sample row queries | 0.006940 |
| 20 | 8 | Bolt + Merkle | opening | Bolt | syndrome evaluation sigma | 0.000560 |
| 20 | 8 | Bolt + Merkle | opening | Bolt | systematic query covector | 0.001304 |
| 20 | 8 | Bolt + Merkle | opening | X | collect and serialize row authentication paths | 0.008411 |
| 20 | 8 | Bolt + Merkle | opening | X | copy and serialize queried row values | 0.005191 |
| 20 | 8 | Bolt + Merkle | opening | Y | collect and serialize row authentication paths | 0.000259 |
| 20 | 8 | Bolt + Merkle | opening | Y | copy and serialize queried row values | 0.000093 |
| 20 | 8 | Bolt + Merkle | opening | batched WHIR | distinct-functional cross evaluations | 0.160590 |
| 20 | 8 | Bolt + Merkle | opening | batched WHIR | shared recursive WHIR proof | 0.040733 |
| 20 | 8 | Bolt + Merkle | opening | common WHIR | WHIR configuration | 0.000006 |
| 20 | 8 | Bolt + Merkle | opening | other | unassigned phase work and timing overhead | 0.023779 |
| 20 | 8 | Bolt + Merkle | opening | source | authenticated WHIR alignment and constraint combination | 0.008130 |
| 20 | 8 | Bolt + Merkle | opening | u | common WHIR RS encode and oracle commitment | 0.002426 |
| 20 | 8 | Bolt + Merkle | opening | x | common WHIR RS encode and oracle commitment | 0.016934 |
| 22 | 1 | Bolt + Canopy | commitment | Bolt | parity RS encoding Y=RS(U) | 0.033954 |
| 22 | 1 | Bolt + Canopy | commitment | Bolt | sparse syndrome U=HX | 0.054913 |
| 22 | 1 | Bolt + Canopy | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.027954 |
| 22 | 1 | Bolt + Canopy | commitment | X | Canopy WHIR configuration | 0.000005 |
| 22 | 1 | Bolt + Canopy | commitment | X | Canopy hash-to-field | 0.020963 |
| 22 | 1 | Bolt + Canopy | commitment | X | row Merkle tree or forest | 0.087288 |
| 22 | 1 | Bolt + Canopy | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.028780 |
| 22 | 1 | Bolt + Canopy | commitment | Y | Canopy WHIR configuration | 0.000005 |
| 22 | 1 | Bolt + Canopy | commitment | Y | Canopy hash-to-field | 0.020963 |
| 22 | 1 | Bolt + Canopy | commitment | Y | row Merkle tree or forest | 0.019240 |
| 22 | 1 | Bolt + Canopy | commitment | other | unassigned phase work and timing overhead | 0.000005 |
| 22 | 1 | Bolt + Canopy | opening | Bolt | column projections x=Xr and u=Ur | 0.011922 |
| 22 | 1 | Bolt + Canopy | opening | Bolt | parity query covector and transpose RS FFT | 0.009962 |
| 22 | 1 | Bolt + Canopy | opening | Bolt | partial evaluation g | 0.051307 |
| 22 | 1 | Bolt + Canopy | opening | Bolt | project queried rows and sample batching challenges | 0.000956 |
| 22 | 1 | Bolt + Canopy | opening | Bolt | public H transpose covector | 0.049571 |
| 22 | 1 | Bolt + Canopy | opening | Bolt | sample row queries | 0.006615 |
| 22 | 1 | Bolt + Canopy | opening | Bolt | syndrome evaluation sigma | 0.002074 |
| 22 | 1 | Bolt + Canopy | opening | Bolt | systematic query covector | 0.004414 |
| 22 | 1 | Bolt + Canopy | opening | X | Canopy OOD evaluations | 0.000483 |
| 22 | 1 | Bolt + Canopy | opening | X | Canopy SEP batching covector | 0.000562 |
| 22 | 1 | Bolt + Canopy | opening | X | Canopy ancestor claims | 0.000125 |
| 22 | 1 | Bolt + Canopy | opening | X | collect and serialize row authentication paths | 0.008181 |
| 22 | 1 | Bolt + Canopy | opening | X | copy and serialize queried row values | 0.005100 |
| 22 | 1 | Bolt + Canopy | opening | Y | Canopy OOD evaluations | 0.000468 |
| 22 | 1 | Bolt + Canopy | opening | Y | Canopy SEP batching covector | 0.000189 |
| 22 | 1 | Bolt + Canopy | opening | Y | Canopy ancestor claims | 0.000004 |
| 22 | 1 | Bolt + Canopy | opening | Y | collect and serialize row authentication paths | 0.000040 |
| 22 | 1 | Bolt + Canopy | opening | Y | copy and serialize queried row values | 0.000089 |
| 22 | 1 | Bolt + Canopy | opening | batched WHIR | distinct-functional cross evaluations | 0.033750 |
| 22 | 1 | Bolt + Canopy | opening | batched WHIR | shared recursive WHIR proof | 0.045410 |
| 22 | 1 | Bolt + Canopy | opening | common WHIR | WHIR configuration | 0.000007 |
| 22 | 1 | Bolt + Canopy | opening | other | unassigned phase work and timing overhead | 0.013208 |
| 22 | 1 | Bolt + Canopy | opening | source | authenticated WHIR alignment and constraint combination | 0.021733 |
| 22 | 1 | Bolt + Canopy | opening | u | common WHIR RS encode and oracle commitment | 0.029589 |
| 22 | 1 | Bolt + Canopy | opening | x | common WHIR RS encode and oracle commitment | 0.072442 |
| 22 | 1 | Bolt + Merkle | commitment | Bolt | parity RS encoding Y=RS(U) | 0.033787 |
| 22 | 1 | Bolt + Merkle | commitment | Bolt | sparse syndrome U=HX | 0.056441 |
| 22 | 1 | Bolt + Merkle | commitment | X | Merkle root transcript | 0.000000 |
| 22 | 1 | Bolt + Merkle | commitment | X | row Merkle tree or forest | 0.091928 |
| 22 | 1 | Bolt + Merkle | commitment | Y | Merkle root transcript | 0.000001 |
| 22 | 1 | Bolt + Merkle | commitment | Y | row Merkle tree or forest | 0.023909 |
| 22 | 1 | Bolt + Merkle | commitment | other | unassigned phase work and timing overhead | 0.000004 |
| 22 | 1 | Bolt + Merkle | opening | Bolt | column projections x=Xr and u=Ur | 0.011952 |
| 22 | 1 | Bolt + Merkle | opening | Bolt | parity query covector and transpose RS FFT | 0.009747 |
| 22 | 1 | Bolt + Merkle | opening | Bolt | partial evaluation g | 0.051006 |
| 22 | 1 | Bolt + Merkle | opening | Bolt | project queried rows and sample batching challenges | 0.000963 |
| 22 | 1 | Bolt + Merkle | opening | Bolt | public H transpose covector | 0.045183 |
| 22 | 1 | Bolt + Merkle | opening | Bolt | sample row queries | 0.006627 |
| 22 | 1 | Bolt + Merkle | opening | Bolt | syndrome evaluation sigma | 0.002061 |
| 22 | 1 | Bolt + Merkle | opening | Bolt | systematic query covector | 0.003417 |
| 22 | 1 | Bolt + Merkle | opening | X | collect and serialize row authentication paths | 0.012923 |
| 22 | 1 | Bolt + Merkle | opening | X | copy and serialize queried row values | 0.005070 |
| 22 | 1 | Bolt + Merkle | opening | Y | collect and serialize row authentication paths | 0.000312 |
| 22 | 1 | Bolt + Merkle | opening | Y | copy and serialize queried row values | 0.000087 |
| 22 | 1 | Bolt + Merkle | opening | batched WHIR | distinct-functional cross evaluations | 0.005740 |
| 22 | 1 | Bolt + Merkle | opening | batched WHIR | shared recursive WHIR proof | 0.017243 |
| 22 | 1 | Bolt + Merkle | opening | common WHIR | WHIR configuration | 0.000008 |
| 22 | 1 | Bolt + Merkle | opening | other | unassigned phase work and timing overhead | 0.007363 |
| 22 | 1 | Bolt + Merkle | opening | source | authenticated WHIR alignment and constraint combination | 0.030992 |
| 22 | 1 | Bolt + Merkle | opening | u | common WHIR RS encode and oracle commitment | 0.009193 |
| 22 | 1 | Bolt + Merkle | opening | x | common WHIR RS encode and oracle commitment | 0.072071 |
| 22 | 2 | Bolt + Canopy | commitment | Bolt | parity RS encoding Y=RS(U) | 0.036773 |
| 22 | 2 | Bolt + Canopy | commitment | Bolt | sparse syndrome U=HX | 0.055193 |
| 22 | 2 | Bolt + Canopy | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.028734 |
| 22 | 2 | Bolt + Canopy | commitment | X | Canopy WHIR configuration | 0.000005 |
| 22 | 2 | Bolt + Canopy | commitment | X | Canopy hash-to-field | 0.020922 |
| 22 | 2 | Bolt + Canopy | commitment | X | row Merkle tree or forest | 0.090432 |
| 22 | 2 | Bolt + Canopy | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.028721 |
| 22 | 2 | Bolt + Canopy | commitment | Y | Canopy WHIR configuration | 0.000004 |
| 22 | 2 | Bolt + Canopy | commitment | Y | Canopy hash-to-field | 0.021001 |
| 22 | 2 | Bolt + Canopy | commitment | Y | row Merkle tree or forest | 0.019331 |
| 22 | 2 | Bolt + Canopy | commitment | other | unassigned phase work and timing overhead | 0.000009 |
| 22 | 2 | Bolt + Canopy | opening | Bolt | column projections x=Xr and u=Ur | 0.011910 |
| 22 | 2 | Bolt + Canopy | opening | Bolt | parity query covector and transpose RS FFT | 0.010565 |
| 22 | 2 | Bolt + Canopy | opening | Bolt | partial evaluation g | 0.051735 |
| 22 | 2 | Bolt + Canopy | opening | Bolt | project queried rows and sample batching challenges | 0.001003 |
| 22 | 2 | Bolt + Canopy | opening | Bolt | public H transpose covector | 0.048626 |
| 22 | 2 | Bolt + Canopy | opening | Bolt | sample row queries | 0.006753 |
| 22 | 2 | Bolt + Canopy | opening | Bolt | syndrome evaluation sigma | 0.002038 |
| 22 | 2 | Bolt + Canopy | opening | Bolt | systematic query covector | 0.005097 |
| 22 | 2 | Bolt + Canopy | opening | X | Canopy OOD evaluations | 0.000494 |
| 22 | 2 | Bolt + Canopy | opening | X | Canopy SEP batching covector | 0.000603 |
| 22 | 2 | Bolt + Canopy | opening | X | Canopy ancestor claims | 0.000167 |
| 22 | 2 | Bolt + Canopy | opening | X | collect and serialize row authentication paths | 0.009375 |
| 22 | 2 | Bolt + Canopy | opening | X | copy and serialize queried row values | 0.005095 |
| 22 | 2 | Bolt + Canopy | opening | Y | Canopy OOD evaluations | 0.000497 |
| 22 | 2 | Bolt + Canopy | opening | Y | Canopy SEP batching covector | 0.000178 |
| 22 | 2 | Bolt + Canopy | opening | Y | Canopy ancestor claims | 0.000007 |
| 22 | 2 | Bolt + Canopy | opening | Y | collect and serialize row authentication paths | 0.000063 |
| 22 | 2 | Bolt + Canopy | opening | Y | copy and serialize queried row values | 0.000183 |
| 22 | 2 | Bolt + Canopy | opening | batched WHIR | distinct-functional cross evaluations | 0.151543 |
| 22 | 2 | Bolt + Canopy | opening | batched WHIR | shared recursive WHIR proof | 0.073142 |
| 22 | 2 | Bolt + Canopy | opening | common WHIR | WHIR configuration | 0.000007 |
| 22 | 2 | Bolt + Canopy | opening | other | unassigned phase work and timing overhead | 0.030104 |
| 22 | 2 | Bolt + Canopy | opening | source | authenticated WHIR alignment and constraint combination | 0.022271 |
| 22 | 2 | Bolt + Canopy | opening | u | common WHIR RS encode and oracle commitment | 0.031344 |
| 22 | 2 | Bolt + Canopy | opening | x | common WHIR RS encode and oracle commitment | 0.081672 |
| 22 | 2 | Bolt + Merkle | commitment | Bolt | parity RS encoding Y=RS(U) | 0.034405 |
| 22 | 2 | Bolt + Merkle | commitment | Bolt | sparse syndrome U=HX | 0.055670 |
| 22 | 2 | Bolt + Merkle | commitment | X | Merkle root transcript | 0.000000 |
| 22 | 2 | Bolt + Merkle | commitment | X | row Merkle tree or forest | 0.093298 |
| 22 | 2 | Bolt + Merkle | commitment | Y | Merkle root transcript | 0.000000 |
| 22 | 2 | Bolt + Merkle | commitment | Y | row Merkle tree or forest | 0.023894 |
| 22 | 2 | Bolt + Merkle | commitment | other | unassigned phase work and timing overhead | 0.000007 |
| 22 | 2 | Bolt + Merkle | opening | Bolt | column projections x=Xr and u=Ur | 0.011911 |
| 22 | 2 | Bolt + Merkle | opening | Bolt | parity query covector and transpose RS FFT | 0.010620 |
| 22 | 2 | Bolt + Merkle | opening | Bolt | partial evaluation g | 0.051997 |
| 22 | 2 | Bolt + Merkle | opening | Bolt | project queried rows and sample batching challenges | 0.000992 |
| 22 | 2 | Bolt + Merkle | opening | Bolt | public H transpose covector | 0.048190 |
| 22 | 2 | Bolt + Merkle | opening | Bolt | sample row queries | 0.006690 |
| 22 | 2 | Bolt + Merkle | opening | Bolt | syndrome evaluation sigma | 0.002046 |
| 22 | 2 | Bolt + Merkle | opening | Bolt | systematic query covector | 0.003610 |
| 22 | 2 | Bolt + Merkle | opening | X | collect and serialize row authentication paths | 0.012684 |
| 22 | 2 | Bolt + Merkle | opening | X | copy and serialize queried row values | 0.005603 |
| 22 | 2 | Bolt + Merkle | opening | Y | collect and serialize row authentication paths | 0.000454 |
| 22 | 2 | Bolt + Merkle | opening | Y | copy and serialize queried row values | 0.000107 |
| 22 | 2 | Bolt + Merkle | opening | batched WHIR | distinct-functional cross evaluations | 0.033967 |
| 22 | 2 | Bolt + Merkle | opening | batched WHIR | shared recursive WHIR proof | 0.027463 |
| 22 | 2 | Bolt + Merkle | opening | common WHIR | WHIR configuration | 0.000008 |
| 22 | 2 | Bolt + Merkle | opening | other | unassigned phase work and timing overhead | 0.016551 |
| 22 | 2 | Bolt + Merkle | opening | source | authenticated WHIR alignment and constraint combination | 0.031650 |
| 22 | 2 | Bolt + Merkle | opening | u | common WHIR RS encode and oracle commitment | 0.009271 |
| 22 | 2 | Bolt + Merkle | opening | x | common WHIR RS encode and oracle commitment | 0.073431 |
| 22 | 4 | Bolt + Canopy | commitment | Bolt | parity RS encoding Y=RS(U) | 0.034686 |
| 22 | 4 | Bolt + Canopy | commitment | Bolt | sparse syndrome U=HX | 0.059405 |
| 22 | 4 | Bolt + Canopy | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.029698 |
| 22 | 4 | Bolt + Canopy | commitment | X | Canopy WHIR configuration | 0.000005 |
| 22 | 4 | Bolt + Canopy | commitment | X | Canopy hash-to-field | 0.020995 |
| 22 | 4 | Bolt + Canopy | commitment | X | row Merkle tree or forest | 0.089015 |
| 22 | 4 | Bolt + Canopy | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.028860 |
| 22 | 4 | Bolt + Canopy | commitment | Y | Canopy WHIR configuration | 0.000005 |
| 22 | 4 | Bolt + Canopy | commitment | Y | Canopy hash-to-field | 0.020929 |
| 22 | 4 | Bolt + Canopy | commitment | Y | row Merkle tree or forest | 0.019437 |
| 22 | 4 | Bolt + Canopy | commitment | other | unassigned phase work and timing overhead | 0.000014 |
| 22 | 4 | Bolt + Canopy | opening | Bolt | column projections x=Xr and u=Ur | 0.012369 |
| 22 | 4 | Bolt + Canopy | opening | Bolt | parity query covector and transpose RS FFT | 0.010419 |
| 22 | 4 | Bolt + Canopy | opening | Bolt | partial evaluation g | 0.052300 |
| 22 | 4 | Bolt + Canopy | opening | Bolt | project queried rows and sample batching challenges | 0.001008 |
| 22 | 4 | Bolt + Canopy | opening | Bolt | public H transpose covector | 0.057631 |
| 22 | 4 | Bolt + Canopy | opening | Bolt | sample row queries | 0.006822 |
| 22 | 4 | Bolt + Canopy | opening | Bolt | syndrome evaluation sigma | 0.002053 |
| 22 | 4 | Bolt + Canopy | opening | Bolt | systematic query covector | 0.004914 |
| 22 | 4 | Bolt + Canopy | opening | X | Canopy OOD evaluations | 0.000518 |
| 22 | 4 | Bolt + Canopy | opening | X | Canopy SEP batching covector | 0.000558 |
| 22 | 4 | Bolt + Canopy | opening | X | Canopy ancestor claims | 0.000204 |
| 22 | 4 | Bolt + Canopy | opening | X | collect and serialize row authentication paths | 0.010514 |
| 22 | 4 | Bolt + Canopy | opening | X | copy and serialize queried row values | 0.005468 |
| 22 | 4 | Bolt + Canopy | opening | Y | Canopy OOD evaluations | 0.000506 |
| 22 | 4 | Bolt + Canopy | opening | Y | Canopy SEP batching covector | 0.000170 |
| 22 | 4 | Bolt + Canopy | opening | Y | Canopy ancestor claims | 0.000043 |
| 22 | 4 | Bolt + Canopy | opening | Y | collect and serialize row authentication paths | 0.000149 |
| 22 | 4 | Bolt + Canopy | opening | Y | copy and serialize queried row values | 0.000278 |
| 22 | 4 | Bolt + Canopy | opening | batched WHIR | distinct-functional cross evaluations | 0.656111 |
| 22 | 4 | Bolt + Canopy | opening | batched WHIR | shared recursive WHIR proof | 0.167553 |
| 22 | 4 | Bolt + Canopy | opening | common WHIR | WHIR configuration | 0.000008 |
| 22 | 4 | Bolt + Canopy | opening | other | unassigned phase work and timing overhead | 0.065563 |
| 22 | 4 | Bolt + Canopy | opening | source | authenticated WHIR alignment and constraint combination | 0.024181 |
| 22 | 4 | Bolt + Canopy | opening | u | common WHIR RS encode and oracle commitment | 0.030983 |
| 22 | 4 | Bolt + Canopy | opening | x | common WHIR RS encode and oracle commitment | 0.090813 |
| 22 | 4 | Bolt + Merkle | commitment | Bolt | parity RS encoding Y=RS(U) | 0.035330 |
| 22 | 4 | Bolt + Merkle | commitment | Bolt | sparse syndrome U=HX | 0.058456 |
| 22 | 4 | Bolt + Merkle | commitment | X | Merkle root transcript | 0.000001 |
| 22 | 4 | Bolt + Merkle | commitment | X | row Merkle tree or forest | 0.094008 |
| 22 | 4 | Bolt + Merkle | commitment | Y | Merkle root transcript | 0.000000 |
| 22 | 4 | Bolt + Merkle | commitment | Y | row Merkle tree or forest | 0.024028 |
| 22 | 4 | Bolt + Merkle | commitment | other | unassigned phase work and timing overhead | 0.000012 |
| 22 | 4 | Bolt + Merkle | opening | Bolt | column projections x=Xr and u=Ur | 0.012069 |
| 22 | 4 | Bolt + Merkle | opening | Bolt | parity query covector and transpose RS FFT | 0.010052 |
| 22 | 4 | Bolt + Merkle | opening | Bolt | partial evaluation g | 0.051624 |
| 22 | 4 | Bolt + Merkle | opening | Bolt | project queried rows and sample batching challenges | 0.000979 |
| 22 | 4 | Bolt + Merkle | opening | Bolt | public H transpose covector | 0.049697 |
| 22 | 4 | Bolt + Merkle | opening | Bolt | sample row queries | 0.006725 |
| 22 | 4 | Bolt + Merkle | opening | Bolt | syndrome evaluation sigma | 0.002075 |
| 22 | 4 | Bolt + Merkle | opening | Bolt | systematic query covector | 0.003332 |
| 22 | 4 | Bolt + Merkle | opening | X | collect and serialize row authentication paths | 0.012689 |
| 22 | 4 | Bolt + Merkle | opening | X | copy and serialize queried row values | 0.005313 |
| 22 | 4 | Bolt + Merkle | opening | Y | collect and serialize row authentication paths | 0.000373 |
| 22 | 4 | Bolt + Merkle | opening | Y | copy and serialize queried row values | 0.000109 |
| 22 | 4 | Bolt + Merkle | opening | batched WHIR | distinct-functional cross evaluations | 0.153218 |
| 22 | 4 | Bolt + Merkle | opening | batched WHIR | shared recursive WHIR proof | 0.056740 |
| 22 | 4 | Bolt + Merkle | opening | common WHIR | WHIR configuration | 0.000008 |
| 22 | 4 | Bolt + Merkle | opening | other | unassigned phase work and timing overhead | 0.033939 |
| 22 | 4 | Bolt + Merkle | opening | source | authenticated WHIR alignment and constraint combination | 0.031961 |
| 22 | 4 | Bolt + Merkle | opening | u | common WHIR RS encode and oracle commitment | 0.009385 |
| 22 | 4 | Bolt + Merkle | opening | x | common WHIR RS encode and oracle commitment | 0.078390 |
| 22 | 8 | Bolt + Canopy | commitment | Bolt | parity RS encoding Y=RS(U) | 0.037022 |
| 22 | 8 | Bolt + Canopy | commitment | Bolt | sparse syndrome U=HX | 0.083206 |
| 22 | 8 | Bolt + Canopy | commitment | X | Canopy RS encode and WHIR oracle commitment | 0.031635 |
| 22 | 8 | Bolt + Canopy | commitment | X | Canopy WHIR configuration | 0.000006 |
| 22 | 8 | Bolt + Canopy | commitment | X | Canopy hash-to-field | 0.021077 |
| 22 | 8 | Bolt + Canopy | commitment | X | row Merkle tree or forest | 0.093755 |
| 22 | 8 | Bolt + Canopy | commitment | Y | Canopy RS encode and WHIR oracle commitment | 0.032167 |
| 22 | 8 | Bolt + Canopy | commitment | Y | Canopy WHIR configuration | 0.000005 |
| 22 | 8 | Bolt + Canopy | commitment | Y | Canopy hash-to-field | 0.021141 |
| 22 | 8 | Bolt + Canopy | commitment | Y | row Merkle tree or forest | 0.020862 |
| 22 | 8 | Bolt + Canopy | commitment | other | unassigned phase work and timing overhead | 0.000027 |
| 22 | 8 | Bolt + Canopy | opening | Bolt | column projections x=Xr and u=Ur | 0.016189 |
| 22 | 8 | Bolt + Canopy | opening | Bolt | parity query covector and transpose RS FFT | 0.011673 |
| 22 | 8 | Bolt + Canopy | opening | Bolt | partial evaluation g | 0.074285 |
| 22 | 8 | Bolt + Canopy | opening | Bolt | project queried rows and sample batching challenges | 0.001021 |
| 22 | 8 | Bolt + Canopy | opening | Bolt | public H transpose covector | 0.064300 |
| 22 | 8 | Bolt + Canopy | opening | Bolt | sample row queries | 0.007168 |
| 22 | 8 | Bolt + Canopy | opening | Bolt | syndrome evaluation sigma | 0.002124 |
| 22 | 8 | Bolt + Canopy | opening | Bolt | systematic query covector | 0.006016 |
| 22 | 8 | Bolt + Canopy | opening | X | Canopy OOD evaluations | 0.000586 |
| 22 | 8 | Bolt + Canopy | opening | X | Canopy SEP batching covector | 0.000569 |
| 22 | 8 | Bolt + Canopy | opening | X | Canopy ancestor claims | 0.000525 |
| 22 | 8 | Bolt + Canopy | opening | X | collect and serialize row authentication paths | 0.014473 |
| 22 | 8 | Bolt + Canopy | opening | X | copy and serialize queried row values | 0.007587 |
| 22 | 8 | Bolt + Canopy | opening | Y | Canopy OOD evaluations | 0.000645 |
| 22 | 8 | Bolt + Canopy | opening | Y | Canopy SEP batching covector | 0.000177 |
| 22 | 8 | Bolt + Canopy | opening | Y | Canopy ancestor claims | 0.000336 |
| 22 | 8 | Bolt + Canopy | opening | Y | collect and serialize row authentication paths | 0.000356 |
| 22 | 8 | Bolt + Canopy | opening | Y | copy and serialize queried row values | 0.000441 |
| 22 | 8 | Bolt + Canopy | opening | batched WHIR | distinct-functional cross evaluations | 2.771203 |
| 22 | 8 | Bolt + Canopy | opening | batched WHIR | shared recursive WHIR proof | 0.523807 |
| 22 | 8 | Bolt + Canopy | opening | common WHIR | WHIR configuration | 0.000011 |
| 22 | 8 | Bolt + Canopy | opening | other | unassigned phase work and timing overhead | 0.160874 |
| 22 | 8 | Bolt + Canopy | opening | source | authenticated WHIR alignment and constraint combination | 0.029891 |
| 22 | 8 | Bolt + Canopy | opening | u | common WHIR RS encode and oracle commitment | 0.037148 |
| 22 | 8 | Bolt + Canopy | opening | x | common WHIR RS encode and oracle commitment | 0.108569 |
| 22 | 8 | Bolt + Merkle | commitment | Bolt | parity RS encoding Y=RS(U) | 0.036302 |
| 22 | 8 | Bolt + Merkle | commitment | Bolt | sparse syndrome U=HX | 0.066107 |
| 22 | 8 | Bolt + Merkle | commitment | X | Merkle root transcript | 0.000000 |
| 22 | 8 | Bolt + Merkle | commitment | X | row Merkle tree or forest | 0.095928 |
| 22 | 8 | Bolt + Merkle | commitment | Y | Merkle root transcript | 0.000000 |
| 22 | 8 | Bolt + Merkle | commitment | Y | row Merkle tree or forest | 0.024691 |
| 22 | 8 | Bolt + Merkle | commitment | other | unassigned phase work and timing overhead | 0.000020 |
| 22 | 8 | Bolt + Merkle | opening | Bolt | column projections x=Xr and u=Ur | 0.015247 |
| 22 | 8 | Bolt + Merkle | opening | Bolt | parity query covector and transpose RS FFT | 0.014919 |
| 22 | 8 | Bolt + Merkle | opening | Bolt | partial evaluation g | 0.061738 |
| 22 | 8 | Bolt + Merkle | opening | Bolt | project queried rows and sample batching challenges | 0.001326 |
| 22 | 8 | Bolt + Merkle | opening | Bolt | public H transpose covector | 0.057884 |
| 22 | 8 | Bolt + Merkle | opening | Bolt | sample row queries | 0.007386 |
| 22 | 8 | Bolt + Merkle | opening | Bolt | syndrome evaluation sigma | 0.002397 |
| 22 | 8 | Bolt + Merkle | opening | Bolt | systematic query covector | 0.005337 |
| 22 | 8 | Bolt + Merkle | opening | X | collect and serialize row authentication paths | 0.019788 |
| 22 | 8 | Bolt + Merkle | opening | X | copy and serialize queried row values | 0.006066 |
| 22 | 8 | Bolt + Merkle | opening | Y | collect and serialize row authentication paths | 0.000960 |
| 22 | 8 | Bolt + Merkle | opening | Y | copy and serialize queried row values | 0.000495 |
| 22 | 8 | Bolt + Merkle | opening | batched WHIR | distinct-functional cross evaluations | 0.663605 |
| 22 | 8 | Bolt + Merkle | opening | batched WHIR | shared recursive WHIR proof | 0.157357 |
| 22 | 8 | Bolt + Merkle | opening | common WHIR | WHIR configuration | 0.000009 |
| 22 | 8 | Bolt + Merkle | opening | other | unassigned phase work and timing overhead | 0.071269 |
| 22 | 8 | Bolt + Merkle | opening | source | authenticated WHIR alignment and constraint combination | 0.036222 |
| 22 | 8 | Bolt + Merkle | opening | u | common WHIR RS encode and oracle commitment | 0.011477 |
| 22 | 8 | Bolt + Merkle | opening | x | common WHIR RS encode and oracle commitment | 0.103798 |
