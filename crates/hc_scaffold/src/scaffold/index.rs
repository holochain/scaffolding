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
    entry_type::{
        definitions::EntryTypeReference,
        integrity::get_all_entry_types,
        utils::{choose_entry_type_reference, choose_reference_entry_hash},
    },
    link_type::integrity::add_link_type_to_integrity_zome,
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
    integrity_zome_file_tree: ZomeFileTree,
    template_file_tree: &FileTree,
    index_name: &String,
    maybe_index_type: &Option<IndexType>,
    maybe_entry_type: &Option<EntryTypeReference>,
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

    let all_entries_names: Vec<String> = all_entries
        .clone()
        .into_iter()
        .map(|e| e.entry_type)
        .collect();
    let entry_type = match maybe_entry_type {
        Some(et) => match !all_entries_names.contains(&et.entry_type) {
            true => Ok(et.clone()),
            false => Err(ScaffoldError::EntryTypeNotFound(
                et.entry_type.clone(),
                integrity_zome_file_tree.dna_file_tree.dna_manifest.name(),
                integrity_zome_file_tree.zome_manifest.name.0.to_string(),
            )),
        },
        None => choose_entry_type_reference(
            &all_entries,
            &"Which entry type should be indexed?".to_string(),
        ),
    }?;

    let link_type_name = index_name.to_case(Case::Pascal);

    let zome_file_tree =
        add_link_type_to_integrity_zome(integrity_zome_file_tree, &link_type_name)?;

    let (dna_file_tree, coordinator_zome) = add_index_to_coordinators(
        zome_file_tree,
        index_name,
        &link_type_name,
        &index_type,
        &entry_type,
    )?;

    let dna_name = dna_file_tree.dna_manifest.name();

    let app_file_tree = scaffold_index_templates(
        dna_file_tree.file_tree(),
        &template_file_tree,
        &dna_name,
        &coordinator_zome,
        &index_type,
        index_name,
        &entry_type,
    )?;

    Ok(app_file_tree)
}
