{
  lib,
  rustPlatform,
  fetchCrate,
  pkg-config,
  openssl,
}:

rustPlatform.buildRustPackage rec {
  pname = "cargo-codspeed";

  # NOTE: Keep in sync with `codspeed-criterion-compat` Rust package.
  version = "2.7.2";

  src = fetchCrate {
    inherit pname version;
    hash = "sha256-BtuY3reG5BMMlas1PYtaxPygbK2dptVRnYG/JRRev3c=";
  };

  cargoHash = "sha256-vioMkv0496s0zVdvi9/aQxtIsk6awXWxLyjTUBJSYhg=";

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
