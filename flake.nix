{
  description = "wayfind";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };

    flake-utils = {
      url = "github:numtide/flake-utils";
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
    inputs@{
      self,
      nixpkgs,
      flake-utils,
      crane,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;

          overlays = [
            (import rust-overlay)

            (self: super: {
              cargo-codspeed = pkgs.callPackage ./nix/pkgs/cargo-codspeed { inherit craneLib; };
              cargo-insta = pkgs.callPackage ./nix/pkgs/cargo-insta { inherit craneLib; };
              cargo-llvm-cov = pkgs.callPackage ./nix/pkgs/cargo-llvm-cov { inherit craneLib; };
              oci-distribution-spec-conformance = pkgs.callPackage ./nix/pkgs/oci-distribution-spec-conformance { };
            })
          ];
        };

        rustToolchain = pkgs.rust-bin.stable."1.82.0".minimal.override {
          extensions = [
            "clippy"
            "rust-analyzer"
            "rust-docs"
            "rust-src"
            "rustfmt"
            "llvm-tools"
          ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
      in
      {
        devShells = {
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

              # Nix
              nixfmt-rfc-style
              nixd
            ];
          };

          # nix develop .#benchmarks
          benchmarks = pkgs.mkShell {
            name = "wayfind-benchmarks-shell";

            RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
            RUSTFLAGS = "-C target-cpu=native";
            CARGO_INCREMENTAL = "0";

            buildInputs = with pkgs; [
              (rust-bin.stable."1.82.0".minimal)
              sccache
              cargo-codspeed
            ];
          };

          # nix develop .#ci
          ci = pkgs.mkShell {
            name = "wayfind-ci-shell";

            RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
            RUSTFLAGS = "-C target-cpu=native";
            CARGO_INCREMENTAL = "0";

            buildInputs = with pkgs; [
              (rust-bin.stable."1.82.0".minimal.override {
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
              (rust-bin.nightly."2024-10-18".minimal.override { extensions = [ "llvm-tools" ]; })
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
              (rust-bin.stable."1.82.0".minimal)
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
              (rust-bin.nightly."2024-07-25".minimal)
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
              (rust-bin.stable."1.66.0".minimal)
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
              (rust-bin.stable."1.82.0".minimal)
              sccache
              oci-distribution-spec-conformance
            ];
          };
        };

        packages = {
          # nix build .#cargo-codspeed
          cargo-codspeed = pkgs.cargo-codspeed;
          # nix build .#cargo-insta
          cargo-insta = pkgs.cargo-insta;
          # nix build .#cargo-llvm-cov
          cargo-llvm-cov = pkgs.cargo-llvm-cov;
          # nix build .#oci-distribution-spec-conformance
          oci-distribution-spec-conformance = pkgs.oci-distribution-spec-conformance;
        };

        nixosConfigurations = {
          benchmarks = nixpkgs.lib.nixosSystem {
            system = "${pkgs.stdenv.hostPlatform.uname.processor}-linux";

            specialArgs = {
              inherit inputs;
              hostPkgs = pkgs;
            };

            modules = [ ./nix/vm.nix ];
          };
        };

        apps = {
          # nix run .#benchmarks
          benchmarks = {
            type = "app";
            program = "${self.nixosConfigurations.${system}.benchmarks.config.system.build.vm}/bin/run-nixos-vm";
          };
        };
      }
    );
}
