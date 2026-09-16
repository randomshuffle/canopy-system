# Bolt-max versus Canopy: proof-size breakdown

These are the saved 256-bit-prime/BLAKE3 experiment parameters, not the published binary-field Bolt benchmark. Each row is a mean over the original sample seeds. Sizes include initial commitment roots; exclude serializer framing and sparse-PCS preprocessing. The per-event 128-bit target is conditional on LDPC distance 0.013. Both schemes use the same common constrained-WHIR proofs.

1 MiB = 1,048,576 bytes; 1 KiB = 1,024 bytes. Every component below reconciles to proof_sizes.csv.

Canopy RS block lengths: systematic k/2 and parity k/2; total k, 80% of Bolt's encoded rows. Heights start at 2 for systematic and 0 for parity.

## Input 2^18: width=16, h_X=14, h_Y=12

| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |
|---|---:|---:|
| Bolt parity bottom authentication paths | 0.00000 | 29.64000 |
| Bolt parity commitment root | 0.03125 | 0.00000 |
| Bolt parity full authentication paths | 29.64000 | 0.00000 |
| Bolt parity opened row values | 159.50000 | 159.50000 |
| Bolt partial evaluation g | 0.50000 | 0.50000 |
| Bolt syndrome evaluation sigma | 0.03125 | 0.03125 |
| Bolt systematic commitment root | 0.03125 | 0.00000 |
| Bolt systematic opened row values | 8192.00000 | 8192.00000 |
| Canopy parity constant RS root | 0.00000 | 0.03125 |
| Canopy parity constant coefficient | 0.00000 | 0.03125 |
| Canopy systematic constant RS root | 0.00000 | 0.03125 |
| Canopy systematic constant coefficient | 0.00000 | 0.03125 |
| common message WHIR: authentication paths | 59.77000 | 59.77000 |
| common message WHIR: initial commitment root | 0.03125 | 0.03125 |
| common message WHIR: queried row values | 193.25000 | 193.25000 |
| common message WHIR: recursive commitment roots | 0.09375 | 0.09375 |
| common message WHIR: sumcheck messages | 0.87500 | 0.87500 |
| common message WHIR: terminal coefficients | 0.12500 | 0.12500 |
| common syndrome WHIR: authentication paths | 9.91031 | 9.91031 |
| common syndrome WHIR: initial commitment root | 0.03125 | 0.03125 |
| common syndrome WHIR: queried row values | 158.75000 | 158.75000 |
| common syndrome WHIR: recursive commitment roots | 0.06250 | 0.06250 |
| common syndrome WHIR: sumcheck messages | 0.68750 | 0.68750 |
| common syndrome WHIR: terminal coefficients | 0.12500 | 0.12500 |
| **Total hashes** | **99.60156** | **99.60156** |
| **Total field-value bytes** | **8705.84375** | **8705.90625** |
| **Total proof (MiB)** | **8.599068** | **8.599129** |

## Interpretation

Each opened Bolt row contains the selected interleaving width in field elements, each 32 bytes. Canopy changes authentication, not these row values. The common message/syndrome WHIR proofs, g, and sigma also remain unchanged. At the full-height endpoint, a Canopy part retains the complete bottom tree and adds a 32-byte constant coefficient; its listed RS block is not transmitted. Nonconstant Canopy parts add an RS/SEP proof to replace the upper authentication paths. WHIR retains repeated queried row values but merges their Merkle paths; the per-oracle CSV makes that cost visible.
