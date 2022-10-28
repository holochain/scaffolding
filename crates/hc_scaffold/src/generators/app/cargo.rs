use std::{ffi::OsString, path::PathBuf, process::Stdio, str::from_utf8};

use crate::file_tree::FileTree;
use cargo_metadata::{Metadata, MetadataCommand};

use crate::error::{ScaffoldError, ScaffoldResult};

pub fn workspace_cargo_toml() -> String {
    format!(
        r#"[workspace]
members = [
  "dnas/*/zomes/coordinator/*",
  "dnas/*/zomes/integrity/*",
]

[profile.dev]
opt-level = "z"

[profile.release]
opt-level = "z"
"#
    )
}

pub fn add_workspace_external_dependency(
    app_file_tree: FileTree,
    crate_name: &String,
    crate_version: &String,
) -> ScaffoldResult<FileTree> {
    add_workspace_dependency(
        app_file_tree,
        crate_name,
        &toml::Value::String(crate_version.clone()),
    )
}

pub fn add_workspace_path_dependency(
    app_file_tree: FileTree,
    crate_name: &String,
    path_from_workspace_root: &PathBuf,
) -> ScaffoldResult<FileTree> {
    let mut table = toml::map::Map::new();
    table.insert(
        "path".to_string(),
        toml::Value::String(
            path_from_workspace_root
                .as_os_str()
                .to_os_string()
                .to_str()
                .unwrap()
                .to_string(),
        ),
    );
    add_workspace_dependency(app_file_tree, crate_name, &toml::Value::Table(table))
}

fn add_workspace_dependency(
    mut app_file_tree: FileTree,
    crate_name: &String,
    crate_location: &toml::Value,
) -> ScaffoldResult<FileTree> {
    let mut workspace_cargo_toml = get_workspace_cargo_toml(&app_file_tree)?;
    let workspace_table = workspace_cargo_toml
        .as_table_mut()
        .ok_or(ScaffoldError::MalformedFile(
            workspace_cargo_toml_path(&app_file_tree),
            String::from("file does not conform to toml"),
        ))?
        .get_mut("workspace")
        .ok_or(ScaffoldError::MalformedFile(
            workspace_cargo_toml_path(&app_file_tree),
            String::from("no workspace table found in workspace root"),
        ))?
        .as_table_mut()
        .ok_or(ScaffoldError::MalformedFile(
            workspace_cargo_toml_path(&app_file_tree),
            String::from("workspace key is not a table"),
        ))?;

    let mut dependencies = match workspace_table.get("dependencies") {
        Some(d) => d
            .as_table()
            .ok_or(ScaffoldError::MalformedFile(
                workspace_cargo_toml_path(&app_file_tree),
                String::from("workspace.dependencies is not a table"),
            ))?
            .clone(),
        None => toml::map::Map::new(),
    };

    dependencies.insert(crate_name.clone(), crate_location.clone());
    workspace_table.insert(
        String::from("dependencies"),
        toml::Value::Table(dependencies),
    );

    let path = workspace_cargo_toml_path(&app_file_tree);

    let cargo_toml_str = toml::to_string(&workspace_cargo_toml)?;

    let v: Vec<OsString> = path.iter().map(|a| a.to_os_string()).collect();
    *app_file_tree
        .path_mut(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(path.clone()))?
        .file_content_mut()
        .unwrap() = cargo_toml_str;

    Ok(app_file_tree)
}

fn workspace_cargo_toml_path(_app_file_tree: &FileTree) -> PathBuf {
    PathBuf::new().join("Cargo.toml")
}

pub fn get_workspace_packages_locations(
    app_file_tree: &FileTree,
) -> ScaffoldResult<Option<Vec<PathBuf>>> {
    let current_dir = std::env::current_dir()?;

    let path = current_dir
        .join(workspace_cargo_toml_path(&app_file_tree))
        .canonicalize()?;
    let command_result = MetadataCommand::new().manifest_path(path).exec();

    match command_result {
        Ok(metadata) => {
            let packages_paths: Vec<PathBuf> = metadata
                .workspace_packages()
                .into_iter()
                .map(|p| {
                    PathBuf::from(p.manifest_path.as_std_path())
                        .into_iter()
                        .skip(current_dir.components().count())
                        .collect()
                })
                .collect();
            Ok(Some(packages_paths))
        }
        Err(_) => Ok(None),
    }
}

pub fn workspace_package_path(
    app_file_tree: &FileTree,
    crate_name: &String,
) -> ScaffoldResult<Option<PathBuf>> {
    let current_dir = std::env::current_dir()?;

    let path = current_dir
        .join(workspace_cargo_toml_path(&app_file_tree))
        .canonicalize()?;
    let metadata = MetadataCommand::new().manifest_path(path).exec()?;

    let package_path: Option<PathBuf> = metadata
        .workspace_packages()
        .into_iter()
        .find(|p| p.name.eq(crate_name))
        .map(|p| {
            PathBuf::from(p.manifest_path.as_std_path())
                .into_iter()
                .skip(current_dir.components().count())
                .collect()
        });
    Ok(package_path)
}

pub fn get_workspace_members(app_file_tree: &FileTree) -> ScaffoldResult<Vec<String>> {
    let cargo_toml = get_workspace_cargo_toml(&app_file_tree)?;

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

pub fn get_workspace_cargo_toml(app_file_tree: &FileTree) -> ScaffoldResult<toml::Value> {
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

pub fn exec_metadata(app_file_tree: &FileTree) -> Result<Metadata, cargo_metadata::Error> {
    let current_dir = std::env::current_dir()?;
    let path = current_dir
        .join(workspace_cargo_toml_path(&app_file_tree))
        .canonicalize()?;
    let output = MetadataCommand::new()
        .manifest_path(path)
        .cargo_command()
        .stderr(Stdio::inherit())
        .output()?;
    if !output.status.success() {
        return Err(cargo_metadata::Error::CargoMetadata {
            stderr: String::from_utf8(output.stderr)?,
        });
    }
    let stdout = from_utf8(&output.stdout)?
        .lines()
        .find(|line| line.starts_with('{'))
        .ok_or(cargo_metadata::Error::NoJson)?;
    MetadataCommand::parse(stdout)
}
