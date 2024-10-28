# Custom Template - Holochain Scaffolding Tool

Custom template for the [scaffolding tool](https://github.com/holochain/scaffolding).

## Using the template

1. To scaffold a new project with this template, run this:

```bash
nix run github:<TODO:REPLACE_ME_WITH_CUSTOM_TEMPLATE_GIT_URL>#app -- web-app
```

2. If you already have an existing project, add the `<TODO:REPLACE_ME_WITH_CUSTOM_TEMPLATE_GIT_URL>` repository as input to your flake, and use it in the packages or your `devShell`:

```diff
{
  description = "Flake for Holochain app development";

  inputs = {
    holonix.url = "github:holochain/holonix?ref=main";
    nixpkgs.follows = "holonix/nixpkgs";
    flake-parts.follows = "holonix/flake-parts";

+   scaffolding.url = "github:<TODO:REPLACE_ME_WITH_CUSTOM_TEMPLATE_GIT_URL>";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      {
        systems = builtins.attrNames inputs.holonix.devShells;
        perSystem =
          { inputs'
          , config
          , pkgs
          , system
          , ...
          }: {
            devShells.default = pkgs.mkShell {
              inputsFrom = [ inputs'.holonix.devShells.default ];
              packages = [
                pkgs.nodejs_20
                # more packages go here
+             ] ++ [
+                inputs'.scaffolding.packages.app
              ];
            };
          };
      };
}
```

---

After this set up, you will be able to `nix develop` from inside your repository, and use the scaffolding tool as normal:

```bash
hc scaffold dna
hc scaffold zome
...
```

And all the `hc scaffold` commands will already be using this custom template.

## Running the tests

To run the tests for this custom template, simply run the `run_test.sh` script:

```bash
sh run_test.sh
```
