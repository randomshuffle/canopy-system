# Bolt versus full-input rate-1/2 RS encoding audit

N=2^22, width 4, 256-bit prime field, 8 Rayon threads, 2 warmups and 5 measured repetitions. Seed 4778121654452702769. Encoder only: no Merkle hashing, H2F, WHIR proof, or setup. Source preparation is excluded; output allocation and internal scratch are included, output destruction excluded. Four execution orders are rotated. Both full-input RS baselines have the same initial width as Bolt. The Canopy-sized RS call uses WHIR width 8.

Bolt emits N/4 parity elements plus the unchanged N-element systematic input; it also retains an N/8 syndrome scratch/witness. Its encoder does not copy the systematic input. Full RS emits 2N elements.

| Encoder | Input field elements | New codeword elements | Mean ms | Median ms | Min ms | Max ms |
|---|---:|---:|---:|---:|---:|---:|
| bolt_max_encoding | 4194304 | 1048576 | 235.697 | 222.680 | 215.663 | 285.251 |
| full_input_rs_arkworks | 4194304 | 8388608 | 655.960 | 614.089 | 602.571 | 756.143 |
| full_input_rs_whir_backend | 4194304 | 8388608 | 673.068 | 651.232 | 628.971 | 802.560 |
| canopy_systematic_rs_only | 131072 | 262144 | 17.055 | 17.409 | 13.548 | 22.676 |

This audits the current prime-field implementations. It does not reproduce the paper's optimized binary-field CPU/GPU commitment benchmark. Small and large RS workloads need not scale linearly.
