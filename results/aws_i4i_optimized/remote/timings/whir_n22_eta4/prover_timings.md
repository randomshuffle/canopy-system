# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 4 | merkle_whir_batched_after_second | 1/2 | 23 | - | 1 | 1.7860 | 1.4875 | 3.2734 | 3.2734 | 1.000x | 1.000x |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 1 | 3.0973 | 2.3935 | 5.4907 | 5.4907 | 1.677x | 1.677x |
| 3 | 4 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 1 | 1.3420 | 1.6249 | 2.9670 | 2.9670 | 0.906x | 0.906x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 4 | merkle_whir_batched_after_second | batched_recursive_whir | 0.1918 | 0.1918 | 5.9% |
| 0 | 4 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.6108 | 0.6108 | 18.7% |
| 0 | 4 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0837 | 0.0837 | 2.6% |
| 0 | 4 | merkle_whir_batched_after_second | initial_merkle_tree | 0.1452 | 0.1452 | 4.4% |
| 0 | 4 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0194 | 0.0194 | 0.6% |
| 0 | 4 | merkle_whir_batched_after_second | initial_rs_encoding | 1.6408 | 1.6408 | 50.1% |
| 0 | 4 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.5818 | 0.5818 | 17.8% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.2844 | 0.2844 | 5.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.7821 | 0.7821 | 14.2% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0837 | 0.0837 | 1.5% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.2904 | 0.2904 | 5.3% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0228 | 0.0228 | 0.4% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 2.8069 | 2.8069 | 51.1% |
| 0 | 4 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 1.2205 | 1.2205 | 22.2% |
| 3 | 4 | canopy_schwartz_zippel | batched_recursive_whir | 0.2496 | 0.2496 | 8.4% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0303 | 0.0303 | 1.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0001 | 0.0001 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0273 | 0.0273 | 0.9% |
| 3 | 4 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 4 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0146 | 0.0146 | 0.5% |
| 3 | 4 | canopy_schwartz_zippel | canopy_rs_encoding | 0.3296 | 0.3296 | 11.1% |
| 3 | 4 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.1611 | 0.1611 | 5.4% |
| 3 | 4 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 1.1947 | 1.1947 | 40.3% |
| 3 | 4 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0194 | 0.0194 | 0.7% |
| 3 | 4 | canopy_schwartz_zippel | outer_rs_encoding | 0.9402 | 0.9402 | 31.7% |
