{
  lib,
  buildGoModule,
  fetchFromGitHub,
}:
buildGoModule rec {
  pname = "oci-distribution-spec-conformance";
  version = "1.1.0";

  src = fetchFromGitHub {
    owner = "opencontainers";
    repo = "distribution-spec";
    rev = "v${version}";
    hash = "sha256-GL28YUwDRicxS65E7SDR/Q3tJOWN4iwgq4AGBjwVPzA=";
  };

  sourceRoot = "source/conformance";
  vendorHash = "sha256-5gn9RpjCALZB/GFjlJHDqPs2fIHl7NJr5QjPmsLnnO4=";

  CGO_ENABLED = 0;

  postInstall = ''
    go test -c ./... -o oci-distribution-spec-conformance
    mkdir -p $out/bin
    mv oci-distribution-spec-conformance $out/bin
  '';

  doCheck = false;

  meta = with lib; {
    description = " OCI Distribution Specification Conformance Tests";
    mainProgram = "oci-distribution-spec-conformance";
    homepage = "https://opencontainers.org";
    changelog = "https://github.com/opencontainers/distribution-spec/releases";
    license = licenses.asl20;
    platforms = platforms.all;
  };
}
