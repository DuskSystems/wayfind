#!/usr/bin/env -S nix develop .#ci-msrv --command bash
set -euxo pipefail

cargo build --lib --package wayfind
cargo build --lib --package wayfind --target thumbv6m-none-eabi
cargo build --lib --package wayfind --target wasm32-unknown-unknown
