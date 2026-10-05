#!/usr/bin/env bash
set -euo pipefail
campaign_base="$(cd -- "${CAMPAIGN_DIR:-$(dirname -- "${BASH_SOURCE[0]}")}" && pwd)"
trap 'campaign_result=$?; printf "%s\n" "$campaign_result" > "$campaign_base/campaign.exit"' EXIT
source "${CARGO_HOME:-$HOME/.cargo}/env"
cd "$campaign_base/canopy-system"
export RAYON_NUM_THREADS=32 CARGO_BUILD_JOBS=32
campaign_cfg=experiments/aws-r5/config-n30-eta1.json
campaign_out=results/aws_i4i_n30_eta1
printf '%s\n' build > "$campaign_base/phase"
python3 scripts/aws_benchmark.py build --offline --config "$campaign_cfg" --output "$campaign_out"
cp target/aws-native/native-build.json "$campaign_out/native-build.json"
printf '%s\n' preparation > "$campaign_base/phase"
python3 scripts/aws_benchmark.py plan --config "$campaign_cfg" --output "$campaign_out"
python3 scripts/aws_benchmark.py prepare --config "$campaign_cfg" --output "$campaign_out"
printf '%s\n' timing > "$campaign_base/phase"
python3 scripts/aws_benchmark.py run --config "$campaign_cfg" --output "$campaign_out"
python3 scripts/aws_benchmark.py report --config "$campaign_cfg" --output "$campaign_out"
printf '%s\n' validation > "$campaign_base/phase"
python3 "$campaign_base/validate.py"
printf '%s\n' complete > "$campaign_base/phase"
