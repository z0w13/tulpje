{
  fetchFromGitHub,
  craneLib,
}:

let
  src = fetchFromGitHub {
    owner = "twilight-rs";
    repo = "gateway-queue";
    rev = "aa727cd6765f55975f6188ad8fac14e1c53280f2";
    hash = "sha256-uOtpCdc9l2/tiaVtKmSv4895JK2K2QJOznsMwxDJ5QY=";
  };
  commonArgs = {
    inherit src;

    pname = "twilight-gateway-queue";
    version = "unstable-2025-10-08";
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
