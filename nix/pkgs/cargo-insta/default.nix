{
  lib,
  rustPlatform,
  fetchCrate,
}:
rustPlatform.buildRustPackage rec {
  pname = "cargo-insta";

  # NOTE: Keep in sync with `cargo-insta` Rust package.
  version = "1.39.0";

  src = fetchCrate {
    inherit pname version;
    hash = "sha256-LUgiTIVWjxPTCQ1gZq5zL2UMxnEfC09w9xudn/9AUwM=";
  };

  cargoHash = "sha256-FH1d8sub8oqUnEr7oO6vofdLoL6KIHgOuF3Exar75t0=";

  meta = with lib; {
    description = "A Cargo subcommand for snapshot testing.";
    mainProgram = "cargo-insta";
    homepage = "https://github.com/mitsuhiko/insta";
    changelog = "https://github.com/mitsuhiko/insta/releases";
    license = licenses.asl20;
    platforms = platforms.all;
  };
}
