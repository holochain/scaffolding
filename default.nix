let
  holonixPath = (import ./nix/sources.nix).holonix; # points to the current state of the Holochain repository
  holonix = import (holonixPath) {
     # holochainVersionId = "v0_0_124"; specifies the Holochain version
    rustVersion = {
      track = "stable";
      version = "1.64.0";
    };
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  packages = with nixpkgs; [
    niv
    # any additional packages needed for this project, e. g. Nodejs
  ];
}
