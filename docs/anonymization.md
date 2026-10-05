# Public source distribution

The published files use an anonymous first-party copyright notice, relative
checkout paths, and placeholder benchmark hostnames. Saved source and measurement
archives have also been sanitized, including tar ownership, timestamps, extended
attributes, and gzip headers. Compiled executables and Python bytecode are excluded
because they can contain original filesystem paths.

The benchmark data and numerical measurements are retained. Historical source and
binary hashes describe the original runs; edits for anonymity do not constitute a
new run or validation of a historical binary. Bundle checksums refer to the
sanitized archives. Use a fresh output directory when rerunning experiments.

Public references to algorithms, papers, dependencies, and their licenses remain.
The third-party copyright names in `vendor/whir/LICENSE-MIT` are upstream
attributions, not an author list for this repository.

Git history, remotes, commit identities, and ignored local build directories are
separate from this file-content cleanup. Deleted pre-GF256 outputs remain in
Git history. A public Git repository exposes that history and its commit
identities; an anonymous snapshot should omit `.git` and build directories.
The current source bundle command excludes those directories.

The measurement and reporting helpers redact the current checkout and home paths
from saved metadata. New AWS source bundles omit filesystem ownership and time
metadata. Review any newly added logs or archives before publishing them.

Archived campaign launchers now locate their own directory, with `CAMPAIGN_DIR`
available as an override. Set `PRIOR_CAMPAIGN_DIR` when importing an earlier
campaign. Recheck scripts use the current checkout, or `CANOPY_ROOT` when provided,
instead of the original machine's directory layout.


The current final artifacts are indexed in [results](../results/README.md);
historical experiments are indexed in [archive](../archive/README.md). The
retained numerical data, logs, receipts, and original bundles are unchanged.
Historical paths inside receipts describe the original run; relocations are
listed in `archive/cleanup_manifest.json`. Current runners reject published
result directories and the archive as output destinations.
