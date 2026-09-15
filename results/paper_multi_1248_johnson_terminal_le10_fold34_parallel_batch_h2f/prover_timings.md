# Final Johnson-bound paper prover timings

Each input oracle has `2^18` field elements. The rate-1/2 rows use a first codeword of `2^19` elements, while the added rate-1/4 batched baseline uses `2^20` elements. All WHIR components are generated and verifier-checked at 128-bit security over the 256-bit FFT field, in johnson mode, with no proof of work. Input generation, verification, and deallocation are excluded. Commitment time contains initial RS encoding and the initial Merkle/Canopy commitment; opening time contains OOD work, openings, recursive oracle commitments, Schwartz--Zippel preprocessing, and recursive WHIR. The selected initial fold is three: Experiment 3 therefore uses aligned Canopy height four, while Experiment 2 uses its independently optimized height `h=18` and has a terminal Canopy claim. To match the paper's at-most-`2^10` stopping rule, the reference implementation's native smaller suffix is separately executed and subtracted. Independent per-input stages are measured once per repetition and scaled by `t`; every batched recursive proof and generalized distinct-functional cross-evaluation stage is executed at the reported `t`.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs independent | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.0311 | 0.0479 | 0.0790 | 0.0750 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0311 | 0.0479 | 0.0790 | 0.0750 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.0602 | 0.0819 | 0.1421 | 0.1339 | 1.799x | 1.799x |
| 2 | 1 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.0393 | 0.0480 | 0.0873 | 0.0833 | 1.106x | 1.106x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.0555 | 0.0532 | 0.1086 | 0.1045 | 1.375x | 1.375x |
| 0 | 2 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.0623 | 0.0957 | 0.1580 | 0.1499 | 1.000x | 1.092x |
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0623 | 0.0824 | 0.1447 | 0.1356 | 0.916x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.1204 | 0.1409 | 0.2614 | 0.2445 | 1.654x | 1.806x |
| 2 | 2 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.0787 | 0.0960 | 0.1747 | 0.1666 | 1.106x | 1.207x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.1175 | 0.0813 | 0.1988 | 0.1885 | 1.259x | 1.374x |
| 0 | 4 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.1245 | 0.1914 | 0.3159 | 0.2999 | 1.000x | 1.135x |
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.1245 | 0.1538 | 0.2784 | 0.2619 | 0.881x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.2409 | 0.2601 | 0.5009 | 0.4694 | 1.586x | 1.800x |
| 2 | 4 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.1573 | 0.1921 | 0.3494 | 0.3331 | 1.106x | 1.255x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.2358 | 0.1496 | 0.3854 | 0.3727 | 1.220x | 1.384x |
| 0 | 8 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.2491 | 0.3828 | 0.6319 | 0.5997 | 1.000x | 1.148x |
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.2491 | 0.3012 | 0.5503 | 0.5166 | 0.871x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.4817 | 0.5013 | 0.9830 | 0.9226 | 1.556x | 1.786x |
| 2 | 8 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.3146 | 0.3841 | 0.6988 | 0.6663 | 1.106x | 1.270x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.4553 | 0.3426 | 0.7979 | 0.7592 | 1.263x | 1.450x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | initial_merkle_tree | 0.0033 | 0.0033 | 4.2% |
| 0 | 1 | merkle_whir_independent | initial_ood_evaluations | 0.0012 | 0.0012 | 1.6% |
| 0 | 1 | merkle_whir_independent | initial_rs_encoding | 0.0278 | 0.0263 | 35.2% |
| 0 | 1 | merkle_whir_independent | whir_opening_proof_after_ood | 0.0466 | 0.0441 | 59.0% |
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0169 | 0.0150 | 21.4% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0033 | 0.0033 | 4.2% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0012 | 0.0012 | 1.6% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0278 | 0.0263 | 35.2% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.0166 | 0.0167 | 21.0% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0003 | 0.0003 | 0.4% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0128 | 0.0126 | 16.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0292 | 0.0233 | 20.6% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0062 | 0.0061 | 4.4% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0013 | 0.0013 | 0.9% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.0540 | 0.0533 | 38.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.0272 | 0.0276 | 19.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0003 | 0.0003 | 0.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0239 | 0.0232 | 16.8% |
| 2 | 1 | canopy_unbatched | canopy_bottom_forest | 0.0031 | 0.0030 | 3.6% |
| 2 | 1 | canopy_unbatched | canopy_bottom_opening | 0.0002 | 0.0002 | 0.2% |
| 2 | 1 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0036 | 0.0035 | 4.1% |
| 2 | 1 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_hash_to_field | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_rs_encoding | 0.0048 | 0.0049 | 5.5% |
| 2 | 1 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0192 | 0.0166 | 22.0% |
| 2 | 1 | canopy_unbatched | outer_ood_evaluations | 0.0012 | 0.0012 | 1.4% |
| 2 | 1 | canopy_unbatched | outer_rs_encoding | 0.0278 | 0.0263 | 31.9% |
| 2 | 1 | canopy_unbatched | outer_second_oracle_whirs | 0.0274 | 0.0275 | 31.4% |
| 2 | 1 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.0296 | 0.0292 | 27.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0026 | 0.0026 | 2.4% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0001 | 0.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0033 | 0.0033 | 3.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0012 | 0.0011 | 1.1% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0211 | 0.0211 | 19.4% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0029 | 0.0029 | 2.7% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0192 | 0.0166 | 17.7% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0012 | 0.0012 | 1.1% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.0272 | 0.0267 | 25.1% |
| 0 | 2 | merkle_whir_independent | initial_merkle_tree | 0.0066 | 0.0067 | 4.2% |
| 0 | 2 | merkle_whir_independent | initial_ood_evaluations | 0.0025 | 0.0025 | 1.6% |
| 0 | 2 | merkle_whir_independent | initial_rs_encoding | 0.0557 | 0.0526 | 35.2% |
| 0 | 2 | merkle_whir_independent | whir_opening_proof_after_ood | 0.0932 | 0.0883 | 59.0% |
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0179 | 0.0174 | 12.4% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0338 | 0.0300 | 23.4% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0026 | 0.0025 | 1.8% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0066 | 0.0067 | 4.6% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0025 | 0.0025 | 1.7% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0557 | 0.0526 | 38.5% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0256 | 0.0253 | 17.7% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0295 | 0.0290 | 11.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0584 | 0.0467 | 22.4% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0026 | 0.0025 | 1.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0125 | 0.0121 | 4.8% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0026 | 0.0026 | 1.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.1079 | 0.1066 | 41.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0478 | 0.0464 | 18.3% |
| 2 | 2 | canopy_unbatched | canopy_bottom_forest | 0.0063 | 0.0059 | 3.6% |
| 2 | 2 | canopy_unbatched | canopy_bottom_opening | 0.0003 | 0.0003 | 0.2% |
| 2 | 2 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0071 | 0.0069 | 4.1% |
| 2 | 2 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_hash_to_field | 0.0001 | 0.0001 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_rs_encoding | 0.0096 | 0.0099 | 5.5% |
| 2 | 2 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0385 | 0.0333 | 22.0% |
| 2 | 2 | canopy_unbatched | outer_ood_evaluations | 0.0025 | 0.0025 | 1.4% |
| 2 | 2 | canopy_unbatched | outer_rs_encoding | 0.0557 | 0.0526 | 31.9% |
| 2 | 2 | canopy_unbatched | outer_second_oracle_whirs | 0.0548 | 0.0550 | 31.4% |
| 2 | 2 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.0345 | 0.0342 | 17.4% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0048 | 0.0047 | 2.4% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0007 | 0.0008 | 0.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0061 | 0.0061 | 3.1% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0024 | 0.0023 | 1.2% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0453 | 0.0452 | 22.8% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0052 | 0.0051 | 2.6% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0385 | 0.0333 | 19.3% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0025 | 0.0025 | 1.2% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.0588 | 0.0544 | 29.6% |
| 0 | 4 | merkle_whir_independent | initial_merkle_tree | 0.0132 | 0.0133 | 4.2% |
| 0 | 4 | merkle_whir_independent | initial_ood_evaluations | 0.0049 | 0.0049 | 1.6% |
| 0 | 4 | merkle_whir_independent | initial_rs_encoding | 0.1113 | 0.1051 | 35.2% |
| 0 | 4 | merkle_whir_independent | whir_opening_proof_after_ood | 0.1865 | 0.1765 | 59.0% |
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0245 | 0.0237 | 8.8% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0677 | 0.0600 | 24.3% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0055 | 0.0054 | 2.0% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0132 | 0.0133 | 4.8% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0049 | 0.0049 | 1.8% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 0.1113 | 0.1051 | 40.0% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0512 | 0.0506 | 18.4% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0369 | 0.0337 | 7.4% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.1169 | 0.0934 | 23.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0055 | 0.0054 | 1.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0250 | 0.0242 | 5.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0052 | 0.0052 | 1.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.2159 | 0.2132 | 43.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0956 | 0.0927 | 19.1% |
| 2 | 4 | canopy_unbatched | canopy_bottom_forest | 0.0125 | 0.0119 | 3.6% |
| 2 | 4 | canopy_unbatched | canopy_bottom_opening | 0.0007 | 0.0006 | 0.2% |
| 2 | 4 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0142 | 0.0138 | 4.1% |
| 2 | 4 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_hash_to_field | 0.0001 | 0.0001 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_rs_encoding | 0.0191 | 0.0198 | 5.5% |
| 2 | 4 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0769 | 0.0665 | 22.0% |
| 2 | 4 | canopy_unbatched | outer_ood_evaluations | 0.0049 | 0.0049 | 1.4% |
| 2 | 4 | canopy_unbatched | outer_rs_encoding | 0.1113 | 0.1051 | 31.9% |
| 2 | 4 | canopy_unbatched | outer_second_oracle_whirs | 0.1096 | 0.1100 | 31.4% |
| 2 | 4 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.0507 | 0.0505 | 13.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0101 | 0.0100 | 2.6% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0005 | 0.0005 | 0.1% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0140 | 0.0121 | 3.6% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0048 | 0.0045 | 1.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0949 | 0.0930 | 24.6% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0165 | 0.0162 | 4.3% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0769 | 0.0665 | 20.0% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0049 | 0.0049 | 1.3% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 0.1121 | 0.1113 | 29.1% |
| 0 | 8 | merkle_whir_independent | initial_merkle_tree | 0.0264 | 0.0267 | 4.2% |
| 0 | 8 | merkle_whir_independent | initial_ood_evaluations | 0.0098 | 0.0098 | 1.6% |
| 0 | 8 | merkle_whir_independent | initial_rs_encoding | 0.2226 | 0.2103 | 35.2% |
| 0 | 8 | merkle_whir_independent | whir_opening_proof_after_ood | 0.3730 | 0.3531 | 59.0% |
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0380 | 0.0381 | 6.9% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.1354 | 0.1199 | 24.6% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0156 | 0.0156 | 2.8% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0264 | 0.0267 | 4.8% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0098 | 0.0098 | 1.8% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 0.2226 | 0.2103 | 40.5% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1024 | 0.1012 | 18.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0504 | 0.0511 | 5.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.2338 | 0.1868 | 23.8% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0156 | 0.0156 | 1.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0500 | 0.0484 | 5.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0104 | 0.0105 | 1.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.4317 | 0.4264 | 43.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.1911 | 0.1854 | 19.4% |
| 2 | 8 | canopy_unbatched | canopy_bottom_forest | 0.0251 | 0.0237 | 3.6% |
| 2 | 8 | canopy_unbatched | canopy_bottom_opening | 0.0013 | 0.0013 | 0.2% |
| 2 | 8 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0284 | 0.0277 | 4.1% |
| 2 | 8 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_hash_to_field | 0.0003 | 0.0002 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_rs_encoding | 0.0382 | 0.0395 | 5.5% |
| 2 | 8 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.1539 | 0.1331 | 22.0% |
| 2 | 8 | canopy_unbatched | outer_ood_evaluations | 0.0098 | 0.0098 | 1.4% |
| 2 | 8 | canopy_unbatched | outer_rs_encoding | 0.2226 | 0.2103 | 31.9% |
| 2 | 8 | canopy_unbatched | outer_second_oracle_whirs | 0.2191 | 0.2200 | 31.4% |
| 2 | 8 | canopy_unbatched | terminal_canopy_claims | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 0.1153 | 0.1109 | 14.5% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0205 | 0.0197 | 2.6% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0008 | 0.0008 | 0.1% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0215 | 0.0218 | 2.7% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0098 | 0.0091 | 1.2% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1799 | 0.1784 | 22.5% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0628 | 0.0617 | 7.9% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.1539 | 0.1331 | 19.3% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0098 | 0.0098 | 1.2% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 0.2236 | 0.2185 | 28.0% |
