use std::{ffi::OsString, path::PathBuf};

use build_fs_tree::file;
use convert_case::{Case, Casing};
use holochain_scaffolding_utils::FileTree;
use holochain_types::prelude::{AppManifest, DnaManifest};

use crate::{
    cli::Crud,
    error::{ScaffoldError, ScaffoldResult},
};

use super::zome::utils::zome_manifest_path;

pub mod definition;
use definition::initial_entry_def_file;

fn add_entry_def_to_integrity_zome(
    mut app_file_tree: FileTree,
    app_manifest: &AppManifest,
    dna_manifest: &DnaManifest,
    integrity_zome_name: &String,
    entry_def_name: &String,
) -> ScaffoldResult<FileTree> {
    let integrity_zome = match dna_manifest {
        DnaManifest::V1(v1) => v1
            .all_zomes()
            .into_iter()
            .find(|z| z.name.0.eq(integrity_zome_name)),
    }
    .ok_or(ScaffoldError::IntegrityZomeNotFound(
        integrity_zome_name.clone(),
        dna_manifest.name(),
    ))?;
    let mut manifest_path = zome_manifest_path(&app_file_tree, &integrity_zome)?.ok_or(
        ScaffoldError::IntegrityZomeNotFound(integrity_zome_name.clone(), dna_manifest.name()),
    )?;

    manifest_path.pop();

    let snake_entry_def_name = entry_def_name.to_case(Case::Snake);

    // 1. Create an ENTRY_DEF_NAME.rs in "src/", with the entry definition struct
    let crate_src_path = manifest_path.join("src");
    let v: Vec<OsString> = crate_src_path.iter().map(|s| s.to_os_string()).collect();
    app_file_tree
        .path_mut(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(crate_src_path.clone()))?
        .dir_content_mut()
        .ok_or(ScaffoldError::PathNotFound(crate_src_path.clone()))?
        .insert(
            OsString::from(snake_entry_def_name),
            file!(initial_entry_def_file(entry_def_name)),
        );

    // 2. Add this file as a module in the entry point for the crate

    // 3. Find the #[hdk_entry_defs] macro
    // 4. Import the new struct
    // 4. Add a variant for the new entry def with the struct as its payload

    Ok(app_file_tree)
}

fn add_crud_functions_to_coordinator(
    coordinator_zome_file_tree: &mut FileTree,
    entry_def_name: &String,
    crud: Option<Crud>,
) -> ScaffoldResult<()> {
    // 1. Create an ENTRY_DEF_NAME.rs in "src/", with the appropriate crud functions
    // 2. Add this file as a module in the entry point for the crate

    Ok(())
}

pub fn scaffold_entry_def(
    mut app_file_tree: FileTree,
    app_manifest: &AppManifest,
    dna_manifest: &DnaManifest,
    integrity_zome_name: &String,
    entry_def_name: &String,
) -> ScaffoldResult<FileTree> {
    let app_file_tree = add_entry_def_to_integrity_zome(
        app_file_tree,
        app_manifest,
        dna_manifest,
        integrity_zome_name,
        &entry_def_name,
    )?;

    Ok(app_file_tree)
}
