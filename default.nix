{ pkgs ? import <nixpkgs> {}, ... }:

let
  workspace = pkgs.yarn2nix-moretea.mkYarnWorkspace {
    src = ./.;
    name = "hc-scaffold";

    buildPhase = ''
      yarn build
    '';

    nativeBuildInputs = [
      pkgs.nodejs-16_x
    ];

  };
in workspace