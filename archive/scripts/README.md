# Historical experiment scripts

These helpers belong to the studies indexed in [the archive](../README.md).
GF256 script paths now resolve the relocated result directories. Some earlier
prime-field scripts remain as source records; their pre-switch outputs were
removed and they require the matching historical Git revision to reproduce.
The isolated prime/binary comparison also requires its original source revision
for the prime control: the current main crate uses GF(2^256).

Run historical tools from the repository root and copy their data directories
before any regeneration; many old scripts have fixed output paths. Shared
metadata and measurement helpers remain in `scripts/` at the repository root.
The [final reproduction guide](../../README.md#reproduce-the-final-suite) uses
fresh output directories and current runners.
