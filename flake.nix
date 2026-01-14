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
              # FIXME: https://github.com/NixOS/nixpkgs/pull/480112
              gungraun-runner = final.callPackage ./pkgs/gungraun-runner { };
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
            # Nix
            NIX_PATH = "nixpkgs=${nixpkgs.outPath}";

            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = "-C target-cpu=native";
            RUSTDOCFLAGS = "-D warnings";
            CARGO_INCREMENTAL = "0";
          };

          buildInputs = with pkgs; [
            (rust-bin.nightly.latest.minimal.override {
              targets = [
                "thumbv6m-none-eabi"
                "wasm32-unknown-unknown"
              ];

              extensions = [
                "clippy"
                "llvm-tools"
                "rust-analyzer"
                "rust-src"
                "rustfmt"
              ];
            })

            # Rust
            sccache
            taplo
            cargo-deny
            cargo-fuzz
            cargo-insta
            cargo-llvm-cov
            cargo-outdated
            # FIXME: https://github.com/NixOS/nixpkgs/pull/480054
            # cargo-semver-checks
            cargo-shear
            vscode-extensions.vadimcn.vscode-lldb.adapter

            # Benchmarking
            gungraun-runner

            # GitHub
            zizmor

            # Spellchecking
            typos
            typos-lsp

            # Nix
            nixfmt
            nixd
            nil
          ];
        };

        # nix develop .#ci
        ci = pkgs.mkShell {
          name = "wayfind-ci-shell";

          env = {
            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTDOCFLAGS = "-D warnings";
            CARGO_INCREMENTAL = "0";
          };

          buildInputs = with pkgs; [
            # Rust
            rust-bin.nightly.latest.rustfmt
            (rust-bin.stable.latest.minimal.override {
              extensions = [
                "clippy"
              ];
            })
            sccache
            cargo-deny

            # GitHub
            zizmor

            # Spellchecking
            typos
          ];
        };

        # nix develop .#ci-nightly
        ci-nightly = pkgs.mkShell {
          name = "wayfind-ci-nightly-shell";

          env = {
            # Rust
            RUSTC_WRAPPER = "sccache";
            CARGO_INCREMENTAL = "0";
          };

          buildInputs = with pkgs; [
            # Rust
            (rust-bin.nightly.latest.minimal.override {
              extensions = [
                "llvm-tools"
                "rust-src"
              ];
            })
            sccache
            cargo-fuzz
            cargo-llvm-cov

            # Benchmarking
            gungraun-runner
          ];
        };

        # nix develop .#ci-msrv
        ci-msrv = pkgs.mkShell {
          name = "wayfind-ci-msrv-shell";

          env = {
            # Rust
            RUSTC_WRAPPER = "sccache";
            CARGO_INCREMENTAL = "0";
          };

          buildInputs = with pkgs; [
            # Rust
            (rust-bin.stable."1.85.0".minimal.override {
              targets = [
                "thumbv6m-none-eabi"
                "wasm32-unknown-unknown"
              ];
            })
            sccache
          ];
        };
      });
    };
}
