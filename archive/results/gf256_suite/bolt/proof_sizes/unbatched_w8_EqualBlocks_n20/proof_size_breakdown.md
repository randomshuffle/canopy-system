# Bolt-max versus Canopy: proof-size breakdown

These are the saved 256-bit-prime/BLAKE3 experiment parameters, not the published binary-field Bolt benchmark. Each row is a mean over the original sample seeds. Sizes include initial commitment roots; exclude serializer framing and sparse-PCS preprocessing. The per-event 128-bit target is conditional on LDPC distance 0.013. Both schemes use the same common constrained-WHIR proofs.

1 MiB = 1,048,576 bytes; 1 KiB = 1,024 bytes. Every component below reconciles to proof_sizes.csv.

Canopy RS block lengths: systematic k/4 and parity k/4; total k/2, 40% of Bolt's encoded rows. Heights start at 3 for systematic and 1 for parity.

## Input 2^20: width=8, h_X=17, h_Y=15

| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |
|---|---:|---:|
| Bolt parity bottom authentication paths | 0.00000 | 58.06219 |
| Bolt parity commitment root | 0.03125 | 0.00000 |
| Bolt parity full authentication paths | 58.06219 | 0.00000 |
| Bolt parity opened row values | 79.75000 | 79.75000 |
| Bolt partial evaluation g | 0.25000 | 0.25000 |
| Bolt syndrome evaluation sigma | 0.03125 | 0.03125 |
| Bolt systematic bottom authentication paths | 0.00000 | 1345.32625 |
| Bolt systematic commitment root | 0.03125 | 0.00000 |
| Bolt systematic full authentication paths | 1345.32625 | 0.00000 |
| Bolt systematic opened row values | 5267.25000 | 5267.25000 |
| Canopy parity constant RS root | 0.00000 | 0.03125 |
| Canopy parity constant coefficient | 0.00000 | 0.03125 |
| Canopy systematic constant RS root | 0.00000 | 0.03125 |
| Canopy systematic constant coefficient | 0.00000 | 0.03125 |
| common message WHIR: authentication paths | 141.98844 | 141.98844 |
| common message WHIR: initial commitment root | 0.03125 | 0.03125 |
| common message WHIR: queried row values | 227.50000 | 227.50000 |
| common message WHIR: recursive commitment roots | 0.12500 | 0.12500 |
| common message WHIR: sumcheck messages | 1.06250 | 1.06250 |
| common message WHIR: terminal coefficients | 0.12500 | 0.12500 |
| common syndrome WHIR: authentication paths | 59.67688 | 59.67688 |
| common syndrome WHIR: initial commitment root | 0.03125 | 0.03125 |
| common syndrome WHIR: queried row values | 193.25000 | 193.25000 |
| common syndrome WHIR: recursive commitment roots | 0.09375 | 0.09375 |
| common syndrome WHIR: sumcheck messages | 0.87500 | 0.87500 |
| common syndrome WHIR: terminal coefficients | 0.12500 | 0.12500 |
| **Total hashes** | **1605.39750** | **1605.39750** |
| **Total field-value bytes** | **5770.21875** | **5770.28125** |
| **Total proof (MiB)** | **7.202750** | **7.202811** |

## Interpretation

Each opened Bolt row contains the selected interleaving width in field elements, each 32 bytes. Canopy changes authentication, not these row values. The common message/syndrome WHIR proofs, g, and sigma also remain unchanged. At the full-height endpoint, a Canopy part retains the complete bottom tree and adds a 32-byte constant coefficient; its listed RS block is not transmitted. Nonconstant Canopy parts add an RS/SEP proof to replace the upper authentication paths. WHIR retains repeated queried row values but merges their Merkle paths; the per-oracle CSV makes that cost visible.
