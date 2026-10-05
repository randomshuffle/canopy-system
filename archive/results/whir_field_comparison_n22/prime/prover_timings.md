# Final Johnson-bound paper prover timings

Field: **Fp(2^256 - 43*2^32 + 1), multiplicative RS**. Inputs and evaluation points are deterministic full-width values.

Each input oracle has `2^22` field elements. The rate-1/2 rows use a first codeword of `2^23` elements, while the added rate-1/4 batched baseline uses `2^24` elements. All WHIR components are generated and verifier-checked at 128-bit security over the configured 256-bit field, in johnson mode, with no proof of work. Input generation, verification, and deallocation are excluded. Commitment time contains initial RS encoding and the initial Merkle/Canopy commitment; opening time contains OOD work, openings, recursive oracle commitments, Schwartz--Zippel preprocessing, and recursive WHIR. The selected initial fold is three: Experiment 3 therefore uses aligned Canopy height four, while Experiment 2 uses its independently optimized height `h=9` with its Canopy claim tail measured separately. To match the paper's at-most-`2^10` stopping rule, the reference implementation's native smaller suffix is separately executed and subtracted. Baseline and Experiment-2 independent stages are measured once per repetition and scaled by `t`. Experiment 3 executes its outer RS encodings, Canopy forests, H2F conversions, Canopy RS encodings, Merkle commitments, and bottom openings concurrently at the reported `t`; its transcript-dependent outer first transitions remain scaled independent work. Every batched recursive proof and generalized distinct-functional cross-evaluation stage is executed at the reported `t`.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs independent | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | 1/2 | 23 | - | 5 | 0.6273 | 0.7557 | 1.3829 | 1.3861 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 0.6273 | 0.7557 | 1.3829 | 1.3861 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 1.2409 | 1.2864 | 2.5273 | 2.5146 | 1.827x | 1.827x |
| 2 | 1 | canopy_unbatched | 1/2 | 23 | 9 | 5 | 1.2219 | 0.9901 | 2.2120 | 2.0273 | 1.599x | 1.599x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 1.2649 | 0.8758 | 2.1408 | 2.0604 | 1.548x | 1.548x |
| 0 | 2 | merkle_whir_independent | 1/2 | 23 | - | 5 | 1.2546 | 1.5113 | 2.7659 | 2.7723 | 1.000x | 1.054x |
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 1.2546 | 1.3704 | 2.6249 | 2.6537 | 0.949x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 2.4819 | 2.2890 | 4.7708 | 4.7569 | 1.725x | 1.818x |
| 2 | 2 | canopy_unbatched | 1/2 | 23 | 9 | 5 | 2.4438 | 1.9802 | 4.4240 | 4.0545 | 1.599x | 1.685x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 2.4486 | 1.3252 | 3.7738 | 3.5788 | 1.364x | 1.438x |
| 0 | 4 | merkle_whir_independent | 1/2 | 23 | - | 5 | 2.5091 | 3.0226 | 5.5318 | 5.5445 | 1.000x | 1.106x |
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 2.5091 | 2.4933 | 5.0024 | 5.0224 | 0.904x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 4.9638 | 4.1079 | 9.0717 | 8.9855 | 1.640x | 1.813x |
| 2 | 4 | canopy_unbatched | 1/2 | 23 | 9 | 5 | 4.8876 | 3.9604 | 8.8480 | 8.1091 | 1.599x | 1.769x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 4.7503 | 2.3133 | 7.0636 | 7.0413 | 1.277x | 1.412x |
| 0 | 8 | merkle_whir_independent | 1/2 | 23 | - | 5 | 5.0182 | 6.0453 | 11.0635 | 11.0890 | 1.000x | 1.098x |
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 5.0182 | 5.0580 | 10.0763 | 10.1637 | 0.911x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 9.9275 | 8.0544 | 17.9819 | 17.8536 | 1.625x | 1.785x |
| 2 | 8 | canopy_unbatched | 1/2 | 23 | 9 | 5 | 9.7752 | 7.9208 | 17.6959 | 16.2182 | 1.599x | 1.756x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 9.8367 | 5.0151 | 14.8518 | 14.9951 | 1.342x | 1.474x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | initial_merkle_tree | 0.0436 | 0.0434 | 3.2% |
| 0 | 1 | merkle_whir_independent | initial_ood_evaluations | 0.0179 | 0.0178 | 1.3% |
| 0 | 1 | merkle_whir_independent | initial_rs_encoding | 0.5837 | 0.5763 | 42.2% |
| 0 | 1 | merkle_whir_independent | whir_opening_proof_after_ood | 0.7378 | 0.7304 | 53.3% |
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.2344 | 0.2280 | 16.9% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0436 | 0.0434 | 3.2% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0179 | 0.0178 | 1.3% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.5837 | 0.5763 | 42.2% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.2490 | 0.2475 | 18.0% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0024 | 0.0024 | 0.2% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.2520 | 0.2479 | 18.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.3246 | 0.3293 | 12.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0861 | 0.0851 | 3.4% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0181 | 0.0179 | 0.7% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 1.1548 | 1.1489 | 45.7% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.4169 | 0.4086 | 16.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0026 | 0.0026 | 0.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.5241 | 0.5224 | 20.7% |
| 2 | 1 | canopy_unbatched | canopy_bottom_forest | 0.0428 | 0.0420 | 1.9% |
| 2 | 1 | canopy_unbatched | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_claim_whirs | 0.2343 | 0.2317 | 10.6% |
| 2 | 1 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0459 | 0.0453 | 2.1% |
| 2 | 1 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_hash_to_field | 0.0006 | 0.0006 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_rs_encoding | 0.5489 | 0.3602 | 24.8% |
| 2 | 1 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.2911 | 0.3042 | 13.2% |
| 2 | 1 | canopy_unbatched | outer_ood_evaluations | 0.0179 | 0.0178 | 0.8% |
| 2 | 1 | canopy_unbatched | outer_rs_encoding | 0.5837 | 0.5763 | 26.4% |
| 2 | 1 | canopy_unbatched | outer_second_oracle_whirs | 0.4467 | 0.4228 | 20.2% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.5186 | 0.5055 | 24.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0431 | 0.0397 | 2.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0464 | 0.0435 | 2.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0184 | 0.0182 | 0.9% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.5158 | 0.4924 | 24.1% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0480 | 0.0477 | 2.2% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.2911 | 0.3042 | 13.6% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0179 | 0.0178 | 0.8% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.6412 | 0.5952 | 30.0% |
| 0 | 2 | merkle_whir_independent | initial_merkle_tree | 0.0872 | 0.0868 | 3.2% |
| 0 | 2 | merkle_whir_independent | initial_ood_evaluations | 0.0357 | 0.0357 | 1.3% |
| 0 | 2 | merkle_whir_independent | initial_rs_encoding | 1.1674 | 1.1525 | 42.2% |
| 0 | 2 | merkle_whir_independent | whir_opening_proof_after_ood | 1.4756 | 1.4608 | 53.3% |
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.3143 | 0.3231 | 12.0% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.4687 | 0.4559 | 17.9% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0475 | 0.0467 | 1.8% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0872 | 0.0868 | 3.3% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0357 | 0.0357 | 1.4% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 1.1674 | 1.1525 | 44.5% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.5041 | 0.4957 | 19.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.5078 | 0.4855 | 10.6% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.6493 | 0.6587 | 13.6% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0475 | 0.0467 | 1.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.1722 | 0.1702 | 3.6% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0363 | 0.0358 | 0.8% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 2.3097 | 2.2978 | 48.4% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 1.0481 | 1.0448 | 22.0% |
| 2 | 2 | canopy_unbatched | canopy_bottom_forest | 0.0856 | 0.0840 | 1.9% |
| 2 | 2 | canopy_unbatched | canopy_bottom_opening | 0.0003 | 0.0003 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_claim_whirs | 0.4685 | 0.4635 | 10.6% |
| 2 | 2 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0917 | 0.0906 | 2.1% |
| 2 | 2 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_hash_to_field | 0.0012 | 0.0013 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_rs_encoding | 1.0979 | 0.7204 | 24.8% |
| 2 | 2 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.5822 | 0.6084 | 13.2% |
| 2 | 2 | canopy_unbatched | outer_ood_evaluations | 0.0357 | 0.0357 | 0.8% |
| 2 | 2 | canopy_unbatched | outer_rs_encoding | 1.1674 | 1.1525 | 26.4% |
| 2 | 2 | canopy_unbatched | outer_second_oracle_whirs | 0.8933 | 0.8456 | 20.2% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.6150 | 0.6083 | 16.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0837 | 0.0821 | 2.2% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0003 | 0.0002 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0909 | 0.0882 | 2.4% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0396 | 0.0383 | 1.1% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.9971 | 0.9553 | 26.4% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0920 | 0.0894 | 2.4% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.5822 | 0.6084 | 15.4% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0357 | 0.0357 | 0.9% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 1.2373 | 1.2165 | 32.8% |
| 0 | 4 | merkle_whir_independent | initial_merkle_tree | 0.1743 | 0.1736 | 3.2% |
| 0 | 4 | merkle_whir_independent | initial_ood_evaluations | 0.0715 | 0.0713 | 1.3% |
| 0 | 4 | merkle_whir_independent | initial_rs_encoding | 2.3348 | 2.3051 | 42.2% |
| 0 | 4 | merkle_whir_independent | whir_opening_proof_after_ood | 2.9512 | 2.9216 | 53.3% |
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.3848 | 0.3725 | 7.7% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.9375 | 0.9119 | 18.7% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0914 | 0.0868 | 1.8% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.1743 | 0.1736 | 3.5% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0715 | 0.0713 | 1.4% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 2.3348 | 2.3051 | 46.7% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 1.0081 | 0.9914 | 20.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.5492 | 0.5203 | 6.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 1.2985 | 1.3174 | 14.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0914 | 0.0868 | 1.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.3444 | 0.3404 | 3.8% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0725 | 0.0716 | 0.8% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 4.6194 | 4.5955 | 50.9% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 2.0963 | 2.0895 | 23.1% |
| 2 | 4 | canopy_unbatched | canopy_bottom_forest | 0.1711 | 0.1681 | 1.9% |
| 2 | 4 | canopy_unbatched | canopy_bottom_opening | 0.0007 | 0.0006 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_claim_whirs | 0.9371 | 0.9270 | 10.6% |
| 2 | 4 | canopy_unbatched | canopy_codeword_merkle_tree | 0.1835 | 0.1812 | 2.1% |
| 2 | 4 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_hash_to_field | 0.0025 | 0.0025 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_rs_encoding | 2.1957 | 1.4408 | 24.8% |
| 2 | 4 | canopy_unbatched | outer_initial_whir_stage_after_ood | 1.1645 | 1.2168 | 13.2% |
| 2 | 4 | canopy_unbatched | outer_ood_evaluations | 0.0715 | 0.0713 | 0.8% |
| 2 | 4 | canopy_unbatched | outer_rs_encoding | 2.3348 | 2.3051 | 26.4% |
| 2 | 4 | canopy_unbatched | outer_second_oracle_whirs | 1.7867 | 1.6911 | 20.2% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.7953 | 0.8218 | 11.3% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.1579 | 0.1576 | 2.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0004 | 0.0004 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.1730 | 0.1750 | 2.4% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0753 | 0.0765 | 1.1% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 1.8701 | 1.8638 | 26.5% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.2816 | 0.2819 | 4.0% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 1.1645 | 1.2168 | 16.5% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0715 | 0.0713 | 1.0% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 2.4740 | 2.4187 | 35.0% |
| 0 | 8 | merkle_whir_independent | initial_merkle_tree | 0.3487 | 0.3473 | 3.2% |
| 0 | 8 | merkle_whir_independent | initial_ood_evaluations | 0.1430 | 0.1427 | 1.3% |
| 0 | 8 | merkle_whir_independent | initial_rs_encoding | 4.6696 | 4.6102 | 42.2% |
| 0 | 8 | merkle_whir_independent | whir_opening_proof_after_ood | 5.9023 | 5.8433 | 53.3% |
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.7442 | 0.6406 | 7.4% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 1.8749 | 1.8238 | 18.6% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.2797 | 0.2808 | 2.8% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.3487 | 0.3473 | 3.5% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.1430 | 0.1427 | 1.4% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 4.6696 | 4.6102 | 46.3% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 2.0162 | 1.9829 | 20.0% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.8401 | 0.7867 | 4.7% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 2.5970 | 2.6347 | 14.4% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.2797 | 0.2808 | 1.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.6888 | 0.6807 | 3.8% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.1450 | 0.1432 | 0.8% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 9.2387 | 9.1910 | 51.4% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 4.1925 | 4.1791 | 23.3% |
| 2 | 8 | canopy_unbatched | canopy_bottom_forest | 0.3422 | 0.3361 | 1.9% |
| 2 | 8 | canopy_unbatched | canopy_bottom_opening | 0.0013 | 0.0012 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_claim_whirs | 1.8742 | 1.8539 | 10.6% |
| 2 | 8 | canopy_unbatched | canopy_codeword_merkle_tree | 0.3670 | 0.3625 | 2.1% |
| 2 | 8 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_hash_to_field | 0.0050 | 0.0051 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_rs_encoding | 4.3914 | 2.8816 | 24.8% |
| 2 | 8 | canopy_unbatched | outer_initial_whir_stage_after_ood | 2.3290 | 2.4335 | 13.2% |
| 2 | 8 | canopy_unbatched | outer_ood_evaluations | 0.1430 | 0.1427 | 0.8% |
| 2 | 8 | canopy_unbatched | outer_rs_encoding | 4.6696 | 4.6102 | 26.4% |
| 2 | 8 | canopy_unbatched | outer_second_oracle_whirs | 3.5734 | 3.3823 | 20.2% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 1.4963 | 1.5221 | 10.1% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.3037 | 0.3025 | 2.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0037 | 0.0042 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.3514 | 0.3430 | 2.4% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.1404 | 0.1395 | 0.9% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 4.6351 | 4.7789 | 31.2% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 1.0432 | 1.0274 | 7.0% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 2.3290 | 2.4335 | 15.7% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.1430 | 0.1427 | 1.0% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 4.4061 | 4.4597 | 29.7% |
