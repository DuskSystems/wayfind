{
  lib,
  buildGoModule,
  fetchFromGitHub,
}:

buildGoModule rec {
  pname = "oci-conformance";
  version = "1.1.1";

  src = fetchFromGitHub {
    owner = "opencontainers";
    repo = "distribution-spec";
    rev = "v${version}";
    hash = "sha256-cD5/9vwqcgI1ZIbIfnS3xdv806SCK1KCcNe/UYToWWk=";
  };

  sourceRoot = "source/conformance";
  vendorHash = "sha256-OYNnPlWc3IvqGl9L8zO60vaq+2bUtK/uP31cDgXw8u4=";

  env = {
    CGO_ENABLED = 0;
  };

  postInstall = ''
    go test -c ./... -o oci-conformance
    mkdir -p $out/bin
    mv oci-conformance $out/bin
  '';

  doCheck = false;

  meta = with lib; {
    homepage = "https://opencontainers.org";
    description = " OCI Distribution Specification Conformance Tests";
    changelog = "https://github.com/opencontainers/distribution-spec/releases";
    license = licenses.asl20;
    platforms = platforms.all;
    mainProgram = "oci-conformance";
  };
}
