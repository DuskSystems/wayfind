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
  version = "2.6.0";

  src = fetchCrate {
    inherit pname version;
    hash = "sha256-e4cDZ+sbylS3gaJpl6FLYPUVedBeAOGqtxDw8YbU9U8=";
  };

  cargoHash = "sha256-ihkmD48EG1fWJ38AAt4Hy6dpATIgnEb6kApDfsTKxwA=";

  nativeBuildInputs = [pkg-config];
  buildInputs =
    [openssl]
    ++ lib.optionals stdenv.isDarwin [
      darwin.apple_sdk.frameworks.SystemConfiguration
      darwin.apple_sdk.frameworks.CoreServices
    ];

  doCheck = false;

  meta = with lib; {
    description = "A cargo subcommand for running CodSpeed on your project.";
    mainProgram = "cargo-codspeed";
    homepage = "https://github.com/CodSpeedHQ/codspeed-rust";
    changelog = "https://github.com/CodSpeedHQ/codspeed-rust/releases/tag/v${version}";
    license = [licenses.mit licenses.asl20];
    platforms = platforms.all;
  };
}
