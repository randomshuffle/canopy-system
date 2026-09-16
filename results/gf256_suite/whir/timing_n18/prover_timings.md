# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Each input oracle has `2^18` field elements. The rate-1/2 rows use a first codeword of `2^19` elements, while the added rate-1/4 batched baseline uses `2^20` elements. All WHIR components are generated and verifier-checked at 128-bit security over the configured 256-bit field, in johnson mode, with no proof of work. Input generation, verification, and deallocation are excluded. Commitment time contains initial RS encoding and the initial Merkle/Canopy commitment; opening time contains OOD work, openings, recursive oracle commitments, Schwartz--Zippel preprocessing, and recursive WHIR. The selected initial fold is three: Experiment 3 therefore uses aligned Canopy height four, while Experiment 2 uses its independently optimized height `h=18` with its Canopy claim tail measured separately. To match the paper's at-most-`2^10` stopping rule, the reference implementation's native smaller suffix is separately executed and subtracted. Baseline and Experiment-2 independent stages are measured once per repetition and scaled by `t`. Experiment 3 executes its outer RS encodings, Canopy forests, H2F conversions, Canopy RS encodings, Merkle commitments, and bottom openings concurrently at the reported `t`; its transcript-dependent outer first transitions remain scaled independent work. Every batched recursive proof and generalized distinct-functional cross-evaluation stage is executed at the reported `t`.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs independent | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.0207 | 0.0315 | 0.0522 | 0.0527 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_independent_rate_quarter | 1/4 | 20 | - | 5 | 0.0354 | 0.0471 | 0.0825 | 0.0817 | 1.581x | 1.581x |
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0207 | 0.0315 | 0.0522 | 0.0527 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.0354 | 0.0471 | 0.0825 | 0.0817 | 1.581x | 1.581x |
| 2 | 1 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.0281 | 0.0317 | 0.0597 | 0.0613 | 1.145x | 1.145x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.0366 | 0.0342 | 0.0708 | 0.0721 | 1.357x | 1.357x |
| 0 | 2 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.0414 | 0.0630 | 0.1044 | 0.1054 | 1.000x | 1.091x |
| 0 | 2 | merkle_whir_independent_rate_quarter | 1/4 | 20 | - | 5 | 0.0708 | 0.0943 | 0.1650 | 0.1633 | 1.581x | 1.725x |
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0414 | 0.0542 | 0.0956 | 0.0970 | 0.916x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.0708 | 0.0774 | 0.1482 | 0.1467 | 1.420x | 1.549x |
| 2 | 2 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.0562 | 0.0633 | 0.1195 | 0.1226 | 1.145x | 1.250x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.0690 | 0.0512 | 0.1202 | 0.1229 | 1.152x | 1.257x |
| 0 | 4 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.0828 | 0.1259 | 0.2088 | 0.2109 | 1.000x | 1.154x |
| 0 | 4 | merkle_whir_independent_rate_quarter | 1/4 | 20 | - | 5 | 0.1415 | 0.1885 | 0.3300 | 0.3267 | 1.581x | 1.824x |
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.0828 | 0.0981 | 0.1809 | 0.1847 | 0.867x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.1415 | 0.1394 | 0.2809 | 0.2770 | 1.346x | 1.553x |
| 2 | 4 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.1124 | 0.1266 | 0.2390 | 0.2451 | 1.145x | 1.321x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.1299 | 0.0948 | 0.2247 | 0.2240 | 1.076x | 1.242x |
| 0 | 8 | merkle_whir_independent | 1/2 | 19 | - | 5 | 0.1657 | 0.2518 | 0.4175 | 0.4217 | 1.000x | 1.158x |
| 0 | 8 | merkle_whir_independent_rate_quarter | 1/4 | 20 | - | 5 | 0.2830 | 0.3770 | 0.6600 | 0.6533 | 1.581x | 1.831x |
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 19 | - | 5 | 0.1657 | 0.1948 | 0.3604 | 0.3672 | 0.863x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 20 | - | 5 | 0.2830 | 0.2685 | 0.5515 | 0.5414 | 1.321x | 1.530x |
| 2 | 8 | canopy_unbatched | 1/2 | 19 | 18 | 5 | 0.2248 | 0.2532 | 0.4780 | 0.4903 | 1.145x | 1.326x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 19 | 4 | 5 | 0.2467 | 0.1996 | 0.4462 | 0.4537 | 1.069x | 1.238x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | initial_merkle_tree | 0.0030 | 0.0029 | 5.8% |
| 0 | 1 | merkle_whir_independent | initial_ood_evaluations | 0.0010 | 0.0009 | 2.0% |
| 0 | 1 | merkle_whir_independent | initial_rs_encoding | 0.0177 | 0.0177 | 33.8% |
| 0 | 1 | merkle_whir_independent | whir_opening_proof_after_ood | 0.0305 | 0.0302 | 58.4% |
| 0 | 1 | merkle_whir_independent_rate_quarter | initial_merkle_tree | 0.0052 | 0.0054 | 6.3% |
| 0 | 1 | merkle_whir_independent_rate_quarter | initial_ood_evaluations | 0.0011 | 0.0010 | 1.4% |
| 0 | 1 | merkle_whir_independent_rate_quarter | initial_rs_encoding | 0.0301 | 0.0300 | 36.5% |
| 0 | 1 | merkle_whir_independent_rate_quarter | whir_opening_proof_after_ood | 0.0460 | 0.0447 | 55.7% |
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0112 | 0.0114 | 21.4% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0030 | 0.0029 | 5.8% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0010 | 0.0009 | 2.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0177 | 0.0177 | 33.8% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.0109 | 0.0105 | 20.8% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0002 | 0.0002 | 0.4% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0082 | 0.0076 | 15.7% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0133 | 0.0126 | 16.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0052 | 0.0054 | 6.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0011 | 0.0010 | 1.4% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.0301 | 0.0300 | 36.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.0182 | 0.0186 | 22.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0002 | 0.0002 | 0.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0143 | 0.0139 | 17.3% |
| 2 | 1 | canopy_unbatched | canopy_bottom_forest | 0.0029 | 0.0027 | 4.9% |
| 2 | 1 | canopy_unbatched | canopy_bottom_opening | 0.0002 | 0.0002 | 0.3% |
| 2 | 1 | canopy_unbatched | canopy_claim_whirs | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0034 | 0.0034 | 5.8% |
| 2 | 1 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_hash_to_field | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_rs_encoding | 0.0040 | 0.0037 | 6.7% |
| 2 | 1 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0124 | 0.0132 | 20.7% |
| 2 | 1 | canopy_unbatched | outer_ood_evaluations | 0.0010 | 0.0009 | 1.7% |
| 2 | 1 | canopy_unbatched | outer_rs_encoding | 0.0177 | 0.0177 | 29.6% |
| 2 | 1 | canopy_unbatched | outer_second_oracle_whirs | 0.0181 | 0.0182 | 30.3% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.0187 | 0.0192 | 26.5% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0023 | 0.0022 | 3.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0004 | 0.0000 | 0.5% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0029 | 0.0028 | 4.1% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0012 | 0.0011 | 1.7% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0117 | 0.0116 | 16.5% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0017 | 0.0017 | 2.4% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0124 | 0.0132 | 17.5% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0010 | 0.0009 | 1.4% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.0184 | 0.0183 | 26.0% |
| 0 | 2 | merkle_whir_independent | initial_merkle_tree | 0.0061 | 0.0059 | 5.8% |
| 0 | 2 | merkle_whir_independent | initial_ood_evaluations | 0.0020 | 0.0018 | 2.0% |
| 0 | 2 | merkle_whir_independent | initial_rs_encoding | 0.0353 | 0.0354 | 33.8% |
| 0 | 2 | merkle_whir_independent | whir_opening_proof_after_ood | 0.0609 | 0.0605 | 58.4% |
| 0 | 2 | merkle_whir_independent_rate_quarter | initial_merkle_tree | 0.0105 | 0.0108 | 6.3% |
| 0 | 2 | merkle_whir_independent_rate_quarter | initial_ood_evaluations | 0.0023 | 0.0019 | 1.4% |
| 0 | 2 | merkle_whir_independent_rate_quarter | initial_rs_encoding | 0.0603 | 0.0600 | 36.5% |
| 0 | 2 | merkle_whir_independent_rate_quarter | whir_opening_proof_after_ood | 0.0920 | 0.0893 | 55.7% |
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0119 | 0.0123 | 12.4% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0223 | 0.0228 | 23.3% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0015 | 0.0015 | 1.6% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0061 | 0.0059 | 6.4% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0020 | 0.0018 | 2.1% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0353 | 0.0354 | 36.9% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0164 | 0.0152 | 17.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0185 | 0.0191 | 12.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0265 | 0.0252 | 17.9% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0015 | 0.0015 | 1.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0105 | 0.0108 | 7.1% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0023 | 0.0019 | 1.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.0603 | 0.0600 | 40.7% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0286 | 0.0279 | 19.3% |
| 2 | 2 | canopy_unbatched | canopy_bottom_forest | 0.0059 | 0.0055 | 4.9% |
| 2 | 2 | canopy_unbatched | canopy_bottom_opening | 0.0003 | 0.0003 | 0.3% |
| 2 | 2 | canopy_unbatched | canopy_claim_whirs | 0.0000 | 0.0000 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0069 | 0.0068 | 5.8% |
| 2 | 2 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_hash_to_field | 0.0001 | 0.0001 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_rs_encoding | 0.0080 | 0.0075 | 6.7% |
| 2 | 2 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0247 | 0.0264 | 20.7% |
| 2 | 2 | canopy_unbatched | outer_ood_evaluations | 0.0020 | 0.0018 | 1.7% |
| 2 | 2 | canopy_unbatched | outer_rs_encoding | 0.0353 | 0.0354 | 29.6% |
| 2 | 2 | canopy_unbatched | outer_second_oracle_whirs | 0.0362 | 0.0365 | 30.3% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.0212 | 0.0218 | 17.6% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0039 | 0.0039 | 3.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0003 | 0.0003 | 0.2% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0049 | 0.0048 | 4.1% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0021 | 0.0021 | 1.7% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0241 | 0.0244 | 20.1% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0030 | 0.0030 | 2.5% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0247 | 0.0264 | 20.6% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0020 | 0.0018 | 1.7% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.0340 | 0.0342 | 28.3% |
| 0 | 4 | merkle_whir_independent | initial_merkle_tree | 0.0122 | 0.0118 | 5.8% |
| 0 | 4 | merkle_whir_independent | initial_ood_evaluations | 0.0041 | 0.0035 | 2.0% |
| 0 | 4 | merkle_whir_independent | initial_rs_encoding | 0.0707 | 0.0708 | 33.8% |
| 0 | 4 | merkle_whir_independent | whir_opening_proof_after_ood | 0.1218 | 0.1210 | 58.4% |
| 0 | 4 | merkle_whir_independent_rate_quarter | initial_merkle_tree | 0.0210 | 0.0216 | 6.3% |
| 0 | 4 | merkle_whir_independent_rate_quarter | initial_ood_evaluations | 0.0045 | 0.0039 | 1.4% |
| 0 | 4 | merkle_whir_independent_rate_quarter | initial_rs_encoding | 0.1206 | 0.1201 | 36.5% |
| 0 | 4 | merkle_whir_independent_rate_quarter | whir_opening_proof_after_ood | 0.1840 | 0.1786 | 55.7% |
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0129 | 0.0123 | 7.1% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0447 | 0.0456 | 24.7% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0035 | 0.0034 | 2.0% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0122 | 0.0118 | 6.7% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0041 | 0.0035 | 2.3% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 0.0707 | 0.0708 | 39.1% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0329 | 0.0303 | 18.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0211 | 0.0213 | 7.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.0530 | 0.0505 | 18.9% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0035 | 0.0034 | 1.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0210 | 0.0216 | 7.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0045 | 0.0039 | 1.6% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.1206 | 0.1201 | 42.9% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.0572 | 0.0558 | 20.4% |
| 2 | 4 | canopy_unbatched | canopy_bottom_forest | 0.0117 | 0.0110 | 4.9% |
| 2 | 4 | canopy_unbatched | canopy_bottom_opening | 0.0007 | 0.0007 | 0.3% |
| 2 | 4 | canopy_unbatched | canopy_claim_whirs | 0.0000 | 0.0000 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0138 | 0.0137 | 5.8% |
| 2 | 4 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_hash_to_field | 0.0001 | 0.0001 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_rs_encoding | 0.0161 | 0.0149 | 6.7% |
| 2 | 4 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0495 | 0.0529 | 20.7% |
| 2 | 4 | canopy_unbatched | outer_ood_evaluations | 0.0041 | 0.0035 | 1.7% |
| 2 | 4 | canopy_unbatched | outer_rs_encoding | 0.0707 | 0.0708 | 29.6% |
| 2 | 4 | canopy_unbatched | outer_second_oracle_whirs | 0.0724 | 0.0730 | 30.3% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.0307 | 0.0290 | 13.7% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0076 | 0.0074 | 3.4% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0005 | 0.0005 | 0.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0099 | 0.0097 | 4.4% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0042 | 0.0040 | 1.9% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0440 | 0.0427 | 19.6% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0101 | 0.0095 | 4.5% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0495 | 0.0529 | 22.0% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0041 | 0.0035 | 1.8% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 0.0642 | 0.0628 | 28.6% |
| 0 | 8 | merkle_whir_independent | initial_merkle_tree | 0.0244 | 0.0235 | 5.8% |
| 0 | 8 | merkle_whir_independent | initial_ood_evaluations | 0.0082 | 0.0071 | 2.0% |
| 0 | 8 | merkle_whir_independent | initial_rs_encoding | 0.1413 | 0.1416 | 33.8% |
| 0 | 8 | merkle_whir_independent | whir_opening_proof_after_ood | 0.2437 | 0.2420 | 58.4% |
| 0 | 8 | merkle_whir_independent_rate_quarter | initial_merkle_tree | 0.0419 | 0.0432 | 6.3% |
| 0 | 8 | merkle_whir_independent_rate_quarter | initial_ood_evaluations | 0.0091 | 0.0077 | 1.4% |
| 0 | 8 | merkle_whir_independent_rate_quarter | initial_rs_encoding | 0.2411 | 0.2402 | 36.5% |
| 0 | 8 | merkle_whir_independent_rate_quarter | whir_opening_proof_after_ood | 0.3679 | 0.3573 | 55.7% |
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.0202 | 0.0196 | 5.6% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0893 | 0.0913 | 24.8% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0114 | 0.0121 | 3.2% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0244 | 0.0235 | 6.8% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0082 | 0.0071 | 2.3% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 0.1413 | 0.1416 | 39.2% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.0657 | 0.0607 | 18.2% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.0276 | 0.0280 | 5.0% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.1060 | 0.1010 | 19.2% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0114 | 0.0121 | 2.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0419 | 0.0432 | 7.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0091 | 0.0077 | 1.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.2411 | 0.2402 | 43.7% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.1144 | 0.1115 | 20.7% |
| 2 | 8 | canopy_unbatched | canopy_bottom_forest | 0.0235 | 0.0220 | 4.9% |
| 2 | 8 | canopy_unbatched | canopy_bottom_opening | 0.0014 | 0.0013 | 0.3% |
| 2 | 8 | canopy_unbatched | canopy_claim_whirs | 0.0000 | 0.0000 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0276 | 0.0274 | 5.8% |
| 2 | 8 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_hash_to_field | 0.0002 | 0.0002 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_rs_encoding | 0.0322 | 0.0298 | 6.7% |
| 2 | 8 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.0990 | 0.1057 | 20.7% |
| 2 | 8 | canopy_unbatched | outer_ood_evaluations | 0.0082 | 0.0071 | 1.7% |
| 2 | 8 | canopy_unbatched | outer_rs_encoding | 0.1413 | 0.1416 | 29.6% |
| 2 | 8 | canopy_unbatched | outer_second_oracle_whirs | 0.1447 | 0.1459 | 30.3% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 0.0533 | 0.0520 | 11.9% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0162 | 0.0156 | 3.6% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0003 | 0.0002 | 0.1% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0175 | 0.0164 | 3.9% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0082 | 0.0077 | 1.8% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0843 | 0.0851 | 18.9% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0389 | 0.0389 | 8.7% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.0990 | 0.1057 | 22.2% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0082 | 0.0071 | 1.8% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 0.1205 | 0.1209 | 27.0% |
