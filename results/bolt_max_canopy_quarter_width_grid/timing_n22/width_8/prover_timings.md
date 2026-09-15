# Bolt-max prover timings

Input 2^22, width 8, 256-bit field, BLAKE3-256 commitments. 1 warmups, 5 measured repetitions, 8 Rayon threads. Every proof verified.

CPU-only, same encoder/hash code in both schemes. Public H setup and random input generation excluded. Commitment includes encoding. Opening includes g, projection, constrained WHIR proofs, row openings, and Canopy SEP proofs. Verification evaluates public H directly; no sparse-PCS preprocessing claim is made. The 128-bit per-event target is conditional on prime-field LDPC distance 0.013.

| scheme | h X / Y | commit median s | opening median s | total prover median s | verify median s | actual proof MiB |
|---|:---:|---:|---:|---:|---:|---:|
| bolt_max_canopy_quarter | 3 / 17 | 0.294680 | 0.401987 | 0.698453 | 0.148208 | 7.770 |
| bolt_max_merkle | - / - | 0.274872 | 0.390753 | 0.664531 | 0.162028 | 8.325 |
