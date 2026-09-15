# Final Johnson-bound paper prover timings

Each input oracle has `2^18` field elements. The rate-1/2 rows use a first codeword of `2^19` elements, while the added rate-1/4 batched baseline uses `2^20` elements. All WHIR components are generated and verifier-checked at 128-bit security over the 256-bit FFT field, in johnson mode, with no proof of work. Input generation, verification, and deallocation are excluded. Commitment time contains initial RS encoding and the initial Merkle/Canopy commitment; opening time contains OOD work, openings, recursive oracle commitments, Schwartz--Zippel preprocessing, and recursive WHIR. The selected initial fold is three: Experiment 3 therefore uses aligned Canopy height four, while Experiment 2 uses its independently optimized height `h=18` and has a terminal Canopy claim. To match the paper's at-most-`2^10` stopping rule, the reference implementation's native smaller suffix is separately executed and subtracted. Independent per-input stages are measured once per repetition and scaled by `t`; every batched recursive proof and generalized distinct-functional cross-evaluation stage is executed at the reported `t`.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs independent | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.0279 | 0.0445 | 0.0725 | 0.0725 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0279 | 0.0445 | 0.0725 | 0.0725 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.0574 | 0.0712 | 0.1286 | 0.1290 | 1.775x | 1.775x |
| 2 | 1 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.0350 | 0.0447 | 0.0796 | 0.0797 | 1.099x | 1.099x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.0582 | 0.0496 | 0.1078 | 0.1084 | 1.487x | 1.487x |
| 0 | 2 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.0559 | 0.0891 | 0.1449 | 0.1449 | 1.000x | 1.086x |
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0559 | 0.0776 | 0.1335 | 0.1336 | 0.921x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.1148 | 0.1216 | 0.2364 | 0.2375 | 1.631x | 1.772x |
| 2 | 2 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.0699 | 0.0893 | 0.1593 | 0.1594 | 1.099x | 1.194x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.1163 | 0.0867 | 0.2031 | 0.2033 | 1.401x | 1.522x |
| 0 | 4 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.1117 | 0.1781 | 0.2898 | 0.2899 | 1.000x | 1.101x |
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.1117 | 0.1515 | 0.2632 | 0.2639 | 0.908x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.2296 | 0.2298 | 0.4594 | 0.4602 | 1.585x | 1.745x |
| 2 | 4 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.1399 | 0.1787 | 0.3186 | 0.3188 | 1.099x | 1.210x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.2327 | 0.1958 | 0.4284 | 0.4279 | 1.478x | 1.628x |
| 0 | 8 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.2235 | 0.3562 | 0.5797 | 0.5798 | 1.000x | 1.042x |
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.2235 | 0.3327 | 0.5561 | 0.5563 | 0.959x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.4591 | 0.4761 | 0.9352 | 0.9392 | 1.613x | 1.682x |
| 2 | 8 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.2798 | 0.3574 | 0.6372 | 0.6377 | 1.099x | 1.146x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.4654 | 0.5563 | 1.0217 | 1.0187 | 1.762x | 1.837x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | initial_merkle_tree | 0.0033 | 0.0032 | 4.6% |
| 0 | 1 | merkle_whir_independent | initial_ood_evaluations | 0.0012 | 0.0012 | 1.7% |
| 0 | 1 | merkle_whir_independent | initial_rs_encoding | 0.0246 | 0.0250 | 34.0% |
| 0 | 1 | merkle_whir_independent | whir_opening_proof_after_ood | 0.0433 | 0.0432 | 59.7% |
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0156 | 0.0153 | 21.5% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0033 | 0.0032 | 4.6% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0012 | 0.0012 | 1.7% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0246 | 0.0250 | 34.0% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.0152 | 0.0155 | 21.0% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0003 | 0.0003 | 0.4% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0122 | 0.0122 | 16.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0216 | 0.0218 | 16.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0058 | 0.0058 | 4.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0013 | 0.0013 | 1.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.0516 | 0.0520 | 40.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.0253 | 0.0253 | 19.7% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0003 | 0.0003 | 0.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0228 | 0.0226 | 17.7% |
| 2 | 1 | canopy_unbatched | canopy_bottom_forest | 0.0031 | 0.0031 | 3.8% |
| 2 | 1 | canopy_unbatched | canopy_bottom_opening | 0.0001 | 0.0001 | 0.2% |
| 2 | 1 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0032 | 0.0032 | 4.0% |
| 2 | 1 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_hash_to_field | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_rs_encoding | 0.0041 | 0.0041 | 5.1% |
| 2 | 1 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0179 | 0.0180 | 22.4% |
| 2 | 1 | canopy_unbatched | outer_ood_evaluations | 0.0012 | 0.0012 | 1.6% |
| 2 | 1 | canopy_unbatched | outer_rs_encoding | 0.0246 | 0.0250 | 30.9% |
| 2 | 1 | canopy_unbatched | outer_second_oracle_whirs | 0.0254 | 0.0255 | 31.9% |
| 2 | 1 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.0281 | 0.0278 | 26.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0026 | 0.0025 | 2.4% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0032 | 0.0032 | 2.9% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0078 | 0.0077 | 7.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0200 | 0.0202 | 18.6% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0024 | 0.0024 | 2.2% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0179 | 0.0180 | 16.6% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0012 | 0.0012 | 1.2% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.0246 | 0.0250 | 22.9% |
| 0 | 2 | merkle_whir_independent | initial_merkle_tree | 0.0066 | 0.0064 | 4.6% |
| 0 | 2 | merkle_whir_independent | initial_ood_evaluations | 0.0025 | 0.0024 | 1.7% |
| 0 | 2 | merkle_whir_independent | initial_rs_encoding | 0.0493 | 0.0500 | 34.0% |
| 0 | 2 | merkle_whir_independent | whir_opening_proof_after_ood | 0.0866 | 0.0864 | 59.7% |
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0174 | 0.0174 | 13.0% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0311 | 0.0307 | 23.3% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0022 | 0.0021 | 1.6% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0066 | 0.0064 | 5.0% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0025 | 0.0024 | 1.9% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0493 | 0.0500 | 36.9% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0244 | 0.0244 | 18.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0282 | 0.0282 | 11.9% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0432 | 0.0435 | 18.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0022 | 0.0021 | 0.9% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0117 | 0.0117 | 4.9% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0026 | 0.0026 | 1.1% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.1031 | 0.1039 | 43.6% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0455 | 0.0451 | 19.3% |
| 2 | 2 | canopy_unbatched | canopy_bottom_forest | 0.0061 | 0.0062 | 3.8% |
| 2 | 2 | canopy_unbatched | canopy_bottom_opening | 0.0003 | 0.0003 | 0.2% |
| 2 | 2 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0064 | 0.0064 | 4.0% |
| 2 | 2 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_hash_to_field | 0.0000 | 0.0000 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_rs_encoding | 0.0081 | 0.0081 | 5.1% |
| 2 | 2 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0357 | 0.0360 | 22.4% |
| 2 | 2 | canopy_unbatched | outer_ood_evaluations | 0.0025 | 0.0024 | 1.6% |
| 2 | 2 | canopy_unbatched | outer_rs_encoding | 0.0493 | 0.0500 | 30.9% |
| 2 | 2 | canopy_unbatched | outer_second_oracle_whirs | 0.0509 | 0.0510 | 31.9% |
| 2 | 2 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.0340 | 0.0338 | 16.7% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0051 | 0.0051 | 2.5% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0063 | 0.0064 | 3.1% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0156 | 0.0153 | 7.7% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0400 | 0.0403 | 19.7% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0144 | 0.0144 | 7.1% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0357 | 0.0360 | 17.6% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0025 | 0.0024 | 1.2% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.0493 | 0.0500 | 24.3% |
| 0 | 4 | merkle_whir_independent | initial_merkle_tree | 0.0132 | 0.0127 | 4.6% |
| 0 | 4 | merkle_whir_independent | initial_ood_evaluations | 0.0050 | 0.0049 | 1.7% |
| 0 | 4 | merkle_whir_independent | initial_rs_encoding | 0.0985 | 0.1000 | 34.0% |
| 0 | 4 | merkle_whir_independent | whir_opening_proof_after_ood | 0.1732 | 0.1728 | 59.7% |
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0224 | 0.0224 | 8.5% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0623 | 0.0614 | 23.7% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0130 | 0.0129 | 4.9% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0132 | 0.0127 | 5.0% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0050 | 0.0049 | 1.9% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0985 | 0.1000 | 37.4% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0488 | 0.0489 | 18.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0342 | 0.0335 | 7.4% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0864 | 0.0870 | 18.8% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0130 | 0.0129 | 2.8% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0233 | 0.0233 | 5.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0051 | 0.0052 | 1.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.2063 | 0.2078 | 44.9% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0911 | 0.0903 | 19.8% |
| 2 | 4 | canopy_unbatched | canopy_bottom_forest | 0.0123 | 0.0124 | 3.8% |
| 2 | 4 | canopy_unbatched | canopy_bottom_opening | 0.0006 | 0.0006 | 0.2% |
| 2 | 4 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0128 | 0.0127 | 4.0% |
| 2 | 4 | canopy_unbatched | canopy_commitment_other | 0.0001 | 0.0001 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_hash_to_field | 0.0001 | 0.0001 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_rs_encoding | 0.0162 | 0.0163 | 5.1% |
| 2 | 4 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0714 | 0.0719 | 22.4% |
| 2 | 4 | canopy_unbatched | outer_ood_evaluations | 0.0050 | 0.0049 | 1.6% |
| 2 | 4 | canopy_unbatched | outer_rs_encoding | 0.0985 | 0.1000 | 30.9% |
| 2 | 4 | canopy_unbatched | outer_second_oracle_whirs | 0.1017 | 0.1020 | 31.9% |
| 2 | 4 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.0518 | 0.0518 | 12.1% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0102 | 0.0101 | 2.4% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0127 | 0.0128 | 3.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0001 | 0.0001 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0311 | 0.0306 | 7.3% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0801 | 0.0807 | 18.7% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0674 | 0.0674 | 15.7% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0714 | 0.0719 | 16.7% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0050 | 0.0049 | 1.2% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 0.0985 | 0.1000 | 23.0% |
| 0 | 8 | merkle_whir_independent | initial_merkle_tree | 0.0264 | 0.0254 | 4.6% |
| 0 | 8 | merkle_whir_independent | initial_ood_evaluations | 0.0099 | 0.0098 | 1.7% |
| 0 | 8 | merkle_whir_independent | initial_rs_encoding | 0.1970 | 0.1999 | 34.0% |
| 0 | 8 | merkle_whir_independent | whir_opening_proof_after_ood | 0.3463 | 0.3456 | 59.7% |
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0400 | 0.0399 | 7.2% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.1246 | 0.1228 | 22.4% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0606 | 0.0604 | 10.9% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0264 | 0.0254 | 4.8% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0099 | 0.0098 | 1.8% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 0.1970 | 0.1999 | 35.4% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0976 | 0.0978 | 17.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0502 | 0.0502 | 5.4% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.1729 | 0.1740 | 18.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0606 | 0.0604 | 6.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0466 | 0.0466 | 5.0% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0102 | 0.0103 | 1.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.4125 | 0.4157 | 44.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.1822 | 0.1806 | 19.5% |
| 2 | 8 | canopy_unbatched | canopy_bottom_forest | 0.0245 | 0.0247 | 3.8% |
| 2 | 8 | canopy_unbatched | canopy_bottom_opening | 0.0011 | 0.0011 | 0.2% |
| 2 | 8 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0255 | 0.0254 | 4.0% |
| 2 | 8 | canopy_unbatched | canopy_commitment_other | 0.0001 | 0.0001 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_hash_to_field | 0.0002 | 0.0002 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_rs_encoding | 0.0324 | 0.0325 | 5.1% |
| 2 | 8 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.1429 | 0.1439 | 22.4% |
| 2 | 8 | canopy_unbatched | outer_ood_evaluations | 0.0099 | 0.0098 | 1.6% |
| 2 | 8 | canopy_unbatched | outer_rs_encoding | 0.1970 | 0.1999 | 30.9% |
| 2 | 8 | canopy_unbatched | outer_second_oracle_whirs | 0.2034 | 0.2040 | 31.9% |
| 2 | 8 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 0.1140 | 0.1138 | 11.2% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0204 | 0.0203 | 2.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0003 | 0.0003 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0254 | 0.0255 | 2.5% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0001 | 0.0001 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0622 | 0.0613 | 6.1% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1602 | 0.1614 | 15.7% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.2892 | 0.2890 | 28.3% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.1429 | 0.1439 | 14.0% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0099 | 0.0098 | 1.0% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 0.1970 | 0.1999 | 19.3% |
