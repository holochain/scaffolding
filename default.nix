{ pkgs ? import (builtins.fetchTarball "https://github.com/NixOS/nixpkgs/archive/9c43581935a23d56734bd02da0ba8e7fda21e747.tar.gz") {}, ... }:

let
  workspace = pkgs.yarn2nix-moretea.mkYarnWorkspace {
    src = ./.;
    name = "hc-scaffold";

    buildPhase = ''
      yarn --offline build
    '';

    nativeBuildInputs = [
      pkgs.nodejs-16_x
    ];
  };
in workspace