# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 27 | - | 1 | 8.0812 | 8.1177 | 16.1989 | 16.1989 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 28 | - | 1 | 13.6581 | 13.3210 | 26.9791 | 26.9791 | 1.665x | 1.665x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 27 | 4 | 1 | 10.8275 | 9.4217 | 20.2491 | 20.2491 | 1.250x | 1.250x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 2.4181 | 2.4181 | 14.9% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.5616 | 0.5616 | 3.5% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0801 | 0.0801 | 0.5% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 7.5196 | 7.5196 | 46.4% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 2.8277 | 2.8277 | 17.5% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0096 | 0.0096 | 0.1% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 2.7824 | 2.7824 | 17.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 3.0444 | 3.0444 | 11.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 1.1345 | 1.1345 | 4.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0741 | 0.0741 | 0.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 12.5236 | 12.5236 | 46.4% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 4.8680 | 4.8680 | 18.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0095 | 0.0095 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 5.3250 | 5.3250 | 19.7% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 3.0436 | 3.0436 | 15.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.4339 | 0.4339 | 2.1% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.2828 | 0.2828 | 1.4% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0602 | 0.0602 | 0.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 2.6259 | 2.6259 | 13.0% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 1.0742 | 1.0742 | 5.3% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 5.2238 | 5.2238 | 25.8% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0801 | 0.0801 | 0.4% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 7.4247 | 7.4247 | 36.7% |
