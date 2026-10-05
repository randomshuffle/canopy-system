# Bolt-max versus Canopy: proof-size breakdown

These are the saved 256-bit-prime/BLAKE3 experiment parameters, not the published binary-field Bolt benchmark. Each row is a mean over the original sample seeds. Sizes include initial commitment roots; exclude serializer framing and sparse-PCS preprocessing. The per-event 128-bit target is conditional on LDPC distance 0.013. Both schemes use the same common constrained-WHIR proofs.

1 MiB = 1,048,576 bytes; 1 KiB = 1,024 bytes. Every component below reconciles to proof_sizes.csv.

Canopy RS block lengths: systematic k and parity k; total 2k, 160% of Bolt's encoded rows. Heights start at 1 for systematic and 0 for parity.

## Input 2^30: width=8, h_X=1, h_Y=25

| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |
|---|---:|---:|
| Bolt parity bottom authentication paths | 0.00000 | 157.56594 |
| Bolt parity commitment root | 0.03125 | 0.00000 |
| Bolt parity full authentication paths | 157.56594 | 0.00000 |
| Bolt parity opened row values | 79.75000 | 79.75000 |
| Bolt partial evaluation g | 0.25000 | 0.25000 |
| Bolt syndrome evaluation sigma | 0.03125 | 0.03125 |
| Bolt systematic bottom authentication paths | 0.00000 | 658.27375 |
| Bolt systematic commitment root | 0.03125 | 0.00000 |
| Bolt systematic full authentication paths | 7734.49281 | 0.00000 |
| Bolt systematic opened row values | 5267.25000 | 5267.25000 |
| Canopy parity constant RS root | 0.00000 | 0.03125 |
| Canopy parity constant coefficient | 0.00000 | 0.03125 |
| Canopy systematic WHIR: OOD evaluations | 0.00000 | 0.25000 |
| Canopy systematic WHIR: authentication paths | 0.00000 | 279.04250 |
| Canopy systematic WHIR: initial commitment root | 0.00000 | 0.03125 |
| Canopy systematic WHIR: queried row values | 0.00000 | 151.75000 |
| Canopy systematic WHIR: recursive commitment roots | 0.00000 | 0.21875 |
| Canopy systematic WHIR: sumcheck messages | 0.00000 | 1.62500 |
| Canopy systematic WHIR: terminal coefficients | 0.00000 | 0.12500 |
| common message WHIR: authentication paths | 605.44844 | 605.44844 |
| common message WHIR: initial commitment root | 0.03125 | 0.03125 |
| common message WHIR: queried row values | 364.50000 | 364.50000 |
| common message WHIR: recursive commitment roots | 0.25000 | 0.25000 |
| common message WHIR: sumcheck messages | 1.68750 | 1.68750 |
| common message WHIR: terminal coefficients | 0.03125 | 0.03125 |
| common syndrome WHIR: authentication paths | 442.83906 | 442.83906 |
| common syndrome WHIR: initial commitment root | 0.03125 | 0.03125 |
| common syndrome WHIR: queried row values | 330.25000 | 330.25000 |
| common syndrome WHIR: recursive commitment roots | 0.21875 | 0.21875 |
| common syndrome WHIR: sumcheck messages | 1.50000 | 1.50000 |
| common syndrome WHIR: terminal coefficients | 0.03125 | 0.03125 |
| **Total hashes** | **8940.94000** | **2143.98219** |
| **Total field-value bytes** | **6045.28125** | **6199.06250** |
| **Total proof (MiB)** | **14.634982** | **8.147505** |

## Interpretation

Each opened Bolt row contains the selected interleaving width in field elements, each 32 bytes. Canopy changes authentication, not these row values. The common message/syndrome WHIR proofs, g, and sigma also remain unchanged. At the full-height endpoint, a Canopy part retains the complete bottom tree and adds a 32-byte constant coefficient; its listed RS block is not transmitted. Nonconstant Canopy parts add an RS/SEP proof to replace the upper authentication paths. WHIR retains repeated queried row values but merges their Merkle paths; the per-oracle CSV makes that cost visible.
