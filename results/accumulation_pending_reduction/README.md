# Accumulation batching primitives — reduction choice pending

These are component measurements, not an end-to-end Canopy accumulation benchmark. In particular, every RS1 row excludes the old RS0 claim's reduction, its Canopy paths, and Canopy-opening SEP construction. No Canopy speedup is claimed.

Parameters: GF(2^256), BLAKE3-256, additive RS, scalar leaves and interleaving depth one, 136-bit local query budget (targeting 128-bit composition), eight Rayon workers, one warmup and five measured repetitions. N is the RS0 message length. RS0 has block length 2N; RS1 has message N/8 and block length N.

The local machine is the Apple M2 used by the repository's prior local runs. Measurements are sequential but the desktop was not isolated. Input generation, public FFT setup, input commitments, input-sigma generation, and verification are excluded from the step. Source commitments are regenerated before each trial and separately timed. The output commitment is included. These are native scalar RS measurements and cannot be substituted for the prior interleaved WHIR timings.

Every source uses a representative compact covector containing one MLE, one out-of-domain evaluation, and q domain evaluation terms. Covector materialization is included and separately reported; fresh inputs or sparse Canopy claims may have cheaper forms. Cost therefore depends on the eventual SEP functional representation.

The benchmark executes batched sumcheck to a common point, combines messages, encodes and commits the combination at the same rate and length, sends one OOD evaluation, and opens all source roots at common random indices. It replays sumcheck and Merkle verification and checks the outgoing witness's MLE and RS query constraints. It is not a complete accumulator API: outgoing SEP serialization, its scalar batching, decider, and adversarial security tests are not implemented. The small outgoing scalar computation is not timed; these remain cost estimates.

## Measured component costs

Medians in milliseconds; min–max is the range of the accounted total. Stage medians need not sum to the median total.

| N | Component | Covector materialization | Sumcheck | Output commitment | Accounted total | Range |
|---|---|---:|---:|---:|---:|---:|
| 2^18 | RS0 batch, 2 sources | 41.27 | 9.98 | 32.97 | 86.51 | 86.08–92.97 |
| 2^20 | RS0 batch, 2 sources | 168.72 | 37.46 | 145.47 | 361.35 | 355.47–381.60 |
| 2^22 | RS0 batch, 2 sources | 719.00 | 125.46 | 997.26 | 1919.12 | 1511.63–2025.26 |
| 2^18 | RS1 batch, 2 sources ONLY | 14.24 | 2.23 | 15.53 | 32.75 | 32.21–33.53 |
| 2^20 | RS1 batch, 2 sources ONLY | 60.54 | 5.82 | 59.12 | 127.20 | 125.11–136.30 |
| 2^22 | RS1 batch, 2 sources ONLY | 264.63 | 24.95 | 260.92 | 563.81 | 513.52–643.42 |
| 2^18 | RS1 batch, 3 sources ONLY | 22.02 | 3.33 | 16.66 | 42.77 | 40.45–52.60 |
| 2^20 | RS1 batch, 3 sources ONLY | 93.86 | 8.98 | 62.66 | 166.20 | 162.97–174.46 |
| 2^22 | RS1 batch, 3 sources ONLY | 384.96 | 39.48 | 263.53 | 693.25 | 671.21–762.53 |

## Expected verifier hash invocations

These count leaf hashes, distinct parent hashes in merged paths, and Fiat–Shamir XOF finalizations. They do not count BLAKE3 compression calls or hash bytes. Outgoing commitments are retained, so their trees are not opened in this step. Digest-to-field conversion costs zero hashes.

For q independent draws with replacement into M leaves, the exact expected count is H(M,q) = sum over levels j=0..log2(M) of (M/2^j) [1 − (1 − 2^j/M)^q]. With s source roots the authentication cost is s H(M,q). The modeled transcript adds log2(k)+4 finalizations. This transcript count is analytic, not instrumented. q=317 for RS0 and q=96 for RS1.

| N | RS0 batch | RS1 two-source batch ONLY | RS1 three-source batch ONLY |
|---|---:|---:|---:|
| 2^18 | 7,504 | 2,423 | 3,625 |
| 2^20 | 8,774 | 2,809 | 4,203 |
| 2^22 | 10,044 | 3,195 | 4,781 |
| 2^24 | 11,314 | 3,581 | 5,359 |
| 2^26 | 12,584 | 3,967 | 5,937 |
| 2^28 | 13,854 | 4,353 | 6,515 |
| 2^30 | 15,124 | 4,739 | 7,093 |

## Protocol decision needed

A Canopy opening reduces authentication of a query-answer relation to a committed RS1 SEP claim. It does not itself reduce SEP[RS0] to SEP[RS1]. For example, changing only the old SEP claim's sigma to a false value leaves all Canopy paths unchanged.

One option is to fully discharge the old RS0 SEP with an IOPP, using Canopy for its initial oracle and ordinary Merkle commitments for its internal oracles. Then only its Canopy-opening obligation remains; batch that with the existing RS1 accumulator (two sources). Another option is one three-variable WHIR fold: it leaves a folded-polynomial RS1 obligation plus a Canopy RS1 obligation, so the existing accumulator makes three sources. The folded output and Canopy code must use the same domain/encoding/layout to batch, not just matching lengths and rates.

Both choices can carry the new Canopy claim unchanged. The verifier must check that equality, and the final decider must check both accumulator components, including the last deferred claim. A formal extraction composition argument remains separate from this cost study.

Reproduce from the repository root:

```sh
cargo build --offline --release --example accumulation_primitives
RAYON_NUM_THREADS=8 target/release/examples/accumulation_primitives --logs 18,20,22 --output results/accumulation_pending_reduction/baseline_rate_half.csv
RAYON_NUM_THREADS=8 target/release/examples/accumulation_primitives --logs 15,17,19 --log-inverse-rate 3 --sources 2 --output results/accumulation_pending_reduction/layer_rate_eighth_two_sources.csv
RAYON_NUM_THREADS=8 target/release/examples/accumulation_primitives --logs 15,17,19 --log-inverse-rate 3 --sources 3 --output results/accumulation_pending_reduction/layer_rate_eighth_three_sources.csv
python3 scripts/report_accumulation_primitives.py
```
