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

  # nix flake show
  outputs =
    inputs@{
      self,
      nixpkgs,
      flake-utils,
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
              cargo-codspeed = pkgs.callPackage ./nix/pkgs/cargo-codspeed { };
              cargo-insta = pkgs.callPackage ./nix/pkgs/cargo-insta { };
              oci-distribution-spec-conformance = pkgs.callPackage ./nix/pkgs/oci-distribution-spec-conformance { };
            })
          ];
        };
      in
      {
        devShells = {
          # nix develop
          default = pkgs.callPackage ./nix/shells/default.nix {
            inherit nixpkgs;
          };

          # nix develop .#benchmarks
          benchmarks = pkgs.callPackage ./nix/shells/benchmarks.nix { };

          # nix develop .#ci
          ci = pkgs.callPackage ./nix/shells/ci.nix { };

          # nix develop .#coverage
          coverage = pkgs.callPackage ./nix/shells/coverage.nix { };

          # nix develop .#docs
          docs = pkgs.callPackage ./nix/shells/docs.nix { };

          # nix develop .#fuzz
          fuzz = pkgs.callPackage ./nix/shells/fuzz.nix { };

          # nix develop .#msrv
          msrv = pkgs.callPackage ./nix/shells/msrv.nix { };

          # nix develop .#oci
          oci = pkgs.callPackage ./nix/shells/oci.nix { };
        };

        nixosConfigurations = {
          dev = nixpkgs.lib.nixosSystem {
            system = "${pkgs.stdenv.hostPlatform.uname.processor}-linux";

            specialArgs = {
              inherit inputs;
              hostPkgs = pkgs;
            };

            modules = [
              ./nix/modules/vm.nix
              ./nix/modules/dev.nix
            ];
          };

          benchmarks = nixpkgs.lib.nixosSystem {
            system = "${pkgs.stdenv.hostPlatform.uname.processor}-linux";

            specialArgs = {
              inherit inputs;
              hostPkgs = pkgs;
            };

            modules = [
              ./nix/modules/vm.nix
              ./nix/modules/benchmarks.nix
            ];
          };
        };

        apps = {
          # nix run .#dev
          dev = {
            type = "app";
            program = "${self.nixosConfigurations.${system}.dev.config.system.build.vm}/bin/run-dev-vm";
          };

          # nix run .#benchmarks
          benchmarks = {
            type = "app";
            program = "${self.nixosConfigurations.${system}.benchmarks.config.system.build.vm}/bin/run-benchmarks-vm";
          };
        };
      }
    );
}
