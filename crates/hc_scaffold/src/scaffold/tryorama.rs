use std::{collections::BTreeMap, ffi::OsString, path::PathBuf};

use crate::{
    definitions::EntryDefinition,
    error::ScaffoldResult,
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
use holochain_types::prelude::ZomeManifest;

use super::{app::utils::read_app_manifest, entry_def::crud::Crud};

fn find_or_choose_tryorama_package_path(_app_file_tree: &FileTree) -> ScaffoldResult<PathBuf> {
    // TODO: Actually implement this
    Ok(PathBuf::from("tests"))
}

pub fn add_tryorama_tests_for_entry_def(
    mut app_file_tree: FileTree,
    dna_manifest_path: &PathBuf,
    coordinator_zome: &String,
    entry_def: &EntryDefinition,
    crud: &Crud,
    create_fns_for_depends_on: &BTreeMap<String, (ZomeManifest, String)>,
) -> ScaffoldResult<FileTree> {
    let tryorama_path = find_or_choose_tryorama_package_path(&app_file_tree)?;

    let app_manifest = read_app_manifest(&app_file_tree, app_manifest_path)?;

    let mut happ_bundle_path_from_root = app_manifest_path.clone();
    happ_bundle_path_from_root.pop();
    happ_bundle_path_from_root =
        happ_bundle_path_from_root.join(format!("{}.happ", app_manifest.app_name()));

    let mut happ_bundle_from_tryorama_path = PathBuf::new();

    for _c in tryorama_path.components() {
        happ_bundle_from_tryorama_path.push("..");
    }
    for c in happ_bundle_path_from_root.components() {
        happ_bundle_from_tryorama_path.push(c);
    }

    let tests_file = entry_crud_tests(
        entry_def,
        &happ_bundle_from_tryorama_path,
        dna_role_id,
        coordinator_zome,
        crud,
        &create_fns_for_depends_on,
    );

    let test_path = tryorama_path
        .join("src")
        .join(dna_role_id)
        .join(coordinator_zome);

    let kebab_entry_def_name = entry_def.name.clone().to_case(Case::Kebab);

    create_dir_all(&mut app_file_tree, &test_path)?;

    insert_file_tree_in_dir(
        &mut app_file_tree,
        &test_path,
        (
            OsString::from(format!("{}.test.ts", kebab_entry_def_name.clone())),
            file!(tests_file),
        ),
    )?;

    Ok(app_file_tree)
}
