{
  fetchFromGitHub,
  craneLib,
}:

let
  src = fetchFromGitHub {
    owner = "twilight-rs";
    repo = "http-proxy";
    rev = "5d9252e5a2750cd011540db9c81b894effe2f2b0";
    hash = "sha256-+bDv9tPvBLsFucDDc+kkAvsZkFGxi03JYrZ+9CKjaJw=";
  };
  commonArgs = {
    inherit src;

    pname = "twilight-http-proxy";
    version = "unstable-2025-03-10";
    strictDeps = true;
    cargoExtraArgs = "--features expose-metrics";
  };
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
craneLib.buildPackage (
  commonArgs
  // {
    inherit cargoArtifacts;
    meta.mainProgram = "twilight-http-proxy";
  }
)
