# Bolt-max prover timings

Input 2^22, width 4, 256-bit field, BLAKE3-256 commitments. 2 warmups, 10 measured repetitions, 8 Rayon threads. Every proof verified.

Canopy RS block lengths: systematic k/4 and parity k/4; total k/2, 40% of Bolt's encoded rows. Heights start at 3 for systematic and 1 for parity.

CPU-only, same encoder/hash code in both schemes. Public H setup and random input generation excluded. Commitment includes encoding. Opening includes g, projection, constrained WHIR proofs, row openings, and Canopy SEP proofs. Verification evaluates public H directly; no sparse-PCS preprocessing claim is made. The 128-bit per-event target is conditional on prime-field LDPC distance 0.013.

| scheme | h X / Y | commit median s | opening median s | total prover median s | verify median s | actual proof MiB |
|---|:---:|---:|---:|---:|---:|---:|
| bolt_max_canopy_equal_blocks | 3 / 18 | 0.349920 | 0.679052 | 1.027973 | 0.217774 | 5.415 |
| bolt_max_merkle | - / - | 0.313813 | 0.662249 | 0.976739 | 0.238138 | 6.460 |
