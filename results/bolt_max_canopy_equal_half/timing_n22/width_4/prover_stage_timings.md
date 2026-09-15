# Bolt-max prover stage profile

Input 2^22, width 4, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k/2 and parity k/2; total k, 80% of Bolt's encoded rows. Heights start at 2 for systematic and 0 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 145.406608 | 141.886788 | -3.519821 |
| Bolt | parity RS encoding Y=RS(U) | 65.399687 | 64.392513 | -1.007175 |
| X | row Merkle tree or forest | 102.973271 | 88.991650 | -13.981621 |
| X | Merkle root transcript | 0.000183 | -0.000000 | -0.000183 |
| Y | row Merkle tree or forest | 26.102938 | 25.486196 | -0.616742 |
| Y | Merkle root transcript | 0.001075 | -0.000000 | -0.001075 |
| other | unassigned phase work and timing overhead | 0.001688 | 0.001967 | +0.000279 |
| X | Canopy hash-to-field | -0.000000 | 48.452367 | +48.452367 |
| X | Canopy WHIR configuration | -0.000000 | 0.007433 | +0.007433 |
| X | Canopy RS encode and WHIR oracle commitment | -0.000000 | 30.551708 | +30.551708 |
| Y | Canopy hash-to-field | -0.000000 | 0.000667 | +0.000667 |
| Y | Canopy constant RS commitment | -0.000000 | 0.004412 | +0.004412 |
| **Subtotal** | **commitment** | **339.885450** | **399.775700** | **+59.890250** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 103.744612 | 92.091533 | -11.653079 |
| Bolt | column projections x=Xr and u=Ur | 24.338046 | 18.607450 | -5.730596 |
| common WHIR | WHIR configuration | 0.009184 | 0.009021 | -0.000163 |
| x | common WHIR RS encode and oracle commitment | 142.437946 | 135.980413 | -6.457533 |
| u | common WHIR RS encode and oracle commitment | 14.197483 | 13.865758 | -0.331725 |
| Bolt | syndrome evaluation sigma | 3.891388 | 3.821850 | -0.069538 |
| Bolt | sample row queries | 6.541729 | 6.506850 | -0.034879 |
| X | copy and serialize queried row values | 15.696379 | 15.429263 | -0.267117 |
| X | collect and serialize row authentication paths | 13.027933 | 4.772517 | -8.255417 |
| Y | copy and serialize queried row values | 0.326150 | 0.237125 | -0.089025 |
| Y | collect and serialize row authentication paths | 0.449754 | 0.417104 | -0.032650 |
| Bolt | project queried rows and sample batching challenges | 1.632466 | 1.757013 | +0.124546 |
| Bolt | systematic query covector | 3.629429 | 3.623642 | -0.005787 |
| Bolt | parity query covector and transpose RS FFT | 15.210321 | 15.397991 | +0.187671 |
| Bolt | public H transpose covector | 166.104262 | 114.166492 | -51.937771 |
| x | common WHIR constrained proof | 227.274000 | 221.053783 | -6.220217 |
| u | common WHIR constrained proof | 26.393808 | 26.980334 | +0.586525 |
| other | unassigned phase work and timing overhead | 1.311497 | 1.408691 | +0.097194 |
| X | Canopy ancestor claims | -0.000000 | 0.181633 | +0.181633 |
| X | Canopy SEP batching covector | -0.000000 | 1.573746 | +1.573746 |
| X | Canopy OOD evaluations | -0.000000 | 1.219717 | +1.219717 |
| X | Canopy WHIR SEP proof | -0.000000 | 41.025908 | +41.025908 |
| Y | Canopy ancestor claims | -0.000000 | 0.000500 | +0.000500 |
| Y | Canopy constant opening | -0.000000 | 0.000371 | +0.000371 |
| **Subtotal** | **opening** | **766.216387** | **720.128704** | **-46.087683** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 1106.101838 | 1003.884292 | 994.671083 | 2011.565334 | 318.364862 | 339.885450 | 766.216387 | 10 |
| bolt_max_canopy_equal_half | 1119.904404 | 1116.004041 | 1097.959707 | 1171.565125 | 21.434181 | 399.775700 | 720.128704 | 10 |
