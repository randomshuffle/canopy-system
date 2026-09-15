# Bolt-max prover timings

Input 2^22, width 4, 256-bit field, BLAKE3-256 commitments. 1 warmups, 5 measured repetitions, 8 Rayon threads. Every proof verified.

CPU-only, same encoder/hash code in both schemes. Public H setup and random input generation excluded. Commitment includes encoding. Opening includes g, projection, constrained WHIR proofs, row openings, and Canopy SEP proofs. Verification evaluates public H directly; no sparse-PCS preprocessing claim is made. The 128-bit per-event target is conditional on prime-field LDPC distance 0.013.

| scheme | h X / Y | commit median s | opening median s | total prover median s | verify median s | actual proof MiB |
|---|:---:|---:|---:|---:|---:|---:|
| bolt_max_canopy_quarter | 3 / 18 | 0.353287 | 0.699486 | 1.052741 | 0.223608 | 5.414 |
| bolt_max_merkle | - / - | 0.320615 | 0.681431 | 1.001551 | 0.243355 | 6.460 |
