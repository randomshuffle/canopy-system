# Bolt-max prover stage profile

Input 2^24, width 4, 256-bit field, BLAKE3, 8 Rayon threads. 2 warmups and 10 measured repetitions per scheme; every proof verified. Heights are selected for proof size. This is the same public-H prime-field Bolt-max adaptation in both columns: native constrained WHIR proofs are common to both. Canopy adds RS commitments and WHIR SEP proofs. The 128-bit per-event target is conditional on LDPC distance 0.013.

All component times below are arithmetic means in milliseconds. They are disjoint wall-clock intervals and sum to the mean phase and total times. They are not sums of parallel-worker CPU time. Medians of separate stages generally do not add; median totals and their spread appear separately below.

Setup, random input generation, verifier work, and proof-object extraction/final cleanup outside the established phase timers are excluded. The residual includes allocation/drop, timer bookkeeping, and untimed work inside each phase. RS encode plus WHIR oracle commitment is measured as one native library call, including encoding, matrix hashing, allocation, and transcript binding; it is not an isolated FFT measurement. Internal WHIR proof times include sumchecks, recursive commitments, query openings, and terminal messages.
Canopy RS block lengths: systematic k/2 and parity k/2; total k, 80% of Bolt's encoded rows. Heights start at 2 for systematic and 0 for parity.

## commitment

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | sparse syndrome U=HX | 657.679521 | 667.167913 | +9.488392 |
| Bolt | parity RS encoding Y=RS(U) | 280.704708 | 279.909221 | -0.795487 |
| X | row Merkle tree or forest | 393.457838 | 360.564917 | -32.892921 |
| X | Merkle root transcript | 0.000154 | -0.000000 | -0.000154 |
| Y | row Merkle tree or forest | 98.936267 | 99.619704 | +0.683437 |
| Y | Merkle root transcript | 0.001450 | -0.000000 | -0.001450 |
| other | unassigned phase work and timing overhead | 0.002466 | 0.002195 | -0.000271 |
| X | Canopy hash-to-field | -0.000000 | 195.895787 | +195.895787 |
| X | Canopy WHIR configuration | -0.000000 | 0.008221 | +0.008221 |
| X | Canopy RS encode and WHIR oracle commitment | -0.000000 | 145.636350 | +145.636350 |
| Y | Canopy hash-to-field | -0.000000 | 0.000767 | +0.000767 |
| Y | Canopy constant RS commitment | -0.000000 | 0.005821 | +0.005821 |
| **Subtotal** | **commitment** | **1430.782404** | **1748.810895** | **+318.028491** |

## opening

| Part | Stage | Merkle mean ms | Canopy mean ms | Canopy minus Merkle ms |
|---|---|---:|---:|---:|
| Bolt | partial evaluation g | 406.460562 | 383.933583 | -22.526979 |
| Bolt | column projections x=Xr and u=Ur | 71.226804 | 76.641683 | +5.414879 |
| common WHIR | WHIR configuration | 0.010892 | 0.011604 | +0.000712 |
| x | common WHIR RS encode and oracle commitment | 679.190900 | 672.968904 | -6.221996 |
| u | common WHIR RS encode and oracle commitment | 68.017309 | 66.686217 | -1.331092 |
| Bolt | syndrome evaluation sigma | 15.036858 | 14.873142 | -0.163717 |
| Bolt | sample row queries | 6.716925 | 6.701704 | -0.015221 |
| X | copy and serialize queried row values | 17.571388 | 17.254717 | -0.316671 |
| X | collect and serialize row authentication paths | 19.329559 | 6.047217 | -13.282342 |
| Y | copy and serialize queried row values | 0.263571 | 0.266684 | +0.003113 |
| Y | collect and serialize row authentication paths | 0.579092 | 0.442404 | -0.136687 |
| Bolt | project queried rows and sample batching challenges | 1.687329 | 1.838775 | +0.151446 |
| Bolt | systematic query covector | 14.594463 | 13.295008 | -1.299454 |
| Bolt | parity query covector and transpose RS FFT | 67.563825 | 65.689563 | -1.874263 |
| Bolt | public H transpose covector | 746.317812 | 754.644333 | +8.326521 |
| x | common WHIR constrained proof | 997.640500 | 1118.231846 | +120.591346 |
| u | common WHIR constrained proof | 110.067025 | 114.614154 | +4.547129 |
| other | unassigned phase work and timing overhead | 8.528416 | 14.600324 | +6.071908 |
| X | Canopy ancestor claims | -0.000000 | 2.690087 | +2.690087 |
| X | Canopy SEP batching covector | -0.000000 | 4.818338 | +4.818338 |
| X | Canopy OOD evaluations | -0.000000 | 5.353563 | +5.353563 |
| X | Canopy WHIR SEP proof | -0.000000 | 181.651679 | +181.651679 |
| Y | Canopy ancestor claims | -0.000000 | 0.001583 | +0.001583 |
| Y | Canopy constant opening | -0.000000 | 0.001117 | +0.001117 |
| **Subtotal** | **opening** | **3230.803229** | **3523.258229** | **+292.455000** |

## Total timings and variation

| Scheme | Total mean ms | Total median ms | Minimum ms | Maximum ms | Sample standard deviation ms | Commit mean ms | Opening mean ms | Verified proofs |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bolt_max_merkle | 4661.585633 | 4573.364709 | 4465.345293 | 5684.279874 | 362.472389 | 1430.782404 | 3230.803229 | 10 |
| bolt_max_canopy_equal_half | 5272.069124 | 5138.512249 | 4955.327666 | 6264.227125 | 382.647704 | 1748.810895 | 3523.258229 | 10 |
