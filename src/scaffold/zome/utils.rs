use dialoguer::{theme::ColorfulTheme, MultiSelect};
use holochain_types::prelude::{DnaManifest, ZomeManifest};

use crate::error::ScaffoldResult;

/// Prompts a MultiSelect dialog to select one or multiple integrity zomes
///
/// Returns empty array if no integrity zomes are present.
pub fn select_integrity_zomes(
    dna_manifest: &DnaManifest,
    prompt: Option<&str>,
) -> ScaffoldResult<Vec<String>> {
    let integrity_zomes: Vec<String> = match dna_manifest {
        DnaManifest::V1(v1) => v1
            .integrity
            .zomes
            .clone()
            .into_iter()
            .map(|z| z.name.0.to_string())
            .collect(),
    };

    if integrity_zomes.is_empty() {
        return Ok(vec![]);
    }

    let prompt = prompt.unwrap_or("Select integrity zome (SPACE to select/unselect):");

    let selected_options = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&integrity_zomes)
        .interact()?;

    let selected_zomes = selected_options
        .iter()
        .map(|i| integrity_zomes[i.to_owned()].clone())
        .collect::<Vec<String>>();

    Ok(selected_zomes)
}

pub fn get_coordinator_zomes_for_integrity(
    dna_manifest: &DnaManifest,
    integrity_zome_name: &str,
) -> Vec<ZomeManifest> {
    match dna_manifest {
        DnaManifest::V1(v1) => v1
            .coordinator
            .zomes
            .clone()
            .into_iter()
            .filter(|z| match &z.dependencies {
                Some(d) => d
                    .iter()
                    .any(|zome_dep| zome_dep.name.0.eq(integrity_zome_name)),
                None => false,
            })
            .collect(),
    }
}
