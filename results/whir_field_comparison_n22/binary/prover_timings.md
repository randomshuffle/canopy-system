# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Each input oracle has `2^22` field elements. The rate-1/2 rows use a first codeword of `2^23` elements, while the added rate-1/4 batched baseline uses `2^24` elements. All WHIR components are generated and verifier-checked at 128-bit security over the configured 256-bit field, in johnson mode, with no proof of work. Input generation, verification, and deallocation are excluded. Commitment time contains initial RS encoding and the initial Merkle/Canopy commitment; opening time contains OOD work, openings, recursive oracle commitments, Schwartz--Zippel preprocessing, and recursive WHIR. The selected initial fold is three: Experiment 3 therefore uses aligned Canopy height four, while Experiment 2 uses its independently optimized height `h=9` with its Canopy claim tail measured separately. To match the paper's at-most-`2^10` stopping rule, the reference implementation's native smaller suffix is separately executed and subtracted. Baseline and Experiment-2 independent stages are measured once per repetition and scaled by `t`. Experiment 3 executes its outer RS encodings, Canopy forests, H2F conversions, Canopy RS encodings, Merkle commitments, and bottom openings concurrently at the reported `t`; its transcript-dependent outer first transitions remain scaled independent work. Every batched recursive proof and generalized distinct-functional cross-evaluation stage is executed at the reported `t`.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs independent | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | 1/2 | 23 | - | 5 | 0.3525 | 0.4882 | 0.8407 | 0.7716 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 0.3525 | 0.4882 | 0.8407 | 0.7716 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 0.6466 | 0.6784 | 1.3250 | 1.3019 | 1.576x | 1.576x |
| 2 | 1 | canopy_unbatched | 1/2 | 23 | 9 | 5 | 0.5627 | 0.6328 | 1.1955 | 1.1201 | 1.422x | 1.422x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 0.6248 | 0.5343 | 1.1591 | 1.1044 | 1.379x | 1.379x |
| 0 | 2 | merkle_whir_independent | 1/2 | 23 | - | 5 | 0.7050 | 0.9764 | 1.6814 | 1.5432 | 1.000x | 1.060x |
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 0.7050 | 0.8811 | 1.5861 | 1.4513 | 0.943x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 1.2932 | 1.1702 | 2.4634 | 2.4116 | 1.465x | 1.553x |
| 2 | 2 | canopy_unbatched | 1/2 | 23 | 9 | 5 | 1.1253 | 1.2657 | 2.3910 | 2.2401 | 1.422x | 1.507x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 1.1788 | 0.8529 | 2.0317 | 1.9111 | 1.208x | 1.281x |
| 0 | 4 | merkle_whir_independent | 1/2 | 23 | - | 5 | 1.4100 | 1.9527 | 3.3627 | 3.0865 | 1.000x | 1.094x |
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 1.4100 | 1.6647 | 3.0747 | 2.7901 | 0.914x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 2.5864 | 2.1568 | 4.7432 | 4.6348 | 1.411x | 1.543x |
| 2 | 4 | canopy_unbatched | 1/2 | 23 | 9 | 5 | 2.2507 | 2.5313 | 4.7820 | 4.4802 | 1.422x | 1.555x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 2.3419 | 1.7418 | 4.0837 | 3.6309 | 1.214x | 1.328x |
| 0 | 8 | merkle_whir_independent | 1/2 | 23 | - | 5 | 2.8200 | 3.9055 | 6.7255 | 6.1730 | 1.000x | 1.048x |
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 2.8200 | 3.6001 | 6.4201 | 6.3702 | 0.955x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 5.1727 | 4.2960 | 9.4687 | 9.2799 | 1.408x | 1.475x |
| 2 | 8 | canopy_unbatched | 1/2 | 23 | 9 | 5 | 4.5014 | 5.0626 | 9.5640 | 8.9604 | 1.422x | 1.490x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 4.8108 | 3.7030 | 8.5138 | 7.6729 | 1.266x | 1.326x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | initial_merkle_tree | 0.0371 | 0.0368 | 4.4% |
| 0 | 1 | merkle_whir_independent | initial_ood_evaluations | 0.0128 | 0.0124 | 1.5% |
| 0 | 1 | merkle_whir_independent | initial_rs_encoding | 0.3154 | 0.3147 | 37.5% |
| 0 | 1 | merkle_whir_independent | whir_opening_proof_after_ood | 0.4753 | 0.4073 | 56.5% |
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.2171 | 0.1510 | 25.8% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0371 | 0.0368 | 4.4% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0128 | 0.0124 | 1.5% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.3154 | 0.3147 | 37.5% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.1320 | 0.1295 | 15.7% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0017 | 0.0016 | 0.2% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1245 | 0.1235 | 14.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.1847 | 0.1932 | 13.9% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0747 | 0.0722 | 5.6% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0131 | 0.0122 | 1.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.5718 | 0.5605 | 43.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.2188 | 0.2168 | 16.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0018 | 0.0016 | 0.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.2601 | 0.2532 | 19.6% |
| 2 | 1 | canopy_unbatched | canopy_bottom_forest | 0.0380 | 0.0365 | 3.2% |
| 2 | 1 | canopy_unbatched | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_claim_whirs | 0.1445 | 0.1418 | 12.1% |
| 2 | 1 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0402 | 0.0380 | 3.4% |
| 2 | 1 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_hash_to_field | 0.0008 | 0.0007 | 0.1% |
| 2 | 1 | canopy_unbatched | canopy_rs_encoding | 0.1684 | 0.1667 | 14.1% |
| 2 | 1 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.2576 | 0.1870 | 21.5% |
| 2 | 1 | canopy_unbatched | outer_ood_evaluations | 0.0128 | 0.0124 | 1.1% |
| 2 | 1 | canopy_unbatched | outer_rs_encoding | 0.3154 | 0.3147 | 26.4% |
| 2 | 1 | canopy_unbatched | outer_second_oracle_whirs | 0.2178 | 0.2203 | 18.2% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.2394 | 0.2377 | 20.7% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0314 | 0.0306 | 2.7% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0003 | 0.0004 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0409 | 0.0371 | 3.5% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0160 | 0.0155 | 1.4% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.2159 | 0.2136 | 18.6% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0242 | 0.0242 | 2.1% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.2576 | 0.1870 | 22.2% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0128 | 0.0124 | 1.1% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.3207 | 0.3219 | 27.7% |
| 0 | 2 | merkle_whir_independent | initial_merkle_tree | 0.0742 | 0.0736 | 4.4% |
| 0 | 2 | merkle_whir_independent | initial_ood_evaluations | 0.0257 | 0.0248 | 1.5% |
| 0 | 2 | merkle_whir_independent | initial_rs_encoding | 0.6308 | 0.6294 | 37.5% |
| 0 | 2 | merkle_whir_independent | whir_opening_proof_after_ood | 0.9507 | 0.8146 | 56.5% |
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.1476 | 0.1461 | 9.3% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.4342 | 0.3020 | 27.4% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0246 | 0.0246 | 1.6% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0742 | 0.0736 | 4.7% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0257 | 0.0248 | 1.6% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 0.6308 | 0.6294 | 39.8% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.2490 | 0.2470 | 15.7% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.2299 | 0.2286 | 9.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.3693 | 0.3864 | 15.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0246 | 0.0246 | 1.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.1495 | 0.1444 | 6.1% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0262 | 0.0245 | 1.1% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 1.1437 | 1.1209 | 46.4% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.5201 | 0.5064 | 21.1% |
| 2 | 2 | canopy_unbatched | canopy_bottom_forest | 0.0759 | 0.0730 | 3.2% |
| 2 | 2 | canopy_unbatched | canopy_bottom_opening | 0.0003 | 0.0003 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_claim_whirs | 0.2890 | 0.2836 | 12.1% |
| 2 | 2 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0803 | 0.0760 | 3.4% |
| 2 | 2 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_hash_to_field | 0.0015 | 0.0013 | 0.1% |
| 2 | 2 | canopy_unbatched | canopy_rs_encoding | 0.3368 | 0.3334 | 14.1% |
| 2 | 2 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.5151 | 0.3740 | 21.5% |
| 2 | 2 | canopy_unbatched | outer_ood_evaluations | 0.0257 | 0.0248 | 1.1% |
| 2 | 2 | canopy_unbatched | outer_rs_encoding | 0.6308 | 0.6294 | 26.4% |
| 2 | 2 | canopy_unbatched | outer_second_oracle_whirs | 0.4356 | 0.4406 | 18.2% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.2636 | 0.2626 | 13.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0618 | 0.0620 | 3.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0004 | 0.0004 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0696 | 0.0695 | 3.4% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0319 | 0.0313 | 1.6% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.4141 | 0.4159 | 20.4% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0480 | 0.0480 | 2.4% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.5151 | 0.3740 | 25.4% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0257 | 0.0248 | 1.3% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.6015 | 0.6044 | 29.6% |
| 0 | 4 | merkle_whir_independent | initial_merkle_tree | 0.1484 | 0.1473 | 4.4% |
| 0 | 4 | merkle_whir_independent | initial_ood_evaluations | 0.0514 | 0.0497 | 1.5% |
| 0 | 4 | merkle_whir_independent | initial_rs_encoding | 1.2616 | 1.2588 | 37.5% |
| 0 | 4 | merkle_whir_independent | whir_opening_proof_after_ood | 1.9014 | 1.6292 | 56.5% |
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.1962 | 0.1803 | 6.4% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.8683 | 0.6041 | 28.2% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0507 | 0.0489 | 1.7% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.1484 | 0.1473 | 4.8% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0514 | 0.0497 | 1.7% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 1.2616 | 1.2588 | 41.0% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.4981 | 0.4940 | 16.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.2748 | 0.2691 | 5.8% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.7387 | 0.7729 | 15.6% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0507 | 0.0489 | 1.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.2990 | 0.2888 | 6.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0524 | 0.0490 | 1.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 2.2874 | 2.2418 | 48.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 1.0402 | 1.0128 | 21.9% |
| 2 | 4 | canopy_unbatched | canopy_bottom_forest | 0.1519 | 0.1460 | 3.2% |
| 2 | 4 | canopy_unbatched | canopy_bottom_opening | 0.0007 | 0.0007 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_claim_whirs | 0.5779 | 0.5673 | 12.1% |
| 2 | 4 | canopy_unbatched | canopy_codeword_merkle_tree | 0.1606 | 0.1519 | 3.4% |
| 2 | 4 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_hash_to_field | 0.0031 | 0.0026 | 0.1% |
| 2 | 4 | canopy_unbatched | canopy_rs_encoding | 0.6736 | 0.6669 | 14.1% |
| 2 | 4 | canopy_unbatched | outer_initial_whir_stage_after_ood | 1.0302 | 0.7480 | 21.5% |
| 2 | 4 | canopy_unbatched | outer_ood_evaluations | 0.0514 | 0.0497 | 1.1% |
| 2 | 4 | canopy_unbatched | outer_rs_encoding | 1.2616 | 1.2588 | 26.4% |
| 2 | 4 | canopy_unbatched | outer_second_oracle_whirs | 0.8712 | 0.8812 | 18.2% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.4973 | 0.3732 | 12.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.1212 | 0.1198 | 3.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0006 | 0.0006 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.1411 | 0.1325 | 3.5% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0644 | 0.0625 | 1.6% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.8525 | 0.8380 | 20.9% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.1624 | 0.1581 | 4.0% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 1.0302 | 0.7480 | 25.2% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0514 | 0.0497 | 1.3% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 1.1627 | 1.1708 | 28.5% |
| 0 | 8 | merkle_whir_independent | initial_merkle_tree | 0.2968 | 0.2946 | 4.4% |
| 0 | 8 | merkle_whir_independent | initial_ood_evaluations | 0.1027 | 0.0994 | 1.5% |
| 0 | 8 | merkle_whir_independent | initial_rs_encoding | 2.5232 | 2.5177 | 37.5% |
| 0 | 8 | merkle_whir_independent | whir_opening_proof_after_ood | 3.8027 | 3.2583 | 56.5% |
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.6054 | 0.3251 | 9.4% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 1.7366 | 1.2082 | 27.0% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.1592 | 0.1582 | 2.5% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.2968 | 0.2946 | 4.6% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.1027 | 0.0994 | 1.6% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 2.5232 | 2.5177 | 39.3% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.9962 | 0.9880 | 15.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.4743 | 0.4788 | 5.0% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 1.4774 | 1.5458 | 15.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.1592 | 0.1582 | 1.7% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.5979 | 0.5777 | 6.3% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.1047 | 0.0980 | 1.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 4.5748 | 4.4837 | 48.3% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 2.0804 | 2.0257 | 22.0% |
| 2 | 8 | canopy_unbatched | canopy_bottom_forest | 0.3037 | 0.2920 | 3.2% |
| 2 | 8 | canopy_unbatched | canopy_bottom_opening | 0.0014 | 0.0013 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_claim_whirs | 1.1558 | 1.1345 | 12.1% |
| 2 | 8 | canopy_unbatched | canopy_codeword_merkle_tree | 0.3212 | 0.3039 | 3.4% |
| 2 | 8 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_hash_to_field | 0.0061 | 0.0053 | 0.1% |
| 2 | 8 | canopy_unbatched | canopy_rs_encoding | 1.3471 | 1.3338 | 14.1% |
| 2 | 8 | canopy_unbatched | outer_initial_whir_stage_after_ood | 2.0604 | 1.4959 | 21.5% |
| 2 | 8 | canopy_unbatched | outer_ood_evaluations | 0.1027 | 0.0994 | 1.1% |
| 2 | 8 | canopy_unbatched | outer_rs_encoding | 2.5232 | 2.5177 | 26.4% |
| 2 | 8 | canopy_unbatched | outer_second_oracle_whirs | 1.7423 | 1.7624 | 18.2% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 0.9432 | 0.9290 | 11.1% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.2326 | 0.2327 | 2.7% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0014 | 0.0009 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.2751 | 0.2686 | 3.2% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.1241 | 0.1243 | 1.5% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 1.8961 | 1.6548 | 22.3% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.5953 | 0.5952 | 7.0% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 2.0604 | 1.4959 | 24.2% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.1027 | 0.0994 | 1.2% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 2.2828 | 2.2924 | 26.8% |
