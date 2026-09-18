# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 23 | - | 1 | 3.5271 | 2.8390 | 6.3661 | 6.3661 | 1.000x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 1 | 6.0763 | 4.5856 | 10.6619 | 10.6619 | 1.675x | 1.675x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 1 | 2.3969 | 3.4729 | 5.8698 | 5.8698 | 0.922x | 0.922x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.2698 | 0.2698 | 4.2% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 1.2224 | 1.2224 | 19.2% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.1439 | 0.1439 | 2.3% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.3001 | 0.3001 | 4.7% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0393 | 0.0393 | 0.6% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 3.2270 | 3.2270 | 50.7% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 1.1637 | 1.1637 | 18.3% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.3613 | 0.3613 | 3.4% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 1.5208 | 1.5208 | 14.3% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.1439 | 0.1439 | 1.3% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.5713 | 0.5713 | 5.4% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0418 | 0.0418 | 0.4% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 5.5050 | 5.5050 | 51.6% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 2.5178 | 2.5178 | 23.6% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 0.4590 | 0.4590 | 7.8% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0530 | 0.0530 | 0.9% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0400 | 0.0400 | 0.7% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0291 | 0.0291 | 0.5% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 0.5862 | 0.5862 | 10.0% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.4745 | 0.4745 | 8.1% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 2.5000 | 2.5000 | 42.6% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0393 | 0.0393 | 0.7% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 1.6886 | 1.6886 | 28.8% |
