# Bolt-max prover stage profile

Input 2^22, width 4, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 165.949825 | 159.675384 | -6.274441 |
| Bolt | parity RS encoding Y=RS(U) | 74.067475 | 68.736158 | -5.331317 |
| X | row Merkle tree or forest | 102.769208 | 101.523696 | -1.245512 |
| X | Merkle root transcript | 0.000179 | -0.000000 | -0.000179 |
| Y | row Merkle tree or forest | 26.049088 | 28.548429 | +2.499342 |
| Y | Merkle root transcript | 0.001150 | -0.000000 | -0.001150 |
| other | unassigned phase work and timing overhead | 0.001471 | 0.002225 | +0.000754 |
| X | Canopy hash-to-field | -0.000000 | 25.026054 | +25.026054 |
| X | Canopy WHIR configuration | -0.000000 | 0.008925 | +0.008925 |
| X | Canopy RS encode and WHIR oracle commitment | -0.000000 | 14.779308 | +14.779308 |
| Y | Canopy hash-to-field | -0.000000 | 0.000675 | +0.000675 |
| Y | Canopy constant RS commitment | -0.000000 | 0.004333 | +0.004333 |
| **Subtotal** | **commitment** | **368.838396** | **398.305188** | **+29.466792** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 94.256913 | 96.661350 | +2.404437 |
| Bolt | column projections x=Xr and u=Ur | 17.702679 | 19.849346 | +2.146667 |
| common WHIR | WHIR configuration | 0.009863 | 0.010037 | +0.000175 |
| x | common WHIR RS encode and oracle commitment | 148.850708 | 155.487954 | +6.637246 |
| u | common WHIR RS encode and oracle commitment | 15.654000 | 16.288575 | +0.634575 |
| Bolt | syndrome evaluation sigma | 3.971929 | 4.056050 | +0.084121 |
| Bolt | sample row queries | 6.619675 | 6.760387 | +0.140712 |
| X | copy and serialize queried row values | 18.989908 | 17.281425 | -1.708483 |
| X | collect and serialize row authentication paths | 13.444017 | 8.338967 | -5.105050 |
| Y | copy and serialize queried row values | 0.262517 | 0.254563 | -0.007954 |
| Y | collect and serialize row authentication paths | 0.381408 | 0.571017 | +0.189608 |
| Bolt | project queried rows and sample batching challenges | 1.657450 | 1.793871 | +0.136421 |
| Bolt | systematic query covector | 3.961717 | 3.892363 | -0.069354 |
| Bolt | parity query covector and transpose RS FFT | 16.662537 | 18.007108 | +1.344571 |
| Bolt | public H transpose covector | 127.095433 | 135.832071 | +8.736637 |
| x | common WHIR constrained proof | 234.192775 | 235.726475 | +1.533700 |
| u | common WHIR constrained proof | 28.660717 | 30.117321 | +1.456604 |
| other | unassigned phase work and timing overhead | 1.129958 | 1.657629 | +0.527671 |
| X | Canopy ancestor claims | -0.000000 | 0.140992 | +0.140992 |
| X | Canopy SEP batching covector | -0.000000 | 1.044204 | +1.044204 |
| X | Canopy OOD evaluations | -0.000000 | 0.695913 | +0.695913 |
| X | Canopy WHIR SEP proof | -0.000000 | 25.216687 | +25.216687 |
| Y | Canopy ancestor claims | -0.000000 | 0.000696 | +0.000696 |
| Y | Canopy constant opening | -0.000000 | 0.000421 | +0.000421 |
| **Subtotal** | **opening** | **733.504204** | **779.685421** | **+46.181216** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 1102.342600 | 1098.309500 | 992.500625 | 1199.511876 | 81.919632 | 368.838396 | 733.504204 | 10 |
| bolt_max_canopy_quarter | 1177.990608 | 1171.433875 | 1044.659041 | 1310.803833 | 71.128018 | 398.305188 | 779.685421 | 10 |
