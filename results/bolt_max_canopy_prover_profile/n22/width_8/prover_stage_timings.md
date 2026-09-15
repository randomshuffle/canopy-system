# Bolt-max prover stage profile

Input 2^22, width 8, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 135.643846 | 133.070462 | -2.573383 |
| Bolt | parity RS encoding Y=RS(U) | 64.219246 | 58.150567 | -6.068679 |
| X | row Merkle tree or forest | 75.031042 | 73.528146 | -1.502896 |
| X | Merkle root transcript | 0.000216 | -0.000000 | -0.000216 |
| Y | row Merkle tree or forest | 19.429162 | 19.319092 | -0.110071 |
| Y | Merkle root transcript | 0.002679 | -0.000000 | -0.002679 |
| other | unassigned phase work and timing overhead | 0.001651 | 0.001754 | +0.000103 |
| X | Canopy hash-to-field | -0.000000 | 12.234383 | +12.234383 |
| X | Canopy WHIR configuration | -0.000000 | 0.005529 | +0.005529 |
| X | Canopy RS encode and WHIR oracle commitment | -0.000000 | 7.819871 | +7.819871 |
| Y | Canopy hash-to-field | -0.000000 | 0.000612 | +0.000612 |
| Y | Canopy constant RS commitment | -0.000000 | 0.003779 | +0.003779 |
| **Subtotal** | **commitment** | **294.327842** | **304.134196** | **+9.806354** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 93.390829 | 92.414042 | -0.976788 |
| Bolt | column projections x=Xr and u=Ur | 18.312421 | 19.207512 | +0.895092 |
| common WHIR | WHIR configuration | 0.008783 | 0.009258 | +0.000475 |
| x | common WHIR RS encode and oracle commitment | 61.806275 | 61.833117 | +0.026842 |
| u | common WHIR RS encode and oracle commitment | 7.389804 | 7.213513 | -0.176292 |
| Bolt | syndrome evaluation sigma | 2.036866 | 2.063479 | +0.026613 |
| Bolt | sample row queries | 6.591275 | 6.613871 | +0.022596 |
| X | copy and serialize queried row values | 30.367088 | 31.544937 | +1.177850 |
| X | collect and serialize row authentication paths | 9.789650 | 6.268908 | -3.520742 |
| Y | copy and serialize queried row values | 0.462404 | 0.457275 | -0.005129 |
| Y | collect and serialize row authentication paths | 0.394596 | 0.398450 | +0.003854 |
| Bolt | project queried rows and sample batching challenges | 3.462550 | 3.485062 | +0.022512 |
| Bolt | systematic query covector | 2.585229 | 2.639625 | +0.054396 |
| Bolt | parity query covector and transpose RS FFT | 8.106159 | 8.337488 | +0.231329 |
| Bolt | public H transpose covector | 35.913646 | 35.207316 | -0.706329 |
| x | common WHIR constrained proof | 110.928533 | 109.399796 | -1.528737 |
| u | common WHIR constrained proof | 15.502929 | 15.557513 | +0.054584 |
| other | unassigned phase work and timing overhead | 0.560424 | 0.783500 | +0.223077 |
| X | Canopy ancestor claims | -0.000000 | 0.089317 | +0.089317 |
| X | Canopy SEP batching covector | -0.000000 | 0.774954 | +0.774954 |
| X | Canopy OOD evaluations | -0.000000 | 0.421771 | +0.421771 |
| X | Canopy WHIR SEP proof | -0.000000 | 13.457958 | +13.457958 |
| Y | Canopy ancestor claims | -0.000000 | 0.000608 | +0.000608 |
| Y | Canopy constant opening | -0.000000 | 0.000384 | +0.000384 |
| **Subtotal** | **opening** | **407.609462** | **418.179654** | **+10.570192** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 701.937304 | 682.157624 | 650.603335 | 815.574916 | 59.956442 | 294.327842 | 407.609462 | 10 |
| bolt_max_canopy_quarter | 722.313850 | 719.464708 | 676.997209 | 783.925958 | 34.946767 | 304.134196 | 418.179654 | 10 |
