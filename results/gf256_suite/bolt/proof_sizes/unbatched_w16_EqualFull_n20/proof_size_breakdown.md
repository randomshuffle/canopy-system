# Bolt-max versus Canopy: proof-size breakdown

These are the saved 256-bit-prime/BLAKE3 experiment parameters, not the published binary-field Bolt benchmark. Each row is a mean over the original sample seeds. Sizes include initial commitment roots; exclude serializer framing and sparse-PCS preprocessing. The per-event 128-bit target is conditional on LDPC distance 0.013. Both schemes use the same common constrained-WHIR proofs.

1 MiB = 1,048,576 bytes; 1 KiB = 1,024 bytes. Every component below reconciles to proof_sizes.csv.

Canopy RS block lengths: systematic k and parity k; total 2k, 160% of Bolt's encoded rows. Heights start at 1 for systematic and 0 for parity.

## Input 2^20: width=16, h_X=1, h_Y=14

| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |
|---|---:|---:|
| Bolt parity bottom authentication paths | 0.00000 | 48.32969 |
| Bolt parity commitment root | 0.03125 | 0.00000 |
| Bolt parity full authentication paths | 48.32969 | 0.00000 |
| Bolt parity opened row values | 159.50000 | 159.50000 |
| Bolt partial evaluation g | 0.50000 | 0.50000 |
| Bolt syndrome evaluation sigma | 0.03125 | 0.03125 |
| Bolt systematic bottom authentication paths | 0.00000 | 446.60125 |
| Bolt systematic commitment root | 0.03125 | 0.00000 |
| Bolt systematic full authentication paths | 797.97844 | 0.00000 |
| Bolt systematic opened row values | 10534.50000 | 10534.50000 |
| Canopy parity constant RS root | 0.00000 | 0.03125 |
| Canopy parity constant coefficient | 0.00000 | 0.03125 |
| Canopy systematic WHIR: OOD evaluations | 0.00000 | 0.15625 |
| Canopy systematic WHIR: authentication paths | 0.00000 | 67.52719 |
| Canopy systematic WHIR: initial commitment root | 0.00000 | 0.03125 |
| Canopy systematic WHIR: queried row values | 0.00000 | 135.00000 |
| Canopy systematic WHIR: recursive commitment roots | 0.00000 | 0.12500 |
| Canopy systematic WHIR: sumcheck messages | 0.00000 | 0.93750 |
| Canopy systematic WHIR: terminal coefficients | 0.00000 | 0.03125 |
| common message WHIR: authentication paths | 114.29781 | 114.29781 |
| common message WHIR: initial commitment root | 0.03125 | 0.03125 |
| common message WHIR: queried row values | 227.50000 | 227.50000 |
| common message WHIR: recursive commitment roots | 0.12500 | 0.12500 |
| common message WHIR: sumcheck messages | 1.00000 | 1.00000 |
| common message WHIR: terminal coefficients | 0.06250 | 0.06250 |
| common syndrome WHIR: authentication paths | 39.50531 | 39.50531 |
| common syndrome WHIR: initial commitment root | 0.03125 | 0.03125 |
| common syndrome WHIR: queried row values | 193.25000 | 193.25000 |
| common syndrome WHIR: recursive commitment roots | 0.09375 | 0.09375 |
| common syndrome WHIR: sumcheck messages | 0.81250 | 0.81250 |
| common syndrome WHIR: terminal coefficients | 0.06250 | 0.06250 |
| **Total hashes** | **1000.45500** | **716.73000** |
| **Total field-value bytes** | **11117.21875** | **11253.37500** |
| **Total proof (MiB)** | **11.833666** | **11.689556** |

## Interpretation

Each opened Bolt row contains the selected interleaving width in field elements, each 32 bytes. Canopy changes authentication, not these row values. The common message/syndrome WHIR proofs, g, and sigma also remain unchanged. At the full-height endpoint, a Canopy part retains the complete bottom tree and adds a 32-byte constant coefficient; its listed RS block is not transmitted. Nonconstant Canopy parts add an RS/SEP proof to replace the upper authentication paths. WHIR retains repeated queried row values but merges their Merkle paths; the per-oracle CSV makes that cost visible.
