{
  lib,
  rustPlatform,
  fetchCrate,
  pkg-config,
  openssl,
}:

rustPlatform.buildRustPackage rec {
  pname = "cargo-codspeed";

  # NOTE: Keep in sync with `codspeed-*-compat` Rust packages.
  version = "2.8.0";

  src = fetchCrate {
    inherit pname version;
    hash = "sha256-JchV9nnaY/Kvq2FIuDrgg/rLdn9h8AWVdUiD/brOdWU=";
  };

  useFetchCargoVendor = true;
  cargoHash = "sha256-1XXh5S2qElsx4M/ag8BvqFaMhb8zQEPiekPsWyRv1mY=";

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];

  doCheck = false;

  meta = with lib; {
    homepage = "https://github.com/CodSpeedHQ/codspeed-rust";
    description = "A cargo subcommand for running CodSpeed on your project.";
    changelog = "https://github.com/CodSpeedHQ/codspeed-rust/releases";
    license = [
      licenses.mit
      licenses.asl20
    ];
    platforms = platforms.all;
    mainProgram = "cargo-codspeed";
  };
}
