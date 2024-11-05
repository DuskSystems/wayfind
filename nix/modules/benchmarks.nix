{
  pkgs,
  ...
}:

{
  system = {
    name = "benchmarks";
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
}
