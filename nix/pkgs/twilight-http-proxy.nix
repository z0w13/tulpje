{
  fetchFromGitHub,
  craneLib,
}:

let
  src = fetchFromGitHub {
    owner = "twilight-rs";
    repo = "http-proxy";
    rev = "a8d38aef1dee68718280c5af0f062084eb727c20";
    hash = "sha256-rrAu1Yl/4eyRMalCV2Lck/aSuthGfnUWD2Sms3tWyfU=";
  };
  commonArgs = {
    inherit src;

    pname = "twilight-http-proxy";
    version = "0.17.0";
    strictDeps = true;
    cargoExtraArgs = "--features metrics";
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
