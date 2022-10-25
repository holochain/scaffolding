use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};
use holochain_types::prelude::DnaManifest;

use crate::error::{ScaffoldError, ScaffoldResult};

pub fn choose_entry_type(all_entries: &Vec<String>, prompt: &String) -> ScaffoldResult<String> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt.clone())
        .default(0)
        .items(&all_entries[..])
        .interact()?;

    Ok(all_entries[selection].clone())
}

pub fn choose_multiple_entry_types(
    all_entries: &Vec<String>,
    prompt: &String,
) -> ScaffoldResult<Vec<String>> {
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt.clone())
        .items(&all_entries[..])
        .interact()?;

    let chosen_entry_types = selection
        .into_iter()
        .map(|i| all_entries[i].clone())
        .collect();

    Ok(chosen_entry_types)
}

pub fn get_or_choose_entry_type(
    dna_manifest: &DnaManifest,
    zome_name: &String,
    entry_type: &Option<String>,
    all_entries: &Vec<String>,
    prompt: &String,
) -> ScaffoldResult<String> {
    match (all_entries.len(), entry_type) {
        (0, None) => Err(ScaffoldError::NoEntryTypesDefFoundForIntegrityZome(
            dna_manifest.name(),
            zome_name.clone(),
        )),
        (_, None) => choose_entry_type(all_entries, prompt),
        (_, Some(name)) => all_entries
            .into_iter()
            .find(|et| et.eq(&name))
            .cloned()
            .ok_or(ScaffoldError::EntryTypeNotFound(
                name.clone(),
                dna_manifest.name(),
                zome_name.clone(),
            )),
    }
}
