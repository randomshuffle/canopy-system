# Final Johnson-bound paper prover timings

Field: **GF(2^256), additive Cantor-subspace RS**. Inputs and evaluation points are deterministic full-width values.

Three batched schemes, 136-bit local security budget. Signed component adjustments reconcile separately measured substages without changing timed samples. Fold log 3; Exp3 height 4. Timings include commitment and opening, exclude input generation and verification. This component harness executes and verifies native WHIR components, subtracts the separately measured native suffix for the paper terminal cap, scales baseline front work by t, and executes batched tails and Exp3 commitments at t. It does not serialize a complete Exp3 proof.

The base code is shared exactly: WHIR's interleaved NTT produces the same row-major matrix for every scheme. Merkle-WHIR commits all of its rows with WHIR's matrix/Merkle commitment; Canopy starts its bottom forest from those same WHIR row hashes and also uses WHIR's interleaved NTT plus matrix/Merkle commitment for the Canopy-layer codeword. Initial OOD evaluations are accounted as opening work.

| experiment | t | scheme | rate | log2 codeword | h | repetitions | commitment s | opening s | total s | median total s | vs batched | vs batched |
|---:|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 2 | merkle_whir_batched_after_second | 1/2 | 23 | - | 3 | 0.6406 | 0.6253 | 1.2658 | 1.2570 | 1.000x | 1.000x |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | 1/4 | 24 | - | 3 | 1.1388 | 0.9812 | 2.1200 | 2.0733 | 1.675x | 1.675x |
| 3 | 2 | canopy_schwartz_zippel | 1/2 | 23 | 4 | 3 | 0.5557 | 0.6704 | 1.2260 | 1.2536 | 0.969x | 0.969x |

## Mean stage breakdown

| experiment | t | scheme | stage | mean s | median s | share |
|---:|---:|---|---|---:|---:|---:|
| 0 | 2 | merkle_whir_batched_after_second | batched_recursive_whir | 0.1419 | 0.1429 | 11.2% |
| 0 | 2 | merkle_whir_batched_after_second | first_transition_residual_after_subtracting_tail | 0.2279 | 0.2466 | 18.0% |
| 0 | 2 | merkle_whir_batched_after_second | generalized_sz_cross_terms | 0.0542 | 0.0528 | 4.3% |
| 0 | 2 | merkle_whir_batched_after_second | initial_merkle_tree | 0.0416 | 0.0246 | 3.3% |
| 0 | 2 | merkle_whir_batched_after_second | initial_ood_evaluations | 0.0058 | 0.0047 | 0.5% |
| 0 | 2 | merkle_whir_batched_after_second | initial_rs_encoding | 0.5990 | 0.6017 | 47.3% |
| 0 | 2 | merkle_whir_batched_after_second | second_oracle_rs_and_merkle_commitment | 0.1955 | 0.1949 | 15.4% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | batched_recursive_whir | 0.2117 | 0.2106 | 10.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | first_transition_residual_after_subtracting_tail | 0.2967 | 0.2580 | 14.0% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | generalized_sz_cross_terms | 0.0542 | 0.0528 | 2.6% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_merkle_tree | 0.1563 | 0.1557 | 7.4% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_ood_evaluations | 0.0047 | 0.0047 | 0.2% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | initial_rs_encoding | 0.9825 | 0.9779 | 46.3% |
| 0 | 2 | merkle_whir_batched_rate_quarter_after_second | second_oracle_rs_and_merkle_commitment | 0.4140 | 0.4153 | 19.5% |
| 3 | 2 | canopy_schwartz_zippel | batched_recursive_whir | 0.1599 | 0.1599 | 13.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_forest | 0.0276 | 0.0268 | 2.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_codeword_merkle_tree | 0.0280 | 0.0278 | 2.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_commitment_other | 0.0000 | 0.0000 | 0.0% |
| 3 | 2 | canopy_schwartz_zippel | canopy_hash_to_field | 0.0035 | 0.0034 | 0.3% |
| 3 | 2 | canopy_schwartz_zippel | canopy_rs_encoding | 0.1171 | 0.1175 | 9.5% |
| 3 | 2 | canopy_schwartz_zippel | generalized_sz_cross_terms | 0.0775 | 0.0783 | 6.3% |
| 3 | 2 | canopy_schwartz_zippel | outer_initial_whir_stage_after_ood | 0.4270 | 0.4524 | 34.8% |
| 3 | 2 | canopy_schwartz_zippel | outer_ood_evaluations | 0.0058 | 0.0047 | 0.5% |
| 3 | 2 | canopy_schwartz_zippel | outer_rs_encoding | 0.3794 | 0.3794 | 30.9% |
