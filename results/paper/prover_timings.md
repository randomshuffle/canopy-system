# Final paper prover timings

All WHIR proofs are generated and verifier-checked at 128-bit security over the 256-bit FFT field, in Johnson mode, with no proof of work. Input generation, verification, and deallocation are excluded. Hybrid totals are sums of the measured stages listed below.

| experiment | scheme | batch | h | repetitions | mean s | median s | vs baseline |
|---:|---|---:|---:|---:|---:|---:|---:|
| 1 | merkle | 128 | - | 20 | 0.0727 | 0.0705 | 1.000x |
| 1 | canopy | 128 | 16 | 20 | 0.1578 | 0.1591 | 2.172x |
| 1 | merkle | 256 | - | 20 | 0.0649 | 0.0643 | 1.000x |
| 1 | canopy | 256 | 16 | 20 | 0.1408 | 0.1393 | 2.170x |
| 1 | merkle | 512 | - | 20 | 0.0648 | 0.0643 | 1.000x |
| 1 | canopy | 512 | 5 | 20 | 0.1791 | 0.1779 | 2.762x |
| 1 | merkle | 1024 | - | 20 | 0.0650 | 0.0644 | 1.000x |
| 1 | canopy | 1024 | 3 | 20 | 0.2171 | 0.2168 | 3.342x |
| 2 | merkle_whir | - | - | 20 | 0.3188 | 0.3166 | 1.000x |
| 2 | canopy_separate | - | 5 | 20 | 0.6346 | 0.6343 | 1.990x |
| 3 | canopy_batched | - | 5 | 20 | 0.5993 | 0.5982 | 1.880x |

## Mean stage breakdown

| experiment | scheme | batch | stage | mean s | median s | share |
|---:|---|---:|---|---:|---:|---:|
| 1 | merkle | 128 | batch_opening | 0.0001 | 0.0001 | 0.1% |
| 1 | merkle | 128 | commitment | 0.0726 | 0.0704 | 99.9% |
| 1 | canopy | 128 | bottom_batch_opening | 0.0001 | 0.0001 | 0.1% |
| 1 | canopy | 128 | canopy_commitment | 0.1576 | 0.1590 | 99.9% |
| 1 | canopy | 128 | whir_claim_prover | 0.0001 | 0.0001 | 0.1% |
| 1 | merkle | 256 | batch_opening | 0.0001 | 0.0001 | 0.2% |
| 1 | merkle | 256 | commitment | 0.0647 | 0.0641 | 99.8% |
| 1 | canopy | 256 | bottom_batch_opening | 0.0002 | 0.0002 | 0.1% |
| 1 | canopy | 256 | canopy_commitment | 0.1405 | 0.1390 | 99.8% |
| 1 | canopy | 256 | whir_claim_prover | 0.0001 | 0.0001 | 0.1% |
| 1 | merkle | 512 | batch_opening | 0.0003 | 0.0003 | 0.5% |
| 1 | merkle | 512 | commitment | 0.0646 | 0.0640 | 99.5% |
| 1 | canopy | 512 | bottom_batch_opening | 0.0002 | 0.0002 | 0.1% |
| 1 | canopy | 512 | canopy_commitment | 0.1522 | 0.1510 | 85.0% |
| 1 | canopy | 512 | whir_claim_prover | 0.0267 | 0.0266 | 14.9% |
| 1 | merkle | 1024 | batch_opening | 0.0006 | 0.0006 | 0.9% |
| 1 | merkle | 1024 | commitment | 0.0644 | 0.0638 | 99.1% |
| 1 | canopy | 1024 | bottom_batch_opening | 0.0002 | 0.0002 | 0.1% |
| 1 | canopy | 1024 | canopy_commitment | 0.1720 | 0.1717 | 79.2% |
| 1 | canopy | 1024 | whir_claim_prover | 0.0449 | 0.0448 | 20.7% |
| 2 | merkle_whir | - | initial_commitment | 0.1094 | 0.1087 | 34.3% |
| 2 | merkle_whir | - | whir_opening_proof | 0.2094 | 0.2077 | 65.7% |
| 2 | canopy_separate | - | canopy_bottom_opening | 0.0005 | 0.0004 | 0.1% |
| 2 | canopy_separate | - | canopy_claim_whir | 0.0384 | 0.0382 | 6.1% |
| 2 | canopy_separate | - | canopy_commitment | 0.2231 | 0.2223 | 35.2% |
| 2 | canopy_separate | - | outer_initial_whir_stage | 0.1715 | 0.1701 | 27.0% |
| 2 | canopy_separate | - | outer_ood_evaluations | 0.0188 | 0.0188 | 3.0% |
| 2 | canopy_separate | - | outer_rs_encoding | 0.1443 | 0.1433 | 22.7% |
| 2 | canopy_separate | - | outer_second_oracle_whir | 0.0379 | 0.0378 | 6.0% |
| 3 | canopy_batched | - | batched_aligned_claims_whir | 0.0410 | 0.0406 | 6.8% |
| 3 | canopy_batched | - | canopy_bottom_opening | 0.0005 | 0.0004 | 0.1% |
| 3 | canopy_batched | - | canopy_commitment | 0.2231 | 0.2223 | 37.2% |
| 3 | canopy_batched | - | outer_initial_whir_stage | 0.1715 | 0.1701 | 28.6% |
| 3 | canopy_batched | - | outer_ood_evaluations | 0.0188 | 0.0188 | 3.1% |
| 3 | canopy_batched | - | outer_rs_encoding | 0.1443 | 0.1433 | 24.1% |
