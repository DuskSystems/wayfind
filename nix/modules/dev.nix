{
  pkgs,
  ...
}:

{
  system = {
    name = "dev";
  };

  environment = {
    defaultPackages = with pkgs; [
      gcc
      (rust-bin.stable."1.82.0".minimal.override {
        extensions = [
          "clippy"
          "rustfmt"
        ];
      })
      gnuplot
      poop
    ];

    variables = {
      RUSTFLAGS = "-C target-cpu=native";
      CARGO_TARGET_DIR = "/tmp";

      OCI_ROOT_URL = "http://127.0.0.1:8000";
      OCI_NAMESPACE = "myorg/myrepo";
      OCI_CROSSMOUNT_NAMESPACE = "myorg/other";
      OCI_USERNAME = "myuser";
      OCI_PASSWORD = "mypass";
      OCI_TEST_PULL = 1;
      OCI_TEST_PUSH = 0;
      OCI_TEST_CONTENT_DISCOVERY = 0;
      OCI_TEST_CONTENT_MANAGEMENT = 0;
      OCI_DEBUG = 1;
      OCI_HIDE_SKIPPED_WORKFLOWS = 1;
    };
  };
}
