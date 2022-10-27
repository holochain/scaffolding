use std::{path::PathBuf, str::FromStr};

use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Select};
use holochain_types::prelude::{AppManifest, DnaManifest};
use serde::Serialize;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::FileTree,
};

use self::coordinator::add_index_to_coordinators;

use super::{
    entry_def::{integrity::get_all_entry_types, utils::choose_multiple_entry_types},
    link_type::integrity::add_link_type_to_integrity_zome,
    web_app::uis::scaffold_index_templates,
    zome::utils::get_coordinator_zomes_for_integrity,
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
    app_file_tree: FileTree,
    app_manifest: &(PathBuf, AppManifest),
    dna_manifest: &DnaManifest,
    integrity_zome_name: &String,
    index_name: &String,
    maybe_index_type: &Option<IndexType>,
    maybe_entry_types: &Option<Vec<String>>,
    link_to_entry_hash: bool,
) -> ScaffoldResult<FileTree> {
    let all_entries = get_all_entry_types(
        &app_file_tree,
        &app_manifest.1,
        dna_manifest,
        integrity_zome_name,
    )?
    .ok_or(ScaffoldError::NoEntryTypesDefFoundForIntegrityZome(
        integrity_zome_name.clone(),
        dna_manifest.name(),
    ))?;

    let index_type = match maybe_index_type {
        Some(t) => Ok(t.clone()),
        None => choose_index_type(),
    }?;

    let entry_types = match maybe_entry_types {
        Some(et) => match et.iter().find(|t| !all_entries.contains(t)) {
            Some(t) => Err(ScaffoldError::EntryTypeNotFound(
                t.clone(),
                dna_manifest.name(),
                integrity_zome_name.clone(),
            )),
            None => Ok(et.clone()),
        },
        None => choose_multiple_entry_types(
            &all_entries,
            &"Which entry types should be indexed?".to_string(),
            false,
        ),
    }?;

    let link_type_name = index_name.to_case(Case::Pascal);

    let app_file_tree = add_link_type_to_integrity_zome(
        app_file_tree,
        dna_manifest,
        integrity_zome_name,
        &link_type_name,
    )?;

    let (app_file_tree, coordinator_zome) = add_index_to_coordinators(
        app_file_tree,
        dna_manifest,
        integrity_zome_name,
        index_name,
        &link_type_name,
        &index_type,
        &entry_types,
        link_to_entry_hash,
    )?;

    let app_file_tree = scaffold_index_templates(
        app_file_tree,
        &dna_manifest.name(),
        &coordinator_zome.name.0.to_string(),
        &index_type,
        index_name,
        &entry_types,
    )?;

    Ok(app_file_tree)
}
