# Table 1 timing provenance audit

Table 1 uses the retained split-half suite. The newer response used fresh timings and a different Merkle batching architecture. Canopy itself retained both k/2 blocks, heights (2,3), w=4, eta=1, N=2^22, the same setup seed, and identical serialized proof size (4,877,048 bytes). All runs use eight threads, one warmup, five measured repetitions, and the same commitment-plus-opening scope with FS; input generation, setup, and verification are excluded.

| Dataset | Merkle architecture | Merkle median (s) | Canopy median (s) | Canopy overhead |
|---|---|---:|---:|---:|
| paper_table_1 | bolt_merkle_shared | 0.568 | 0.732 | 28.8% |
| latest_response | bolt_merkle_split | 0.621 | 0.682 | 9.8% |
| frozen_paper_binary | bolt_merkle_shared | 0.568 | 0.742 | 30.7% |
| current_binary_original_parameters | bolt_merkle_shared | 0.587 | 0.746 | 27.1% |

The paper uses a shared Merkle tail, with proof size 6,455.01 KiB. The later search admits immediate joins and selects separate Merkle tails because their proof is slightly smaller (6,446.50 KiB, a 0.13% reduction). That configuration is slower in the reported measurements. The Canopy modeled proof size is unchanged at 4,763.28 KiB.

The previously reported 9.8% total-time overhead combines a different Merkle baseline with a faster Canopy measurement from a separate campaign. It is not evidence of an algorithmic improvement over Table 1. Likewise, the earlier statement that opening time was essentially unchanged compared Canopy with the split-tail Merkle baseline, not the paper shared-tail baseline.

The frozen executable used for Table 1 was retained and its SHA-256 matches the original process record exactly. Replaying it with the original parameters reproduces approximately the paper timing. Replaying the current executable with those same parameters also gives a similar Canopy time. The audit does not identify a protocol speedup; the saved records do not establish which runtime/environment factor produced the earlier faster measurement.

Both audited runs verified all 12 warmup/measured proofs. `comparison.csv` retains phase means, median totals, ranges, and serialized sizes. `processes.json` records binary identities and original commands. Table 1 and its source measurements are unchanged. A paper-level update should use a consistent baseline policy and one matched timing campaign rather than replace individual values from different runs.
