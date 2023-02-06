use build_fs_tree::file;

use crate::file_tree::*;
use crate::versions::holochain_nix_version;

pub fn default_nix() -> FileTree {
    file!(format!(
        r#"{
  description = "Template for Holochain app development";

  inputs = {
    nixpkgs.follows = "holonix/nixpkgs";
    holonix.url = "github:holochain/holochain/pr_holonix_on_flakes";
  };

  outputs = inputs@{ ... }:
    inputs.holonix.inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = builtins.attrNames inputs.holonix.devShells;
      perSystem = { config, pkgs, system, ... }: {
        devShells.default = pkgs.mkShell {
          inputsFrom = [ inputs.holonix.devShells.${system}.holonix ];
        };
      };
    };
}"#))
}
