#!/usr/bin/env -S nix develop .#ci-nightly --command bash
set -euxo pipefail

TIME="${1:-60}"

rm -rf fuzz/artifacts
rm -rf fuzz/corpus

# Timeout: 1 ms
cargo fuzz run e2e \
  --sanitizer none \
  -- \
  -timeout=0.001 \
  -max_total_time="${TIME}" \
  -jobs="$(nproc)"
