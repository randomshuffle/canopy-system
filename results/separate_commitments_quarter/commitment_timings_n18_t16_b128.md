# Separate-commitment prover timings

`N = 2^18`, `t = 16`, and `B = 128` per commitment. Canopy uses an `N/4` codeword, and the batch selects its proof-size-optimal height. Commitments are executed sequentially; their internal Rayon parallelism is unchanged. Witness generation, verifier-challenge generation, and deallocation are excluded. Canopy includes construction and commitment of the 5 challenge-dependent linear combinations, but not WHIR execution.

| scheme | codeword | h | combinations | repetitions | mean s | median s | min s | max s | vs Merkle mean |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| merkle_separate | - | - | 0 | 20 | 0.2308 | 0.2246 | 0.2208 | 0.2864 | 1.000x |
| canopy_quarter_batched_whir | N/4 | 5 | 5 | 20 | 0.2635 | 0.2610 | 0.2581 | 0.2870 | 1.141x |

## Mean stage profile

| scheme | codeword | h | stage | work | mean s | median s | share |
|---|---:|---:|---|---:|---:|---:|---:|
| merkle_separate | - | - | separate_merkle_trees | 8388592 leaf/internal hashes | 0.2308 | 0.2246 | 100.0% |
| canopy_quarter_batched_whir | N/4 | 5 | separate_bottom_forests | 8257536 leaf/internal hashes | 0.2138 | 0.2121 | 81.2% |
| canopy_quarter_batched_whir | N/4 | 5 | hash_to_field | 131072 forest roots | 0.0090 | 0.0089 | 3.4% |
| canopy_quarter_batched_whir | N/4 | 5 | reed_solomon_encoding | 1048576 codeword elements | 0.0201 | 0.0200 | 7.6% |
| canopy_quarter_batched_whir | N/4 | 5 | old_codeword_merkle_trees | 131056 leaf/internal hashes | 0.0160 | 0.0159 | 6.1% |
| canopy_quarter_batched_whir | N/4 | 5 | linear_combinations | 5242880 field multiply-adds | 0.0022 | 0.0022 | 0.8% |
| canopy_quarter_batched_whir | N/4 | 5 | combined_oracle_merkle_tree | 8191 leaf/internal hashes | 0.0024 | 0.0024 | 0.9% |
| canopy_quarter_batched_whir | N/4 | 5 | other | - | 0.0000 | 0.0000 | 0.0% |
