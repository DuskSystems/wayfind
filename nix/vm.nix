{
  inputs,
  name,
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
    inherit name;
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

  networking = {
    hostName = name;
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
      gnuplot
    ];

    variables = {
      RUSTFLAGS = "-C target-cpu=native";
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
