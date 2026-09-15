# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Each input oracle has `2^14` field elements. The rate-1/2 rows use a first codeword of `2^15` elements, while the added rate-1/4 batched baseline uses `2^16` elements. All WHIR components are generated and verifier-checked at 128-bit security over the configured 256-bit field, in johnson mode, with no proof of work. Input generation, verification, and deallocation are excluded. Commitment time contains initial RS encoding and the initial Merkle/Canopy commitment; opening time contains OOD work, openings, recursive oracle commitments, Schwartz--Zippel preprocessing, and recursive WHIR. The selected initial fold is three: Experiment 3 therefore uses aligned Canopy height four, while Experiment 2 uses its independently optimized height `h=14` with its Canopy claim tail measured separately. To match the paper's at-most-`2^10` stopping rule, the reference implementation's native smaller suffix is separately executed and subtracted. Baseline and Experiment-2 independent stages are measured once per repetition and scaled by `t`. Experiment 3 executes its outer RS encodings, Canopy forests, H2F conversions, Canopy RS encodings, Merkle commitments, and bottom openings concurrently at the reported `t`; its transcript-dependent outer first transitions remain scaled independent work. Every batched recursive proof and generalized distinct-functional cross-evaluation stage is executed at the reported `t`.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs independent | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | 1/2 | 15 | - | 1 | 0.0017 | 0.0050 | 0.0067 | 0.0067 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 15 | - | 1 | 0.0017 | 0.0050 | 0.0067 | 0.0067 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 16 | - | 1 | 0.0026 | 0.0049 | 0.0075 | 0.0075 | 1.121x | 1.121x |
| 2 | 1 | canopy_unbatched | 1/2 | 15 | 14 | 1 | 0.0027 | 0.0051 | 0.0078 | 0.0078 | 1.170x | 1.170x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 15 | 4 | 1 | 0.0024 | 0.0052 | 0.0076 | 0.0076 | 1.131x | 1.131x |
| 0 | 2 | merkle_whir_independent | 1/2 | 15 | - | 1 | 0.0034 | 0.0100 | 0.0134 | 0.0134 | 1.000x | 1.048x |
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 15 | - | 1 | 0.0034 | 0.0094 | 0.0128 | 0.0128 | 0.955x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 16 | - | 1 | 0.0053 | 0.0090 | 0.0143 | 0.0143 | 1.066x | 1.116x |
| 2 | 2 | canopy_unbatched | 1/2 | 15 | 14 | 1 | 0.0055 | 0.0102 | 0.0157 | 0.0157 | 1.170x | 1.226x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 15 | 4 | 1 | 0.0043 | 0.0097 | 0.0141 | 0.0141 | 1.049x | 1.099x |
| 0 | 4 | merkle_whir_independent | 1/2 | 15 | - | 1 | 0.0068 | 0.0200 | 0.0268 | 0.0268 | 1.000x | 1.069x |
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 15 | - | 1 | 0.0068 | 0.0182 | 0.0251 | 0.0251 | 0.935x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 16 | - | 1 | 0.0105 | 0.0169 | 0.0274 | 0.0274 | 1.023x | 1.094x |
| 2 | 4 | canopy_unbatched | 1/2 | 15 | 14 | 1 | 0.0110 | 0.0204 | 0.0314 | 0.0314 | 1.170x | 1.251x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 15 | 4 | 1 | 0.0063 | 0.0187 | 0.0250 | 0.0250 | 0.931x | 0.995x |
| 0 | 8 | merkle_whir_independent | 1/2 | 15 | - | 1 | 0.0137 | 0.0399 | 0.0536 | 0.0536 | 1.000x | 1.071x |
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 15 | - | 1 | 0.0137 | 0.0364 | 0.0501 | 0.0501 | 0.934x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 16 | - | 1 | 0.0210 | 0.0329 | 0.0539 | 0.0539 | 1.006x | 1.077x |
| 2 | 8 | canopy_unbatched | 1/2 | 15 | 14 | 1 | 0.0220 | 0.0408 | 0.0627 | 0.0627 | 1.170x | 1.253x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 15 | 4 | 1 | 0.0115 | 0.0382 | 0.0497 | 0.0497 | 0.926x | 0.992x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | initial_merkle_tree | 0.0004 | 0.0004 | 6.0% |
| 0 | 1 | merkle_whir_independent | initial_ood_evaluations | 0.0003 | 0.0003 | 3.9% |
| 0 | 1 | merkle_whir_independent | initial_rs_encoding | 0.0013 | 0.0013 | 19.6% |
| 0 | 1 | merkle_whir_independent | whir_opening_proof_after_ood | 0.0047 | 0.0047 | 70.5% |
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0033 | 0.0033 | 49.7% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0004 | 0.0004 | 6.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0003 | 0.0003 | 3.9% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0013 | 0.0013 | 19.6% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.0007 | 0.0007 | 10.7% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0000 | 0.0000 | 0.7% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0006 | 0.0006 | 9.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0025 | 0.0025 | 33.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0007 | 0.0007 | 9.7% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0002 | 0.0002 | 2.4% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.0019 | 0.0019 | 25.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.0011 | 0.0011 | 14.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0000 | 0.0000 | 0.6% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0011 | 0.0011 | 14.6% |
| 2 | 1 | canopy_unbatched | canopy_bottom_forest | 0.0005 | 0.0005 | 5.9% |
| 2 | 1 | canopy_unbatched | canopy_bottom_opening | 0.0001 | 0.0001 | 1.3% |
| 2 | 1 | canopy_unbatched | canopy_claim_whirs | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0005 | 0.0005 | 6.4% |
| 2 | 1 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_hash_to_field | 0.0000 | 0.0000 | 0.2% |
| 2 | 1 | canopy_unbatched | canopy_rs_encoding | 0.0005 | 0.0005 | 5.8% |
| 2 | 1 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0037 | 0.0037 | 47.1% |
| 2 | 1 | canopy_unbatched | outer_ood_evaluations | 0.0003 | 0.0003 | 3.4% |
| 2 | 1 | canopy_unbatched | outer_rs_encoding | 0.0013 | 0.0013 | 16.7% |
| 2 | 1 | canopy_unbatched | outer_second_oracle_whirs | 0.0010 | 0.0010 | 13.2% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.0011 | 0.0011 | 14.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0003 | 0.0003 | 3.4% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0000 | 0.0000 | 0.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0004 | 0.0004 | 5.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0001 | 0.0001 | 2.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0006 | 0.0006 | 8.4% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0001 | 0.0001 | 1.9% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0037 | 0.0037 | 48.7% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0003 | 0.0003 | 3.5% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.0009 | 0.0009 | 12.2% |
| 0 | 2 | merkle_whir_independent | initial_merkle_tree | 0.0008 | 0.0008 | 6.0% |
| 0 | 2 | merkle_whir_independent | initial_ood_evaluations | 0.0005 | 0.0005 | 3.9% |
| 0 | 2 | merkle_whir_independent | initial_rs_encoding | 0.0026 | 0.0026 | 19.6% |
| 0 | 2 | merkle_whir_independent | whir_opening_proof_after_ood | 0.0095 | 0.0095 | 70.5% |
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0008 | 0.0008 | 5.9% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0067 | 0.0067 | 52.1% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0002 | 0.0002 | 1.2% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0008 | 0.0008 | 6.2% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0005 | 0.0005 | 4.1% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0026 | 0.0026 | 20.5% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0013 | 0.0013 | 9.9% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0013 | 0.0013 | 9.4% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0050 | 0.0050 | 34.9% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0002 | 0.0002 | 1.1% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0015 | 0.0015 | 10.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0004 | 0.0004 | 2.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.0038 | 0.0038 | 26.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0022 | 0.0022 | 15.3% |
| 2 | 2 | canopy_unbatched | canopy_bottom_forest | 0.0009 | 0.0009 | 5.9% |
| 2 | 2 | canopy_unbatched | canopy_bottom_opening | 0.0002 | 0.0002 | 1.3% |
| 2 | 2 | canopy_unbatched | canopy_claim_whirs | 0.0000 | 0.0000 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0010 | 0.0010 | 6.4% |
| 2 | 2 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_hash_to_field | 0.0000 | 0.0000 | 0.2% |
| 2 | 2 | canopy_unbatched | canopy_rs_encoding | 0.0009 | 0.0009 | 5.8% |
| 2 | 2 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0074 | 0.0074 | 47.1% |
| 2 | 2 | canopy_unbatched | outer_ood_evaluations | 0.0005 | 0.0005 | 3.4% |
| 2 | 2 | canopy_unbatched | outer_rs_encoding | 0.0026 | 0.0026 | 16.7% |
| 2 | 2 | canopy_unbatched | outer_second_oracle_whirs | 0.0021 | 0.0021 | 13.2% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.0015 | 0.0015 | 10.5% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0004 | 0.0004 | 2.5% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.4% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0005 | 0.0005 | 3.9% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0002 | 0.0002 | 1.2% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0016 | 0.0016 | 11.3% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0003 | 0.0003 | 2.0% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0074 | 0.0074 | 52.5% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0005 | 0.0005 | 3.7% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.0017 | 0.0017 | 11.9% |
| 0 | 4 | merkle_whir_independent | initial_merkle_tree | 0.0016 | 0.0016 | 6.0% |
| 0 | 4 | merkle_whir_independent | initial_ood_evaluations | 0.0011 | 0.0011 | 3.9% |
| 0 | 4 | merkle_whir_independent | initial_rs_encoding | 0.0052 | 0.0052 | 19.6% |
| 0 | 4 | merkle_whir_independent | whir_opening_proof_after_ood | 0.0189 | 0.0189 | 70.5% |
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0011 | 0.0011 | 4.3% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0133 | 0.0133 | 53.2% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0002 | 0.0002 | 1.0% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0016 | 0.0016 | 6.4% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0011 | 0.0011 | 4.2% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0052 | 0.0052 | 20.9% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0025 | 0.0025 | 10.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0016 | 0.0016 | 5.9% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0100 | 0.0100 | 36.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0002 | 0.0002 | 0.9% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0029 | 0.0029 | 10.7% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0007 | 0.0007 | 2.6% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.0076 | 0.0076 | 27.6% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0044 | 0.0044 | 16.0% |
| 2 | 4 | canopy_unbatched | canopy_bottom_forest | 0.0018 | 0.0018 | 5.9% |
| 2 | 4 | canopy_unbatched | canopy_bottom_opening | 0.0004 | 0.0004 | 1.3% |
| 2 | 4 | canopy_unbatched | canopy_claim_whirs | 0.0000 | 0.0000 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0020 | 0.0020 | 6.4% |
| 2 | 4 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_hash_to_field | 0.0001 | 0.0001 | 0.2% |
| 2 | 4 | canopy_unbatched | canopy_rs_encoding | 0.0018 | 0.0018 | 5.8% |
| 2 | 4 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0148 | 0.0148 | 47.1% |
| 2 | 4 | canopy_unbatched | outer_ood_evaluations | 0.0011 | 0.0011 | 3.4% |
| 2 | 4 | canopy_unbatched | outer_rs_encoding | 0.0052 | 0.0052 | 16.7% |
| 2 | 4 | canopy_unbatched | outer_second_oracle_whirs | 0.0041 | 0.0041 | 13.2% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.0021 | 0.0021 | 8.4% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0005 | 0.0005 | 2.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0008 | 0.0008 | 3.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0003 | 0.0003 | 1.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0019 | 0.0019 | 7.6% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0007 | 0.0007 | 2.9% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0148 | 0.0148 | 59.2% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0011 | 0.0011 | 4.2% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 0.0028 | 0.0028 | 11.2% |
| 0 | 8 | merkle_whir_independent | initial_merkle_tree | 0.0032 | 0.0032 | 6.0% |
| 0 | 8 | merkle_whir_independent | initial_ood_evaluations | 0.0021 | 0.0021 | 3.9% |
| 0 | 8 | merkle_whir_independent | initial_rs_encoding | 0.0105 | 0.0105 | 19.6% |
| 0 | 8 | merkle_whir_independent | whir_opening_proof_after_ood | 0.0378 | 0.0378 | 70.5% |
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0019 | 0.0019 | 3.8% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0267 | 0.0267 | 53.2% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0007 | 0.0007 | 1.3% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0032 | 0.0032 | 6.4% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0021 | 0.0021 | 4.2% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0105 | 0.0105 | 20.9% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0051 | 0.0051 | 10.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0021 | 0.0021 | 3.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0199 | 0.0199 | 36.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0007 | 0.0007 | 1.2% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0058 | 0.0058 | 10.8% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0015 | 0.0015 | 2.7% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.0152 | 0.0152 | 28.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0088 | 0.0088 | 16.3% |
| 2 | 8 | canopy_unbatched | canopy_bottom_forest | 0.0037 | 0.0037 | 5.9% |
| 2 | 8 | canopy_unbatched | canopy_bottom_opening | 0.0008 | 0.0008 | 1.3% |
| 2 | 8 | canopy_unbatched | canopy_claim_whirs | 0.0000 | 0.0000 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0040 | 0.0040 | 6.4% |
| 2 | 8 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_hash_to_field | 0.0001 | 0.0001 | 0.2% |
| 2 | 8 | canopy_unbatched | canopy_rs_encoding | 0.0036 | 0.0036 | 5.8% |
| 2 | 8 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0295 | 0.0295 | 47.1% |
| 2 | 8 | canopy_unbatched | outer_ood_evaluations | 0.0021 | 0.0021 | 3.4% |
| 2 | 8 | canopy_unbatched | outer_rs_encoding | 0.0105 | 0.0105 | 16.7% |
| 2 | 8 | canopy_unbatched | outer_second_oracle_whirs | 0.0083 | 0.0083 | 13.2% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 0.0041 | 0.0041 | 8.2% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0009 | 0.0009 | 1.8% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.2% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0011 | 0.0011 | 2.2% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0006 | 0.0006 | 1.1% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0036 | 0.0036 | 7.2% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0023 | 0.0023 | 4.7% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0295 | 0.0295 | 59.5% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0021 | 0.0021 | 4.2% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 0.0054 | 0.0054 | 10.8% |
