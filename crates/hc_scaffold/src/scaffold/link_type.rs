use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Select};
use holochain_types::prelude::DnaManifest;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::FileTree,
    templates::link_type::scaffold_link_type_templates,
    utils::input_snake_case,
};

use self::{
    coordinator::add_link_type_functions_to_coordinator, integrity::add_link_type_to_integrity_zome,
};

use super::{
    entry_type::{
        integrity::get_all_entry_types,
        utils::{get_or_choose_entry_type, get_or_choose_optional_entry_type},
    },
    zome::{utils::get_coordinator_zomes_for_integrity, ZomeFileTree},
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

pub fn choose_use_entry_hash(prompt: &String) -> ScaffoldResult<bool> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .item("ActionHash (recommended)")
        .item("EntryHash")
        .interact()?;

    match selection {
        0 => Ok(false),
        _ => Ok(true),
    }
}

pub fn scaffold_link_type(
    zome_file_tree: ZomeFileTree,
    template_file_tree: &FileTree,
    from_entry_type: &Option<String>,
    to_entry_type: &Option<String>,
    link_from_entry_hash: &Option<bool>,
    link_to_entry_hash: &Option<bool>,
) -> ScaffoldResult<(FileTree, String)> {
    let from_entry_type = get_or_choose_entry_type(
        &zome_file_tree,
        from_entry_type,
        &String::from("Link from which entry type?"),
    )?;

    let link_from_entry_hash: bool = match link_from_entry_hash {
        Some(l) => l.clone(),
        None => match from_entry_type.as_str() {
            "AgentPubKey" => false,
            _ => choose_use_entry_hash(&String::from(
                "Link from the entry hash or the action hash?",
            ))?,
        },
    };

    let to_entry_type = get_or_choose_optional_entry_type(
        &zome_file_tree,
        to_entry_type,
        &String::from("Link to which entry type?"),
    )?;

    let link_to_entry_hash: bool = match to_entry_type.clone() {
        None => false,
        Some(to_entry_type) => match link_to_entry_hash {
            Some(l) => l.clone(),
            None => match to_entry_type.as_str() {
                "AgentPubKey" => false,
                _ => choose_use_entry_hash(&String::from(
                    "Link to the entry hash or the action hash?",
                ))?,
            },
        },
    };

    let link_type_name = match to_entry_type.clone() {
        Some(to_entry_type) => link_type_name(&from_entry_type, &to_entry_type),
        None => input_snake_case(&String::from("Enter link type name:"))?.to_case(Case::Pascal),
    };

    let zome_file_tree = add_link_type_to_integrity_zome(zome_file_tree, &link_type_name)?;

    let integrity_zome_name = zome_file_tree.zome_manifest.name.0.to_string();

    let coordinator_zomes_for_integrity = get_coordinator_zomes_for_integrity(
        &zome_file_tree.dna_file_tree.dna_manifest,
        &zome_file_tree.zome_manifest.name.0.to_string(),
    );

    let coordinator_zome = match coordinator_zomes_for_integrity.len() {
        0 => Err(ScaffoldError::NoCoordinatorZomesFoundForIntegrityZome(
            zome_file_tree.dna_file_tree.dna_manifest.name(),
            zome_file_tree.zome_manifest.name.0.to_string(),
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

    let dna_manifest = zome_file_tree.dna_file_tree.dna_manifest.clone();

    let app_file_tree = add_link_type_functions_to_coordinator(
        zome_file_tree,
        &integrity_zome_name,
        &link_type_name,
        &from_entry_type,
        &to_entry_type,
        link_from_entry_hash,
        link_to_entry_hash,
    )?;

    let file_tree = scaffold_link_type_templates(
        app_file_tree.dna_file_tree.file_tree(),
        &template_file_tree,
        &dna_manifest.name(),
        &coordinator_zome.name.0.to_string(),
        &from_entry_type,
        &to_entry_type,
    )?;

    Ok((file_tree, link_type_name))
}
