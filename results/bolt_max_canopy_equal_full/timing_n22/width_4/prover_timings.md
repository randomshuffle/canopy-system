# Bolt-max prover timings

Input 2^22, width 4, 256-bit field, BLAKE3-256 commitments. 2 warmups, 10 measured repetitions, 8 Rayon threads. Every proof verified.

Canopy RS block lengths: systematic k and parity k; total 2k, 160% of Bolt's encoded rows. Heights start at 1 for systematic and 0 for parity.

CPU-only, same encoder/hash code in both schemes. Public H setup and random input generation excluded. Commitment includes encoding. Opening includes g, projection, constrained WHIR proofs, row openings, and Canopy SEP proofs. Verification evaluates public H directly; no sparse-PCS preprocessing claim is made. The 128-bit per-event target is conditional on prime-field LDPC distance 0.013.

| scheme | h X / Y | commit median s | opening median s | total prover median s | verify median s | actual proof MiB |
|---|:---:|---:|---:|---:|---:|---:|
| bolt_max_canopy_equal_full | 1 / 18 | 0.475360 | 0.798341 | 1.271845 | 0.229881 | 4.305 |
| bolt_max_merkle | - / - | 0.333479 | 0.715697 | 1.046714 | 0.263746 | 6.460 |
