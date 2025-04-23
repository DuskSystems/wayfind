{
  lib,
  rustPlatform,
  fetchCrate,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "cargo-insta";

  # NOTE: Keep in sync with `cargo-insta` Rust package.
  version = "1.42.2";

  src = fetchCrate {
    inherit (finalAttrs) pname version;
    hash = "sha256-bsLV+iQYbqOv+OftOPAt/89vZ738GgnNlHt1JNAc+m0=";
  };

  useFetchCargoVendor = true;
  cargoHash = "sha256-bdioXT3Bm+BnSRiMW9M7b587KceVFLqeJ+N8+x9+0sE=";

  doCheck = false;

  meta = with lib; {
    homepage = "https://github.com/mitsuhiko/insta";
    description = "A Cargo subcommand for snapshot testing.";
    changelog = "https://github.com/mitsuhiko/insta/releases";
    license = licenses.asl20;
    platforms = platforms.all;
    mainProgram = "cargo-insta";
  };
})
