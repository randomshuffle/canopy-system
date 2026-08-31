# Prover commitment timings

Input-vector generation and deallocation are excluded. Canopy heights are the batch-128 proof-size optima for their respective codeword lengths.

| scheme | codeword | h | repetitions | mean s | median s | min s | max s |
|---|---|---:|---:|---:|---:|---:|---:|
| merkle | - | - | 20 | 0.2221 | 0.2169 | 0.2100 | 0.2905 |
| canopy_original | N/1 | 6 | 20 | 0.3168 | 0.3101 | 0.3060 | 0.3684 |
| canopy_half | N/2 | 6 | 20 | 0.2646 | 0.2640 | 0.2622 | 0.2707 |
| canopy_quarter | N/4 | 7 | 20 | 0.2391 | 0.2359 | 0.2344 | 0.2838 |

## Mean stage breakdown

Stage percentages use mean stage time divided by mean total time, so they add to 100% within rounding.

| scheme | codeword | h | stage | work | mean s | median s | share |
|---|---|---:|---|---:|---:|---:|---:|
| merkle | - | - | full_merkle_tree | 8388607 hashes | 0.2221 | 0.2169 | 100.0% |
| canopy_original | N/1 | 6 | bottom_forest | 8323072 hashes | 0.2132 | 0.2072 | 67.3% |
| canopy_original | N/1 | 6 | hash_to_field | 65536 roots | 0.0230 | 0.0226 | 7.3% |
| canopy_original | N/1 | 6 | reed_solomon_encoding | 4194304 codeword elements | 0.0436 | 0.0433 | 13.8% |
| canopy_original | N/1 | 6 | codeword_merkle_tree | 524287 hashes | 0.0370 | 0.0364 | 11.7% |
| canopy_original | N/1 | 6 | other | - | 0.0000 | 0.0000 | 0.0% |
| canopy_half | N/2 | 6 | bottom_forest | 8323072 hashes | 0.2051 | 0.2042 | 77.5% |
| canopy_half | N/2 | 6 | hash_to_field | 65536 roots | 0.0225 | 0.0225 | 8.5% |
| canopy_half | N/2 | 6 | reed_solomon_encoding | 2097152 codeword elements | 0.0187 | 0.0187 | 7.1% |
| canopy_half | N/2 | 6 | codeword_merkle_tree | 262143 hashes | 0.0183 | 0.0183 | 6.9% |
| canopy_half | N/2 | 6 | other | - | 0.0000 | 0.0000 | 0.0% |
| canopy_quarter | N/4 | 7 | bottom_forest | 8355840 hashes | 0.2083 | 0.2057 | 87.1% |
| canopy_quarter | N/4 | 7 | hash_to_field | 32768 roots | 0.0114 | 0.0112 | 4.8% |
| canopy_quarter | N/4 | 7 | reed_solomon_encoding | 1048576 codeword elements | 0.0096 | 0.0096 | 4.0% |
| canopy_quarter | N/4 | 7 | codeword_merkle_tree | 131071 hashes | 0.0098 | 0.0095 | 4.1% |
| canopy_quarter | N/4 | 7 | other | - | 0.0000 | 0.0000 | 0.0% |
