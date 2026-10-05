# Standalone GF(2^256) WHIR prototype

This isolated crate introduced the binary field and additive Reed–Solomon encoding for matched WHIR/Canopy timing comparisons against the earlier 256-bit prime. It checks polynomial-basis arithmetic, serialization, additive domains, and transposed encoding while reusing the vendored WHIR protocol. The main crate later adopted these sources as its default backend.

Original location: `experiments/binary-whir/`. This is the historical field-switch prototype; its prime comparison needs the earlier revision, and current final experiments run from the main crate. See [final results and reproduction guidance](../../../results/README.md).
