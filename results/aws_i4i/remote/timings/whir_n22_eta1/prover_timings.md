# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 23 | - | 3 | 0.3366 | 0.3434 | 0.6800 | 0.6776 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 3 | 0.5642 | 0.6135 | 1.1777 | 1.1771 | 1.732x | 1.732x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 3 | 0.4309 | 0.4125 | 0.8434 | 0.8381 | 1.240x | 1.240x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.0995 | 0.0953 | 14.6% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0389 | 0.0385 | 5.7% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0024 | 0.0025 | 0.4% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.2977 | 0.2966 | 43.8% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.1267 | 0.1264 | 18.6% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0008 | 0.0009 | 0.1% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1139 | 0.1137 | 16.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.1720 | 0.1720 | 14.6% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0746 | 0.0748 | 6.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0024 | 0.0024 | 0.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.4895 | 0.4875 | 41.6% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.2007 | 0.2018 | 17.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0009 | 0.0009 | 0.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.2375 | 0.2359 | 20.2% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.1399 | 0.1384 | 16.6% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0224 | 0.0223 | 2.7% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0197 | 0.0193 | 2.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0020 | 0.0021 | 0.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.0929 | 0.0920 | 11.0% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0544 | 0.0540 | 6.4% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.2157 | 0.2081 | 25.6% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0024 | 0.0025 | 0.3% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.2939 | 0.2929 | 34.9% |
