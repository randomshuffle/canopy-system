# Bolt-max prover stage profile

Input 2^22, width 4, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k/4 and parity k/4; total k/2, 40% of Bolt's encoded rows. Heights start at 3 for systematic and 1 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 135.740800 | 138.118146 | +2.377346 |
| Bolt | parity RS encoding Y=RS(U) | 60.052017 | 60.051554 | -0.000463 |
| X | row Merkle tree or forest | 94.764446 | 90.865779 | -3.898667 |
| X | Merkle root transcript | 0.000233 | -0.000000 | -0.000233 |
| Y | row Merkle tree or forest | 24.443783 | 25.136517 | +0.692733 |
| Y | Merkle root transcript | 0.000834 | -0.000000 | -0.000834 |
| other | unassigned phase work and timing overhead | 0.001120 | 0.001500 | +0.000380 |
| X | Canopy hash-to-field | -0.000000 | 24.168646 | +24.168646 |
| X | Canopy WHIR configuration | -0.000000 | 0.004617 | +0.004617 |
| X | Canopy RS encode and WHIR oracle commitment | -0.000000 | 14.180404 | +14.180404 |
| Y | Canopy hash-to-field | -0.000000 | 0.000550 | +0.000550 |
| Y | Canopy constant RS commitment | -0.000000 | 0.004358 | +0.004358 |
| **Subtotal** | **commitment** | **315.003233** | **352.532071** | **+37.528838** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 91.089375 | 92.125004 | +1.035629 |
| Bolt | column projections x=Xr and u=Ur | 16.691983 | 17.527658 | +0.835675 |
| common WHIR | WHIR configuration | 0.008400 | 0.008046 | -0.000354 |
| x | common WHIR RS encode and oracle commitment | 130.836687 | 132.770075 | +1.933388 |
| u | common WHIR RS encode and oracle commitment | 13.512129 | 13.798471 | +0.286342 |
| Bolt | syndrome evaluation sigma | 3.758925 | 3.792292 | +0.033367 |
| Bolt | sample row queries | 6.484929 | 6.483933 | -0.000996 |
| X | copy and serialize queried row values | 16.141409 | 15.504846 | -0.636563 |
| X | collect and serialize row authentication paths | 12.731471 | 7.323579 | -5.407892 |
| Y | copy and serialize queried row values | 0.254667 | 0.246813 | -0.007854 |
| Y | collect and serialize row authentication paths | 0.309467 | 0.295992 | -0.013475 |
| Bolt | project queried rows and sample batching challenges | 1.623446 | 1.698850 | +0.075404 |
| Bolt | systematic query covector | 3.498683 | 3.555421 | +0.056738 |
| Bolt | parity query covector and transpose RS FFT | 15.012771 | 15.221033 | +0.208263 |
| Bolt | public H transpose covector | 110.804896 | 116.414013 | +5.609117 |
| x | common WHIR constrained proof | 212.209638 | 218.713550 | +6.503912 |
| u | common WHIR constrained proof | 25.976242 | 27.008642 | +1.032400 |
| other | unassigned phase work and timing overhead | 1.065516 | 1.212383 | +0.146867 |
| X | Canopy ancestor claims | -0.000000 | 0.124358 | +0.124358 |
| X | Canopy SEP batching covector | -0.000000 | 0.945987 | +0.945987 |
| X | Canopy OOD evaluations | -0.000000 | 0.741704 | +0.741704 |
| X | Canopy WHIR SEP proof | -0.000000 | 22.081687 | +22.081687 |
| Y | Canopy ancestor claims | -0.000000 | 0.000633 | +0.000633 |
| Y | Canopy constant opening | -0.000000 | 0.000362 | +0.000362 |
| **Subtotal** | **opening** | **662.010634** | **697.595333** | **+35.584700** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 977.013867 | 976.738917 | 972.646291 | 985.682374 | 3.741734 | 315.003233 | 662.010634 | 10 |
| bolt_max_canopy_equal_blocks | 1050.127404 | 1027.973062 | 1019.981834 | 1155.140541 | 45.847415 | 352.532071 | 697.595333 | 10 |
