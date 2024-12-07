{
  lib,
  craneLib,
  fetchCrate,
  llvmPackages_19,
}:
craneLib.buildPackage rec {
  pname = "cargo-llvm-cov";
  version = "0.6.14";

  src = fetchCrate {
    inherit pname version;
    hash = "sha256-7Cno7/AlVnFuMSn94DF1u8g2QK9+v1/5RQ4HieejTek=";
  };

  env = {
    CARGO_PROFILE_RELEASE_STRIP = "none";
    LLVM_COV = "${llvmPackages_19.llvm}/bin/llvm-cov";
    LLVM_PROFDATA = "${llvmPackages_19.llvm}/bin/llvm-profdata";
  };

  doCheck = false;

  meta = with lib; {
    homepage = "https://github.com/taiki-e/cargo-llvm-cov";
    description = "A Cargo subcommand to easily use LLVM source-based code coverage.";
    changelog = "https://github.com/taiki-e/cargo-llvm-cov/releases";
    license = [
      licenses.asl20
      licenses.mit
    ];
    platforms = platforms.all;
    mainProgram = "cargo-llvm-cov";
  };
}
