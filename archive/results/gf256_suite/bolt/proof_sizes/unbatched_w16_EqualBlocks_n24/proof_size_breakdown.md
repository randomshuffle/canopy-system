# Bolt-max versus Canopy: proof-size breakdown

These are the saved 256-bit-prime/BLAKE3 experiment parameters, not the published binary-field Bolt benchmark. Each row is a mean over the original sample seeds. Sizes include initial commitment roots; exclude serializer framing and sparse-PCS preprocessing. The per-event 128-bit target is conditional on LDPC distance 0.013. Both schemes use the same common constrained-WHIR proofs.

1 MiB = 1,048,576 bytes; 1 KiB = 1,024 bytes. Every component below reconciles to proof_sizes.csv.

Canopy RS block lengths: systematic k/4 and parity k/4; total k/2, 40% of Bolt's encoded rows. Heights start at 3 for systematic and 1 for parity.

## Input 2^24: width=16, h_X=3, h_Y=18

| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |
|---|---:|---:|
| Bolt parity bottom authentication paths | 0.00000 | 87.76562 |
| Bolt parity commitment root | 0.03125 | 0.00000 |
| Bolt parity full authentication paths | 87.76562 | 0.00000 |
| Bolt parity opened row values | 159.50000 | 159.50000 |
| Bolt partial evaluation g | 0.50000 | 0.50000 |
| Bolt syndrome evaluation sigma | 0.03125 | 0.03125 |
| Bolt systematic bottom authentication paths | 0.00000 | 1860.31531 |
| Bolt systematic commitment root | 0.03125 | 0.00000 |
| Bolt systematic full authentication paths | 3168.25156 | 0.00000 |
| Bolt systematic opened row values | 10534.50000 | 10534.50000 |
| Canopy parity constant RS root | 0.00000 | 0.03125 |
| Canopy parity constant coefficient | 0.00000 | 0.03125 |
| Canopy systematic WHIR: OOD evaluations | 0.00000 | 0.15625 |
| Canopy systematic WHIR: authentication paths | 0.00000 | 100.75312 |
| Canopy systematic WHIR: initial commitment root | 0.00000 | 0.03125 |
| Canopy systematic WHIR: queried row values | 0.00000 | 135.00000 |
| Canopy systematic WHIR: recursive commitment roots | 0.00000 | 0.12500 |
| Canopy systematic WHIR: sumcheck messages | 0.00000 | 1.06250 |
| Canopy systematic WHIR: terminal coefficients | 0.00000 | 0.12500 |
| common message WHIR: authentication paths | 248.06750 | 248.06750 |
| common message WHIR: initial commitment root | 0.03125 | 0.03125 |
| common message WHIR: queried row values | 261.75000 | 261.75000 |
| common message WHIR: recursive commitment roots | 0.15625 | 0.15625 |
| common message WHIR: sumcheck messages | 1.25000 | 1.25000 |
| common message WHIR: terminal coefficients | 0.12500 | 0.12500 |
| common syndrome WHIR: authentication paths | 141.84063 | 141.84063 |
| common syndrome WHIR: initial commitment root | 0.03125 | 0.03125 |
| common syndrome WHIR: queried row values | 227.50000 | 227.50000 |
| common syndrome WHIR: recursive commitment roots | 0.12500 | 0.12500 |
| common syndrome WHIR: sumcheck messages | 1.06250 | 1.06250 |
| common syndrome WHIR: terminal coefficients | 0.12500 | 0.12500 |
| **Total hashes** | **3646.33156** | **2439.27344** |
| **Total field-value bytes** | **11186.34375** | **11322.71875** |
| **Total proof (MiB)** | **14.485034** | **13.439445** |

## Interpretation

Each opened Bolt row contains the selected interleaving width in field elements, each 32 bytes. Canopy changes authentication, not these row values. The common message/syndrome WHIR proofs, g, and sigma also remain unchanged. At the full-height endpoint, a Canopy part retains the complete bottom tree and adds a 32-byte constant coefficient; its listed RS block is not transmitted. Nonconstant Canopy parts add an RS/SEP proof to replace the upper authentication paths. WHIR retains repeated queried row values but merges their Merkle paths; the per-oracle CSV makes that cost visible.
