> Archived: retired from the WHIR/Canopy experiment suite. This was a symbolic
> field-switch model, not an executable prover benchmark. Historical results
> are retained for provenance only.

# Experiment 4: one-round field switching

## Parameters

For `t in {4,8}` input polynomials, Experiment 4 uses the compatible tower

- `B = GF(2^64)`, the common ground field;
- `L = GF(2^256)`, the source field; and
- `M = GF(2^192)`, the recursive WHIR field.

The additive RS domain lies in `B`. There is no embedding from `L` into `M`;
instead both are extensions of `B`. Verifier challenges before the switch are
sampled from `L`, and the switch and all later challenges are sampled from
`M`. No challenge is sampled from the 64-bit field.

## Reduction

After the first outer-WHIR transition, each input produces two aligned
`L`-valued source codewords: the Canopy output and the outer second oracle.
Thus there are `b=2t` independently rooted words `W_a:D -> L`, all with the
same block length and rate. Their source claims have already been fixed by the
transcript:

```text
Phi_a(W_a) = v_a,                 0 <= a < b.
```

Fix a `B`-basis `beta_0,...,beta_3` of `L` and expand each source word into
four coordinate rows. Taking coordinates of all `b` source relations produces
`c=4b=8t` equations over `B`. This remains valid when `Phi_a` has coefficients
in `L`: multiplication by a fixed `L` coefficient is simply a fixed 4-by-4
`B`-linear map on coordinates. Let the resulting discrepancies be
`e_0,...,e_{c-1} in B`.

Only after all source roots, functionals, and claimed values are fixed, the
verifier samples `zeta <- M` and checks one projected equation:

```text
Delta(zeta) = sum_{j=0}^{c-1} zeta^j e_j = 0.
```

The `c` rows are indexed by `s=log2(c)` selector variables: `s=5` for four
inputs and `s=6` for eight inputs. Their selector fold is combined with the
first spatial fold of the recursive WHIR tail. The next commitment is one
`M`-valued RS word, and every subsequent OOD value, sumcheck message, folding
challenge, query answer, and terminal coefficient is in `M`.

For outer first-fold width `f_0` and first spatial tail fold `g_0`, each
physical source word has message log `n-f_0`, codeword log `n+1`, and inverse
rate log `f_0+1`. The logical selector word has message log `n-f_0+s` and
codeword log `n+1+s`; its first fold has width `s+g_0`. An initial codeword-log
reduction of `s+1` removes the selector dimensions and WHIR's ordinary one-bit
shrink, so the next codeword has log length `n`.

The initial logical oracle is virtual. At a queried spatial leaf, the prover
opens one width-`2^g_0` leaf in each of the `2t` source roots. The verifier
decomposes the opened `L` elements into `B` coordinates locally. Accordingly,
the estimator charges `2t` authentication paths and `2t * 2^g_0` 256-bit
elements, rather than materializing `8t * 2^g_0` 192-bit elements.

## Round-by-round soundness

Condition on the transcript prefix immediately before `zeta`. The roots,
claims, and correlated list `Lambda` are then fixed. For every fixed candidate
that violates at least one source claim, `Delta(Z)` is a nonzero polynomial of
degree at most `c-1`. Schwartz--Zippel and a union bound over that already-fixed
list give the one-transition error

```text
epsilon_switch <= (c-1) |Lambda| / |M|,
switch_bits = 192 - log2(|Lambda|) - log2(c-1).
```

This is a single random projection, not parallel repetition over `B`. The
64-bit values are deterministic coordinates; there are no multiple 64-bit
soundness challenges whose probabilities are multiplied. The field switch is
paid for once, after which the ordinary recursive-WHIR RBR guarantee applies
to the one projected relation. The errors compose additively.

For the proved Johnson regime, the estimator uses `eta=sqrt(rho)/20` and the
exact exceptional-set expression of
[Theorem 4.6](https://eccc.weizmann.ac.il/report/2025/169/download). For the capacity regime it
uses `eta=rho/20`, but that list-decoding input remains conjectural. Because
the corrected Johnson expression is tighter than the earlier approximation,
some 192-bit rows—especially at the largest message sizes—fall below 128 bits.
Those rows are retained and explicitly marked `meets_128_bit_rbr=false` rather
than being silently discarded.
