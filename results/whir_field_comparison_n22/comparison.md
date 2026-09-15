# WHIR/Canopy: GF(2^256) versus the 256-bit prime

Matched fresh runs on **Apple M2, 16 GiB RAM**, at **2^22 field elements per polynomial**, **t=1,2,4,8**, eight Rayon threads, BLAKE3-256, 128-bit Johnson parameters, no PoW, one warmup and five measured repetitions. Both fields use 32-byte elements and deterministic full-width inputs and claim points.

**Timing boundary:** these reproduce the existing paper's component-composed prover benchmark. Commitment and opening are both included. Executable WHIR components are verified, while independent first transitions are measured once and scaled by t; batched tails and cross terms execute at their reported t. A separately measured native suffix is subtracted to model termination at at most 2^10 coefficients. These totals are not stopwatch measurements of a single end-to-end Canopy proof. Setup/input generation, verification and final deallocation are excluded.

The initial/recursive executable folding log is 3 (factor 8). The existing n=22 proof-size grid selects h=9 for independent Canopy; its additional Canopy WHIR tail is included. Batched Canopy aligns its two source types at h=4. Heights and geometry match across fields.

## Median total prover times

| t | Scheme | Prime s | GF(2^256) s | Binary/prime |
|---:|---|---:|---:|---:|
| 1 | Merkle-WHIR, independent | 1.386 | 0.772 | 0.557x |
| 1 | Merkle-WHIR, batched, rate 1/2 | 1.386 | 0.772 | 0.557x |
| 1 | Merkle-WHIR, batched, rate 1/4 | 2.515 | 1.302 | 0.518x |
| 1 | Canopy, independent tails, h=9 | 2.027 | 1.120 | 0.552x |
| 1 | Canopy, batched tails, h=4 | 2.060 | 1.104 | 0.536x |
| 2 | Merkle-WHIR, independent | 2.772 | 1.543 | 0.557x |
| 2 | Merkle-WHIR, batched, rate 1/2 | 2.654 | 1.451 | 0.547x |
| 2 | Merkle-WHIR, batched, rate 1/4 | 4.757 | 2.412 | 0.507x |
| 2 | Canopy, independent tails, h=9 | 4.055 | 2.240 | 0.552x |
| 2 | Canopy, batched tails, h=4 | 3.579 | 1.911 | 0.534x |
| 4 | Merkle-WHIR, independent | 5.545 | 3.086 | 0.557x |
| 4 | Merkle-WHIR, batched, rate 1/2 | 5.022 | 2.790 | 0.556x |
| 4 | Merkle-WHIR, batched, rate 1/4 | 8.985 | 4.635 | 0.516x |
| 4 | Canopy, independent tails, h=9 | 8.109 | 4.480 | 0.552x |
| 4 | Canopy, batched tails, h=4 | 7.041 | 3.631 | 0.516x |
| 8 | Merkle-WHIR, independent | 11.089 | 6.173 | 0.557x |
| 8 | Merkle-WHIR, batched, rate 1/2 | 10.164 | 6.370 | 0.627x |
| 8 | Merkle-WHIR, batched, rate 1/4 | 17.854 | 9.280 | 0.520x |
| 8 | Canopy, independent tails, h=9 | 16.218 | 8.960 | 0.552x |
| 8 | Canopy, batched tails, h=4 | 14.995 | 7.673 | 0.512x |

A slower binary repetition noticeably affects the means. Medians retain all samples and reduce that influence; raw measurements are linked below. These are five-repeat measurements, not confidence bounds.

## Mean commitment + opening times

| t | Scheme | Prime commit s | Binary commit s | Prime opening s | Binary opening s | Prime total s | Binary total s | Binary/prime total |
|---:|---|---:|---:|---:|---:|---:|---:|---:|
| 1 | Merkle-WHIR, independent | 0.627 | 0.353 | 0.756 | 0.488 | 1.383 | 0.841 | 0.608x |
| 1 | Merkle-WHIR, batched, rate 1/2 | 0.627 | 0.353 | 0.756 | 0.488 | 1.383 | 0.841 | 0.608x |
| 1 | Merkle-WHIR, batched, rate 1/4 | 1.241 | 0.647 | 1.286 | 0.678 | 2.527 | 1.325 | 0.524x |
| 1 | Canopy, independent tails, h=9 | 1.222 | 0.563 | 0.990 | 0.633 | 2.212 | 1.196 | 0.540x |
| 1 | Canopy, batched tails, h=4 | 1.265 | 0.625 | 0.876 | 0.534 | 2.141 | 1.159 | 0.541x |
| 2 | Merkle-WHIR, independent | 1.255 | 0.705 | 1.511 | 0.976 | 2.766 | 1.681 | 0.608x |
| 2 | Merkle-WHIR, batched, rate 1/2 | 1.255 | 0.705 | 1.370 | 0.881 | 2.625 | 1.586 | 0.604x |
| 2 | Merkle-WHIR, batched, rate 1/4 | 2.482 | 1.293 | 2.289 | 1.170 | 4.771 | 2.463 | 0.516x |
| 2 | Canopy, independent tails, h=9 | 2.444 | 1.125 | 1.980 | 1.266 | 4.424 | 2.391 | 0.540x |
| 2 | Canopy, batched tails, h=4 | 2.449 | 1.179 | 1.325 | 0.853 | 3.774 | 2.032 | 0.538x |
| 4 | Merkle-WHIR, independent | 2.509 | 1.410 | 3.023 | 1.953 | 5.532 | 3.363 | 0.608x |
| 4 | Merkle-WHIR, batched, rate 1/2 | 2.509 | 1.410 | 2.493 | 1.665 | 5.002 | 3.075 | 0.615x |
| 4 | Merkle-WHIR, batched, rate 1/4 | 4.964 | 2.586 | 4.108 | 2.157 | 9.072 | 4.743 | 0.523x |
| 4 | Canopy, independent tails, h=9 | 4.888 | 2.251 | 3.960 | 2.531 | 8.848 | 4.782 | 0.540x |
| 4 | Canopy, batched tails, h=4 | 4.750 | 2.342 | 2.313 | 1.742 | 7.064 | 4.084 | 0.578x |
| 8 | Merkle-WHIR, independent | 5.018 | 2.820 | 6.045 | 3.905 | 11.064 | 6.725 | 0.608x |
| 8 | Merkle-WHIR, batched, rate 1/2 | 5.018 | 2.820 | 5.058 | 3.600 | 10.076 | 6.420 | 0.637x |
| 8 | Merkle-WHIR, batched, rate 1/4 | 9.928 | 5.173 | 8.054 | 4.296 | 17.982 | 9.469 | 0.527x |
| 8 | Canopy, independent tails, h=9 | 9.775 | 4.501 | 7.921 | 5.063 | 17.696 | 9.564 | 0.540x |
| 8 | Canopy, batched tails, h=4 | 9.837 | 4.811 | 5.015 | 3.703 | 14.852 | 8.514 | 0.573x |

## Canopy overhead within each field (mean totals)

| t | Canopy variant | Prime Canopy / corresponding Merkle | Binary Canopy / corresponding Merkle |
|---:|---|---:|---:|
| 1 | Canopy, independent tails, h=9 | 1.599x | 1.422x |
| 1 | Canopy, batched tails, h=4 | 1.548x | 1.379x |
| 2 | Canopy, independent tails, h=9 | 1.599x | 1.422x |
| 2 | Canopy, batched tails, h=4 | 1.438x | 1.281x |
| 4 | Canopy, independent tails, h=9 | 1.599x | 1.422x |
| 4 | Canopy, batched tails, h=4 | 1.412x | 1.328x |
| 8 | Canopy, independent tails, h=9 | 1.599x | 1.422x |
| 8 | Canopy, batched tails, h=4 | 1.474x | 1.326x |

## Stage comparison

| t | Scheme | Stage | Prime s | Binary s | Binary/prime |
|---:|---|---|---:|---:|---:|
| 1 | Merkle-WHIR, independent | initial_merkle_tree | 0.04358 | 0.03710 | 0.851x |
| 1 | Merkle-WHIR, independent | initial_ood_evaluations | 0.01787 | 0.01284 | 0.719x |
| 1 | Merkle-WHIR, independent | initial_rs_encoding | 0.58370 | 0.31540 | 0.540x |
| 1 | Merkle-WHIR, independent | whir_opening_proof_after_ood | 0.73779 | 0.47534 | 0.644x |
| 1 | Merkle-WHIR, batched, rate 1/2 | first_transition_residual_after_subtracting_tail | 0.23437 | 0.21708 | 0.926x |
| 1 | Merkle-WHIR, batched, rate 1/2 | generalized_sz_cross_terms | 0.00000 | 0.00000 | — |
| 1 | Merkle-WHIR, batched, rate 1/2 | initial_merkle_tree | 0.04358 | 0.03710 | 0.851x |
| 1 | Merkle-WHIR, batched, rate 1/2 | initial_ood_evaluations | 0.01787 | 0.01284 | 0.719x |
| 1 | Merkle-WHIR, batched, rate 1/2 | initial_rs_encoding | 0.58370 | 0.31540 | 0.540x |
| 1 | Merkle-WHIR, batched, rate 1/2 | recursive_tail_after_ood | 0.24898 | 0.13202 | 0.530x |
| 1 | Merkle-WHIR, batched, rate 1/2 | recursive_tail_ood_evaluations | 0.00241 | 0.00172 | 0.712x |
| 1 | Merkle-WHIR, batched, rate 1/2 | second_oracle_rs_and_merkle_commitment | 0.25203 | 0.12452 | 0.494x |
| 1 | Merkle-WHIR, batched, rate 1/4 | first_transition_residual_after_subtracting_tail | 0.32463 | 0.18467 | 0.569x |
| 1 | Merkle-WHIR, batched, rate 1/4 | generalized_sz_cross_terms | 0.00000 | 0.00000 | — |
| 1 | Merkle-WHIR, batched, rate 1/4 | initial_merkle_tree | 0.08610 | 0.07474 | 0.868x |
| 1 | Merkle-WHIR, batched, rate 1/4 | initial_ood_evaluations | 0.01813 | 0.01309 | 0.722x |
| 1 | Merkle-WHIR, batched, rate 1/4 | initial_rs_encoding | 1.15484 | 0.57185 | 0.495x |
| 1 | Merkle-WHIR, batched, rate 1/4 | recursive_tail_after_ood | 0.41692 | 0.21876 | 0.525x |
| 1 | Merkle-WHIR, batched, rate 1/4 | recursive_tail_ood_evaluations | 0.00262 | 0.00182 | 0.696x |
| 1 | Merkle-WHIR, batched, rate 1/4 | second_oracle_rs_and_merkle_commitment | 0.52407 | 0.26005 | 0.496x |
| 1 | Canopy, independent tails, h=9 | canopy_bottom_forest | 0.04278 | 0.03797 | 0.887x |
| 1 | Canopy, independent tails, h=9 | canopy_bottom_opening | 0.00016 | 0.00017 | 1.060x |
| 1 | Canopy, independent tails, h=9 | canopy_claim_whirs | 0.23427 | 0.14448 | 0.617x |
| 1 | Canopy, independent tails, h=9 | canopy_codeword_merkle_tree | 0.04587 | 0.04015 | 0.875x |
| 1 | Canopy, independent tails, h=9 | canopy_commitment_other | 0.00000 | 0.00000 | 1.218x |
| 1 | Canopy, independent tails, h=9 | canopy_hash_to_field | 0.00062 | 0.00076 | 1.227x |
| 1 | Canopy, independent tails, h=9 | canopy_rs_encoding | 0.54893 | 0.16839 | 0.307x |
| 1 | Canopy, independent tails, h=9 | outer_initial_whir_stage_after_ood | 0.29112 | 0.25755 | 0.885x |
| 1 | Canopy, independent tails, h=9 | outer_ood_evaluations | 0.01787 | 0.01284 | 0.719x |
| 1 | Canopy, independent tails, h=9 | outer_rs_encoding | 0.58370 | 0.31540 | 0.540x |
| 1 | Canopy, independent tails, h=9 | outer_second_oracle_whirs | 0.44667 | 0.21779 | 0.488x |
| 1 | Canopy, batched tails, h=4 | batched_recursive_whir | 0.51862 | 0.23937 | 0.462x |
| 1 | Canopy, batched tails, h=4 | canopy_bottom_forest | 0.04312 | 0.03139 | 0.728x |
| 1 | Canopy, batched tails, h=4 | canopy_bottom_opening | 0.00019 | 0.00029 | 1.526x |
| 1 | Canopy, batched tails, h=4 | canopy_codeword_merkle_tree | 0.04644 | 0.04090 | 0.881x |
| 1 | Canopy, batched tails, h=4 | canopy_commitment_other | 0.00000 | 0.00000 | 1.184x |
| 1 | Canopy, batched tails, h=4 | canopy_hash_to_field | 0.01842 | 0.01598 | 0.868x |
| 1 | Canopy, batched tails, h=4 | canopy_rs_encoding | 0.51577 | 0.21587 | 0.419x |
| 1 | Canopy, batched tails, h=4 | generalized_sz_cross_terms | 0.04803 | 0.02421 | 0.504x |
| 1 | Canopy, batched tails, h=4 | outer_initial_whir_stage_after_ood | 0.29112 | 0.25755 | 0.885x |
| 1 | Canopy, batched tails, h=4 | outer_ood_evaluations | 0.01787 | 0.01284 | 0.719x |
| 1 | Canopy, batched tails, h=4 | outer_rs_encoding | 0.64118 | 0.32069 | 0.500x |
| 2 | Merkle-WHIR, independent | initial_merkle_tree | 0.08717 | 0.07421 | 0.851x |
| 2 | Merkle-WHIR, independent | initial_ood_evaluations | 0.03574 | 0.02568 | 0.719x |
| 2 | Merkle-WHIR, independent | initial_rs_encoding | 1.16739 | 0.63080 | 0.540x |
| 2 | Merkle-WHIR, independent | whir_opening_proof_after_ood | 1.47558 | 0.95068 | 0.644x |
| 2 | Merkle-WHIR, batched, rate 1/2 | batched_recursive_whir | 0.31434 | 0.14760 | 0.470x |
| 2 | Merkle-WHIR, batched, rate 1/2 | first_transition_residual_after_subtracting_tail | 0.46873 | 0.43415 | 0.926x |
| 2 | Merkle-WHIR, batched, rate 1/2 | generalized_sz_cross_terms | 0.04749 | 0.02463 | 0.519x |
| 2 | Merkle-WHIR, batched, rate 1/2 | initial_merkle_tree | 0.08717 | 0.07421 | 0.851x |
| 2 | Merkle-WHIR, batched, rate 1/2 | initial_ood_evaluations | 0.03574 | 0.02568 | 0.719x |
| 2 | Merkle-WHIR, batched, rate 1/2 | initial_rs_encoding | 1.16739 | 0.63080 | 0.540x |
| 2 | Merkle-WHIR, batched, rate 1/2 | second_oracle_rs_and_merkle_commitment | 0.50406 | 0.24905 | 0.494x |
| 2 | Merkle-WHIR, batched, rate 1/4 | batched_recursive_whir | 0.50783 | 0.22991 | 0.453x |
| 2 | Merkle-WHIR, batched, rate 1/4 | first_transition_residual_after_subtracting_tail | 0.64925 | 0.36935 | 0.569x |
| 2 | Merkle-WHIR, batched, rate 1/4 | generalized_sz_cross_terms | 0.04749 | 0.02463 | 0.519x |
| 2 | Merkle-WHIR, batched, rate 1/4 | initial_merkle_tree | 0.17219 | 0.14949 | 0.868x |
| 2 | Merkle-WHIR, batched, rate 1/4 | initial_ood_evaluations | 0.03625 | 0.02619 | 0.722x |
| 2 | Merkle-WHIR, batched, rate 1/4 | initial_rs_encoding | 2.30969 | 1.14370 | 0.495x |
| 2 | Merkle-WHIR, batched, rate 1/4 | second_oracle_rs_and_merkle_commitment | 1.04814 | 0.52011 | 0.496x |
| 2 | Canopy, independent tails, h=9 | canopy_bottom_forest | 0.08556 | 0.07593 | 0.887x |
| 2 | Canopy, independent tails, h=9 | canopy_bottom_opening | 0.00033 | 0.00034 | 1.060x |
| 2 | Canopy, independent tails, h=9 | canopy_claim_whirs | 0.46854 | 0.28895 | 0.617x |
| 2 | Canopy, independent tails, h=9 | canopy_codeword_merkle_tree | 0.09174 | 0.08031 | 0.875x |
| 2 | Canopy, independent tails, h=9 | canopy_commitment_other | 0.00000 | 0.00000 | 1.218x |
| 2 | Canopy, independent tails, h=9 | canopy_hash_to_field | 0.00124 | 0.00153 | 1.227x |
| 2 | Canopy, independent tails, h=9 | canopy_rs_encoding | 1.09785 | 0.33678 | 0.307x |
| 2 | Canopy, independent tails, h=9 | outer_initial_whir_stage_after_ood | 0.58224 | 0.51511 | 0.885x |
| 2 | Canopy, independent tails, h=9 | outer_ood_evaluations | 0.03574 | 0.02568 | 0.719x |
| 2 | Canopy, independent tails, h=9 | outer_rs_encoding | 1.16739 | 0.63080 | 0.540x |
| 2 | Canopy, independent tails, h=9 | outer_second_oracle_whirs | 0.89334 | 0.43558 | 0.488x |
| 2 | Canopy, batched tails, h=4 | batched_recursive_whir | 0.61501 | 0.26365 | 0.429x |
| 2 | Canopy, batched tails, h=4 | canopy_bottom_forest | 0.08367 | 0.06184 | 0.739x |
| 2 | Canopy, batched tails, h=4 | canopy_bottom_opening | 0.00026 | 0.00045 | 1.699x |
| 2 | Canopy, batched tails, h=4 | canopy_codeword_merkle_tree | 0.09090 | 0.06959 | 0.766x |
| 2 | Canopy, batched tails, h=4 | canopy_commitment_other | 0.00000 | 0.00000 | 0.945x |
| 2 | Canopy, batched tails, h=4 | canopy_hash_to_field | 0.03964 | 0.03185 | 0.804x |
| 2 | Canopy, batched tails, h=4 | canopy_rs_encoding | 0.99707 | 0.41408 | 0.415x |
| 2 | Canopy, batched tails, h=4 | generalized_sz_cross_terms | 0.09198 | 0.04801 | 0.522x |
| 2 | Canopy, batched tails, h=4 | outer_initial_whir_stage_after_ood | 0.58224 | 0.51511 | 0.885x |
| 2 | Canopy, batched tails, h=4 | outer_ood_evaluations | 0.03574 | 0.02568 | 0.719x |
| 2 | Canopy, batched tails, h=4 | outer_rs_encoding | 1.23729 | 0.60148 | 0.486x |
| 4 | Merkle-WHIR, independent | initial_merkle_tree | 0.17434 | 0.14841 | 0.851x |
| 4 | Merkle-WHIR, independent | initial_ood_evaluations | 0.07148 | 0.05136 | 0.719x |
| 4 | Merkle-WHIR, independent | initial_rs_encoding | 2.33479 | 1.26159 | 0.540x |
| 4 | Merkle-WHIR, independent | whir_opening_proof_after_ood | 2.95116 | 1.90137 | 0.644x |
| 4 | Merkle-WHIR, batched, rate 1/2 | batched_recursive_whir | 0.38481 | 0.19623 | 0.510x |
| 4 | Merkle-WHIR, batched, rate 1/2 | first_transition_residual_after_subtracting_tail | 0.93747 | 0.86830 | 0.926x |
| 4 | Merkle-WHIR, batched, rate 1/2 | generalized_sz_cross_terms | 0.09143 | 0.05074 | 0.555x |
| 4 | Merkle-WHIR, batched, rate 1/2 | initial_merkle_tree | 0.17434 | 0.14841 | 0.851x |
| 4 | Merkle-WHIR, batched, rate 1/2 | initial_ood_evaluations | 0.07148 | 0.05136 | 0.719x |
| 4 | Merkle-WHIR, batched, rate 1/2 | initial_rs_encoding | 2.33479 | 1.26159 | 0.540x |
| 4 | Merkle-WHIR, batched, rate 1/2 | second_oracle_rs_and_merkle_commitment | 1.00811 | 0.49810 | 0.494x |
| 4 | Merkle-WHIR, batched, rate 1/4 | batched_recursive_whir | 0.54921 | 0.27480 | 0.500x |
| 4 | Merkle-WHIR, batched, rate 1/4 | first_transition_residual_after_subtracting_tail | 1.29851 | 0.73869 | 0.569x |
| 4 | Merkle-WHIR, batched, rate 1/4 | generalized_sz_cross_terms | 0.09143 | 0.05074 | 0.555x |
| 4 | Merkle-WHIR, batched, rate 1/4 | initial_merkle_tree | 0.34439 | 0.29897 | 0.868x |
| 4 | Merkle-WHIR, batched, rate 1/4 | initial_ood_evaluations | 0.07250 | 0.05237 | 0.722x |
| 4 | Merkle-WHIR, batched, rate 1/4 | initial_rs_encoding | 4.61937 | 2.28739 | 0.495x |
| 4 | Merkle-WHIR, batched, rate 1/4 | second_oracle_rs_and_merkle_commitment | 2.09627 | 1.04021 | 0.496x |
| 4 | Canopy, independent tails, h=9 | canopy_bottom_forest | 0.17112 | 0.15186 | 0.887x |
| 4 | Canopy, independent tails, h=9 | canopy_bottom_opening | 0.00065 | 0.00069 | 1.060x |
| 4 | Canopy, independent tails, h=9 | canopy_claim_whirs | 0.93709 | 0.57790 | 0.617x |
| 4 | Canopy, independent tails, h=9 | canopy_codeword_merkle_tree | 0.18349 | 0.16062 | 0.875x |
| 4 | Canopy, independent tails, h=9 | canopy_commitment_other | 0.00000 | 0.00001 | 1.218x |
| 4 | Canopy, independent tails, h=9 | canopy_hash_to_field | 0.00249 | 0.00305 | 1.227x |
| 4 | Canopy, independent tails, h=9 | canopy_rs_encoding | 2.19571 | 0.67355 | 0.307x |
| 4 | Canopy, independent tails, h=9 | outer_initial_whir_stage_after_ood | 1.16448 | 1.03021 | 0.885x |
| 4 | Canopy, independent tails, h=9 | outer_ood_evaluations | 0.07148 | 0.05136 | 0.719x |
| 4 | Canopy, independent tails, h=9 | outer_rs_encoding | 2.33479 | 1.26159 | 0.540x |
| 4 | Canopy, independent tails, h=9 | outer_second_oracle_whirs | 1.78668 | 0.87116 | 0.488x |
| 4 | Canopy, batched tails, h=4 | batched_recursive_whir | 0.79533 | 0.49727 | 0.625x |
| 4 | Canopy, batched tails, h=4 | canopy_bottom_forest | 0.15786 | 0.12117 | 0.768x |
| 4 | Canopy, batched tails, h=4 | canopy_bottom_opening | 0.00045 | 0.00056 | 1.259x |
| 4 | Canopy, batched tails, h=4 | canopy_codeword_merkle_tree | 0.17303 | 0.14110 | 0.815x |
| 4 | Canopy, batched tails, h=4 | canopy_commitment_other | 0.00000 | 0.00000 | 0.929x |
| 4 | Canopy, batched tails, h=4 | canopy_hash_to_field | 0.07534 | 0.06443 | 0.855x |
| 4 | Canopy, batched tails, h=4 | canopy_rs_encoding | 1.87010 | 0.85246 | 0.456x |
| 4 | Canopy, batched tails, h=4 | generalized_sz_cross_terms | 0.28156 | 0.16237 | 0.577x |
| 4 | Canopy, batched tails, h=4 | outer_initial_whir_stage_after_ood | 1.16448 | 1.03021 | 0.885x |
| 4 | Canopy, batched tails, h=4 | outer_ood_evaluations | 0.07148 | 0.05136 | 0.719x |
| 4 | Canopy, batched tails, h=4 | outer_rs_encoding | 2.47402 | 1.16275 | 0.470x |
| 8 | Merkle-WHIR, independent | initial_merkle_tree | 0.34867 | 0.29683 | 0.851x |
| 8 | Merkle-WHIR, independent | initial_ood_evaluations | 0.14295 | 0.10272 | 0.719x |
| 8 | Merkle-WHIR, independent | initial_rs_encoding | 4.66957 | 2.52319 | 0.540x |
| 8 | Merkle-WHIR, independent | whir_opening_proof_after_ood | 5.90232 | 3.80274 | 0.644x |
| 8 | Merkle-WHIR, batched, rate 1/2 | batched_recursive_whir | 0.74423 | 0.60542 | 0.813x |
| 8 | Merkle-WHIR, batched, rate 1/2 | first_transition_residual_after_subtracting_tail | 1.87493 | 1.73660 | 0.926x |
| 8 | Merkle-WHIR, batched, rate 1/2 | generalized_sz_cross_terms | 0.27967 | 0.15917 | 0.569x |
| 8 | Merkle-WHIR, batched, rate 1/2 | initial_merkle_tree | 0.34867 | 0.29683 | 0.851x |
| 8 | Merkle-WHIR, batched, rate 1/2 | initial_ood_evaluations | 0.14295 | 0.10272 | 0.719x |
| 8 | Merkle-WHIR, batched, rate 1/2 | initial_rs_encoding | 4.66957 | 2.52319 | 0.540x |
| 8 | Merkle-WHIR, batched, rate 1/2 | second_oracle_rs_and_merkle_commitment | 2.01623 | 0.99619 | 0.494x |
| 8 | Merkle-WHIR, batched, rate 1/4 | batched_recursive_whir | 0.84014 | 0.47425 | 0.564x |
| 8 | Merkle-WHIR, batched, rate 1/4 | first_transition_residual_after_subtracting_tail | 2.59701 | 1.47739 | 0.569x |
| 8 | Merkle-WHIR, batched, rate 1/4 | generalized_sz_cross_terms | 0.27967 | 0.15917 | 0.569x |
| 8 | Merkle-WHIR, batched, rate 1/4 | initial_merkle_tree | 0.68878 | 0.59794 | 0.868x |
| 8 | Merkle-WHIR, batched, rate 1/4 | initial_ood_evaluations | 0.14501 | 0.10474 | 0.722x |
| 8 | Merkle-WHIR, batched, rate 1/4 | initial_rs_encoding | 9.23874 | 4.57479 | 0.495x |
| 8 | Merkle-WHIR, batched, rate 1/4 | second_oracle_rs_and_merkle_commitment | 4.19255 | 2.08043 | 0.496x |
| 8 | Canopy, independent tails, h=9 | canopy_bottom_forest | 0.34225 | 0.30373 | 0.887x |
| 8 | Canopy, independent tails, h=9 | canopy_bottom_opening | 0.00130 | 0.00138 | 1.060x |
| 8 | Canopy, independent tails, h=9 | canopy_claim_whirs | 1.87418 | 1.15580 | 0.617x |
| 8 | Canopy, independent tails, h=9 | canopy_codeword_merkle_tree | 0.36698 | 0.32124 | 0.875x |
| 8 | Canopy, independent tails, h=9 | canopy_commitment_other | 0.00001 | 0.00001 | 1.218x |
| 8 | Canopy, independent tails, h=9 | canopy_hash_to_field | 0.00498 | 0.00611 | 1.227x |
| 8 | Canopy, independent tails, h=9 | canopy_rs_encoding | 4.39141 | 1.34710 | 0.307x |
| 8 | Canopy, independent tails, h=9 | outer_initial_whir_stage_after_ood | 2.32895 | 2.06042 | 0.885x |
| 8 | Canopy, independent tails, h=9 | outer_ood_evaluations | 0.14295 | 0.10272 | 0.719x |
| 8 | Canopy, independent tails, h=9 | outer_rs_encoding | 4.66957 | 2.52319 | 0.540x |
| 8 | Canopy, independent tails, h=9 | outer_second_oracle_whirs | 3.57337 | 1.74232 | 0.488x |
| 8 | Canopy, batched tails, h=4 | batched_recursive_whir | 1.49625 | 0.94318 | 0.630x |
| 8 | Canopy, batched tails, h=4 | canopy_bottom_forest | 0.30368 | 0.23264 | 0.766x |
| 8 | Canopy, batched tails, h=4 | canopy_bottom_opening | 0.00369 | 0.00144 | 0.391x |
| 8 | Canopy, batched tails, h=4 | canopy_codeword_merkle_tree | 0.35142 | 0.27514 | 0.783x |
| 8 | Canopy, batched tails, h=4 | canopy_commitment_other | 0.00000 | 0.00000 | 1.005x |
| 8 | Canopy, batched tails, h=4 | canopy_hash_to_field | 0.14039 | 0.12408 | 0.884x |
| 8 | Canopy, batched tails, h=4 | canopy_rs_encoding | 4.63506 | 1.89612 | 0.409x |
| 8 | Canopy, batched tails, h=4 | generalized_sz_cross_terms | 1.04321 | 0.59525 | 0.571x |
| 8 | Canopy, batched tails, h=4 | outer_initial_whir_stage_after_ood | 2.32895 | 2.06042 | 0.885x |
| 8 | Canopy, batched tails, h=4 | outer_ood_evaluations | 0.14295 | 0.10272 | 0.719x |
| 8 | Canopy, batched tails, h=4 | outer_rs_encoding | 4.40614 | 2.28281 | 0.518x |

## Memory and reproducibility

| Field | Peak process RSS GiB | Process wall s |
|---|---:|---:|
| prime | 6.673 | 417.4 |
| binary | 7.225 | 218.4 |

RSS covers the whole process, including setup and verification, with all five schemes executed sequentially; it is not a per-scheme prover peak.

- [Prime raw samples](prime/prover_timing_samples.csv), [stages](prime/prover_timing_stages.csv), [command and executable hash](prime/process.json).
- [Binary raw samples](binary/prover_timing_samples.csv), [stages](binary/prover_timing_stages.csv), [command and executable hash](binary/process.json).
- [Machine-readable comparison](comparison.csv).
- [Field validation](field_validation.json), [machine](machine.json), and [matching dependency versions](common_dependencies.json).
- [Backend and reproduction instructions](../../experiments/binary-whir/README.md).

The prime field is p=2^256-43*2^32+1. The binary backend uses GF(2^128)[w]/(w^2+w+x^121), with the base polynomial x^128+x^7+x^2+x+1, PMULL multiplication and monomial RS evaluation via additive FFTs. WHIR query aggregation uses a transposed additive transform; the prime backend retains its sparse NTT optimization. This is a comparison of these implementations on this host, not a hardware-independent claim about field arithmetic.

The older 2^18 prime timings also used integer-pattern inputs and smaller claim points. The fresh 2^22 prime run is the denominator here; comparing directly with the old totals would mix input size, data and field changes. The 192-bit field-switch model is retired and its historical artifacts are archived. Bolt's field and experiment are unchanged.
