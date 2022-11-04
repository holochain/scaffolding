use build_fs_tree::{dir, file};
use std::{ffi::OsString, path::PathBuf};

use crate::error::ScaffoldResult;
use crate::templates::template_path;
use crate::templates::web_app::scaffold_web_app_template;
use crate::{error::ScaffoldError, file_tree::FileTree};

use self::uis::{choose_ui_framework, UiFramework};

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
    template_file_tree: FileTree,
) -> ScaffoldResult<FileTree> {
    let mut app_file_tree = dir! {
      ".gitignore" => file!(gitignore())
      "workdir" => dir!{
        "happ.yaml" => file!(empty_happ_manifest(app_name.clone(), description)?)
        "web-happ.yaml" => file!(web_happ_manifest(app_name.clone(), format!("./{}.happ", app_name), String::from("../ui/dist.zip"))?)
      }
      "package.json" => file!(workspace_package_json(&app_name, &String::from("ui"), &String::from("workdir"), &String::from("workdir")))
      "tests" => dir!{
        "package.json" => file!(tryorama_package_json())
        "tsconfig.json" => file!(tryorama_tsconfig())
        "src" => dir! {}
      }
      "Cargo.toml" => file!(workspace_cargo_toml())
      "dnas" => dir! {},
    };

    if !skip_nix {
        app_file_tree
            .dir_content_mut()
            .ok_or(ScaffoldError::PathNotFound(PathBuf::new()))?
            .insert(OsString::from("default.nix"), default_nix());
    }
    app_file_tree
        .dir_content_mut()
        .ok_or(ScaffoldError::PathNotFound(PathBuf::new()))?
        .insert(OsString::from(template_path()), template_file_tree.clone());

    let mut app_file_tree =
        scaffold_web_app_template(app_file_tree, &template_file_tree, &app_name)?;

    app_file_tree
        .dir_content_mut()
        .unwrap()
        .insert(OsString::from("dnas"), dir! {});

    Ok(app_file_tree)
}

pub fn scaffold_web_app(
    app_name: String,
    description: Option<String>,
    skip_nix: bool,
    template_file_tree: FileTree,
) -> ScaffoldResult<FileTree> {
    Ok(dir! {
      app_name.clone() => web_app_skeleton(app_name, description, skip_nix, template_file_tree)?
    })
}
