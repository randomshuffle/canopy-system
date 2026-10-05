# Bolt-max versus Canopy: proof-size breakdown

These are the saved 256-bit-prime/BLAKE3 experiment parameters, not the published binary-field Bolt benchmark. Each row is a mean over the original sample seeds. Sizes include initial commitment roots; exclude serializer framing and sparse-PCS preprocessing. The per-event 128-bit target is conditional on LDPC distance 0.013. Both schemes use the same common constrained-WHIR proofs.

1 MiB = 1,048,576 bytes; 1 KiB = 1,024 bytes. Every component below reconciles to proof_sizes.csv.

Canopy RS block lengths: systematic k/2 and parity k/2; total k, 80% of Bolt's encoded rows. Heights start at 2 for systematic and 0 for parity.

## Input 2^18: width=4, h_X=16, h_Y=14

| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |
|---|---:|---:|
| Bolt parity bottom authentication paths | 0.00000 | 48.35344 |
| Bolt parity commitment root | 0.03125 | 0.00000 |
| Bolt parity full authentication paths | 48.35344 | 0.00000 |
| Bolt parity opened row values | 39.87500 | 39.87500 |
| Bolt partial evaluation g | 0.12500 | 0.12500 |
| Bolt syndrome evaluation sigma | 0.03125 | 0.03125 |
| Bolt systematic bottom authentication paths | 0.00000 | 797.95188 |
| Bolt systematic commitment root | 0.03125 | 0.00000 |
| Bolt systematic full authentication paths | 797.95188 | 0.00000 |
| Bolt systematic opened row values | 2633.62500 | 2633.62500 |
| Canopy parity constant RS root | 0.00000 | 0.03125 |
| Canopy parity constant coefficient | 0.00000 | 0.03125 |
| Canopy systematic constant RS root | 0.00000 | 0.03125 |
| Canopy systematic constant coefficient | 0.00000 | 0.03125 |
| common message WHIR: authentication paths | 114.24406 | 114.24406 |
| common message WHIR: initial commitment root | 0.03125 | 0.03125 |
| common message WHIR: queried row values | 227.50000 | 227.50000 |
| common message WHIR: recursive commitment roots | 0.12500 | 0.12500 |
| common message WHIR: sumcheck messages | 1.00000 | 1.00000 |
| common message WHIR: terminal coefficients | 0.06250 | 0.06250 |
| common syndrome WHIR: authentication paths | 39.50875 | 39.50875 |
| common syndrome WHIR: initial commitment root | 0.03125 | 0.03125 |
| common syndrome WHIR: queried row values | 193.25000 | 193.25000 |
| common syndrome WHIR: recursive commitment roots | 0.09375 | 0.09375 |
| common syndrome WHIR: sumcheck messages | 0.81250 | 0.81250 |
| common syndrome WHIR: terminal coefficients | 0.06250 | 0.06250 |
| **Total hashes** | **1000.40188** | **1000.40188** |
| **Total field-value bytes** | **3096.34375** | **3096.40625** |
| **Total proof (MiB)** | **4.000728** | **4.000789** |

## Interpretation

Each opened Bolt row contains the selected interleaving width in field elements, each 32 bytes. Canopy changes authentication, not these row values. The common message/syndrome WHIR proofs, g, and sigma also remain unchanged. At the full-height endpoint, a Canopy part retains the complete bottom tree and adds a 32-byte constant coefficient; its listed RS block is not transmitted. Nonconstant Canopy parts add an RS/SEP proof to replace the upper authentication paths. WHIR retains repeated queried row values but merges their Merkle paths; the per-oracle CSV makes that cost visible.
