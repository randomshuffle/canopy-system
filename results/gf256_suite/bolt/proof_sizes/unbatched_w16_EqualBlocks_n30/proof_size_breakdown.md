# Bolt-max versus Canopy: proof-size breakdown

These are the saved 256-bit-prime/BLAKE3 experiment parameters, not the published binary-field Bolt benchmark. Each row is a mean over the original sample seeds. Sizes include initial commitment roots; exclude serializer framing and sparse-PCS preprocessing. The per-event 128-bit target is conditional on LDPC distance 0.013. Both schemes use the same common constrained-WHIR proofs.

1 MiB = 1,048,576 bytes; 1 KiB = 1,024 bytes. Every component below reconciles to proof_sizes.csv.

Canopy RS block lengths: systematic k/4 and parity k/4; total k/2, 40% of Bolt's encoded rows. Heights start at 3 for systematic and 1 for parity.

## Input 2^30: width=16, h_X=3, h_Y=24

| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |
|---|---:|---:|
| Bolt parity bottom authentication paths | 0.00000 | 147.46969 |
| Bolt parity commitment root | 0.03125 | 0.00000 |
| Bolt parity full authentication paths | 147.46969 | 0.00000 |
| Bolt parity opened row values | 159.50000 | 159.50000 |
| Bolt partial evaluation g | 0.50000 | 0.50000 |
| Bolt syndrome evaluation sigma | 0.03125 | 0.03125 |
| Bolt systematic bottom authentication paths | 0.00000 | 1973.30969 |
| Bolt systematic commitment root | 0.03125 | 0.00000 |
| Bolt systematic full authentication paths | 7076.44313 | 0.00000 |
| Bolt systematic opened row values | 10534.50000 | 10534.50000 |
| Canopy parity constant RS root | 0.00000 | 0.03125 |
| Canopy parity constant coefficient | 0.00000 | 0.03125 |
| Canopy systematic WHIR: OOD evaluations | 0.00000 | 0.21875 |
| Canopy systematic WHIR: authentication paths | 0.00000 | 216.92687 |
| Canopy systematic WHIR: initial commitment root | 0.00000 | 0.03125 |
| Canopy systematic WHIR: queried row values | 0.00000 | 147.00000 |
| Canopy systematic WHIR: recursive commitment roots | 0.00000 | 0.18750 |
| Canopy systematic WHIR: sumcheck messages | 0.00000 | 1.43750 |
| Canopy systematic WHIR: terminal coefficients | 0.00000 | 0.12500 |
| common message WHIR: authentication paths | 525.63094 | 525.63094 |
| common message WHIR: initial commitment root | 0.03125 | 0.03125 |
| common message WHIR: queried row values | 330.25000 | 330.25000 |
| common message WHIR: recursive commitment roots | 0.21875 | 0.21875 |
| common message WHIR: sumcheck messages | 1.62500 | 1.62500 |
| common message WHIR: terminal coefficients | 0.12500 | 0.12500 |
| common syndrome WHIR: authentication paths | 375.78594 | 375.78594 |
| common syndrome WHIR: initial commitment root | 0.03125 | 0.03125 |
| common syndrome WHIR: queried row values | 296.00000 | 296.00000 |
| common syndrome WHIR: recursive commitment roots | 0.18750 | 0.18750 |
| common syndrome WHIR: sumcheck messages | 1.43750 | 1.43750 |
| common syndrome WHIR: terminal coefficients | 0.12500 | 0.12500 |
| **Total hashes** | **8125.86094** | **3239.84188** |
| **Total field-value bytes** | **11324.09375** | **11472.90625** |
| **Total proof (MiB)** | **18.994096** | **14.367918** |

## Interpretation

Each opened Bolt row contains the selected interleaving width in field elements, each 32 bytes. Canopy changes authentication, not these row values. The common message/syndrome WHIR proofs, g, and sigma also remain unchanged. At the full-height endpoint, a Canopy part retains the complete bottom tree and adds a 32-byte constant coefficient; its listed RS block is not transmitted. Nonconstant Canopy parts add an RS/SEP proof to replace the upper authentication paths. WHIR retains repeated queried row values but merges their Merkle paths; the per-oracle CSV makes that cost visible.
