{
  lib,
  fetchFromGitHub,
  buildGoModule,
}:

let
  rev = "02af044d499381f6a34d65e9ac9a290251b44ac9";
in
buildGoModule {
  src = fetchFromGitHub {
    inherit rev;

    owner = "PluralKit";
    repo = "nirn-proxy";
    hash = "sha256-RtKDYpWzAQOEOnLSDzcGQyY8slJPSF4U36JkZb22ufQ=";
  };

  pname = "nirn-proxy";
  version = builtins.substring 0 7 rev;

  vendorHash = "sha256-vggC3pZmT3hProXQyudAh0K1GFJRfuoOAZTENLct7N8=";

  env.CGO_ENABLED = 0;
  tags = [ "timetzdata" ];

  meta = {
    description = "Distributed transparent REST proxy for the Discord API";
    homepage = "https://github.com/PluralKit/nirn-proxy";
    license = lib.licenses.gpl3Only;
    mainProgram = "nirn-proxy";
  };
}
