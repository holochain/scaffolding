{ sources ? import ./nix/sources.nix {} 
, holonix ? import (sources.holonix) {}
, pkgs ? holonix.pkgs
, ...
}:

let
  workspace = pkgs.yarn2nix-moretea.mkYarnWorkspace {
    src = pkgs.nix-gitignore.gitignoreSource [] ./.;
    name = "hc-scaffold";

    buildPhase = ''
      yarn --offline build
    '';

    nativeBuildInputs = [
      pkgs.nodejs-16_x
    ];
  };
in workspace
