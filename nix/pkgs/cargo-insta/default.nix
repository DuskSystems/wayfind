{
  lib,
  rustPlatform,
  fetchCrate,
}:
rustPlatform.buildRustPackage rec {
  pname = "cargo-insta";

  # NOTE: Keep in sync with `cargo-insta` Rust package.
  version = "1.40.0";

  src = fetchCrate {
    inherit pname version;
    hash = "sha256-rdfFriv3ghjqoPvRD7+TcdHehHE8ZGW4n0UT38+rRXc=";
  };

  cargoHash = "sha256-ogIPR1dronvBMyEfSkXfOsrtXYpeWGZ5a/j9ks6m4sQ=";

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
