{
  lib,
  stdenv,
  craneLib,
  fetchCrate,
  pkg-config,
  openssl,
  darwin,
}:
craneLib.buildPackage rec {
  pname = "cargo-codspeed";

  # NOTE: Keep in sync with `codspeed-criterion-compat` Rust package.
  version = "2.7.2";

  src = fetchCrate {
    inherit pname version;
    hash = "sha256-BtuY3reG5BMMlas1PYtaxPygbK2dptVRnYG/JRRev3c=";
  };

  env = {
    CARGO_PROFILE_RELEASE_STRIP = "none";
  };

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
