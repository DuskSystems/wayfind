{
  nixpkgs,
  pkgs,
  ...
}:

pkgs.mkShell {
  name = "wayfind-shell";

  NIX_PATH = "nixpkgs=${nixpkgs.outPath}";

  RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
  RUSTFLAGS = "-C target-cpu=native";
  CARGO_INCREMENTAL = "0";

  OCI_ROOT_URL = "http://127.0.0.1:8000";
  OCI_NAMESPACE = "myorg/myrepo";
  OCI_CROSSMOUNT_NAMESPACE = "myorg/other";
  OCI_USERNAME = "myuser";
  OCI_PASSWORD = "mypass";
  OCI_TEST_PULL = 1;
  OCI_TEST_PUSH = 0;
  OCI_TEST_CONTENT_DISCOVERY = 0;
  OCI_TEST_CONTENT_MANAGEMENT = 0;
  OCI_DEBUG = 1;
  OCI_HIDE_SKIPPED_WORKFLOWS = 1;

  buildInputs = with pkgs; [
    # Rust
    (rust-bin.stable."1.82.0".minimal.override {
      extensions = [
        "clippy"
        "rust-analyzer"
        "rust-docs"
        "rust-src"
        "rustfmt"
      ];
    })
    sccache
    cargo-insta
    cargo-outdated
    cargo-watch

    # Benchmarking
    cargo-codspeed
    gnuplot
    samply

    # Release
    cargo-semver-checks

    # OCI
    oci-distribution-spec-conformance

    # Nix
    nixfmt-rfc-style
    nixd
  ];
}
