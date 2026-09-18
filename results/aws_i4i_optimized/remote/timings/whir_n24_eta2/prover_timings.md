# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 25 | - | 1 | 3.8135 | 3.5135 | 7.3270 | 7.3270 | 1.000x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 26 | - | 1 | 6.6290 | 5.5626 | 12.1916 | 12.1916 | 1.664x | 1.664x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 25 | 4 | 1 | 3.8440 | 3.7739 | 7.6179 | 7.6179 | 1.040x | 1.040x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.6995 | 0.6995 | 9.5% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 1.1950 | 1.1950 | 16.3% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.2713 | 0.2713 | 3.7% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.2862 | 0.2862 | 3.9% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0375 | 0.0375 | 0.5% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 3.5273 | 3.5273 | 48.1% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 1.3102 | 1.3102 | 17.9% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 1.2099 | 1.2099 | 9.9% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 1.5017 | 1.5017 | 12.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.2713 | 0.2713 | 2.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.5772 | 0.5772 | 4.7% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0444 | 0.0444 | 0.4% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 6.0518 | 6.0518 | 49.6% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 2.5354 | 2.5354 | 20.8% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.8471 | 0.8471 | 11.1% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.1330 | 0.1330 | 1.7% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0829 | 0.0829 | 1.1% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0303 | 0.0303 | 0.4% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.8934 | 0.8934 | 11.7% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.3658 | 0.3658 | 4.8% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 2.5234 | 2.5234 | 33.1% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0375 | 0.0375 | 0.5% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 2.7044 | 2.7044 | 35.5% |
