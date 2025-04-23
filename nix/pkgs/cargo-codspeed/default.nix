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
  version = "2.10.1";

  src = fetchCrate {
    inherit pname version;
    hash = "sha256-tgMOqi2XGbKTDrVQ3Op+m4DBAUAITIB+GibjaQ8x/MI=";
  };

  useFetchCargoVendor = true;
  cargoHash = "sha256-YGmnz9P2S0baENC9lP1qMMLSzv2lUbvyjcl8FWpDvZ0=";

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
