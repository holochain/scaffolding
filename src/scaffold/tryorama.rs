use std::{collections::BTreeMap, ffi::OsString, path::PathBuf};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{create_dir_all, insert_file_tree_in_dir, FileTree},
};

pub mod entry_crud_tests;
pub mod link_type_tests;
pub mod package_json;
pub mod tsconfig_json;
pub mod utils;

use build_fs_tree::file;
use convert_case::{Case, Casing};
use entry_crud_tests::entry_crud_tests;
use holochain::prelude::AppManifest;
use mr_bundle::Location;

use super::{
    app::{choose_app, find_app_manifests},
    entry_type::{crud::Crud, definitions::EntryDefinition},
    zome::ZomeFileTree,
};

fn find_or_choose_tryorama_package_path(_app_file_tree: &FileTree) -> ScaffoldResult<PathBuf> {
    // TODO: Actually implement this
    Ok(PathBuf::from("tests"))
}

fn get_app_manifests_for_dna(
    file_tree: &FileTree,
    dna_manifest_path: &PathBuf,
) -> ScaffoldResult<BTreeMap<PathBuf, AppManifest>> {
    let manifests = find_app_manifests(file_tree)?;

    Ok(manifests
        .into_iter()
        .filter(|(p, m)| {
            let mut relative_dna_path = PathBuf::new();
            let mut p = p.clone();
            p.pop();
            for _c in p.components() {
                relative_dna_path.push("..");
            }
            relative_dna_path = relative_dna_path.join(dna_manifest_path);
            m.app_roles()
                .iter()
                .find(|r| {
                    if let Some(Location::Bundled(path)) = &r.dna.location {
                        if path.eq(&relative_dna_path) {
                            return true;
                        }
                    }
                    false
                })
                .is_some()
        })
        .collect())
}

pub fn add_tryorama_tests_for_entry_def(
    coordinator_zome_file_tree: ZomeFileTree,
    entry_def: &EntryDefinition,
    crud: &Crud,
    link_original_to_each_update: bool,
) -> ScaffoldResult<FileTree> {
    let tryorama_path = find_or_choose_tryorama_package_path(
        coordinator_zome_file_tree.dna_file_tree.file_tree_ref(),
    )?;

    let dna_name = coordinator_zome_file_tree.dna_file_tree.dna_manifest.name();
    let coordinator_zome_name = coordinator_zome_file_tree.zome_manifest.name.0.to_string();

    let mut dna_bundle_path_from_root = coordinator_zome_file_tree
        .dna_file_tree
        .dna_manifest_path
        .clone();
    dna_bundle_path_from_root.pop();
    dna_bundle_path_from_root = dna_bundle_path_from_root.join(format!("{}.dna", dna_name));

    let apps = get_app_manifests_for_dna(
        coordinator_zome_file_tree.dna_file_tree.file_tree_ref(),
        &dna_bundle_path_from_root,
    )?;

    let (app_manifest_path, manifest) = match apps.len() {
        0 => Err(ScaffoldError::NoAppsFoundForDna(dna_name.clone()))?,
        1 => apps.into_iter().next().unwrap(),
        _ => choose_app(apps)?,
    };

    let mut app_bundle_from_tryorama_path = PathBuf::new();

    for _c in tryorama_path.components() {
        app_bundle_from_tryorama_path.push("..");
    }
    for c in app_manifest_path.components() {
        app_bundle_from_tryorama_path.push(c);
    }

    app_bundle_from_tryorama_path.pop();
    app_bundle_from_tryorama_path =
        app_bundle_from_tryorama_path.join(format!("{}.happ", manifest.app_name()));

    let tests_file = entry_crud_tests(
        entry_def,
        &app_bundle_from_tryorama_path,
        &coordinator_zome_name,
        crud,
        link_original_to_each_update,
    );

    let test_path = tryorama_path
        .join("src")
        .join(dna_name)
        .join(coordinator_zome_name);

    let kebab_entry_def_name = entry_def.name.clone().to_case(Case::Kebab);

    let mut file_tree = coordinator_zome_file_tree.dna_file_tree.file_tree();

    create_dir_all(&mut file_tree, &test_path)?;

    insert_file_tree_in_dir(
        &mut file_tree,
        &test_path,
        (
            OsString::from(format!("{}.test.ts", kebab_entry_def_name.clone())),
            file!(tests_file),
        ),
    )?;

    Ok(file_tree)
}
