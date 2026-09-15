# Bolt-max prover stage profile

Input 2^22, width 4, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k and parity k; total 2k, 160% of Bolt's encoded rows. Heights start at 1 for systematic and 0 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 148.335633 | 150.537488 | +2.201854 |
| Bolt | parity RS encoding Y=RS(U) | 64.325804 | 64.246867 | -0.078937 |
| X | row Merkle tree or forest | 99.739133 | 82.001471 | -17.737662 |
| X | Merkle root transcript | 0.000183 | -0.000000 | -0.000183 |
| Y | row Merkle tree or forest | 25.269171 | 25.473454 | +0.204283 |
| Y | Merkle root transcript | 0.001112 | -0.000000 | -0.001112 |
| other | unassigned phase work and timing overhead | 0.001638 | 0.001850 | +0.000213 |
| X | Canopy hash-to-field | -0.000000 | 96.949421 | +96.949421 |
| X | Canopy WHIR configuration | -0.000000 | 0.007846 | +0.007846 |
| X | Canopy RS encode and WHIR oracle commitment | -0.000000 | 59.281808 | +59.281808 |
| Y | Canopy hash-to-field | -0.000000 | 0.000796 | +0.000796 |
| Y | Canopy constant RS commitment | -0.000000 | 0.007579 | +0.007579 |
| **Subtotal** | **commitment** | **337.672675** | **478.508579** | **+140.835905** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 94.175137 | 94.285200 | +0.110063 |
| Bolt | column projections x=Xr and u=Ur | 17.357996 | 17.886700 | +0.528704 |
| common WHIR | WHIR configuration | 0.010921 | 0.009354 | -0.001567 |
| x | common WHIR RS encode and oracle commitment | 139.644792 | 140.653213 | +1.008421 |
| u | common WHIR RS encode and oracle commitment | 14.414642 | 14.934796 | +0.520154 |
| Bolt | syndrome evaluation sigma | 3.894154 | 3.936096 | +0.041942 |
| Bolt | sample row queries | 6.579337 | 6.635067 | +0.055729 |
| X | copy and serialize queried row values | 16.517217 | 16.595087 | +0.077871 |
| X | collect and serialize row authentication paths | 13.736896 | 2.847992 | -10.888904 |
| Y | copy and serialize queried row values | 0.261079 | 0.248675 | -0.012404 |
| Y | collect and serialize row authentication paths | 0.327412 | 0.294433 | -0.032979 |
| Bolt | project queried rows and sample batching challenges | 1.692296 | 1.667821 | -0.024475 |
| Bolt | systematic query covector | 4.137775 | 3.916634 | -0.221141 |
| Bolt | parity query covector and transpose RS FFT | 16.123121 | 15.699742 | -0.423379 |
| Bolt | public H transpose covector | 144.458129 | 130.826871 | -13.631258 |
| x | common WHIR constrained proof | 232.170162 | 228.756183 | -3.413979 |
| u | common WHIR constrained proof | 27.499408 | 28.310729 | +0.811321 |
| other | unassigned phase work and timing overhead | 1.166851 | 1.773983 | +0.607133 |
| X | Canopy ancestor claims | -0.000000 | 0.205679 | +0.205679 |
| X | Canopy SEP batching covector | -0.000000 | 2.699809 | +2.699809 |
| X | Canopy OOD evaluations | -0.000000 | 2.681033 | +2.681033 |
| X | Canopy WHIR SEP proof | -0.000000 | 89.509250 | +89.509250 |
| Y | Canopy ancestor claims | -0.000000 | 0.000671 | +0.000671 |
| Y | Canopy constant opening | -0.000000 | 0.000404 | +0.000404 |
| **Subtotal** | **opening** | **734.167325** | **804.375421** | **+70.208096** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 1071.840000 | 1046.713916 | 1030.302958 | 1241.742833 | 65.146165 | 337.672675 | 734.167325 | 10 |
| bolt_max_canopy_equal_full | 1282.884001 | 1271.845167 | 1262.727584 | 1366.135709 | 30.315779 | 478.508579 | 804.375421 | 10 |
