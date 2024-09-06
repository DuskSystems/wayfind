{
  lib,
  stdenv,
  rustPlatform,
  fetchCrate,
  pkg-config,
  openssl,
  darwin,
}:
rustPlatform.buildRustPackage rec {
  pname = "cargo-codspeed";

  # NOTE: Keep in sync with `codspeed-criterion-compat` Rust package.
  version = "2.7.1";

  src = fetchCrate {
    inherit pname version;
    hash = "sha256-crucisC3wCiEX5eXufricyBY/UuueJNDXQoyGY2+RF0=";
  };

  cargoHash = "sha256-47aJR4ZXax39fjhSu/0eVOtnCmmGw+S/8ea/I7/RAg0=";

  nativeBuildInputs = [ pkg-config ];
  buildInputs =
    [ openssl ]
    ++ lib.optionals stdenv.isDarwin [
      darwin.apple_sdk.frameworks.SystemConfiguration
      darwin.apple_sdk.frameworks.CoreServices
    ];

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
