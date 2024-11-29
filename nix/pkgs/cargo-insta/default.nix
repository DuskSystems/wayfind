{
  lib,
  craneLib,
  fetchCrate,
}:
craneLib.buildPackage rec {
  pname = "cargo-insta";

  # NOTE: Keep in sync with `cargo-insta` Rust package.
  version = "1.41.1";

  src = fetchCrate {
    inherit pname version;
    hash = "sha256-BeY3O28+tgFyCONItA8m/Ghf+OaqbbtYgbUeQwP/4NY=";
  };

  env = {
    CARGO_PROFILE_RELEASE_STRIP = "none";
  };

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
