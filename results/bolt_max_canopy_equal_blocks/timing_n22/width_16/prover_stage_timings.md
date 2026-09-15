# Bolt-max prover stage profile

Input 2^22, width 16, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k/4 and parity k/4; total k/2, 40% of Bolt's encoded rows. Heights start at 3 for systematic and 1 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 103.618021 | 103.464767 | -0.153254 |
| Bolt | parity RS encoding Y=RS(U) | 56.555667 | 53.821112 | -2.734554 |
| X | row Merkle tree or forest | 59.690975 | 59.013475 | -0.677500 |
| X | Merkle root transcript | 0.000246 | -0.000000 | -0.000246 |
| Y | row Merkle tree or forest | 15.430604 | 15.601154 | +0.170550 |
| Y | Merkle root transcript | 0.000642 | -0.000000 | -0.000642 |
| other | unassigned phase work and timing overhead | 0.000983 | 0.001296 | +0.000313 |
| X | Canopy hash-to-field | -0.000000 | 6.020908 | +6.020908 |
| X | Canopy WHIR configuration | -0.000000 | 0.003488 | +0.003488 |
| X | Canopy RS encode and WHIR oracle commitment | -0.000000 | 3.522362 | +3.522362 |
| Y | Canopy hash-to-field | -0.000000 | 0.000529 | +0.000529 |
| Y | Canopy constant RS commitment | -0.000000 | 0.004213 | +0.004213 |
| **Subtotal** | **commitment** | **235.297138** | **241.453304** | **+6.156167** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 145.781363 | 145.761404 | -0.019959 |
| Bolt | column projections x=Xr and u=Ur | 17.810471 | 17.712821 | -0.097650 |
| common WHIR | WHIR configuration | 0.008016 | 0.007679 | -0.000337 |
| x | common WHIR RS encode and oracle commitment | 28.387796 | 28.141392 | -0.246404 |
| u | common WHIR RS encode and oracle commitment | 3.437779 | 3.342908 | -0.094871 |
| Bolt | syndrome evaluation sigma | 1.064412 | 1.061267 | -0.003146 |
| Bolt | sample row queries | 6.601167 | 6.615867 | +0.014700 |
| X | copy and serialize queried row values | 63.247329 | 61.271734 | -1.975596 |
| X | collect and serialize row authentication paths | 7.506158 | 5.307317 | -2.198842 |
| Y | copy and serialize queried row values | 0.909496 | 0.886525 | -0.022971 |
| Y | collect and serialize row authentication paths | 0.318704 | 0.312304 | -0.006400 |
| Bolt | project queried rows and sample batching challenges | 6.913612 | 6.949187 | +0.035575 |
| Bolt | systematic query covector | 1.538983 | 1.534050 | -0.004933 |
| Bolt | parity query covector and transpose RS FFT | 4.164925 | 4.416146 | +0.251221 |
| Bolt | public H transpose covector | 13.173500 | 13.308213 | +0.134713 |
| x | common WHIR constrained proof | 52.520858 | 54.081650 | +1.560792 |
| u | common WHIR constrained proof | 9.502196 | 9.579529 | +0.077333 |
| other | unassigned phase work and timing overhead | 0.353501 | 0.449779 | +0.096278 |
| X | Canopy ancestor claims | -0.000000 | 0.090167 | +0.090167 |
| X | Canopy SEP batching covector | -0.000000 | 0.635617 | +0.635617 |
| X | Canopy OOD evaluations | -0.000000 | 0.285267 | +0.285267 |
| X | Canopy WHIR SEP proof | -0.000000 | 8.460262 | +8.460262 |
| Y | Canopy ancestor claims | -0.000000 | 0.000683 | +0.000683 |
| Y | Canopy constant opening | -0.000000 | 0.000367 | +0.000367 |
| **Subtotal** | **opening** | **363.240266** | **370.212133** | **+6.971867** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 598.537404 | 593.682562 | 583.011666 | 654.076542 | 20.631050 | 235.297138 | 363.240266 | 10 |
| bolt_max_canopy_equal_blocks | 611.665438 | 607.720709 | 599.043416 | 648.856542 | 13.742552 | 241.453304 | 370.212133 | 10 |
