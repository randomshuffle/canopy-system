# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 23 | - | 10 | 0.3974 | 0.4952 | 0.8926 | 0.8837 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 10 | 0.7792 | 1.0805 | 1.8597 | 1.7010 | 2.083x | 2.083x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 10 | 0.5490 | 0.5328 | 1.0819 | 1.0878 | 1.212x | 1.212x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.1811 | 0.1876 | 20.3% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0375 | 0.0370 | 4.2% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0144 | 0.0123 | 1.6% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.3599 | 0.3479 | 40.3% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.1608 | 0.1400 | 18.0% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0018 | 0.0017 | 0.2% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1443 | 0.1470 | 16.2% |
| 0 | 1 | merkle_whir_batched_after_second | separately_measured_component_adjustment | -0.0071 | 0.0000 | -0.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.5306 | 0.4342 | 28.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0771 | 0.0714 | 4.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0223 | 0.0126 | 1.2% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.7021 | 0.6454 | 37.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.2331 | 0.2245 | 12.5% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0018 | 0.0016 | 0.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.2927 | 0.2869 | 15.7% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.1584 | 0.1497 | 14.6% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0356 | 0.0356 | 3.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0003 | 0.0003 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0183 | 0.0181 | 1.7% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0185 | 0.0176 | 1.7% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1207 | 0.1221 | 11.2% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0253 | 0.0253 | 2.3% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.3345 | 0.3256 | 30.9% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0144 | 0.0123 | 1.3% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.3559 | 0.3529 | 32.9% |
