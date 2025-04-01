{
  inputs,
  lib,
  craneLib,
  name,
}:
let
  unfilteredRoot = ../..;
  src = lib.fileset.toSource {
    root = unfilteredRoot;
    fileset = lib.fileset.unions [
      (craneLib.fileset.commonCargoSources unfilteredRoot)

      (unfilteredRoot + "/handler/migrations")
      (unfilteredRoot + "/handler/.sqlx")
    ];
  };
  commonArgs = {
    inherit src;

    pname = name;
    strictDeps = true;
    cargoExtraArgs = "-p ${name}";
  };
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
craneLib.buildPackage (
  commonArgs
  // {
    inherit cargoArtifacts;

    env = {
      TULPJE_VERSION_EXTRA = inputs.self.shortRev or inputs.self.dirtyShortRev or "";
      TULPJE_SKIP_VERGEN = true;
    };

    meta.mainProgram = (if name != "tulpje-utils" then name else null);
  }
)
