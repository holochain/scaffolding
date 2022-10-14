use std::{ffi::OsString, path::PathBuf};

use cargo_metadata::{Metadata, MetadataCommand};
use holochain_scaffolding_utils::FileTree;

use crate::error::{ScaffoldError, ScaffoldResult};

pub fn workspace_cargo_toml() -> String {
    format!(
        r#"[workspace]
resolver = "2"
members = [
  "dnas/*/zomes/*",
]

[profile.dev]
opt-level = "z"

[profile.release]
opt-level = "z"
"#
    )
}

fn workspace_cargo_toml_path(_app_file_tree: &FileTree) -> PathBuf {
    PathBuf::new().join("Cargo.toml")
}

pub fn get_workspace_packages_locations(
    app_file_tree: &FileTree,
) -> ScaffoldResult<Option<Vec<PathBuf>>> {
    let path = std::env::current_dir()?
        .join(workspace_cargo_toml_path(&app_file_tree))
        .canonicalize()?;
    let command_result = MetadataCommand::new().manifest_path(path).exec();

    match command_result {
        Ok(metadata) => {
            let packages_paths: Vec<PathBuf> = metadata
                .workspace_packages()
                .into_iter()
                .map(|p| PathBuf::from(p.manifest_path.as_std_path()))
                .collect();
            Ok(Some(packages_paths))
        }
        Err(_) => Ok(None),
    }
}

pub fn get_workspace_members(app_file_tree: &FileTree) -> ScaffoldResult<Vec<String>> {
    let cargo_toml = get_worspace_cargo_toml(&app_file_tree)?;

    let members: Vec<String> = cargo_toml
        .as_table()
        .ok_or(ScaffoldError::MalformedFile(
            workspace_cargo_toml_path(&app_file_tree),
            String::from("file does not conform to toml"),
        ))?
        .get("workspace")
        .ok_or(ScaffoldError::MalformedFile(
            workspace_cargo_toml_path(&app_file_tree),
            String::from("should have a workspace table"),
        ))?
        .as_table()
        .ok_or(ScaffoldError::MalformedFile(
            workspace_cargo_toml_path(&app_file_tree),
            String::from("should have a workspace table"),
        ))?
        .get("members")
        .ok_or(ScaffoldError::MalformedFile(
            workspace_cargo_toml_path(&app_file_tree),
            String::from("should have a members field in the workspace table"),
        ))?
        .as_array()
        .ok_or(ScaffoldError::MalformedFile(
            workspace_cargo_toml_path(&app_file_tree),
            String::from("the members field in the workspace table should be an array"),
        ))?
        .into_iter()
        .filter_map(|s| s.as_str())
        .map(|s| s.to_string())
        .collect();

    Ok(members)
}

pub fn get_worspace_cargo_toml(app_file_tree: &FileTree) -> ScaffoldResult<toml::Value> {
    let path = workspace_cargo_toml_path(app_file_tree);

    let v: Vec<OsString> = path.iter().map(|a| a.to_os_string()).collect();
    let cargo_toml_str = app_file_tree
        .path(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(path.clone()))?
        .file_content()
        .ok_or(ScaffoldError::PathNotFound(path.clone()))?
        .clone();

    let v = toml::from_str(cargo_toml_str.as_str())?;

    Ok(v)
}
