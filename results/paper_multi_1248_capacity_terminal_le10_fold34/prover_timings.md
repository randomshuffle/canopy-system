# Conjectural capacity-bound paper prover timings

Each input oracle has `2^18` field elements. The rate-1/2 rows use a first codeword of `2^19` elements, while the added rate-1/4 batched baseline uses `2^20` elements. All WHIR components are generated and verifier-checked at 128-bit security over the 256-bit FFT field, in capacity_conjecture mode, with no proof of work. Capacity-bound soundness relies on STIR Conjecture 5.6 and is not proved. Input generation, verification, and deallocation are excluded. Commitment time contains initial RS encoding and the initial Merkle/Canopy commitment; opening time contains OOD work, openings, recursive oracle commitments, Schwartz--Zippel preprocessing, and recursive WHIR. The selected initial fold is three: Experiment 3 therefore uses aligned Canopy height four, while Experiment 2 uses its independently optimized height `h=18` and has a terminal Canopy claim. To match the paper's at-most-`2^10` stopping rule, the reference implementation's native smaller suffix is separately executed and subtracted. Independent per-input stages are measured once per repetition and scaled by `t`; every batched recursive proof and generalized distinct-functional cross-evaluation stage is executed at the reported `t`.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs independent | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.0331 | 0.0707 | 0.1038 | 0.1000 | 1.000x | 0.943x |
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0331 | 0.0770 | 0.1101 | 0.1078 | 1.060x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.0621 | 0.0838 | 0.1459 | 0.1406 | 1.406x | 1.325x |
| 2 | 1 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.0406 | 0.0708 | 0.1114 | 0.1097 | 1.073x | 1.012x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.0687 | 0.0755 | 0.1443 | 0.1420 | 1.390x | 1.310x |
| 0 | 2 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.0662 | 0.1415 | 0.2076 | 0.1999 | 1.000x | 1.163x |
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0662 | 0.1124 | 0.1785 | 0.1785 | 0.860x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.1243 | 0.1172 | 0.2415 | 0.2585 | 1.163x | 1.353x |
| 2 | 2 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.0812 | 0.1416 | 0.2228 | 0.2194 | 1.073x | 1.248x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.1375 | 0.1539 | 0.2914 | 0.2992 | 1.404x | 1.632x |
| 0 | 4 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.1323 | 0.2829 | 0.4152 | 0.3999 | 1.000x | 1.200x |
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.1323 | 0.2136 | 0.3459 | 0.3517 | 0.833x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.2485 | 0.2250 | 0.4735 | 0.5057 | 1.140x | 1.369x |
| 2 | 4 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.1624 | 0.2832 | 0.4456 | 0.4388 | 1.073x | 1.288x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.2750 | 0.3069 | 0.5818 | 0.5799 | 1.401x | 1.682x |
| 0 | 8 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.2646 | 0.5658 | 0.8304 | 0.7998 | 1.000x | 1.128x |
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.2646 | 0.4715 | 0.7361 | 0.7342 | 0.886x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.4970 | 0.4846 | 0.9816 | 1.0365 | 1.182x | 1.333x |
| 2 | 8 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.3248 | 0.5664 | 0.8912 | 0.8776 | 1.073x | 1.211x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.5500 | 0.8172 | 1.3672 | 1.4342 | 1.646x | 1.857x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | initial_merkle_tree | 0.0037 | 0.0032 | 3.6% |
| 0 | 1 | merkle_whir_independent | initial_ood_evaluations | 0.0013 | 0.0012 | 1.2% |
| 0 | 1 | merkle_whir_independent | initial_rs_encoding | 0.0294 | 0.0279 | 28.3% |
| 0 | 1 | merkle_whir_independent | whir_opening_proof_after_ood | 0.0695 | 0.0681 | 66.9% |
| 0 | 1 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0336 | 0.0183 | 30.5% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | independent_first_transitions | 0.0765 | 0.0791 | 69.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0388 | 0.0225 | 26.6% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0065 | 0.0062 | 4.4% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0020 | 0.0017 | 1.4% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.0557 | 0.0562 | 38.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | whir_first_transition_after_ood | 0.0433 | 0.0517 | 29.6% |
| 2 | 1 | canopy_unbatched | canopy_bottom_forest | 0.0032 | 0.0031 | 2.9% |
| 2 | 1 | canopy_unbatched | canopy_bottom_opening | 0.0001 | 0.0001 | 0.1% |
| 2 | 1 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0035 | 0.0033 | 3.2% |
| 2 | 1 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_hash_to_field | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_rs_encoding | 0.0044 | 0.0040 | 4.0% |
| 2 | 1 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0463 | 0.0440 | 41.6% |
| 2 | 1 | canopy_unbatched | outer_ood_evaluations | 0.0013 | 0.0012 | 1.1% |
| 2 | 1 | canopy_unbatched | outer_rs_encoding | 0.0294 | 0.0279 | 26.4% |
| 2 | 1 | canopy_unbatched | outer_second_oracle_whirs | 0.0232 | 0.0238 | 20.8% |
| 2 | 1 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.0254 | 0.0243 | 17.6% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0033 | 0.0026 | 2.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0041 | 0.0032 | 2.9% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0088 | 0.0083 | 6.1% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0231 | 0.0215 | 16.0% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0025 | 0.0025 | 1.7% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0463 | 0.0440 | 32.1% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0013 | 0.0012 | 0.9% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.0294 | 0.0279 | 20.4% |
| 0 | 2 | merkle_whir_independent | initial_merkle_tree | 0.0074 | 0.0065 | 3.6% |
| 0 | 2 | merkle_whir_independent | initial_ood_evaluations | 0.0025 | 0.0025 | 1.2% |
| 0 | 2 | merkle_whir_independent | initial_rs_encoding | 0.0587 | 0.0558 | 28.3% |
| 0 | 2 | merkle_whir_independent | whir_opening_proof_after_ood | 0.1389 | 0.1361 | 66.9% |
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0233 | 0.0184 | 13.1% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0022 | 0.0022 | 1.2% |
| 0 | 2 | merkle_whir_batched_after_second | independent_first_transitions | 0.1530 | 0.1583 | 85.7% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0252 | 0.0243 | 10.4% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0022 | 0.0022 | 0.9% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0129 | 0.0125 | 5.4% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0039 | 0.0033 | 1.6% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.1113 | 0.1125 | 46.1% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | whir_first_transition_after_ood | 0.0865 | 0.1033 | 35.8% |
| 2 | 2 | canopy_unbatched | canopy_bottom_forest | 0.0064 | 0.0061 | 2.9% |
| 2 | 2 | canopy_unbatched | canopy_bottom_opening | 0.0002 | 0.0002 | 0.1% |
| 2 | 2 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0071 | 0.0065 | 3.2% |
| 2 | 2 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_hash_to_field | 0.0001 | 0.0001 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_rs_encoding | 0.0089 | 0.0081 | 4.0% |
| 2 | 2 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0926 | 0.0879 | 41.6% |
| 2 | 2 | canopy_unbatched | outer_ood_evaluations | 0.0025 | 0.0025 | 1.1% |
| 2 | 2 | canopy_unbatched | outer_rs_encoding | 0.0587 | 0.0558 | 26.4% |
| 2 | 2 | canopy_unbatched | outer_second_oracle_whirs | 0.0463 | 0.0476 | 20.8% |
| 2 | 2 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.0386 | 0.0296 | 13.2% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0065 | 0.0052 | 2.2% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0082 | 0.0064 | 2.8% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0177 | 0.0167 | 6.1% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0463 | 0.0430 | 15.9% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0201 | 0.0145 | 6.9% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0926 | 0.0879 | 31.8% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0025 | 0.0025 | 0.9% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.0587 | 0.0558 | 20.2% |
| 0 | 4 | merkle_whir_independent | initial_merkle_tree | 0.0149 | 0.0129 | 3.6% |
| 0 | 4 | merkle_whir_independent | initial_ood_evaluations | 0.0051 | 0.0050 | 1.2% |
| 0 | 4 | merkle_whir_independent | initial_rs_encoding | 0.1174 | 0.1116 | 28.3% |
| 0 | 4 | merkle_whir_independent | whir_opening_proof_after_ood | 0.2778 | 0.2723 | 66.9% |
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0246 | 0.0238 | 7.1% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0154 | 0.0130 | 4.4% |
| 0 | 4 | merkle_whir_batched_after_second | independent_first_transitions | 0.3060 | 0.3165 | 88.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0298 | 0.0296 | 6.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0154 | 0.0130 | 3.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0259 | 0.0249 | 5.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0079 | 0.0067 | 1.7% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.2226 | 0.2249 | 47.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | whir_first_transition_after_ood | 0.1730 | 0.2066 | 36.5% |
| 2 | 4 | canopy_unbatched | canopy_bottom_forest | 0.0129 | 0.0123 | 2.9% |
| 2 | 4 | canopy_unbatched | canopy_bottom_opening | 0.0003 | 0.0003 | 0.1% |
| 2 | 4 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0141 | 0.0130 | 3.2% |
| 2 | 4 | canopy_unbatched | canopy_commitment_other | 0.0001 | 0.0001 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_hash_to_field | 0.0001 | 0.0001 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_rs_encoding | 0.0177 | 0.0162 | 4.0% |
| 2 | 4 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.1852 | 0.1759 | 41.6% |
| 2 | 4 | canopy_unbatched | outer_ood_evaluations | 0.0051 | 0.0050 | 1.1% |
| 2 | 4 | canopy_unbatched | outer_rs_encoding | 0.1174 | 0.1116 | 26.4% |
| 2 | 4 | canopy_unbatched | outer_second_oracle_whirs | 0.0926 | 0.0952 | 20.8% |
| 2 | 4 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.0483 | 0.0487 | 8.3% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0130 | 0.0103 | 2.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0165 | 0.0129 | 2.8% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0001 | 0.0001 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0354 | 0.0333 | 6.1% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0926 | 0.0860 | 15.9% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0682 | 0.0680 | 11.7% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.1852 | 0.1759 | 31.8% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0051 | 0.0050 | 0.9% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 0.1174 | 0.1116 | 20.2% |
| 0 | 8 | merkle_whir_independent | initial_merkle_tree | 0.0297 | 0.0258 | 3.6% |
| 0 | 8 | merkle_whir_independent | initial_ood_evaluations | 0.0102 | 0.0100 | 1.2% |
| 0 | 8 | merkle_whir_independent | initial_rs_encoding | 0.2349 | 0.2231 | 28.3% |
| 0 | 8 | merkle_whir_independent | whir_opening_proof_after_ood | 0.5557 | 0.5445 | 66.9% |
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0465 | 0.0429 | 6.3% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0777 | 0.0614 | 10.6% |
| 0 | 8 | merkle_whir_batched_after_second | independent_first_transitions | 0.6120 | 0.6330 | 83.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0474 | 0.0458 | 4.8% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0777 | 0.0614 | 7.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0518 | 0.0498 | 5.3% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0158 | 0.0133 | 1.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.4453 | 0.4498 | 45.4% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | whir_first_transition_after_ood | 0.3460 | 0.4133 | 35.2% |
| 2 | 8 | canopy_unbatched | canopy_bottom_forest | 0.0258 | 0.0246 | 2.9% |
| 2 | 8 | canopy_unbatched | canopy_bottom_opening | 0.0006 | 0.0006 | 0.1% |
| 2 | 8 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0283 | 0.0261 | 3.2% |
| 2 | 8 | canopy_unbatched | canopy_commitment_other | 0.0001 | 0.0001 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_hash_to_field | 0.0002 | 0.0003 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_rs_encoding | 0.0355 | 0.0324 | 4.0% |
| 2 | 8 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.3704 | 0.3517 | 41.6% |
| 2 | 8 | canopy_unbatched | outer_ood_evaluations | 0.0102 | 0.0100 | 1.1% |
| 2 | 8 | canopy_unbatched | outer_rs_encoding | 0.2349 | 0.2231 | 26.4% |
| 2 | 8 | canopy_unbatched | outer_second_oracle_whirs | 0.1853 | 0.1904 | 20.8% |
| 2 | 8 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 0.1136 | 0.1118 | 8.3% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0260 | 0.0206 | 1.9% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0329 | 0.0257 | 2.4% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0001 | 0.0001 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0708 | 0.0666 | 5.2% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1852 | 0.1720 | 13.5% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.3229 | 0.2950 | 23.6% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.3704 | 0.3517 | 27.1% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0102 | 0.0100 | 0.7% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 0.2349 | 0.2231 | 17.2% |
