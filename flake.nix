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
        default = (pkgs.mkShell.override { stdenv = pkgs.clangStdenv; }) {
          name = "wayfind-shell";

          env = {
            # Nix
            NIX_PATH = "nixpkgs=${nixpkgs.outPath}";

            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = "-C target-cpu=native -C linker=clang -C link-arg=--ld-path=wild";
            CARGO_INCREMENTAL = "0";
          };

          buildInputs = with pkgs; [
            # Rust
            rust-bin.nightly.latest.rustfmt
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
                "llvm-tools"
              ];
            })
            sccache
            wild
            taplo
            cargo-insta
            cargo-llvm-cov
            cargo-llvm-lines
            cargo-outdated
            # cargo-semver-checks
            cargo-shear
            vscode-extensions.vadimcn.vscode-lldb.adapter

            # Spellchecking
            typos
            typos-lsp

            # GitHub
            zizmor

            # Nix
            nixfmt
            nixd
            nil
          ];
        };

        # nix develop .#nightly
        nightly = (pkgs.mkShell.override { stdenv = pkgs.clangStdenv; }) {
          name = "wayfind-nightly-shell";

          env = {
            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = "-C target-cpu=native -C linker=clang -C link-arg=--ld-path=wild";
            CARGO_INCREMENTAL = "0";

            # C++
            LD_LIBRARY_PATH = "${pkgs.clangStdenv.cc.cc.lib}/lib";
          };

          buildInputs = with pkgs; [
            # Rust
            (rust-bin.nightly.latest.minimal.override {
              extensions = [ "llvm-tools" ];
            })
            wild
            sccache
            cargo-fuzz
            cargo-llvm-cov
          ];
        };

        # nix develop .#msrv
        msrv = (pkgs.mkShell.override { stdenv = pkgs.clangStdenv; }) {
          name = "wayfind-msrv-shell";

          env = {
            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = "-C target-cpu=native -C linker=clang -C link-arg=--ld-path=wild";
            CARGO_INCREMENTAL = "0";
          };

          buildInputs = with pkgs; [
            # Rust
            (rust-bin.stable."1.85.0".minimal.override {
              targets = [
                "wasm32-unknown-unknown"
                "thumbv6m-none-eabi"
              ];
            })
            wild
            sccache
          ];
        };

        # nix develop .#ci
        ci = (pkgs.mkShell.override { stdenv = pkgs.clangStdenv; }) {
          name = "wayfind-ci-shell";

          env = {
            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = "-C target-cpu=native -C linker=clang -C link-arg=--ld-path=wild";
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
            wild
            sccache
          ];
        };
      });
    };
}
