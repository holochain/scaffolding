{
  description = "Flake for Holochain app development";

  inputs = {
    holonix.url = "github:holochain/holonix?ref=main";
    crane.url = "github:ipetkov/crane";

    nixpkgs.follows = "holonix/nixpkgs";
    flake-parts.follows = "holonix/flake-parts";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "holonix/nixpkgs";
    };
  };

  outputs = inputs @ { flake-parts, nixpkgs, crane, rust-overlay, ... }:
    flake-parts.lib.mkFlake { inherit inputs; }
      rec {
        flake = {
          templates.default = {
            path = ./templates/custom-template;
            description = "Custom template for the scaffolding tool";
          };

          lib.wrapCustomTemplate = { system, pkgs, customTemplatePath }:
            let
              scaffolding = inputs.holochain.packages.${system}.hc-scaffold;
            in
            pkgs.runCommand "hc-scaffold"
              {
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
        systems = builtins.attrNames inputs.holonix.devShells;
        perSystem = { inputs', self', config, system, pkgs, lib, ... }: {
          formatter = pkgs.nixpkgs-fmt;

          packages.hc-scaffold =
            let
              pkgs = import nixpkgs {
                inherit system;
                overlays = [ (import rust-overlay) ];
              };
              rustToolchain = pkgs.rust-bin.stable."1.79.0".minimal;
              craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
              crateInfo = craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; };

              # source filtering to ensure builds using include_str! or include_bytes! succeed
              # https://crane.dev/faq/building-with-non-rust-includes.html
              nonCargoBuildFiles = path: _type: builtins.match ".*(gitignore|md|hbs)$" path != null;
              includeFilesFilter = path: type:
                (craneLib.filterCargoSources path type) || (nonCargoBuildFiles path type);
            in
            craneLib.buildPackage {
              pname = "hc-scaffold";
              version = crateInfo.version;
              src = lib.cleanSourceWith {
                src = ./.;
                filter = includeFilesFilter;
                name = "source";
              };
              doCheck = false;

              buildInputs = [ pkgs.openssl pkgs.go ]
                ++ (lib.optionals pkgs.stdenv.isDarwin
                (with pkgs.darwin.apple_sdk.frameworks; [
                  CoreFoundation
                  SystemConfiguration
                  Security
                ]));

              nativeBuildInputs = [ pkgs.perl ];
            };

          checks.custom-template = flake.lib.wrapCustomTemplate {
            inherit pkgs system;
            customTemplatePath = ./templates/custom-template/custom-template;
          };

          devShells.default = pkgs.mkShell {
            inputsFrom = [ inputs'.holonix.devShells ];

            packages = (with inputs'.holonix.packages; [
              holochain
              lair-keystore
              hc-launch
              hn-introspect
              rust
            ]) ++ (with pkgs; [
              nodejs_20
              binaryen
            ]) ++ [
              self'.packages.hc-scaffold
            ];

            shellHook = ''
              export PS1='\[\033[1;34m\][holonix:\w]\$\[\033[0m\] '
            '';
          };

          devShells.ci = pkgs.mkShell {
            inputsFrom = [ self'.devShells.default ];
            packages = [
              self'.packages.hc-scaffold
            ];
          };
        };
      };
}
