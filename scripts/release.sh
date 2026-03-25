#!/usr/bin/env -S nix develop .#ci --command bash
set -euo pipefail

BRANCH=$(git branch --show-current)
if [[ "${BRANCH}" != "main" ]]; then
  echo "error: must be on main"
  exit 1
fi

SUBJECT=$(git log -1 --format=%s)
if [[ "${SUBJECT}" != "chore: release v"* ]]; then
  echo "error: not a release commit"
  exit 1
fi

export GIT_TOKEN=$(gh auth token)
release-plz release --dry-run

read -p "Proceed? (Y/N): " CONFIRM
if [[ ! "${CONFIRM,,}" =~ ^y(es)?$ ]]; then
  exit 1
fi

release-plz release
