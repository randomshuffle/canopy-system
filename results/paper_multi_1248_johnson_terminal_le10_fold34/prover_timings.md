# Final Johnson-bound paper prover timings

Each input oracle has `2^18` field elements. The rate-1/2 rows use a first codeword of `2^19` elements, while the added rate-1/4 batched baseline uses `2^20` elements. All WHIR components are generated and verifier-checked at 128-bit security over the 256-bit FFT field, in johnson mode, with no proof of work. Input generation, verification, and deallocation are excluded. Commitment time contains initial RS encoding and the initial Merkle/Canopy commitment; opening time contains OOD work, openings, recursive oracle commitments, Schwartz--Zippel preprocessing, and recursive WHIR. The selected initial fold is three: Experiment 3 therefore uses aligned Canopy height four, while Experiment 2 uses its independently optimized height `h=18` and has a terminal Canopy claim. To match the paper's at-most-`2^10` stopping rule, the reference implementation's native smaller suffix is separately executed and subtracted. Independent per-input stages are measured once per repetition and scaled by `t`; every batched recursive proof and generalized distinct-functional cross-evaluation stage is executed at the reported `t`.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs independent | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.0277 | 0.0900 | 0.1177 | 0.1157 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0277 | 0.0900 | 0.1177 | 0.1157 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.0578 | 0.0821 | 0.1399 | 0.1359 | 1.189x | 1.189x |
| 2 | 1 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.0347 | 0.0902 | 0.1249 | 0.1221 | 1.061x | 1.061x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.0577 | 0.0944 | 0.1522 | 0.1484 | 1.293x | 1.293x |
| 0 | 2 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.0553 | 0.1801 | 0.2354 | 0.2314 | 1.000x | 1.082x |
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0553 | 0.1622 | 0.2175 | 0.2122 | 0.924x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.1155 | 0.1461 | 0.2616 | 0.2533 | 1.111x | 1.203x |
| 2 | 2 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.0694 | 0.1804 | 0.2497 | 0.2442 | 1.061x | 1.148x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.1155 | 0.1754 | 0.2908 | 0.2867 | 1.235x | 1.337x |
| 0 | 4 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.1106 | 0.3602 | 0.4708 | 0.4627 | 1.000x | 1.104x |
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.1106 | 0.3158 | 0.4265 | 0.4156 | 0.906x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.2311 | 0.2724 | 0.5035 | 0.4908 | 1.069x | 1.181x |
| 2 | 4 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.1387 | 0.3608 | 0.4995 | 0.4884 | 1.061x | 1.171x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.2309 | 0.3705 | 0.6014 | 0.5964 | 1.277x | 1.410x |
| 0 | 8 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.2213 | 0.7204 | 0.9416 | 0.9254 | 1.000x | 1.085x |
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.2213 | 0.6464 | 0.8677 | 0.8526 | 0.921x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.4622 | 0.5625 | 1.0247 | 0.9978 | 1.088x | 1.181x |
| 2 | 8 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.2775 | 0.7215 | 0.9990 | 0.9767 | 1.061x | 1.151x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.4618 | 0.9141 | 1.3759 | 1.3571 | 1.461x | 1.586x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | initial_merkle_tree | 0.0034 | 0.0034 | 2.9% |
| 0 | 1 | merkle_whir_independent | initial_ood_evaluations | 0.0012 | 0.0011 | 1.0% |
| 0 | 1 | merkle_whir_independent | initial_rs_encoding | 0.0242 | 0.0239 | 20.6% |
| 0 | 1 | merkle_whir_independent | whir_opening_proof_after_ood | 0.0889 | 0.0874 | 75.5% |
| 0 | 1 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0226 | 0.0224 | 19.2% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | independent_first_transitions | 0.0951 | 0.0933 | 80.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0257 | 0.0253 | 18.4% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0059 | 0.0058 | 4.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0014 | 0.0012 | 1.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.0518 | 0.0502 | 37.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | whir_first_transition_after_ood | 0.0550 | 0.0535 | 39.3% |
| 2 | 1 | canopy_unbatched | canopy_bottom_forest | 0.0030 | 0.0029 | 2.4% |
| 2 | 1 | canopy_unbatched | canopy_bottom_opening | 0.0001 | 0.0001 | 0.1% |
| 2 | 1 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0033 | 0.0032 | 2.7% |
| 2 | 1 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_hash_to_field | 0.0000 | 0.0001 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_rs_encoding | 0.0040 | 0.0040 | 3.2% |
| 2 | 1 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0619 | 0.0623 | 49.6% |
| 2 | 1 | canopy_unbatched | outer_ood_evaluations | 0.0012 | 0.0011 | 0.9% |
| 2 | 1 | canopy_unbatched | outer_rs_encoding | 0.0242 | 0.0239 | 19.4% |
| 2 | 1 | canopy_unbatched | outer_second_oracle_whirs | 0.0270 | 0.0266 | 21.6% |
| 2 | 1 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.0289 | 0.0277 | 19.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0027 | 0.0027 | 1.8% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0032 | 0.0032 | 2.1% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0077 | 0.0075 | 5.1% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0199 | 0.0195 | 13.1% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0024 | 0.0024 | 1.6% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0619 | 0.0623 | 40.7% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0012 | 0.0011 | 0.8% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.0242 | 0.0239 | 15.9% |
| 0 | 2 | merkle_whir_independent | initial_merkle_tree | 0.0068 | 0.0068 | 2.9% |
| 0 | 2 | merkle_whir_independent | initial_ood_evaluations | 0.0023 | 0.0023 | 1.0% |
| 0 | 2 | merkle_whir_independent | initial_rs_encoding | 0.0485 | 0.0478 | 20.6% |
| 0 | 2 | merkle_whir_independent | whir_opening_proof_after_ood | 0.1778 | 0.1748 | 75.5% |
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0250 | 0.0235 | 11.5% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0022 | 0.0022 | 1.0% |
| 0 | 2 | merkle_whir_batched_after_second | independent_first_transitions | 0.1903 | 0.1866 | 87.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0311 | 0.0277 | 11.9% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0022 | 0.0022 | 0.8% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0119 | 0.0116 | 4.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0027 | 0.0024 | 1.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.1037 | 0.1005 | 39.6% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | whir_first_transition_after_ood | 0.1101 | 0.1071 | 42.1% |
| 2 | 2 | canopy_unbatched | canopy_bottom_forest | 0.0060 | 0.0058 | 2.4% |
| 2 | 2 | canopy_unbatched | canopy_bottom_opening | 0.0003 | 0.0003 | 0.1% |
| 2 | 2 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0067 | 0.0064 | 2.7% |
| 2 | 2 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_hash_to_field | 0.0001 | 0.0001 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_rs_encoding | 0.0081 | 0.0081 | 3.2% |
| 2 | 2 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.1238 | 0.1246 | 49.6% |
| 2 | 2 | canopy_unbatched | outer_ood_evaluations | 0.0023 | 0.0023 | 0.9% |
| 2 | 2 | canopy_unbatched | outer_rs_encoding | 0.0485 | 0.0478 | 19.4% |
| 2 | 2 | canopy_unbatched | outer_second_oracle_whirs | 0.0540 | 0.0532 | 21.6% |
| 2 | 2 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.0340 | 0.0331 | 11.7% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0054 | 0.0053 | 1.9% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0064 | 0.0064 | 2.2% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0154 | 0.0149 | 5.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0398 | 0.0389 | 13.7% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0152 | 0.0145 | 5.2% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.1238 | 0.1246 | 42.6% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0023 | 0.0023 | 0.8% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.0485 | 0.0478 | 16.7% |
| 0 | 4 | merkle_whir_independent | initial_merkle_tree | 0.0136 | 0.0135 | 2.9% |
| 0 | 4 | merkle_whir_independent | initial_ood_evaluations | 0.0046 | 0.0046 | 1.0% |
| 0 | 4 | merkle_whir_independent | initial_rs_encoding | 0.0970 | 0.0955 | 20.6% |
| 0 | 4 | merkle_whir_independent | whir_opening_proof_after_ood | 0.3556 | 0.3496 | 75.5% |
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0329 | 0.0295 | 7.7% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0130 | 0.0130 | 3.0% |
| 0 | 4 | merkle_whir_batched_after_second | independent_first_transitions | 0.3805 | 0.3732 | 89.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0338 | 0.0336 | 6.7% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0130 | 0.0130 | 2.6% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0237 | 0.0231 | 4.7% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0055 | 0.0048 | 1.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.2074 | 0.2009 | 41.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | whir_first_transition_after_ood | 0.2201 | 0.2142 | 43.7% |
| 2 | 4 | canopy_unbatched | canopy_bottom_forest | 0.0119 | 0.0117 | 2.4% |
| 2 | 4 | canopy_unbatched | canopy_bottom_opening | 0.0006 | 0.0006 | 0.1% |
| 2 | 4 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0134 | 0.0127 | 2.7% |
| 2 | 4 | canopy_unbatched | canopy_commitment_other | 0.0001 | 0.0001 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_hash_to_field | 0.0002 | 0.0002 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_rs_encoding | 0.0162 | 0.0161 | 3.2% |
| 2 | 4 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.2476 | 0.2491 | 49.6% |
| 2 | 4 | canopy_unbatched | outer_ood_evaluations | 0.0046 | 0.0046 | 0.9% |
| 2 | 4 | canopy_unbatched | outer_rs_encoding | 0.0970 | 0.0955 | 19.4% |
| 2 | 4 | canopy_unbatched | outer_second_oracle_whirs | 0.1080 | 0.1064 | 21.6% |
| 2 | 4 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.0503 | 0.0500 | 8.4% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0108 | 0.0106 | 1.8% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0128 | 0.0127 | 2.1% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0001 | 0.0001 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0308 | 0.0299 | 5.1% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0795 | 0.0778 | 13.2% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0679 | 0.0675 | 11.3% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.2476 | 0.2491 | 41.2% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0046 | 0.0046 | 0.8% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 0.0970 | 0.0955 | 16.1% |
| 0 | 8 | merkle_whir_independent | initial_merkle_tree | 0.0273 | 0.0270 | 2.9% |
| 0 | 8 | merkle_whir_independent | initial_ood_evaluations | 0.0092 | 0.0092 | 1.0% |
| 0 | 8 | merkle_whir_independent | initial_rs_encoding | 0.1940 | 0.1911 | 20.6% |
| 0 | 8 | merkle_whir_independent | whir_opening_proof_after_ood | 0.7111 | 0.6993 | 75.5% |
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0460 | 0.0453 | 5.3% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0606 | 0.0605 | 7.0% |
| 0 | 8 | merkle_whir_batched_after_second | independent_first_transitions | 0.7611 | 0.7464 | 87.7% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0507 | 0.0500 | 4.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0606 | 0.0605 | 5.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0474 | 0.0463 | 4.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0110 | 0.0097 | 1.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.4147 | 0.4019 | 40.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | whir_first_transition_after_ood | 0.4403 | 0.4283 | 43.0% |
| 2 | 8 | canopy_unbatched | canopy_bottom_forest | 0.0239 | 0.0233 | 2.4% |
| 2 | 8 | canopy_unbatched | canopy_bottom_opening | 0.0012 | 0.0012 | 0.1% |
| 2 | 8 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0268 | 0.0255 | 2.7% |
| 2 | 8 | canopy_unbatched | canopy_commitment_other | 0.0001 | 0.0001 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_hash_to_field | 0.0004 | 0.0004 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_rs_encoding | 0.0323 | 0.0322 | 3.2% |
| 2 | 8 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.4951 | 0.4983 | 49.6% |
| 2 | 8 | canopy_unbatched | outer_ood_evaluations | 0.0092 | 0.0092 | 0.9% |
| 2 | 8 | canopy_unbatched | outer_rs_encoding | 0.1940 | 0.1911 | 19.4% |
| 2 | 8 | canopy_unbatched | outer_second_oracle_whirs | 0.2160 | 0.2129 | 21.6% |
| 2 | 8 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 0.1102 | 0.1075 | 8.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0215 | 0.0213 | 1.6% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0004 | 0.0003 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0255 | 0.0254 | 1.9% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0001 | 0.0001 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0616 | 0.0598 | 4.5% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1590 | 0.1557 | 11.6% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.2992 | 0.2893 | 21.7% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.4951 | 0.4983 | 36.0% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0092 | 0.0092 | 0.7% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 0.1940 | 0.1911 | 14.1% |
