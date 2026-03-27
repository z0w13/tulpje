{
  inputs,
  lib,
  craneLib,
  name,
  clang,
  wild,
}:
let
  unfilteredRoot = ../..;
  src = lib.fileset.toSource {
    root = unfilteredRoot;
    fileset = lib.fileset.unions [
      (craneLib.fileset.commonCargoSources unfilteredRoot)

      # add migrations and sqlx related files to sources
      (unfilteredRoot + "/crates/tulpje-handler/migrations")
      (unfilteredRoot + "/crates/tulpje-handler/.sqlx")
    ];
  };
  commonArgs = {
    inherit src;
    strictDeps = true;
    nativeBuildInputs = [
      clang
      wild
    ];
  };
  cargoArtifacts = craneLib.buildDepsOnly commonArgs // {
    pname = "tulpje-deps";
  };
in
craneLib.buildPackage (
  commonArgs
  // {
    inherit cargoArtifacts;

    pname = name;

    # to build a pecific create
    cargoExtraArgs = "-p ${name}";

    env = {
      TULPJE_VERSION_EXTRA = inputs.self.shortRev or inputs.self.dirtyShortRev or "";
      TULPJE_SKIP_VERGEN = true;
    };

    meta.mainProgram = (if name != "tulpje-utils" then name else null);
  }
)
