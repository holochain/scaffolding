use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};
use holochain_types::prelude::DnaManifest;

use crate::error::{ScaffoldError, ScaffoldResult};

pub fn choose_entry_type(all_entries: &Vec<String>, prompt: &String) -> ScaffoldResult<String> {
    let mut all_options = all_entries.clone();
    all_options.push("AgentPubKey".into());

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt.clone())
        .default(0)
        .items(&all_options[..])
        .interact()?;

    Ok(all_options[selection].clone())
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
    dna_manifest: &DnaManifest,
    zome_name: &String,
    entry_type: &Option<String>,
    all_entries: &Vec<String>,
    prompt: &String,
) -> ScaffoldResult<String> {
    match entry_type {
        None => choose_entry_type(all_entries, prompt),
        Some(name) => all_entries
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
