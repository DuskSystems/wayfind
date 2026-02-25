#!/usr/bin/env -S nix develop .#ci --command bash
set -euxo pipefail

cargo doc --no-deps --document-private-items
rm target/doc/.lock
echo '<meta http-equiv="refresh" content="0; url=wayfind/index.html">' > target/doc/index.html
