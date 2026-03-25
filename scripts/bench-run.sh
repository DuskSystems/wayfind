#!/usr/bin/env -S nix develop .#ci --command bash
set -euxo pipefail

export CARGO_PROFILE_DEV_CODEGEN_BACKEND=llvm

cargo codspeed run --workspace
