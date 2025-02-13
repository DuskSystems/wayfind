{
  lib,
  rustPlatform,
  fetchCrate,
}:

rustPlatform.buildRustPackage rec {
  pname = "cargo-insta";

  # NOTE: Keep in sync with `cargo-insta` Rust package.
  version = "1.42.1";

  src = fetchCrate {
    inherit pname version;
    hash = "sha256-qAe3GhGcXlzmt73M/sCdUAlSCYrEaaJbxmCf4fVk6Tg=";
  };

  useFetchCargoVendor = true;
  cargoHash = "sha256-hoMAssMikg6RmNYEMxsBXXhQ5zPUSMOJh9t8mtkF8ZQ=";

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
