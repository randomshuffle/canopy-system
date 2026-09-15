# Bolt-max prover stage profile

Input 2^18, width 8, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k and parity k; total 2k, 160% of Bolt's encoded rows. Heights start at 1 for systematic and 0 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 3.604796 | 3.691021 | +0.086225 |
| Bolt | parity RS encoding Y=RS(U) | 3.497996 | 3.307988 | -0.190008 |
| X | row Merkle tree or forest | 5.175358 | 5.320662 | +0.145304 |
| X | Merkle root transcript | 0.000112 | -0.000000 | -0.000112 |
| Y | row Merkle tree or forest | 1.624946 | 1.697362 | +0.072417 |
| Y | Merkle root transcript | 0.000487 | -0.000000 | -0.000487 |
| other | unassigned phase work and timing overhead | 0.001142 | 0.001233 | +0.000091 |
| X | Canopy hash-to-field | -0.000000 | 0.000550 | +0.000550 |
| X | Canopy constant RS commitment | -0.000000 | 0.003075 | +0.003075 |
| Y | Canopy hash-to-field | -0.000000 | 0.000342 | +0.000342 |
| Y | Canopy constant RS commitment | -0.000000 | 0.003279 | +0.003279 |
| **Subtotal** | **commitment** | **13.904838** | **14.025513** | **+0.120675** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 6.028979 | 5.925621 | -0.103358 |
| Bolt | column projections x=Xr and u=Ur | 1.426696 | 1.342104 | -0.084591 |
| common WHIR | WHIR configuration | 0.006067 | 0.006409 | +0.000342 |
| x | common WHIR RS encode and oracle commitment | 3.508421 | 3.563029 | +0.054608 |
| u | common WHIR RS encode and oracle commitment | 0.810579 | 0.837750 | +0.027171 |
| Bolt | syndrome evaluation sigma | 0.251763 | 0.247863 | -0.003900 |
| Bolt | sample row queries | 7.653217 | 7.588833 | -0.064383 |
| X | copy and serialize queried row values | 29.787150 | 29.860375 | +0.073225 |
| X | collect and serialize row authentication paths | 2.840629 | 2.771312 | -0.069317 |
| Y | copy and serialize queried row values | 0.462721 | 0.464050 | +0.001329 |
| Y | collect and serialize row authentication paths | 0.137042 | 0.144433 | +0.007392 |
| Bolt | project queried rows and sample batching challenges | 3.481041 | 3.469983 | -0.011058 |
| Bolt | systematic query covector | 0.905379 | 0.917529 | +0.012150 |
| Bolt | parity query covector and transpose RS FFT | 0.990883 | 1.138667 | +0.147783 |
| Bolt | public H transpose covector | 1.591792 | 1.609458 | +0.017667 |
| x | common WHIR constrained proof | 11.168754 | 10.876946 | -0.291808 |
| u | common WHIR constrained proof | 2.714996 | 2.725533 | +0.010537 |
| other | unassigned phase work and timing overhead | 0.031726 | 0.056079 | +0.024354 |
| X | Canopy ancestor claims | -0.000000 | 0.024246 | +0.024246 |
| X | Canopy constant opening | -0.000000 | 0.000358 | +0.000358 |
| Y | Canopy ancestor claims | -0.000000 | 0.000775 | +0.000775 |
| Y | Canopy constant opening | -0.000000 | 0.000354 | +0.000354 |
| **Subtotal** | **opening** | **73.797833** | **73.571708** | **-0.226125** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 87.702671 | 87.530188 | 83.850750 | 92.624917 | 2.830108 | 13.904838 | 73.797833 | 10 |
| bolt_max_canopy_equal_full | 87.597221 | 87.317001 | 85.885000 | 91.173625 | 1.481486 | 14.025513 | 73.571708 | 10 |
