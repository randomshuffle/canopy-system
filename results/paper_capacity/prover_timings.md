# Conjectural capacity-bound paper prover timings

All WHIR proofs are generated and verifier-checked at 128-bit security over the 256-bit FFT field, in capacity_conjecture mode, with no proof of work. Capacity-bound soundness relies on STIR Conjecture 5.6 and is not proved. Input generation, verification, and deallocation are excluded. Hybrid totals are sums of the measured stages listed below.

| experiment | scheme | batch | h | repetitions | mean s | median s | vs baseline |
|---:|---|---:|---:|---:|---:|---:|---:|
| 1 | merkle | 128 | - | 20 | 0.0671 | 0.0663 | 1.000x |
| 1 | canopy | 128 | 16 | 20 | 0.1474 | 0.1441 | 2.198x |
| 1 | merkle | 256 | - | 20 | 0.0689 | 0.0664 | 1.000x |
| 1 | canopy | 256 | 5 | 20 | 0.1867 | 0.1854 | 2.711x |
| 1 | merkle | 512 | - | 20 | 0.0664 | 0.0663 | 1.000x |
| 1 | canopy | 512 | 3 | 20 | 0.2208 | 0.2203 | 3.325x |
| 1 | merkle | 1024 | - | 20 | 0.0668 | 0.0664 | 1.000x |
| 1 | canopy | 1024 | 2 | 20 | 0.2589 | 0.2581 | 3.873x |
| 2 | merkle_whir | - | - | 20 | 0.2781 | 0.2770 | 1.000x |
| 2 | canopy_separate | - | 5 | 20 | 0.6050 | 0.6058 | 2.176x |
| 3 | canopy_batched | - | 5 | 20 | 0.5723 | 0.5722 | 2.058x |

## Mean stage breakdown

| experiment | scheme | batch | stage | mean s | median s | share |
|---:|---|---:|---|---:|---:|---:|
| 1 | merkle | 128 | batch_opening | 0.0001 | 0.0001 | 0.1% |
| 1 | merkle | 128 | commitment | 0.0670 | 0.0662 | 99.9% |
| 1 | canopy | 128 | bottom_batch_opening | 0.0001 | 0.0001 | 0.1% |
| 1 | canopy | 128 | canopy_commitment | 0.1472 | 0.1439 | 99.9% |
| 1 | canopy | 128 | whir_claim_prover | 0.0001 | 0.0001 | 0.1% |
| 1 | merkle | 256 | batch_opening | 0.0002 | 0.0002 | 0.2% |
| 1 | merkle | 256 | commitment | 0.0687 | 0.0663 | 99.8% |
| 1 | canopy | 256 | bottom_batch_opening | 0.0001 | 0.0001 | 0.1% |
| 1 | canopy | 256 | canopy_commitment | 0.1589 | 0.1586 | 85.1% |
| 1 | canopy | 256 | whir_claim_prover | 0.0278 | 0.0266 | 14.9% |
| 1 | merkle | 512 | batch_opening | 0.0003 | 0.0003 | 0.5% |
| 1 | merkle | 512 | commitment | 0.0661 | 0.0660 | 99.5% |
| 1 | canopy | 512 | bottom_batch_opening | 0.0001 | 0.0001 | 0.1% |
| 1 | canopy | 512 | canopy_commitment | 0.1789 | 0.1782 | 81.0% |
| 1 | canopy | 512 | whir_claim_prover | 0.0418 | 0.0417 | 18.9% |
| 1 | merkle | 1024 | batch_opening | 0.0006 | 0.0006 | 0.9% |
| 1 | merkle | 1024 | commitment | 0.0662 | 0.0658 | 99.1% |
| 1 | canopy | 1024 | bottom_batch_opening | 0.0002 | 0.0002 | 0.1% |
| 1 | canopy | 1024 | canopy_commitment | 0.2063 | 0.2058 | 79.7% |
| 1 | canopy | 1024 | whir_claim_prover | 0.0523 | 0.0522 | 20.2% |
| 2 | merkle_whir | - | initial_commitment | 0.1140 | 0.1142 | 41.0% |
| 2 | merkle_whir | - | whir_opening_proof | 0.1641 | 0.1629 | 59.0% |
| 2 | canopy_separate | - | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 2 | canopy_separate | - | canopy_claim_whir | 0.0360 | 0.0361 | 5.9% |
| 2 | canopy_separate | - | canopy_commitment | 0.2324 | 0.2317 | 38.4% |
| 2 | canopy_separate | - | outer_initial_whir_stage | 0.1286 | 0.1282 | 21.3% |
| 2 | canopy_separate | - | outer_ood_evaluations | 0.0193 | 0.0193 | 3.2% |
| 2 | canopy_separate | - | outer_rs_encoding | 0.1530 | 0.1515 | 25.3% |
| 2 | canopy_separate | - | outer_second_oracle_whir | 0.0355 | 0.0355 | 5.9% |
| 3 | canopy_batched | - | batched_aligned_claims_whir | 0.0387 | 0.0384 | 6.8% |
| 3 | canopy_batched | - | canopy_bottom_opening | 0.0002 | 0.0002 | 0.0% |
| 3 | canopy_batched | - | canopy_commitment | 0.2324 | 0.2317 | 40.6% |
| 3 | canopy_batched | - | outer_initial_whir_stage | 0.1286 | 0.1282 | 22.5% |
| 3 | canopy_batched | - | outer_ood_evaluations | 0.0193 | 0.0193 | 3.4% |
| 3 | canopy_batched | - | outer_rs_encoding | 0.1530 | 0.1515 | 26.7% |
