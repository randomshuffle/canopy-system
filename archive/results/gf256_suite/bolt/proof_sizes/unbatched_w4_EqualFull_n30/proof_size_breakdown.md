# Bolt-max versus Canopy: proof-size breakdown

These are the saved 256-bit-prime/BLAKE3 experiment parameters, not the published binary-field Bolt benchmark. Each row is a mean over the original sample seeds. Sizes include initial commitment roots; exclude serializer framing and sparse-PCS preprocessing. The per-event 128-bit target is conditional on LDPC distance 0.013. Both schemes use the same common constrained-WHIR proofs.

1 MiB = 1,048,576 bytes; 1 KiB = 1,024 bytes. Every component below reconciles to proof_sizes.csv.

Canopy RS block lengths: systematic k and parity k; total 2k, 160% of Bolt's encoded rows. Heights start at 1 for systematic and 0 for parity.

## Input 2^30: width=4, h_X=1, h_Y=26

| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |
|---|---:|---:|
| Bolt parity bottom authentication paths | 0.00000 | 167.32875 |
| Bolt parity commitment root | 0.03125 | 0.00000 |
| Bolt parity full authentication paths | 167.32875 | 0.00000 |
| Bolt parity opened row values | 39.87500 | 39.87500 |
| Bolt partial evaluation g | 0.12500 | 0.12500 |
| Bolt syndrome evaluation sigma | 0.03125 | 0.03125 |
| Bolt systematic bottom authentication paths | 0.00000 | 658.34937 |
| Bolt systematic commitment root | 0.03125 | 0.00000 |
| Bolt systematic full authentication paths | 8393.77000 | 0.00000 |
| Bolt systematic opened row values | 2633.62500 | 2633.62500 |
| Canopy parity constant RS root | 0.00000 | 0.03125 |
| Canopy parity constant coefficient | 0.00000 | 0.03125 |
| Canopy systematic WHIR: OOD evaluations | 0.00000 | 0.28125 |
| Canopy systematic WHIR: authentication paths | 0.00000 | 304.46063 |
| Canopy systematic WHIR: initial commitment root | 0.00000 | 0.03125 |
| Canopy systematic WHIR: queried row values | 0.00000 | 156.00000 |
| Canopy systematic WHIR: recursive commitment roots | 0.00000 | 0.25000 |
| Canopy systematic WHIR: sumcheck messages | 0.00000 | 1.68750 |
| Canopy systematic WHIR: terminal coefficients | 0.00000 | 0.03125 |
| common message WHIR: authentication paths | 650.73812 | 650.73812 |
| common message WHIR: initial commitment root | 0.03125 | 0.03125 |
| common message WHIR: queried row values | 364.50000 | 364.50000 |
| common message WHIR: recursive commitment roots | 0.25000 | 0.25000 |
| common message WHIR: sumcheck messages | 1.75000 | 1.75000 |
| common message WHIR: terminal coefficients | 0.06250 | 0.06250 |
| common syndrome WHIR: authentication paths | 484.00969 | 484.00969 |
| common syndrome WHIR: initial commitment root | 0.03125 | 0.03125 |
| common syndrome WHIR: queried row values | 330.25000 | 330.25000 |
| common syndrome WHIR: recursive commitment roots | 0.21875 | 0.21875 |
| common syndrome WHIR: sumcheck messages | 1.56250 | 1.56250 |
| common syndrome WHIR: terminal coefficients | 0.06250 | 0.06250 |
| **Total hashes** | **9696.44031** | **2265.73031** |
| **Total field-value bytes** | **3371.84375** | **3529.87500** |
| **Total proof (MiB)** | **12.761996** | **5.659771** |

## Interpretation

Each opened Bolt row contains the selected interleaving width in field elements, each 32 bytes. Canopy changes authentication, not these row values. The common message/syndrome WHIR proofs, g, and sigma also remain unchanged. At the full-height endpoint, a Canopy part retains the complete bottom tree and adds a 32-byte constant coefficient; its listed RS block is not transmitted. Nonconstant Canopy parts add an RS/SEP proof to replace the upper authentication paths. WHIR retains repeated queried row values but merges their Merkle paths; the per-oracle CSV makes that cost visible.
