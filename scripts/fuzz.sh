#!/usr/bin/env -S nix develop .#ci-nightly --command bash
set -euxo pipefail

TIME="${1:-15}"
for TARGET in $(cargo fuzz list); do
  cargo fuzz run "${TARGET}" --sanitizer none -- -max_total_time="${TIME}" -jobs="$(nproc)"
done
