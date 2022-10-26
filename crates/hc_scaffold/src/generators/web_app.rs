use build_fs_tree::{dir, file};
use std::{ffi::OsString, path::PathBuf};

use crate::error::ScaffoldResult;
use crate::{error::ScaffoldError, file_tree::FileTree};

use self::uis::{choose_ui_framework, scaffold_web_app_ui, UiFramework};

use super::{
    app::{
        cargo::workspace_cargo_toml,
        default_nix::default_nix,
        gitignore::gitignore,
        manifests::{empty_happ_manifest, web_happ_manifest},
    },
    tryorama::{package_json::tryorama_package_json, tsconfig_json::tryorama_tsconfig},
};

mod package_json;
pub mod uis;

use package_json::workspace_package_json;

fn web_app_skeleton(
    app_name: String,
    description: Option<String>,
    skip_nix: bool,
    ui_framework: &UiFramework,
) -> ScaffoldResult<FileTree> {
    let mut app_file_tree = dir! {
      ".gitignore" => file!(gitignore())
      "workdir" => dir!{
        "happ.yaml" => file!(empty_happ_manifest(app_name.clone(), description)?)
        "web-happ.yaml" => file!(web_happ_manifest(app_name.clone(), format!("./{}.happ", app_name), String::from("./ui/dist.zip"))?)
      }
      "ui" => scaffold_web_app_ui(ui_framework, &app_name)?
      "package.json" => file!(workspace_package_json(app_name, String::from("ui"), String::from("workdir"), String::from("workdir")))
      "tests" => dir!{
        "package.json" => file!(tryorama_package_json())
        "tsconfig.json" => file!(tryorama_tsconfig())
        "src" => dir! {}
      }
      "Cargo.toml" => file!(workspace_cargo_toml())
      "dnas" => dir! {}
    };

    if !skip_nix {
        app_file_tree
            .dir_content_mut()
            .ok_or(ScaffoldError::PathNotFound(PathBuf::new()))?
            .insert(OsString::from("default.nix"), default_nix());
    }

    Ok(app_file_tree)
}

pub fn scaffold_web_app(
    app_name: String,
    description: Option<String>,
    skip_nix: bool,
    ui_framework: &Option<UiFramework>,
) -> ScaffoldResult<FileTree> {
    let ui = match ui_framework {
        Some(ui) => ui.clone(),
        None => choose_ui_framework()?,
    };

    Ok(dir! {
      app_name.clone() => web_app_skeleton(app_name, description, skip_nix, &ui)?
    })
}
