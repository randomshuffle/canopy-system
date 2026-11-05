# Public distribution

The public tree keeps the three current experiments and their completed
reference campaign. Generated builds, local timing runs, temporary manuscript
files, credentials, and Python caches are ignored. The root MIT license and
vendored WHIR's upstream MIT/Apache licenses are retained.

The reference CSVs, logs, and source archive are unchanged evidence. Their
paths and source hashes describe the historical build; the selection manifest
maps them into the public layout. New runs write separate provenance and cannot
use the retained result directory as their destination.

This cleanup changes the working tree. Git history still contains deleted
experiments, old data, and commit author identities, and retains their storage
cost. Publishing the existing repository exposes that history. If a smaller or
anonymous public snapshot is required, prepare a new repository from the
reviewed files rather than assuming file deletion removes historical content.
