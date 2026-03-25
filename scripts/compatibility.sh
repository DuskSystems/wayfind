#!/usr/bin/env -S nix develop .#ci-compatibility --command bash
set -euxo pipefail

unset CARGO_PROFILE_DEV_CODEGEN_BACKEND

cargo build --locked --lib --package wayfind
cargo build --locked --lib --package wayfind --target thumbv6m-none-eabi
cargo build --locked --lib --package wayfind --target wasm32-unknown-unknown
