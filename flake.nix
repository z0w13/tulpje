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
          # create the toolchain from `rust-toolchain.toml`
          toolchain = inputs.fenix.packages.${system}.fromToolchainFile {
            file = ./rust-toolchain.toml;
            sha256 = "sha256-qqF33vNuAdU5vua96VKVIwuc43j4EFeEXbjQ6+l4mO4=";
          };
          # initialise crane with custom toolchain
          craneLib = ((inputs.crane.mkLib pkgs).overrideToolchain (_: toolchain));
          # function to build a workspace crate
          buildCrate =
            name:
            pkgs.callPackage ./nix/pkgs/tulpje.nix {
              inherit name inputs craneLib;
            };
        in
        {
          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              git-cliff

              uv
              ruff

              jq
              cachix
              hyperfine
              skopeo
              toolchain
              cargo-edit
              cargo-hakari
              cargo-machete
              cargo-outdated
              cargo-semver-checks

              sqlx-cli
            ];
          };

          packages = {
            rust-toolchain = toolchain;

            # project binaries
            tulpje-handler = buildCrate "tulpje-handler";
            tulpje-gateway = buildCrate "tulpje-gateway";
            tulpje-utils = buildCrate "tulpje-utils";

            # third party binaries
            twilight-gateway-queue = pkgs.callPackage ./nix/pkgs/twilight-gateway-queue.nix {
              inherit craneLib;
            };
            nirn-proxy = pkgs.callPackage ./nix/pkgs/nirn-proxy.nix { };

            # default package that builds all binaries
            default = pkgs.symlinkJoin {
              name = "tulpje";
              paths = [
                self'.packages.tulpje-handler
                self'.packages.tulpje-gateway
                self'.packages.tulpje-utils
                self'.packages.twilight-gateway-queue
                self'.packages.nirn-proxy
              ];
            };

            # docker images
            docker-gateway = pkgs.callPackage ./nix/oci-image.nix {
              main = self'.packages.tulpje-gateway;
            };
            docker-handler = pkgs.callPackage ./nix/oci-image.nix {
              main = self'.packages.tulpje-handler;
            };
            docker-gateway-queue = pkgs.callPackage ./nix/oci-image.nix {
              main = self'.packages.twilight-gateway-queue;
              utils = self'.packages.tulpje-utils;
            };
            docker-nirn-proxy = pkgs.callPackage ./nix/oci-image.nix {
              main = self'.packages.nirn-proxy;
              utils = self'.packages.tulpje-utils;
            };
          };
        };
    };
}
