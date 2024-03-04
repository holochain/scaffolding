{
  description = "Template for Holochain app development";

  inputs = {
    holochain-nix-versions.url  = "github:holochain/holochain?dir=versions/weekly";

    holochain-flake = {
      url = "github:holochain/holochain";
      inputs.versions.follows = "holochain-nix-versions";
    };

    nixpkgs.follows = "holochain-flake/nixpkgs";
    flake-parts.follows = "holochain-flake/flake-parts";
    scaffolding.url = "github:<TODO:REPLACE_ME_WITH_THE_APPROPRIATE_GIT_URL>";
  };

  outputs = inputs @ { flake-parts, holochain-flake, ... }:
    flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      {
        systems = builtins.attrNames holochain-flake.devShells;
        perSystem =
          { config
          , pkgs
          , system
          , inputs'
          , ...
          }: {
            devShells.default = pkgs.mkShell {
              inputsFrom = [ holochain-flake.devShells.${system}.holonix ];

              packages = with pkgs; [
                nodejs_20
              ];
            };

            packages.hc-scaffold-custom-template = inputs.scaffolding.lib.wrapCustomTemplate {
              inherit pkgs system;
              customTemplatePath = ./template;
            };

          };
      };
}
