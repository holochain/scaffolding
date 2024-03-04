# Custom Template - Holochain Scaffolding Tool

Custom template for the [scaffolding tool](https://github.com/holochain/scaffolding).

## Using the template

1. To scaffold a new project with this template, run this:

`nix run github:<TODO:REPLACE_ME_WITH_THE_APPROPRIATE_GIT_URL>#hc-scaffold-custom-template -- web-app`

2. If you already have an existing project, add the `<TODO:REPLACE_ME_WITH_THE_APPROPRIATE_GIT_URL>` repository as input to your flake, and use it in the packages or your `devShell`:

```diff
{
  description = "Template for Holochain app development";

  inputs = {
    versions.url  = "github:holochain/holochain?dir=versions/weekly";

    holochain-flake.url = "github:holochain/holochain";
    holochain-flake.inputs.versions.follows = "versions";

    nixpkgs.follows = "holochain-flake/nixpkgs";
    flake-parts.follows = "holochain-flake/flake-parts";

+   scaffolding.url = "github:<TODO:REPLACE_ME_WITH_THE_APPROPRIATE_GIT_URL>";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      {
        systems = builtins.attrNames inputs.holochain-flake.devShells;
        perSystem =
          { inputs'
          , config
          , pkgs
          , system
          , ...
          }: {
            devShells.default = pkgs.mkShell {
              inputsFrom = [ inputs'.holochain-flake.devShells.holonix ];
              packages = [
                pkgs.nodejs_20
                # more packages go here
+             ] ++ [
+                inputs'.scaffolding.packages.hc-scaffold-custom-template
              ];
            };
          };
      };
}  
```

---

After this set up, you will be able to `nix develop` from inside your repository, and use the scaffolding tool as normal:

```
hc scaffold dna
hc scaffold zome
...
```

And all the `hc scaffold` commands will already be using this custom template.
