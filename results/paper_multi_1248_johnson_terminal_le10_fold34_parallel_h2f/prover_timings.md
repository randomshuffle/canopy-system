# Final Johnson-bound paper prover timings

Each input oracle has `2^18` field elements. The rate-1/2 rows use a first codeword of `2^19` elements, while the added rate-1/4 batched baseline uses `2^20` elements. All WHIR components are generated and verifier-checked at 128-bit security over the 256-bit FFT field, in johnson mode, with no proof of work. Input generation, verification, and deallocation are excluded. Commitment time contains initial RS encoding and the initial Merkle/Canopy commitment; opening time contains OOD work, openings, recursive oracle commitments, Schwartz--Zippel preprocessing, and recursive WHIR. The selected initial fold is three: Experiment 3 therefore uses aligned Canopy height four, while Experiment 2 uses its independently optimized height `h=18` and has a terminal Canopy claim. To match the paper's at-most-`2^10` stopping rule, the reference implementation's native smaller suffix is separately executed and subtracted. Baseline and Experiment-2 independent stages are measured once per repetition and scaled by `t`. Experiment 3 executes its outer RS encodings, Canopy forests, H2F conversions, Canopy RS encodings, Merkle commitments, and bottom openings concurrently at the reported `t`; its transcript-dependent outer first transitions remain scaled independent work. Every batched recursive proof and generalized distinct-functional cross-evaluation stage is executed at the reported `t`.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs independent | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.0294 | 0.0460 | 0.0754 | 0.0741 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0294 | 0.0460 | 0.0754 | 0.0741 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.0590 | 0.0741 | 0.1331 | 0.1332 | 1.765x | 1.765x |
| 2 | 1 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.0375 | 0.0461 | 0.0836 | 0.0824 | 1.109x | 1.109x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.0545 | 0.0518 | 0.1063 | 0.1048 | 1.410x | 1.410x |
| 0 | 2 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.0589 | 0.0919 | 0.1508 | 0.1483 | 1.000x | 1.091x |
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0589 | 0.0794 | 0.1383 | 0.1366 | 0.917x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.1179 | 0.1255 | 0.2434 | 0.2435 | 1.614x | 1.760x |
| 2 | 2 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.0750 | 0.0922 | 0.1672 | 0.1647 | 1.109x | 1.209x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.1025 | 0.0764 | 0.1789 | 0.1781 | 1.186x | 1.294x |
| 0 | 4 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.1178 | 0.1838 | 0.3016 | 0.2965 | 1.000x | 1.147x |
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.1178 | 0.1452 | 0.2630 | 0.2590 | 0.872x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.2358 | 0.2258 | 0.4616 | 0.4611 | 1.530x | 1.755x |
| 2 | 4 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.1500 | 0.1845 | 0.3345 | 0.3295 | 1.109x | 1.272x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.1979 | 0.1378 | 0.3357 | 0.3358 | 1.113x | 1.276x |
| 0 | 8 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.2356 | 0.3677 | 0.6032 | 0.5931 | 1.000x | 1.159x |
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.2356 | 0.2848 | 0.5204 | 0.5131 | 0.863x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.4716 | 0.4348 | 0.9064 | 0.9064 | 1.503x | 1.742x |
| 2 | 8 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.3000 | 0.3689 | 0.6689 | 0.6589 | 1.109x | 1.285x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.3860 | 0.2950 | 0.6809 | 0.6750 | 1.129x | 1.309x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | initial_merkle_tree | 0.0032 | 0.0032 | 4.3% |
| 0 | 1 | merkle_whir_independent | initial_ood_evaluations | 0.0013 | 0.0013 | 1.8% |
| 0 | 1 | merkle_whir_independent | initial_rs_encoding | 0.0262 | 0.0262 | 34.8% |
| 0 | 1 | merkle_whir_independent | whir_opening_proof_after_ood | 0.0446 | 0.0436 | 59.2% |
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0160 | 0.0153 | 21.2% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0032 | 0.0032 | 4.3% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0013 | 0.0013 | 1.8% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0262 | 0.0262 | 34.8% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.0161 | 0.0161 | 21.4% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0003 | 0.0003 | 0.4% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0123 | 0.0123 | 16.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0226 | 0.0224 | 16.9% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0064 | 0.0061 | 4.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0013 | 0.0013 | 1.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.0526 | 0.0523 | 39.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.0269 | 0.0271 | 20.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0002 | 0.0002 | 0.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0231 | 0.0230 | 17.3% |
| 2 | 1 | canopy_unbatched | canopy_bottom_forest | 0.0030 | 0.0030 | 3.6% |
| 2 | 1 | canopy_unbatched | canopy_bottom_opening | 0.0002 | 0.0002 | 0.2% |
| 2 | 1 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0035 | 0.0034 | 4.2% |
| 2 | 1 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_hash_to_field | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_rs_encoding | 0.0047 | 0.0047 | 5.6% |
| 2 | 1 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0177 | 0.0171 | 21.2% |
| 2 | 1 | canopy_unbatched | outer_ood_evaluations | 0.0013 | 0.0013 | 1.6% |
| 2 | 1 | canopy_unbatched | outer_rs_encoding | 0.0262 | 0.0262 | 31.4% |
| 2 | 1 | canopy_unbatched | outer_second_oracle_whirs | 0.0269 | 0.0278 | 32.2% |
| 2 | 1 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.0296 | 0.0301 | 27.9% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0027 | 0.0027 | 2.5% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0033 | 0.0033 | 3.1% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0012 | 0.0012 | 1.1% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0214 | 0.0213 | 20.1% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0031 | 0.0029 | 2.9% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0177 | 0.0171 | 16.7% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0013 | 0.0013 | 1.2% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.0260 | 0.0261 | 24.5% |
| 0 | 2 | merkle_whir_independent | initial_merkle_tree | 0.0064 | 0.0063 | 4.3% |
| 0 | 2 | merkle_whir_independent | initial_ood_evaluations | 0.0026 | 0.0025 | 1.8% |
| 0 | 2 | merkle_whir_independent | initial_rs_encoding | 0.0525 | 0.0524 | 34.8% |
| 0 | 2 | merkle_whir_independent | whir_opening_proof_after_ood | 0.0893 | 0.0873 | 59.2% |
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0176 | 0.0175 | 12.7% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0319 | 0.0306 | 23.1% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0026 | 0.0026 | 1.9% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0064 | 0.0063 | 4.6% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0026 | 0.0025 | 1.9% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0525 | 0.0524 | 37.9% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0246 | 0.0245 | 17.8% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0290 | 0.0292 | 11.9% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0451 | 0.0447 | 18.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0026 | 0.0026 | 1.1% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0127 | 0.0121 | 5.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0027 | 0.0027 | 1.1% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.1052 | 0.1046 | 43.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0462 | 0.0460 | 19.0% |
| 2 | 2 | canopy_unbatched | canopy_bottom_forest | 0.0061 | 0.0061 | 3.6% |
| 2 | 2 | canopy_unbatched | canopy_bottom_opening | 0.0003 | 0.0003 | 0.2% |
| 2 | 2 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0070 | 0.0068 | 4.2% |
| 2 | 2 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_hash_to_field | 0.0000 | 0.0000 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_rs_encoding | 0.0094 | 0.0094 | 5.6% |
| 2 | 2 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0354 | 0.0342 | 21.2% |
| 2 | 2 | canopy_unbatched | outer_ood_evaluations | 0.0026 | 0.0025 | 1.6% |
| 2 | 2 | canopy_unbatched | outer_rs_encoding | 0.0525 | 0.0524 | 31.4% |
| 2 | 2 | canopy_unbatched | outer_second_oracle_whirs | 0.0538 | 0.0555 | 32.2% |
| 2 | 2 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.0333 | 0.0330 | 18.6% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0050 | 0.0049 | 2.8% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.1% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0057 | 0.0057 | 3.2% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0023 | 0.0022 | 1.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0398 | 0.0401 | 22.3% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0049 | 0.0048 | 2.7% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0354 | 0.0342 | 19.8% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0026 | 0.0025 | 1.5% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.0497 | 0.0497 | 27.8% |
| 0 | 4 | merkle_whir_independent | initial_merkle_tree | 0.0128 | 0.0126 | 4.3% |
| 0 | 4 | merkle_whir_independent | initial_ood_evaluations | 0.0053 | 0.0051 | 1.8% |
| 0 | 4 | merkle_whir_independent | initial_rs_encoding | 0.1050 | 0.1048 | 34.8% |
| 0 | 4 | merkle_whir_independent | whir_opening_proof_after_ood | 0.1785 | 0.1746 | 59.2% |
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0218 | 0.0217 | 8.3% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0639 | 0.0612 | 24.3% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0051 | 0.0050 | 1.9% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0128 | 0.0126 | 4.9% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0053 | 0.0051 | 2.0% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 0.1050 | 0.1048 | 39.9% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0492 | 0.0490 | 18.7% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0328 | 0.0328 | 7.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0902 | 0.0895 | 19.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0051 | 0.0050 | 1.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0255 | 0.0243 | 5.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0054 | 0.0053 | 1.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.2103 | 0.2092 | 45.6% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0923 | 0.0919 | 20.0% |
| 2 | 4 | canopy_unbatched | canopy_bottom_forest | 0.0122 | 0.0121 | 3.6% |
| 2 | 4 | canopy_unbatched | canopy_bottom_opening | 0.0006 | 0.0006 | 0.2% |
| 2 | 4 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0140 | 0.0136 | 4.2% |
| 2 | 4 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_hash_to_field | 0.0001 | 0.0001 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_rs_encoding | 0.0187 | 0.0187 | 5.6% |
| 2 | 4 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0709 | 0.0684 | 21.2% |
| 2 | 4 | canopy_unbatched | outer_ood_evaluations | 0.0053 | 0.0051 | 1.6% |
| 2 | 4 | canopy_unbatched | outer_rs_encoding | 0.1050 | 0.1048 | 31.4% |
| 2 | 4 | canopy_unbatched | outer_second_oracle_whirs | 0.1076 | 0.1110 | 32.2% |
| 2 | 4 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.0448 | 0.0446 | 13.4% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0097 | 0.0095 | 2.9% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0003 | 0.0001 | 0.1% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0108 | 0.0107 | 3.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0047 | 0.0047 | 1.4% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0778 | 0.0782 | 23.2% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0165 | 0.0163 | 4.9% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0709 | 0.0684 | 21.1% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0053 | 0.0051 | 1.6% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 0.0949 | 0.0948 | 28.3% |
| 0 | 8 | merkle_whir_independent | initial_merkle_tree | 0.0257 | 0.0252 | 4.3% |
| 0 | 8 | merkle_whir_independent | initial_ood_evaluations | 0.0106 | 0.0101 | 1.8% |
| 0 | 8 | merkle_whir_independent | initial_rs_encoding | 0.2099 | 0.2096 | 34.8% |
| 0 | 8 | merkle_whir_independent | whir_opening_proof_after_ood | 0.3571 | 0.3491 | 59.2% |
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0327 | 0.0325 | 6.3% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.1277 | 0.1224 | 24.5% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0154 | 0.0154 | 3.0% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0257 | 0.0252 | 4.9% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0106 | 0.0101 | 2.0% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 0.2099 | 0.2096 | 40.3% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0984 | 0.0981 | 18.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0437 | 0.0439 | 4.8% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.1804 | 0.1790 | 19.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0154 | 0.0154 | 1.7% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0510 | 0.0486 | 5.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0107 | 0.0106 | 1.2% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.4207 | 0.4183 | 46.4% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.1846 | 0.1839 | 20.4% |
| 2 | 8 | canopy_unbatched | canopy_bottom_forest | 0.0244 | 0.0243 | 3.6% |
| 2 | 8 | canopy_unbatched | canopy_bottom_opening | 0.0012 | 0.0012 | 0.2% |
| 2 | 8 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0281 | 0.0272 | 4.2% |
| 2 | 8 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_hash_to_field | 0.0002 | 0.0001 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_rs_encoding | 0.0375 | 0.0374 | 5.6% |
| 2 | 8 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.1418 | 0.1367 | 21.2% |
| 2 | 8 | canopy_unbatched | outer_ood_evaluations | 0.0106 | 0.0101 | 1.6% |
| 2 | 8 | canopy_unbatched | outer_rs_encoding | 0.2099 | 0.2096 | 31.4% |
| 2 | 8 | canopy_unbatched | outer_second_oracle_whirs | 0.2153 | 0.2221 | 32.2% |
| 2 | 8 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 0.0819 | 0.0818 | 12.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0195 | 0.0190 | 2.9% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0006 | 0.0003 | 0.1% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0216 | 0.0211 | 3.2% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0090 | 0.0088 | 1.3% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1533 | 0.1496 | 22.5% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0601 | 0.0599 | 8.8% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.1418 | 0.1367 | 20.8% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0106 | 0.0101 | 1.6% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 0.1827 | 0.1802 | 26.8% |
