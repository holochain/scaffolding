import { ScFile, ScNodeType } from '@source-craft/types';

export const defaultNix = (holonixRev: string, holochainVersion: string): ScFile => ({
  type: ScNodeType.File,
  content: `let
  holonixRev = "${holonixRev}";

  holonixPath = builtins.fetchTarball "https://github.com/holochain/holonix/archive/\${holonixRev}.tar.gz";
  holonix = import (holonixPath) {
    holochainVersionId = "${holochainVersion}";
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  packages = with nixpkgs; [
    # Additional packages go here
    nodejs-16_x
  ];
}
`,
});
