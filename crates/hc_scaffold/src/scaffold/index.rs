use std::str::FromStr;

use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::Serialize;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::FileTree,
    templates::index::scaffold_index_templates,
};

use self::coordinator::add_index_to_coordinators;

use super::{
    entry_type::{integrity::get_all_entry_types, utils::choose_multiple_entry_types},
    link_type::{choose_use_entry_hash, integrity::add_link_type_to_integrity_zome},
    zome::ZomeFileTree,
};

pub mod coordinator;

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum IndexType {
    Global,
    ByAuthor,
}

impl FromStr for IndexType {
    type Err = ScaffoldError;
    fn from_str(s: &str) -> ScaffoldResult<Self> {
        match s {
            "global" => Ok(IndexType::Global),
            "by-author" => Ok(IndexType::ByAuthor),
            _ => Err(ScaffoldError::InvalidIndexType(
                s.to_string(),
                "global, by-author".to_string(),
            )),
        }
    }
}

pub fn choose_index_type() -> ScaffoldResult<IndexType> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Which type of index should be scaffolded?")
        .default(0)
        .item("Global (get all entries of the selected entry types)")
        .item("By author (get entries of the selected entry types that a given author has created)")
        .interact()?;
    match selection {
        0 => Ok(IndexType::Global),
        1 => Ok(IndexType::ByAuthor),
        _ => Err(ScaffoldError::InvalidIndexType(
            selection.to_string(),
            "".into(),
        )),
    }
}

pub fn scaffold_index(
    mut integrity_zome_file_tree: ZomeFileTree,
    template_file_tree: &FileTree,
    index_name: &String,
    maybe_index_type: &Option<IndexType>,
    maybe_entry_types: &Option<Vec<String>>,
    link_to_entry_hash: &Option<bool>,
) -> ScaffoldResult<FileTree> {
    let all_entries = get_all_entry_types(&integrity_zome_file_tree)?.ok_or(
        ScaffoldError::NoEntryTypesDefFoundForIntegrityZome(
            integrity_zome_file_tree.dna_file_tree.dna_manifest.name(),
            integrity_zome_file_tree.zome_manifest.name.0.to_string(),
        ),
    )?;

    let index_type = match maybe_index_type {
        Some(t) => Ok(t.clone()),
        None => choose_index_type(),
    }?;

    let entry_types = match maybe_entry_types {
        Some(et) => match et.iter().find(|t| !all_entries.contains(t)) {
            Some(t) => Err(ScaffoldError::EntryTypeNotFound(
                t.clone(),
                integrity_zome_file_tree.dna_file_tree.dna_manifest.name(),
                integrity_zome_file_tree.zome_manifest.name.0.to_string(),
            )),
            None => Ok(et.clone()),
        },
        None => choose_multiple_entry_types(
            &all_entries,
            &"Which entry types should be indexed?".to_string(),
            false,
        ),
    }?;

    let link_to_entry_hash: bool = match link_to_entry_hash {
        Some(l) => l.clone(),
        None => choose_use_entry_hash(&String::from("Link to the entry hash or the action hash?"))?,
    };

    let link_type_name = index_name.to_case(Case::Pascal);

    let zome_file_tree =
        add_link_type_to_integrity_zome(integrity_zome_file_tree, &link_type_name)?;

    let (dna_file_tree, coordinator_zome) = add_index_to_coordinators(
        zome_file_tree,
        index_name,
        &link_type_name,
        &index_type,
        &entry_types,
        link_to_entry_hash,
    )?;

    let dna_name = dna_file_tree.dna_manifest.name();

    let app_file_tree = scaffold_index_templates(
        dna_file_tree.file_tree(),
        &template_file_tree,
        &dna_name,
        &coordinator_zome,
        &index_type,
        index_name,
        &entry_types,
    )?;

    Ok(app_file_tree)
}
