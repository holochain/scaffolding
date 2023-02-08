use build_fs_tree::file;
use dirs::config_dir;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::error::{ScaffoldError, ScaffoldResult};
use crate::file_tree::*;
use crate::versions::holochain_nix_version;

pub fn flake_nix() -> FileTree {
    let holochain_version = holochain_nix_version();
    file!(format!(
        r#"{{
  description = "Template for Holochain app development";

  inputs = {{
    nixpkgs.follows = "holochain/nixpkgs";

    holochain = {{
      url = "github:holochain/holochain/pr_holonix_on_flakes";
      inputs.versions.url = "github:holochain/holochain/pr_holonix_on_flakes?dir=versions/{holochain_version}";
    }};
  }};

  outputs = inputs @ {{ ... }}:
    inputs.holochain.inputs.flake-parts.lib.mkFlake
      {{
        inherit inputs;
      }}
      {{
        systems = builtins.attrNames inputs.holochain.devShells;
        perSystem =
          {{ config
          , pkgs
          , system
          , ...
          }}: {{
            devShells.default = pkgs.mkShell {{
              inputsFrom = [ inputs.holochain.devShells.${{system}}.holonix ];
              packages = [ pkgs.nodejs-18_x ];
            }};
          }};
      }};
}}"#
    ))
}

pub fn setup_nix_developer_environment(dir: &PathBuf) -> ScaffoldResult<()> {
    if cfg!(target_os = "windows") {
        return Err(ScaffoldError::NixSetupError(
            "Windows doesn't support nix".to_string(),
        ));
    }

    println!("Setting up nix development environment...");

    add_extra_experimental_features()?;

    let output = Command::new("nix")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(dir)
        .args(["flake", "update"])
        .output()?;

    if !output.status.success() {
        return Err(ScaffoldError::NixSetupError("".to_string()))?;
    }

    Ok(())
}

const EXTRA_EXPERIMENTAL_FEATURES_LINE: &'static str =
    "extra-experimental-features = flakes nix-command";

pub fn add_extra_experimental_features() -> ScaffoldResult<()> {
    let config_path = config_dir().ok_or(ScaffoldError::NixSetupError(
        "Config dir doesn't exist".to_string(),
    ))?;

    let nix_conf_dir = config_path.join("nix");
    fs::create_dir_all(&nix_conf_dir)?;

    let nix_conf_path = nix_conf_dir.join("nix.conf");
    if let Ok(contents) = fs::read_to_string(&nix_conf_path) {
        if contents.contains(EXTRA_EXPERIMENTAL_FEATURES_LINE) {
            return Ok(());
        }
    }

    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(nix_conf_path)?;

    file.write_all(EXTRA_EXPERIMENTAL_FEATURES_LINE.as_bytes())?;

    Ok(())
}
