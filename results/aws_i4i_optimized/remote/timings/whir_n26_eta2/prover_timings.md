# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 27 | - | 1 | 15.9684 | 14.7514 | 30.7199 | 30.7199 | 1.000x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 28 | - | 1 | 26.9989 | 22.7967 | 49.7955 | 49.7955 | 1.621x | 1.621x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 27 | 4 | 1 | 16.5772 | 15.4028 | 31.9801 | 31.9801 | 1.041x | 1.041x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 3.0203 | 3.0203 | 9.8% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 5.0973 | 5.0973 | 16.6% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 1.0771 | 1.0771 | 3.5% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 1.1406 | 1.1406 | 3.7% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.1604 | 0.1604 | 0.5% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 14.8278 | 14.8278 | 48.3% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 5.3963 | 5.3963 | 17.6% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 5.1261 | 5.1261 | 10.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 5.9201 | 5.9201 | 11.9% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 1.0771 | 1.0771 | 2.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 2.2285 | 2.2285 | 4.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.1555 | 0.1555 | 0.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 24.7703 | 24.7703 | 49.7% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 10.5179 | 10.5179 | 21.1% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 3.5605 | 3.5605 | 11.1% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.5168 | 0.5168 | 1.6% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.3273 | 0.3273 | 1.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.1198 | 0.1198 | 0.4% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 3.9359 | 3.9359 | 12.3% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 1.2286 | 1.2286 | 3.8% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 10.4532 | 10.4532 | 32.7% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.1604 | 0.1604 | 0.5% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 11.6774 | 11.6774 | 36.5% |
