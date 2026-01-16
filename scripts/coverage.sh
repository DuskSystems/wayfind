#!/usr/bin/env -S nix develop .#ci-nightly --command bash
set -euxo pipefail

cargo llvm-cov --workspace --doctests --codecov --output-path codecov.json
