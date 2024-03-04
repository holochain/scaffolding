{
  description = "Template for Holochain app development";

  inputs = {
    nixpkgs.follows = "holochain/nixpkgs";
    versions.url = "github:holochain/holochain?dir=versions/weekly";

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
    ({ withSystem, flake-parts-lib, ...}: {
      flake = {
        templates.default = {
          path = ./templates/custom-template;
          description  = "Custom template for the scaffolding tool";
        };
      
        lib.wrapCustomTemplate = { system, pkgs, customTemplatePath }: 
          let 
        	  scaffolding = withSystem system ({config, ...}: inputs.holochain.${system}.packages.hc-scaffold);
        	in 
        		pkgs.runCommand "hc-scaffold" {
        	    buildInputs = [ pkgs.makeWrapper ];
        	    src = customTemplatePath;
        	  } ''
        	    mkdir $out
        	    mkdir $out/bin
        	    # We create the bin folder ourselves and link every binary in it
        	    ln -s ${scaffolding}/bin/* $out/bin
        	    # Except the hello binary
        	    rm $out/bin/hc-scaffold
        	    cp $src -R $out/template
        	    # Because we create this ourself, by creating a wrapper
        	    makeWrapper ${scaffolding}/bin/hc-scaffold $out/bin/hc-scaffold \
        	      --add-flags "--template $out/template"
        	  '';
      };
    
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

        # TODO: Expose the scaffolding tool CLI as the main package for this crate
        # packages.default = inputs'.holochain.packages.hc-scaffold;
      };
    });
}
