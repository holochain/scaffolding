use std::ffi::OsString;

use build_fs_tree::file;
use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use holochain_types::prelude::{DnaManifest, ZomeManifest};
use quote::{__private::TokenStream, quote};

use crate::{
    cli::Crud,
    error::{ScaffoldError, ScaffoldResult},
    file_tree::FileTree,
    generators::zome::utils::zome_manifest_path,
};

pub fn read_handler(entry_def_name: &String) -> String {
    format!(
        r#"#[hdk_extern]
pub fn get_{}(action_hash: ActionHash) -> ExternResult<Option<Record>> {{
  get(action_hash, GetOptions::default())
}}
"#,
        entry_def_name.to_case(Case::Snake)
    )
}

pub fn create_handler(entry_def_name: &String) -> String {
    format!(
        r#"#[hdk_extern]
pub fn create_{}({}: {}) -> ExternResult<Record> {{
  let {}_hash = create_entry(&EntryTypes::{}({}.clone()))?;
    
  let record = get({}_hash.clone(), GetOptions::default())?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("Could not find the newly created {}"))))?;

  Ok(record)
}}
"#,
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Pascal),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Pascal),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Pascal)
    )
}

pub fn update_handler(entry_def_name: &String) -> String {
    format!(
        r#"#[derive(Serialize, Deserialize, Debug)]
pub struct Update{}Input {{
  original_action_hash: ActionHash,
  updated_{}: {}
}}

#[hdk_extern]
pub fn update_{}(input: Update{}Input) -> ExternResult<Record> {{
  let updated_{}_hash = update_entry(input.original_action_hash, &input.updated_{})?;

  let record = get(updated_{}_hash.clone(), GetOptions::default())?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("Could not find the newly updated {}"))))?;
    
  Ok(record)
}}
"#,
        entry_def_name.to_case(Case::Pascal),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Pascal),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Pascal),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Pascal)
    )
}

pub fn delete_handler(entry_def_name: &String) -> String {
    format!(
        r#"#[hdk_extern]
pub fn delete_{}(action_hash: ActionHash) -> ExternResult<ActionHash> {{
  delete_entry(action_hash)
}}
"#,
        entry_def_name.to_case(Case::Snake)
    )
}

fn initial_crud_handlers(
    integrity_zome_name: &String,
    entry_def_name: &String,
    crud: &Crud,
) -> String {
    let mut initial = format!(
        r#"use hdk::prelude::*;
use {}::*;

{}
"#,
        integrity_zome_name,
        create_handler(entry_def_name)
    );

    if crud.read {
        initial.push_str(read_handler(entry_def_name).as_str());
    }
    if crud.update {
        initial.push_str(update_handler(entry_def_name).as_str());
    }
    if crud.delete {
        initial.push_str(delete_handler(entry_def_name).as_str());
    }

    initial
}

pub fn add_crud_functions_to_coordinator(
    mut app_file_tree: FileTree,
    dna_manifest: &DnaManifest,
    integrity_zome_name: &String,
    coordinator_zome: &ZomeManifest,
    entry_def_name: &String,
    crud: &Crud,
) -> ScaffoldResult<FileTree> {
    // 1. Create an ENTRY_DEF_NAME.rs in "src/", with the appropriate crud functions
    let mut manifest_path = zome_manifest_path(&app_file_tree, &coordinator_zome)?.ok_or(
        ScaffoldError::CoordinatorZomeNotFound(
            coordinator_zome.name.0.to_string(),
            dna_manifest.name(),
        ),
    )?;

    manifest_path.pop();

    let snake_entry_def_name = entry_def_name.to_case(Case::Snake);

    let crate_src_path = manifest_path.join("src");
    let crate_src_path_iter: Vec<OsString> =
        crate_src_path.iter().map(|s| s.to_os_string()).collect();
    app_file_tree
        .path_mut(&mut crate_src_path_iter.iter())
        .ok_or(ScaffoldError::PathNotFound(crate_src_path.clone()))?
        .dir_content_mut()
        .ok_or(ScaffoldError::PathNotFound(crate_src_path.clone()))?
        .insert(
            OsString::from(format!("{}.rs", snake_entry_def_name.clone())),
            file!(initial_crud_handlers(
                integrity_zome_name,
                entry_def_name,
                crud
            )),
        );

    // 2. Add this file as a module in the entry point for the crate

    let lib_rs_path = crate_src_path.join("lib.rs");
    let v: Vec<OsString> = lib_rs_path.iter().map(|s| s.to_os_string()).collect();
    app_file_tree
        .path_mut(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(lib_rs_path.clone()))?
        .file_content_mut()
        .ok_or(ScaffoldError::PathNotFound(lib_rs_path.clone()))?
        .insert_str(
            0,
            format!(
                r#"pub mod {};
 "#,
                snake_entry_def_name,
            )
            .as_str(),
        );

    Ok(app_file_tree)
}
