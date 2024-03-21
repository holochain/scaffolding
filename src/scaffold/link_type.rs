use std::path::PathBuf;

use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{insert_file, map_file, FileTree},
    templates::{link_type::scaffold_link_type_templates, ScaffoldedTemplate},
    utils::input_with_case,
};

use self::{
    coordinator::add_link_type_functions_to_coordinator, integrity::add_link_type_to_integrity_zome,
};

use super::{
    app::AppFileTree,
    dna::DnaFileTree,
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
    delete: &Option<bool>,
    bidirectional: &Option<bool>,
    no_ui: bool,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let dna_manifest_path = zome_file_tree.dna_file_tree.dna_manifest_path.clone();
    let zome_manifest = zome_file_tree.zome_manifest.clone();

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
        None => input_with_case(&String::from("Enter link type name:"), Case::Pascal)?,
    };

    let bidirectional = match (&to_referenceable, bidirectional) {
        (None, _) => false,
        (_, Some(b)) => *b,
        (_, None) => Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Should the link be bidirectional?")
            .interact()?,
    };
    let delete = match delete {
        Some(d) => *d,
        None => Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Can the link be deleted?")
            .interact()?,
    };

    // 1. Create an LINK_TYPE_NAME.rs in "src/", with the link type validation
    let crate_src_path = zome_file_tree.zome_crate_path.join("src");

    let link_type_file_name = PathBuf::from(format!("{}.rs", link_type.to_case(Case::Snake)));

    let mut file_tree = zome_file_tree.dna_file_tree.file_tree();

    insert_file(
        &mut file_tree,
        &crate_src_path.join(&link_type_file_name),
        "use hdi::prelude::*;",
    )?;

    // 2. Add this file as a module in the entry point for the crate

    let lib_rs_path = crate_src_path.join("lib.rs");

    map_file(&mut file_tree, &lib_rs_path, |s| {
        Ok(format!(
            r#"pub mod {};
pub use {}::*;

{}"#,
            link_type.to_case(Case::Snake),
            link_type.to_case(Case::Snake),
            s
        ))
    })?;

    let dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;
    let zome_file_tree = ZomeFileTree::from_zome_manifest(dna_file_tree, zome_manifest)?;

    let mut zome_file_tree = add_link_type_to_integrity_zome(
        zome_file_tree,
        &link_type,
        &Some(from_referenceable.clone()),
        &to_referenceable,
        delete,
        &link_type_file_name,
    )?;

    let inverse_link_type = if bidirectional {
        if let Some(to) = &to_referenceable {
            let inverse_link_type = link_type_name(to, &from_referenceable);
            zome_file_tree = add_link_type_to_integrity_zome(
                zome_file_tree,
                &inverse_link_type,
                &to_referenceable,
                &Some(from_referenceable.clone()),
                delete,
                &link_type_file_name,
            )?;
            Some(inverse_link_type)
        } else {
            None
        }
    } else {
        None
    };

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

    let zome_file_tree = add_link_type_functions_to_coordinator(
        zome_file_tree,
        &integrity_zome_name,
        &link_type,
        &from_referenceable,
        &to_referenceable,
        delete,
        bidirectional,
    )?;

    let app_file_tree =
        AppFileTree::get_or_choose(zome_file_tree.dna_file_tree.file_tree(), &None)?;

    let app_name = app_file_tree.app_manifest.app_name().to_string();

    scaffold_link_type_templates(
        app_file_tree.file_tree(),
        template_file_tree,
        &app_name,
        &dna_manifest.name(),
        &coordinator_zome,
        &link_type,
        &from_referenceable,
        &to_referenceable,
        delete,
        &inverse_link_type,
        no_ui,
    )
}
