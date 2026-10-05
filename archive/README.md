# Historical experiments

The GF(2^256) transition happened on **September 15–16, 2026**.
Commit `3603dd86c5f5173e76a470e5a5fa7041dcc17717` (September 16 at
11:13 a.m. Eastern) first records GF(2^256) as the main WHIR, Canopy, and Bolt
default. Its predecessor, `71865e1`, still uses a 256-bit prime field.
Saved GF256 process receipts show runs beginning September 15 at 1:33 p.m.
Eastern. The later direct-digest conversion change is a separate transition.

Cleanup follows the recorded field and provenance. The 45 confirmed
prime/small-field result directories and 14 loose files were removed from
the working tree; their original content is recoverable from Git revision
`71865e1`. The [cleanup manifest](cleanup_manifest.json) lists those paths and
relocations. The earlier mixed prime/binary pilot is retained to document the
field comparison.

Final artifacts are in [results](../results/README.md). These archived studies
retain their original numerical data, process logs, receipts, source hashes,
and source bundles. Their geometry, batching, hash accounting, hardware, or
measurement design differ from the final suite. Each study's `EXPERIMENT.md`
briefly explains its purpose. Paths inside immutable receipts/bundles refer to
the original layout; the relocation manifest maps them to this layout.

| Retained study |
|---|
| [accumulation_aws_20260917](results/accumulation_aws_20260917/EXPERIMENT.md) |
| [accumulation_one_fold](results/accumulation_one_fold/EXPERIMENT.md) |
| [accumulation_pending_reduction](results/accumulation_pending_reduction/EXPERIMENT.md) |
| [aws_i4i](results/aws_i4i/EXPERIMENT.md) |
| [aws_i4i_fair](results/aws_i4i_fair/EXPERIMENT.md) |
| [aws_i4i_n30_eta1_prepared](results/aws_i4i_n30_eta1_prepared/EXPERIMENT.md) |
| [aws_r5_prepared](results/aws_r5_prepared/EXPERIMENT.md) |
| [bolt_delayed_systematic_eta1](results/bolt_delayed_systematic_eta1/EXPERIMENT.md) |
| [bolt_half_quarter_shared_baseline_eta1](results/bolt_half_quarter_shared_baseline_eta1/EXPERIMENT.md) |
| [bolt_one_fold_systematic_eta1](results/bolt_one_fold_systematic_eta1/EXPERIMENT.md) |
| [bolt_width_study_eta1](results/bolt_width_study_eta1/EXPERIMENT.md) |
| [gf256_batched_suite](results/gf256_batched_suite/EXPERIMENT.md) |
| [gf256_split_half_suite](results/gf256_split_half_suite/EXPERIMENT.md) |
| [gf256_split_suite](results/gf256_split_suite/EXPERIMENT.md) |
| [gf256_split_w16_suite](results/gf256_split_w16_suite/EXPERIMENT.md) |
| [gf256_suite](results/gf256_suite/EXPERIMENT.md) |
| [n30_prover_time_estimates](results/n30_prover_time_estimates/EXPERIMENT.md) |
| [parallel_prover_n22_eta1](results/parallel_prover_n22_eta1/EXPERIMENT.md) |
| [table1_timing_audit](results/table1_timing_audit/EXPERIMENT.md) |
| [whir_field_comparison_n22](results/whir_field_comparison_n22/EXPERIMENT.md) |
| [whir_half_block_eta1](results/whir_half_block_eta1/EXPERIMENT.md) |

Historical runners are in [scripts](scripts/README.md), the isolated
GF(2^256) pilot is in [experiments/binary-whir](experiments/binary-whir/EXPERIMENT.md),
and earlier protocol notes are in `docs/`. The [old AWS launchers](launchers/README.md)
expect the original host layout. Use the current final reproduction tools for
new runs. Historical commands may write into their stored study directories;
copy the study before rerunning an old script. Later changes to shared Rust
sources mean old source hashes do not describe a current-source rerun.
