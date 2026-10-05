# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 23 | - | 5 | 0.3280 | 0.3899 | 0.7179 | 0.7174 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 5 | 0.5894 | 0.7761 | 1.3655 | 1.3450 | 1.902x | 1.902x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 5 | 0.5836 | 0.4350 | 1.0185 | 1.0114 | 1.419x | 1.419x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.1227 | 0.1214 | 17.1% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0350 | 0.0347 | 4.9% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0117 | 0.0117 | 1.6% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 0.2930 | 0.2930 | 40.8% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.1322 | 0.1325 | 18.4% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0016 | 0.0015 | 0.2% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1217 | 0.1217 | 17.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.3039 | 0.2968 | 22.3% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.0689 | 0.0677 | 5.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0115 | 0.0116 | 0.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.5205 | 0.5205 | 38.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.2042 | 0.2045 | 15.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0015 | 0.0015 | 0.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.2550 | 0.2453 | 18.7% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.2276 | 0.2231 | 22.3% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0296 | 0.0294 | 2.9% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0003 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0345 | 0.0346 | 3.4% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0148 | 0.0149 | 1.4% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.2026 | 0.2025 | 19.9% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0233 | 0.0234 | 2.3% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.1721 | 0.1719 | 16.9% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0117 | 0.0117 | 1.1% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 0.3020 | 0.3014 | 29.7% |
