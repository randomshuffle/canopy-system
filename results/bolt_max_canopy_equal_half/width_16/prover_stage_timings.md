# Bolt-max prover stage profile

Input 2^18, width 16, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k/2 and parity k/2; total k, 80% of Bolt's encoded rows. Heights start at 2 for systematic and 0 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 3.953167 | 4.365188 | +0.412021 |
| Bolt | parity RS encoding Y=RS(U) | 3.545429 | 3.504150 | -0.041279 |
| X | row Merkle tree or forest | 4.721246 | 4.883887 | +0.162642 |
| X | Merkle root transcript | 0.000129 | -0.000000 | -0.000129 |
| Y | row Merkle tree or forest | 1.464667 | 1.552754 | +0.088088 |
| Y | Merkle root transcript | 0.000542 | -0.000000 | -0.000542 |
| other | unassigned phase work and timing overhead | 0.001104 | 0.002899 | +0.001796 |
| X | Canopy hash-to-field | -0.000000 | 0.000679 | +0.000679 |
| X | Canopy constant RS commitment | -0.000000 | 0.003612 | +0.003612 |
| Y | Canopy hash-to-field | -0.000000 | 0.000454 | +0.000454 |
| Y | Canopy constant RS commitment | -0.000000 | 0.004942 | +0.004942 |
| **Subtotal** | **commitment** | **13.686283** | **14.318566** | **+0.632283** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 6.686775 | 6.975779 | +0.289004 |
| Bolt | column projections x=Xr and u=Ur | 1.379263 | 1.382583 | +0.003321 |
| common WHIR | WHIR configuration | 0.005646 | 0.006133 | +0.000487 |
| x | common WHIR RS encode and oracle commitment | 2.069000 | 2.064675 | -0.004325 |
| u | common WHIR RS encode and oracle commitment | 0.626650 | 0.731892 | +0.105242 |
| Bolt | syndrome evaluation sigma | 0.127429 | 0.133417 | +0.005988 |
| Bolt | sample row queries | 0.077079 | 0.078900 | +0.001821 |
| X | copy and serialize queried row values | 48.991071 | 48.567571 | -0.423500 |
| X | collect and serialize row authentication paths | 1.444733 | 1.442113 | -0.002621 |
| Y | copy and serialize queried row values | 0.889279 | 0.878808 | -0.010471 |
| Y | collect and serialize row authentication paths | 0.104671 | 0.106858 | +0.002188 |
| Bolt | project queried rows and sample batching challenges | 5.548296 | 5.475396 | -0.072900 |
| Bolt | systematic query covector | 0.675829 | 0.706687 | +0.030858 |
| Bolt | parity query covector and transpose RS FFT | 0.820200 | 0.801050 | -0.019150 |
| Bolt | public H transpose covector | 0.926671 | 0.799592 | -0.127080 |
| x | common WHIR constrained proof | 7.042704 | 7.172346 | +0.129642 |
| u | common WHIR constrained proof | 1.652275 | 1.620871 | -0.031404 |
| other | unassigned phase work and timing overhead | 0.026875 | 0.013992 | -0.012883 |
| X | Canopy ancestor claims | -0.000000 | 0.016833 | +0.016833 |
| X | Canopy constant opening | -0.000000 | 0.000417 | +0.000417 |
| Y | Canopy ancestor claims | -0.000000 | 0.000613 | +0.000613 |
| Y | Canopy constant opening | -0.000000 | 0.000254 | +0.000254 |
| **Subtotal** | **opening** | **79.094446** | **78.976779** | **-0.117667** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 92.780729 | 93.538604 | 88.112708 | 98.200208 | 3.954902 | 13.686283 | 79.094446 | 10 |
| bolt_max_canopy_equal_half | 93.295345 | 93.481145 | 84.244292 | 114.619750 | 9.046051 | 14.318566 | 78.976779 | 10 |
