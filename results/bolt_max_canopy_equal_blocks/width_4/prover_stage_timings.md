# Bolt-max prover stage profile

Input 2^18, width 4, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k/4 and parity k/4; total k/2, 40% of Bolt's encoded rows. Heights start at 3 for systematic and 1 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 3.882854 | 3.581150 | -0.301704 |
| Bolt | parity RS encoding Y=RS(U) | 3.896142 | 3.688991 | -0.207150 |
| X | row Merkle tree or forest | 8.784737 | 6.999800 | -1.784937 |
| X | Merkle root transcript | 0.000238 | -0.000000 | -0.000238 |
| Y | row Merkle tree or forest | 2.608716 | 2.186754 | -0.421962 |
| Y | Merkle root transcript | 0.000608 | -0.000000 | -0.000608 |
| other | unassigned phase work and timing overhead | 0.001171 | 0.001117 | -0.000055 |
| X | Canopy hash-to-field | -0.000000 | 0.000971 | +0.000971 |
| X | Canopy constant RS commitment | -0.000000 | 0.005196 | +0.005196 |
| Y | Canopy hash-to-field | -0.000000 | 0.000392 | +0.000392 |
| Y | Canopy constant RS commitment | -0.000000 | 0.003675 | +0.003675 |
| **Subtotal** | **commitment** | **19.174467** | **16.468046** | **-2.706421** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 6.769454 | 5.997579 | -0.771875 |
| Bolt | column projections x=Xr and u=Ur | 1.655012 | 1.433446 | -0.221566 |
| common WHIR | WHIR configuration | 0.005258 | 0.005417 | +0.000159 |
| x | common WHIR RS encode and oracle commitment | 7.722817 | 7.326446 | -0.396371 |
| u | common WHIR RS encode and oracle commitment | 1.227592 | 1.191170 | -0.036421 |
| Bolt | syndrome evaluation sigma | 0.462650 | 0.455479 | -0.007171 |
| Bolt | sample row queries | 5.898829 | 5.818796 | -0.080033 |
| X | copy and serialize queried row values | 15.669121 | 15.012425 | -0.656696 |
| X | collect and serialize row authentication paths | 4.215075 | 4.138967 | -0.076108 |
| Y | copy and serialize queried row values | 0.238729 | 0.244379 | +0.005650 |
| Y | collect and serialize row authentication paths | 0.177983 | 0.172571 | -0.005413 |
| Bolt | project queried rows and sample batching challenges | 2.193808 | 1.720067 | -0.473742 |
| Bolt | systematic query covector | 0.982313 | 0.963504 | -0.018808 |
| Bolt | parity query covector and transpose RS FFT | 1.562846 | 1.535475 | -0.027371 |
| Bolt | public H transpose covector | 3.122667 | 3.083525 | -0.039142 |
| x | common WHIR constrained proof | 17.254279 | 16.770475 | -0.483804 |
| u | common WHIR constrained proof | 4.637383 | 4.046150 | -0.591233 |
| other | unassigned phase work and timing overhead | 0.170550 | 0.058588 | -0.111962 |
| X | Canopy ancestor claims | -0.000000 | 0.019383 | +0.019383 |
| X | Canopy constant opening | -0.000000 | 0.000396 | +0.000396 |
| Y | Canopy ancestor claims | -0.000000 | 0.000583 | +0.000583 |
| Y | Canopy constant opening | -0.000000 | 0.000271 | +0.000271 |
| **Subtotal** | **opening** | **73.966367** | **69.995092** | **-3.971275** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 93.140834 | 85.725042 | 82.065250 | 142.919666 | 18.419550 | 19.174467 | 73.966367 | 10 |
| bolt_max_canopy_equal_blocks | 86.463137 | 86.014375 | 81.124875 | 91.355583 | 3.319048 | 16.468046 | 69.995092 | 10 |
