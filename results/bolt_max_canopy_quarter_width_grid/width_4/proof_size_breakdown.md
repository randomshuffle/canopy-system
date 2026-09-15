# Bolt-max versus Canopy: proof-size breakdown

These are the saved 256-bit-prime/BLAKE3 experiment parameters, not the published binary-field Bolt benchmark. Each row is a mean over the original sample seeds. Sizes include initial commitment roots; exclude serializer framing and sparse-PCS preprocessing. The per-event 128-bit target is conditional on LDPC distance 0.013. Both schemes use the same common constrained-WHIR proofs.

1 MiB = 1,048,576 bytes; 1 KiB = 1,024 bytes. Every component below reconciles to proof_sizes.csv.

## Input 2^18: width=4, h_X=16, h_Y=14

| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |
|---|---:|---:|
| Bolt parity bottom authentication paths | 0.00000 | 47.29281 |
| Bolt parity commitment root | 0.03125 | 0.00000 |
| Bolt parity full authentication paths | 47.29281 | 0.00000 |
| Bolt parity opened row values | 38.62500 | 38.62500 |
| Bolt partial evaluation g | 0.12500 | 0.12500 |
| Bolt syndrome evaluation sigma | 0.03125 | 0.03125 |
| Bolt systematic bottom authentication paths | 0.00000 | 796.45531 |
| Bolt systematic commitment root | 0.03125 | 0.00000 |
| Bolt systematic full authentication paths | 796.45531 | 0.00000 |
| Bolt systematic opened row values | 2553.87500 | 2553.87500 |
| Canopy parity constant RS root | 0.00000 | 0.03125 |
| Canopy parity constant coefficient | 0.00000 | 0.03125 |
| Canopy systematic constant RS root | 0.00000 | 0.03125 |
| Canopy systematic constant coefficient | 0.00000 | 0.03125 |
| common message WHIR: authentication paths | 109.73500 | 109.73500 |
| common message WHIR: initial commitment root | 0.03125 | 0.03125 |
| common message WHIR: queried row values | 214.25000 | 214.25000 |
| common message WHIR: recursive commitment roots | 0.12500 | 0.12500 |
| common message WHIR: sumcheck messages | 1.00000 | 1.00000 |
| common message WHIR: terminal coefficients | 0.06250 | 0.06250 |
| common syndrome WHIR: authentication paths | 38.68219 | 38.68219 |
| common syndrome WHIR: initial commitment root | 0.03125 | 0.03125 |
| common syndrome WHIR: queried row values | 182.00000 | 182.00000 |
| common syndrome WHIR: recursive commitment roots | 0.09375 | 0.09375 |
| common syndrome WHIR: sumcheck messages | 0.81250 | 0.81250 |
| common syndrome WHIR: terminal coefficients | 0.06250 | 0.06250 |
| **Total hashes** | **992.50906** | **992.50906** |
| **Total field-value bytes** | **2990.84375** | **2990.90625** |
| **Total proof (MiB)** | **3.889993** | **3.890054** |

## Input 2^22: width=4, h_X=3, h_Y=18

| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |
|---|---:|---:|
| Bolt parity bottom authentication paths | 0.00000 | 85.45406 |
| Bolt parity commitment root | 0.03125 | 0.00000 |
| Bolt parity full authentication paths | 85.45406 | 0.00000 |
| Bolt parity opened row values | 38.62500 | 38.62500 |
| Bolt partial evaluation g | 0.12500 | 0.12500 |
| Bolt syndrome evaluation sigma | 0.03125 | 0.03125 |
| Bolt systematic bottom authentication paths | 0.00000 | 1807.25375 |
| Bolt systematic commitment root | 0.03125 | 0.00000 |
| Bolt systematic full authentication paths | 3099.80219 | 0.00000 |
| Bolt systematic opened row values | 2553.87500 | 2553.87500 |
| Canopy parity constant RS root | 0.00000 | 0.03125 |
| Canopy parity constant coefficient | 0.00000 | 0.03125 |
| Canopy systematic WHIR: OOD evaluations | 0.00000 | 0.15625 |
| Canopy systematic WHIR: authentication paths | 0.00000 | 96.21813 |
| Canopy systematic WHIR: initial commitment root | 0.00000 | 0.03125 |
| Canopy systematic WHIR: queried row values | 0.00000 | 127.00000 |
| Canopy systematic WHIR: recursive commitment roots | 0.00000 | 0.12500 |
| Canopy systematic WHIR: sumcheck messages | 0.00000 | 1.06250 |
| Canopy systematic WHIR: terminal coefficients | 0.00000 | 0.12500 |
| common message WHIR: authentication paths | 236.27219 | 236.27219 |
| common message WHIR: initial commitment root | 0.03125 | 0.03125 |
| common message WHIR: queried row values | 246.50000 | 246.50000 |
| common message WHIR: recursive commitment roots | 0.15625 | 0.15625 |
| common message WHIR: sumcheck messages | 1.25000 | 1.25000 |
| common message WHIR: terminal coefficients | 0.12500 | 0.12500 |
| common syndrome WHIR: authentication paths | 135.91406 | 135.91406 |
| common syndrome WHIR: initial commitment root | 0.03125 | 0.03125 |
| common syndrome WHIR: queried row values | 214.25000 | 214.25000 |
| common syndrome WHIR: recursive commitment roots | 0.12500 | 0.12500 |
| common syndrome WHIR: sumcheck messages | 1.06250 | 1.06250 |
| common syndrome WHIR: terminal coefficients | 0.12500 | 0.12500 |
| **Total hashes** | **3557.84875** | **2361.64344** |
| **Total field-value bytes** | **3055.96875** | **3184.34375** |
| **Total proof (MiB)** | **6.458806** | **5.416003** |

## Input 2^26: width=4, h_X=3, h_Y=22

| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |
|---|---:|---:|
| Bolt parity bottom authentication paths | 0.00000 | 124.00562 |
| Bolt parity commitment root | 0.03125 | 0.00000 |
| Bolt parity full authentication paths | 124.00562 | 0.00000 |
| Bolt parity opened row values | 38.62500 | 38.62500 |
| Bolt partial evaluation g | 0.12500 | 0.12500 |
| Bolt syndrome evaluation sigma | 0.03125 | 0.03125 |
| Bolt systematic bottom authentication paths | 0.00000 | 1908.48531 |
| Bolt systematic commitment root | 0.03125 | 0.00000 |
| Bolt systematic full authentication paths | 5617.40219 | 0.00000 |
| Bolt systematic opened row values | 2553.87500 | 2553.87500 |
| Canopy parity constant RS root | 0.00000 | 0.03125 |
| Canopy parity constant coefficient | 0.00000 | 0.03125 |
| Canopy systematic WHIR: OOD evaluations | 0.00000 | 0.21875 |
| Canopy systematic WHIR: authentication paths | 0.00000 | 170.86875 |
| Canopy systematic WHIR: initial commitment root | 0.00000 | 0.03125 |
| Canopy systematic WHIR: queried row values | 0.00000 | 138.00000 |
| Canopy systematic WHIR: recursive commitment roots | 0.00000 | 0.18750 |
| Canopy systematic WHIR: sumcheck messages | 0.00000 | 1.31250 |
| Canopy systematic WHIR: terminal coefficients | 0.00000 | 0.03125 |
| common message WHIR: authentication paths | 420.52344 | 420.52344 |
| common message WHIR: initial commitment root | 0.03125 | 0.03125 |
| common message WHIR: queried row values | 311.00000 | 311.00000 |
| common message WHIR: recursive commitment roots | 0.21875 | 0.21875 |
| common message WHIR: sumcheck messages | 1.50000 | 1.50000 |
| common message WHIR: terminal coefficients | 0.03125 | 0.03125 |
| common syndrome WHIR: authentication paths | 287.40250 | 287.40250 |
| common syndrome WHIR: initial commitment root | 0.03125 | 0.03125 |
| common syndrome WHIR: queried row values | 278.75000 | 278.75000 |
| common syndrome WHIR: recursive commitment roots | 0.18750 | 0.18750 |
| common syndrome WHIR: sumcheck messages | 1.31250 | 1.31250 |
| common syndrome WHIR: terminal coefficients | 0.03125 | 0.03125 |
| **Total hashes** | **6449.86500** | **2912.00437** |
| **Total field-value bytes** | **3185.28125** | **3324.87500** |
| **Total proof (MiB)** | **9.409323** | **6.090703** |

## Input 2^30: width=4, h_X=3, h_Y=26

| Component | Bolt-max + Merkle (KiB) | Bolt-max + Canopy (KiB) |
|---|---:|---:|
| Bolt parity bottom authentication paths | 0.00000 | 162.51844 |
| Bolt parity commitment root | 0.03125 | 0.00000 |
| Bolt parity full authentication paths | 162.51844 | 0.00000 |
| Bolt parity opened row values | 38.62500 | 38.62500 |
| Bolt partial evaluation g | 0.12500 | 0.12500 |
| Bolt syndrome evaluation sigma | 0.03125 | 0.03125 |
| Bolt systematic bottom authentication paths | 0.00000 | 1914.93938 |
| Bolt systematic commitment root | 0.03125 | 0.00000 |
| Bolt systematic full authentication paths | 8168.12219 | 0.00000 |
| Bolt systematic opened row values | 2553.87500 | 2553.87500 |
| Canopy parity constant RS root | 0.00000 | 0.03125 |
| Canopy parity constant coefficient | 0.00000 | 0.03125 |
| Canopy systematic WHIR: OOD evaluations | 0.00000 | 0.25000 |
| Canopy systematic WHIR: authentication paths | 0.00000 | 245.96812 |
| Canopy systematic WHIR: initial commitment root | 0.00000 | 0.03125 |
| Canopy systematic WHIR: queried row values | 0.00000 | 142.50000 |
| Canopy systematic WHIR: recursive commitment roots | 0.00000 | 0.21875 |
| Canopy systematic WHIR: sumcheck messages | 0.00000 | 1.56250 |
| Canopy systematic WHIR: terminal coefficients | 0.00000 | 0.06250 |
| common message WHIR: authentication paths | 616.44687 | 616.44687 |
| common message WHIR: initial commitment root | 0.03125 | 0.03125 |
| common message WHIR: queried row values | 343.25000 | 343.25000 |
| common message WHIR: recursive commitment roots | 0.25000 | 0.25000 |
| common message WHIR: sumcheck messages | 1.75000 | 1.75000 |
| common message WHIR: terminal coefficients | 0.06250 | 0.06250 |
| common syndrome WHIR: authentication paths | 459.24812 | 459.24812 |
| common syndrome WHIR: initial commitment root | 0.03125 | 0.03125 |
| common syndrome WHIR: queried row values | 311.00000 | 311.00000 |
| common syndrome WHIR: recursive commitment roots | 0.21875 | 0.21875 |
| common syndrome WHIR: sumcheck messages | 1.56250 | 1.56250 |
| common syndrome WHIR: terminal coefficients | 0.06250 | 0.06250 |
| **Total hashes** | **9406.92938** | **3399.93344** |
| **Total field-value bytes** | **3250.34375** | **3394.75000** |
| **Total proof (MiB)** | **12.360618** | **6.635433** |

## Interpretation

Each opened Bolt row contains the selected interleaving width in field elements, each 32 bytes. Canopy changes authentication, not these row values. The common message/syndrome WHIR proofs, g, and sigma also remain unchanged. At the full-height endpoint, a Canopy part retains the complete bottom tree and adds a 32-byte constant coefficient; its listed RS block is not transmitted. Nonconstant Canopy parts add an RS/SEP proof to replace the upper authentication paths. WHIR retains repeated queried row values but merges their Merkle paths; the per-oracle CSV makes that cost visible.
