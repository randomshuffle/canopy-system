# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | 1/2 | 25 | - | 3 | 1.4717 | 1.6064 | 3.0781 | 3.0364 | 1.000x | 1.000x |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 26 | - | 3 | 2.4259 | 2.5690 | 4.9948 | 4.9822 | 1.623x | 1.623x |
| 3 | 1 | canopy_schwartz_zippel | 1/2 | 25 | 4 | 3 | 1.9205 | 1.9202 | 3.8406 | 3.8395 | 1.248x | 1.248x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 1 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.5634 | 0.5523 | 18.3% |
| 0 | 1 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | initial_merkle_tree | 0.1412 | 0.1407 | 4.6% |
| 0 | 1 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0076 | 0.0074 | 0.2% |
| 0 | 1 | merkle_whir_batched_after_second | initial_rs_encoding | 1.3305 | 1.2964 | 43.2% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_after_ood | 0.5534 | 0.5568 | 18.0% |
| 0 | 1 | merkle_whir_batched_after_second | recursive_tail_ood_evaluations | 0.0015 | 0.0015 | 0.0% |
| 0 | 1 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.4806 | 0.4809 | 15.6% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.6777 | 0.6580 | 13.6% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0000 | 0.0000 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.2813 | 0.2799 | 5.6% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0072 | 0.0073 | 0.1% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 2.1445 | 2.1403 | 42.9% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_after_ood | 0.9397 | 0.9408 | 18.8% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | recursive_tail_ood_evaluations | 0.0016 | 0.0016 | 0.0% |
| 0 | 1 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.9427 | 0.9412 | 18.9% |
| 3 | 1 | canopy_schwartz_zippel | batched_recursive_whir | 0.5952 | 0.5935 | 15.5% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_forest | 0.1076 | 0.1069 | 2.8% |
| 3 | 1 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0734 | 0.0733 | 1.9% |
| 3 | 1 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 1 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0060 | 0.0060 | 0.2% |
| 3 | 1 | canopy_schwartz_zippel | canopy_rs_encoding | 0.4430 | 0.4440 | 11.5% |
| 3 | 1 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.2692 | 0.2689 | 7.0% |
| 3 | 1 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 1.0481 | 1.0406 | 27.3% |
| 3 | 1 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0076 | 0.0074 | 0.2% |
| 3 | 1 | canopy_schwartz_zippel | outer_rs_encoding | 1.2905 | 1.2889 | 33.6% |
