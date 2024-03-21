use convert_case::Case;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    reserved_words::check_for_reserved_words,
    scaffold::zome::ZomeFileTree,
    utils::input_with_case,
};

use super::{
    definitions::{EntryTypeReference, Referenceable},
    integrity::get_all_entry_types,
};

pub fn choose_reference_entry_hash(prompt: &str, recommended: bool) -> ScaffoldResult<bool> {
    match recommended {
        true => {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(prompt)
                .default(0)
                .item("EntryHash (recommended)")
                .item("ActionHash")
                .interact()?;

            match selection {
                0 => Ok(true),
                _ => Ok(false),
            }
        }
        false => {
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
    }
}

fn inner_choose_referenceable(
    all_entries: &Vec<EntryTypeReference>,
    prompt: &str,
    optional: bool,
) -> ScaffoldResult<Option<Referenceable>> {
    let mut all_options: Vec<String> = all_entries
        .clone()
        .into_iter()
        .map(|r| r.entry_type)
        .collect();

    all_options.push("Agent".to_string());

    if optional {
        all_options.push("[None]".to_string());
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&all_options[..])
        .interact()?;

    if selection == all_entries.len() {
        let role = input_with_case(&String::from(
            "Which role does this agent play in the relationship ? (eg. \"creator\", \"invitee\")",
        ), Case::Snake)?;
        check_for_reserved_words(&role)?;
        Ok(Some(Referenceable::Agent { role }))
    } else if selection == all_entries.len() + 1 {
        Ok(None)
    } else {
        Ok(Some(Referenceable::EntryType(EntryTypeReference {
            entry_type: all_options[selection].clone(),
            reference_entry_hash: choose_reference_entry_hash(
                &String::from("Reference this entry type with its entry hash or its action hash?"),
                all_entries[selection].reference_entry_hash,
            )?,
        })))
    }
}

pub fn choose_referenceable(
    all_entries: &Vec<EntryTypeReference>,
    prompt: &str,
) -> ScaffoldResult<Referenceable> {
    let maybe_reference_type = inner_choose_referenceable(all_entries, prompt, false)?;

    Ok(maybe_reference_type.expect("reference type should not be None"))
}

pub fn choose_optional_referenceable(
    all_entries: &Vec<EntryTypeReference>,
    prompt: &str,
) -> ScaffoldResult<Option<Referenceable>> {
    inner_choose_referenceable(all_entries, prompt, true)
}

pub fn choose_entry_type_reference(
    all_entries: &[EntryTypeReference],
    prompt: &str,
) -> ScaffoldResult<EntryTypeReference> {
    let all_options: Vec<String> = all_entries.iter().cloned().map(|r| r.entry_type).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&all_options[..])
        .interact()?;

    Ok(all_entries[selection].clone())
}

pub fn get_or_choose_referenceable(
    zome_file_tree: &ZomeFileTree,
    entry_type: &Option<Referenceable>,
    prompt: &str,
) -> ScaffoldResult<Referenceable> {
    let all_entries = get_all_entry_types(zome_file_tree)?.unwrap_or_else(Vec::new);

    match &entry_type {
        None => choose_referenceable(&all_entries, prompt),
        Some(Referenceable::Agent { role }) => {
            check_for_reserved_words(role)?;
            Ok(Referenceable::Agent { role: role.clone() })
        }
        Some(Referenceable::EntryType(app_entry_reference)) => {
            let all_entries: Vec<String> = all_entries.into_iter().map(|e| e.entry_type).collect();

            all_entries
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
    prompt: &str,
) -> ScaffoldResult<Option<Referenceable>> {
    let all_entries = get_all_entry_types(zome_file_tree)?.unwrap_or_else(Vec::new);

    match entry_type {
        None => choose_optional_referenceable(&all_entries, prompt),
        Some(Referenceable::Agent { .. }) => Ok(entry_type.clone()),
        Some(Referenceable::EntryType(app_entry_reference)) => {
            let all_entries: Vec<String> = all_entries.into_iter().map(|e| e.entry_type).collect();

            all_entries
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
