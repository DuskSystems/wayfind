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
            RUSTFLAGS = "-C target-cpu=native -C link-arg=-fuse-ld=mold";
            CARGO_INCREMENTAL = "0";
          };

          buildInputs = with pkgs; [
            # Rust
            (rust-bin.stable.latest.minimal.override {
              targets = [
                "wasm32-unknown-unknown"
                "thumbv6m-none-eabi"
              ];
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
            mold
            taplo
            cargo-insta
            cargo-outdated
            vscode-extensions.vadimcn.vscode-lldb.adapter

            # Coverage
            cargo-llvm-cov

            # Release
            cargo-semver-checks

            # Spellchecking
            typos
            typos-lsp

            # GitHub
            zizmor

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
            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = "-C target-cpu=native -C link-arg=-fuse-ld=mold";
            CARGO_INCREMENTAL = "0";
          };

          buildInputs = with pkgs; [
            # Rust
            (rust-bin.nightly.latest.minimal.override {
              extensions = [ "llvm-tools" ];
            })
            mold
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
            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = "-C target-cpu=native -C link-arg=-fuse-ld=mold";
            CARGO_INCREMENTAL = "0";
          };

          buildInputs = with pkgs; [
            # Rust
            (rust-bin.stable."1.88.0".minimal.override {
              targets = [
                "wasm32-unknown-unknown"
                "thumbv6m-none-eabi"
              ];
            })
            mold
            sccache
          ];
        };

        # nix develop .#ci
        ci = pkgs.mkShell {
          name = "wayfind-ci-shell";

          env = {
            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = "-C target-cpu=native -C link-arg=-fuse-ld=mold";
            CARGO_INCREMENTAL = "0";
          };

          buildInputs = with pkgs; [
            # Rust
            (rust-bin.stable.latest.minimal.override {
              extensions = [
                "clippy"
                "rustfmt"
              ];
            })
            mold
            sccache
          ];
        };
      });
    };
}
