# Bolt-max prover stage profile

Input 2^18, width 8, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 3.494004 | 3.732779 | +0.238775 |
| Bolt | parity RS encoding Y=RS(U) | 3.560804 | 3.556046 | -0.004758 |
| X | row Merkle tree or forest | 5.269517 | 5.379533 | +0.110016 |
| X | Merkle root transcript | 0.000183 | -0.000000 | -0.000183 |
| Y | row Merkle tree or forest | 1.577421 | 1.690355 | +0.112934 |
| Y | Merkle root transcript | 0.000600 | -0.000000 | -0.000600 |
| other | unassigned phase work and timing overhead | 0.001029 | 0.001121 | +0.000092 |
| X | Canopy hash-to-field | -0.000000 | 0.000487 | +0.000487 |
| X | Canopy constant RS commitment | -0.000000 | 0.002892 | +0.002892 |
| Y | Canopy hash-to-field | -0.000000 | 0.000412 | +0.000412 |
| Y | Canopy constant RS commitment | -0.000000 | 0.003112 | +0.003112 |
| **Subtotal** | **commitment** | **13.903558** | **14.366738** | **+0.463180** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 5.696246 | 5.826717 | +0.130471 |
| Bolt | column projections x=Xr and u=Ur | 1.323900 | 1.385392 | +0.061492 |
| common WHIR | WHIR configuration | 0.005329 | 0.005646 | +0.000317 |
| x | common WHIR RS encode and oracle commitment | 3.403692 | 3.579754 | +0.176062 |
| u | common WHIR RS encode and oracle commitment | 0.826825 | 0.795333 | -0.031492 |
| Bolt | syndrome evaluation sigma | 0.248308 | 0.244138 | -0.004171 |
| Bolt | sample row queries | 7.488929 | 7.430525 | -0.058404 |
| X | copy and serialize queried row values | 28.637708 | 28.972175 | +0.334467 |
| X | collect and serialize row authentication paths | 2.643216 | 2.661046 | +0.017830 |
| Y | copy and serialize queried row values | 0.442254 | 0.434312 | -0.007942 |
| Y | collect and serialize row authentication paths | 0.133883 | 0.137583 | +0.003700 |
| Bolt | project queried rows and sample batching challenges | 3.449596 | 3.571650 | +0.122055 |
| Bolt | systematic query covector | 0.914283 | 0.913287 | -0.000996 |
| Bolt | parity query covector and transpose RS FFT | 1.120271 | 1.061708 | -0.058563 |
| Bolt | public H transpose covector | 1.598833 | 1.580917 | -0.017917 |
| x | common WHIR constrained proof | 10.414233 | 10.323046 | -0.091188 |
| u | common WHIR constrained proof | 2.680162 | 2.625796 | -0.054367 |
| other | unassigned phase work and timing overhead | 0.009567 | 0.017062 | +0.007495 |
| X | Canopy ancestor claims | -0.000000 | 0.021775 | +0.021775 |
| X | Canopy constant opening | -0.000000 | 0.000433 | +0.000433 |
| Y | Canopy ancestor claims | -0.000000 | 0.000625 | +0.000625 |
| Y | Canopy constant opening | -0.000000 | 0.000267 | +0.000267 |
| **Subtotal** | **opening** | **71.037238** | **71.589187** | **+0.551950** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 84.940796 | 84.851854 | 82.777208 | 88.152166 | 1.905946 | 13.903558 | 71.037238 | 10 |
| bolt_max_canopy_quarter | 85.955925 | 85.688437 | 83.022958 | 89.606833 | 2.319629 | 14.366738 | 71.589187 | 10 |
