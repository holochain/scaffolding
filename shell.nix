let
  holonixRev = "d15633710a8d4349dc0ff03b7b47ad01eb9f2433";

  holonixPath = builtins.fetchTarball "https://github.com/holochain/holonix/archive/${holonixRev}.tar.gz";
  holonix = import (holonixPath) {};
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  packages = with nixpkgs; [
    # Additional packages go here
    nodejs-16_x
  ];
}
