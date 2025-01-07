{
  lib,
  buildGoModule,
  fetchFromGitHub,
}:

buildGoModule rec {
  pname = "oci-conformance";
  version = "1.1.0";

  src = fetchFromGitHub {
    owner = "opencontainers";
    repo = "distribution-spec";
    rev = "v${version}";
    hash = "sha256-GL28YUwDRicxS65E7SDR/Q3tJOWN4iwgq4AGBjwVPzA=";
  };

  sourceRoot = "source/conformance";
  vendorHash = "sha256-5gn9RpjCALZB/GFjlJHDqPs2fIHl7NJr5QjPmsLnnO4=";

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
