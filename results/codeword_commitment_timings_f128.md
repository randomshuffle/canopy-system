# Prover commitment timings

Input-vector generation and deallocation are excluded. Canopy heights are the batch-128 proof-size optima for their respective codeword lengths.

| scheme | codeword | h | repetitions | mean s | median s | min s | max s |
|---|---|---:|---:|---:|---:|---:|---:|
| merkle | - | - | 20 | 0.2166 | 0.2113 | 0.2051 | 0.2517 |
| canopy_original | N/1 | 7 | 20 | 0.3862 | 0.3838 | 0.3782 | 0.4117 |
| canopy_half | N/2 | 7 | 20 | 0.2968 | 0.2914 | 0.2874 | 0.3333 |
| canopy_quarter | N/4 | 8 | 20 | 0.2522 | 0.2491 | 0.2460 | 0.2655 |

## Mean stage breakdown

Stage percentages use mean stage time divided by mean total time, so they add to 100% within rounding.

| scheme | codeword | h | stage | work | mean s | median s | share |
|---|---|---:|---|---:|---:|---:|---:|
| merkle | - | - | full_merkle_tree | 8388607 hashes | 0.2166 | 0.2113 | 100.0% |
| canopy_original | N/1 | 7 | bottom_forest | 8355840 hashes | 0.2080 | 0.2063 | 53.9% |
| canopy_original | N/1 | 7 | hash_to_field | 32768 roots | 0.0049 | 0.0048 | 1.3% |
| canopy_original | N/1 | 7 | reed_solomon_encoding | 4194304 codeword elements | 0.1267 | 0.1256 | 32.8% |
| canopy_original | N/1 | 7 | codeword_merkle_tree | 524287 hashes | 0.0465 | 0.0461 | 12.0% |
| canopy_original | N/1 | 7 | other | - | 0.0000 | 0.0000 | 0.0% |
| canopy_half | N/2 | 7 | bottom_forest | 8355840 hashes | 0.2073 | 0.2040 | 69.8% |
| canopy_half | N/2 | 7 | hash_to_field | 32768 roots | 0.0048 | 0.0047 | 1.6% |
| canopy_half | N/2 | 7 | reed_solomon_encoding | 2097152 codeword elements | 0.0608 | 0.0587 | 20.5% |
| canopy_half | N/2 | 7 | codeword_merkle_tree | 262143 hashes | 0.0239 | 0.0237 | 8.1% |
| canopy_half | N/2 | 7 | other | - | 0.0000 | 0.0000 | 0.0% |
| canopy_quarter | N/4 | 8 | bottom_forest | 8372224 hashes | 0.2078 | 0.2054 | 82.4% |
| canopy_quarter | N/4 | 8 | hash_to_field | 16384 roots | 0.0025 | 0.0024 | 1.0% |
| canopy_quarter | N/4 | 8 | reed_solomon_encoding | 1048576 codeword elements | 0.0297 | 0.0288 | 11.8% |
| canopy_quarter | N/4 | 8 | codeword_merkle_tree | 131071 hashes | 0.0123 | 0.0121 | 4.9% |
| canopy_quarter | N/4 | 8 | other | - | 0.0000 | 0.0000 | 0.0% |
