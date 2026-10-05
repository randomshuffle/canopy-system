# Proof-size breakdown: w=16, Merkle vs Canopy k/2

These tables decompose the existing 100-query-sample estimates. No prover-time reruns are needed. All proof-size entries are KiB (1,024 bytes); verifier hashes use the existing hash/XOF-finalization unit.

At N=2^22, k=N/16=2^18. Both Canopy RS blocks have length k/2=2^17. The optimized heights are h_X=2 and h_Y=3; the Merkle baseline has one shared WHIR batch and Canopy has separate systematic and parity batches.

## N=2^22, eta=1

| Component | Merkle KiB | Canopy k/2 KiB | Canopy minus Merkle KiB |
|---|---:|---:|---:|
| Systematic X row values | 10,488.50 | 10,488.50 | +0.00 |
| Parity Y row values | 159.50 | 159.50 | +0.00 |
| Initial X/Y commitment roots | 0.06 | 0.06 | +0.00 |
| Systematic row authentication | 1,924.10 | 1,135.72 | -788.37 |
| Parity row authentication | 67.95 | 29.48 | -38.47 |
| Bolt messages g and sigma | 0.53 | 0.53 | +0.00 |
| WHIR reductions and batched proofs | 473.04 | 674.80 | +201.75 |
| Total | 13,113.68 | 12,488.59 | -625.09 |
| Verifier hashes (count) | 135,614 | 95,833 | -39,781 |

Systematic row values are the actual 16 field elements in each queried row, not authentication paths. Each row occupies 16 x 32 = 512 bytes. The 20,977 systematic queries contribute 10,488.50 KiB and the 319 parity queries contribute 159.50 KiB. These values are identical in both methods.

Canopy saves 826.84 KiB of outer authentication and adds 201.75 KiB of WHIR/batching communication, leaving a net saving of 625.09 KiB (4.77%). The unchanged row values account for 85.3% of the Canopy proof.

Canopy's WHIR subtotal is 468.38 KiB for the systematic side and 206.41 KiB for the parity side. Merkle's single shared subtotal is 473.04 KiB and cannot be assigned uniquely to one side. These subtotals include prefix reductions, recursive oracle openings/authentication, internal commitment roots, out-of-domain values, sumcheck/terminal messages, and native batching cross evaluations. They exclude the two initial X/Y roots, which have their own row above.

Fiat-Shamir contributes zero proof bytes. Its absorbed/squeezed byte counts and verifier hash finalizations remain recorded separately in the CSV; challenges are derived by the verifier. The Bolt messages consist of w field elements in g and one field element sigma per input. Prover estimates exclude serializer framing, matching the existing report.

## N=2^22, all input counts

| eta | Method | Row values KiB | Initial roots KiB | Outer auth KiB | Bolt messages KiB | WHIR/batching KiB | Total KiB | Verifier hashes |
|---:|---|---:|---:|---:|---:|---:|---:|---:|
| 1 | Merkle | 10,648.00 | 0.06 | 1,992.04 | 0.53 | 473.04 | 13,113.68 | 135,614 |
| 1 | Canopy k/2 | 10,648.00 | 0.06 | 1,165.20 | 0.53 | 674.80 | 12,488.59 | 95,833 |
| 2 | Merkle | 21,457.00 | 0.12 | 4,000.79 | 1.06 | 860.39 | 26,319.36 | 271,330 |
| 2 | Canopy k/2 | 21,457.00 | 0.12 | 2,345.41 | 1.06 | 1,207.57 | 25,011.16 | 190,319 |
| 4 | Merkle | 43,236.00 | 0.25 | 8,034.94 | 2.12 | 1,636.28 | 52,909.59 | 544,252 |
| 4 | Canopy k/2 | 43,236.00 | 0.25 | 4,720.70 | 2.12 | 2,276.24 | 50,235.31 | 380,487 |
| 8 | Merkle | 87,120.00 | 0.50 | 16,137.74 | 4.25 | 3,195.13 | 106,457.62 | 1,093,186 |
| 8 | Canopy k/2 | 87,120.00 | 0.50 | 9,502.14 | 4.25 | 4,426.06 | 101,052.96 | 763,271 |

`proof_size_breakdown.csv` contains the same decomposition for all 7 input lengths and all 4 input counts (56 rows). Every row is checked to sum exactly to the corresponding saved total, up to floating-point precision. The scripts retain the w=16 field width and selected Canopy heights explicitly.

Reproduce with `python3 archive/scripts/breakdown_gf256_split_w16.py`.
