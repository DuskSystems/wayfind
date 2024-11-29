{
  description = "wayfind";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };

    crane = {
      url = "github:ipetkov/crane";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";

      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  # nix flake show
  outputs =
    {
      nixpkgs,
      crane,
      rust-overlay,
      ...
    }:

    let
      perSystem = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;

      systemPkgs = perSystem (
        system:

        import nixpkgs {
          inherit system;

          overlays = [
            (import rust-overlay)

            (final: prev: {
              rustToolchain = prev.rust-bin.stable."1.83.0".minimal.override {
                targets = [
                  "thumbv7m-none-eabi"
                  "wasm32-unknown-unknown"
                ];
                extensions = [
                  "clippy"
                  "rust-analyzer"
                  "rust-docs"
                  "rust-src"
                  "rustfmt"
                  "llvm-tools"
                ];
              };
              craneLib = (crane.mkLib prev).overrideToolchain final.rustToolchain;

              cargo-codspeed = prev.callPackage ./nix/pkgs/cargo-codspeed { craneLib = final.craneLib; };
              cargo-insta = prev.callPackage ./nix/pkgs/cargo-insta { craneLib = final.craneLib; };
              cargo-llvm-cov = prev.callPackage ./nix/pkgs/cargo-llvm-cov { craneLib = final.craneLib; };
              oci-distribution-spec-conformance = prev.callPackage ./nix/pkgs/oci-distribution-spec-conformance { };
            })
          ];
        }
      );

      perSystemPkgs = f: perSystem (system: f (systemPkgs.${system}));
    in
    {
      devShells = perSystemPkgs (pkgs: {
        # nix develop
        default = pkgs.mkShell {
          name = "wayfind-shell";

          NIX_PATH = "nixpkgs=${nixpkgs.outPath}";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          RUSTFLAGS = "-C target-cpu=native";
          CARGO_INCREMENTAL = "0";

          OCI_ROOT_URL = "http://127.0.0.1:8000";
          OCI_NAMESPACE = "myorg/myrepo";
          OCI_TEST_PULL = 1;

          buildInputs = with pkgs; [
            # Rust
            rustToolchain
            sccache
            cargo-insta
            cargo-outdated
            cargo-watch

            # Benchmarking
            cargo-codspeed
            gnuplot
            samply

            # Coverage
            cargo-llvm-cov

            # Release
            cargo-semver-checks

            # OCI
            oci-distribution-spec-conformance

            # TOML
            taplo

            # Nix
            nixfmt-rfc-style
            nixd
            nil
          ];
        };

        # nix develop .#benchmarks
        benchmarks = pkgs.mkShell {
          name = "wayfind-benchmarks-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          RUSTFLAGS = "-C target-cpu=native";
          CARGO_INCREMENTAL = "0";

          buildInputs = with pkgs; [
            (rust-bin.stable."1.83.0".minimal)
            sccache
            cargo-codspeed
          ];
        };

        # nix develop .#checks
        checks = pkgs.mkShell {
          name = "wayfind-checks-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          RUSTFLAGS = "-C target-cpu=native";
          CARGO_INCREMENTAL = "0";

          buildInputs = with pkgs; [
            (rust-bin.stable."1.83.0".minimal.override {
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
          RUSTFLAGS = "-C target-cpu=native";
          CARGO_INCREMENTAL = "0";

          buildInputs = with pkgs; [
            (rust-bin.nightly."2024-11-28".minimal.override { extensions = [ "llvm-tools" ]; })
            sccache
            cargo-llvm-cov
          ];
        };

        # nix develop .#docs
        docs = pkgs.mkShell {
          name = "wayfind-docs-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          RUSTFLAGS = "-C target-cpu=native";
          CARGO_INCREMENTAL = "0";

          buildInputs = with pkgs; [
            (rust-bin.stable."1.83.0".minimal)
            sccache
          ];
        };

        # nix develop .#fuzz
        fuzz = pkgs.mkShell {
          name = "wayfind-fuzz-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          RUSTFLAGS = "-C target-cpu=native";
          CARGO_INCREMENTAL = "0";

          buildInputs = with pkgs; [
            (rust-bin.nightly."2024-11-28".minimal)
            sccache
            cargo-fuzz
          ];
        };

        # nix develop .#msrv
        msrv = pkgs.mkShell {
          name = "wayfind-msrv-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          RUSTFLAGS = "-C target-cpu=native";
          CARGO_INCREMENTAL = "0";

          buildInputs = with pkgs; [
            (rust-bin.stable."1.83.0".minimal)
            sccache
          ];
        };

        # nix develop .#no-std
        no-std = pkgs.mkShell {
          name = "wayfind-no-std-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          CARGO_INCREMENTAL = "0";
          CARGO_BUILD_TARGET = "thumbv7m-none-eabi";

          buildInputs = with pkgs; [
            (rust-bin.stable."1.83.0".minimal.override {
              targets = [ "thumbv7m-none-eabi" ];
            })
            sccache
          ];
        };

        # nix develop .#wasm
        wasm = pkgs.mkShell {
          name = "wayfind-wasm-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          CARGO_INCREMENTAL = "0";
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";

          buildInputs = with pkgs; [
            (rust-bin.stable."1.83.0".minimal.override {
              targets = [ "wasm32-unknown-unknown" ];
            })
            sccache
          ];
        };

        # nix develop .#oci
        oci = pkgs.mkShell {
          name = "wayfind-oci-shell";

          RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          RUSTFLAGS = "-C target-cpu=native";
          CARGO_INCREMENTAL = "0";

          OCI_ROOT_URL = "http://127.0.0.1:8000";
          OCI_NAMESPACE = "myorg/myrepo";
          OCI_TEST_PULL = 1;

          buildInputs = with pkgs; [
            (rust-bin.stable."1.83.0".minimal)
            sccache
            oci-distribution-spec-conformance
          ];
        };
      });

      packages = perSystemPkgs (pkgs: {
        # nix build .#cargo-codspeed
        cargo-codspeed = pkgs.cargo-codspeed;

        # nix build .#cargo-insta
        cargo-insta = pkgs.cargo-insta;

        # nix build .#cargo-llvm-cov
        cargo-llvm-cov = pkgs.cargo-llvm-cov;

        # nix build .#oci-distribution-spec-conformance
        oci-distribution-spec-conformance = pkgs.oci-distribution-spec-conformance;
      });
    };
}
