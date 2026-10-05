# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 25 | - | 3 | 11.5861 | 9.8908 | 21.4769 | 21.3214 | 1.000x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 26 | - | 3 | 20.5621 | 15.2748 | 35.8368 | 35.3014 | 1.669x | 1.669x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 25 | 4 | 3 | 5.3428 | 11.3914 | 16.7341 | 16.8211 | 0.779x | 0.779x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.9257 | 0.9215 | 4.3% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 4.4998 | 4.4331 | 21.0% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.4851 | 0.4723 | 2.3% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 1.1510 | 1.1547 | 5.4% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0581 | 0.0582 | 0.3% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 10.4351 | 10.3986 | 48.6% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 3.9220 | 3.8659 | 18.3% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 1.3150 | 1.3104 | 3.7% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 5.7335 | 5.5951 | 16.0% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.4851 | 0.4723 | 1.4% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 2.3168 | 2.3124 | 6.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0581 | 0.0580 | 0.2% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 18.2452 | 17.9307 | 50.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 7.6830 | 7.6938 | 21.4% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 1.4522 | 1.4485 | 8.7% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.2055 | 0.2050 | 1.2% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0004 | 0.0003 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.1327 | 0.1335 | 0.8% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0421 | 0.0421 | 0.3% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 1.2582 | 1.2579 | 7.5% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 1.4482 | 1.4401 | 8.7% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 8.4324 | 8.4570 | 50.4% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0581 | 0.0582 | 0.3% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 3.7043 | 3.7197 | 22.1% |
