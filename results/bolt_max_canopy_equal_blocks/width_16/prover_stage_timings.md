# Bolt-max prover stage profile

Input 2^18, width 16, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k/4 and parity k/4; total k/2, 40% of Bolt's encoded rows. Heights start at 3 for systematic and 1 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 3.351584 | 3.383125 | +0.031541 |
| Bolt | parity RS encoding Y=RS(U) | 3.072087 | 3.291429 | +0.219342 |
| X | row Merkle tree or forest | 4.287733 | 4.354842 | +0.067108 |
| X | Merkle root transcript | 0.000187 | -0.000000 | -0.000187 |
| Y | row Merkle tree or forest | 1.397129 | 1.421575 | +0.024446 |
| Y | Merkle root transcript | 0.000383 | -0.000000 | -0.000383 |
| other | unassigned phase work and timing overhead | 0.000783 | 0.000804 | +0.000021 |
| X | Canopy hash-to-field | -0.000000 | 0.000429 | +0.000429 |
| X | Canopy constant RS commitment | -0.000000 | 0.002654 | +0.002654 |
| Y | Canopy hash-to-field | -0.000000 | 0.000292 | +0.000292 |
| Y | Canopy constant RS commitment | -0.000000 | 0.002733 | +0.002733 |
| **Subtotal** | **commitment** | **12.109888** | **12.457884** | **+0.347996** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 5.815675 | 5.958175 | +0.142500 |
| Bolt | column projections x=Xr and u=Ur | 1.278650 | 1.321942 | +0.043291 |
| common WHIR | WHIR configuration | 0.003517 | 0.003750 | +0.000234 |
| x | common WHIR RS encode and oracle commitment | 1.768267 | 1.852325 | +0.084058 |
| u | common WHIR RS encode and oracle commitment | 0.605708 | 0.602083 | -0.003625 |
| Bolt | syndrome evaluation sigma | 0.126321 | 0.129313 | +0.002992 |
| Bolt | sample row queries | 0.075133 | 0.074729 | -0.000404 |
| X | copy and serialize queried row values | 45.222367 | 46.192379 | +0.970013 |
| X | collect and serialize row authentication paths | 1.442487 | 1.407875 | -0.034612 |
| Y | copy and serialize queried row values | 0.856717 | 0.866358 | +0.009641 |
| Y | collect and serialize row authentication paths | 0.107658 | 0.102725 | -0.004933 |
| Bolt | project queried rows and sample batching challenges | 5.498471 | 5.561800 | +0.063329 |
| Bolt | systematic query covector | 0.662729 | 0.708371 | +0.045642 |
| Bolt | parity query covector and transpose RS FFT | 0.757450 | 0.780942 | +0.023492 |
| Bolt | public H transpose covector | 0.741875 | 0.744192 | +0.002316 |
| x | common WHIR constrained proof | 6.845354 | 6.884071 | +0.038716 |
| u | common WHIR constrained proof | 1.611904 | 1.568979 | -0.042925 |
| other | unassigned phase work and timing overhead | 0.057663 | 0.065175 | +0.007512 |
| X | Canopy ancestor claims | -0.000000 | 0.016592 | +0.016592 |
| X | Canopy constant opening | -0.000000 | 0.000404 | +0.000404 |
| Y | Canopy ancestor claims | -0.000000 | 0.000562 | +0.000562 |
| Y | Canopy constant opening | -0.000000 | 0.000267 | +0.000267 |
| **Subtotal** | **opening** | **73.477946** | **74.843008** | **+1.365062** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 85.587834 | 85.947855 | 83.767334 | 87.048042 | 1.135454 | 12.109888 | 73.477946 | 10 |
| bolt_max_canopy_equal_blocks | 87.300892 | 86.741667 | 83.840625 | 92.757083 | 3.124835 | 12.457884 | 74.843008 | 10 |
