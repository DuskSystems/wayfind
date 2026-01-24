#!/usr/bin/env -S nix develop .#ci-nightly --command bash
set -euxo pipefail

cargo codspeed build
