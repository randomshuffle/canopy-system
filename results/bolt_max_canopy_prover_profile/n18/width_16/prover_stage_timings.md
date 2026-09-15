# Bolt-max prover stage profile

Input 2^18, width 16, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 3.555483 | 3.767483 | +0.212000 |
| Bolt | parity RS encoding Y=RS(U) | 3.206800 | 3.653416 | +0.446617 |
| X | row Merkle tree or forest | 4.609925 | 4.775912 | +0.165988 |
| X | Merkle root transcript | 0.000179 | -0.000000 | -0.000179 |
| Y | row Merkle tree or forest | 1.419450 | 1.467171 | +0.047721 |
| Y | Merkle root transcript | 0.000462 | -0.000000 | -0.000462 |
| other | unassigned phase work and timing overhead | 0.000959 | 0.001038 | +0.000079 |
| X | Canopy hash-to-field | -0.000000 | 0.000567 | +0.000567 |
| X | Canopy constant RS commitment | -0.000000 | 0.002900 | +0.002900 |
| Y | Canopy hash-to-field | -0.000000 | 0.000333 | +0.000333 |
| Y | Canopy constant RS commitment | -0.000000 | 0.002538 | +0.002538 |
| **Subtotal** | **commitment** | **12.793258** | **13.671358** | **+0.878100** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 6.225475 | 6.409583 | +0.184108 |
| Bolt | column projections x=Xr and u=Ur | 1.387883 | 1.454137 | +0.066254 |
| common WHIR | WHIR configuration | 0.004483 | 0.005300 | +0.000817 |
| x | common WHIR RS encode and oracle commitment | 1.820392 | 1.892013 | +0.071621 |
| u | common WHIR RS encode and oracle commitment | 0.601129 | 0.618550 | +0.017421 |
| Bolt | syndrome evaluation sigma | 0.128342 | 0.129017 | +0.000675 |
| Bolt | sample row queries | 0.075375 | 0.075546 | +0.000170 |
| X | copy and serialize queried row values | 47.650875 | 48.303400 | +0.652525 |
| X | collect and serialize row authentication paths | 1.360404 | 1.370133 | +0.009729 |
| Y | copy and serialize queried row values | 0.897029 | 0.914779 | +0.017750 |
| Y | collect and serialize row authentication paths | 0.111129 | 0.107046 | -0.004083 |
| Bolt | project queried rows and sample batching challenges | 5.663175 | 5.790417 | +0.127242 |
| Bolt | systematic query covector | 0.671550 | 0.709742 | +0.038192 |
| Bolt | parity query covector and transpose RS FFT | 0.795921 | 1.136121 | +0.340200 |
| Bolt | public H transpose covector | 0.777062 | 0.784525 | +0.007463 |
| x | common WHIR constrained proof | 6.537558 | 6.721875 | +0.184317 |
| u | common WHIR constrained proof | 1.559221 | 1.567287 | +0.008066 |
| other | unassigned phase work and timing overhead | 0.006408 | 0.052871 | +0.046463 |
| X | Canopy ancestor claims | -0.000000 | 0.015275 | +0.015275 |
| X | Canopy constant opening | -0.000000 | 0.000404 | +0.000404 |
| Y | Canopy ancestor claims | -0.000000 | 0.000629 | +0.000629 |
| Y | Canopy constant opening | -0.000000 | 0.000279 | +0.000279 |
| **Subtotal** | **opening** | **76.273413** | **78.058929** | **+1.785517** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 89.066671 | 89.964062 | 84.074416 | 93.227751 | 3.409807 | 12.793258 | 76.273413 | 10 |
| bolt_max_canopy_quarter | 91.730287 | 89.952937 | 85.158167 | 104.433749 | 6.341396 | 13.671358 | 78.058929 | 10 |
