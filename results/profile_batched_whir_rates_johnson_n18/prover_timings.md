# Final Johnson-bound paper prover timings

Each input oracle has `2^18` field elements. The rate-1/2 rows use a first codeword of `2^19` elements, while the added rate-1/4 batched baseline uses `2^20` elements. All WHIR components are generated and verifier-checked at 128-bit security over the 256-bit FFT field, in johnson mode, with no proof of work. Input generation, verification, and deallocation are excluded. Commitment time contains initial RS encoding and the initial Merkle/Canopy commitment; opening time contains OOD work, openings, recursive oracle commitments, Schwartz--Zippel preprocessing, and recursive WHIR. The selected initial fold is three: Experiment 3 therefore uses aligned Canopy height four, while Experiment 2 uses its independently optimized height `h=18` and has a terminal Canopy claim. To match the paper's at-most-`2^10` stopping rule, the reference implementation's native smaller suffix is separately executed and subtracted. Independent per-input stages are measured once per repetition and scaled by `t`; every batched recursive proof and generalized distinct-functional cross-evaluation stage is executed at the reported `t`.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs independent | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | 1/2 | 19 | - | 30 | 0.0290 | 0.0936 | 0.1227 | 0.1222 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 19 | - | 30 | 0.0290 | 0.0936 | 0.1227 | 0.1222 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 30 | 0.0598 | 0.0917 | 0.1515 | 0.1453 | 1.235x | 1.235x |
| 2 | 1 | canopy_unbatched | 1/2 | 19 | 18 | 30 | 0.0366 | 0.0938 | 0.1304 | 0.1298 | 1.063x | 1.063x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 30 | 0.0624 | 0.0973 | 0.1597 | 0.1578 | 1.302x | 1.302x |
| 0 | 2 | merkle_whir_independent | 1/2 | 19 | - | 30 | 0.0581 | 0.1872 | 0.2453 | 0.2444 | 1.000x | 1.098x |
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 19 | - | 30 | 0.0581 | 0.1654 | 0.2235 | 0.2260 | 0.911x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 30 | 0.1196 | 0.1432 | 0.2628 | 0.2677 | 1.071x | 1.176x |
| 2 | 2 | canopy_unbatched | 1/2 | 19 | 18 | 30 | 0.0732 | 0.1875 | 0.2607 | 0.2595 | 1.063x | 1.167x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 30 | 0.1249 | 0.1802 | 0.3051 | 0.3030 | 1.244x | 1.365x |
| 0 | 4 | merkle_whir_independent | 1/2 | 19 | - | 30 | 0.1162 | 0.3744 | 0.4906 | 0.4888 | 1.000x | 1.123x |
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 19 | - | 30 | 0.1162 | 0.3207 | 0.4368 | 0.4429 | 0.890x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 30 | 0.2392 | 0.2695 | 0.5087 | 0.5209 | 1.037x | 1.165x |
| 2 | 4 | canopy_unbatched | 1/2 | 19 | 18 | 30 | 0.1463 | 0.3751 | 0.5214 | 0.5190 | 1.063x | 1.194x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 30 | 0.2497 | 0.3819 | 0.6316 | 0.6278 | 1.287x | 1.446x |
| 0 | 8 | merkle_whir_independent | 1/2 | 19 | - | 30 | 0.2324 | 0.7489 | 0.9812 | 0.9777 | 1.000x | 1.096x |
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 19 | - | 30 | 0.2324 | 0.6629 | 0.8953 | 0.9068 | 0.912x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 30 | 0.4784 | 0.5558 | 1.0342 | 1.0593 | 1.054x | 1.155x |
| 2 | 8 | canopy_unbatched | 1/2 | 19 | 18 | 30 | 0.2926 | 0.7502 | 1.0428 | 1.0380 | 1.063x | 1.165x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 30 | 0.4995 | 0.9263 | 1.4257 | 1.4112 | 1.453x | 1.592x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | initial_merkle_tree | 0.0033 | 0.0033 | 2.7% |
| 0 | 1 | merkle_whir_independent | initial_ood_evaluations | 0.0013 | 0.0013 | 1.0% |
| 0 | 1 | merkle_whir_independent | initial_rs_encoding | 0.0257 | 0.0258 | 21.0% |
| 0 | 1 | merkle_whir_independent | whir_opening_proof_after_ood | 0.0923 | 0.0918 | 75.3% |
| 0 | 1 | merkle_whir_batched_after_second | first_transition_other | 0.0554 | 0.0572 | 45.2% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0033 | 0.0033 | 2.7% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0013 | 0.0013 | 1.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0257 | 0.0258 | 21.0% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.0239 | 0.0220 | 19.5% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0003 | 0.0003 | 0.2% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0152 | 0.0123 | 12.4% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_other | 0.0315 | 0.0335 | 20.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0063 | 0.0061 | 4.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0013 | 0.0013 | 0.9% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.0535 | 0.0531 | 35.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.0359 | 0.0267 | 23.7% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0004 | 0.0002 | 0.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0253 | 0.0234 | 16.7% |
| 2 | 1 | canopy_unbatched | canopy_bottom_forest | 0.0031 | 0.0030 | 2.4% |
| 2 | 1 | canopy_unbatched | canopy_bottom_opening | 0.0002 | 0.0002 | 0.1% |
| 2 | 1 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0034 | 0.0034 | 2.6% |
| 2 | 1 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_hash_to_field | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_rs_encoding | 0.0043 | 0.0042 | 3.3% |
| 2 | 1 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0640 | 0.0650 | 49.1% |
| 2 | 1 | canopy_unbatched | outer_ood_evaluations | 0.0013 | 0.0013 | 1.0% |
| 2 | 1 | canopy_unbatched | outer_rs_encoding | 0.0257 | 0.0258 | 19.7% |
| 2 | 1 | canopy_unbatched | outer_second_oracle_whirs | 0.0284 | 0.0269 | 21.8% |
| 2 | 1 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.0295 | 0.0291 | 18.5% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0030 | 0.0026 | 1.9% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0035 | 0.0033 | 2.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0092 | 0.0077 | 5.8% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0210 | 0.0205 | 13.1% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0025 | 0.0024 | 1.6% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0640 | 0.0650 | 40.0% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0013 | 0.0013 | 0.8% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.0257 | 0.0258 | 16.1% |
| 0 | 2 | merkle_whir_independent | initial_merkle_tree | 0.0066 | 0.0066 | 2.7% |
| 0 | 2 | merkle_whir_independent | initial_ood_evaluations | 0.0025 | 0.0025 | 1.0% |
| 0 | 2 | merkle_whir_independent | initial_rs_encoding | 0.0515 | 0.0516 | 21.0% |
| 0 | 2 | merkle_whir_independent | whir_opening_proof_after_ood | 0.1847 | 0.1836 | 75.3% |
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0243 | 0.0244 | 10.9% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_other | 0.1108 | 0.1143 | 49.6% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0022 | 0.0022 | 1.0% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0066 | 0.0066 | 3.0% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0025 | 0.0025 | 1.1% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0515 | 0.0516 | 23.0% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0305 | 0.0246 | 13.6% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0301 | 0.0293 | 11.4% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_other | 0.0630 | 0.0670 | 24.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0022 | 0.0022 | 0.8% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0126 | 0.0123 | 4.8% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0026 | 0.0026 | 1.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.1070 | 0.1062 | 40.7% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0506 | 0.0469 | 19.3% |
| 2 | 2 | canopy_unbatched | canopy_bottom_forest | 0.0061 | 0.0061 | 2.4% |
| 2 | 2 | canopy_unbatched | canopy_bottom_opening | 0.0003 | 0.0003 | 0.1% |
| 2 | 2 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0068 | 0.0067 | 2.6% |
| 2 | 2 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_hash_to_field | 0.0000 | 0.0000 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_rs_encoding | 0.0087 | 0.0085 | 3.3% |
| 2 | 2 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.1279 | 0.1299 | 49.1% |
| 2 | 2 | canopy_unbatched | outer_ood_evaluations | 0.0025 | 0.0025 | 1.0% |
| 2 | 2 | canopy_unbatched | outer_rs_encoding | 0.0515 | 0.0516 | 19.7% |
| 2 | 2 | canopy_unbatched | outer_second_oracle_whirs | 0.0568 | 0.0539 | 21.8% |
| 2 | 2 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.0349 | 0.0345 | 11.5% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0060 | 0.0051 | 2.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0069 | 0.0067 | 2.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0185 | 0.0154 | 6.1% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0420 | 0.0411 | 13.8% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0147 | 0.0146 | 4.8% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.1279 | 0.1299 | 41.9% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0025 | 0.0025 | 0.8% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.0515 | 0.0516 | 16.9% |
| 0 | 4 | merkle_whir_independent | initial_merkle_tree | 0.0133 | 0.0131 | 2.7% |
| 0 | 4 | merkle_whir_independent | initial_ood_evaluations | 0.0051 | 0.0050 | 1.0% |
| 0 | 4 | merkle_whir_independent | initial_rs_encoding | 0.1029 | 0.1031 | 21.0% |
| 0 | 4 | merkle_whir_independent | whir_opening_proof_after_ood | 0.3694 | 0.3671 | 75.3% |
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0298 | 0.0297 | 6.8% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_other | 0.2215 | 0.2286 | 50.7% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0132 | 0.0131 | 3.0% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0133 | 0.0131 | 3.0% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0051 | 0.0050 | 1.2% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 0.1029 | 0.1031 | 23.6% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0609 | 0.0493 | 13.9% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0345 | 0.0343 | 6.8% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_other | 0.1261 | 0.1340 | 24.8% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0132 | 0.0131 | 2.6% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0253 | 0.0245 | 5.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0053 | 0.0052 | 1.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.2139 | 0.2124 | 42.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.1012 | 0.0937 | 19.9% |
| 2 | 4 | canopy_unbatched | canopy_bottom_forest | 0.0123 | 0.0122 | 2.4% |
| 2 | 4 | canopy_unbatched | canopy_bottom_opening | 0.0007 | 0.0006 | 0.1% |
| 2 | 4 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0136 | 0.0135 | 2.6% |
| 2 | 4 | canopy_unbatched | canopy_commitment_other | 0.0001 | 0.0001 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_hash_to_field | 0.0001 | 0.0001 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_rs_encoding | 0.0174 | 0.0169 | 3.3% |
| 2 | 4 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.2558 | 0.2598 | 49.1% |
| 2 | 4 | canopy_unbatched | outer_ood_evaluations | 0.0051 | 0.0050 | 1.0% |
| 2 | 4 | canopy_unbatched | outer_rs_encoding | 0.1029 | 0.1031 | 19.7% |
| 2 | 4 | canopy_unbatched | outer_second_oracle_whirs | 0.1136 | 0.1077 | 21.8% |
| 2 | 4 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.0523 | 0.0514 | 8.3% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0120 | 0.0103 | 1.9% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0138 | 0.0133 | 2.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0001 | 0.0001 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0370 | 0.0308 | 5.9% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0839 | 0.0821 | 13.3% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0685 | 0.0683 | 10.8% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.2558 | 0.2598 | 40.5% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0051 | 0.0050 | 0.8% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 0.1029 | 0.1031 | 16.3% |
| 0 | 8 | merkle_whir_independent | initial_merkle_tree | 0.0266 | 0.0262 | 2.7% |
| 0 | 8 | merkle_whir_independent | initial_ood_evaluations | 0.0101 | 0.0101 | 1.0% |
| 0 | 8 | merkle_whir_independent | initial_rs_encoding | 0.2058 | 0.2063 | 21.0% |
| 0 | 8 | merkle_whir_independent | whir_opening_proof_after_ood | 0.7387 | 0.7343 | 75.3% |
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0463 | 0.0460 | 5.2% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_other | 0.4430 | 0.4573 | 49.5% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0612 | 0.0610 | 6.8% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0266 | 0.0262 | 3.0% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0101 | 0.0101 | 1.1% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 0.2058 | 0.2063 | 23.0% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1218 | 0.0985 | 13.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0508 | 0.0507 | 4.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_other | 0.2522 | 0.2680 | 24.4% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0612 | 0.0610 | 5.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0506 | 0.0491 | 4.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0105 | 0.0104 | 1.0% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.4279 | 0.4249 | 41.4% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.2024 | 0.1874 | 19.6% |
| 2 | 8 | canopy_unbatched | canopy_bottom_forest | 0.0245 | 0.0243 | 2.4% |
| 2 | 8 | canopy_unbatched | canopy_bottom_opening | 0.0013 | 0.0012 | 0.1% |
| 2 | 8 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0272 | 0.0269 | 2.6% |
| 2 | 8 | canopy_unbatched | canopy_commitment_other | 0.0001 | 0.0001 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_hash_to_field | 0.0002 | 0.0001 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_rs_encoding | 0.0347 | 0.0339 | 3.3% |
| 2 | 8 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.5116 | 0.5197 | 49.1% |
| 2 | 8 | canopy_unbatched | outer_ood_evaluations | 0.0101 | 0.0101 | 1.0% |
| 2 | 8 | canopy_unbatched | outer_rs_encoding | 0.2058 | 0.2063 | 19.7% |
| 2 | 8 | canopy_unbatched | outer_second_oracle_whirs | 0.2271 | 0.2155 | 21.8% |
| 2 | 8 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 0.1118 | 0.1111 | 7.8% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0240 | 0.0205 | 1.7% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0004 | 0.0004 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0277 | 0.0267 | 1.9% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0001 | 0.0001 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0740 | 0.0616 | 5.2% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1678 | 0.1642 | 11.8% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.2924 | 0.2917 | 20.5% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.5116 | 0.5197 | 35.9% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0101 | 0.0101 | 0.7% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 0.2058 | 0.2063 | 14.4% |
