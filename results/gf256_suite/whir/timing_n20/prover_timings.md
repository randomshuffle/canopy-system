# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Each input oracle has `2^20` field elements. The rate-1/2 rows use a first codeword of `2^21` elements, while the added rate-1/4 batched baseline uses `2^22` elements. All WHIR components are generated and verifier-checked at 128-bit security over the configured 256-bit field, in johnson mode, with no proof of work. Input generation, verification, and deallocation are excluded. Commitment time contains initial RS encoding and the initial Merkle/Canopy commitment; opening time contains OOD work, openings, recursive oracle commitments, Schwartz--Zippel preprocessing, and recursive WHIR. The selected initial fold is three: Experiment 3 therefore uses aligned Canopy height four, while Experiment 2 uses its independently optimized height `h=7` with its Canopy claim tail measured separately. To match the paper's at-most-`2^10` stopping rule, the reference implementation's native smaller suffix is separately executed and subtracted. Baseline and Experiment-2 independent stages are measured once per repetition and scaled by `t`. Experiment 3 executes its outer RS encodings, Canopy forests, H2F conversions, Canopy RS encodings, Merkle commitments, and bottom openings concurrently at the reported `t`; its transcript-dependent outer first transitions remain scaled independent work. Every batched recursive proof and generalized distinct-functional cross-evaluation stage is executed at the reported `t`.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs independent | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | 1/2 | 21 | - | 5 | 0.0847 | 0.1217 | 0.2064 | 0.2071 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_independent_rate_quarter | 1/4 | 22 | - | 5 | 0.1547 | 0.1832 | 0.3379 | 0.3369 | 1.637x | 1.637x |
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 21 | - | 5 | 0.0847 | 0.1217 | 0.2064 | 0.2071 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 22 | - | 5 | 0.1547 | 0.1832 | 0.3379 | 0.3369 | 1.637x | 1.637x |
| 2 | 1 | canopy_unbatched | 1/2 | 21 | 7 | 5 | 0.1396 | 0.1649 | 0.3046 | 0.3046 | 1.476x | 1.476x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 21 | 4 | 5 | 0.1532 | 0.1318 | 0.2850 | 0.2859 | 1.381x | 1.381x |
| 0 | 2 | merkle_whir_independent | 1/2 | 21 | - | 5 | 0.1693 | 0.2435 | 0.4128 | 0.4143 | 1.000x | 1.073x |
| 0 | 2 | merkle_whir_independent_rate_quarter | 1/4 | 22 | - | 5 | 0.3095 | 0.3664 | 0.6759 | 0.6739 | 1.637x | 1.756x |
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 21 | - | 5 | 0.1693 | 0.2155 | 0.3848 | 0.3902 | 0.932x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 22 | - | 5 | 0.3095 | 0.3119 | 0.6214 | 0.6272 | 1.505x | 1.615x |
| 2 | 2 | canopy_unbatched | 1/2 | 21 | 7 | 5 | 0.2792 | 0.3299 | 0.6091 | 0.6092 | 1.476x | 1.583x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 21 | 4 | 5 | 0.2883 | 0.2094 | 0.4976 | 0.4973 | 1.206x | 1.293x |
| 0 | 4 | merkle_whir_independent | 1/2 | 21 | - | 5 | 0.3386 | 0.4869 | 0.8256 | 0.8286 | 1.000x | 1.129x |
| 0 | 4 | merkle_whir_independent_rate_quarter | 1/4 | 22 | - | 5 | 0.6190 | 0.7328 | 1.3517 | 1.3478 | 1.637x | 1.849x |
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 21 | - | 5 | 0.3386 | 0.3925 | 0.7311 | 0.7320 | 0.886x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 22 | - | 5 | 0.6190 | 0.5683 | 1.1872 | 1.1939 | 1.438x | 1.624x |
| 2 | 4 | canopy_unbatched | 1/2 | 21 | 7 | 5 | 0.5584 | 0.6598 | 1.2182 | 1.2184 | 1.476x | 1.666x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 21 | 4 | 5 | 0.5656 | 0.3792 | 0.9448 | 0.9510 | 1.144x | 1.292x |
| 0 | 8 | merkle_whir_independent | 1/2 | 21 | - | 5 | 0.6773 | 0.9738 | 1.6511 | 1.6572 | 1.000x | 1.124x |
| 0 | 8 | merkle_whir_independent_rate_quarter | 1/4 | 22 | - | 5 | 1.2379 | 1.4655 | 2.7034 | 2.6956 | 1.637x | 1.840x |
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 21 | - | 5 | 0.6773 | 0.7921 | 1.4693 | 1.4656 | 0.890x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 22 | - | 5 | 1.2379 | 1.1249 | 2.3628 | 2.3837 | 1.431x | 1.608x |
| 2 | 8 | canopy_unbatched | 1/2 | 21 | 7 | 5 | 1.1169 | 1.3196 | 2.4364 | 2.4368 | 1.476x | 1.658x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 21 | 4 | 5 | 1.2248 | 0.8575 | 2.0824 | 1.9855 | 1.261x | 1.417x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | initial_merkle_tree | 0.0108 | 0.0104 | 5.2% |
| 0 | 1 | merkle_whir_independent | initial_ood_evaluations | 0.0039 | 0.0034 | 1.9% |
| 0 | 1 | merkle_whir_independent | initial_rs_encoding | 0.0739 | 0.0739 | 35.8% |
| 0 | 1 | merkle_whir_independent | whir_opening_proof_after_ood | 0.1178 | 0.1192 | 57.1% |
| 0 | 1 | merkle_whir_independent_rate_quarter | initial_merkle_tree | 0.0204 | 0.0208 | 6.0% |
| 0 | 1 | merkle_whir_independent_rate_quarter | initial_ood_evaluations | 0.0033 | 0.0031 | 1.0% |
| 0 | 1 | merkle_whir_independent_rate_quarter | initial_rs_encoding | 0.1343 | 0.1345 | 39.7% |
| 0 | 1 | merkle_whir_independent_rate_quarter | whir_opening_proof_after_ood | 0.1799 | 0.1783 | 53.2% |
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0469 | 0.0490 | 22.7% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0108 | 0.0104 | 5.2% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0039 | 0.0034 | 1.9% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0739 | 0.0739 | 35.8% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.0389 | 0.0389 | 18.8% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0007 | 0.0006 | 0.3% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0315 | 0.0316 | 15.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0545 | 0.0564 | 16.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0204 | 0.0208 | 6.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0033 | 0.0031 | 1.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.1343 | 0.1345 | 39.7% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.0625 | 0.0619 | 18.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0005 | 0.0005 | 0.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0624 | 0.0626 | 18.5% |
| 2 | 1 | canopy_unbatched | canopy_bottom_forest | 0.0094 | 0.0088 | 3.1% |
| 2 | 1 | canopy_unbatched | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_claim_whirs | 0.0431 | 0.0422 | 14.2% |
| 2 | 1 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0130 | 0.0131 | 4.3% |
| 2 | 1 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_hash_to_field | 0.0006 | 0.0005 | 0.2% |
| 2 | 1 | canopy_unbatched | canopy_rs_encoding | 0.0427 | 0.0429 | 14.0% |
| 2 | 1 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0537 | 0.0543 | 17.6% |
| 2 | 1 | canopy_unbatched | outer_ood_evaluations | 0.0039 | 0.0034 | 1.3% |
| 2 | 1 | canopy_unbatched | outer_rs_encoding | 0.0739 | 0.0739 | 24.3% |
| 2 | 1 | canopy_unbatched | outer_second_oracle_whirs | 0.0641 | 0.0645 | 21.1% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.0664 | 0.0657 | 23.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0084 | 0.0077 | 3.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0006 | 0.0002 | 0.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0108 | 0.0111 | 3.8% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0041 | 0.0040 | 1.4% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0523 | 0.0515 | 18.3% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0072 | 0.0066 | 2.5% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0537 | 0.0543 | 18.8% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0039 | 0.0034 | 1.4% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.0776 | 0.0772 | 27.2% |
| 0 | 2 | merkle_whir_independent | initial_merkle_tree | 0.0215 | 0.0209 | 5.2% |
| 0 | 2 | merkle_whir_independent | initial_ood_evaluations | 0.0078 | 0.0068 | 1.9% |
| 0 | 2 | merkle_whir_independent | initial_rs_encoding | 0.1478 | 0.1479 | 35.8% |
| 0 | 2 | merkle_whir_independent | whir_opening_proof_after_ood | 0.2356 | 0.2384 | 57.1% |
| 0 | 2 | merkle_whir_independent_rate_quarter | initial_merkle_tree | 0.0408 | 0.0415 | 6.0% |
| 0 | 2 | merkle_whir_independent_rate_quarter | initial_ood_evaluations | 0.0067 | 0.0063 | 1.0% |
| 0 | 2 | merkle_whir_independent_rate_quarter | initial_rs_encoding | 0.2686 | 0.2690 | 39.7% |
| 0 | 2 | merkle_whir_independent_rate_quarter | whir_opening_proof_after_ood | 0.3597 | 0.3565 | 53.2% |
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0442 | 0.0436 | 11.5% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0937 | 0.0979 | 24.4% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0069 | 0.0070 | 1.8% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0215 | 0.0209 | 5.6% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0078 | 0.0068 | 2.0% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 0.1478 | 0.1479 | 38.4% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0629 | 0.0631 | 16.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0646 | 0.0644 | 10.4% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.1090 | 0.1127 | 17.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0069 | 0.0070 | 1.1% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0408 | 0.0415 | 6.6% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0067 | 0.0063 | 1.1% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.2686 | 0.2690 | 43.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.1248 | 0.1252 | 20.1% |
| 2 | 2 | canopy_unbatched | canopy_bottom_forest | 0.0188 | 0.0177 | 3.1% |
| 2 | 2 | canopy_unbatched | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_claim_whirs | 0.0862 | 0.0844 | 14.2% |
| 2 | 2 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0260 | 0.0261 | 4.3% |
| 2 | 2 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_hash_to_field | 0.0012 | 0.0011 | 0.2% |
| 2 | 2 | canopy_unbatched | canopy_rs_encoding | 0.0855 | 0.0858 | 14.0% |
| 2 | 2 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.1074 | 0.1085 | 17.6% |
| 2 | 2 | canopy_unbatched | outer_ood_evaluations | 0.0078 | 0.0068 | 1.3% |
| 2 | 2 | canopy_unbatched | outer_rs_encoding | 0.1478 | 0.1479 | 24.3% |
| 2 | 2 | canopy_unbatched | outer_second_oracle_whirs | 0.1283 | 0.1289 | 21.1% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.0792 | 0.0768 | 15.9% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0162 | 0.0155 | 3.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0010 | 0.0010 | 0.2% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0206 | 0.0206 | 4.1% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0084 | 0.0081 | 1.7% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0992 | 0.0997 | 19.9% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0140 | 0.0142 | 2.8% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.1074 | 0.1085 | 21.6% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0078 | 0.0068 | 1.6% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.1438 | 0.1434 | 28.9% |
| 0 | 4 | merkle_whir_independent | initial_merkle_tree | 0.0430 | 0.0417 | 5.2% |
| 0 | 4 | merkle_whir_independent | initial_ood_evaluations | 0.0156 | 0.0136 | 1.9% |
| 0 | 4 | merkle_whir_independent | initial_rs_encoding | 0.2956 | 0.2958 | 35.8% |
| 0 | 4 | merkle_whir_independent | whir_opening_proof_after_ood | 0.4713 | 0.4768 | 57.1% |
| 0 | 4 | merkle_whir_independent_rate_quarter | initial_merkle_tree | 0.0817 | 0.0830 | 6.0% |
| 0 | 4 | merkle_whir_independent_rate_quarter | initial_ood_evaluations | 0.0133 | 0.0126 | 1.0% |
| 0 | 4 | merkle_whir_independent_rate_quarter | initial_rs_encoding | 0.5373 | 0.5380 | 39.7% |
| 0 | 4 | merkle_whir_independent_rate_quarter | whir_opening_proof_after_ood | 0.7194 | 0.7131 | 53.2% |
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0500 | 0.0494 | 6.8% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.1875 | 0.1958 | 25.6% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0135 | 0.0130 | 1.9% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0430 | 0.0417 | 5.9% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0156 | 0.0136 | 2.1% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 0.2956 | 0.2958 | 40.4% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1258 | 0.1263 | 17.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0739 | 0.0749 | 6.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.2180 | 0.2255 | 18.4% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0135 | 0.0130 | 1.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0817 | 0.0830 | 6.9% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0133 | 0.0126 | 1.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.5373 | 0.5380 | 45.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.2496 | 0.2504 | 21.0% |
| 2 | 4 | canopy_unbatched | canopy_bottom_forest | 0.0376 | 0.0353 | 3.1% |
| 2 | 4 | canopy_unbatched | canopy_bottom_opening | 0.0005 | 0.0005 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_claim_whirs | 0.1724 | 0.1687 | 14.2% |
| 2 | 4 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0520 | 0.0522 | 4.3% |
| 2 | 4 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_hash_to_field | 0.0023 | 0.0022 | 0.2% |
| 2 | 4 | canopy_unbatched | canopy_rs_encoding | 0.1710 | 0.1715 | 14.0% |
| 2 | 4 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.2147 | 0.2170 | 17.6% |
| 2 | 4 | canopy_unbatched | outer_ood_evaluations | 0.0156 | 0.0136 | 1.3% |
| 2 | 4 | canopy_unbatched | outer_rs_encoding | 0.2956 | 0.2958 | 24.3% |
| 2 | 4 | canopy_unbatched | outer_second_oracle_whirs | 0.2566 | 0.2579 | 21.1% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.1028 | 0.1031 | 10.9% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0320 | 0.0318 | 3.4% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0010 | 0.0011 | 0.1% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0394 | 0.0384 | 4.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0173 | 0.0173 | 1.8% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1945 | 0.1949 | 20.6% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0451 | 0.0452 | 4.8% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.2147 | 0.2170 | 22.7% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0156 | 0.0136 | 1.7% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 0.2824 | 0.2814 | 29.9% |
| 0 | 8 | merkle_whir_independent | initial_merkle_tree | 0.0861 | 0.0835 | 5.2% |
| 0 | 8 | merkle_whir_independent | initial_ood_evaluations | 0.0313 | 0.0272 | 1.9% |
| 0 | 8 | merkle_whir_independent | initial_rs_encoding | 0.5912 | 0.5915 | 35.8% |
| 0 | 8 | merkle_whir_independent | whir_opening_proof_after_ood | 0.9426 | 0.9535 | 57.1% |
| 0 | 8 | merkle_whir_independent_rate_quarter | initial_merkle_tree | 0.1633 | 0.1660 | 6.0% |
| 0 | 8 | merkle_whir_independent_rate_quarter | initial_ood_evaluations | 0.0266 | 0.0251 | 1.0% |
| 0 | 8 | merkle_whir_independent_rate_quarter | initial_rs_encoding | 1.0746 | 1.0760 | 39.7% |
| 0 | 8 | merkle_whir_independent_rate_quarter | whir_opening_proof_after_ood | 1.4389 | 1.4262 | 53.2% |
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0830 | 0.0816 | 5.6% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.3749 | 0.3917 | 25.5% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0513 | 0.0490 | 3.5% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0861 | 0.0835 | 5.9% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0313 | 0.0272 | 2.1% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 0.5912 | 0.5915 | 40.2% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.2516 | 0.2525 | 17.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.1119 | 0.1089 | 4.7% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.4360 | 0.4510 | 18.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0513 | 0.0490 | 2.2% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.1633 | 0.1660 | 6.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0266 | 0.0251 | 1.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 1.0746 | 1.0760 | 45.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.4991 | 0.5007 | 21.1% |
| 2 | 8 | canopy_unbatched | canopy_bottom_forest | 0.0751 | 0.0706 | 3.1% |
| 2 | 8 | canopy_unbatched | canopy_bottom_opening | 0.0009 | 0.0009 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_claim_whirs | 0.3448 | 0.3375 | 14.2% |
| 2 | 8 | canopy_unbatched | canopy_codeword_merkle_tree | 0.1040 | 0.1044 | 4.3% |
| 2 | 8 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_hash_to_field | 0.0046 | 0.0044 | 0.2% |
| 2 | 8 | canopy_unbatched | canopy_rs_encoding | 0.3420 | 0.3431 | 14.0% |
| 2 | 8 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.4295 | 0.4341 | 17.6% |
| 2 | 8 | canopy_unbatched | outer_ood_evaluations | 0.0313 | 0.0272 | 1.3% |
| 2 | 8 | canopy_unbatched | outer_rs_encoding | 0.5912 | 0.5915 | 24.3% |
| 2 | 8 | canopy_unbatched | outer_second_oracle_whirs | 0.5131 | 0.5157 | 21.1% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 0.2216 | 0.2163 | 10.6% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0672 | 0.0643 | 3.2% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0035 | 0.0016 | 0.2% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0879 | 0.0764 | 4.2% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0365 | 0.0344 | 1.8% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 0.4728 | 0.3913 | 22.7% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.1717 | 0.1668 | 8.2% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.4295 | 0.4341 | 20.6% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0313 | 0.0272 | 1.5% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 0.5605 | 0.5609 | 26.9% |
