use std::ffi::OsString;

use build_fs_tree::file;
use convert_case::{Case, Casing};
use holochain_types::prelude::{DnaManifest, ZomeManifest};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{map_file, FileTree},
    generators::zome::utils::zome_manifest_path,
};

fn initial_link_type_handlers(
    integrity_zome_name: &String,
    link_type_name: &String,
    from_entry_type: &String,
    to_entry_type: &String,
    link_from_entry_hash: bool,
    link_to_entry_hash: bool,
) -> String {
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
    mut app_file_tree: FileTree,
    dna_manifest: &DnaManifest,
    integrity_zome_name: &String,
    coordinator_zome: &ZomeManifest,
    link_type_name: &String,
    from_entry_type: &String,
    to_entry_type: &String,
    link_from_entry_hash: bool,
    link_to_entry_hash: bool,
) -> ScaffoldResult<FileTree> {
    // 1. Create an LINK_TYPE_NAME.rs in "src/", with the appropriate zome functions
    let mut manifest_path = zome_manifest_path(&app_file_tree, &coordinator_zome)?.ok_or(
        ScaffoldError::CoordinatorZomeNotFound(
            coordinator_zome.name.0.to_string(),
            dna_manifest.name(),
        ),
    )?;

    manifest_path.pop();

    let snake_link_type_name = link_type_name.to_case(Case::Snake);

    let crate_src_path = manifest_path.join("src");
    let crate_src_path_iter: Vec<OsString> =
        crate_src_path.iter().map(|s| s.to_os_string()).collect();
    app_file_tree
        .path_mut(&mut crate_src_path_iter.iter())
        .ok_or(ScaffoldError::PathNotFound(crate_src_path.clone()))?
        .dir_content_mut()
        .ok_or(ScaffoldError::PathNotFound(crate_src_path.clone()))?
        .insert(
            OsString::from(format!("{}.rs", snake_link_type_name.clone())),
            file!(initial_link_type_handlers(
                integrity_zome_name,
                link_type_name,
                from_entry_type,
                to_entry_type,
                link_from_entry_hash,
                link_to_entry_hash
            )),
        );

    // 2. Add this file as a module in the entry point for the crate

    let lib_rs_path = crate_src_path.join("lib.rs");

    map_file(&mut app_file_tree, &lib_rs_path, |file| {
        format!(
            r#"pub mod {};

{}"#,
            snake_link_type_name, file
        )
    })?;

    Ok(app_file_tree)
}
