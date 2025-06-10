{
  description = "wayfind";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
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
            rust-overlay.overlays.default
            (final: prev: {
              oci-conformance = prev.callPackage ./nix/pkgs/oci-conformance { };
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

          env = {
            NIX_PATH = "nixpkgs=${nixpkgs.outPath}";

            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = "-C target-cpu=native";
            CARGO_INCREMENTAL = "0";

            OCI_ROOT_URL = "http://127.0.0.1:8000";
            OCI_NAMESPACE = "myorg/myenv/myrepo";
            OCI_TEST_PULL = 1;
          };

          buildInputs = with pkgs; [
            # Rust
            (rust-bin.stable.latest.minimal.override {
              targets = [ "wasm32-unknown-unknown" ];
              extensions = [
                "clippy"
                "rust-analyzer"
                "rust-docs"
                "rust-src"
                "rustfmt"
                "llvm-tools"
              ];
            })
            sccache
            taplo
            cargo-insta
            cargo-outdated

            # Coverage
            cargo-llvm-cov

            # Release
            cargo-semver-checks

            # OCI
            oci-conformance

            # Spellchecking
            typos

            # Nix
            nixfmt-rfc-style
            nixd
            nil
          ];
        };

        # nix develop .#nightly
        nightly = pkgs.mkShell {
          name = "wayfind-nightly-shell";

          env = {
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = "-C target-cpu=native";
            CARGO_INCREMENTAL = "0";
          };

          buildInputs = with pkgs; [
            # Rust
            (rust-bin.nightly.latest.minimal.override { extensions = [ "llvm-tools" ]; })
            sccache

            # Coverage
            cargo-llvm-cov

            # Fuzzing
            cargo-fuzz
          ];
        };

        # nix develop .#msrv
        msrv = pkgs.mkShell {
          name = "wayfind-msrv-shell";

          env = {
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = "-C target-cpu=native";
            CARGO_INCREMENTAL = "0";
          };

          buildInputs = with pkgs; [
            # Rust
            (rust-bin.stable."1.63.0".minimal.override {
              targets = [ "wasm32-unknown-unknown" ];
            })
            sccache
          ];
        };

        # nix develop .#ci
        ci = pkgs.mkShell {
          name = "wayfind-ci-shell";

          env = {
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = "-C target-cpu=native";
            CARGO_INCREMENTAL = "0";

            OCI_ROOT_URL = "http://127.0.0.1:8000";
            OCI_NAMESPACE = "myorg/myenv/myrepo";
            OCI_TEST_PULL = 1;
          };

          buildInputs = with pkgs; [
            # Rust
            (rust-bin.stable.latest.minimal.override {
              extensions = [
                "clippy"
                "rustfmt"
              ];
            })
            sccache

            # Benchmarking
            cargo-codspeed

            # OCI
            oci-conformance
          ];
        };
      });

      packages = perSystemPkgs (pkgs: {
        oci-conformance = pkgs.oci-conformance;
      });
    };
}
