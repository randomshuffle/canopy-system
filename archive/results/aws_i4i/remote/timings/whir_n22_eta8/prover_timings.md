# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 23 | - | 3 | 1.9091 | 2.0974 | 4.0065 | 4.3642 | 1.000x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 3 | 4.2474 | 3.7441 | 7.9915 | 8.1155 | 1.995x | 1.995x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 3 | 1.1549 | 2.4159 | 3.5709 | 3.4288 | 0.891x | 0.891x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 0.2226 | 0.2224 | 5.6% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 1.0327 | 0.6914 | 25.8% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0993 | 0.0974 | 2.5% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 0.1678 | 0.0993 | 4.2% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0198 | 0.0198 | 0.5% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 1.7413 | 1.8019 | 43.5% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.7231 | 0.8997 | 18.0% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.3010 | 0.2961 | 3.8% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 1.8263 | 1.3875 | 22.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0993 | 0.0974 | 1.2% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.4251 | 0.5729 | 5.3% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0198 | 0.0196 | 0.2% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 3.8222 | 3.9217 | 47.8% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 1.4977 | 1.8431 | 18.7% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 0.3532 | 0.3549 | 9.9% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0466 | 0.0461 | 1.3% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0003 | 0.0003 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0362 | 0.0367 | 1.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0105 | 0.0105 | 0.3% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 0.2543 | 0.2568 | 7.1% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.2356 | 0.2329 | 6.6% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 1.8072 | 1.6618 | 50.6% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0198 | 0.0198 | 0.6% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 0.8073 | 0.8089 | 22.6% |
