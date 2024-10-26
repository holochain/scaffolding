use convert_case::Case;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use mr_bundle::Location;
use regex::Regex;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

use crate::{
    file_tree::{build_file_tree, file_exists, insert_file_tree_in_dir, FileTree},
    reserved_words::check_for_reserved_keywords,
    templates::{
        coordinator::scaffold_coordinator_zome_templates,
        integrity::scaffold_integrity_zome_templates, ScaffoldedTemplate,
    },
    utils::{input_with_case, unparse_pretty},
    versions,
};
use build_fs_tree::{dir, file};
use holochain_types::prelude::{DnaManifest, ZomeManifest};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    utils::choose_directory_path,
};

pub mod coordinator;
pub mod integrity;
pub mod utils;

use super::{
    app::cargo::{
        add_workspace_external_dependency, add_workspace_path_dependency, get_workspace_members,
        get_workspace_packages_locations, workspace_package_path,
    },
    dna::{
        coordinator::{add_coordinator_zome_to_manifest, new_coordinator_zome_manifest},
        integrity::{add_integrity_zome_to_manifest, new_integrity_zome_manifest},
        DnaFileTree,
    },
};

pub struct ZomeFileTree {
    pub dna_file_tree: DnaFileTree,
    pub zome_manifest: ZomeManifest,
    pub zome_crate_path: PathBuf,
}

impl ZomeFileTree {
    pub fn get_or_choose_integrity(
        dna_file_tree: DnaFileTree,
        integrity_zome_name: Option<&str>,
    ) -> ScaffoldResult<ZomeFileTree> {
        let integrity_zomes = match dna_file_tree.dna_manifest.clone() {
            DnaManifest::V1(v1) => v1.integrity.zomes.clone(),
        };

        let zome_manifest = match (integrity_zomes.len(), integrity_zome_name) {
            (0, None) => Err(ScaffoldError::NoIntegrityZomesFound(
                dna_file_tree.dna_manifest.name(),
            )),
            (1, None) => {
                integrity_zomes
                    .into_iter()
                    .last()
                    .ok_or(ScaffoldError::NoIntegrityZomesFound(
                        dna_file_tree.dna_manifest.name(),
                    ))
            }
            (_, None) => {
                choose_integrity_zome(&dna_file_tree.dna_manifest.name(), &integrity_zomes)
            }
            (_, Some(name)) => integrity_zomes
                .into_iter()
                .find(|zome| zome.name.0.to_string().eq(name))
                .ok_or(ScaffoldError::IntegrityZomeNotFound(
                    name.to_owned(),
                    dna_file_tree.dna_manifest.name(),
                )),
        }?;
        ZomeFileTree::from_zome_manifest(dna_file_tree, zome_manifest)
    }

    pub fn from_zome_manifest(
        dna_file_tree: DnaFileTree,
        zome_manifest: ZomeManifest,
    ) -> ScaffoldResult<ZomeFileTree> {
        let zome_crate_path = zome_crate_path(&dna_file_tree, &zome_manifest)?;

        let lib_rs_path = zome_crate_path.join("src").join("lib.rs");

        if !file_exists(dna_file_tree.file_tree_ref(), &lib_rs_path) {
            return Err(ScaffoldError::PathNotFound(lib_rs_path.clone()));
        }

        Ok(ZomeFileTree {
            dna_file_tree,
            zome_manifest,
            zome_crate_path,
        })
    }
}

fn zome_crate_path(
    dna_file_tree: &DnaFileTree,
    zome_manifest: &ZomeManifest,
) -> ScaffoldResult<PathBuf> {
    match zome_manifest.location.clone() {
        Location::Bundled(bundled_path) => {
            let file_name_os_str = bundled_path.file_name().unwrap();
            let file_name = file_name_os_str
                .to_os_string()
                .to_str()
                .unwrap()
                .to_string();

            let crate_name = file_name.split(".wasm").next().unwrap().to_string();

            let mut manifest_path =
                workspace_package_path(dna_file_tree.file_tree_ref(), &crate_name)?.ok_or(
                    ScaffoldError::IntegrityZomeNotFound(
                        zome_manifest.name.0.to_string(),
                        dna_file_tree.dna_manifest.name(),
                    ),
                )?;

            manifest_path.pop();

            Ok(manifest_path)
        }
        _ => Err(ScaffoldError::IntegrityZomeNotFound(
            zome_manifest.name.0.to_string(),
            dna_file_tree.dna_manifest.name(),
        )),
    }
}

pub fn integrity_zome_name(coordinator_zome_name: &str) -> String {
    format!("{}_integrity", coordinator_zome_name)
}

pub fn iter_all_eq<T: PartialEq>(iter: impl IntoIterator<Item = T>) -> Option<T> {
    let mut iter = iter.into_iter();
    let first = iter.next()?;
    iter.all(|elem| elem == first).then_some(first)
}

fn choose_integrity_zome(
    dna_name: &str,
    integrity_zomes: &[ZomeManifest],
) -> ScaffoldResult<ZomeManifest> {
    let integrity_zome_names: Vec<String> = integrity_zomes
        .iter()
        .map(|z| z.name.0.to_string())
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Multiple integrity zomes were found in DNA {}, choose one:",
            dna_name
        ))
        .default(0)
        .items(&integrity_zome_names[..])
        .interact()?;

    Ok(integrity_zomes[selection].clone())
}

/// Tries to guess the location of the integrity zomes
///
/// Procedure:
/// 1. If there is a workspace member string with [anything]*/zomes/[something with "integrity"], pick it and replace the star with the dna name
/// 2. If there is a workspace member string with [anything]*/[something with "integrity"], pick it and replace the star with the dna name
/// 3. If there is only one workspace member string and it contains a * somewhere in the middle, replace the * with the dna name
/// 4. If there is only one workspace member string and it only contains a * at the end, take that folder (without the *)
/// 5. If all package paths without the [crate name]/Cargo.toml ending are equal, assume this is the zomes folder
///    and it is not being differentiated between integrity and coordinator zomes in that project
///
fn try_to_guess_integrity_zomes_location(
    dna_file_tree: &DnaFileTree,
) -> ScaffoldResult<Option<PathBuf>> {
    let members = get_workspace_members(dna_file_tree.file_tree_ref())?;

    // if there is a workspace member string containing an expression with the word integrity followed by /*, pick this one
    // and add the right dna name to the path
    let re =
        Regex::new(r"\A(?P<a>([^/*]*/)*)\*/(?P<b>(zomes/[^/*]*integrity[^/*]*/))\*\z").unwrap();
    for member in members.clone() {
        if re.is_match(member.as_str()) {
            // if there is a zomes/[something with "integrity"]/* pattern
            let new_path = re.replace(
                member.as_str(),
                format!("${{a}}{}/${{b}}", dna_file_tree.dna_manifest.name()),
            );
            return Ok(Some(PathBuf::from(new_path.to_string())));
        }
    }

    let re = Regex::new(r"\A(?P<a>([^/*]*/)*)\*/(?P<b>([^/*]*integrity[^/*]*/))\*\z").unwrap();
    for member in members.clone() {
        if re.is_match(member.as_str()) {
            // if there is a [something with "integrity"]/* pattern
            let new_path = re.replace(
                member.as_str(),
                format!("${{a}}{}/${{b}}", dna_file_tree.dna_manifest.name()),
            );
            return Ok(Some(PathBuf::from(new_path.to_string())));
        }
    }

    if members.len() == 1 {
        let re = Regex::new(r"\A(?P<a>([^/*]*/)*)\*/(?P<b>([^/*]*/)*)\*\z").unwrap();

        if re.is_match(members[0].as_str()) {
            let new_path = re.replace(
                members[0].as_str(),
                format!("${{a}}{}/${{b}}", dna_file_tree.dna_manifest.name()),
            );
            return Ok(Some(PathBuf::from(new_path.to_string())));
        }

        let re = Regex::new(r"\A(?P<a>([^/*]*/)*)\*\z").unwrap();

        if re.is_match(members[0].as_str()) {
            let new_path = re.replace(members[0].as_str(), r"${a}");
            return Ok(Some(PathBuf::from(new_path.to_string())));
        }
    }

    let maybe_packages_paths = get_workspace_packages_locations(dna_file_tree.file_tree_ref())?;

    match maybe_packages_paths {
        Some(mut packages_paths) if !packages_paths.is_empty() => {
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
/// 1. If there is a workspace member string with [anything]*/zomes/[something with "coordinator"], pick it and replace the star with the dna name
/// 2. If there is a workspace member string with [anything]*/[something with "coordinator"], pick it and replace the star with the dna name
/// 3. If there is only one workspace member string and it contains a * somewhere in the middle, replace the * with the dna name
/// 4. If there is only one workspace member string and it only contains a * at the end, take that folder (without the *)
/// 5. If all package paths without the [crate name]/Cargo.toml ending are equal, assume this is the zomes folder
///    and it is not being differentiated between integrity and coordinator zomes in that project
///
fn try_to_guess_coordinator_zomes_location(
    dna_file_tree: &DnaFileTree,
) -> ScaffoldResult<Option<PathBuf>> {
    let members = get_workspace_members(dna_file_tree.file_tree_ref())?;

    // if there is a workspace member string containing an expression with the word integrity followed by /*, pick this one
    // and add the right dna name to the path
    let re =
        Regex::new(r"\A(?P<a>([^/*]*/)*)\*/(?P<b>(zomes/[^/*]*coordinator[^/*]*/))\*\z").unwrap();
    for member in members.clone() {
        if re.is_match(member.as_str()) {
            // if there is a zomes/[something with "coordinator"]/* pattern
            let new_path = re.replace(
                member.as_str(),
                format!("${{a}}{}/${{b}}", dna_file_tree.dna_manifest.name()),
            );
            return Ok(Some(PathBuf::from(new_path.to_string())));
        }
    }

    let re = Regex::new(r"\A(?P<a>([^/*]*/)*)\*/(?P<b>([^/*]*coordinator[^/*]*/))\*\z").unwrap();
    for member in members.clone() {
        if re.is_match(member.as_str()) {
            // if there is a [something with "coordinator"]/* pattern
            let new_path = re.replace(
                member.as_str(),
                format!("${{a}}{}/${{b}}", dna_file_tree.dna_manifest.name()),
            );
            return Ok(Some(PathBuf::from(new_path.to_string())));
        }
    }

    if members.len() == 1 {
        let re = Regex::new(r"\A(?P<a>([^/*]*/)*)\*/(?P<b>([^/*]*/)*)\*\z").unwrap();

        if re.is_match(members[0].as_str()) {
            let new_path = re.replace(
                members[0].as_str(),
                format!("${{a}}{}/${{b}}", dna_file_tree.dna_manifest.name()),
            );
            return Ok(Some(PathBuf::from(new_path.to_string())));
        }

        let re = Regex::new(r"\A(?P<a>([^/*]*/)*)\*\z").unwrap();

        if re.is_match(members[0].as_str()) {
            let new_path = re.replace(members[0].as_str(), r"${a}");
            return Ok(Some(PathBuf::from(new_path.to_string())));
        }
    }

    let maybe_packages_paths = get_workspace_packages_locations(dna_file_tree.file_tree_ref())?;

    match maybe_packages_paths {
        Some(mut packages_paths) if !packages_paths.is_empty() => {
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

pub fn add_common_zome_dependencies_to_workspace_cargo(
    file_tree: FileTree,
) -> ScaffoldResult<FileTree> {
    let file_tree = add_workspace_external_dependency(
        file_tree,
        "hdi",
        &format!("={}", versions::HDI_VERSION),
    )?;
    let file_tree = add_workspace_external_dependency(
        file_tree,
        "hdk",
        &format!("={}", versions::HDK_VERSION),
    )?;
    let file_tree = add_workspace_external_dependency(file_tree, "serde", "1.0")?;
    Ok(file_tree)
}

pub fn scaffold_integrity_zome_with_path(
    dna_file_tree: DnaFileTree,
    template_file_tree: &FileTree,
    zome_name: &str,
    path: &Path,
) -> ScaffoldResult<ScaffoldedTemplate> {
    check_for_reserved_keywords(zome_name)?;

    let dna_manifest_path = dna_file_tree.dna_manifest_path.clone();
    let dna_manifest = dna_file_tree.dna_manifest.clone();

    let file_tree = add_common_zome_dependencies_to_workspace_cargo(dna_file_tree.file_tree())?;

    let folder_name = match zome_name.strip_suffix("_integrity") {
        Some(f) => f.to_string(),
        None => zome_name.to_owned(),
    };

    let mut file_tree =
        add_workspace_path_dependency(file_tree, zome_name, &path.join(&folder_name))?;

    let initial_lib_rs = integrity::initial_lib_rs();

    // Add zome to workspace Cargo.toml
    let zome: FileTree = dir! {
        "Cargo.toml" => file!(integrity::initial_cargo_toml(zome_name)),
        "src" => dir! {
            "lib.rs" => file!(
                unparse_pretty(&syn::parse_quote!{ #initial_lib_rs })
            )
        }
    };
    insert_file_tree_in_dir(&mut file_tree, path, (OsString::from(folder_name), zome))?;

    let dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;

    let zome_manifest = new_integrity_zome_manifest(&dna_file_tree, zome_name)?;

    let dna_file_tree = add_integrity_zome_to_manifest(dna_file_tree, zome_manifest.clone())?;

    scaffold_integrity_zome_templates(
        dna_file_tree.file_tree(),
        template_file_tree,
        &dna_manifest.name(),
        &zome_manifest,
    )
}

pub fn scaffold_integrity_zome(
    dna_file_tree: DnaFileTree,
    template_file_tree: &FileTree,
    zome_name: &str,
    path: &Option<PathBuf>,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let path_to_scaffold_in = match path {
        Some(p) => p.clone(),
        None => match try_to_guess_integrity_zomes_location(&dna_file_tree)? {
            Some(p) => {
                if Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(format!("Scaffold integrity zome in folder {:?}?", p))
                    .interact()?
                {
                    p
                } else {
                    choose_directory_path(
                        &String::from("Where should the integrity zome be scaffolded instead?"),
                        dna_file_tree.file_tree_ref(),
                    )?
                }
            }
            None => choose_directory_path(
                &String::from("Where should the integrity zome be scaffolded?"),
                dna_file_tree.file_tree_ref(),
            )?,
        },
    };

    scaffold_integrity_zome_with_path(
        dna_file_tree,
        template_file_tree,
        zome_name,
        &path_to_scaffold_in,
    )
}

pub fn scaffold_coordinator_zome_in_path(
    dna_file_tree: DnaFileTree,
    template_file_tree: &FileTree,
    zome_name: &str,
    dependencies: Option<&Vec<String>>,
    path: &Path,
) -> ScaffoldResult<ScaffoldedTemplate> {
    check_for_reserved_keywords(zome_name)?;

    let dna_manifest = dna_file_tree.dna_manifest.clone();

    let coordinator_zome_manifest =
        new_coordinator_zome_manifest(&dna_file_tree, zome_name, dependencies)?;

    let dna_file_tree =
        add_coordinator_zome_to_manifest(dna_file_tree, coordinator_zome_manifest.clone())?;

    // Add zome to workspace Cargo.toml
    let file_tree = add_common_zome_dependencies_to_workspace_cargo(dna_file_tree.file_tree())?;
    let mut file_tree = add_workspace_path_dependency(file_tree, zome_name, &path.join(zome_name))?;

    let initial_lib_rs = coordinator::initial_lib_rs(dependencies);

    let zome: FileTree = dir! {
        "Cargo.toml" => file!(coordinator::initial_cargo_toml(zome_name, dependencies)),
        "src" => dir! {
            "lib.rs" => file!(
                unparse_pretty(&syn::parse_quote!{ #initial_lib_rs })
            )
        }
    };

    insert_file_tree_in_dir(&mut file_tree, path, (OsString::from(zome_name), zome))?;

    scaffold_coordinator_zome_templates(
        file_tree,
        template_file_tree,
        &dna_manifest.name(),
        &coordinator_zome_manifest,
    )
}

pub fn scaffold_coordinator_zome(
    dna_file_tree: DnaFileTree,
    template_file_tree: &FileTree,
    zome_name: &str,
    dependencies: Option<&Vec<String>>,
    path: &Option<PathBuf>,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let prompt = String::from("Where should the coordinator zome be scaffolded?");

    let path_to_scaffold_in = match path {
        Some(p) => p.clone(),
        None => match try_to_guess_coordinator_zomes_location(&dna_file_tree)? {
            Some(p) => {
                if Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(format!("Scaffold coordinator zome in {:?}?", p))
                    .interact()?
                {
                    p
                } else {
                    choose_directory_path(&prompt, dna_file_tree.file_tree_ref())?
                }
            }
            None => choose_directory_path(&prompt, dna_file_tree.file_tree_ref())?,
        },
    };

    scaffold_coordinator_zome_in_path(
        dna_file_tree,
        template_file_tree,
        zome_name,
        dependencies,
        &path_to_scaffold_in,
    )
}

pub fn scaffold_zome_pair(
    app_file_tree: FileTree,
    template_file_tree: FileTree,
    dna_name: &str,
) -> Result<(), ScaffoldError> {
    let mut dna_file_tree = DnaFileTree::get_or_choose(app_file_tree, Some(dna_name))?;
    let dna_manifest_path = dna_file_tree.dna_manifest_path.clone();

    let zome_name = input_with_case(
            "Enter coordinator zome name (snake_case):\n(The integrity zome will automatically be named '{name of coordinator zome}_integrity')\n",
            Some( dna_name ),
            Case::Snake,
        )?;

    let integrity_zome_name = integrity_zome_name(&zome_name);
    let ScaffoldedTemplate { file_tree, .. } = scaffold_integrity_zome(
        dna_file_tree,
        &template_file_tree,
        &integrity_zome_name,
        &None,
    )?;
    dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;

    let ScaffoldedTemplate { file_tree, .. } = scaffold_coordinator_zome(
        dna_file_tree,
        &template_file_tree,
        &zome_name,
        Some(&vec![integrity_zome_name]),
        &None,
    )?;

    build_file_tree(file_tree, ".")?;
    Ok(())
}
