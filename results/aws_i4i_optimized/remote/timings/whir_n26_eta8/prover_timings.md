# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 8 | merkle_whir_batched_after_second | 1/2 | 27 | - | 1 | 65.1474 | 50.0594 | 115.2067 | 115.2067 | 1.000x | 1.000x |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 28 | - | 1 | 108.3469 | 77.2085 | 185.5554 | 185.5554 | 1.611x | 1.611x |
| 3 | 8 | canopy_schwartz_zippel | 1/2 | 27 | 4 | 1 | 47.0715 | 59.3723 | 106.4437 | 106.4437 | 0.924x | 0.924x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 8 | merkle_whir_batched_after_second | batched_recursive_whir | 4.8055 | 4.8055 | 4.2% |
| 0 | 8 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 20.0511 | 20.0511 | 17.4% |
| 0 | 8 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 2.7397 | 2.7397 | 2.4% |
| 0 | 8 | merkle_whir_batched_after_second | initial_merkle_tree | 4.5720 | 4.5720 | 4.0% |
| 0 | 8 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.6419 | 0.6419 | 0.6% |
| 0 | 8 | merkle_whir_batched_after_second | initial_rs_encoding | 60.5753 | 60.5753 | 52.6% |
| 0 | 8 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 21.8212 | 21.8212 | 18.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 6.9655 | 6.9655 | 3.8% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 25.4134 | 25.4134 | 13.7% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 2.7397 | 2.7397 | 1.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 9.1229 | 9.1229 | 4.9% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.6921 | 0.6921 | 0.4% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 99.2240 | 99.2240 | 53.5% |
| 0 | 8 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 41.3977 | 41.3977 | 22.3% |
| 3 | 8 | canopy_schwartz_zippel | batched_recursive_whir | 8.0934 | 8.0934 | 7.6% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_forest | 0.9648 | 0.9648 | 0.9% |
| 3 | 8 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0003 | 0.0003 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.5952 | 0.5952 | 0.6% |
| 3 | 8 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 8 | canopy_schwartz_zippel | canopy_hash_to_field | 0.4787 | 0.4787 | 0.4% |
| 3 | 8 | canopy_schwartz_zippel | canopy_rs_encoding | 11.8550 | 11.8550 | 11.1% |
| 3 | 8 | canopy_schwartz_zippel | generalized_sz_cross_terms | 8.6675 | 8.6675 | 8.1% |
| 3 | 8 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 41.9692 | 41.9692 | 39.4% |
| 3 | 8 | canopy_schwartz_zippel | outer_ood_evaluations | 0.6419 | 0.6419 | 0.6% |
| 3 | 8 | canopy_schwartz_zippel | outer_rs_encoding | 33.1778 | 33.1778 | 31.2% |
