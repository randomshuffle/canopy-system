# Bolt-max prover stage profile

Input 2^24, width 4, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k and parity k; total 2k, 160% of Bolt's encoded rows. Heights start at 1 for systematic and 0 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 647.136413 | 719.194762 | +72.058350 |
| Bolt | parity RS encoding Y=RS(U) | 285.244825 | 286.266213 | +1.021388 |
| X | row Merkle tree or forest | 390.615700 | 329.652463 | -60.963238 |
| X | Merkle root transcript | 0.000196 | -0.000000 | -0.000196 |
| Y | row Merkle tree or forest | 99.087900 | 97.848975 | -1.238925 |
| Y | Merkle root transcript | 0.003625 | -0.000000 | -0.003625 |
| other | unassigned phase work and timing overhead | 0.001683 | 0.001983 | +0.000300 |
| X | Canopy hash-to-field | -0.000000 | 392.608729 | +392.608729 |
| X | Canopy WHIR configuration | -0.000000 | 0.007429 | +0.007429 |
| X | Canopy RS encode and WHIR oracle commitment | -0.000000 | 278.088346 | +278.088346 |
| Y | Canopy hash-to-field | -0.000000 | 0.001038 | +0.001038 |
| Y | Canopy constant RS commitment | -0.000000 | 0.006175 | +0.006175 |
| **Subtotal** | **commitment** | **1422.090341** | **2103.676112** | **+681.585771** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 377.610233 | 375.658696 | -1.951538 |
| Bolt | column projections x=Xr and u=Ur | 69.181108 | 72.506725 | +3.325617 |
| common WHIR | WHIR configuration | 0.011183 | 0.011666 | +0.000483 |
| x | common WHIR RS encode and oracle commitment | 636.790983 | 694.246362 | +57.455379 |
| u | common WHIR RS encode and oracle commitment | 63.813721 | 66.942867 | +3.129146 |
| Bolt | syndrome evaluation sigma | 14.632421 | 15.121829 | +0.489408 |
| Bolt | sample row queries | 6.644379 | 6.853333 | +0.208954 |
| X | copy and serialize queried row values | 17.526825 | 22.958883 | +5.432058 |
| X | collect and serialize row authentication paths | 17.996917 | 5.765350 | -12.231567 |
| Y | copy and serialize queried row values | 0.288263 | 0.341867 | +0.053604 |
| Y | collect and serialize row authentication paths | 0.469417 | 0.688200 | +0.218784 |
| Bolt | project queried rows and sample batching challenges | 1.671537 | 1.726350 | +0.054812 |
| Bolt | systematic query covector | 11.337312 | 14.265162 | +2.927850 |
| Bolt | parity query covector and transpose RS FFT | 64.279904 | 64.639000 | +0.359096 |
| Bolt | public H transpose covector | 725.771371 | 742.002359 | +16.230988 |
| x | common WHIR constrained proof | 1036.430950 | 1074.391279 | +37.960329 |
| u | common WHIR constrained proof | 107.865842 | 113.622604 | +5.756762 |
| other | unassigned phase work and timing overhead | 9.557704 | 15.540054 | +5.982350 |
| X | Canopy ancestor claims | -0.000000 | 5.050175 | +5.050175 |
| X | Canopy SEP batching covector | -0.000000 | 5.467463 | +5.467463 |
| X | Canopy OOD evaluations | -0.000000 | 9.180967 | +9.180967 |
| X | Canopy WHIR SEP proof | -0.000000 | 368.142213 | +368.142213 |
| Y | Canopy ancestor claims | -0.000000 | 0.001162 | +0.001162 |
| Y | Canopy constant opening | -0.000000 | 0.001417 | +0.001417 |
| **Subtotal** | **opening** | **3161.880071** | **3675.125983** | **+513.245913** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 4583.970412 | 4606.043541 | 4335.056499 | 4964.853417 | 184.737023 | 1422.090341 | 3161.880071 | 10 |
| bolt_max_canopy_equal_full | 5778.802096 | 5758.267374 | 5378.029250 | 6484.794625 | 336.668066 | 2103.676112 | 3675.125983 | 10 |
