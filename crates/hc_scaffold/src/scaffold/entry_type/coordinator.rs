use convert_case::{Case, Casing};

use crate::{
    definitions::{Cardinality, EntryDefinition},
    error::ScaffoldResult,
    file_tree::{insert_file, map_file},
    scaffold::{dna::DnaFileTree, link_type::link_type_name, zome::ZomeFileTree},
};

use super::{crud::Crud, depends_on_field_name, depends_on_itself_field_name};

pub fn no_update_read_handler(entry_def_name: &String) -> String {
    format!(
        r#"#[hdk_extern]
pub fn get_{}({}_hash: ActionHash) -> ExternResult<Option<Record>> {{
  get({}_hash, GetOptions::default())
}}"#,
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake)
    )
}

pub fn read_handler_without_linking_to_updates(entry_def_name: &String) -> String {
    format!(
        r#"#[hdk_extern]
pub fn get_{}(original_{}_hash: ActionHash) -> ExternResult<Option<Record>> {{
  get_latest_{}(original_{}_hash)
}}

fn get_latest_{}({}_hash: ActionHash) -> ExternResult<Option<Record>> {{
  let details = get_details({}_hash, GetOptions::default())?
      .ok_or(wasm_error!(WasmErrorInner::Guest("{} not found".into())))?;

  let record_details = match details {{
    Details::Entry(_) => Err(wasm_error!(WasmErrorInner::Guest(
      "Malformed details".into()
    ))),
    Details::Record(record_details) => Ok(record_details)
  }}?;

  // If there is some delete action, it means that the whole entry is deleted
  if record_details.deletes.len() > 0 {{
    return Ok(None);
  }}
    
  match record_details.updates.last() {{
    Some(update) => get_latest_{}(update.action_address().clone()),
    None => Ok(Some(record_details.record)),
  }}
}}
"#,
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Pascal),
        entry_def_name.to_case(Case::Snake)
    )
}

pub fn updates_link_name(entry_def_name: &String) -> String {
    format!("{}Updates", entry_def_name.to_case(Case::Pascal))
}

pub fn read_handler_with_linking_to_updates(entry_def_name: &String) -> String {
    format!(
        r#"#[hdk_extern]
pub fn get_{}(original_{}_hash: ActionHash) -> ExternResult<Option<Record>> {{
  let links = get_links(original_{}_hash, LinkTypes::{}, None)?;

  let latest_link = links.into_iter().max_by(|link_a, link_b| link_b.timestamp.cmp(&link_a.timestamp));
  
  let latest_{}_hash = match latest_link {{
    Some(link) => ActionHash::from(link.target.clone()),
    None => original_{}_hash.clone()   
  }};
 
  get(latest_{}_hash, GetOptions::default())
}}
"#,
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        updates_link_name(entry_def_name),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
    )
}

pub fn create_link_for_cardinality(
    entry_def: &EntryDefinition,
    field_name: &String,
    link_type_name: &String,
    cardinality: &Cardinality,
) -> String {
    match cardinality {
        Cardinality::Single => format!(
            r#"  create_link({}.{}.clone(), {}_hash.clone(), LinkTypes::{}, ())?;"#,
            entry_def.singular_name.to_case(Case::Snake),
            field_name,
            entry_def.singular_name.to_case(Case::Snake),
            link_type_name
        ),
        Cardinality::Option => format!(
            r#"  if let Some(action_hash) = {}.{}.clone() {{
    create_link(action_hash, {}_hash.clone(), LinkTypes::{}, ())?;
  }}"#,
            entry_def.singular_name.to_case(Case::Snake),
            field_name,
            entry_def.singular_name.to_case(Case::Snake),
            link_type_name
        ),
        Cardinality::Vector => format!(
            r#"  for action_hash in {}.{}.clone() {{
    create_link(action_hash, {}_hash.clone(), LinkTypes::{}, ())?;
  }}"#,
            entry_def.singular_name.to_case(Case::Snake),
            field_name,
            entry_def.singular_name.to_case(Case::Snake),
            link_type_name
        ),
    }
}

pub fn create_handler(entry_def: &EntryDefinition) -> String {
    let mut create_links_str = entry_def
        .depends_on
        .iter()
        .map(|d| {
            create_link_for_cardinality(
                entry_def,
                &depends_on_field_name(d),
                &link_type_name(&d.entry_type, &entry_def.plural_name),
                &d.cardinality,
            )
        })
        .collect::<Vec<String>>();

    if let Some(c) = &entry_def.depends_on_itself {
        create_links_str.push(create_link_for_cardinality(
            entry_def,
            &depends_on_itself_field_name(&entry_def.singular_name, &entry_def.plural_name, c),
            &link_type_name(&entry_def.singular_name, &entry_def.plural_name),
            &c.clone().into(),
        ));
    }

    let create_links_str = create_links_str.join("\n\n");

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
        entry_def.singular_name.to_case(Case::Snake),
        entry_def.singular_name.to_case(Case::Snake),
        entry_def.singular_name.to_case(Case::Pascal),
        entry_def.singular_name.to_case(Case::Snake),
        entry_def.singular_name.to_case(Case::Pascal),
        entry_def.singular_name.to_case(Case::Snake),
        create_links_str,
        entry_def.singular_name.to_case(Case::Snake),
        entry_def.singular_name.to_case(Case::Pascal)
    )
}

pub fn update_handler(entry_def_name: &String, link_from_original_to_each_update: bool) -> String {
    match link_from_original_to_each_update {
        true => update_handler_linking_on_each_update(entry_def_name),
        false => update_handler_without_linking_on_each_update(entry_def_name),
    }
}

pub fn update_handler_without_linking_on_each_update(entry_def_name: &String) -> String {
    format!(
        r#"#[derive(Serialize, Deserialize, Debug)]
pub struct Update{}Input {{
  pub previous_{}_hash: ActionHash,
  pub updated_{}: {}
}}

#[hdk_extern]
pub fn update_{}(input: Update{}Input) -> ExternResult<Record> {{
  let updated_{}_hash = update_entry(input.previous_{}_hash, &input.updated_{})?;

  let record = get(updated_{}_hash.clone(), GetOptions::default())?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("Could not find the newly updated {}"))))?;
    
  Ok(record)
}}
"#,
        entry_def_name.to_case(Case::Pascal),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Pascal),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Pascal),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Pascal)
    )
}

pub fn update_handler_linking_on_each_update(entry_def_name: &String) -> String {
    format!(
        r#"#[derive(Serialize, Deserialize, Debug)]
pub struct Update{}Input {{
  pub original_{}_hash: ActionHash,
  pub previous_{}_hash: ActionHash,
  pub updated_{}: {}
}}

#[hdk_extern]
pub fn update_{}(input: Update{}Input) -> ExternResult<Record> {{
  let updated_{}_hash = update_entry(input.previous_{}_hash, &input.updated_{})?;
        
  create_link(input.original_{}_hash.clone(), input.previous_{}_hash, LinkTypes::{}, ())?;

  let record = get(updated_{}_hash.clone(), GetOptions::default())?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("Could not find the newly updated {}"))))?;
    
  Ok(record)
}}
"#,
        entry_def_name.to_case(Case::Pascal),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Pascal),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Pascal),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
        updates_link_name(entry_def_name),
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Pascal)
    )
}

pub fn delete_handler(entry_def_name: &String) -> String {
    format!(
        r#"#[hdk_extern]
pub fn delete_{}(original_{}_hash: ActionHash) -> ExternResult<ActionHash> {{
  delete_entry(original_{}_hash)
}}
"#,
        entry_def_name.to_case(Case::Snake),
        entry_def_name.to_case(Case::Snake),
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

fn depends_on_itself_handler(singular_name: &String, plural_name: &String) -> String {
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
        singular_name.to_case(Case::Snake),
        singular_name.to_case(Case::Snake),
        singular_name.to_case(Case::Snake),
        link_type_name(
            &singular_name.to_case(Case::Pascal),
            &plural_name.to_case(Case::Pascal)
        ),
    )
}

fn initial_crud_handlers(
    integrity_zome_name: &String,
    entry_def: &EntryDefinition,
    crud: &Crud,
    link_from_original_to_each_update: bool,
) -> String {
    let mut initial = format!(
        r#"use hdk::prelude::*;
use {}::*;

{}
"#,
        integrity_zome_name,
        create_handler(entry_def)
    );

    if !crud.update {
        initial.push_str(no_update_read_handler(&entry_def.singular_name).as_str());
    } else {
        if link_from_original_to_each_update {
            initial
                .push_str(read_handler_with_linking_to_updates(&entry_def.singular_name).as_str());
        } else {
            initial.push_str(
                read_handler_without_linking_to_updates(&entry_def.singular_name).as_str(),
            );
        }
    }
    if crud.update {
        initial.push_str(
            update_handler(&entry_def.singular_name, link_from_original_to_each_update).as_str(),
        );
    }
    if crud.delete {
        initial.push_str(delete_handler(&entry_def.singular_name).as_str());
    }

    for d in &entry_def.depends_on {
        initial.push_str(depends_on_handler(&entry_def.plural_name, &d.entry_type).as_str());
    }
    if let Some(_cardinality) = &entry_def.depends_on_itself {
        initial.push_str(
            depends_on_itself_handler(&entry_def.singular_name, &entry_def.plural_name).as_str(),
        );
    }

    initial
}

pub fn add_crud_functions_to_coordinator(
    zome_file_tree: ZomeFileTree,
    integrity_zome_name: &String,
    entry_def: &EntryDefinition,
    crud: &Crud,
    link_from_original_to_each_update: bool,
) -> ScaffoldResult<ZomeFileTree> {
    let dna_manifest_path = zome_file_tree.dna_file_tree.dna_manifest_path.clone();
    let zome_manifest = zome_file_tree.zome_manifest.clone();

    // 1. Create an ENTRY_DEF_NAME.rs in "src/", with the appropriate crud functions
    let crate_src_path = zome_file_tree.zome_crate_path.join("src");

    let mut file_tree = zome_file_tree.dna_file_tree.file_tree();
    insert_file(
        &mut file_tree,
        &crate_src_path.join(format!(
            "{}.rs",
            entry_def.singular_name.to_case(Case::Snake)
        )),
        &initial_crud_handlers(
            integrity_zome_name,
            &entry_def,
            crud,
            link_from_original_to_each_update,
        ),
    )?;

    // 2. Add this file as a module in the entry point for the crate

    let lib_rs_path = crate_src_path.join("lib.rs");

    map_file(&mut file_tree, &lib_rs_path, |s| {
        format!(
            r#"pub mod {};

{}"#,
            entry_def.singular_name.to_case(Case::Snake),
            s
        )
    })?;

    let dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;
    let zome_file_tree = ZomeFileTree::from_zome_manifest(dna_file_tree, zome_manifest)?;

    Ok(zome_file_tree)
}
