# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Each input oracle has `2^22` field elements. The rate-1/2 rows use a first codeword of `2^23` elements, while the added rate-1/4 batched baseline uses `2^24` elements. All WHIR components are generated and verifier-checked at 128-bit security over the configured 256-bit field, in johnson mode, with no proof of work. Input generation, verification, and deallocation are excluded. Commitment time contains initial RS encoding and the initial Merkle/Canopy commitment; opening time contains OOD work, openings, recursive oracle commitments, Schwartz--Zippel preprocessing, and recursive WHIR. The selected initial fold is three: Experiment 3 therefore uses aligned Canopy height four, while Experiment 2 uses its independently optimized height `h=9` with its Canopy claim tail measured separately. To match the paper's at-most-`2^10` stopping rule, the reference implementation's native smaller suffix is separately executed and subtracted. Baseline and Experiment-2 independent stages are measured once per repetition and scaled by `t`. Experiment 3 executes its outer RS encodings, Canopy forests, H2F conversions, Canopy RS encodings, Merkle commitments, and bottom openings concurrently at the reported `t`; its transcript-dependent outer first transitions remain scaled independent work. Every batched recursive proof and generalized distinct-functional cross-evaluation stage is executed at the reported `t`.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs independent | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | 1/2 | 23 | - | 5 | 0.4816 | 0.5335 | 1.0151 | 1.0200 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_independent_rate_quarter | 1/4 | 24 | - | 5 | 0.8746 | 0.9323 | 1.8069 | 1.7613 | 1.780x | 1.780x |
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 0.4816 | 0.5335 | 1.0151 | 1.0200 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 0.8746 | 0.9323 | 1.8069 | 1.7613 | 1.780x | 1.780x |
| 2 | 1 | canopy_unbatched | 1/2 | 23 | 9 | 5 | 0.7646 | 0.7104 | 1.4750 | 1.4591 | 1.453x | 1.453x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 0.8741 | 0.5816 | 1.4557 | 1.4513 | 1.434x | 1.434x |
| 0 | 2 | merkle_whir_independent | 1/2 | 23 | - | 5 | 0.9632 | 1.0670 | 2.0302 | 2.0400 | 1.000x | 1.064x |
| 0 | 2 | merkle_whir_independent_rate_quarter | 1/4 | 24 | - | 5 | 1.7491 | 1.8647 | 3.6138 | 3.5227 | 1.780x | 1.894x |
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 0.9632 | 0.9445 | 1.9077 | 1.8930 | 0.940x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 1.7491 | 1.6280 | 3.3771 | 3.2966 | 1.663x | 1.770x |
| 2 | 2 | canopy_unbatched | 1/2 | 23 | 9 | 5 | 1.5293 | 1.4208 | 2.9501 | 2.9181 | 1.453x | 1.546x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 1.6764 | 0.8865 | 2.5629 | 2.5294 | 1.262x | 1.343x |
| 0 | 4 | merkle_whir_independent | 1/2 | 23 | - | 5 | 1.9264 | 2.1340 | 4.0604 | 4.0800 | 1.000x | 1.109x |
| 0 | 4 | merkle_whir_independent_rate_quarter | 1/4 | 24 | - | 5 | 3.4982 | 3.7294 | 7.2276 | 7.0454 | 1.780x | 1.974x |
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 1.9264 | 1.7348 | 3.6612 | 3.6416 | 0.902x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 3.4982 | 2.9857 | 6.4839 | 6.3325 | 1.597x | 1.771x |
| 2 | 4 | canopy_unbatched | 1/2 | 23 | 9 | 5 | 3.0586 | 2.8415 | 5.9001 | 5.8362 | 1.453x | 1.612x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 3.2421 | 1.8228 | 5.0649 | 4.9941 | 1.247x | 1.383x |
| 0 | 8 | merkle_whir_independent | 1/2 | 23 | - | 5 | 3.8528 | 4.2680 | 8.1207 | 8.1601 | 1.000x | 1.108x |
| 0 | 8 | merkle_whir_independent_rate_quarter | 1/4 | 24 | - | 5 | 6.9964 | 7.4588 | 14.4552 | 14.0908 | 1.780x | 1.973x |
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 3.8528 | 3.4750 | 7.3278 | 7.3756 | 0.902x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 6.9964 | 5.9807 | 12.9771 | 12.8408 | 1.598x | 1.771x |
| 2 | 8 | canopy_unbatched | 1/2 | 23 | 9 | 5 | 6.1171 | 5.6831 | 11.8002 | 11.6724 | 1.453x | 1.610x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 8.5322 | 3.8969 | 12.4291 | 11.6625 | 1.531x | 1.696x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_independent | initial_merkle_tree | 0.0501 | 0.0500 | 4.9% |
| 0 | 1 | merkle_whir_independent | initial_ood_evaluations | 0.0151 | 0.0149 | 1.5% |
| 0 | 1 | merkle_whir_independent | initial_rs_encoding | 0.4315 | 0.4361 | 42.5% |
| 0 | 1 | merkle_whir_independent | whir_opening_proof_after_ood | 0.5184 | 0.5224 | 51.1% |
| 0 | 1 | merkle_whir_independent_rate_quarter | initial_merkle_tree | 0.0949 | 0.0936 | 5.3% |
| 0 | 1 | merkle_whir_independent_rate_quarter | initial_ood_evaluations | 0.0200 | 0.0176 | 1.1% |
| 0 | 1 | merkle_whir_independent_rate_quarter | initial_rs_encoding | 0.7797 | 0.7729 | 43.1% |
| 0 | 1 | merkle_whir_independent_rate_quarter | whir_opening_proof_after_ood | 0.9123 | 0.8829 | 50.5% |
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.1707 | 0.1716 | 16.8% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0501 | 0.0500 | 4.9% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0151 | 0.0149 | 1.5% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.4315 | 0.4361 | 42.5% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.1764 | 0.1775 | 17.4% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0022 | 0.0020 | 0.2% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1692 | 0.1674 | 16.7% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.2442 | 0.2511 | 13.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0949 | 0.0936 | 5.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0200 | 0.0176 | 1.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.7797 | 0.7729 | 43.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.2910 | 0.2846 | 16.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0021 | 0.0020 | 0.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.3751 | 0.3580 | 20.8% |
| 2 | 1 | canopy_unbatched | canopy_bottom_forest | 0.0496 | 0.0486 | 3.4% |
| 2 | 1 | canopy_unbatched | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_claim_whirs | 0.1767 | 0.1764 | 12.0% |
| 2 | 1 | canopy_unbatched | canopy_codeword_merkle_tree | 0.0524 | 0.0533 | 3.6% |
| 2 | 1 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 1 | canopy_unbatched | canopy_hash_to_field | 0.0012 | 0.0011 | 0.1% |
| 2 | 1 | canopy_unbatched | canopy_rs_encoding | 0.2300 | 0.2243 | 15.6% |
| 2 | 1 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.2178 | 0.2189 | 14.8% |
| 2 | 1 | canopy_unbatched | outer_ood_evaluations | 0.0151 | 0.0149 | 1.0% |
| 2 | 1 | canopy_unbatched | outer_rs_encoding | 0.4315 | 0.4361 | 29.3% |
| 2 | 1 | canopy_unbatched | outer_second_oracle_whirs | 0.3006 | 0.3041 | 20.4% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.3129 | 0.3120 | 21.5% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0440 | 0.0429 | 3.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0001 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0493 | 0.0496 | 3.4% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0221 | 0.0215 | 1.5% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.3057 | 0.3057 | 21.0% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0357 | 0.0355 | 2.5% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.2178 | 0.2189 | 15.0% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0151 | 0.0149 | 1.0% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.4529 | 0.4471 | 31.1% |
| 0 | 2 | merkle_whir_independent | initial_merkle_tree | 0.1002 | 0.1000 | 4.9% |
| 0 | 2 | merkle_whir_independent | initial_ood_evaluations | 0.0302 | 0.0298 | 1.5% |
| 0 | 2 | merkle_whir_independent | initial_rs_encoding | 0.8630 | 0.8721 | 42.5% |
| 0 | 2 | merkle_whir_independent | whir_opening_proof_after_ood | 1.0368 | 1.0448 | 51.1% |
| 0 | 2 | merkle_whir_independent_rate_quarter | initial_merkle_tree | 0.1898 | 0.1872 | 5.3% |
| 0 | 2 | merkle_whir_independent_rate_quarter | initial_ood_evaluations | 0.0400 | 0.0351 | 1.1% |
| 0 | 2 | merkle_whir_independent_rate_quarter | initial_rs_encoding | 1.5593 | 1.5457 | 43.1% |
| 0 | 2 | merkle_whir_independent_rate_quarter | whir_opening_proof_after_ood | 1.8247 | 1.7657 | 50.5% |
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.1982 | 0.1970 | 10.4% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.3413 | 0.3431 | 17.9% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0364 | 0.0363 | 1.9% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.1002 | 0.1000 | 5.3% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0302 | 0.0298 | 1.6% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 0.8630 | 0.8721 | 45.2% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.3383 | 0.3348 | 17.7% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.3130 | 0.3131 | 9.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.4884 | 0.5023 | 14.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0364 | 0.0363 | 1.1% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.1898 | 0.1872 | 5.6% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0400 | 0.0351 | 1.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 1.5593 | 1.5457 | 46.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.7501 | 0.7160 | 22.2% |
| 2 | 2 | canopy_unbatched | canopy_bottom_forest | 0.0992 | 0.0972 | 3.4% |
| 2 | 2 | canopy_unbatched | canopy_bottom_opening | 0.0004 | 0.0004 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_claim_whirs | 0.3534 | 0.3528 | 12.0% |
| 2 | 2 | canopy_unbatched | canopy_codeword_merkle_tree | 0.1048 | 0.1066 | 3.6% |
| 2 | 2 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 2 | canopy_unbatched | canopy_hash_to_field | 0.0024 | 0.0022 | 0.1% |
| 2 | 2 | canopy_unbatched | canopy_rs_encoding | 0.4600 | 0.4485 | 15.6% |
| 2 | 2 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.4355 | 0.4378 | 14.8% |
| 2 | 2 | canopy_unbatched | outer_ood_evaluations | 0.0302 | 0.0298 | 1.0% |
| 2 | 2 | canopy_unbatched | outer_rs_encoding | 0.8630 | 0.8721 | 29.3% |
| 2 | 2 | canopy_unbatched | outer_second_oracle_whirs | 0.6013 | 0.6082 | 20.4% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.3565 | 0.3555 | 13.9% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0849 | 0.0862 | 3.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0004 | 0.0005 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0976 | 0.0982 | 3.8% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0445 | 0.0438 | 1.7% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.5966 | 0.5980 | 23.3% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0639 | 0.0638 | 2.5% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.4355 | 0.4378 | 17.0% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0302 | 0.0298 | 1.2% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.8527 | 0.8437 | 33.3% |
| 0 | 4 | merkle_whir_independent | initial_merkle_tree | 0.2004 | 0.2000 | 4.9% |
| 0 | 4 | merkle_whir_independent | initial_ood_evaluations | 0.0604 | 0.0596 | 1.5% |
| 0 | 4 | merkle_whir_independent | initial_rs_encoding | 1.7259 | 1.7443 | 42.5% |
| 0 | 4 | merkle_whir_independent | whir_opening_proof_after_ood | 2.0735 | 2.0895 | 51.1% |
| 0 | 4 | merkle_whir_independent_rate_quarter | initial_merkle_tree | 0.3795 | 0.3745 | 5.3% |
| 0 | 4 | merkle_whir_independent_rate_quarter | initial_ood_evaluations | 0.0800 | 0.0702 | 1.1% |
| 0 | 4 | merkle_whir_independent_rate_quarter | initial_rs_encoding | 3.1187 | 3.0915 | 43.1% |
| 0 | 4 | merkle_whir_independent_rate_quarter | whir_opening_proof_after_ood | 3.6494 | 3.5314 | 50.5% |
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.2449 | 0.2449 | 6.7% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.6827 | 0.6862 | 18.6% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0702 | 0.0700 | 1.9% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.2004 | 0.2000 | 5.5% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0604 | 0.0596 | 1.7% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 1.7259 | 1.7443 | 47.1% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.6766 | 0.6696 | 18.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.3584 | 0.3608 | 5.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.9769 | 1.0045 | 15.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0702 | 0.0700 | 1.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.3795 | 0.3745 | 5.9% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0800 | 0.0702 | 1.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 3.1187 | 3.0915 | 48.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 1.5002 | 1.4320 | 23.1% |
| 2 | 4 | canopy_unbatched | canopy_bottom_forest | 0.1983 | 0.1943 | 3.4% |
| 2 | 4 | canopy_unbatched | canopy_bottom_opening | 0.0008 | 0.0008 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_claim_whirs | 0.7068 | 0.7056 | 12.0% |
| 2 | 4 | canopy_unbatched | canopy_codeword_merkle_tree | 0.2096 | 0.2132 | 3.6% |
| 2 | 4 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 4 | canopy_unbatched | canopy_hash_to_field | 0.0047 | 0.0045 | 0.1% |
| 2 | 4 | canopy_unbatched | canopy_rs_encoding | 0.9200 | 0.8970 | 15.6% |
| 2 | 4 | canopy_unbatched | outer_initial_whir_stage_after_ood | 0.8710 | 0.8755 | 14.8% |
| 2 | 4 | canopy_unbatched | outer_ood_evaluations | 0.0604 | 0.0596 | 1.0% |
| 2 | 4 | canopy_unbatched | outer_rs_encoding | 1.7259 | 1.7443 | 29.3% |
| 2 | 4 | canopy_unbatched | outer_second_oracle_whirs | 1.2025 | 1.2164 | 20.4% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.6766 | 0.6286 | 13.4% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.1647 | 0.1635 | 3.3% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0006 | 0.0006 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.1833 | 0.1793 | 3.6% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0885 | 0.0883 | 1.7% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 1.1633 | 1.1628 | 23.0% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.2142 | 0.2100 | 4.2% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.8710 | 0.8755 | 17.2% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0604 | 0.0596 | 1.2% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 1.6423 | 1.6374 | 32.4% |
| 0 | 8 | merkle_whir_independent | initial_merkle_tree | 0.4009 | 0.4001 | 4.9% |
| 0 | 8 | merkle_whir_independent | initial_ood_evaluations | 0.1209 | 0.1193 | 1.5% |
| 0 | 8 | merkle_whir_independent | initial_rs_encoding | 3.4519 | 3.4885 | 42.5% |
| 0 | 8 | merkle_whir_independent | whir_opening_proof_after_ood | 4.1471 | 4.1790 | 51.1% |
| 0 | 8 | merkle_whir_independent_rate_quarter | initial_merkle_tree | 0.7591 | 0.7490 | 5.3% |
| 0 | 8 | merkle_whir_independent_rate_quarter | initial_ood_evaluations | 0.1601 | 0.1404 | 1.1% |
| 0 | 8 | merkle_whir_independent_rate_quarter | initial_rs_encoding | 6.2373 | 6.1829 | 43.1% |
| 0 | 8 | merkle_whir_independent_rate_quarter | whir_opening_proof_after_ood | 7.2987 | 7.0628 | 50.5% |
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.4281 | 0.4195 | 5.8% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 1.3653 | 1.3724 | 18.6% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.2075 | 0.2025 | 2.8% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.4009 | 0.4001 | 5.5% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.1209 | 0.1193 | 1.6% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 3.4519 | 3.4885 | 47.1% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 1.3532 | 1.3392 | 18.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.6589 | 0.6328 | 5.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 1.9538 | 2.0090 | 15.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.2075 | 0.2025 | 1.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.7591 | 0.7490 | 5.8% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.1601 | 0.1404 | 1.2% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 6.2373 | 6.1829 | 48.1% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 3.0004 | 2.8641 | 23.1% |
| 2 | 8 | canopy_unbatched | canopy_bottom_forest | 0.3966 | 0.3887 | 3.4% |
| 2 | 8 | canopy_unbatched | canopy_bottom_opening | 0.0016 | 0.0016 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_claim_whirs | 1.4135 | 1.4111 | 12.0% |
| 2 | 8 | canopy_unbatched | canopy_codeword_merkle_tree | 0.4192 | 0.4265 | 3.6% |
| 2 | 8 | canopy_unbatched | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 2 | 8 | canopy_unbatched | canopy_hash_to_field | 0.0095 | 0.0089 | 0.1% |
| 2 | 8 | canopy_unbatched | canopy_rs_encoding | 1.8400 | 1.7940 | 15.6% |
| 2 | 8 | canopy_unbatched | outer_initial_whir_stage_after_ood | 1.7420 | 1.7510 | 14.8% |
| 2 | 8 | canopy_unbatched | outer_ood_evaluations | 0.1209 | 0.1193 | 1.0% |
| 2 | 8 | canopy_unbatched | outer_rs_encoding | 3.4519 | 3.4885 | 29.3% |
| 2 | 8 | canopy_unbatched | outer_second_oracle_whirs | 2.4051 | 2.4329 | 20.4% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 1.1695 | 1.1742 | 9.4% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.3305 | 0.3256 | 2.7% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0115 | 0.0052 | 0.1% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.3470 | 0.3211 | 2.8% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.1770 | 0.1782 | 1.4% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 4.1080 | 3.7261 | 33.1% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.8530 | 0.8243 | 6.9% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 1.7420 | 1.7510 | 14.0% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.1209 | 0.1193 | 1.0% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 3.5697 | 3.2510 | 28.7% |
