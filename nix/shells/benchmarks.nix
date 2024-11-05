{
  pkgs,
  ...
}:

pkgs.mkShell {
  name = "wayfind-benchmarks-shell";

  RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
  RUSTFLAGS = "-C target-cpu=native";
  CARGO_INCREMENTAL = "0";

  buildInputs = with pkgs; [
    (rust-bin.stable."1.82.0".minimal)
    sccache
    cargo-codspeed
  ];
}
