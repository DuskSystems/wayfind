#!/usr/bin/env -S nix develop .#ci --command bash
set -euxo pipefail

cargo fmt --all --check
cargo clippy --workspace
typos
zizmor --pedantic .github
cargo deny check
cargo check --workspace
cargo build --workspace
cargo test --workspace
cargo test --workspace --doc
cargo doc --workspace --no-deps
