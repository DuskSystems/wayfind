#!/usr/bin/env -S nix develop .#ci-nightly --command bash
set -euxo pipefail

TIME="${1:-15}"

rm -rf fuzz/artifacts
rm -rf fuzz/corpus

# Timeout: 100 µs
cargo fuzz run e2e \
  --sanitizer none \
  -- \
  -timeout=0.0001 \
  -max_total_time="${TIME}" \
  -fork="$(nproc)"
