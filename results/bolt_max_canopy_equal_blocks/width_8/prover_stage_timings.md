# Bolt-max prover stage profile

Input 2^18, width 8, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k/4 and parity k/4; total k/2, 40% of Bolt's encoded rows. Heights start at 3 for systematic and 1 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 4.830479 | 3.659875 | -1.170604 |
| Bolt | parity RS encoding Y=RS(U) | 3.811238 | 3.669308 | -0.141929 |
| X | row Merkle tree or forest | 5.519504 | 5.867562 | +0.348058 |
| X | Merkle root transcript | 0.000200 | -0.000000 | -0.000200 |
| Y | row Merkle tree or forest | 1.804404 | 1.933146 | +0.128742 |
| Y | Merkle root transcript | 0.000429 | -0.000000 | -0.000429 |
| other | unassigned phase work and timing overhead | 0.000866 | 0.001109 | +0.000242 |
| X | Canopy hash-to-field | -0.000000 | 0.000637 | +0.000637 |
| X | Canopy constant RS commitment | -0.000000 | 0.003554 | +0.003554 |
| Y | Canopy hash-to-field | -0.000000 | 0.000358 | +0.000358 |
| Y | Canopy constant RS commitment | -0.000000 | 0.003233 | +0.003233 |
| **Subtotal** | **commitment** | **15.967121** | **15.138783** | **-0.828337** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 5.907733 | 5.969025 | +0.061292 |
| Bolt | column projections x=Xr and u=Ur | 1.734650 | 1.593508 | -0.141142 |
| common WHIR | WHIR configuration | 0.005521 | 0.005183 | -0.000337 |
| x | common WHIR RS encode and oracle commitment | 3.633883 | 4.682521 | +1.048637 |
| u | common WHIR RS encode and oracle commitment | 0.856371 | 0.853908 | -0.002462 |
| Bolt | syndrome evaluation sigma | 0.247625 | 0.247767 | +0.000142 |
| Bolt | sample row queries | 7.630662 | 8.064258 | +0.433596 |
| X | copy and serialize queried row values | 29.353217 | 29.809787 | +0.456571 |
| X | collect and serialize row authentication paths | 2.691387 | 2.729500 | +0.038113 |
| Y | copy and serialize queried row values | 0.441725 | 0.449438 | +0.007713 |
| Y | collect and serialize row authentication paths | 0.141333 | 0.136629 | -0.004704 |
| Bolt | project queried rows and sample batching challenges | 3.470108 | 3.439929 | -0.030179 |
| Bolt | systematic query covector | 0.908492 | 0.931208 | +0.022717 |
| Bolt | parity query covector and transpose RS FFT | 1.069004 | 1.060492 | -0.008512 |
| Bolt | public H transpose covector | 1.528042 | 1.578679 | +0.050638 |
| x | common WHIR constrained proof | 10.994833 | 11.333521 | +0.338687 |
| u | common WHIR constrained proof | 2.754338 | 2.759313 | +0.004975 |
| other | unassigned phase work and timing overhead | 0.015350 | 0.029558 | +0.014208 |
| X | Canopy ancestor claims | -0.000000 | 0.021625 | +0.021625 |
| X | Canopy constant opening | -0.000000 | 0.000304 | +0.000304 |
| Y | Canopy ancestor claims | -0.000000 | 0.000608 | +0.000608 |
| Y | Canopy constant opening | -0.000000 | 0.000304 | +0.000304 |
| **Subtotal** | **opening** | **73.384275** | **75.697067** | **+2.312792** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 89.351395 | 87.696292 | 82.072500 | 107.219832 | 7.330358 | 15.967121 | 73.384275 | 10 |
| bolt_max_canopy_equal_blocks | 90.835850 | 87.505833 | 81.670418 | 109.577250 | 8.855306 | 15.138783 | 75.697067 | 10 |
