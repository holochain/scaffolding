use anyhow::Context;
use convert_case::Case;
use dialoguer::{theme::ColorfulTheme, Select};

use super::definitions::{EntryTypeReference, Referenceable};
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

    let (_, value) = options[selection];

    Ok(value)
}

pub fn get_or_choose_referenceable(
    prompt: &str,
    zome_file_tree: &ZomeFileTree,
    entry_type: Option<&Referenceable>,
    all_entries: &[EntryTypeReference],
) -> ScaffoldResult<Referenceable> {
    match &entry_type {
        Some(Referenceable::Agent { role }) => {
            check_for_reserved_keywords(role)?;
            Ok(Referenceable::Agent { role: role.clone() })
        }
        Some(Referenceable::EntryType(app_entry_reference)) => {
            let all_entries: Vec<&str> =
                all_entries.iter().map(|e| e.entry_type.as_str()).collect();

            all_entries
                .into_iter()
                .find(|et| et == &app_entry_reference.entry_type)
                .ok_or(ScaffoldError::EntryTypeNotFound(
                    app_entry_reference.entry_type.to_string().clone(),
                    zome_file_tree.dna_file_tree.dna_manifest.name(),
                    zome_file_tree.zome_manifest.name.to_string(),
                ))?;

            Ok(Referenceable::EntryType(app_entry_reference.clone()))
        }
        _ => choose_referenceable(all_entries, prompt),
    }
}

pub fn get_or_choose_optional_reference_type(
    prompt: &str,
    zome_file_tree: &ZomeFileTree,
    entry_type: Option<&Referenceable>,
    all_entries: &[EntryTypeReference],
) -> ScaffoldResult<Option<Referenceable>> {
    match entry_type {
        Some(Referenceable::Agent { .. }) => Ok(entry_type.cloned()),
        Some(Referenceable::EntryType(app_entry_reference)) => {
            let all_entries: Vec<&str> =
                all_entries.iter().map(|e| e.entry_type.as_str()).collect();

            all_entries
                .into_iter()
                .find(|et| et == &app_entry_reference.entry_type)
                .ok_or(ScaffoldError::EntryTypeNotFound(
                    app_entry_reference.entry_type.to_string().clone(),
                    zome_file_tree.dna_file_tree.dna_manifest.name(),
                    zome_file_tree.zome_manifest.name.to_string(),
                ))?;

            Ok(entry_type.cloned())
        }
        _ => choose_optional_referenceable(all_entries, prompt),
    }
}

pub fn choose_referenceable(
    all_entries: &[EntryTypeReference],
    prompt: &str,
) -> ScaffoldResult<Referenceable> {
    let maybe_reference_type = inner_choose_referenceable(all_entries, prompt, None)?;
    Ok(maybe_reference_type.context("Reference type should not be None")?)
}

pub fn choose_optional_referenceable(
    all_entries: &[EntryTypeReference],
    prompt: &str,
) -> ScaffoldResult<Option<Referenceable>> {
    inner_choose_referenceable(
        all_entries,
        prompt,
        Some(vec!["[None] (Use this link to attach meta-data only)"]),
    )
}

fn inner_choose_referenceable(
    all_entries: &[EntryTypeReference],
    prompt: &str,
    extra_options: Option<Vec<&str>>,
) -> ScaffoldResult<Option<Referenceable>> {
    let mut all_options: Vec<String> = all_entries
        .iter()
        .map(|r| r.entry_type.to_owned())
        .collect();

    all_options.push("ExternalHash".to_string());
    all_options.push("Agent".to_string());

    if let Some(options) = extra_options {
        all_options.extend(options.into_iter().map(String::from).collect::<Vec<_>>())
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
            None,
            Case::Snake,
        )?;
            check_for_reserved_keywords(&role)?;
            Ok(Some(Referenceable::Agent { role }))
        }
        "ExternalHash" => {
            let name = input_with_case(
                "What name should be given to the link for this hash?",
                None,
                Case::Snake,
            )?;
            Ok(Some(Referenceable::ExternalHash { name }))
        }
        entry_type if entry_type.starts_with("[None]") => Ok(None),
        entry_type => Ok(Some(Referenceable::EntryType(EntryTypeReference {
            entry_type: entry_type.to_owned(),
            reference_entry_hash: choose_reference_entry_hash(
                "Reference this entry type with its entry hash or its action hash?",
                all_entries[selection].reference_entry_hash,
            )?,
        }))),
    }
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
