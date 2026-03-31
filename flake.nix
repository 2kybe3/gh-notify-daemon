{
  nixConfig.extra-substituters = [ "https://attic.kybe.xyz/main" ];
  nixConfig.extra-trusted-public-keys = [
    "main:cb7V485kGP0lG7LtQ/suOgKOgtVxNXrnD6i5yCtnaMQ="
  ];

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    crane.url = "github:ipetkov/crane/pull/1002/head";
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };

  outputs =
    {
      self,
      crane,
      nixpkgs,
      treefmt-nix,
      flake-utils,
      advisory-db,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        treefmtEval = treefmt-nix.lib.evalModule pkgs ./nix/treefmt.nix;
        craneLib = crane.mkLib pkgs;
        src = craneLib.cleanCargoSource ./.;

        commonArgs = {
          inherit src;
          strictDeps = true;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        gh-notify-daemon = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
          }
        );
      in
      {
        packages = rec {
          inherit gh-notify-daemon;
          default = gh-notify-daemon;
        };
        apps.default = {
          type = "app";
          program = "${gh-notify-daemon}/bin/gh-notify-daemon";
          meta.description = "A simple github notification daemon";
        };
        checks = {
          inherit gh-notify-daemon;

          gh-notify-daemon-clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );

          gh-notify-daemon-audit = craneLib.cargoAudit (
            commonArgs
            // {
              inherit src advisory-db;
            }
          );

          formatting = treefmtEval.config.build.check self;
        };
        devShells.default = craneLib.devShell {
          checks = self.checks.${system};
        };
        formatter = treefmtEval.config.build.wrapper;
      }
    )
    // {
      homeManagerModules = rec {
        gh-notify-daemon =
          args@{ pkgs, ... }:
          import ./nix/home-manager/gh-notify-daemon.nix (
            args
            // {
              inherit pkgs;
              package = self.packages.${pkgs.stdenv.hostPlatform.system}.gh-notify-daemon;
            }
          );
        default = gh-notify-daemon;
      };
      homeManagerModule = self.homeManagerModules.gh-notify-daemon;
    };
}
