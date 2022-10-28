use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Select};
use holochain_types::prelude::DnaManifest;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::FileTree,
};

use self::{
    coordinator::add_link_type_functions_to_coordinator, integrity::add_link_type_to_integrity_zome,
};

use super::{
    entry_def::{integrity::get_all_entry_types, utils::get_or_choose_entry_type},
    zome::utils::get_coordinator_zomes_for_integrity,
};

pub mod coordinator;
pub mod integrity;

pub fn link_type_name(from_entry_type: &String, to_entry_type: &String) -> String {
    format!(
        "{}To{}",
        from_entry_type.to_case(Case::Pascal),
        to_entry_type.to_case(Case::Pascal)
    )
}

pub fn scaffold_link_type(
    app_file_tree: FileTree,
    dna_manifest: &DnaManifest,
    integrity_zome_name: &String,
    from_entry_type: &Option<String>,
    to_entry_type: &Option<String>,
    link_from_entry_hash: bool,
    link_to_entry_hash: bool,
) -> ScaffoldResult<(FileTree, String)> {
    let all_entries = get_all_entry_types(&app_file_tree, dna_manifest, integrity_zome_name)?
        .unwrap_or_else(|| vec![]);

    let from_entry_type = get_or_choose_entry_type(
        dna_manifest,
        integrity_zome_name,
        from_entry_type,
        &all_entries,
        &String::from("Link from which entry type?"),
    )?;

    let to_entry_type = get_or_choose_entry_type(
        dna_manifest,
        integrity_zome_name,
        to_entry_type,
        &all_entries,
        &String::from("Link to which entry type?"),
    )?;

    let link_type_name = link_type_name(&from_entry_type, &to_entry_type);

    let app_file_tree = add_link_type_to_integrity_zome(
        app_file_tree,
        dna_manifest,
        integrity_zome_name,
        &link_type_name,
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
                .with_prompt(
                    "Which coordinator zome should the link type functions be scaffolded in?",
                )
                .default(0)
                .items(&names[..])
                .interact()?;

            Ok(coordinator_zomes_for_integrity[selection].clone())
        }
    }?;

    let app_file_tree = add_link_type_functions_to_coordinator(
        app_file_tree,
        dna_manifest,
        integrity_zome_name,
        &coordinator_zome,
        &link_type_name,
        &from_entry_type,
        &to_entry_type,
        link_from_entry_hash,
        link_to_entry_hash,
    )?;

    Ok((app_file_tree, link_type_name))
}
