# Bolt-max prover stage profile

Input 2^22, width 8, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k/2 and parity k/2; total k, 80% of Bolt's encoded rows. Heights start at 2 for systematic and 0 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 130.613525 | 146.984204 | +16.370679 |
| Bolt | parity RS encoding Y=RS(U) | 60.209400 | 59.694850 | -0.514550 |
| X | row Merkle tree or forest | 82.015658 | 73.134112 | -8.881546 |
| X | Merkle root transcript | 0.000184 | -0.000000 | -0.000184 |
| Y | row Merkle tree or forest | 19.652629 | 19.216471 | -0.436158 |
| Y | Merkle root transcript | 0.000821 | -0.000000 | -0.000821 |
| other | unassigned phase work and timing overhead | 0.001483 | 0.002175 | +0.000692 |
| X | Canopy hash-to-field | -0.000000 | 24.557354 | +24.557354 |
| X | Canopy WHIR configuration | -0.000000 | 0.006308 | +0.006308 |
| X | Canopy RS encode and WHIR oracle commitment | -0.000000 | 14.337883 | +14.337883 |
| Y | Canopy hash-to-field | -0.000000 | 0.000650 | +0.000650 |
| Y | Canopy constant RS commitment | -0.000000 | 0.004742 | +0.004742 |
| **Subtotal** | **commitment** | **292.493700** | **337.938750** | **+45.445050** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 92.919096 | 92.226275 | -0.692821 |
| Bolt | column projections x=Xr and u=Ur | 18.255500 | 17.736308 | -0.519192 |
| common WHIR | WHIR configuration | 0.009192 | 0.008937 | -0.000254 |
| x | common WHIR RS encode and oracle commitment | 58.857221 | 60.592696 | +1.735475 |
| u | common WHIR RS encode and oracle commitment | 7.566342 | 7.430075 | -0.136267 |
| Bolt | syndrome evaluation sigma | 1.990204 | 2.048329 | +0.058125 |
| Bolt | sample row queries | 6.584912 | 6.688708 | +0.103796 |
| X | copy and serialize queried row values | 30.761050 | 33.836946 | +3.075896 |
| X | collect and serialize row authentication paths | 9.693879 | 4.774183 | -4.919696 |
| Y | copy and serialize queried row values | 0.472225 | 0.468213 | -0.004012 |
| Y | collect and serialize row authentication paths | 0.429575 | 0.343488 | -0.086088 |
| Bolt | project queried rows and sample batching challenges | 3.454208 | 3.508437 | +0.054229 |
| Bolt | systematic query covector | 2.596621 | 2.822458 | +0.225837 |
| Bolt | parity query covector and transpose RS FFT | 8.013375 | 9.414525 | +1.401150 |
| Bolt | public H transpose covector | 33.917442 | 39.712696 | +5.795255 |
| x | common WHIR constrained proof | 122.705208 | 138.416750 | +15.711542 |
| u | common WHIR constrained proof | 17.317508 | 19.441142 | +2.123634 |
| other | unassigned phase work and timing overhead | 0.691779 | 0.836604 | +0.144825 |
| X | Canopy ancestor claims | -0.000000 | 0.138071 | +0.138071 |
| X | Canopy SEP batching covector | -0.000000 | 1.039708 | +1.039708 |
| X | Canopy OOD evaluations | -0.000000 | 0.769054 | +0.769054 |
| X | Canopy WHIR SEP proof | -0.000000 | 22.913017 | +22.913017 |
| Y | Canopy ancestor claims | -0.000000 | 0.000642 | +0.000642 |
| Y | Canopy constant opening | -0.000000 | 0.000312 | +0.000312 |
| **Subtotal** | **opening** | **416.235337** | **465.167575** | **+48.932238** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 708.729038 | 685.918729 | 654.842917 | 979.306208 | 96.952855 | 292.493700 | 416.235337 | 10 |
| bolt_max_canopy_equal_half | 803.106325 | 732.182916 | 716.838417 | 1161.077041 | 141.892772 | 337.938750 | 465.167575 | 10 |
