# Bolt-max prover stage profile

Input 2^22, width 16, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k/2 and parity k/2; total k, 80% of Bolt's encoded rows. Heights start at 2 for systematic and 0 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 105.533121 | 108.222600 | +2.689479 |
| Bolt | parity RS encoding Y=RS(U) | 54.875321 | 55.077367 | +0.202046 |
| X | row Merkle tree or forest | 60.926521 | 59.126192 | -1.800329 |
| X | Merkle root transcript | 0.000183 | -0.000000 | -0.000183 |
| Y | row Merkle tree or forest | 15.773950 | 15.853604 | +0.079654 |
| Y | Merkle root transcript | 0.000758 | -0.000000 | -0.000758 |
| other | unassigned phase work and timing overhead | 0.001242 | 0.001925 | +0.000683 |
| X | Canopy hash-to-field | -0.000000 | 12.217396 | +12.217396 |
| X | Canopy WHIR configuration | -0.000000 | 0.005800 | +0.005800 |
| X | Canopy RS encode and WHIR oracle commitment | -0.000000 | 7.440979 | +7.440979 |
| Y | Canopy hash-to-field | -0.000000 | 0.000642 | +0.000642 |
| Y | Canopy constant RS commitment | -0.000000 | 0.004454 | +0.004454 |
| **Subtotal** | **commitment** | **237.111096** | **257.950958** | **+20.839862** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 147.928971 | 148.646300 | +0.717329 |
| Bolt | column projections x=Xr and u=Ur | 18.517325 | 18.513896 | -0.003429 |
| common WHIR | WHIR configuration | 0.008446 | 0.008992 | +0.000546 |
| x | common WHIR RS encode and oracle commitment | 29.543121 | 30.633971 | +1.090850 |
| u | common WHIR RS encode and oracle commitment | 3.587838 | 3.640146 | +0.052308 |
| Bolt | syndrome evaluation sigma | 1.046509 | 1.080638 | +0.034129 |
| Bolt | sample row queries | 6.630250 | 6.682908 | +0.052658 |
| X | copy and serialize queried row values | 60.663779 | 63.292213 | +2.628433 |
| X | collect and serialize row authentication paths | 7.577204 | 4.011829 | -3.565375 |
| Y | copy and serialize queried row values | 0.888329 | 0.950825 | +0.062496 |
| Y | collect and serialize row authentication paths | 0.283346 | 0.315029 | +0.031683 |
| Bolt | project queried rows and sample batching challenges | 7.011979 | 6.828329 | -0.183650 |
| Bolt | systematic query covector | 1.592038 | 1.643504 | +0.051467 |
| Bolt | parity query covector and transpose RS FFT | 4.732725 | 4.434562 | -0.298162 |
| Bolt | public H transpose covector | 14.639467 | 13.870013 | -0.769454 |
| x | common WHIR constrained proof | 53.942817 | 53.802096 | -0.140721 |
| u | common WHIR constrained proof | 9.847954 | 9.970817 | +0.122863 |
| other | unassigned phase work and timing overhead | 0.350646 | 0.438108 | +0.087462 |
| X | Canopy ancestor claims | -0.000000 | 0.092579 | +0.092579 |
| X | Canopy SEP batching covector | -0.000000 | 0.827525 | +0.827525 |
| X | Canopy OOD evaluations | -0.000000 | 0.488013 | +0.488013 |
| X | Canopy WHIR SEP proof | -0.000000 | 13.173821 | +13.173821 |
| Y | Canopy ancestor claims | -0.000000 | 0.000617 | +0.000617 |
| Y | Canopy constant opening | -0.000000 | 0.000308 | +0.000308 |
| **Subtotal** | **opening** | **368.792742** | **383.347038** | **+14.554296** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 605.903838 | 602.992293 | 593.307501 | 631.468500 | 11.501901 | 237.111096 | 368.792742 | 10 |
| bolt_max_canopy_equal_half | 641.297996 | 636.205271 | 628.522918 | 665.991208 | 12.282235 | 257.950958 | 383.347038 | 10 |
