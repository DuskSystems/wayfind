{
  inputs,
  config,
  hostPkgs,
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
        source = toString ../../.;
        target = "/wayfind";
      };
    };
  };

  networking = {
    hostName = config.system.name;
  };

  services = {
    getty = {
      autologinUser = "root";
    };
  };

  documentation = {
    enable = false;
  };
}
