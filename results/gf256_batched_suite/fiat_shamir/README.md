# Fiat–Shamir accounting

The headline **verifier hashes** metric is authentication (leaf + parent)
plus Canopy hash-to-field plus Fiat–Shamir hash/XOF finalizations. Each complete
BLAKE3 hash or creation of an XOF output reader counts once. Consecutive
squeezes share that reader; a subsequent absorption starts a new stream.
This is not a count of BLAKE3 compression calls and is not a verifier-time
estimate. In particular, a long stream can cost substantially more than a
short one. Absorbed and squeezed transcript bytes are reported separately.

Fiat–Shamir adds **zero proof bytes**: verifier challenges are derived locally,
all prover messages already appear in the communication model, and there is
no proof of work or transmitted random seed. The deterministic seeds used
for sampled proof-size estimates are experiment seeds, not proof messages.
The accounting pass itself changes no query counts, selected parameters, proof
sizes, or timings. The Bolt experiment was separately rerun with fixed w=4
and eta=1,2,4,8 inputs; its multi-input proofs share one recursive tail.

## Accounting boundary

A per-proof ledger starts by absorbing 64-byte protocol and 32-byte session
identifiers plus the empty public-instance encoding. We treat construction
of these fixed identifiers (SHA3 in Spongefish's domain-separator helper) as
cached public setup, and include their 96 bytes of per-proof absorption.
Application-specific public inputs outside the synthetic experiment are not
invented or charged here. GF(2^256) challenges consume exactly 32 bytes; index
sampling consumes ceil(log2(number of leaves)/8) bytes per draw.

Native WHIR's authenticated row values and Merkle paths are transcript hints,
not sponge inputs. Its roots, OOD responses, sumcheck coefficients, batching
cross evaluations, and final vector are absorbed. The Bolt adapter additionally
absorbs its outer row values and authentication digests before forming their
linear constraints. This explains the much larger absorbed-byte totals for
Bolt. H2F calls are included in verifier hashes but are not FS transcript bytes.

## WHIR: explicit composed protocol model

`src/paper_fiat_shamir.rs` records one composed transcript, with three
sumcheck coefficients per round to match the paper communication model.
It does not sum the transcripts of independently benchmarked components.
For one baseline input it uses a direct ordinary WHIR transcript. Otherwise,
each input's first transition runs in input order (initial root and OOD,
constraint combination, first sumcheck, aligned second root and OOD, old-oracle
queries and constraint combination), followed by one shared recursive proof.
The Canopy front also absorbs its bottom opening values/digests before the SEP
combination challenge and supplies OOD responses for the Canopy source.
The sources then send the 2p-3 generalized Schwartz–Zippel cross coefficients
and draw the batching challenge. Recursive rounds absorb a new root and OOD
responses, sample old-oracle queries, combine constraints, and run sumcheck.
The final vector precedes the last queries and final sumcheck.

All 84 selected configurations reconcile absorbed roots/algebraic messages
against independently generated proof-size components. Canopy's mean bottom
opening byte count comes from the same 100 query samples as its size estimate.
The SHA3 identifier boundary and BLAKE3 XOF convention match the native
transcript. These WHIR totals are modeled; the component timing harness does
not produce a single serialized end-to-end Exp3 argument. Native WHIR uses
two sumcheck coefficients, so the modeled byte totals are not asserted to
match its component transcripts byte for byte.

## Bolt: model checked against executable transcripts

`src/bolt_fiat_shamir.rs` models the existing serialized public-H adapter.
The optional `fs-accounting` feature wraps the exact BLAKE3 sponge without
changing its output bytes. `bolt_validation.csv` records 20 verified proofs: all four selected layouts
at N=2^18 for each eta=1,2,4,8, and the baseline/selected Canopy pair at
N=2^22 for eta=1 and eta=8. Width is fixed at four throughout. Their complete absorb/squeeze event sequences match
the model (adjacent absorptions normalized), and absorbed bytes equal
96 + the serialized non-interactive message string length.

`bolt.csv` covers all 112 selected input-count/layout/size configurations.
All 2 eta outer roots are absorbed first, followed by each input’s front
transition, then all alignment reductions and one shared native WHIR proof. Its
absorbed-byte means use the existing sampled outer openings. The outer
sampler draws distinct rows, so expected squeezed bytes include duplicate
retries: for q<R, the expected number of draws is sum(i=0..q-1) R/(R-i).
A full read uses no query randomness. Internal WHIR queries sample with
replacement. Stream-finalization counts are deterministic at fixed parameters.
Actual trace byte totals need not equal these means.

## Reproduce

```sh
cargo run --offline --release --features fs-accounting \
  --example fiat_shamir_accounting -- --validate
python3 scripts/summarize_gf256_suite.py
```

Omit `--validate` for the quick accounting-only pass. The validation pass
executes fresh proofs solely to check transcript accounting; it does not
replace or add to the saved prover timing samples. The feature is disabled
in ordinary benchmark builds. `whir.csv` and `bolt.csv` join onto the selected
proof-size CSVs; raw height-search candidates retain their original auth/H2F
components and are not claimed to contain full FS totals.
