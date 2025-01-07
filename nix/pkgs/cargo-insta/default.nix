{
  lib,
  rustPlatform,
  fetchCrate,
}:

rustPlatform.buildRustPackage rec {
  pname = "cargo-insta";

  # NOTE: Keep in sync with `cargo-insta` Rust package.
  version = "1.42.0";

  src = fetchCrate {
    inherit pname version;
    hash = "sha256-FMEyP5AJpPCeW9I/me1Radlte1GVuvb7rib9rpkTD7I=";
  };

  cargoHash = "sha256-bk1ert4RaIpyrLNZ+ESg8nwBamxZqvvZmkKYLM5rz2E=";

  doCheck = false;

  meta = with lib; {
    homepage = "https://github.com/mitsuhiko/insta";
    description = "A Cargo subcommand for snapshot testing.";
    changelog = "https://github.com/mitsuhiko/insta/releases";
    license = licenses.asl20;
    platforms = platforms.all;
    mainProgram = "cargo-insta";
  };
}
