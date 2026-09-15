# Bolt-max prover stage profile

Input 2^22, width 8, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k/4 and parity k/4; total k/2, 40% of Bolt's encoded rows. Heights start at 3 for systematic and 1 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 118.000333 | 118.882679 | +0.882346 |
| Bolt | parity RS encoding Y=RS(U) | 56.155404 | 55.790442 | -0.364962 |
| X | row Merkle tree or forest | 71.198479 | 69.376679 | -1.821800 |
| X | Merkle root transcript | 0.000221 | -0.000000 | -0.000221 |
| Y | row Merkle tree or forest | 18.365662 | 18.376246 | +0.010583 |
| Y | Merkle root transcript | 0.000896 | -0.000000 | -0.000896 |
| other | unassigned phase work and timing overhead | 0.001180 | 0.001604 | +0.000424 |
| X | Canopy hash-to-field | -0.000000 | 12.044071 | +12.044071 |
| X | Canopy WHIR configuration | -0.000000 | 0.005283 | +0.005283 |
| X | Canopy RS encode and WHIR oracle commitment | -0.000000 | 7.192450 | +7.192450 |
| Y | Canopy hash-to-field | -0.000000 | 0.000588 | +0.000588 |
| Y | Canopy constant RS commitment | -0.000000 | 0.004479 | +0.004479 |
| **Subtotal** | **commitment** | **263.722175** | **281.674521** | **+17.952346** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 90.223038 | 89.635362 | -0.587675 |
| Bolt | column projections x=Xr and u=Ur | 17.462542 | 17.138550 | -0.323991 |
| common WHIR | WHIR configuration | 0.008513 | 0.007763 | -0.000750 |
| x | common WHIR RS encode and oracle commitment | 57.356187 | 56.274079 | -1.082108 |
| u | common WHIR RS encode and oracle commitment | 6.984233 | 6.923033 | -0.061200 |
| Bolt | syndrome evaluation sigma | 1.946633 | 1.979533 | +0.032900 |
| Bolt | sample row queries | 6.532917 | 6.558646 | +0.025729 |
| X | copy and serialize queried row values | 30.481188 | 30.523138 | +0.041950 |
| X | collect and serialize row authentication paths | 9.601921 | 6.065800 | -3.536121 |
| Y | copy and serialize queried row values | 0.462629 | 0.465933 | +0.003304 |
| Y | collect and serialize row authentication paths | 0.375700 | 0.300854 | -0.074846 |
| Bolt | project queried rows and sample batching challenges | 3.415509 | 3.329013 | -0.086496 |
| Bolt | systematic query covector | 2.488950 | 2.483562 | -0.005387 |
| Bolt | parity query covector and transpose RS FFT | 7.797687 | 7.784366 | -0.013321 |
| Bolt | public H transpose covector | 31.229808 | 31.210629 | -0.019179 |
| x | common WHIR constrained proof | 105.302712 | 105.970204 | +0.667492 |
| u | common WHIR constrained proof | 15.127900 | 14.936746 | -0.191154 |
| other | unassigned phase work and timing overhead | 0.629779 | 0.701233 | +0.071455 |
| X | Canopy ancestor claims | -0.000000 | 0.093067 | +0.093067 |
| X | Canopy SEP batching covector | -0.000000 | 0.794775 | +0.794775 |
| X | Canopy OOD evaluations | -0.000000 | 0.461787 | +0.461787 |
| X | Canopy WHIR SEP proof | -0.000000 | 12.802125 | +12.802125 |
| Y | Canopy ancestor claims | -0.000000 | 0.000683 | +0.000683 |
| Y | Canopy constant opening | -0.000000 | 0.000271 | +0.000271 |
| **Subtotal** | **opening** | **387.427846** | **396.441154** | **+9.013308** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 651.150021 | 648.922521 | 642.672710 | 666.722041 | 7.156579 | 263.722175 | 387.427846 | 10 |
| bolt_max_canopy_equal_blocks | 678.115675 | 677.613812 | 671.662625 | 688.423750 | 5.407893 | 281.674521 | 396.441154 | 10 |
