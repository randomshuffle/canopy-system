# Bolt-max prover stage profile

Input 2^18, width 4, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k and parity k; total 2k, 160% of Bolt's encoded rows. Heights start at 1 for systematic and 0 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 4.408017 | 3.773375 | -0.634642 |
| Bolt | parity RS encoding Y=RS(U) | 5.126021 | 4.125967 | -1.000054 |
| X | row Merkle tree or forest | 8.508346 | 5.813825 | -2.694521 |
| X | Merkle root transcript | 0.000192 | -0.000000 | -0.000192 |
| Y | row Merkle tree or forest | 3.543096 | 2.115283 | -1.427813 |
| Y | Merkle root transcript | 0.000646 | -0.000000 | -0.000646 |
| other | unassigned phase work and timing overhead | 0.001629 | 0.001449 | -0.000180 |
| X | Canopy hash-to-field | -0.000000 | 6.114800 | +6.114800 |
| X | Canopy WHIR configuration | -0.000000 | 0.004575 | +0.004575 |
| X | Canopy RS encode and WHIR oracle commitment | -0.000000 | 3.634854 | +3.634854 |
| Y | Canopy hash-to-field | -0.000000 | 0.000408 | +0.000408 |
| Y | Canopy constant RS commitment | -0.000000 | 0.003979 | +0.003979 |
| **Subtotal** | **commitment** | **21.587946** | **25.588516** | **+4.000571** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 7.075708 | 6.042608 | -1.033100 |
| Bolt | column projections x=Xr and u=Ur | 1.649408 | 1.520763 | -0.128646 |
| common WHIR | WHIR configuration | 0.007608 | 0.005396 | -0.002213 |
| x | common WHIR RS encode and oracle commitment | 8.245558 | 7.600462 | -0.645096 |
| u | common WHIR RS encode and oracle commitment | 1.220663 | 1.172613 | -0.048050 |
| Bolt | syndrome evaluation sigma | 0.500592 | 0.461692 | -0.038900 |
| Bolt | sample row queries | 6.149367 | 5.905454 | -0.243913 |
| X | copy and serialize queried row values | 16.519750 | 15.340087 | -1.179662 |
| X | collect and serialize row authentication paths | 4.548121 | 1.704154 | -2.843967 |
| Y | copy and serialize queried row values | 0.241675 | 0.233784 | -0.007891 |
| Y | collect and serialize row authentication paths | 0.205463 | 0.173858 | -0.031604 |
| Bolt | project queried rows and sample batching challenges | 2.038654 | 1.881458 | -0.157196 |
| Bolt | systematic query covector | 1.066617 | 0.940088 | -0.126529 |
| Bolt | parity query covector and transpose RS FFT | 2.340337 | 1.623017 | -0.717321 |
| Bolt | public H transpose covector | 4.084608 | 3.315346 | -0.769263 |
| x | common WHIR constrained proof | 20.630492 | 17.515988 | -3.114504 |
| u | common WHIR constrained proof | 4.345158 | 4.086425 | -0.258733 |
| other | unassigned phase work and timing overhead | 0.079762 | 0.121182 | +0.041420 |
| X | Canopy ancestor claims | -0.000000 | 0.081629 | +0.081629 |
| X | Canopy SEP batching covector | -0.000000 | 0.768708 | +0.768708 |
| X | Canopy OOD evaluations | -0.000000 | 0.295092 | +0.295092 |
| X | Canopy WHIR SEP proof | -0.000000 | 8.969596 | +8.969596 |
| Y | Canopy ancestor claims | -0.000000 | 0.000696 | +0.000696 |
| Y | Canopy constant opening | -0.000000 | 0.000304 | +0.000304 |
| **Subtotal** | **opening** | **80.949541** | **79.760400** | **-1.189142** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 102.537487 | 89.645124 | 85.886334 | 187.510958 | 31.715818 | 21.587946 | 80.949541 | 10 |
| bolt_max_canopy_equal_full | 105.348916 | 103.800458 | 99.429207 | 114.300250 | 4.801935 | 25.588516 | 79.760400 | 10 |
