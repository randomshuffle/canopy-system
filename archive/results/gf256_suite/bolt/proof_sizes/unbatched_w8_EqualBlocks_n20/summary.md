# Bolt-max with Canopy

Field: `GF(2^256) = GF(2^128)[w]/(w^2+w+x^121)`. BLAKE3-256 commitments, H2F, and Fiat-Shamir challenges; 128-bit composed target (132-bit external tests; 136-bit WHIR), no PoW.

Degree 8, compression 8, rate-1/2 syndrome RS, 8 field elements per row.
The LDPC ensemble distance gamma=0.013 remains an explicit assumption for these measurements.

Systematic/parity parts have separate commitments in both schemes. Canopy RS block lengths: systematic k/4 and parity k/4; total k/2, 40% of Bolt's encoded rows. Heights start at 3 for systematic and 1 for parity. Heights count Merkle levels above whole rows.

Constrained RS uses unique-decoding WHIR; Canopy uses Johnson WHIR. Three-variable folds (smaller for tiny messages), native terminal, independent proofs. Public H is evaluated directly; Spark/Mulperm is excluded. Full executable protocol and limits: [experiment notes](../../../../../../docs/experiment_bolt_max.md).

Queries before full-read saturation: X=21069, Y=319. External queries are sampled without replacement; when the budget exceeds the row count, read all rows. WHIR retains repeated row values from its native with-replacement queries; authentication paths are merged.

Sizes include both commitment roots, opened rows, g, sigma, both common WHIR proofs, and Canopy proofs. Estimates omit reference serialization framing; timings record actual serialized bytes. 100 deterministic samples per height.

| log2 input | h X / Y | Merkle MiB | Canopy MiB | ratio | Merkle auth KiB | Canopy auth KiB |
|---:|:---:|---:|---:|---:|---:|---:|
| 20 | 17 / 15 | 7.203 | 7.203 | 1.00001 | 1403.45 | 1403.51 |

Upstream inspected: https://github.com/bcc-research/bolt-rs/tree/3832e47b24e7b3e10525c9c5bcfc1cfe66d525f2 ; paper: https://eprint.iacr.org/2026/310 (2026-05-20 revision).
