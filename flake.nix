{
  description = "wayfind";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };

    flake-utils = {
      url = "github:numtide/flake-utils";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";

      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  # nix flake show --all-systems
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;

        overlays = [
          (import rust-overlay)

          (self: super: {
            cargo-codspeed = pkgs.callPackage ./nix/pkgs/cargo-codspeed {};
            cargo-insta = pkgs.callPackage ./nix/pkgs/cargo-insta {};
          })
        ];
      };

      rust-toolchain = pkgs.rust-bin.stable."1.80.1".minimal.override {
        extensions = [
          "clippy"
          "rust-analyzer"
          "rust-docs"
          "rust-src"
          "rustfmt"
        ];
      };

      rust-toolchain-ci = pkgs.rust-bin.stable."1.80.1".minimal.override {
        extensions = [
          "clippy"
          "rustfmt"
        ];
      };

      rust-toolchain-msrv = pkgs.rust-bin.stable."1.66.0".minimal;
      rust-toolchain-nightly = pkgs.rust-bin.nightly."2024-07-25".minimal;
    in {
      devShells = {
        # nix develop
        default = pkgs.mkShell {
          name = "wayfind-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          CARGO_INCREMENTAL = "0";

          buildInputs = with pkgs; [
            # Rust
            rust-toolchain
            sccache
            cargo-codspeed
            cargo-insta

            # Benchmarking
            gnuplot

            # Nix
            alejandra
            statix
            nil
          ];
        };

        # nix develop .#ci
        ci = pkgs.mkShell {
          name = "wayfind-ci-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          CARGO_INCREMENTAL = "0";

          buildInputs = with pkgs; [
            # Rust
            rust-toolchain-ci
            sccache
            cargo-codspeed
          ];
        };

        # nix develop .#msrv
        msrv = pkgs.mkShell {
          name = "wayfind-msrv-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          CARGO_INCREMENTAL = "0";

          buildInputs = with pkgs; [
            # Rust
            rust-toolchain-msrv
            sccache
          ];
        };

        # nix develop .#fuzz
        fuzz = pkgs.mkShell {
          name = "wayfind-fuzz-shell";

          buildInputs = with pkgs; [
            # Rust
            rust-toolchain-nightly

            # Fuzzing
            cargo-fuzz
          ];
        };
      };
    });
}
