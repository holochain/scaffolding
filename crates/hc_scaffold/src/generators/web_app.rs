use std::ffi::OsString;

use build_fs_tree::{dir, file, Build, MergeableFileSystemTree};
use holochain_scaffolding_utils::FileTree;

use crate::error::ScaffoldResult;

use super::{
    app::{
        default_nix::default_nix,
        cargo::workspace_cargo_toml,
        manifests::{empty_happ_manifest, web_happ_manifest},
    },
    tryorama::package_json::tryorama_package_json,
};

mod package_json;
use package_json::workspace_package_json;

fn web_app_skeleton(app_name: String, description: Option<String>) -> ScaffoldResult<FileTree> {
    Ok(dir! {
      "default.nix" => default_nix("main".into())
      "workdir" => dir!{
        "happ.yaml" => file!(empty_happ_manifest(app_name.clone(), description)?)
        "web-happ.yaml" => file!(web_happ_manifest(app_name.clone(), format!("./{}.happ", app_name), String::from("./ui/dist.zip"))?)
      }
      "ui" => dir! {}
      "package.json" => file!(workspace_package_json(app_name, String::from("ui"), String::from("workdir"), String::from("workdir")))
      "tests" => dir!{
        "package.json" => file!(tryorama_package_json(String::from("^0.9.0")))
        "src" => dir! {

        }
      }
      "Cargo.toml" => file!(workspace_cargo_toml())
      "dnas" => dir! {}
    })
}

pub fn scaffold_web_app(app_name: String, description: Option<String>) -> anyhow::Result<()> {
    let file_tree = MergeableFileSystemTree::<OsString, String>::from(dir! {
      app_name.clone() => web_app_skeleton(app_name, description)?
    });

    file_tree.build(&".".into())?;

    Ok(())
}
