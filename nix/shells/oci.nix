{
  pkgs,
  ...
}:

pkgs.mkShell {
  name = "wayfind-oci-shell";

  RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
  RUSTFLAGS = "-C target-cpu=native";
  CARGO_INCREMENTAL = "0";

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

  buildInputs = with pkgs; [
    (rust-bin.stable."1.82.0".minimal)
    sccache
    oci-distribution-spec-conformance
  ];
}
