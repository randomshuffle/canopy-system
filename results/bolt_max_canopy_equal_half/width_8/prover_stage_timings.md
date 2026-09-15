# Bolt-max prover stage profile

Input 2^18, width 8, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k/2 and parity k/2; total k, 80% of Bolt's encoded rows. Heights start at 2 for systematic and 0 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 3.742037 | 3.636725 | -0.105313 |
| Bolt | parity RS encoding Y=RS(U) | 3.617296 | 3.384225 | -0.233071 |
| X | row Merkle tree or forest | 5.486329 | 5.137679 | -0.348650 |
| X | Merkle root transcript | 0.000117 | -0.000000 | -0.000117 |
| Y | row Merkle tree or forest | 1.801467 | 1.641666 | -0.159800 |
| Y | Merkle root transcript | 0.000446 | -0.000000 | -0.000446 |
| other | unassigned phase work and timing overhead | 0.001066 | 0.001284 | +0.000217 |
| X | Canopy hash-to-field | -0.000000 | 0.000496 | +0.000496 |
| X | Canopy constant RS commitment | -0.000000 | 0.003000 | +0.003000 |
| Y | Canopy hash-to-field | -0.000000 | 0.000400 | +0.000400 |
| Y | Canopy constant RS commitment | -0.000000 | 0.003471 | +0.003471 |
| **Subtotal** | **commitment** | **14.648758** | **13.808946** | **-0.839813** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 5.961046 | 6.012175 | +0.051129 |
| Bolt | column projections x=Xr and u=Ur | 1.442858 | 1.312933 | -0.129925 |
| common WHIR | WHIR configuration | 0.005787 | 0.005512 | -0.000275 |
| x | common WHIR RS encode and oracle commitment | 3.578417 | 3.513292 | -0.065125 |
| u | common WHIR RS encode and oracle commitment | 0.798271 | 0.809942 | +0.011671 |
| Bolt | syndrome evaluation sigma | 0.246279 | 0.252413 | +0.006134 |
| Bolt | sample row queries | 7.615346 | 7.604300 | -0.011046 |
| X | copy and serialize queried row values | 28.751700 | 28.968713 | +0.217013 |
| X | collect and serialize row authentication paths | 2.870288 | 2.750617 | -0.119671 |
| Y | copy and serialize queried row values | 0.446804 | 0.467058 | +0.020254 |
| Y | collect and serialize row authentication paths | 0.142467 | 0.227887 | +0.085421 |
| Bolt | project queried rows and sample batching challenges | 3.442162 | 3.376467 | -0.065696 |
| Bolt | systematic query covector | 0.906088 | 0.954121 | +0.048033 |
| Bolt | parity query covector and transpose RS FFT | 1.112071 | 1.173033 | +0.060962 |
| Bolt | public H transpose covector | 1.580067 | 1.614021 | +0.033954 |
| x | common WHIR constrained proof | 10.627675 | 10.745650 | +0.117975 |
| u | common WHIR constrained proof | 2.727833 | 2.734667 | +0.006833 |
| other | unassigned phase work and timing overhead | 0.053291 | 0.045425 | -0.007866 |
| X | Canopy ancestor claims | -0.000000 | 0.020883 | +0.020883 |
| X | Canopy constant opening | -0.000000 | 0.000425 | +0.000425 |
| Y | Canopy ancestor claims | -0.000000 | 0.000608 | +0.000608 |
| Y | Canopy constant opening | -0.000000 | 0.000267 | +0.000267 |
| **Subtotal** | **opening** | **72.308450** | **72.590408** | **+0.281958** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 86.957208 | 86.193250 | 84.298999 | 90.754625 | 1.934058 | 14.648758 | 72.308450 | 10 |
| bolt_max_canopy_equal_half | 86.399354 | 85.235833 | 83.876334 | 93.961126 | 2.965401 | 13.808946 | 72.590408 | 10 |
