# Measured timings at 2^22

Open and verify values are means of 30 repetitions. Merkle rows are end-to-end. Canopy commitment is complete, but Canopy open/verify rows measure only the bottom-tree opening and committed-SEP reduction; WHIR execution is excluded.

Canopy proof estimates use 128-bit WHIR security and terminate recursive folding once message length is at most `2^8`; all folds use `2^4`.

Original-vector and WHIR field elements use a 32-bit field and occupy 4 bytes.

No proof of work is used. WHIR execution is excluded; proof sizes include the field messages from independently repeated sumcheck challenges.

| scheme | h | batch | commit s | prover/open ms | verifier ms | estimated proof KiB |
|---|---:|---:|---:|---:|---:|---:|
| merkle | - | 64 | 0.264 | 0.022 | 0.243 | 30.22 |
| merkle | - | 128 | 0.264 | 0.048 | 0.494 | 56.59 |
| merkle | - | 256 | 0.264 | 0.124 | 1.010 | 106.50 |
| merkle | - | 512 | 0.264 | 0.276 | 1.995 | 195.50 |
| merkle | - | 1024 | 0.264 | 0.587 | 3.947 | 358.47 |
| canopy_frontend | 4 | 64 | 0.415 | 0.006 | 0.092 | 41.36 |
| canopy_frontend | 4 | 128 | 0.415 | 0.012 | 0.190 | 49.61 |
| canopy_frontend | 4 | 256 | 0.415 | 0.031 | 0.402 | 66.11 |
| canopy_frontend | 4 | 512 | 0.415 | 0.080 | 0.845 | 98.98 |
| canopy_frontend | 4 | 1024 | 0.415 | 0.183 | 1.763 | 164.92 |
| canopy_frontend | 5 | 64 | 0.345 | 0.007 | 0.103 | 37.52 |
| canopy_frontend | 5 | 128 | 0.345 | 0.015 | 0.211 | 47.77 |
| canopy_frontend | 5 | 256 | 0.345 | 0.038 | 0.443 | 68.27 |
| canopy_frontend | 5 | 512 | 0.345 | 0.099 | 0.938 | 109.14 |
| canopy_frontend | 5 | 1024 | 0.345 | 0.212 | 1.973 | 191.02 |
| canopy_frontend | 6 | 64 | 0.326 | 0.007 | 0.114 | 32.18 |
| canopy_frontend | 6 | 128 | 0.326 | 0.017 | 0.237 | 44.43 |
| canopy_frontend | 6 | 256 | 0.326 | 0.042 | 0.511 | 68.87 |
| canopy_frontend | 6 | 512 | 0.326 | 0.117 | 1.076 | 117.87 |
| canopy_frontend | 6 | 1024 | 0.326 | 0.261 | 2.285 | 215.62 |
| canopy_frontend | 7 | 64 | 0.327 | 0.010 | 0.135 | 31.25 |
| canopy_frontend | 7 | 128 | 0.327 | 0.020 | 0.275 | 45.50 |
| canopy_frontend | 7 | 256 | 0.327 | 0.058 | 0.584 | 74.00 |
| canopy_frontend | 7 | 512 | 0.327 | 0.144 | 1.263 | 130.65 |
| canopy_frontend | 7 | 1024 | 0.327 | 0.299 | 2.614 | 243.46 |
