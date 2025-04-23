{
  description = "Flake for Holochain app development";

  inputs = {
    holonix = {
      url = "github:holochain/holonix?ref=main-0.5";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-parts.follows = "flake-parts";
        crane.follows = "crane";
        rust-overlay.follows = "rust-overlay";
      };
    };

    nixpkgs.url = "github:nixos/nixpkgs?ref=24.11";

    flake-parts.url = "github:hercules-ci/flake-parts";

    crane.url = "github:ipetkov/crane";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ { flake-parts, nixpkgs, crane, rust-overlay, ... }:
    flake-parts.lib.mkFlake { inherit inputs; }
      {
        flake = {
          lib.wrapCustomTemplate = { system, pkgs, customTemplatePath }:
            let
              scaffolding = inputs.holonix.packages.${system}.hc-scaffold;
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
              rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
              craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
              crateInfo = craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; };

              # source filtering to ensure builds using include_str! or include_bytes! succeed
              # https://crane.dev/faq/building-with-non-rust-includes.html
              nonCargoBuildFiles = path: _type: builtins.match ".*(gitignore|md|hbs|nix|sh)$" path != null;
              includeFilesFilter = path: type:
                (craneLib.filterCargoSources path type) || (nonCargoBuildFiles path type);

              buildInputs = [ pkgs.openssl pkgs.go ]
                ++ (lib.optionals pkgs.stdenv.isDarwin
                (with pkgs.darwin.apple_sdk.frameworks; [
                  CoreFoundation
                  SystemConfiguration
                  Security
                ]));

              nativeBuildInputs = [ pkgs.perl ];

              cargoArtifacts = craneLib.buildDepsOnly {
                pname = "hc-scaffold-deps";
                src = lib.cleanSourceWith {
                  src = ./.;
                  filter = includeFilesFilter;
                  name = "source";
                };
                inherit buildInputs nativeBuildInputs;
              };
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

              inherit cargoArtifacts buildInputs nativeBuildInputs;
            };

          devShells.default = pkgs.mkShell {
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
            packages = [ inputs'.holonix.packages.rust self'.packages.hc-scaffold ];
          };
        };
      };


  nixConfig = {
    extra-substituters = [ "https://holochain-ci.cachix.org" "https://holochain-scaffolding-cli.cachix.org" ];
    extra-trusted-public-keys = [ "holochain-ci.cachix.org-1:5IUSkZc0aoRS53rfkvH9Kid40NpyjwCMCzwRTXy+QN8=" "holochain-scaffolding-cli.cachix.org-1:DznpFcPqqPqiP05k/0lT6qeQ/N3KOvKQW/EP4Pf3A2E=" ];
  };
}
