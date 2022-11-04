use std::ffi::OsString;

use build_fs_tree::file;
use convert_case::{Case, Casing};
use holochain_types::prelude::{DnaManifest, ZomeManifest};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{insert_file, map_file, FileTree},
    scaffold::{dna::DnaFileTree, zome::ZomeFileTree},
};

fn initial_link_type_handlers(
    integrity_zome_name: &String,
    link_type_name: &String,
    from_entry_type: &String,
    to_entry_type: &Option<String>,
    link_from_entry_hash: bool,
    link_to_entry_hash: bool,
) -> String {
    let to_entry_type = match to_entry_type {
        Some(t) => t,
        None => link_type_name,
    };

    let from_hash_type = match link_from_entry_hash {
        true => String::from("EntryHash"),
        false => String::from("ActionHash"),
    };
    let to_hash_type = match link_to_entry_hash {
        true => String::from("EntryHash"),
        false => String::from("ActionHash"),
    };
    let pascal_from_entry_type = from_entry_type.to_case(Case::Pascal);
    let pascal_to_entry_type = to_entry_type.to_case(Case::Pascal);
    let snake_from_entry_type = from_entry_type.to_case(Case::Snake);
    let snake_to_entry_type = to_entry_type.to_case(Case::Snake);

    format!(
        r#"use hdk::prelude::*;
use {}::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Create{}For{}Input {{
    {}_hash: {},
    {}_hash: {},
}}
#[hdk_extern]
pub fn add_{}_for_{}(input: Create{}For{}Input) -> ExternResult<()> {{
    create_link(input.{}_hash, input.{}_hash, LinkTypes::{}, ())?;

    Ok(())    
}}

#[hdk_extern]
pub fn get_{}_for_{}({}_hash: {}) -> ExternResult<Vec<Record>> {{
    let links = get_links({}_hash, LinkTypes::{}, None)?;
    
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new({}::from(link.target).into(), GetOptions::default()))
        .collect();

    let maybe_records = HDK.with(|hdk| hdk.borrow().get(get_input))?;

    let record: Vec<Record> = maybe_records.into_iter().filter_map(|r| r).collect();

    Ok(record)
}}
"#,
        integrity_zome_name,
        pascal_to_entry_type,
        pascal_from_entry_type,
        snake_from_entry_type,
        from_hash_type,
        snake_to_entry_type,
        to_hash_type,
        snake_to_entry_type,
        snake_from_entry_type,
        pascal_to_entry_type,
        pascal_from_entry_type,
        snake_from_entry_type,
        snake_to_entry_type,
        link_type_name,
        snake_to_entry_type,
        snake_from_entry_type,
        snake_from_entry_type,
        from_hash_type,
        snake_from_entry_type,
        link_type_name,
        to_hash_type
    )
}

pub fn add_link_type_functions_to_coordinator(
    mut coordinator_zome_file_tree: ZomeFileTree,
    integrity_zome_name: &String,
    link_type_name: &String,
    from_entry_type: &String,
    to_entry_type: &Option<String>,
    link_from_entry_hash: bool,
    link_to_entry_hash: bool,
) -> ScaffoldResult<ZomeFileTree> {
    let dna_manifest_path = coordinator_zome_file_tree
        .dna_file_tree
        .dna_manifest_path
        .clone();
    let zome_manifest = coordinator_zome_file_tree.zome_manifest.clone();

    let snake_link_type_name = link_type_name.to_case(Case::Snake);

    let new_file_path = coordinator_zome_file_tree
        .zome_crate_path
        .join("src")
        .join(format!("{}.rs", snake_link_type_name.clone()));
    let lib_rs_path = coordinator_zome_file_tree
        .zome_crate_path
        .join("src")
        .join("lib.rs");

    let mut file_tree = coordinator_zome_file_tree.dna_file_tree.file_tree();

    insert_file(
        &mut file_tree,
        &new_file_path,
        &initial_link_type_handlers(
            integrity_zome_name,
            link_type_name,
            from_entry_type,
            to_entry_type,
            link_from_entry_hash,
            link_to_entry_hash,
        ),
    )?;

    // 2. Add this file as a module in the entry point for the crate

    map_file(&mut file_tree, &lib_rs_path, |file| {
        format!(
            r#"pub mod {};

{}"#,
            snake_link_type_name, file
        )
    })?;

    let dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;
    let zome_file_tree = ZomeFileTree::from_zome_manifest(dna_file_tree, zome_manifest)?;

    Ok(zome_file_tree)
}
