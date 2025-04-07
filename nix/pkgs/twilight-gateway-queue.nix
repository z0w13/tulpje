{
  fetchFromGitHub,
  craneLib,
}:

let
  src = fetchFromGitHub {
    owner = "twilight-rs";
    repo = "gateway-queue";
    rev = "5f5e0c10757953de09a5f6b4b89951d59e94f2bf";
    hash = "sha256-yshIrPUZPtuKXOvNh2gjXRa2rsSUZboO5ZytZUjvymc=";
  };
  commonArgs = {
    inherit src;

    pname = "twilight-gateway-queue";
    version = "unstable-2025-04-04";
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
