# Bolt-max versus Canopy: proof-size breakdown

These are the saved 256-bit-prime/BLAKE3 experiment parameters, not the published binary-field Bolt benchmark. Each row is a mean over the original sample seeds. Sizes include initial commitment roots; exclude serializer framing and sparse-PCS preprocessing. The per-event 128-bit target is conditional on LDPC distance 0.013. Both schemes use the same common constrained-WHIR proofs.

1 MiB = 1,048,576 bytes; 1 KiB = 1,024 bytes. Every component below reconciles to proof_sizes.csv.

## Input 2^18: h_X=11, h_Y=9

| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |
|---|---:|---:|
| Bolt parity bottom authentication paths | 0.00000 | 5.00063 |
| Bolt parity commitment root | 0.03125 | 0.00000 |
| Bolt parity full authentication paths | 5.00063 | 0.00000 |
| Bolt parity opened row values | 1236.00000 | 1236.00000 |
| Bolt partial evaluation g | 4.00000 | 4.00000 |
| Bolt syndrome evaluation sigma | 0.03125 | 0.03125 |
| Bolt systematic commitment root | 0.03125 | 0.00000 |
| Bolt systematic opened row values | 8192.00000 | 8192.00000 |
| Canopy parity constant RS root | 0.00000 | 0.03125 |
| Canopy parity constant coefficient | 0.00000 | 0.03125 |
| Canopy systematic constant RS root | 0.00000 | 0.03125 |
| Canopy systematic constant coefficient | 0.00000 | 0.03125 |
| common message WHIR: authentication paths | 10.06375 | 10.06375 |
| common message WHIR: initial commitment root | 0.03125 | 0.03125 |
| common message WHIR: queried row values | 149.50000 | 149.50000 |
| common message WHIR: recursive commitment roots | 0.06250 | 0.06250 |
| common message WHIR: sumcheck messages | 0.68750 | 0.68750 |
| common message WHIR: terminal coefficients | 0.12500 | 0.12500 |
| common syndrome WHIR: authentication paths | 0.01969 | 0.01969 |
| common syndrome WHIR: initial commitment root | 0.03125 | 0.03125 |
| common syndrome WHIR: queried row values | 116.00000 | 116.00000 |
| common syndrome WHIR: recursive commitment roots | 0.03125 | 0.03125 |
| common syndrome WHIR: sumcheck messages | 0.50000 | 0.50000 |
| common syndrome WHIR: terminal coefficients | 0.12500 | 0.12500 |
| **Total hashes** | **15.30281** | **15.30281** |
| **Total field-value bytes** | **9698.96875** | **9699.03125** |
| **Total proof (MiB)** | **9.486593** | **9.486654** |

## Input 2^22: h_X=15, h_Y=13

| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |
|---|---:|---:|
| Bolt parity bottom authentication paths | 0.00000 | 38.07187 |
| Bolt parity commitment root | 0.03125 | 0.00000 |
| Bolt parity full authentication paths | 38.07187 | 0.00000 |
| Bolt parity opened row values | 1236.00000 | 1236.00000 |
| Bolt partial evaluation g | 4.00000 | 4.00000 |
| Bolt syndrome evaluation sigma | 0.03125 | 0.03125 |
| Bolt systematic bottom authentication paths | 0.00000 | 307.81250 |
| Bolt systematic commitment root | 0.03125 | 0.00000 |
| Bolt systematic full authentication paths | 307.81250 | 0.00000 |
| Bolt systematic opened row values | 81724.00000 | 81724.00000 |
| Canopy parity constant RS root | 0.00000 | 0.03125 |
| Canopy parity constant coefficient | 0.00000 | 0.03125 |
| Canopy systematic constant RS root | 0.00000 | 0.03125 |
| Canopy systematic constant coefficient | 0.00000 | 0.03125 |
| common message WHIR: authentication paths | 84.80156 | 84.80156 |
| common message WHIR: initial commitment root | 0.03125 | 0.03125 |
| common message WHIR: queried row values | 214.25000 | 214.25000 |
| common message WHIR: recursive commitment roots | 0.12500 | 0.12500 |
| common message WHIR: sumcheck messages | 0.93750 | 0.93750 |
| common message WHIR: terminal coefficients | 0.03125 | 0.03125 |
| common syndrome WHIR: authentication paths | 22.43781 | 22.43781 |
| common syndrome WHIR: initial commitment root | 0.03125 | 0.03125 |
| common syndrome WHIR: queried row values | 182.00000 | 182.00000 |
| common syndrome WHIR: recursive commitment roots | 0.09375 | 0.09375 |
| common syndrome WHIR: sumcheck messages | 0.75000 | 0.75000 |
| common syndrome WHIR: terminal coefficients | 0.03125 | 0.03125 |
| **Total hashes** | **453.46750** | **453.46750** |
| **Total field-value bytes** | **83362.03125** | **83362.09375** |
| **Total proof (MiB)** | **81.851073** | **81.851134** |

## Input 2^26: h_X=3, h_Y=17

| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |
|---|---:|---:|
| Bolt parity bottom authentication paths | 0.00000 | 75.86750 |
| Bolt parity commitment root | 0.03125 | 0.00000 |
| Bolt parity full authentication paths | 75.86750 | 0.00000 |
| Bolt parity opened row values | 1236.00000 | 1236.00000 |
| Bolt partial evaluation g | 4.00000 | 4.00000 |
| Bolt syndrome evaluation sigma | 0.03125 | 0.03125 |
| Bolt systematic bottom authentication paths | 0.00000 | 1705.43000 |
| Bolt systematic commitment root | 0.03125 | 0.00000 |
| Bolt systematic full authentication paths | 2488.82625 | 0.00000 |
| Bolt systematic opened row values | 81724.00000 | 81724.00000 |
| Canopy parity constant RS root | 0.00000 | 0.03125 |
| Canopy parity constant coefficient | 0.00000 | 0.03125 |
| Canopy systematic WHIR: OOD evaluations | 0.00000 | 0.15625 |
| Canopy systematic WHIR: authentication paths | 0.00000 | 80.42938 |
| Canopy systematic WHIR: initial commitment root | 0.00000 | 0.03125 |
| Canopy systematic WHIR: queried row values | 0.00000 | 127.00000 |
| Canopy systematic WHIR: recursive commitment roots | 0.00000 | 0.12500 |
| Canopy systematic WHIR: sumcheck messages | 0.00000 | 1.00000 |
| Canopy systematic WHIR: terminal coefficients | 0.00000 | 0.06250 |
| common message WHIR: authentication paths | 205.62375 | 205.62375 |
| common message WHIR: initial commitment root | 0.03125 | 0.03125 |
| common message WHIR: queried row values | 246.50000 | 246.50000 |
| common message WHIR: recursive commitment roots | 0.15625 | 0.15625 |
| common message WHIR: sumcheck messages | 1.18750 | 1.18750 |
| common message WHIR: terminal coefficients | 0.06250 | 0.06250 |
| common syndrome WHIR: authentication paths | 109.97781 | 109.97781 |
| common syndrome WHIR: initial commitment root | 0.03125 | 0.03125 |
| common syndrome WHIR: queried row values | 214.25000 | 214.25000 |
| common syndrome WHIR: recursive commitment roots | 0.12500 | 0.12500 |
| common syndrome WHIR: sumcheck messages | 1.00000 | 1.00000 |
| common syndrome WHIR: terminal coefficients | 0.06250 | 0.06250 |
| **Total hashes** | **2880.70156** | **2177.85969** |
| **Total field-value bytes** | **83427.09375** | **83555.34375** |
| **Total proof (MiB)** | **84.284956** | **83.723831** |

## Input 2^30: h_X=3, h_Y=21

| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |
|---|---:|---:|
| Bolt parity bottom authentication paths | 0.00000 | 114.29500 |
| Bolt parity commitment root | 0.03125 | 0.00000 |
| Bolt parity full authentication paths | 114.29500 | 0.00000 |
| Bolt parity opened row values | 1236.00000 | 1236.00000 |
| Bolt partial evaluation g | 4.00000 | 4.00000 |
| Bolt syndrome evaluation sigma | 0.03125 | 0.03125 |
| Bolt systematic bottom authentication paths | 0.00000 | 1901.64375 |
| Bolt systematic commitment root | 0.03125 | 0.00000 |
| Bolt systematic full authentication paths | 4982.52906 | 0.00000 |
| Bolt systematic opened row values | 81724.00000 | 81724.00000 |
| Canopy parity constant RS root | 0.00000 | 0.03125 |
| Canopy parity constant coefficient | 0.00000 | 0.03125 |
| Canopy systematic WHIR: OOD evaluations | 0.00000 | 0.18750 |
| Canopy systematic WHIR: authentication paths | 0.00000 | 149.39937 |
| Canopy systematic WHIR: initial commitment root | 0.00000 | 0.03125 |
| Canopy systematic WHIR: queried row values | 0.00000 | 133.00000 |
| Canopy systematic WHIR: recursive commitment roots | 0.00000 | 0.15625 |
| Canopy systematic WHIR: sumcheck messages | 0.00000 | 1.25000 |
| Canopy systematic WHIR: terminal coefficients | 0.00000 | 0.12500 |
| common message WHIR: authentication paths | 357.10281 | 357.10281 |
| common message WHIR: initial commitment root | 0.03125 | 0.03125 |
| common message WHIR: queried row values | 278.75000 | 278.75000 |
| common message WHIR: recursive commitment roots | 0.18750 | 0.18750 |
| common message WHIR: sumcheck messages | 1.43750 | 1.43750 |
| common message WHIR: terminal coefficients | 0.12500 | 0.12500 |
| common syndrome WHIR: authentication paths | 236.03250 | 236.03250 |
| common syndrome WHIR: initial commitment root | 0.03125 | 0.03125 |
| common syndrome WHIR: queried row values | 246.50000 | 246.50000 |
| common syndrome WHIR: recursive commitment roots | 0.15625 | 0.15625 |
| common syndrome WHIR: sumcheck messages | 1.25000 | 1.25000 |
| common syndrome WHIR: terminal coefficients | 0.12500 | 0.12500 |
| **Total hashes** | **5690.42813** | **2759.09844** |
| **Total field-value bytes** | **83492.21875** | **83626.81250** |
| **Total proof (MiB)** | **87.092429** | **84.361241** |

## Interpretation

Each opened Bolt row contains 128 field elements, or exactly 4 KiB. Canopy changes authentication, not these row values. The common message/syndrome WHIR proofs, g, and sigma also remain unchanged. At the full-height endpoint, a Canopy part retains the complete bottom tree and adds a 32-byte constant coefficient; its listed RS block is not transmitted. Nonconstant Canopy parts add an RS/SEP proof to replace the upper authentication paths. WHIR retains repeated queried row values but merges their Merkle paths; the per-oracle CSV makes that cost visible.
