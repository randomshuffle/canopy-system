# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 23 | - | 1 | 0.4602 | 0.4503 | 0.9105 | 0.9105 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 1 | 0.7809 | 0.7706 | 1.5515 | 1.5515 | 1.704x | 1.704x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 1 | 0.6121 | 0.5137 | 1.1258 | 1.1258 | 1.236x | 1.236x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.1513 | 0.1513 | 16.6% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0382 | 0.0382 | 4.2% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0049 | 0.0049 | 0.5% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.4220 | 0.4220 | 46.3% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.1465 | 0.1465 | 16.1% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0008 | 0.0008 | 0.1% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1469 | 0.1469 | 16.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.2165 | 0.2165 | 14.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0740 | 0.0740 | 4.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0049 | 0.0049 | 0.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.7070 | 0.7070 | 45.6% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.2397 | 0.2397 | 15.4% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0008 | 0.0008 | 0.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.3087 | 0.3087 | 19.9% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.1557 | 0.1557 | 13.8% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0260 | 0.0260 | 2.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0182 | 0.0182 | 1.6% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0042 | 0.0042 | 0.4% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1380 | 0.1380 | 12.3% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0542 | 0.0542 | 4.8% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.2988 | 0.2988 | 26.5% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0049 | 0.0049 | 0.4% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.4257 | 0.4257 | 37.8% |
