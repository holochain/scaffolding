use build_fs_tree::file;
use dirs::home_dir;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::error::{ScaffoldError, ScaffoldResult};
use crate::file_tree::*;
use crate::versions;

pub fn flake_nix(holo_enabled: bool) -> FileTree {
    let holochain_nix_version = versions::HOLOCHAIN_NIX_VERSION;

    let holo_inputs = holo_enabled
        .then_some(
            r#"
    hds-releases.url = "github:holo-host/hds-releases";
    "#,
        )
        .unwrap_or_default();

    let holo_packages = holo_enabled
        .then_some("inputs'.hds-releases.packages.holo-dev-server-bin")
        .unwrap_or_default();

    file!(format!(
        r#"{{
  description = "Template for Holochain app development";

  inputs = {{
    versions.url  = "github:holochain/holochain?dir=versions/{holochain_nix_version}";

    holochain-flake.url = "github:holochain/holochain";
    holochain-flake.inputs.versions.follows = "versions";

    nixpkgs.follows = "holochain-flake/nixpkgs";
    flake-parts.follows = "holochain-flake/flake-parts";
    {holo_inputs}
  }};

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake
      {{
        inherit inputs;
      }}
      {{
        systems = builtins.attrNames inputs.holochain-flake.devShells;
        perSystem =
          {{ inputs'
          , config
          , pkgs
          , system
          , ...
          }}: {{
            devShells.default = pkgs.mkShell {{
              inputsFrom = [ inputs'.holochain-flake.devShells.holonix ];
              packages = [
                pkgs.nodejs_20
                {holo_packages}
                # more packages go here
              ];
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

    // This is here to catch the issue from this thread https://discourse.nixos.org/t/nix-flakes-nix-store-source-no-such-file-or-directory/17836
    // If you run Scaffolding inside a Git repository when the `nix flake update` will fail. At some point Nix should report this so we don't need
    // to worry about it but for now this helps solve a strange error message.
    match Command::new("git")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .current_dir(dir)
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
    {
        Ok(output) => {
            if output.status.success() && output.stdout == b"true\n" {
                return Err(ScaffoldError::NixSetupError("- detected that Scaffolding is running inside an existing Git repository, please choose a different location to scaffold".to_string()));
            }
        }
        Err(_) => {} // Ignore errors, Git isn't necessarily available.
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
    let config_path = home_dir().ok_or(ScaffoldError::NixSetupError(
        "Config dir doesn't exist".to_string(),
    ))?;

    let nix_conf_dir = config_path.join(".config").join("nix");
    fs::create_dir_all(&nix_conf_dir)?;

    let nix_conf_path = nix_conf_dir.join("nix.conf");
    if let Ok(contents) = fs::read_to_string(&nix_conf_path) {
        if contents.contains(EXTRA_EXPERIMENTAL_FEATURES_LINE) {
            return Ok(());
        }
    }

    if let Ok(mut file) = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(nix_conf_path)
    {
        file.write_all(EXTRA_EXPERIMENTAL_FEATURES_LINE.as_bytes())?;
    } else {
        println!("Warning: could not write extra-experimental-features to nix.conf");
    }
    Ok(())
}
