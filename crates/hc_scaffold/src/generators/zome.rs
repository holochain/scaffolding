use dialoguer::{theme::ColorfulTheme, Confirm};
use regex::Regex;
use std::{ffi::OsString, path::PathBuf};

use crate::{
    file_tree::FileTree,
    versions::{hdi_version, hdk_version},
};
use build_fs_tree::{dir, file};
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

use super::app::cargo::{
    add_workspace_external_dependency, add_workspace_path_dependency, exec_metadata,
    get_workspace_members, get_workspace_packages_locations,
};

pub fn integrity_zome_name(coordinator_zome_name: &String) -> String {
    format!("{}_integrity", coordinator_zome_name)
}

pub fn scaffold_zome_pair(
    app_file_tree: FileTree,
    app_manifest: &AppManifest,
    dna_manifest_path: &PathBuf,
    zome_name: &String,
    path: &Option<PathBuf>,
) -> ScaffoldResult<FileTree> {
    let integrity_zome_name = integrity_zome_name(zome_name);

    let app_file_tree = scaffold_integrity_zome(
        app_file_tree,
        &app_manifest,
        &dna_manifest_path,
        &integrity_zome_name,
        &path,
    )?;

    let app_file_tree = scaffold_coordinator_zome(
        app_file_tree,
        app_manifest,
        &dna_manifest_path,
        zome_name,
        &Some(vec![integrity_zome_name.clone()]),
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
                Ok(Some(p))
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}

/// Tries to guess the location of the integrity zomes
///
/// Procedure:
/// 1. If there is a folder [anything]/zomes/[something with "integrity"], pick it
/// 2. If there is a folder [anything]/[something with "integrity"] pick it
/// 3. If all package paths without the [crate name]/Cargo.toml ending are equal, assume this is the zomes folder
///    and it is not being differentiated between integrity and coordinator zomes in that project
///
fn try_to_guess_integrity_zomes_location(
    app_file_tree: &FileTree,
    dna_name: &String,
) -> ScaffoldResult<Option<PathBuf>> {

    let members = get_workspace_members(app_file_tree)?;

    // if there is a workspace member string containing an expression with the word integrity followed by /*, pick this one
    // and add the right dna name to the path
    let re = Regex::new(r"\A(?P<a>([^/*]*/)*)\*/(?P<b>(zomes/[^/*]*integrity[^/*]*/))\*\z").unwrap();
    for member in members.clone() {
        if re.is_match(member.as_str()) { // if there is a zomes/[something with "integrity"]/* pattern
            let new_path = re.replace(member.as_str(), format!("${{a}}{}/${{b}}", dna_name));
            return Ok(Some(PathBuf::from(new_path.to_string())));
        }
    }

    let re = Regex::new(r"\A(?P<a>([^/*]*/)*)\*/(?P<b>([^/*]*integrity[^/*]*/))\*\z").unwrap();
    for member in members.clone() {
        if re.is_match(member.as_str()) { // if there is a [something with "integrity"]/* pattern
            let new_path = re.replace(member.as_str(), format!("${{a}}{}/${{b}}", dna_name));
            return Ok(Some(PathBuf::from(new_path.to_string())));
        }
    }


    // "[blabla/*/asdas]*/[blabla/*/]*"
    // e.g. "dnas/*/zomes/*" or "subfolder/*/dnas/*/"
    // if members.len() == 1 {
    //     let re = Regex::new(r"\A(?P<a>([^/*]*/)*)\*/(?P<b>([^/*]*/)*)\*\z").unwrap();

    //     if re.is_match(members[0].as_str()) {
    //         let new_path = re.replace(members[0].as_str(), format!("${{a}}{}/${{b}}", dna_name));
    //         return Ok(Some(PathBuf::from(new_path.to_string())));
    //     }

    //     let re = Regex::new(r"\A(?P<a>([^/*]*/)*)\*\z").unwrap();

    //     if re.is_match(members[0].as_str()) {
    //         let new_path = re.replace(members[0].as_str(), r"${a}");
    //         return Ok(Some(PathBuf::from(new_path.to_string())));
    //     }
    // }

    let maybe_packages_paths = get_workspace_packages_locations(&app_file_tree)?;

    match maybe_packages_paths {
        Some(mut packages_paths) if packages_paths.len() != 0 => {
            for p in packages_paths.iter_mut() {
                // Pop the "Cargo.toml" component
                p.pop();
                // Pop crate's folder component
                p.pop();
            }

            if let Some(p) = iter_all_eq(packages_paths) {
                Ok(Some(p))
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}

/// Tries to guess the location of the coordinator zomes
///
/// Procedure:
/// 1. If there is a folder [anything]/zomes/[something with "coordinator"], pick it
/// 2. If there is a folder [anything]/[something with "coordinator"] pick it
/// 3. If all package paths without the [crate name]/Cargo.toml ending are equal, assume this is the zomes folder
///    and it is not being differentiated between integrity and coordinator zomes in that project
///
fn try_to_guess_coordinator_zomes_location(
    app_file_tree: &FileTree,
    dna_name: &String,
) -> ScaffoldResult<Option<PathBuf>> {

    let members = get_workspace_members(app_file_tree)?;

    // if there is a workspace member string containing an expression with the word integrity followed by /*, pick this one
    // and add the right dna name to the path
    let re = Regex::new(r"\A(?P<a>([^/*]*/)*)\*/(?P<b>(zomes/[^/*]*coordinator[^/*]*/))\*\z").unwrap();
    for member in members.clone() {
        if re.is_match(member.as_str()) { // if there is a zomes/[something with "coordinator"]/* pattern
            let new_path = re.replace(member.as_str(), format!("${{a}}{}/${{b}}", dna_name));
            return Ok(Some(PathBuf::from(new_path.to_string())));
        }
    }

    let re = Regex::new(r"\A(?P<a>([^/*]*/)*)\*/(?P<b>([^/*]*coordinator[^/*]*/))\*\z").unwrap();
    for member in members.clone() {
        if re.is_match(member.as_str()) { // if there is a [something with "coordinator"]/* pattern
            let new_path = re.replace(member.as_str(), format!("${{a}}{}/${{b}}", dna_name));
            return Ok(Some(PathBuf::from(new_path.to_string())));
        }
    }

    let maybe_packages_paths = get_workspace_packages_locations(&app_file_tree)?;

    match maybe_packages_paths {
        Some(mut packages_paths) if packages_paths.len() != 0 => {
            for p in packages_paths.iter_mut() {
                // Pop the "Cargo.toml" component
                p.pop();
                // Pop crate's folder component
                p.pop();
            }

            if let Some(p) = iter_all_eq(packages_paths) {
                Ok(Some(p))
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}


pub fn scaffold_integrity_zome_with_path(
    app_file_tree: FileTree,
    app_manifest: &AppManifest,
    dna_manifest_path: &PathBuf,
    zome_name: &String,
    path: &PathBuf,
) -> ScaffoldResult<FileTree> {
    let app_file_tree = add_integrity_zome_to_manifest(
        app_file_tree,
        &app_manifest.app_name().to_string(),
        &dna_manifest_path,
        zome_name,
    )?;

    let app_file_tree =
        add_workspace_external_dependency(app_file_tree, &"hdi".to_string(), &hdi_version())?;
    let app_file_tree =
        add_workspace_external_dependency(app_file_tree, &"serde".to_string(), &"1".to_string())?;
    let mut app_file_tree =
        add_workspace_path_dependency(app_file_tree, zome_name, &path.join(zome_name))?;

    // Add zome to workspace Cargo.toml

    let zome: FileTree = dir! {
        "Cargo.toml" => file!(integrity::initial_cargo_toml(zome_name)),
        "src" => dir! {
            "lib.rs" => file!(integrity::initial_lib_rs())
        }
    };
    let v: Vec<OsString> = path.iter().map(|s| s.to_os_string()).collect();
    app_file_tree
        .path_mut(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(path.clone()))?
        .dir_content_mut()
        .ok_or(ScaffoldError::PathNotFound(path.clone()))?
        .insert(OsString::from(zome_name), zome);

    Ok(app_file_tree)
}

pub fn scaffold_integrity_zome(
    app_file_tree: FileTree,
    app_manifest: &AppManifest,
    dna_manifest_path: &PathBuf,
    zome_name: &String,
    path: &Option<PathBuf>,
) -> ScaffoldResult<FileTree> {
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

            match try_to_guess_integrity_zomes_location(&app_file_tree, &dna_manifest.name())? {
                Some(p) => {
                    if Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt(format!("Scaffold integrity zome in folder {:?}?", p))
                        .interact()?
                    {
                        p
                    } else {
                        choose_directory_path(
                            &String::from("Where should the integrity zome be scaffolded instead?"),
                            &app_file_tree
                        )?
                    }
                }
                None => choose_directory_path(
                    &String::from("Where should the integrity zome be scaffolded?"),
                    &app_file_tree
                )?,
            }
        }
    };

    scaffold_integrity_zome_with_path(
        app_file_tree,
        app_manifest,
        dna_manifest_path,
        zome_name,
        &path_to_scaffold_in,
    )
}

pub fn scaffold_coordinator_zome_in_path(
    app_file_tree: FileTree,
    app_manifest: &AppManifest,
    dna_manifest_path: &PathBuf,
    zome_name: &String,
    dependencies: &Option<Vec<String>>,
    path: &PathBuf,
) -> ScaffoldResult<FileTree> {
    let app_file_tree = add_coordinator_zome_to_manifest(
        app_file_tree,
        &app_manifest.app_name().to_string(),
        &dna_manifest_path,
        zome_name,
        dependencies,
    )?;

    // Add zome to workspace Cargo.toml

    let app_file_tree =
        add_workspace_external_dependency(app_file_tree, &"hdk".to_string(), &hdk_version())?;
    let app_file_tree =
        add_workspace_external_dependency(app_file_tree, &"serde".to_string(), &"1".to_string())?;
    let mut app_file_tree =
        add_workspace_path_dependency(app_file_tree, zome_name, &path.join(zome_name))?;

    let zome: FileTree = dir! {
        "Cargo.toml" => file!(coordinator::initial_cargo_toml(zome_name, dependencies)),
        "src" => dir! {
            "lib.rs" => file!(coordinator::initial_lib_rs())
        }
    };
    let v: Vec<OsString> = path.iter().map(|s| s.to_os_string()).collect();
    app_file_tree
        .path_mut(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(path.clone()))?
        .dir_content_mut()
        .ok_or(ScaffoldError::PathNotFound(path.clone()))?
        .insert(OsString::from(zome_name), zome);

    Ok(app_file_tree)
}

pub fn scaffold_coordinator_zome(
    app_file_tree: FileTree,
    app_manifest: &AppManifest,
    dna_manifest_path: &PathBuf,
    zome_name: &String,
    dependencies: &Option<Vec<String>>,
    path: &Option<PathBuf>,
) -> ScaffoldResult<FileTree> {
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
            match try_to_guess_coordinator_zomes_location(&app_file_tree, &dna_manifest.name())? {
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

    scaffold_coordinator_zome_in_path(
        app_file_tree,
        app_manifest,
        dna_manifest_path,
        zome_name,
        dependencies,
        &path_to_scaffold_in,
    )
}
