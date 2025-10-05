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

    crate2nix = {
      url = "github:nix-community/crate2nix";
      inputs.nixpkgs.follows = "nixpkgs";
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
            sha256 = "sha256-SJwZ8g0zF2WrKDVmHrVG3pD2RGoQeo24MEXnNx5FyuI=";
          };
          # initialise crane with custom toolchain
          craneLib = ((inputs.crane.mkLib pkgs).overrideToolchain (_: toolchain));
          # function to build a workspace crate
          buildCrate =
            name:
            pkgs.callPackage ./nix/pkgs/tulpje.nix {
              inherit name inputs craneLib;
            };
          cargoNix = pkgs.callPackage ./Cargo.nix {
            buildRustCrateForPkgs =
              crate:
              pkgs.buildRustCrate.override {
                rustc = toolchain;
                cargo = toolchain;
              };
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
              cargo-edit
              cargo-machete
              cargo-outdated
              cargo-semver-checks

              inputs.crate2nix.packages.${system}.default
            ];
          };

          packages = {
            rust-toolchain = toolchain;

            # project binaries
            tulpje-handler = buildCrate "tulpje-handler";
            tulpje-gateway = buildCrate "tulpje-gateway";
            tulpje-manager = buildCrate "tulpje-manager";
            tulpje-utils = buildCrate "tulpje-utils";

            # third party binaries
            twilight-gateway-queue = pkgs.callPackage ./nix/pkgs/twilight-gateway-queue.nix {
              inherit craneLib;
            };
            twilight-http-proxy = pkgs.callPackage ./nix/pkgs/twilight-http-proxy.nix {
              inherit craneLib;
            };

            # default package that builds all binaries
            default = pkgs.symlinkJoin {
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
            crate2nix = pkgs.symlinkJoin {
              name = "all-workspace-members";
              paths =
                let
                  members = builtins.attrValues cargoNix.workspaceMembers;
                in
                builtins.map (
                  m:
                  m.build.override {
                    runTests = true;
                  }
                ) members;
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
            docker-http-proxy = pkgs.callPackage ./nix/oci-image.nix {
              main = self'.packages.twilight-http-proxy;
              utils = self'.packages.tulpje-utils;
            };
          };
        };
    };
}
