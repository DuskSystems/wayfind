{
  pkgs,
  ...
}:

pkgs.mkShell {
  name = "wayfind-coverage-shell";

  RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
  RUSTFLAGS = "-C target-cpu=native";
  CARGO_INCREMENTAL = "0";

  buildInputs = with pkgs; [
    (rust-bin.nightly."2024-10-18".minimal.override { extensions = [ "llvm-tools" ]; })
    sccache
    cargo-llvm-cov
  ];
}
