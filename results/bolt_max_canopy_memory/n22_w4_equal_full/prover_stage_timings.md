# Bolt-max prover stage profile

Input 2^22, width 4, 256-bit field, BLAKE3, 8 Rayon threads. 0 warmups and 1 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k and parity k; total 2k, 160% of Bolt's encoded rows. Heights start at 1 for systematic and 0 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 164.186709 | 147.535250 | -16.651459 |
| Bolt | parity RS encoding Y=RS(U) | 70.453709 | 63.539250 | -6.914459 |
| X | row Merkle tree or forest | 105.697042 | 85.511917 | -20.185125 |
| X | Merkle root transcript | 0.000250 | -0.000000 | -0.000250 |
| Y | row Merkle tree or forest | 25.446875 | 25.052625 | -0.394250 |
| Y | Merkle root transcript | 0.000958 | -0.000000 | -0.000958 |
| other | unassigned phase work and timing overhead | 0.185166 | 0.001459 | -0.183707 |
| X | Canopy hash-to-field | -0.000000 | 97.151250 | +97.151250 |
| X | Canopy WHIR configuration | -0.000000 | 0.007958 | +0.007958 |
| X | Canopy RS encode and WHIR oracle commitment | -0.000000 | 58.914458 | +58.914458 |
| Y | Canopy hash-to-field | -0.000000 | 0.000917 | +0.000917 |
| Y | Canopy constant RS commitment | -0.000000 | 0.004875 | +0.004875 |
| **Subtotal** | **commitment** | **365.970709** | **477.719959** | **+111.749250** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 98.865250 | 93.123458 | -5.741792 |
| Bolt | column projections x=Xr and u=Ur | 19.957208 | 17.559166 | -2.398042 |
| common WHIR | WHIR configuration | 0.011209 | 0.008333 | -0.002876 |
| x | common WHIR RS encode and oracle commitment | 179.109958 | 140.553167 | -38.556791 |
| u | common WHIR RS encode and oracle commitment | 16.376750 | 13.590167 | -2.786583 |
| Bolt | syndrome evaluation sigma | 4.168791 | 3.861958 | -0.306833 |
| Bolt | sample row queries | 7.221291 | 6.633291 | -0.588000 |
| X | copy and serialize queried row values | 16.686208 | 14.864208 | -1.822000 |
| X | collect and serialize row authentication paths | 14.556000 | 2.505167 | -12.050833 |
| Y | copy and serialize queried row values | 0.283542 | 0.229792 | -0.053750 |
| Y | collect and serialize row authentication paths | 0.656791 | 0.277750 | -0.379041 |
| Bolt | project queried rows and sample batching challenges | 1.707167 | 1.656042 | -0.051125 |
| Bolt | systematic query covector | 13.891959 | 3.514084 | -10.377875 |
| Bolt | parity query covector and transpose RS FFT | 24.553542 | 15.370375 | -9.183167 |
| Bolt | public H transpose covector | 135.042333 | 129.496500 | -5.545833 |
| x | common WHIR constrained proof | 251.707375 | 229.252583 | -22.454792 |
| u | common WHIR constrained proof | 26.899000 | 28.037000 | +1.138000 |
| other | unassigned phase work and timing overhead | 0.890417 | 2.061584 | +1.171167 |
| X | Canopy ancestor claims | -0.000000 | 0.209750 | +0.209750 |
| X | Canopy SEP batching covector | -0.000000 | 2.575917 | +2.575917 |
| X | Canopy OOD evaluations | -0.000000 | 2.607000 | +2.607000 |
| X | Canopy WHIR SEP proof | -0.000000 | 84.509542 | +84.509542 |
| Y | Canopy ancestor claims | -0.000000 | 0.000458 | +0.000458 |
| Y | Canopy constant opening | -0.000000 | 0.000250 | +0.000250 |
| **Subtotal** | **opening** | **812.584791** | **792.497542** | **-20.087249** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 1178.555500 | 1178.555500 | 1178.555500 | 1178.555500 | 0.000000 | 365.970709 | 812.584791 | 1 |
| bolt_max_canopy_equal_full | 1270.217501 | 1270.217501 | 1270.217501 | 1270.217501 | 0.000000 | 477.719959 | 792.497542 | 1 |
