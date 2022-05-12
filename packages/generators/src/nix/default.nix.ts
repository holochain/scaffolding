import { ScFile, ScNodeType } from '@source-craft/types';

export const defaultNix = (holochainVersion: string): ScFile => ({
  type: ScNodeType.File,
  content: `let
  holonixPath = (import ./nix/sources.nix).holonix; # points to the current state of the Holochain repository
  holonix = import (holonixPath) {
    holochainVersionId = "${holochainVersion}"; # specifies the Holochain version
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  packages = with nixpkgs; [
    niv
    # any additional packages needed for this project, e. g. Nodejs
  ];
}
`,
});
