use dialoguer::{theme::ColorfulTheme, Select};
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

pub fn get_or_choose_entry_type(
    dna_manifest: &DnaManifest,
    zome_name: &String,
    entry_type: &Option<String>,
    all_entries: &Vec<String>,
    prompt: &String,
) -> ScaffoldResult<String> {
    match (all_entries.len(), entry_type) {
        (0, None) => Err(ScaffoldError::NoEntryDefsFoundForIntegrityZome(
            dna_manifest.name(),
            zome_name.clone(),
        )),
        (_, None) => choose_entry_type(all_entries, prompt),
        (_, Some(name)) => all_entries
            .into_iter()
            .find(|et| et.eq(&name))
            .cloned()
            .ok_or(ScaffoldError::EntryDefNotFound(
                name.clone(),
                dna_manifest.name(),
                zome_name.clone(),
            )),
    }
}
