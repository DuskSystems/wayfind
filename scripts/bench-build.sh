#!/usr/bin/env -S nix develop .#ci --command bash
set -euxo pipefail

cargo codspeed build
