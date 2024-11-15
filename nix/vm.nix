{
  inputs,
  hostPkgs,
  pkgs,
  lib,
  modulesPath,
  ...
}:

{
  imports = [
    (modulesPath + "/virtualisation/qemu-vm.nix")
  ];

  nix = {
    settings = {
      experimental-features = [
        "nix-command"
        "flakes"
      ];
    };
  };

  nixpkgs = {
    overlays = [
      (import inputs.rust-overlay)
    ];
  };

  system = {
    stateVersion = lib.trivial.release;
  };

  virtualisation = {
    cores = 2;
    memorySize = 4096;
    diskSize = 20480;
    graphics = false;

    host = {
      pkgs = hostPkgs;
    };

    mountHostNixStore = true;
    writableStoreUseTmpfs = false;

    sharedDirectories = {
      wayfind = {
        source = toString ../.;
        target = "/wayfind";
      };
    };
  };

  services = {
    getty = {
      autologinUser = "root";
    };
  };

  environment = {
    defaultPackages = with pkgs; [
      gcc
      (rust-bin.stable."1.82.0".minimal)
      sccache
      gnuplot
    ];

    variables = {
      RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
      RUSTFLAGS = "-C target-cpu=native";
      CARGO_INCREMENTAL = "0";
      CARGO_TARGET_DIR = "/tmp";
    };

    loginShellInit = ''
      cd /wayfind
      cargo bench
      shutdown -h now
    '';
  };

  documentation = {
    enable = false;
  };
}
