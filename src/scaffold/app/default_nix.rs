use build_fs_tree::file;

use crate::file_tree::*;
use crate::versions::holochain_nix_version;

pub fn default_nix() -> FileTree {
    file!(format!(
        r#"let
  holonixPath = (import ./nix/sources.nix).holonix; # points to the current state of the Holochain repository
  holonix = import (holonixPath) {{
    holochainVersionId = "{}"; # specifies the Holochain version
  }};
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {{
  inputsFrom = [ holonix.main ];
  packages = with nixpkgs; [
    niv
    nodejs-18_x
    # any additional packages needed for this project, e. g. Nodejs
  ];

}}"#,
        holochain_nix_version()
    ))
}
