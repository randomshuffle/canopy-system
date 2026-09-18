# AWS pilot versus saved Mac measurements

N=2^22, eta=1. Median commitment + opening seconds. Same selected protocol geometries; different measurement campaigns. AWS uses 96 workers and 3 measured runs; saved Mac uses 8 workers and 5 WHIR / 10 Bolt measured runs. Modest differences are observations, not statistical significance claims.

| Configuration | Mac seconds | AWS seconds | AWS / Mac |
|---|---:|---:|---:|
| WHIR 1/2 | 0.730817 | 0.677701 | 0.927 |
| WHIR 1/4 | 1.467526 | 1.181210 | 0.805 |
| Canopy–WHIR, half length | 0.902544 | 0.846272 | 0.938 |
| Bolt–Merkle | 0.557525 | 0.611360 | 1.097 |
| Bolt–Canopy, k/4 | 0.612072 | 0.682921 | 1.116 |

Bolt–Merkle mean commitment falls from 0.226709 s to 0.080003 s, while mean opening rises from 0.345379 s to 0.526949 s. Large opening increases are partial evaluation g (0.0520 to 0.1358 s) and public-H transpose covector construction (0.0515 to 0.1007 s). These stage observations do not by themselves identify a hardware-level bottleneck.

Across whole N=2^22 jobs, (user CPU + system CPU)/wall time is 19.5–26.6 logical-CPU equivalents for WHIR and 8.8–10.7 for Bolt. This includes setup and verification, not just the timed prover region. It shows that setting 96 workers does not translate to using 96 CPU equivalents throughout the harness.

The old approximately 3-hour estimate summed extrapolated prover work on Mac. AWS ETA extrapolates complete job wall time, including setup, input generation, verification, and extra WHIR component/suffix runs. At eta=1, the AWS WHIR job takes 22.0 s; four times the sum of its mean reported prover times is about 10.85 s. Bolt takes 9.0 s versus about 5.17 s for four passes of mean prover work. The remaining-time estimate must include that overhead. Larger-input measured job wall times replace N=2^22 anchors as they complete.

Sources: persistent/comparison.csv; persistent/timings/*/process.json and prover_timing_stages.csv; ../whir_half_block_eta1/timing_n22/; ../bolt_half_quarter_shared_baseline_eta1/timing_n22/.
