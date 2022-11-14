use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};
use holochain_types::prelude::DnaManifest;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    scaffold::zome::ZomeFileTree,
    utils::input_snake_case,
};

use super::{
    definitions::{EntryTypeReference, Referenceable},
    integrity::get_all_entry_types,
};

pub fn choose_reference_entry_hash(recommended: bool) -> ScaffoldResult<bool> {
    let mut select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(String::from(
            "Reference this entry type by its entry hash or its action hash?",
        ))
        .default(0);

    match recommended {
        true => {
            let selection = select
                .item("EntryHash (recommended)")
                .item("ActionHash")
                .interact()?;

            match selection {
                0 => Ok(true),
                _ => Ok(false),
            }
        }
        false => {
            let selection = select
                .item("ActionHash (recommended)")
                .item("EntryHash")
                .interact()?;

            match selection {
                0 => Ok(false),
                _ => Ok(true),
            }
        }
    }
}

pub fn choose_fixed() -> ScaffoldResult<bool> {
    let mut select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(String::from("Is this entry type fixed?"))
        .default(0);

    let selection = select
        .item("Not fixed: can be updated and deleted, referred to by ActionHash (recommended)")
        .item("Fixed: can't be deleted or updated, referred to by EntryHash")
        .interact()?;

    match selection {
        0 => Ok(false),
        _ => Ok(true),
    }
}

fn inner_choose_referenceable(
    all_entries: &Vec<EntryTypeReference>,
    prompt: &String,
    optional: bool,
) -> ScaffoldResult<Option<Referenceable>> {
    let mut all_options: Vec<String> = all_entries
        .clone()
        .into_iter()
        .map(|r| r.entry_type)
        .collect();

    let mut select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt.clone())
        .default(0)
        .items(&all_options[..])
        .item("Agent");

    if optional {
        select.item("[None]");
    }

    let selection = select.interact()?;

    if selection == all_options.len() {
        let role = input_snake_case(&String::from("Which role does this agent play in the relationship ? (eg. \"creator\", \"\", \"invitee\")"))?;
        return Ok(Some(Referenceable::Agent { role }));
    } else if selection == all_options.len() + 1 {
        return Ok(None);
    } else {
        Ok(Some(Referenceable::EntryType(EntryTypeReference {
            entry_type: all_options[selection].clone(),
            reference_entry_hash: choose_reference_entry_hash(
                all_entries[selection].reference_entry_hash,
            )?,
        })))
    }
}

pub fn choose_referenceable(
    all_entries: &Vec<EntryTypeReference>,
    prompt: &String,
) -> ScaffoldResult<Referenceable> {
    let maybe_reference_type = inner_choose_referenceable(all_entries, prompt, false)?;

    Ok(maybe_reference_type.expect("reference type should not be None"))
}

pub fn choose_optional_referenceable(
    all_entries: &Vec<EntryTypeReference>,
    prompt: &String,
) -> ScaffoldResult<Option<Referenceable>> {
    inner_choose_referenceable(all_entries, prompt, true)
}

pub fn choose_multiple_entry_types(
    all_entries: &Vec<EntryTypeReference>,
    prompt: &String,
    allow_empty_selection: bool,
) -> ScaffoldResult<Vec<EntryTypeReference>> {
    let mut all_options: Vec<String> = all_entries
        .clone()
        .into_iter()
        .map(|r| r.entry_type)
        .collect();

    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt.clone())
        .items(&all_options[..])
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

pub fn get_or_choose_referenceable(
    zome_file_tree: &ZomeFileTree,
    entry_type: &Option<Referenceable>,
    prompt: &String,
) -> ScaffoldResult<Referenceable> {
    let all_entries = get_all_entry_types(&zome_file_tree)?.unwrap_or_else(|| vec![]);

    match &entry_type {
        None => choose_referenceable(&all_entries, prompt),
        Some(Referenceable::Agent { role }) => Ok(Referenceable::Agent { role: role.clone() }),
        Some(Referenceable::EntryType(app_entry_reference)) => {
            let all_entries: Vec<String> = all_entries.into_iter().map(|e| e.entry_type).collect();

            let entry_type_name = all_entries
                .into_iter()
                .find(|et| et.eq(&app_entry_reference.entry_type.to_string()))
                .ok_or(ScaffoldError::EntryTypeNotFound(
                    app_entry_reference.entry_type.to_string().clone(),
                    zome_file_tree.dna_file_tree.dna_manifest.name(),
                    zome_file_tree.zome_manifest.name.0.to_string(),
                ))?;

            Ok(Referenceable::EntryType(app_entry_reference.clone()))
        }
    }
}

pub fn get_or_choose_optional_reference_type(
    zome_file_tree: &ZomeFileTree,
    entry_type: &Option<Referenceable>,
    prompt: &String,
) -> ScaffoldResult<Option<Referenceable>> {
    let all_entries = get_all_entry_types(&zome_file_tree)?.unwrap_or_else(|| vec![]);

    match entry_type {
        None => choose_optional_referenceable(&all_entries, prompt),
        Some(Referenceable::Agent { role }) => Ok(entry_type.clone()),
        Some(Referenceable::EntryType(app_entry_reference)) => {
            let all_entries: Vec<String> = all_entries.into_iter().map(|e| e.entry_type).collect();

            let entry_type_name = all_entries
                .into_iter()
                .find(|et| et.eq(&app_entry_reference.entry_type.to_string()))
                .ok_or(ScaffoldError::EntryTypeNotFound(
                    app_entry_reference.entry_type.to_string().clone(),
                    zome_file_tree.dna_file_tree.dna_manifest.name(),
                    zome_file_tree.zome_manifest.name.0.to_string(),
                ))?;

            Ok(entry_type.clone())
        }
    }
}
