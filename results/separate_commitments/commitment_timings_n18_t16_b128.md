# Separate-commitment prover timings

`N = 2^18`, `t = 16`, and `B = 128` per commitment. The batch selects the proof-size-optimal Canopy height. Commitments are executed sequentially; their internal Rayon parallelism is unchanged. Witness generation, verifier-challenge generation, and deallocation are excluded. Canopy includes construction and commitment of the 5 challenge-dependent linear combinations, but not WHIR execution.

| scheme | h | combinations | repetitions | mean s | median s | min s | max s | vs Merkle mean |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| merkle_separate | - | 0 | 20 | 0.2333 | 0.2234 | 0.2205 | 0.3000 | 1.000x |
| canopy_batched_whir | 3 | 5 | 20 | 0.3415 | 0.3376 | 0.3355 | 0.3923 | 1.464x |

## Mean stage profile

| scheme | h | stage | work | mean s | median s | share |
|---|---:|---|---:|---:|---:|---:|
| merkle_separate | - | separate_merkle_trees | 8388592 leaf/internal hashes | 0.2333 | 0.2234 | 100.0% |
| canopy_batched_whir | 3 | separate_bottom_forests | 7864320 leaf/internal hashes | 0.2005 | 0.1982 | 58.7% |
| canopy_batched_whir | 3 | hash_to_field | 524288 forest roots | 0.0327 | 0.0321 | 9.6% |
| canopy_batched_whir | 3 | reed_solomon_encoding | 4194304 codeword elements | 0.0470 | 0.0471 | 13.8% |
| canopy_batched_whir | 3 | old_codeword_merkle_trees | 524272 leaf/internal hashes | 0.0447 | 0.0442 | 13.1% |
| canopy_batched_whir | 3 | linear_combinations | 20971520 field multiply-adds | 0.0082 | 0.0082 | 2.4% |
| canopy_batched_whir | 3 | combined_oracle_merkle_tree | 32767 leaf/internal hashes | 0.0084 | 0.0084 | 2.5% |
| canopy_batched_whir | 3 | other | - | 0.0000 | 0.0000 | 0.0% |
