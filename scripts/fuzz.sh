#!/usr/bin/env -S nix develop .#ci-nightly --command bash
set -euxo pipefail

rm -rf fuzz/artifacts
rm -rf fuzz/corpus

cargo fuzz build

for TARGET in $(cargo fuzz list); do
  # Timeout: 100 µs
  cargo fuzz run "${TARGET}" \
    -- \
    -dict=fuzz/dict/wayfind.dict \
    -timeout=0.0001 \
    -max_total_time=60 \
    -fork="$(nproc)" \
    -print_final_stats=1 \
    "${@}"
done
