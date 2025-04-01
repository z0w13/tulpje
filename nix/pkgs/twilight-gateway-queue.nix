{
  fetchFromGitHub,
  craneLib,
}:

let
  src = fetchFromGitHub {
    owner = "twilight-rs";
    repo = "gateway-queue";
    rev = "50a4610bcfac229d821cb831f9d6f55d8fefdc6b";
    hash = "sha256-2ugLlx98ykpNN4onYe/0Cxx7iPMHDQhDoXDwcdhHykc=";
  };
  commonArgs = {
    inherit src;

    pname = "twilight-gateway-queue";
    version = "unstable-2025-03-10";
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
