{
  lib,
  rustPlatform,
  fetchCrate,
  llvmPackages_19,
}:
rustPlatform.buildRustPackage rec {
  pname = "cargo-llvm-cov";
  version = "0.6.14";

  src = fetchCrate {
    inherit pname version;
    hash = "sha256-7Cno7/AlVnFuMSn94DF1u8g2QK9+v1/5RQ4HieejTek=";
  };

  cargoHash = "sha256-boyMQTSQqP43I38MhWv5KHa0ZASeSYdJoK+e7O20/Bw=";

  LLVM_COV = "${llvmPackages_19.llvm}/bin/llvm-cov";
  LLVM_PROFDATA = "${llvmPackages_19.llvm}/bin/llvm-profdata";

  doCheck = false;

  meta = with lib; {
    homepage = "https://github.com/taiki-e/cargo-llvm-cov";
    description = "A Cargo subcommand to easily use LLVM source-based code coverage.";
    changelog = "https://github.com/taiki-e/cargo-llvm-cov/releases";
    license = [
      licenses.asl20
      licenses.mit
    ];
    platforms = platforms.linux;
    mainProgram = "cargo-llvm-cov";
  };
}
