#!/usr/bin/env -S nix develop .#ci --command bash
set -euo pipefail

export GIT_TOKEN=$(gh auth token)
release-plz release-pr
