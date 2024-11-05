{
  pkgs,
  ...
}:

pkgs.mkShell {
  name = "wayfind-fuzz-shell";

  RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
  RUSTFLAGS = "-C target-cpu=native";
  CARGO_INCREMENTAL = "0";

  buildInputs = with pkgs; [
    (rust-bin.nightly."2024-07-25".minimal)
    sccache
    cargo-fuzz
  ];
}
