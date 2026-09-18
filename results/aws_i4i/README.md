# Original AWS i4i campaign — stopped

The approved 70-point experiment uses an i4i.24xlarge (Intel Xeon Platinum 8375C, 96 vCPUs / 48 physical cores, nominal 768 GiB RAM), 96 Rayon workers, one warmup and three measured repetitions. N=2^22,2^24,2^26 use eta=1,2,4,8; N=2^28 uses eta=1,2. [Full protocol and measurement plan](../../experiments/aws-r5/README.md).

All five configurations and proof-size/hash conventions are unchanged from the approved plan. The first pilot completed all 20 N=2^22 points, passed the native Intel field/FFT/binding tests, and verified the Bolt proofs. Its wall-time projections indicated roughly 3h25m for the remaining larger grid; this is an extrapolation, not a completion guarantee. Small-case speedups were substantially below the hoped-for 3x.

The initial build was placed in /dev/shm because the instance's root disk is small. Ubuntu's IPC cleanup removed that directory when the pilot tmux session ended. Results persisted on disk. The failed continuation never began another timing job. We moved the build to persistent disk and restarted the full campaign in a separate output directory to retain a consistent build for the final timing set.

- `remote/`: first successful pilot and all hardware-independent estimates; retained separately from the final campaign.
- `persistent/`: synchronized final-campaign outputs, when available. A point is usable only if its process completed, its three samples exist and applicable proof checks passed.
- `connection.json`: remote directories/session names; no key material.
- `canopy-aws-source.tar.gz`, `bundle.json`: source bundle and manifest.

Remote campaign: `./results/aws_i4i_persistent`.
Persistent binaries: `./target/aws-native/release`.
Tmux session: `canopy-aws-persistent`.
Status reporter: `./status.py`.
Root campaign log/exit marker: `./campaign.log` and `campaign.exit`.

Stopped at the user’s request with 55 completed points (22 jobs). The active WHIR N=2^26, eta=8 job is marked interrupted and excluded from accepted timings. All completed data are preserved. The five-minute monitor now follows the [parallel, single-pass restart](../aws_i4i_optimized/README.md). WHIR timings retain their component-composed scope; Bolt timings are complete serialized public-H adapter proofs.
