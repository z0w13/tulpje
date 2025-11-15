{
  fetchFromGitHub,
  craneLib,
}:

let
  src = fetchFromGitHub {
    owner = "z0w13";
    repo = "gateway-queue";
    rev = "543aae35bca748cdefaa46d0ffa570b5a8c1c4bf";
    hash = "sha256-xXl2wU5wF0iQAvW+0+crshXp3jpm9k5c+uWGowahAQs=";
  };
  commonArgs = {
    inherit src;

    pname = "twilight-gateway-queue";
    version = "unstable-2025-11-15";
    strictDeps = true;
  };
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
craneLib.buildPackage (
  commonArgs
  // {
    inherit cargoArtifacts;
    meta.mainProgram = "twilight-gateway-queue";
  }
)
