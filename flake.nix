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
        default = pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
          name = "wayfind-shell";

          env = {
            # Nix
            NIX_PATH = "nixpkgs=${nixpkgs.outPath}";

            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = pkgs.lib.concatStringsSep " " [
              "-C target-cpu=native"
              "-C linker=clang"
              "-C link-arg=--ld-path=wild"
              "-Z threads=0"
            ];

            # Cargo
            CARGO_PROFILE_DEV_CODEGEN_BACKEND = "cranelift";
          };

          buildInputs = with pkgs; [
            # Rust
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
                "rustc-codegen-cranelift"
                "rustfmt"
              ];
            })
            sccache
            wild
            tombi
            cargo-codspeed
            cargo-deny
            cargo-expand
            cargo-features-manager
            cargo-fuzz
            cargo-insta
            cargo-llvm-cov
            cargo-llvm-lines
            cargo-nextest
            cargo-outdated
            cargo-semver-checks
            cargo-shear
            cargo-show-asm
            release-plz
            vscode-extensions.vadimcn.vscode-lldb.adapter

            # Git
            committed

            # GitHub
            gh
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
        ci = pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
          name = "wayfind-ci-shell";

          env = {
            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = pkgs.lib.concatStringsSep " " [
              "-C target-cpu=native"
              "-C target-feature=-avx512f"
              "-C linker=clang"
              "-C link-arg=--ld-path=wild"
              "-Z threads=0"
            ];

            # Cargo
            CARGO_INCREMENTAL = "0";
            CARGO_PROFILE_DEV_CODEGEN_BACKEND = "cranelift";
          };

          buildInputs = with pkgs; [
            # Rust
            (rust-bin.nightly.latest.minimal.override {
              extensions = [
                "clippy"
                "llvm-tools"
                "rust-src"
                "rustc-codegen-cranelift"
                "rustfmt"
              ];
            })
            sccache
            wild
            tombi
            cargo-codspeed
            cargo-deny
            cargo-fuzz
            cargo-llvm-cov
            cargo-nextest
            cargo-shear

            # Git
            committed

            # GitHub
            zizmor

            # Spellchecking
            typos

            # Nix
            nixfmt
          ];
        };

        # nix develop .#ci-compatibility
        ci-compatibility = pkgs.mkShell.override { stdenv = pkgs.clangStdenv; } {
          name = "wayfind-ci-compatibility-shell";

          env = {
            # Rust
            RUSTC_WRAPPER = "sccache";
            RUSTFLAGS = pkgs.lib.concatStringsSep " " [
              "-C target-cpu=native"
              "-C target-feature=-avx512f"
              "-C linker=clang"
              "-C link-arg=--ld-path=wild"
            ];

            # Cargo
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
            wild
          ];
        };
      });
    };
}
