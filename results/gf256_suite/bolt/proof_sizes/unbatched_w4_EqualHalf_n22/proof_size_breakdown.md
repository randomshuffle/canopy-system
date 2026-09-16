# Bolt-max versus Canopy: proof-size breakdown

These are the saved 256-bit-prime/BLAKE3 experiment parameters, not the published binary-field Bolt benchmark. Each row is a mean over the original sample seeds. Sizes include initial commitment roots; exclude serializer framing and sparse-PCS preprocessing. The per-event 128-bit target is conditional on LDPC distance 0.013. Both schemes use the same common constrained-WHIR proofs.

1 MiB = 1,048,576 bytes; 1 KiB = 1,024 bytes. Every component below reconciles to proof_sizes.csv.

Canopy RS block lengths: systematic k/2 and parity k/2; total k, 80% of Bolt's encoded rows. Heights start at 2 for systematic and 0 for parity.

## Input 2^22: width=4, h_X=2, h_Y=18

| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |
|---|---:|---:|
| Bolt parity bottom authentication paths | 0.00000 | 87.78562 |
| Bolt parity commitment root | 0.03125 | 0.00000 |
| Bolt parity full authentication paths | 87.78562 | 0.00000 |
| Bolt parity opened row values | 39.87500 | 39.87500 |
| Bolt partial evaluation g | 0.12500 | 0.12500 |
| Bolt syndrome evaluation sigma | 0.03125 | 0.03125 |
| Bolt systematic bottom authentication paths | 0.00000 | 1271.18375 |
| Bolt systematic commitment root | 0.03125 | 0.00000 |
| Bolt systematic full authentication paths | 3168.39031 | 0.00000 |
| Bolt systematic opened row values | 2633.62500 | 2633.62500 |
| Canopy parity constant RS root | 0.00000 | 0.03125 |
| Canopy parity constant coefficient | 0.00000 | 0.03125 |
| Canopy systematic WHIR: OOD evaluations | 0.00000 | 0.18750 |
| Canopy systematic WHIR: authentication paths | 0.00000 | 121.81813 |
| Canopy systematic WHIR: initial commitment root | 0.00000 | 0.03125 |
| Canopy systematic WHIR: queried row values | 0.00000 | 141.50000 |
| Canopy systematic WHIR: recursive commitment roots | 0.00000 | 0.15625 |
| Canopy systematic WHIR: sumcheck messages | 0.00000 | 1.12500 |
| Canopy systematic WHIR: terminal coefficients | 0.00000 | 0.03125 |
| common message WHIR: authentication paths | 248.06719 | 248.06719 |
| common message WHIR: initial commitment root | 0.03125 | 0.03125 |
| common message WHIR: queried row values | 261.75000 | 261.75000 |
| common message WHIR: recursive commitment roots | 0.15625 | 0.15625 |
| common message WHIR: sumcheck messages | 1.25000 | 1.25000 |
| common message WHIR: terminal coefficients | 0.12500 | 0.12500 |
| common syndrome WHIR: authentication paths | 141.80313 | 141.80313 |
| common syndrome WHIR: initial commitment root | 0.03125 | 0.03125 |
| common syndrome WHIR: queried row values | 227.50000 | 227.50000 |
| common syndrome WHIR: recursive commitment roots | 0.12500 | 0.12500 |
| common syndrome WHIR: sumcheck messages | 1.06250 | 1.06250 |
| common syndrome WHIR: terminal coefficients | 0.12500 | 0.12500 |
| **Total hashes** | **3646.45250** | **1871.22031** |
| **Total field-value bytes** | **3165.46875** | **3308.34375** |
| **Total proof (MiB)** | **6.652267** | **5.058168** |

## Interpretation

Each opened Bolt row contains the selected interleaving width in field elements, each 32 bytes. Canopy changes authentication, not these row values. The common message/syndrome WHIR proofs, g, and sigma also remain unchanged. At the full-height endpoint, a Canopy part retains the complete bottom tree and adds a 32-byte constant coefficient; its listed RS block is not transmitted. Nonconstant Canopy parts add an RS/SEP proof to replace the upper authentication paths. WHIR retains repeated queried row values but merges their Merkle paths; the per-oracle CSV makes that cost visible.
