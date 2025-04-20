{
  nixConfig = {
    extra-substituters = [
      "https://nix-community.cachix.org"
      "https://tulpje.cachix.org"
    ];
    extra-trusted-public-keys = [
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
      "tulpje.cachix.org-1:ISTRSvsZPKD+bCTDAq3lz6XusN2dWaSE7jcOcCIhqN4="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";

    # rust
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      perSystem =
        {
          self',
          pkgs,
          system,
          ...
        }:
        let
          toolchain = inputs.fenix.packages.${system}.fromToolchainFile {
            file = ./rust-toolchain.toml;
            sha256 = "sha256-Hn2uaQzRLidAWpfmRwSRdImifGUCAb9HeAqTYFXWeQk=";
          };
          craneLib = ((inputs.crane.mkLib pkgs).overrideToolchain (_: toolchain));
          buildCrate =
            name:
            pkgs.callPackage ./nix/pkgs/tulpje.nix {
              inherit name inputs craneLib;
            };
        in
        {
          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              uv
              ruff

              jq
              cachix
              skopeo
              toolchain
              cargo-semver-checks
            ];
          };

          packages.rust-toolchain = toolchain;

          packages.tulpje-handler = buildCrate "tulpje-handler";
          packages.tulpje-gateway = buildCrate "tulpje-gateway";
          packages.tulpje-manager = buildCrate "tulpje-manager";
          packages.tulpje-utils = buildCrate "tulpje-utils";
          packages.twilight-gateway-queue = pkgs.callPackage ./nix/pkgs/twilight-gateway-queue.nix {
            inherit craneLib;
          };
          packages.twilight-http-proxy = pkgs.callPackage ./nix/pkgs/twilight-http-proxy.nix {
            inherit craneLib;
          };

          packages.default = pkgs.symlinkJoin {
            name = "tulpje";
            paths = [
              self'.packages.tulpje-handler
              self'.packages.tulpje-gateway
              self'.packages.tulpje-manager
              self'.packages.tulpje-utils
              self'.packages.twilight-gateway-queue
              self'.packages.twilight-http-proxy
            ];
          };

          packages.docker-gateway = pkgs.callPackage ./nix/oci-image.nix {
            main = self'.packages.tulpje-gateway;
          };
          packages.docker-handler = pkgs.callPackage ./nix/oci-image.nix {
            main = self'.packages.tulpje-handler;
          };
          packages.docker-gateway-queue = pkgs.callPackage ./nix/oci-image.nix {
            main = self'.packages.twilight-gateway-queue;
            utils = self'.packages.tulpje-utils;
          };
          packages.docker-http-proxy = pkgs.callPackage ./nix/oci-image.nix {
            main = self'.packages.twilight-http-proxy;
            utils = self'.packages.tulpje-utils;
          };
        };
      flake = {
        # The usual flake attributes can be defined here, including system-
        # agnostic ones like nixosModule and system-enumerating ones, although
        # those are more easily expressed in perSystem.
      };
    };
}
