use convert_case::{Case, Casing};

use crate::{
    error::ScaffoldResult,
    file_tree::{insert_file, map_file},
    scaffold::{dna::DnaFileTree, link_type::link_type_name, zome::ZomeFileTree},
};

use super::crud::Crud;

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

pub fn create_handler(entry_def_name: &String, depends_on: &Vec<String>) -> String {
    let snake_entry_type = entry_def_name.to_case(Case::Snake);

    let create_links_str = depends_on
        .iter()
        .map(|s| {
            format!(
                r#"  create_link({}.{}_hash.clone(), {}_hash.clone(), LinkTypes::{}, ())?;"#,
                snake_entry_type,
                s.to_case(Case::Snake),
                snake_entry_type,
                link_type_name(s, entry_def_name)
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        r#"#[hdk_extern]
pub fn create_{}({}: {}) -> ExternResult<Record> {{
  let {}_hash = create_entry(&EntryTypes::{}({}.clone()))?;
{}
    
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
        create_links_str,
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

fn depends_on_handler(plural_name: &String, depends_on: &String) -> String {
    format!(
        r#"
#[hdk_extern]
pub fn get_{}_for_{}({}_hash: ActionHash) -> ExternResult<Vec<Record>> {{
    let links = get_links({}_hash, LinkTypes::{}, None)?;
    
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(ActionHash::from(link.target).into(), GetOptions::default()))
        .collect();

    let maybe_records = HDK.with(|hdk| hdk.borrow().get(get_input))?;

    let record: Vec<Record> = maybe_records.into_iter().filter_map(|r| r).collect();

    Ok(record)
}}"#,
        plural_name.to_case(Case::Snake),
        depends_on.to_case(Case::Snake),
        depends_on.to_case(Case::Snake),
        depends_on.to_case(Case::Snake),
        link_type_name(depends_on, &plural_name.to_case(Case::Pascal)),
    )
}

fn initial_crud_handlers(
    integrity_zome_name: &String,
    singular_name: &String,
    plural_name: &String,
    crud: &Crud,
    depends_on: &Vec<String>,
) -> String {
    let mut initial = format!(
        r#"use hdk::prelude::*;
use {}::*;

{}
"#,
        integrity_zome_name,
        create_handler(singular_name, depends_on)
    );

    if crud.read {
        initial.push_str(read_handler(singular_name).as_str());
    }
    if crud.update {
        initial.push_str(update_handler(singular_name).as_str());
    }
    if crud.delete {
        initial.push_str(delete_handler(singular_name).as_str());
    }

    for d in depends_on {
        initial.push_str(depends_on_handler(plural_name, d).as_str());
    }

    initial
}

pub fn add_crud_functions_to_coordinator(
    mut zome_file_tree: ZomeFileTree,
    integrity_zome_name: &String,
    singular_name: &String,
    plural_name: &String,
    crud: &Crud,
    depends_on: &Vec<String>,
) -> ScaffoldResult<ZomeFileTree> {
    let dna_manifest_path = zome_file_tree.dna_file_tree.dna_manifest_path.clone();
    let zome_manifest = zome_file_tree.zome_manifest.clone();

    // 1. Create an ENTRY_DEF_NAME.rs in "src/", with the appropriate crud functions
    let crate_src_path = zome_file_tree.zome_crate_path.join("src");

    let mut file_tree = zome_file_tree.dna_file_tree.file_tree();
    insert_file(
        &mut file_tree,
        &crate_src_path.join(format!("{}.rs", singular_name.to_case(Case::Snake))),
        &initial_crud_handlers(
            integrity_zome_name,
            singular_name,
            plural_name,
            crud,
            depends_on,
        ),
    )?;

    // 2. Add this file as a module in the entry point for the crate

    let lib_rs_path = crate_src_path.join("lib.rs");

    map_file(&mut file_tree, &lib_rs_path, |s| {
        format!(
            r#"pub mod {};

{}"#,
            singular_name.to_case(Case::Snake),
            s
        )
    })?;

    let dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;
    let zome_file_tree = ZomeFileTree::from_zome_manifest(dna_file_tree, zome_manifest)?;

    Ok(zome_file_tree)
}
