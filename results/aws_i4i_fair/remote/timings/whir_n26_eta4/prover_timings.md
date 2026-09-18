# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. All three schemes execute concurrent initial encodings and Merkle commitments at t and report batch wall times. The half-rate baseline and Exp3 share the exact measured base RS batch. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales single-input opening-front work by t for all three schemes, and executes batched tails at t. It does not serialize a complete Exp3 proof. See timing_method.json for the scheduling policy.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 27 | - | 1 | 19.1174 | 25.6707 | 44.7881 | 44.7881 | 1.000x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 28 | - | 1 | 32.4479 | 40.0153 | 72.4633 | 72.4633 | 1.618x | 1.618x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 27 | 4 | 1 | 25.9072 | 28.4301 | 54.3373 | 54.3373 | 1.213x | 1.213x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 3.5616 | 3.5616 | 8.0% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 9.6608 | 9.6608 | 21.6% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 1.1926 | 1.1926 | 2.7% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.8230 | 0.8230 | 1.8% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.2962 | 0.2962 | 0.7% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 18.2944 | 18.2944 | 40.8% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 10.9595 | 10.9595 | 24.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 5.6238 | 5.6238 | 7.8% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 11.8939 | 11.8939 | 16.4% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 1.1926 | 1.1926 | 1.6% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 1.6376 | 1.6376 | 2.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.2961 | 0.2961 | 0.4% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 30.8104 | 30.8104 | 42.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 21.0089 | 21.0089 | 29.0% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 4.7347 | 4.7347 | 8.7% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.6723 | 0.6723 | 1.2% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.4090 | 0.4090 | 0.8% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.2400 | 0.2400 | 0.4% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 6.2916 | 6.2916 | 11.6% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 2.7744 | 2.7744 | 5.1% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 20.6246 | 20.6246 | 38.0% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.2962 | 0.2962 | 0.5% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 18.2944 | 18.2944 | 33.7% |
