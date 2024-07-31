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
            cargo-insta = pkgs.callPackage nix/pkgs/cargo-insta {};
          })
        ];
      };

      rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    in {
      devShells = {
        # nix develop
        default = pkgs.mkShell {
          name = "wayfind-shell";

          buildInputs = with pkgs; [
            # Rust
            rust-toolchain
            cargo-insta

            # Nix
            alejandra
            statix
            nil
          ];
        };
      };
    });
}
