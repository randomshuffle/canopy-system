# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. All three schemes execute concurrent initial encodings and Merkle commitments at t and report batch wall times. The half-rate baseline and Exp3 share the exact measured base RS batch. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales single-input opening-front work by t for all three schemes, and executes batched tails at t. It does not serialize a complete Exp3 proof. See timing_method.json for the scheduling policy.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 31 | - | 1 | 153.1837 | 137.6416 | 290.8253 | 290.8253 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 32 | - | 1 | 253.5498 | 242.1285 | 495.6783 | 495.6783 | 1.704x | 1.704x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 31 | 4 | 1 | 202.9535 | 160.8464 | 363.8000 | 363.8000 | 1.251x | 1.251x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 42.1565 | 42.1565 | 14.5% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 8.9813 | 8.9813 | 3.1% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 1.4423 | 1.4423 | 0.5% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 144.2024 | 144.2024 | 49.6% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 46.0726 | 46.0726 | 15.8% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.1817 | 0.1817 | 0.1% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 47.7884 | 47.7884 | 16.4% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 62.1629 | 62.1629 | 12.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 18.2465 | 18.2465 | 3.7% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 1.4485 | 1.4485 | 0.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 235.3033 | 235.3033 | 47.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 85.8080 | 85.8080 | 17.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.1771 | 0.1771 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 92.5321 | 92.5321 | 18.7% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 50.6742 | 50.6742 | 13.9% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 7.0839 | 7.0839 | 1.9% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0004 | 0.0004 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 4.5475 | 4.5475 | 1.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.1150 | 0.1150 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 47.0049 | 47.0049 | 12.9% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 18.8959 | 18.8959 | 5.2% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 89.8335 | 89.8335 | 24.7% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 1.4423 | 1.4423 | 0.4% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 144.2024 | 144.2024 | 39.6% |
