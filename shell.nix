let
  sources = import ./nix/sources.nix;
  holonix = import sources.holonix {};
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  packages = with nixpkgs; [
    # Additional packages go here
    nodejs-16_x

    niv
  ];
}
