# AWS one-fold accumulation comparison

Measured one accumulation step for plain Merkle at rates 1/2 and 1/4 and Canopy with one native WHIR fold, on Intel i4i hardware with 32 pinned workers. The GF(2^256) experiment includes proof verification, phase timings, verifier hashes, and a separate incoming-commitment cost.

Original location: `results/accumulation_aws_20260917/`. An accumulation workload, separate from the final WHIR/Bolt paper comparison.

The retained logs, manifests, commands, and source bundles describe the original run; historical paths and source hashes have not been rewritten. See [final results and reproduction guidance](../../../results/README.md) for the current experiments.
