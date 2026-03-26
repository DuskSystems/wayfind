#!/usr/bin/env -S nix develop .#ci --command bash
set -euxo pipefail

export CARGO_PROFILE_DEV_CODEGEN_BACKEND=llvm

# Must disable LTO:
# https://github.com/rust-fuzz/cargo-fuzz/issues/384
export CARGO_PROFILE_RELEASE_LTO=false

rm -rf fuzz/artifacts
rm -rf fuzz/corpus

# No `--locked` support:
# https://github.com/rust-fuzz/cargo-fuzz/issues/312
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
