# Bolt-max prover stage profile

Input 2^18, width 4, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k/2 and parity k/2; total k, 80% of Bolt's encoded rows. Heights start at 2 for systematic and 0 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 3.563850 | 3.611825 | +0.047975 |
| Bolt | parity RS encoding Y=RS(U) | 3.615717 | 3.556775 | -0.058942 |
| X | row Merkle tree or forest | 6.970996 | 6.835429 | -0.135567 |
| X | Merkle root transcript | 0.000208 | -0.000000 | -0.000208 |
| Y | row Merkle tree or forest | 2.073550 | 2.063409 | -0.010142 |
| Y | Merkle root transcript | 0.000596 | -0.000000 | -0.000596 |
| other | unassigned phase work and timing overhead | 0.001383 | 0.001337 | -0.000046 |
| X | Canopy hash-to-field | -0.000000 | 0.000608 | +0.000608 |
| X | Canopy constant RS commitment | -0.000000 | 0.003812 | +0.003812 |
| Y | Canopy hash-to-field | -0.000000 | 0.000383 | +0.000383 |
| Y | Canopy constant RS commitment | -0.000000 | 0.003342 | +0.003342 |
| **Subtotal** | **commitment** | **16.226300** | **16.076921** | **-0.149379** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 6.149921 | 6.150217 | +0.000296 |
| Bolt | column projections x=Xr and u=Ur | 1.353458 | 1.332138 | -0.021320 |
| common WHIR | WHIR configuration | 0.005746 | 0.005804 | +0.000058 |
| x | common WHIR RS encode and oracle commitment | 7.474883 | 7.258179 | -0.216704 |
| u | common WHIR RS encode and oracle commitment | 1.194017 | 1.104592 | -0.089425 |
| Bolt | syndrome evaluation sigma | 0.466871 | 0.476558 | +0.009688 |
| Bolt | sample row queries | 5.931754 | 5.946604 | +0.014850 |
| X | copy and serialize queried row values | 14.828208 | 15.113338 | +0.285129 |
| X | collect and serialize row authentication paths | 4.079458 | 4.033525 | -0.045934 |
| Y | copy and serialize queried row values | 0.237579 | 0.242663 | +0.005084 |
| Y | collect and serialize row authentication paths | 0.185229 | 0.173783 | -0.011446 |
| Bolt | project queried rows and sample batching challenges | 1.870758 | 1.859108 | -0.011650 |
| Bolt | systematic query covector | 0.995771 | 0.930183 | -0.065587 |
| Bolt | parity query covector and transpose RS FFT | 1.592287 | 1.569808 | -0.022479 |
| Bolt | public H transpose covector | 3.232408 | 3.183883 | -0.048525 |
| x | common WHIR constrained proof | 16.870462 | 16.618362 | -0.252100 |
| u | common WHIR constrained proof | 4.078846 | 4.086334 | +0.007488 |
| other | unassigned phase work and timing overhead | 0.063264 | 0.060645 | -0.002619 |
| X | Canopy ancestor claims | -0.000000 | 0.018408 | +0.018408 |
| X | Canopy constant opening | -0.000000 | 0.000317 | +0.000317 |
| Y | Canopy ancestor claims | -0.000000 | 0.000883 | +0.000883 |
| Y | Canopy constant opening | -0.000000 | 0.000321 | +0.000321 |
| **Subtotal** | **opening** | **70.610921** | **70.165654** | **-0.445267** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 86.837221 | 86.484083 | 85.513917 | 90.650084 | 1.564353 | 16.226300 | 70.610921 | 10 |
| bolt_max_canopy_equal_half | 86.242575 | 86.367937 | 84.672167 | 87.900876 | 0.973972 | 16.076921 | 70.165654 | 10 |
