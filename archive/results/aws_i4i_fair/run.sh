#!/usr/bin/env bash
set -euo pipefail
campaign_base="$(cd -- "${CAMPAIGN_DIR:-$(dirname -- "${BASH_SOURCE[0]}")}" && pwd)"
prior_base="$(cd -- "${PRIOR_CAMPAIGN_DIR:?Set PRIOR_CAMPAIGN_DIR to the previous campaign directory}" && pwd)"
export PRIOR_CAMPAIGN_DIR="$prior_base"
trap 'campaign_result=$?; printf "%s\n" "$campaign_result" > "$campaign_base/campaign.exit"' EXIT
printf 'Waiting for the previous campaign to release our CPU allocation.\n'
while [ ! -f "$prior_base/campaign.exit" ]; do sleep 10; done
if [ "$(cat "$prior_base/campaign.exit")" != 0 ]; then
    printf 'Previous campaign did not complete successfully; inspect it before importing results.\n'
    exit 2
fi
source "${CARGO_HOME:-$HOME/.cargo}/env"
cd "$campaign_base/canopy-system"
export RAYON_NUM_THREADS=32 CARGO_BUILD_JOBS=32
python3 "$campaign_base/import_prior.py"
mkdir -p target
cp -a "$prior_base/canopy-system/target/aws-native" target/aws-native
python3 scripts/aws_benchmark.py build --offline --output archive/results/aws_i4i_fair
cp target/aws-native/native-build.json archive/results/aws_i4i_fair/native-build.json
python3 scripts/aws_benchmark.py plan --output archive/results/aws_i4i_fair
python3 scripts/aws_benchmark.py run --family whir --output archive/results/aws_i4i_fair
python3 scripts/aws_benchmark.py report --output archive/results/aws_i4i_fair
