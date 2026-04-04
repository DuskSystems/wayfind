#!/usr/bin/env -S nix develop .#ci --command bash
set -euo pipefail

if [[ "${CI}" != "true" ]]; then
  exit 1
fi

MESSAGE=$(git log -1 --format=%s)
if [[ "${MESSAGE}" != "chore: Release v"* ]]; then
  exit 0
fi

release-plz release
