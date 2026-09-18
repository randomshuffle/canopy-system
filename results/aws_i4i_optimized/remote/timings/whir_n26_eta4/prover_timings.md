# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 27 | - | 1 | 32.4194 | 25.7985 | 58.2179 | 58.2179 | 1.000x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 28 | - | 1 | 53.9610 | 40.2381 | 94.1992 | 94.1992 | 1.618x | 1.618x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 27 | 4 | 1 | 26.3482 | 28.5347 | 54.8829 | 54.8829 | 0.943x | 0.943x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 3.5310 | 3.5310 | 6.1% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 9.7256 | 9.7256 | 16.7% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 1.1915 | 1.1915 | 2.0% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 2.2621 | 2.2621 | 3.9% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.3203 | 0.3203 | 0.6% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 30.1573 | 30.1573 | 51.8% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 11.0301 | 11.0301 | 18.9% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 5.6149 | 5.6149 | 6.0% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 11.9270 | 11.9270 | 12.7% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 1.1915 | 1.1915 | 1.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 4.4876 | 4.4876 | 4.8% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.3030 | 0.3030 | 0.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 49.4735 | 49.4735 | 52.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 21.2017 | 21.2017 | 22.5% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 4.7321 | 4.7321 | 8.6% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.6692 | 0.6692 | 1.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.4079 | 0.4079 | 0.7% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.2397 | 0.2397 | 0.4% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 6.9626 | 6.9626 | 12.7% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 2.7758 | 2.7758 | 5.1% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 20.7064 | 20.7064 | 37.7% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.3203 | 0.3203 | 0.6% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 18.0688 | 18.0688 | 32.9% |
