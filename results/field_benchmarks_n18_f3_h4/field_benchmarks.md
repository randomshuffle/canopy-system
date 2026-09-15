# Large-field RS and H2F benchmark

WHIR interleaved rate-1/2 RS encoding: `2^18` inputs to `2^19` codeword elements, folding width `8`. H2F uses the exact Canopy BLAKE3-XOF-to-64-bytes plus modular-reduction method on `32768` roots (the Experiment-3 `h=4` workload). Times are release-build wall times after `6` warmup(s), with `24` measured repetitions; the harness rotates all six field orders. Output allocation is included; input generation and output deallocation are excluded.

| field | bits | 2-adicity | RS mean ms | RS median ms | RS speedup | H2F mean ms | H2F median ms | H2F ns/root | H2F speedup |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| current_256_bit | 256 | 32 | 24.261 | 24.092 | 1.000x | 7.802 | 7.741 | 238.1 | 1.000x |
| bn254_scalar | 254 | 28 | 23.072 | 22.733 | 1.052x | 9.405 | 9.285 | 287.0 | 0.830x |
| bls12_381_scalar | 255 | 32 | 23.242 | 23.138 | 1.044x | 9.119 | 9.105 | 278.3 | 0.856x |
