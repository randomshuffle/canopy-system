# Bolt-max prover stage profile

Input 2^22, width 8, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k and parity k; total 2k, 160% of Bolt's encoded rows. Heights start at 1 for systematic and 0 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 136.774729 | 134.743542 | -2.031188 |
| Bolt | parity RS encoding Y=RS(U) | 60.444421 | 60.193937 | -0.250483 |
| X | row Merkle tree or forest | 75.016758 | 68.006454 | -7.010304 |
| X | Merkle root transcript | 0.000221 | -0.000000 | -0.000221 |
| Y | row Merkle tree or forest | 20.224113 | 19.408717 | -0.815396 |
| Y | Merkle root transcript | 0.001158 | -0.000000 | -0.001158 |
| other | unassigned phase work and timing overhead | 0.001708 | 0.001846 | +0.000137 |
| X | Canopy hash-to-field | -0.000000 | 48.561492 | +48.561492 |
| X | Canopy WHIR configuration | -0.000000 | 0.007808 | +0.007808 |
| X | Canopy RS encode and WHIR oracle commitment | -0.000000 | 30.515242 | +30.515242 |
| Y | Canopy hash-to-field | -0.000000 | 0.000750 | +0.000750 |
| Y | Canopy constant RS commitment | -0.000000 | 0.004750 | +0.004750 |
| **Subtotal** | **commitment** | **292.463108** | **361.444537** | **+68.981429** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 97.658383 | 97.640433 | -0.017950 |
| Bolt | column projections x=Xr and u=Ur | 18.178867 | 17.902884 | -0.275983 |
| common WHIR | WHIR configuration | 0.010096 | 0.009471 | -0.000625 |
| x | common WHIR RS encode and oracle commitment | 60.174808 | 58.918762 | -1.256046 |
| u | common WHIR RS encode and oracle commitment | 7.648429 | 7.463396 | -0.185033 |
| Bolt | syndrome evaluation sigma | 2.058929 | 2.042521 | -0.016408 |
| Bolt | sample row queries | 6.660283 | 6.604054 | -0.056229 |
| X | copy and serialize queried row values | 30.524987 | 30.653821 | +0.128833 |
| X | collect and serialize row authentication paths | 10.157525 | 2.642963 | -7.514562 |
| Y | copy and serialize queried row values | 0.466946 | 0.459958 | -0.006987 |
| Y | collect and serialize row authentication paths | 0.538587 | 0.264050 | -0.274537 |
| Bolt | project queried rows and sample batching challenges | 3.458325 | 3.372083 | -0.086242 |
| Bolt | systematic query covector | 2.827641 | 2.788266 | -0.039375 |
| Bolt | parity query covector and transpose RS FFT | 8.148371 | 8.215329 | +0.066958 |
| Bolt | public H transpose covector | 39.873887 | 39.808358 | -0.065529 |
| x | common WHIR constrained proof | 111.570771 | 112.051963 | +0.481192 |
| u | common WHIR constrained proof | 15.810684 | 15.527138 | -0.283546 |
| other | unassigned phase work and timing overhead | 0.714689 | 0.954434 | +0.239745 |
| X | Canopy ancestor claims | -0.000000 | 0.185604 | +0.185604 |
| X | Canopy SEP batching covector | -0.000000 | 1.703937 | +1.703937 |
| X | Canopy OOD evaluations | -0.000000 | 1.398821 | +1.398821 |
| X | Canopy WHIR SEP proof | -0.000000 | 42.183371 | +42.183371 |
| Y | Canopy ancestor claims | -0.000000 | 0.000604 | +0.000604 |
| Y | Canopy constant opening | -0.000000 | 0.000383 | +0.000383 |
| **Subtotal** | **opening** | **416.482208** | **452.792604** | **+36.310396** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 708.945316 | 698.817625 | 695.177750 | 758.482875 | 20.031786 | 292.463108 | 416.482208 | 10 |
| bolt_max_canopy_equal_full | 814.237142 | 811.479041 | 806.855708 | 827.191500 | 7.341641 | 361.444537 | 452.792604 | 10 |
