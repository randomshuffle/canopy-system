# Bolt-max prover stage profile

Input 2^22, width 4, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k and parity k; total 2k, 160% of Bolt's encoded rows. Heights start at 1 for systematic and 0 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 137.856842 | 139.427950 | +1.571108 |
| Bolt | parity RS encoding Y=RS(U) | 68.595683 | 62.458713 | -6.136971 |
| X | row Merkle tree or forest | 103.670975 | 81.039917 | -22.631058 |
| X | Merkle root transcript | 0.000196 | -0.000000 | -0.000196 |
| Y | row Merkle tree or forest | 28.384641 | 40.477037 | +12.092396 |
| Y | Merkle root transcript | 0.000854 | -0.000000 | -0.000854 |
| other | unassigned phase work and timing overhead | 0.001488 | 0.002167 | +0.000679 |
| X | Canopy hash-to-field | -0.000000 | 98.020996 | +98.020996 |
| X | Canopy WHIR configuration | -0.000000 | 0.007950 | +0.007950 |
| X | Canopy RS encode and WHIR oracle commitment | -0.000000 | 65.314633 | +65.314633 |
| Y | Canopy hash-to-field | -0.000000 | 0.000850 | +0.000850 |
| Y | Canopy constant RS commitment | -0.000000 | 0.006329 | +0.006329 |
| **Subtotal** | **commitment** | **338.510679** | **486.756542** | **+148.245862** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 92.697817 | 96.130379 | +3.432563 |
| Bolt | column projections x=Xr and u=Ur | 17.724158 | 17.794013 | +0.069854 |
| common WHIR | WHIR configuration | 0.010583 | 0.009221 | -0.001362 |
| x | common WHIR RS encode and oracle commitment | 133.273658 | 139.653616 | +6.379958 |
| u | common WHIR RS encode and oracle commitment | 13.650242 | 14.986021 | +1.335779 |
| Bolt | syndrome evaluation sigma | 3.813850 | 3.925454 | +0.111604 |
| Bolt | sample row queries | 6.434771 | 6.555079 | +0.120308 |
| X | copy and serialize queried row values | 15.157504 | 15.865766 | +0.708262 |
| X | collect and serialize row authentication paths | 13.469496 | 2.690900 | -10.778596 |
| Y | copy and serialize queried row values | 0.243408 | 0.257921 | +0.014513 |
| Y | collect and serialize row authentication paths | 0.324987 | 0.272663 | -0.052325 |
| Bolt | project queried rows and sample batching challenges | 1.647392 | 1.664350 | +0.016958 |
| Bolt | systematic query covector | 3.671837 | 3.560342 | -0.111496 |
| Bolt | parity query covector and transpose RS FFT | 15.678204 | 15.448921 | -0.229284 |
| Bolt | public H transpose covector | 116.915663 | 115.033633 | -1.882029 |
| x | common WHIR constrained proof | 229.227350 | 229.887937 | +0.660588 |
| u | common WHIR constrained proof | 28.730413 | 27.017121 | -1.713292 |
| other | unassigned phase work and timing overhead | 1.074229 | 1.699563 | +0.625334 |
| X | Canopy ancestor claims | -0.000000 | 0.192012 | +0.192012 |
| X | Canopy SEP batching covector | -0.000000 | 2.503317 | +2.503317 |
| X | Canopy OOD evaluations | -0.000000 | 2.669583 | +2.669583 |
| X | Canopy WHIR SEP proof | -0.000000 | 87.553521 | +87.553521 |
| Y | Canopy ancestor claims | -0.000000 | 0.000541 | +0.000541 |
| Y | Canopy constant opening | -0.000000 | 0.000317 | +0.000317 |
| **Subtotal** | **opening** | **693.745562** | **785.372192** | **+91.626629** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 1032.256242 | 1009.716771 | 973.757416 | 1194.215042 | 72.791437 | 338.510679 | 693.745562 | 10 |
| bolt_max_canopy_equal_full | 1272.128734 | 1264.001791 | 1191.324250 | 1491.489501 | 88.756868 | 486.756542 | 785.372192 | 10 |
