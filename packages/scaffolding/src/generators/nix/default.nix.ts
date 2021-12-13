export default (holonixRev: string) =>
  `{
  holonixPath ?  builtins.fetchTarball { url = "https://github.com/holochain/holonix/archive/${holonixRev}.tar.gz"; }
}:

let
  holonix = import (holonixPath) { };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  buildInputs = with nixpkgs; [
    binaryen
    nodejs-16_x
  ];
}  
`;
