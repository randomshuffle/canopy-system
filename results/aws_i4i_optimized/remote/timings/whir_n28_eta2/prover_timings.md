# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 29 | - | 1 | 70.6271 | 60.2265 | 130.8536 | 130.8536 | 1.000x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 30 | - | 1 | 120.7803 | 95.1920 | 215.9722 | 215.9722 | 1.650x | 1.650x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 29 | 4 | 1 | 72.9199 | 63.0259 | 135.9459 | 135.9459 | 1.039x | 1.039x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 11.9556 | 11.9556 | 9.1% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 20.0153 | 20.0153 | 15.3% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 4.4553 | 4.4553 | 3.4% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 4.5780 | 4.5780 | 3.5% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.6124 | 0.6124 | 0.5% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 66.0490 | 66.0490 | 50.5% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 23.1879 | 23.1879 | 17.7% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 21.2324 | 21.2324 | 9.8% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 24.9075 | 24.9075 | 11.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 4.4553 | 4.4553 | 2.1% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 9.1617 | 9.1617 | 4.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.5903 | 0.5903 | 0.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 111.6186 | 111.6186 | 51.7% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 44.0065 | 44.0065 | 20.4% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 13.9148 | 13.9148 | 10.2% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 2.0695 | 2.0695 | 1.5% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 1.3111 | 1.3111 | 1.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.4786 | 0.4786 | 0.4% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 16.0887 | 16.0887 | 11.8% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 5.0083 | 5.0083 | 3.7% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 43.4903 | 43.4903 | 32.0% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.6124 | 0.6124 | 0.5% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 52.9721 | 52.9721 | 39.0% |
