# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 23 | - | 10 | 0.3521 | 0.4274 | 0.7795 | 0.7725 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 10 | 0.6409 | 0.9137 | 1.5546 | 1.5878 | 1.994x | 1.994x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 10 | 0.4885 | 0.4668 | 0.9553 | 0.9564 | 1.226x | 1.226x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.1461 | 0.1461 | 18.7% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0373 | 0.0364 | 4.8% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0123 | 0.0122 | 1.6% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.3148 | 0.3114 | 40.4% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.1336 | 0.1332 | 17.1% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0016 | 0.0016 | 0.2% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1338 | 0.1329 | 17.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.4311 | 0.4416 | 27.7% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0718 | 0.0712 | 4.6% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0123 | 0.0122 | 0.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.5691 | 0.5614 | 36.6% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.2148 | 0.2145 | 13.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0017 | 0.0016 | 0.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.2538 | 0.2536 | 16.3% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.1475 | 0.1457 | 15.4% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0309 | 0.0309 | 3.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0003 | 0.0003 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0185 | 0.0182 | 1.9% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0154 | 0.0152 | 1.6% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1085 | 0.1079 | 11.4% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0250 | 0.0250 | 2.6% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.2818 | 0.2800 | 29.5% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0123 | 0.0122 | 1.3% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.3153 | 0.3131 | 33.0% |
