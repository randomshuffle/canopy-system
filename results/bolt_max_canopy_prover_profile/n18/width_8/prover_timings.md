# Bolt-max prover timings

Input 2^18, width 8, 256-bit field, BLAKE3-256 commitments. 2 warmups, 10 measured repetitions, 8 Rayon threads. Every proof verified.

CPU-only, same encoder/hash code in both schemes. Public H setup and random input generation excluded. Commitment includes encoding. Opening includes g, projection, constrained WHIR proofs, row openings, and Canopy SEP proofs. Verification evaluates public H directly; no sparse-PCS preprocessing claim is made. The 128-bit per-event target is conditional on prime-field LDPC distance 0.013.

| scheme | h X / Y | commit median s | opening median s | total prover median s | verify median s | actual proof MiB |
|---|:---:|---:|---:|---:|---:|---:|
| bolt_max_canopy_quarter | 15 / 13 | 0.013814 | 0.071571 | 0.085688 | 0.087091 | 5.896 |
| bolt_max_merkle | - / - | 0.013864 | 0.070577 | 0.084852 | 0.086480 | 5.893 |
