# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 23 | - | 1 | 0.3881 | 0.4985 | 0.8866 | 0.8866 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 1 | 0.6735 | 0.8029 | 1.4764 | 1.4764 | 1.665x | 1.665x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 1 | 0.5065 | 0.5367 | 1.0431 | 1.0431 | 1.177x | 1.177x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.2092 | 0.2092 | 23.6% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0408 | 0.0408 | 4.6% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0139 | 0.0139 | 1.6% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.3473 | 0.3473 | 39.2% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.1427 | 0.1427 | 16.1% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0018 | 0.0018 | 0.2% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1308 | 0.1308 | 14.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.2854 | 0.2854 | 19.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0774 | 0.0774 | 5.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0155 | 0.0155 | 1.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.5961 | 0.5961 | 40.4% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.2283 | 0.2283 | 15.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0018 | 0.0018 | 0.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.2719 | 0.2719 | 18.4% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.1564 | 0.1564 | 15.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0329 | 0.0329 | 3.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0199 | 0.0199 | 1.9% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0160 | 0.0160 | 1.5% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1197 | 0.1197 | 11.5% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0249 | 0.0249 | 2.4% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.3414 | 0.3414 | 32.7% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0139 | 0.0139 | 1.3% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.3180 | 0.3180 | 30.5% |
