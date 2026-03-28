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

      # add migrations and sqlx related files to sources
      (unfilteredRoot + "/crates/tulpje-handler/migrations")
      (unfilteredRoot + "/.sqlx")
    ];
  };
  commonArgs = {
    inherit src;

    pname = name;
    strictDeps = true;
  };
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
craneLib.buildPackage (
  commonArgs
  // {
    inherit cargoArtifacts;

    # to build a pecific create
    cargoExtraArgs = "-p ${name}";

    env = {
      TULPJE_VERSION_EXTRA = inputs.self.shortRev or inputs.self.dirtyShortRev or "";
      TULPJE_SKIP_VERGEN = true;
    };

    meta.mainProgram = (if name != "tulpje-utils" then name else null);
  }
)
