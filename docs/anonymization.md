# Anonymous source distribution

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
outside this file-content cleanup. Share the sanitized snapshot through the GitHub
anonymizer, rather than distributing the local `.git` or `target` directories.
Include the content changes and bytecode deletions in the snapshot before sharing.

The measurement and reporting helpers redact the current checkout and home paths
from saved metadata. New AWS source bundles omit filesystem ownership and time
metadata. Review any newly added logs or archives before publishing them.

Archived campaign launchers now locate their own directory, with `CAMPAIGN_DIR`
available as an override. Set `PRIOR_CAMPAIGN_DIR` when importing an earlier
campaign. Recheck scripts use the current checkout, or `CANOPY_ROOT` when provided,
instead of the original machine's directory layout.
