use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::FileTree,
    templates::{link_type::scaffold_link_type_templates, ScaffoldedTemplate},
    utils::input_snake_case,
};

use self::{
    coordinator::add_link_type_functions_to_coordinator, integrity::add_link_type_to_integrity_zome,
};

use super::{
    entry_type::{
        definitions::{Cardinality, Referenceable},
        utils::{get_or_choose_optional_reference_type, get_or_choose_referenceable},
    },
    zome::{utils::get_coordinator_zomes_for_integrity, ZomeFileTree},
};

pub mod coordinator;
pub mod integrity;

pub fn link_type_name(
    from_referenceable: &Referenceable,
    to_referenceable: &Referenceable,
) -> String {
    format!(
        "{}To{}",
        pluralizer::pluralize(
            from_referenceable.to_string(&Cardinality::Single).as_str(),
            1,
            false
        )
        .to_case(Case::Pascal),
        pluralizer::pluralize(
            to_referenceable.to_string(&Cardinality::Vector).as_str(),
            2,
            false
        )
        .to_case(Case::Pascal),
    )
}

pub fn scaffold_link_type(
    zome_file_tree: ZomeFileTree,
    template_file_tree: &FileTree,
    from_referenceable: &Option<Referenceable>,
    to_referenceable: &Option<Referenceable>,
    bidireccional: &Option<bool>,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let from_referenceable = get_or_choose_referenceable(
        &zome_file_tree,
        from_referenceable,
        &String::from("Link from which entry type?"),
    )?;

    let to_referenceable = get_or_choose_optional_reference_type(
        &zome_file_tree,
        to_referenceable,
        &String::from("Link to which entry type?"),
    )?;

    let link_type = match to_referenceable.clone() {
        Some(to_referenceable) => link_type_name(&from_referenceable, &to_referenceable),
        None => input_snake_case(&String::from("Enter link type name:"))?.to_case(Case::Pascal),
    };

    let bidireccional = match bidireccional {
        Some(b) => b.clone(),
        None => Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Should the link be bidireccional?")
            .interact()?,
    };

    let mut zome_file_tree = add_link_type_to_integrity_zome(zome_file_tree, &link_type)?;

    if bidireccional {
        if let Some(to) = &to_referenceable {
            zome_file_tree = add_link_type_to_integrity_zome(
                zome_file_tree,
                &link_type_name(&to, &from_referenceable),
            )?;
        }
    }

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

    let zome_file_tree =
        ZomeFileTree::from_zome_manifest(zome_file_tree.dna_file_tree, coordinator_zome.clone())?;

    let app_file_tree = add_link_type_functions_to_coordinator(
        zome_file_tree,
        &integrity_zome_name,
        &link_type,
        &from_referenceable,
        &to_referenceable,
        bidireccional,
    )?;

    scaffold_link_type_templates(
        app_file_tree.dna_file_tree.file_tree(),
        &template_file_tree,
        &dna_manifest.name(),
        &coordinator_zome,
        &from_referenceable,
        &to_referenceable,
        bidireccional,
    )
}
