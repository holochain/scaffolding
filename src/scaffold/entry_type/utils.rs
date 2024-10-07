use convert_case::Case;
use dialoguer::{theme::ColorfulTheme, Select};

use super::{
    definitions::{EntryTypeReference, Referenceable},
    integrity::get_all_entry_types,
};
use crate::{
    error::{ScaffoldError, ScaffoldResult},
    reserved_words::check_for_reserved_keywords,
    scaffold::zome::ZomeFileTree,
    utils::input_with_case,
};

pub fn choose_reference_entry_hash(prompt: &str, recommended: bool) -> ScaffoldResult<bool> {
    let options = if recommended {
        [("EntryHash", true), ("ActionHash", false)]
    } else {
        [("ActionHash", false), ("EntryHash", true)]
    };

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&options.map(|(name, _)| name))
        .interact()?;

    Ok(options[selection].1)
}

fn inner_choose_referenceable(
    all_entries: &[EntryTypeReference],
    prompt: &str,
    optional: bool,
) -> ScaffoldResult<Option<Referenceable>> {
    let mut all_options: Vec<String> = all_entries
        .iter()
        .map(|r| r.entry_type.to_owned())
        .collect();

    all_options.push("Agent".to_string());

    if optional {
        all_options.push("AnyLinkableHash".to_string());
        all_options.push("[None]".to_string());
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&all_options[..])
        .interact()?;

    match all_options[selection].as_str() {
        "Agent" => {
            let role = input_with_case(
            "Which role does this agent play in the relationship ? (eg. \"creator\", \"invitee\")",
            Case::Snake,
        )?;
            check_for_reserved_keywords(&role)?;
            Ok(Some(Referenceable::Agent { role }))
        }
        "AnyLinkableHash" => {
            let name = input_with_case(
                "What name should be given to the link for this hash?",
                Case::Snake,
            )?;
            Ok(Some(Referenceable::AnyLinkableHash { name }))
        }
        "[None]" => Ok(None),
        entry_type => Ok(Some(Referenceable::EntryType(EntryTypeReference {
            entry_type: entry_type.to_owned(),
            reference_entry_hash: choose_reference_entry_hash(
                &String::from("Reference this entry type with its entry hash or its action hash?"),
                all_entries[selection].reference_entry_hash,
            )?,
        }))),
    }
}

pub fn choose_referenceable(
    all_entries: &[EntryTypeReference],
    prompt: &str,
) -> ScaffoldResult<Referenceable> {
    let maybe_reference_type = inner_choose_referenceable(all_entries, prompt, false)?;

    Ok(maybe_reference_type.expect("reference type should not be None"))
}

pub fn choose_optional_referenceable(
    all_entries: &[EntryTypeReference],
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
    entry_type: Option<&Referenceable>,
    prompt: &str,
) -> ScaffoldResult<Referenceable> {
    let all_entries = get_all_entry_types(zome_file_tree)?.unwrap_or_else(Vec::new);

    match &entry_type {
        Some(Referenceable::Agent { role }) => {
            check_for_reserved_keywords(role)?;
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
        _ => choose_referenceable(&all_entries, prompt),
    }
}

pub fn get_or_choose_optional_reference_type(
    zome_file_tree: &ZomeFileTree,
    entry_type: Option<&Referenceable>,
    prompt: &str,
) -> ScaffoldResult<Option<Referenceable>> {
    let all_entries = get_all_entry_types(zome_file_tree)?.unwrap_or_else(Vec::new);

    match entry_type {
        Some(Referenceable::Agent { .. }) => Ok(entry_type.cloned()),
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

            Ok(entry_type.cloned())
        }
        _ => choose_optional_referenceable(&all_entries, prompt),
    }
}
