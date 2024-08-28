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
    in {
      devShells = {
        # nix develop
        default = pkgs.mkShell {
          name = "wayfind-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          CARGO_INCREMENTAL = "0";

          buildInputs = with pkgs; [
            # Rust
            (pkgs.rust-bin.stable."1.80.1".minimal.override {
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
            cargo-watch

            # Benchmarking
            cargo-codspeed
            gnuplot

            # Release
            cargo-semver-checks

            # Nix
            alejandra
            statix
            nil
          ];
        };

        # nix develop .#benchmarks
        benchmarks = pkgs.mkShell {
          name = "wayfind-benchmarks-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          CARGO_INCREMENTAL = "0";

          buildInputs = with pkgs; [
            (pkgs.rust-bin.stable."1.80.1".minimal)
            sccache
            cargo-codspeed
          ];
        };

        # nix develop .#ci
        ci = pkgs.mkShell {
          name = "wayfind-ci-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          CARGO_INCREMENTAL = "0";

          buildInputs = with pkgs; [
            (pkgs.rust-bin.stable."1.80.1".minimal.override {
              extensions = [
                "clippy"
                "rustfmt"
              ];
            })
            sccache
          ];
        };

        # nix develop .#coverage
        coverage = pkgs.mkShell {
          name = "wayfind-coverage-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          CARGO_INCREMENTAL = "0";

          buildInputs = with pkgs; [
            (pkgs.rust-bin.nightly."2024-07-25".minimal.override {
              extensions = [
                "llvm-tools"
              ];
            })
            sccache
            cargo-llvm-cov
          ];
        };

        # nix develop .#docs
        docs = pkgs.mkShell {
          name = "wayfind-docs-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          CARGO_INCREMENTAL = "0";

          buildInputs = with pkgs; [
            (pkgs.rust-bin.stable."1.80.1".minimal)
            sccache
          ];
        };

        # nix develop .#fuzz
        fuzz = pkgs.mkShell {
          name = "wayfind-fuzz-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          CARGO_INCREMENTAL = "0";

          buildInputs = with pkgs; [
            (pkgs.rust-bin.nightly."2024-07-25".minimal)
            sccache
            cargo-fuzz
          ];
        };

        # nix develop .#msrv
        msrv = pkgs.mkShell {
          name = "wayfind-msrv-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          CARGO_INCREMENTAL = "0";

          buildInputs = with pkgs; [
            (pkgs.rust-bin.stable."1.66.0".minimal)
            sccache
          ];
        };
      };
    });
}
