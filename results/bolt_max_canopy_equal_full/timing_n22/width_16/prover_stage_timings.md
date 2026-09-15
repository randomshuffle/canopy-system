# Bolt-max prover stage profile

Input 2^22, width 16, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k and parity k; total 2k, 160% of Bolt's encoded rows. Heights start at 1 for systematic and 0 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 143.130142 | 139.999571 | -3.130571 |
| Bolt | parity RS encoding Y=RS(U) | 64.466604 | 64.071950 | -0.394654 |
| X | row Merkle tree or forest | 64.649967 | 60.249337 | -4.400629 |
| X | Merkle root transcript | 0.000188 | -0.000000 | -0.000188 |
| Y | row Merkle tree or forest | 17.444688 | 17.050463 | -0.394225 |
| Y | Merkle root transcript | 0.001029 | -0.000000 | -0.001029 |
| other | unassigned phase work and timing overhead | 0.001513 | 0.001613 | +0.000100 |
| X | Canopy hash-to-field | -0.000000 | 24.432825 | +24.432825 |
| X | Canopy WHIR configuration | -0.000000 | 0.005496 | +0.005496 |
| X | Canopy RS encode and WHIR oracle commitment | -0.000000 | 15.337054 | +15.337054 |
| Y | Canopy hash-to-field | -0.000000 | 0.000633 | +0.000633 |
| Y | Canopy constant RS commitment | -0.000000 | 0.004342 | +0.004342 |
| **Subtotal** | **commitment** | **289.694129** | **321.153283** | **+31.459154** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 157.567379 | 161.658059 | +4.090679 |
| Bolt | column projections x=Xr and u=Ur | 18.522138 | 20.282100 | +1.759962 |
| common WHIR | WHIR configuration | 0.009367 | 0.010383 | +0.001017 |
| x | common WHIR RS encode and oracle commitment | 31.242658 | 34.040758 | +2.798100 |
| u | common WHIR RS encode and oracle commitment | 3.635846 | 3.791683 | +0.155838 |
| Bolt | syndrome evaluation sigma | 1.098583 | 1.134875 | +0.036292 |
| Bolt | sample row queries | 6.739500 | 6.852746 | +0.113246 |
| X | copy and serialize queried row values | 62.596625 | 63.791867 | +1.195242 |
| X | collect and serialize row authentication paths | 7.499167 | 2.520892 | -4.978275 |
| Y | copy and serialize queried row values | 0.925009 | 0.902675 | -0.022333 |
| Y | collect and serialize row authentication paths | 0.344833 | 0.331471 | -0.013362 |
| Bolt | project queried rows and sample batching challenges | 7.015367 | 7.002383 | -0.012983 |
| Bolt | systematic query covector | 1.709446 | 1.800575 | +0.091129 |
| Bolt | parity query covector and transpose RS FFT | 4.764896 | 4.937163 | +0.172267 |
| Bolt | public H transpose covector | 14.504966 | 15.797558 | +1.292592 |
| x | common WHIR constrained proof | 55.897371 | 59.079266 | +3.181895 |
| u | common WHIR constrained proof | 9.997746 | 10.246529 | +0.248783 |
| other | unassigned phase work and timing overhead | 0.365642 | 0.747933 | +0.382291 |
| X | Canopy ancestor claims | -0.000000 | 0.136233 | +0.136233 |
| X | Canopy SEP batching covector | -0.000000 | 1.048996 | +1.048996 |
| X | Canopy OOD evaluations | -0.000000 | 0.704813 | +0.704813 |
| X | Canopy WHIR SEP proof | -0.000000 | 24.883463 | +24.883463 |
| Y | Canopy ancestor claims | -0.000000 | 0.000617 | +0.000617 |
| Y | Canopy constant opening | -0.000000 | 0.000354 | +0.000354 |
| **Subtotal** | **opening** | **384.436537** | **421.703392** | **+37.266854** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 674.130667 | 643.310896 | 595.007625 | 981.759125 | 113.020575 | 289.694129 | 384.436537 | 10 |
| bolt_max_canopy_equal_full | 742.856675 | 695.762438 | 642.223208 | 1073.755208 | 132.455054 | 321.153283 | 421.703392 | 10 |
