{
  description = "Template for Holochain app development";

  inputs = {
    nixpkgs.follows = "holochain-flake/nixpkgs";

    # this can now be udpated directly, e.g.:
    # nix flake lock --override-input holochain github:holochain/holochain/holochain-0.1.3
    holochain.url = "github:holochain/holochain";
    holochain.flake = false;

    holochain-flake = {
      url = "github:holochain/holochain";
      inputs.versions.url = "github:holochain/holochain/?dir=versions/0_1";
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
                  # more packages go here
                  pkgs.nodejs-18_x pkgs.binaryen 
              ];
            };
          };
      };
}