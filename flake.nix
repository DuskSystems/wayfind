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

      rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      rust-toolchain-msrv = pkgs.rust-bin.stable."1.66.0".default;
      rust-toolchain-nightly = pkgs.rust-bin.nightly."2024-07-25".default;
    in {
      devShells = {
        # nix develop
        default = pkgs.mkShell {
          name = "wayfind-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          CARGO_INCREMENTAL = "0";

          buildInputs = with pkgs;
            [
              # Rust
              rust-toolchain
              sccache
              cargo-codspeed
              cargo-insta
              cargo-nextest

              # Benchmarking
              gnuplot

              # Nix
              alejandra
              statix
              nil
            ]
            ++ lib.optionals pkgs.stdenv.isLinux [
              # Rust
              # NOTE: https://github.com/NixOS/nixpkgs/pull/260725
              cargo-llvm-cov
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
