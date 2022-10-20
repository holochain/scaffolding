use std::{ffi::OsString, path::PathBuf};

use crate::file_tree::FileTree;
use build_fs_tree::file;
use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};
use holochain_types::prelude::{AppManifest, DnaManifest};

use crate::{
    cli::Crud,
    error::{ScaffoldError, ScaffoldResult},
};

use self::{
    coordinator::add_crud_functions_to_coordinator, integrity::add_entry_def_to_integrity_zome,
};

use super::zome::utils::{get_coordinator_zomes_for_integrity, zome_manifest_path};

pub mod coordinator;
pub mod integrity;

pub fn scaffold_entry_def(
    mut app_file_tree: FileTree,
    app_manifest: &AppManifest,
    dna_manifest: &DnaManifest,
    integrity_zome_name: &String,
    entry_def_name: &String,
    crud: &Option<Crud>,
) -> ScaffoldResult<FileTree> {
    let app_file_tree = add_entry_def_to_integrity_zome(
        app_file_tree,
        app_manifest,
        dna_manifest,
        integrity_zome_name,
        &entry_def_name,
    )?;

    let coordinator_zomes_for_integrity =
        get_coordinator_zomes_for_integrity(dna_manifest, integrity_zome_name);

    let coordinator_zome = match coordinator_zomes_for_integrity.len() {
        0 => Err(ScaffoldError::NoCoordinatorZomesFoundForIntegrityZome(
            dna_manifest.name(),
            integrity_zome_name.clone(),
        )),
        1 => Ok(coordinator_zomes_for_integrity[0].clone()),
        _ => {
            let names: Vec<String> = coordinator_zomes_for_integrity
                .iter()
                .map(|z| z.name.0.to_string())
                .collect();
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Which coordinator zome should the CRUD functions be scaffolded in?")
                .default(0)
                .items(&names[..])
                .interact()?;

            Ok(coordinator_zomes_for_integrity[selection].clone())
        }
    }?;

    let crud = get_or_choose_crud(crud);
    let app_file_tree = add_crud_functions_to_coordinator(
        app_file_tree,
        dna_manifest,
        integrity_zome_name,
        &coordinator_zome,
        entry_def_name,
        &crud,
    )?;

    Ok(app_file_tree)
}

fn get_or_choose_crud(crud: &Option<Crud>) -> Crud {
    match crud {
        Some(c) => c.clone(),
        None => {
            let selections = MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Which CRUD functions should be scaffolded?")
                .item_checked("Read", true)
                .item_checked("Update", true)
                .item_checked("Delete", true)
                .interact()
                .unwrap();

            let mut crud = Crud {
                delete: false,
                read: false,
                update: false,
            };

            for selection in selections {
                if selection == 0 {
                    crud.read = true;
                }
                if selection == 1 {
                    crud.update = true;
                }
                if selection == 2 {
                    crud.delete = true;
                }
            }

            crud
        }
    }
}
