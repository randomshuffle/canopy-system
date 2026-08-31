# Separate-commitment Canopy vs. Merkle proof sizes

Each of the `t` vectors has a separate commitment and separate Merkle path. All commitments open the same `B` logical indices and use the same SEP-reduction challenge so their functionals align. Canopy replaces `t` independent WHIR executions with independently randomized linear combinations, interleaves the resulting codewords in one commitment, checks that commitment against all old codeword commitments, and runs parallel WHIR instances with shared oracle paths.

The number of linear combinations is `ceil(128 / log2(|F|))`; this is five for the 32-bit field and provides 157.92 bits against cancellation. Consistency uses the RS minimum distance `1-2^-h` and `ceil(128/h)` shared leaf queries. WHIR retains the existing capacity-bound estimator. Combined-oracle values at consistency positions are verifier-derived, and the combined-oracle authentication frontier is merged with its first WHIR opening.

Mean of 100 deterministic samples; seed `0x5345504152415445`. WHIR targets 128 bits, terminates at at most `2^8` variables, and folds `2^4` values per column.

| log2 N | t | B each | total opened | Merkle KiB | combinations | best h | consistency q | Canopy KiB | hashes KiB | fields KiB | Canopy/Merkle |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 18 | 16 | 128 | 2048 | 654.94 | 5 | 3 | 43 | 466.44 | 383.62 | 82.82 | 0.712 |
