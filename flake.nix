{
  description = "Template for Holochain app development";

  inputs = {
    nixpkgs.follows = "holochain-flake/nixpkgs";

    holochain.url = "github:holochain/holochain";
    holochain.flake = false;

    holochain-flake = {
      url = "github:holochain/holochain";
      inputs.versions.url = "github:holochain/holochain?dir=versions/0_1";
      inputs.holochain.url = "github:holochain/holochain/holochain-0.2.0-beta-rc.6";
    };
  };

  outputs = inputs @ { ... }:
    inputs.holochain-flake.inputs.flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      {
        systems = builtins.attrNames inputs.holochain-flake.devShells;
        perSystem =
          { config
          , pkgs
          , system
          , ...
          }: {
            devShells.default = pkgs.mkShell {
              inputsFrom = [ inputs.holochain-flake.devShells.${system}.holonix ];
              packages = with pkgs; [
                  pkgs.nodejs-18_x 
              ];
            };
          };
      };
}