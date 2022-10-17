use holochain_scaffolding_utils::FileTree;

use crate::{cli::Crud, error::ScaffoldResult};

fn add_entry_def_to_integrity_zome(
    integrity_zome_file_tree: FileTree,
    entry_def_name: &String,
) -> ScaffoldResult<FileTree> {
    // 1. Create an ENTRY_DEF_NAME.rs in "src/", with the entry definition struct
    // 2. Add this file as a module in the entry point for the crate

    // 3. Find the #[hdk_entry_defs] macro
    // 4. Import the new struct
    // 4. Add a variant for the new entry def with the struct as its payload

    Ok(integrity_zome_file_tree)
}

fn add_crud_functions_to_coordinator(
    coordinator_zome_file_tree: FileTree,
    entry_def_name: &String,
    crud: Option<Crud>,
) -> ScaffoldResult<FileTree> {
    // 1. Create an ENTRY_DEF_NAME.rs in "src/", with the appropriate crud functions
    // 2. Add this file as a module in the entry point for the crate

    Ok(coordinator_zome_file_tree)
}

pub fn scaffold_entry_def(app_file_tree: FileTree) -> ScaffoldResult<FileTree> {
    Ok(app_file_tree)
}
