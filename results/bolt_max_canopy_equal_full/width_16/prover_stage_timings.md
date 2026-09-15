# Bolt-max prover stage profile

Input 2^18, width 16, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k and parity k; total 2k, 160% of Bolt's encoded rows. Heights start at 1 for systematic and 0 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 4.023979 | 3.657404 | -0.366575 |
| Bolt | parity RS encoding Y=RS(U) | 3.391321 | 3.468592 | +0.077271 |
| X | row Merkle tree or forest | 7.539446 | 4.465808 | -3.073637 |
| X | Merkle root transcript | 0.000129 | -0.000000 | -0.000129 |
| Y | row Merkle tree or forest | 2.228029 | 1.405921 | -0.822108 |
| Y | Merkle root transcript | 0.000479 | -0.000000 | -0.000479 |
| other | unassigned phase work and timing overhead | 0.001208 | 0.001137 | -0.000071 |
| X | Canopy hash-to-field | -0.000000 | 0.000504 | +0.000504 |
| X | Canopy constant RS commitment | -0.000000 | 0.002950 | +0.002950 |
| Y | Canopy hash-to-field | -0.000000 | 0.000408 | +0.000408 |
| Y | Canopy constant RS commitment | -0.000000 | 0.004413 | +0.004413 |
| **Subtotal** | **commitment** | **17.184591** | **13.007138** | **-4.177454** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 6.418279 | 6.400454 | -0.017825 |
| Bolt | column projections x=Xr and u=Ur | 2.350458 | 1.371504 | -0.978954 |
| common WHIR | WHIR configuration | 0.005262 | 0.005462 | +0.000200 |
| x | common WHIR RS encode and oracle commitment | 1.920634 | 2.010588 | +0.089954 |
| u | common WHIR RS encode and oracle commitment | 0.645575 | 0.627067 | -0.018508 |
| Bolt | syndrome evaluation sigma | 0.133621 | 0.127967 | -0.005654 |
| Bolt | sample row queries | 0.088546 | 0.077092 | -0.011454 |
| X | copy and serialize queried row values | 47.663359 | 48.262208 | +0.598850 |
| X | collect and serialize row authentication paths | 1.531967 | 1.500729 | -0.031238 |
| Y | copy and serialize queried row values | 0.874296 | 0.890846 | +0.016550 |
| Y | collect and serialize row authentication paths | 0.105008 | 0.110321 | +0.005313 |
| Bolt | project queried rows and sample batching challenges | 6.084825 | 5.535042 | -0.549783 |
| Bolt | systematic query covector | 0.676029 | 0.679413 | +0.003383 |
| Bolt | parity query covector and transpose RS FFT | 0.786954 | 0.825100 | +0.038146 |
| Bolt | public H transpose covector | 0.764621 | 0.785658 | +0.021038 |
| x | common WHIR constrained proof | 7.108254 | 7.164196 | +0.055942 |
| u | common WHIR constrained proof | 1.646104 | 1.601371 | -0.044733 |
| other | unassigned phase work and timing overhead | 0.010412 | 0.032868 | +0.022456 |
| X | Canopy ancestor claims | -0.000000 | 0.017429 | +0.017429 |
| X | Canopy constant opening | -0.000000 | 0.000404 | +0.000404 |
| Y | Canopy ancestor claims | -0.000000 | 0.000608 | +0.000608 |
| Y | Canopy constant opening | -0.000000 | 0.000267 | +0.000267 |
| **Subtotal** | **opening** | **78.814204** | **78.026592** | **-0.787612** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 95.998796 | 90.263438 | 87.061959 | 143.554875 | 16.930151 | 17.184591 | 78.814204 | 10 |
| bolt_max_canopy_equal_full | 91.033729 | 90.925855 | 88.914584 | 93.782459 | 1.364252 | 13.007138 | 78.026592 | 10 |
