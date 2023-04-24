{
  description = "Template for Holochain app development";

  inputs = {
    nixpkgs.follows = "holochain/nixpkgs";

    holochain = {
      url = "github:holochain/holochain";
      inputs.versions.url = "github:holochain/holochain?dir=versions/0_1";
      inputs.holochain.url = "github:holochain/holochain/holochain-0.2.0-beta-rc.6";
    };
  };

  outputs = inputs @ { ... }:
    inputs.holochain.inputs.flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      {
        systems = builtins.attrNames inputs.holochain.devShells;
        perSystem =
          { config
          , pkgs
          , system
          , ...
          }: {
            devShells.default = pkgs.mkShell {
              inputsFrom = [ inputs.holochain.devShells.${system}.holonix ];
              # TODO: remove sqlite package once https://github.com/holochain/holochain/pull/2248 is released
              packages = [ pkgs.nodejs-18_x pkgs.sqlite ]; 
            };
          };
      };
}
