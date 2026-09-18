# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 23 | - | 1 | 0.8772 | 0.8216 | 1.6988 | 1.6988 | 1.000x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 1 | 1.5670 | 1.3443 | 2.9113 | 2.9113 | 1.714x | 1.714x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 1 | 0.8672 | 0.8897 | 1.7569 | 1.7569 | 1.034x | 1.034x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.1581 | 0.1581 | 9.3% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.3030 | 0.3030 | 17.8% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0590 | 0.0590 | 3.5% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0743 | 0.0743 | 4.4% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0097 | 0.0097 | 0.6% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 0.8030 | 0.8030 | 47.3% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.2918 | 0.2918 | 17.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.2616 | 0.2616 | 9.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.3969 | 0.3969 | 13.6% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0590 | 0.0590 | 2.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.1455 | 0.1455 | 5.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0109 | 0.0109 | 0.4% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 1.4215 | 1.4215 | 48.8% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.6159 | 0.6159 | 21.2% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.1845 | 0.1845 | 10.5% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0238 | 0.0238 | 1.4% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0209 | 0.0209 | 1.2% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0076 | 0.0076 | 0.4% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1880 | 0.1880 | 10.7% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0853 | 0.0853 | 4.9% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.6101 | 0.6101 | 34.7% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0097 | 0.0097 | 0.6% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.6268 | 0.6268 | 35.7% |
