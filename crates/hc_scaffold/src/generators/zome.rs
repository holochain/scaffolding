use dialoguer::{theme::ColorfulTheme, Confirm};
use regex::Regex;
use std::{ffi::OsString, path::PathBuf};

use build_fs_tree::{dir, file};
use holochain_scaffolding_utils::FileTree;
use holochain_types::prelude::{AppManifest, DnaManifest};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    utils::choose_directory_path,
};

pub mod coordinator;
pub mod integrity;
pub mod utils;

use coordinator::add_coordinator_zome_to_manifest;
use integrity::add_integrity_zome_to_manifest;

use super::app::cargo::{get_workspace_members, get_workspace_packages_locations};

pub fn integrity_zome_name(coordinator_zome_name: &String) -> String {
    format!("{}_integrity", coordinator_zome_name)
}

pub fn scaffold_zome_pair(
    app_file_tree: FileTree,
    app_manifest: &AppManifest,
    dna_manifest_path: &PathBuf,
    zome_name: &String,
    hdi_version: &String,
    hdk_version: &String,
    path: &Option<PathBuf>,
) -> ScaffoldResult<FileTree> {
    let integrity_zome_name = integrity_zome_name(zome_name);

    let app_file_tree = scaffold_coordinator_zome(
        app_file_tree,
        app_manifest,
        &dna_manifest_path,
        zome_name,
        hdk_version,
        &Some(vec![integrity_zome_name.clone()]),
        &path,
    )?;
    let app_file_tree = scaffold_integrity_zome(
        app_file_tree,
        &app_manifest,
        &dna_manifest_path,
        &integrity_zome_name,
        &hdi_version,
        &path,
    )?;

    Ok(app_file_tree)
}

pub fn iter_all_eq<T: PartialEq>(iter: impl IntoIterator<Item = T>) -> Option<T> {
    let mut iter = iter.into_iter();
    let first = iter.next()?;
    iter.all(|elem| elem == first).then(|| first)
}

fn try_to_guess_zomes_location(
    app_file_tree: &FileTree,
    dna_name: &String,
) -> ScaffoldResult<Option<PathBuf>> {
    let maybe_packages_paths = get_workspace_packages_locations(&app_file_tree)?;

    let members = get_workspace_members(app_file_tree)?;

    if members.len() == 1 {
        let re = Regex::new(r"\A(?P<a>([^/*]*/)*)\*/(?P<b>([^/*]*/)*)\*\z").unwrap();

        if re.is_match(members[0].as_str()) {
            let new_path = re.replace(members[0].as_str(), format!("${{a}}{}/${{b}}", dna_name));
            return Ok(Some(PathBuf::from(new_path.to_string())));
        }

        let re = Regex::new(r"\A(?P<a>([^/*]*/)*)\*\z").unwrap();

        if re.is_match(members[0].as_str()) {
            let new_path = re.replace(members[0].as_str(), r"${a}");
            return Ok(Some(PathBuf::from(new_path.to_string())));
        }
    }

    match maybe_packages_paths {
        Some(mut packages_paths) if packages_paths.len() != 0 => {
            for p in packages_paths.iter_mut() {
                // Pop the "Cargo.toml" component
                p.pop();
                // Pop crate's folder component
                p.pop();
            }

            if let Some(mut p) = iter_all_eq(packages_paths) {
                let current_dir = std::env::current_dir()?;

                p = p
                    .into_iter()
                    .skip(current_dir.components().count())
                    .collect();

                Ok(Some(p))
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}

pub fn scaffold_integrity_zome(
    app_file_tree: FileTree,
    app_manifest: &AppManifest,
    dna_manifest_path: &PathBuf,
    zome_name: &String,
    hdi_version: &String,
    path: &Option<PathBuf>,
) -> ScaffoldResult<FileTree> {
    let mut app_file_tree = add_integrity_zome_to_manifest(
        app_file_tree,
        &app_manifest.app_name().to_string(),
        &dna_manifest_path,
        zome_name,
    )?;

    let zome: FileTree = dir! {
        "Cargo.toml" => file!(integrity::initial_cargo_toml(zome_name, hdi_version)),
        "src" => dir! {
            "lib.rs" => file!(integrity::initial_lib_rs())
        }
    };

    let prompt = String::from("Where should the integrity zome be scaffolded?");

    let path_to_scaffold_in = match path {
        Some(p) => p.clone(),
        None => {
            let v: Vec<OsString> = dna_manifest_path.iter().map(|s| s.to_os_string()).collect();
            let dna_manifest: DnaManifest = serde_yaml::from_str(
                app_file_tree
                    .path(&mut v.iter())
                    .ok_or(ScaffoldError::PathNotFound(dna_manifest_path.clone()))?
                    .file_content()
                    .ok_or(ScaffoldError::PathNotFound(dna_manifest_path.clone()))?,
            )?;
            match try_to_guess_zomes_location(&app_file_tree, &dna_manifest.name())? {
                Some(p) => {
                    if Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt(format!("Scaffold integrity zome in {:?}?", p))
                        .interact()?
                    {
                        p
                    } else {
                        choose_directory_path(&prompt, &app_file_tree)?
                    }
                }
                None => choose_directory_path(&prompt, &app_file_tree)?,
            }
        }
    };

    let v: Vec<OsString> = path_to_scaffold_in
        .iter()
        .map(|s| s.to_os_string())
        .collect();
    app_file_tree
        .path_mut(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(path_to_scaffold_in.clone()))?
        .dir_content_mut()
        .ok_or(ScaffoldError::PathNotFound(path_to_scaffold_in))?
        .insert(OsString::from(zome_name), zome);

    Ok(app_file_tree)
}

pub fn scaffold_coordinator_zome(
    app_file_tree: FileTree,
    app_manifest: &AppManifest,
    dna_manifest_path: &PathBuf,
    zome_name: &String,
    hdk_version: &String,
    dependencies: &Option<Vec<String>>,
    path: &Option<PathBuf>,
) -> ScaffoldResult<FileTree> {
    let mut app_file_tree = add_coordinator_zome_to_manifest(
        app_file_tree,
        &app_manifest.app_name().to_string(),
        &dna_manifest_path,
        zome_name,
        dependencies,
    )?;
    let zome: FileTree = dir! {
        "Cargo.toml" => file!(coordinator::initial_cargo_toml(zome_name, hdk_version)),
        "src" => dir! {
            "lib.rs" => file!(coordinator::initial_lib_rs())
        }
    };

    let prompt = String::from("Where should the coordinator zome be scaffolded?");

    let path_to_scaffold_in = match path {
        Some(p) => p.clone(),
        None => {
            let v: Vec<OsString> = dna_manifest_path.iter().map(|s| s.to_os_string()).collect();
            let dna_manifest: DnaManifest = serde_yaml::from_str(
                app_file_tree
                    .path(&mut v.iter())
                    .ok_or(ScaffoldError::PathNotFound(dna_manifest_path.clone()))?
                    .file_content()
                    .ok_or(ScaffoldError::PathNotFound(dna_manifest_path.clone()))?,
            )?;
            match try_to_guess_zomes_location(&app_file_tree, &dna_manifest.name())? {
                Some(p) => {
                    if Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt(format!("Scaffold coordinator zome in {:?}?", p))
                        .interact()?
                    {
                        p
                    } else {
                        choose_directory_path(&prompt, &app_file_tree)?
                    }
                }
                None => choose_directory_path(&prompt, &app_file_tree)?,
            }
        }
    };

    let v: Vec<OsString> = path_to_scaffold_in
        .iter()
        .map(|s| s.to_os_string())
        .collect();
    app_file_tree
        .path_mut(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(path_to_scaffold_in.clone()))?
        .dir_content_mut()
        .ok_or(ScaffoldError::PathNotFound(path_to_scaffold_in))?
        .insert(OsString::from(zome_name), zome);

    Ok(app_file_tree)
}
