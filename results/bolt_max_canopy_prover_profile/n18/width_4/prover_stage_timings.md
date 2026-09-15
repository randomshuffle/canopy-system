# Bolt-max prover stage profile

Input 2^18, width 4, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 4.125196 | 4.136633 | +0.011437 |
| Bolt | parity RS encoding Y=RS(U) | 4.370600 | 4.587633 | +0.217033 |
| X | row Merkle tree or forest | 8.041133 | 8.385858 | +0.344725 |
| X | Merkle root transcript | 0.000196 | -0.000000 | -0.000196 |
| Y | row Merkle tree or forest | 2.382817 | 2.557125 | +0.174308 |
| Y | Merkle root transcript | 0.000829 | -0.000000 | -0.000829 |
| other | unassigned phase work and timing overhead | 0.001316 | 0.002629 | +0.001313 |
| X | Canopy hash-to-field | -0.000000 | 0.000692 | +0.000692 |
| X | Canopy constant RS commitment | -0.000000 | 0.003159 | +0.003159 |
| Y | Canopy hash-to-field | -0.000000 | 0.000750 | +0.000750 |
| Y | Canopy constant RS commitment | -0.000000 | 0.008083 | +0.008083 |
| **Subtotal** | **commitment** | **18.922087** | **19.682563** | **+0.760475** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 6.217754 | 6.323675 | +0.105921 |
| Bolt | column projections x=Xr and u=Ur | 1.600021 | 1.713542 | +0.113521 |
| common WHIR | WHIR configuration | 0.006546 | 0.006579 | +0.000033 |
| x | common WHIR RS encode and oracle commitment | 8.218954 | 7.934150 | -0.284804 |
| u | common WHIR RS encode and oracle commitment | 1.159658 | 1.220929 | +0.061270 |
| Bolt | syndrome evaluation sigma | 0.477408 | 0.491150 | +0.013742 |
| Bolt | sample row queries | 6.018658 | 6.065250 | +0.046592 |
| X | copy and serialize queried row values | 15.637967 | 15.064392 | -0.573575 |
| X | collect and serialize row authentication paths | 4.241767 | 4.229354 | -0.012412 |
| Y | copy and serialize queried row values | 0.251854 | 0.252316 | +0.000462 |
| Y | collect and serialize row authentication paths | 0.239554 | 0.202137 | -0.037417 |
| Bolt | project queried rows and sample batching challenges | 1.882216 | 1.844167 | -0.038050 |
| Bolt | systematic query covector | 0.977900 | 1.008996 | +0.031096 |
| Bolt | parity query covector and transpose RS FFT | 1.646454 | 1.865242 | +0.218788 |
| Bolt | public H transpose covector | 3.354517 | 3.483500 | +0.128983 |
| x | common WHIR constrained proof | 18.769650 | 18.207633 | -0.562017 |
| u | common WHIR constrained proof | 4.221529 | 4.304404 | +0.082875 |
| other | unassigned phase work and timing overhead | 0.069996 | 0.053796 | -0.016200 |
| X | Canopy ancestor claims | -0.000000 | 0.019575 | +0.019575 |
| X | Canopy constant opening | -0.000000 | 0.000338 | +0.000338 |
| Y | Canopy ancestor claims | -0.000000 | 0.000671 | +0.000671 |
| Y | Canopy constant opening | -0.000000 | 0.000333 | +0.000333 |
| **Subtotal** | **opening** | **74.992404** | **74.292129** | **-0.700275** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 93.914492 | 92.548520 | 89.708001 | 104.412125 | 4.249913 | 18.922087 | 74.992404 | 10 |
| bolt_max_canopy_quarter | 93.974692 | 94.294791 | 88.447959 | 98.474418 | 3.204756 | 19.682563 | 74.292129 | 10 |
