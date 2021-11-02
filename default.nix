{ pkgs ? import <nixpkgs> {}, ... }:

let
  workspace = pkgs.yarn2nix-moretea.mkYarnWorkspace {
    src = ./.;
    name = "hc-scaffold";

    buildPhase = ''
      yarn build
    '';

  };
in workspace