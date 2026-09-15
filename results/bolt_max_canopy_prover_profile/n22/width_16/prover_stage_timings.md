# Bolt-max prover stage profile

Input 2^22, width 16, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 109.623321 | 114.176304 | +4.552983 |
| Bolt | parity RS encoding Y=RS(U) | 57.444179 | 57.975067 | +0.530888 |
| X | row Merkle tree or forest | 62.335962 | 62.246488 | -0.089475 |
| X | Merkle root transcript | 0.000183 | -0.000000 | -0.000183 |
| Y | row Merkle tree or forest | 16.336887 | 17.114241 | +0.777354 |
| Y | Merkle root transcript | 0.000808 | -0.000000 | -0.000808 |
| other | unassigned phase work and timing overhead | 0.001237 | 0.001745 | +0.000508 |
| X | Canopy hash-to-field | -0.000000 | 6.097017 | +6.097017 |
| X | Canopy WHIR configuration | -0.000000 | 0.005650 | +0.005650 |
| X | Canopy RS encode and WHIR oracle commitment | -0.000000 | 3.823388 | +3.823388 |
| Y | Canopy hash-to-field | -0.000000 | 0.000667 | +0.000667 |
| Y | Canopy constant RS commitment | -0.000000 | 0.003592 | +0.003592 |
| **Subtotal** | **commitment** | **245.742579** | **261.444158** | **+15.701579** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 149.252587 | 153.734913 | +4.482325 |
| Bolt | column projections x=Xr and u=Ur | 18.269429 | 19.465854 | +1.196425 |
| common WHIR | WHIR configuration | 0.008541 | 0.012754 | +0.004213 |
| x | common WHIR RS encode and oracle commitment | 30.249579 | 30.628217 | +0.378637 |
| u | common WHIR RS encode and oracle commitment | 3.437538 | 3.644675 | +0.207137 |
| Bolt | syndrome evaluation sigma | 1.069346 | 1.080692 | +0.011346 |
| Bolt | sample row queries | 6.646550 | 6.655454 | +0.008904 |
| X | copy and serialize queried row values | 58.408708 | 60.662212 | +2.253504 |
| X | collect and serialize row authentication paths | 7.670917 | 5.510525 | -2.160392 |
| Y | copy and serialize queried row values | 0.913475 | 0.897671 | -0.015804 |
| Y | collect and serialize row authentication paths | 0.352979 | 0.298667 | -0.054313 |
| Bolt | project queried rows and sample batching challenges | 7.071079 | 7.384867 | +0.313788 |
| Bolt | systematic query covector | 1.619200 | 1.681850 | +0.062650 |
| Bolt | parity query covector and transpose RS FFT | 4.481704 | 4.510883 | +0.029179 |
| Bolt | public H transpose covector | 14.350462 | 14.634437 | +0.283975 |
| x | common WHIR constrained proof | 57.432637 | 57.769550 | +0.336913 |
| u | common WHIR constrained proof | 10.867358 | 9.857971 | -1.009387 |
| other | unassigned phase work and timing overhead | 0.584900 | 0.428267 | -0.156633 |
| X | Canopy ancestor claims | -0.000000 | 0.089333 | +0.089333 |
| X | Canopy SEP batching covector | -0.000000 | 0.703650 | +0.703650 |
| X | Canopy OOD evaluations | -0.000000 | 0.288817 | +0.288817 |
| X | Canopy WHIR SEP proof | -0.000000 | 8.659704 | +8.659704 |
| Y | Canopy ancestor claims | -0.000000 | 0.000683 | +0.000683 |
| Y | Canopy constant opening | -0.000000 | 0.000321 | +0.000321 |
| **Subtotal** | **opening** | **372.686992** | **388.601967** | **+15.914975** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 618.429571 | 610.452437 | 595.100959 | 693.576750 | 28.513587 | 245.742579 | 372.686992 | 10 |
| bolt_max_canopy_quarter | 650.046125 | 649.549542 | 614.728333 | 704.580626 | 26.254846 | 261.444158 | 388.601967 | 10 |
