#!/usr/bin/env -S nix develop .#ci --command bash
set -euxo pipefail

export CARGO_PROFILE_DEV_CODEGEN_BACKEND=llvm

cargo llvm-cov --no-report nextest --locked --workspace
cargo llvm-cov --no-report --doc --locked --workspace
cargo llvm-cov report --doctests --codecov --output-path codecov.json
