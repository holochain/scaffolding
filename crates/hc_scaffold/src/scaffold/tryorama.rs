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

use super::{dna::DnaFileTree, entry_type::crud::Crud, zome::ZomeFileTree};

fn find_or_choose_tryorama_package_path(_app_file_tree: &FileTree) -> ScaffoldResult<PathBuf> {
    // TODO: Actually implement this
    Ok(PathBuf::from("tests"))
}

pub fn add_tryorama_tests_for_entry_def(
    coordinator_zome_file_tree: ZomeFileTree,
    entry_def: &EntryDefinition,
    crud: &Crud,
    link_original_to_each_update: bool,
    create_fns_for_depends_on: &BTreeMap<String, (ZomeManifest, String)>,
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

    let mut dna_bundle_from_tryorama_path = PathBuf::new();

    for _c in tryorama_path.components() {
        dna_bundle_from_tryorama_path.push("..");
    }
    for c in dna_bundle_path_from_root.components() {
        dna_bundle_from_tryorama_path.push(c);
    }

    let tests_file = entry_crud_tests(
        entry_def,
        &dna_bundle_from_tryorama_path,
        &coordinator_zome_name,
        crud,
        link_original_to_each_update,
        &create_fns_for_depends_on,
    );

    let test_path = tryorama_path
        .join("src")
        .join(dna_name)
        .join(coordinator_zome_name);

    let kebab_entry_def_name = entry_def.singular_name.clone().to_case(Case::Kebab);

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
