# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 25 | - | 3 | 2.8696 | 3.0088 | 5.8785 | 5.8700 | 1.000x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 26 | - | 3 | 4.7684 | 4.5514 | 9.3198 | 9.3195 | 1.585x | 1.585x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 25 | 4 | 3 | 2.3910 | 3.1849 | 5.5759 | 5.5533 | 0.949x | 0.949x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.6069 | 0.6065 | 10.3% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 1.1549 | 1.1625 | 19.6% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.2726 | 0.2740 | 4.6% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.2890 | 0.2870 | 4.9% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0146 | 0.0147 | 0.2% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 2.5807 | 2.5901 | 43.9% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.9598 | 0.9627 | 16.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.9832 | 0.9876 | 10.5% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 1.3894 | 1.4107 | 14.9% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.2726 | 0.2740 | 2.9% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.5685 | 0.5649 | 6.1% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0206 | 0.0146 | 0.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 4.1999 | 4.1623 | 45.1% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 1.8856 | 1.8946 | 20.2% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.7090 | 0.7073 | 12.7% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.1315 | 0.1325 | 2.4% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0837 | 0.0838 | 1.5% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0114 | 0.0112 | 0.2% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.5622 | 0.5643 | 10.1% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.3428 | 0.3408 | 6.1% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 2.1183 | 2.1187 | 38.0% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0146 | 0.0147 | 0.3% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 1.6022 | 1.5959 | 28.7% |
