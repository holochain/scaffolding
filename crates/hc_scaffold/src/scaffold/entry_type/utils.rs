use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};
use holochain_types::prelude::DnaManifest;

use crate::{
    definitions::EntryType,
    error::{ScaffoldError, ScaffoldResult},
    scaffold::zome::ZomeFileTree,
};

use super::integrity::get_all_entry_types;

pub fn choose_entry_type(
    all_entries: &Vec<String>,
    prompt: &String,
    include_agent_pub_key: bool,
) -> ScaffoldResult<EntryType> {
    let mut all_options = all_entries.clone();

    let mut select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt.clone())
        .default(0)
        .items(&all_options[..]);

    if include_agent_pub_key {
        select.item("Agent");
    }

    let selection = select.interact()?;

    match selection == all_options.len() {
        true => Ok(EntryType::Agent),
        false => Ok(EntryType::App(all_options[selection].clone())),
    }
}

pub fn choose_optional_entry_type(
    all_entries: &Vec<String>,
    prompt: &String,
) -> ScaffoldResult<Option<EntryType>> {
    let mut all_options = all_entries.clone();
    all_options.push("Agent".into());

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt.clone())
        .default(0)
        .items(&all_options[..])
        .item("[None]")
        .interact()?;

    if selection == all_options.len() {
        return Ok(None);
    } else if selection == all_options.len() - 1 {
        return Ok(Some(EntryType::Agent));
    }

    Ok(Some(EntryType::App(all_options[selection].clone())))
}

pub fn choose_multiple_entry_types(
    all_entries: &Vec<String>,
    prompt: &String,
    allow_empty_selection: bool,
) -> ScaffoldResult<Vec<String>> {
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt.clone())
        .items(&all_entries[..])
        .interact()?;

    let chosen_entry_types = match (selection.len(), allow_empty_selection) {
        (0, false) => choose_multiple_entry_types(all_entries, &String::from("X You must choose at least one entry type to index. Press SPACE to select/unselect an entry type:"), false)?,
        _ => selection
                .into_iter()
                .map(|i| all_entries[i].clone())
                .collect()
    };

    Ok(chosen_entry_types)
}

pub fn get_or_choose_entry_type(
    zome_file_tree: &ZomeFileTree,
    entry_type: &Option<EntryType>,
    prompt: &String,
) -> ScaffoldResult<EntryType> {
    let all_entries = get_all_entry_types(&zome_file_tree)?.unwrap_or_else(|| vec![]);

    match entry_type {
        None => choose_entry_type(&all_entries, prompt, true),
        Some(entry_type) => {
            if let EntryType::Agent = entry_type {
                return Ok(EntryType::Agent);
            }
            let entry_type_name = all_entries
                .into_iter()
                .find(|et| et.eq(&entry_type.to_string()))
                .ok_or(ScaffoldError::EntryTypeNotFound(
                    entry_type.to_string().clone(),
                    zome_file_tree.dna_file_tree.dna_manifest.name(),
                    zome_file_tree.zome_manifest.name.0.to_string(),
                ))?;

            Ok(EntryType::App(entry_type_name))
        }
    }
}

pub fn get_or_choose_optional_entry_type(
    zome_file_tree: &ZomeFileTree,
    entry_type: &Option<EntryType>,
    prompt: &String,
) -> ScaffoldResult<Option<EntryType>> {
    let all_entries = get_all_entry_types(&zome_file_tree)?.unwrap_or_else(|| vec![]);

    match entry_type {
        None => choose_optional_entry_type(&all_entries, prompt),
        Some(entry_type) => {
            if let EntryType::Agent = entry_type {
                return Ok(Some(EntryType::Agent));
            }
            let entry_type_name = all_entries
                .into_iter()
                .find(|et| et.eq(&entry_type.to_string()))
                .ok_or(ScaffoldError::EntryTypeNotFound(
                    entry_type.to_string().clone(),
                    zome_file_tree.dna_file_tree.dna_manifest.name(),
                    zome_file_tree.zome_manifest.name.0.to_string(),
                ))?;

            Ok(Some(EntryType::App(entry_type_name)))
        }
    }
}
