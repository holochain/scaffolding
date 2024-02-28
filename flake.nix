{
  description = "Template for Holochain app development";

  inputs = {
    nixpkgs.follows = "holochain/nixpkgs";
    versions.url = "github:holochain/holochain?dir=versions/weekly";
    # Override https://github.com/holochain/holochain/blob/cee25ff75590b73366cba588e54912e40f86bdb5/versions/weekly/flake.nix#L21 to point to the actual path from this repository
    # TODO: potentially move all of [this file](https://github.com/holochain/holochain/blob/develop/nix/modules/scaffolding.nix) here, requires the holochain crate to re-export the rust toolchain it uses to compile holochain itself
    versions.inputs.scaffolding.url = "path:.";

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
          packages = [pkgs.nodejs_20];
        };

        devShells.ci = pkgs.mkShell {
          inputsFrom = [self'.devShells.default];
          packages = [
            inputs'.holochain.packages.hc-scaffold
          ];
        };

        # Expose the scaffolding tool CLI as the main package for this crate
        packages.default = inputs'.holochain.packages.hc-scaffold;
      };
    };
}
