#!/usr/bin/env bash
set -euo pipefail
campaign_base="$(cd -- "${CAMPAIGN_DIR:-$(dirname -- "${BASH_SOURCE[0]}")}" && pwd)"
trap 'campaign_result=$?; printf "%s\n" "$campaign_result" > "$campaign_base/campaign.exit"' EXIT
source "${CARGO_HOME:-$HOME/.cargo}/env"
cd "$campaign_base/canopy-system"
export RAYON_NUM_THREADS=32 CARGO_BUILD_JOBS=32
python3 scripts/aws_benchmark.py build --offline --output results/aws_i4i_optimized
cp target/aws-native/native-build.json results/aws_i4i_optimized/native-build.json
python3 scripts/aws_benchmark.py plan --output results/aws_i4i_optimized
python3 scripts/aws_benchmark.py run --output results/aws_i4i_optimized
python3 scripts/aws_benchmark.py report --output results/aws_i4i_optimized
