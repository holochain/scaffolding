{
  description = "Template for Holochain app development";

  inputs = {
    nixpkgs.follows = "holochain/nixpkgs";
    versions.url = "github:holochain/holochain?dir=versions/0_2";

    holochain = {
      url = "github:holochain/holochain";
      inputs.versions.follows = "versions";
    };
  };

  outputs = inputs @ {...}:
    inputs.holochain.inputs.flake-parts.lib.mkFlake
    {
      inherit inputs;
    }
    {
      systems = builtins.attrNames inputs.holochain.devShells;
      perSystem = {
        self',
        inputs',
        config,
        pkgs,
        system,
        ...
      }: {
        devShells.default = pkgs.mkShell {
          inputsFrom = [inputs'.holochain.devShells.rustDev];
          packages = [pkgs.nodejs-18_x];
        };

        devShells.ci = pkgs.mkShell {
          inputsFrom = [self'.devShells.default];
          packages = [
            inputs'.holochain.packages.hc-scaffold
          ];
        };
      };
    };
}

