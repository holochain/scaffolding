use std::{path::PathBuf, str::FromStr};

use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::Serialize;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::FileTree,
    reserved_words::check_for_reserved_words,
    templates::{collection::scaffold_collection_templates, ScaffoldedTemplate},
};

use self::coordinator::add_collection_to_coordinators;

use super::{
    app::AppFileTree,
    entry_type::{
        definitions::{EntryTypeReference, Referenceable},
        integrity::get_all_entry_types,
        utils::choose_entry_type_reference,
    },
    link_type::integrity::add_link_type_to_integrity_zome,
    zome::ZomeFileTree,
};

pub mod coordinator;

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum CollectionType {
    Global,
    ByAuthor,
}

impl FromStr for CollectionType {
    type Err = ScaffoldError;
    fn from_str(s: &str) -> ScaffoldResult<Self> {
        match s {
            "global" => Ok(CollectionType::Global),
            "by-author" => Ok(CollectionType::ByAuthor),
            _ => Err(ScaffoldError::InvalidCollectionType(
                s.to_string(),
                "global, by-author".to_string(),
            )),
        }
    }
}

pub fn choose_collection_type() -> ScaffoldResult<CollectionType> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which type of collection should be scaffolded?")
        .default(0)
        .item("Global (get all entries of the selected entry types)")
        .item("By author (get entries of the selected entry types that a given author has created)")
        .interact()?;
    match selection {
        0 => Ok(CollectionType::Global),
        1 => Ok(CollectionType::ByAuthor),
        _ => Err(ScaffoldError::InvalidCollectionType(
            selection.to_string(),
            "".into(),
        )),
    }
}

pub fn scaffold_collection(
    integrity_zome_file_tree: ZomeFileTree,
    template_file_tree: &FileTree,
    collection_name: &String,
    maybe_collection_type: &Option<CollectionType>,
    maybe_entry_type: &Option<EntryTypeReference>,
    no_ui: bool,
) -> ScaffoldResult<ScaffoldedTemplate> {
    check_for_reserved_words(collection_name)?;

    let all_entries = get_all_entry_types(&integrity_zome_file_tree)?.ok_or(
        ScaffoldError::NoEntryTypesDefFoundForIntegrityZome(
            integrity_zome_file_tree.dna_file_tree.dna_manifest.name(),
            integrity_zome_file_tree.zome_manifest.name.0.to_string(),
        ),
    )?;

    let collection_type = match maybe_collection_type {
        Some(t) => Ok(t.clone()),
        None => choose_collection_type(),
    }?;

    let all_entries_names: Vec<String> = all_entries
        .clone()
        .into_iter()
        .map(|e| e.entry_type)
        .collect();
    let entry_type = match maybe_entry_type {
        Some(et) => match all_entries_names.contains(&et.entry_type.to_case(Case::Pascal)) {
            true => Ok(et.clone()),
            false => Err(ScaffoldError::EntryTypeNotFound(
                et.entry_type.clone(),
                integrity_zome_file_tree.dna_file_tree.dna_manifest.name(),
                integrity_zome_file_tree.zome_manifest.name.0.to_string(),
            )),
        },
        None => choose_entry_type_reference(
            &all_entries,
            &"Which entry type should be collected?".to_string(),
        ),
    }?;

    let link_type_name = collection_name.to_case(Case::Pascal);

    let zome_file_tree = add_link_type_to_integrity_zome(
        integrity_zome_file_tree,
        &link_type_name,
        &None,
        &Some(Referenceable::EntryType(entry_type.clone())),
        true,
        &PathBuf::from(format!("{}.rs", entry_type.entry_type.to_case(Case::Snake))),
    )?;

    let (dna_file_tree, coordinator_zome, deletable) = add_collection_to_coordinators(
        zome_file_tree,
        collection_name,
        &link_type_name,
        &collection_type,
        &entry_type,
    )?;

    let dna_name = dna_file_tree.dna_manifest.name();

    let app_file_tree = AppFileTree::get_or_choose(dna_file_tree.file_tree(), &None)?;

    let app_name = app_file_tree.app_manifest.app_name().to_string();

    scaffold_collection_templates(
        app_file_tree.file_tree(),
        template_file_tree,
        &app_name,
        &dna_name,
        &coordinator_zome,
        &collection_type,
        collection_name,
        &entry_type,
        deletable,
        no_ui,
    )
}
