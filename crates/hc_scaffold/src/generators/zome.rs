use holochain_scaffolding_utils::FileTree;

use crate::error::ScaffoldResult;

pub mod coordinator;
pub mod integrity;
pub mod utils;

use coordinator::add_coordinator_zome_to_manifest;
use integrity::add_integrity_zome_to_manifest;

use super::{app::utils::get_or_choose_app_manifest, dna::utils::get_or_choose_dna_manifest};

pub fn scaffold_zome(
    app_file_tree: FileTree,
    app_name: Option<String>,
    dna_name: Option<String>,
    zome_name: String,
) -> ScaffoldResult<FileTree> {
    let app_manifest = get_or_choose_app_manifest(&app_file_tree, app_name)?;
    let (dna_manifest_path, _dna_manifest) =
        get_or_choose_dna_manifest(&app_file_tree, &app_manifest, dna_name)?;

    let integrity_zome_name = format!("{}_integrity", zome_name);

    let app_file_tree = add_integrity_zome_to_manifest(
        app_file_tree,
        &dna_manifest_path,
        integrity_zome_name.clone(),
    )?;
    let app_file_tree = add_coordinator_zome_to_manifest(
        app_file_tree,
        &dna_manifest_path,
        zome_name,
        Some(vec![integrity_zome_name]),
    )?;

    Ok(app_file_tree)
}

pub fn scaffold_integrity_zome(
    app_file_tree: FileTree,
    app_name: Option<String>,
    dna_name: Option<String>,
    zome_name: String,
) -> ScaffoldResult<FileTree> {
    let app_manifest = get_or_choose_app_manifest(&app_file_tree, app_name)?;
    let (dna_manifest_path, _dna_manifest) =
        get_or_choose_dna_manifest(&app_file_tree, &app_manifest, dna_name)?;

    add_integrity_zome_to_manifest(app_file_tree, &dna_manifest_path, zome_name)
}

pub fn scaffold_coordinator_zome(
    app_file_tree: FileTree,
    app_name: Option<String>,
    dna_name: Option<String>,
    zome_name: String,
    dependencies: Option<Vec<String>>,
) -> ScaffoldResult<FileTree> {
    let app_manifest = get_or_choose_app_manifest(&app_file_tree, app_name)?;
    let (dna_manifest_path, _dna_manifest) =
        get_or_choose_dna_manifest(&app_file_tree, &app_manifest, dna_name)?;

    add_coordinator_zome_to_manifest(app_file_tree, &dna_manifest_path, zome_name, dependencies)
}
