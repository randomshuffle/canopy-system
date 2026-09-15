# Bolt-max with Canopy-quarter

Prime field: `115792089237316195423570985008687907853269984665640564039457584007728446046209`. BLAKE3-256 commitments, H2F, and Fiat-Shamir challenges; 128-bit per-event target, no PoW.

Degree 8, compression 8, rate-1/2 syndrome RS, 4 field elements per row.
The LDPC distance 0.013 is an assumption for this prime-field port, not a proved consequence of the binary-field Bolt theorem.

Systematic/parity parts have separate commitments in both schemes. Their Canopy RS lengths are k/4 and k/16: exactly (k+k/4)/4. Heights count Merkle levels above whole rows.

Constrained RS uses unique-decoding WHIR; Canopy uses Johnson WHIR. Three-variable folds (smaller for tiny messages), native terminal, independent proofs. Public H is evaluated directly; Spark/Mulperm is excluded. Full executable protocol and limits: [experiment notes](../../../docs/experiment_bolt_max.md).

Queries before full-read saturation: X=20431, Y=309. External queries are sampled without replacement; when the budget exceeds the row count, read all rows. WHIR retains repeated row values from its native with-replacement queries; authentication paths are merged.

Sizes include both commitment roots, opened rows, g, sigma, both common WHIR proofs, and Canopy proofs. Estimates omit reference serialization framing; timings record actual serialized bytes. 100 deterministic samples per height.

| log2 input | h X / Y | Merkle MiB | Canopy MiB | ratio | Merkle auth KiB | Canopy auth KiB |
|---:|:---:|---:|---:|---:|---:|---:|
| 18 | 16 / 14 | 3.890 | 3.890 | 1.00002 | 843.81 | 843.87 |
| 22 | 3 / 18 | 6.459 | 5.416 | 0.83855 | 3185.32 | 2117.49 |
| 26 | 3 / 22 | 9.409 | 6.091 | 0.64731 | 5741.47 | 2343.20 |
| 30 | 3 / 26 | 12.361 | 6.635 | 0.53682 | 8330.70 | 2468.11 |

Upstream inspected: https://github.com/bcc-research/bolt-rs/tree/3832e47b24e7b3e10525c9c5bcfc1cfe66d525f2 ; paper: https://eprint.iacr.org/2026/310 (2026-05-20 revision).
