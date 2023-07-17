{
  description = "Template for Holochain app development";

  inputs = {
    nixpkgs.follows = "holochain-flake/nixpkgs";
    versions.url = "github:holochain/holochain?dir=versions/0_2";
    versions.inputs.holochain.url = "github:holochain/holochain/holochain-0.3.0-beta-dev.9";

    holochain-flake = {
      url = "github:holochain/holochain";
      inputs.versions.follows = "versions";
      inputs.holochain.url = "github:holochain/holochain/holochain-0.2.1-beta-dev.0";
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
