use std::{collections::BTreeMap, ffi::OsString, path::PathBuf};

use crate::{
    cli::Crud,
    definitions::EntryDefinition,
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{create_dir_all, FileTree},
};

pub mod entry_crud_tests;
pub mod link_type_tests;
pub mod package_json;
pub mod utils;

use build_fs_tree::file;
use convert_case::{Case, Casing};
use entry_crud_tests::entry_crud_tests;
use holochain_types::prelude::AppManifest;

fn find_or_choose_tryorama_package_path(app_file_tree: &FileTree) -> ScaffoldResult<PathBuf> {
    // TODO: Actually implement this
    Ok(PathBuf::from("tests"))
}

pub fn add_tryorama_tests_for_entry_def(
    mut app_file_tree: FileTree,
    app_manifest: &(PathBuf, AppManifest),
    dna_role_id: &String,
    coordinator_zome: &String,
    entry_def: &EntryDefinition,
    crud: &Crud,
) -> ScaffoldResult<FileTree> {
    let tryorama_path = find_or_choose_tryorama_package_path(&app_file_tree)?;

    let mut happ_bundle_path_from_root = app_manifest.0.clone();
    happ_bundle_path_from_root.pop();
    happ_bundle_path_from_root =
        happ_bundle_path_from_root.join(format!("{}.happ", app_manifest.1.app_name()));

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
    );

    let test_path = tryorama_path
        .join("src")
        .join(dna_role_id)
        .join(coordinator_zome);

    let kebab_entry_def_name = entry_def.name.clone().to_case(Case::Kebab);

    create_dir_all(&mut app_file_tree, &test_path)?;
    let v: Vec<OsString> = test_path.iter().map(|s| s.to_os_string()).collect();
    app_file_tree
        .path_mut(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(test_path.clone()))?
        .dir_content_mut()
        .ok_or(ScaffoldError::PathNotFound(test_path.clone()))?
        .insert(
            OsString::from(format!("{}.test.ts", kebab_entry_def_name.clone())),
            file!(tests_file),
        );

    Ok(app_file_tree)
}
