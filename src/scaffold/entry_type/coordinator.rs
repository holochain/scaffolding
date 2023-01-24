use std::{ffi::OsString, path::PathBuf};

use convert_case::{Case, Casing};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{insert_file, map_file, map_rust_files},
    scaffold::{
        dna::DnaFileTree,
        link_type::{coordinator::get_links_handler, link_type_name},
        zome::ZomeFileTree,
    },
};

use super::{
    crud::Crud,
    definitions::{Cardinality, EntryDefinition},
    integrity::find_ending_match_expr_in_block,
};

pub fn no_update_read_handler(entry_def: &EntryDefinition) -> String {
    let hash_type = entry_def.referenceable().hash_type().to_string();
    let snake_entry_def = entry_def.name.to_case(Case::Snake);

    format!(
        r#"#[hdk_extern]
pub fn get_{snake_entry_def}({snake_entry_def}_hash: {hash_type}) -> ExternResult<Option<Record>> {{
  get({snake_entry_def}_hash, GetOptions::default())
}}"#,
    )
}

pub fn read_handler_without_linking_to_updates(entry_def: &EntryDefinition) -> String {
    let entry_def_name = entry_def.name.clone();

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
  let links = get_links(original_{}_hash.clone(), LinkTypes::{}, None)?;

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
    let link_target = match entry_def.reference_entry_hash {
        true => format!("{}_entry_hash", entry_def.name.to_case(Case::Snake)),
        false => format!("{}_hash", entry_def.name.to_case(Case::Snake)),
    };

    match cardinality {
        Cardinality::Single => format!(
            r#"  create_link({}.{}.clone(), {}.clone(), LinkTypes::{}, ())?;"#,
            entry_def.name.to_case(Case::Snake),
            field_name,
            link_target,
            link_type_name
        ),
        Cardinality::Option => format!(
            r#"  if let Some(base) = {}.{}.clone() {{
    create_link(base, {}.clone(), LinkTypes::{}, ())?;
  }}"#,
            entry_def.name.to_case(Case::Snake),
            field_name,
            link_target,
            link_type_name
        ),
        Cardinality::Vector => format!(
            r#"  for base in {}.{}.clone() {{
    create_link(base, {}.clone(), LinkTypes::{}, ())?;
  }}"#,
            entry_def.name.to_case(Case::Snake),
            field_name,
            link_target,
            link_type_name
        ),
    }
}

pub fn create_handler(entry_def: &EntryDefinition) -> String {
    let linked_from_count = entry_def
        .fields
        .iter()
        .filter(|f| f.linked_from.is_some())
        .count();
    let mut create_links_str: Vec<String> = match entry_def.reference_entry_hash {
        true if linked_from_count > 0 => vec![format!(
            r#"let {}_entry_hash = hash_entry(&{})?;"#,
            entry_def.name.to_case(Case::Snake),
            entry_def.name.to_case(Case::Snake)
        )],
        _ => vec![],
    };

    for f in &entry_def.fields {
        if let Some(linked_from) = &f.linked_from {
            create_links_str.push(create_link_for_cardinality(
                entry_def,
                &f.field_name,
                &link_type_name(&linked_from, &entry_def.referenceable()),
                &f.cardinality,
            ));
        }
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
        entry_def.name.to_case(Case::Snake),
        entry_def.name.to_case(Case::Snake),
        entry_def.name.to_case(Case::Pascal),
        entry_def.name.to_case(Case::Snake),
        entry_def.name.to_case(Case::Pascal),
        entry_def.name.to_case(Case::Snake),
        create_links_str,
        entry_def.name.to_case(Case::Snake),
        entry_def.name.to_case(Case::Pascal)
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
  let updated_{}_hash = update_entry(input.previous_{}_hash.clone(), &input.updated_{})?;
        
  create_link(input.original_{}_hash.clone(), updated_{}_hash.clone(), LinkTypes::{}, ())?;

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
        initial.push_str(no_update_read_handler(entry_def).as_str());
    } else {
        if link_from_original_to_each_update {
            initial.push_str(read_handler_with_linking_to_updates(&entry_def.name).as_str());
        } else {
            initial.push_str(read_handler_without_linking_to_updates(&entry_def).as_str());
        }
    }
    if crud.update {
        initial
            .push_str(update_handler(&entry_def.name, link_from_original_to_each_update).as_str());
    }
    if crud.delete {
        initial.push_str(delete_handler(&entry_def.name).as_str());
    }

    for f in &entry_def.fields {
        if let Some(linked_from) = &f.linked_from {
            initial.push_str(get_links_handler(&linked_from, &entry_def.referenceable()).as_str());
        }
    }

    initial
}

fn signal_has_entry_types(signal_enum: &syn::ItemEnum) -> bool {
    signal_enum
        .variants
        .iter()
        .find(|v| v.ident.to_string().eq(&String::from("EntryCreated")))
        .is_some()
}

fn signal_action_has_entry_types(expr_match: &syn::ExprMatch) -> bool {
    expr_match
        .arms
        .iter()
        .find(|arm| {
            if let syn::Pat::TupleStruct(tuple_struct_pat) = &arm.pat {
                if let Some(first_segment) = tuple_struct_pat.path.segments.last() {
                    if first_segment.ident.to_string().eq(&String::from("Create")) {
                        return true;
                    }
                }
            }
            false
        })
        .is_some()
}

fn signal_entry_types_variants() -> ScaffoldResult<Vec<syn::Variant>> {
    Ok(vec![
        syn::parse_str::<syn::Variant>(
            "EntryCreated {
        action: SignedActionHashed,
        app_entry: EntryTypes,
    }",
        )?,
        syn::parse_str::<syn::Variant>(
            "EntryUpdated {
        action: SignedActionHashed,
        app_entry: EntryTypes,
        original_app_entry: EntryTypes,
    }",
        )?,
        syn::parse_str::<syn::Variant>(
            "EntryDeleted {
        action: SignedActionHashed,
        original_app_entry: EntryTypes,
    }",
        )?,
    ])
}

fn signal_action_match_arms() -> ScaffoldResult<Vec<syn::Arm>> {
    Ok(vec![
        syn::parse_str::<syn::Arm>("Action::Create(_create) => {
            let app_entry = get_entry_for_action(&action.hashed.hash)?.ok_or(wasm_error!(WasmErrorInner::Guest(\"Create should carry an entry\".to_string())))?;
            emit_signal(Signal::EntryCreated {
                action,
                app_entry
            })?;
            Ok(())
        }")?,
        syn::parse_str::<syn::Arm>("Action::Update(update) => {
            let app_entry = get_entry_for_action(&action.hashed.hash)?.ok_or(wasm_error!(WasmErrorInner::Guest(\"Update should carry an entry\".to_string())))?;
            let original_app_entry = get_entry_for_action(&update.original_action_address)?.ok_or(wasm_error!(WasmErrorInner::Guest(\"Updated action should carry an entry\".to_string())))?;
            emit_signal(Signal::EntryUpdated {
                action,
                app_entry,
                original_app_entry
            })?;
            Ok(())
        }")?,
        syn::parse_str::<syn::Arm>("Action::Delete(delete) => {
            let original_app_entry = get_entry_for_action(&delete.deletes_address)?.ok_or(wasm_error!(WasmErrorInner::Guest(\"Deleted action should carry an entry\".to_string())))?;
            emit_signal(Signal::EntryDeleted {
                action,
                original_app_entry,
            })?;
            Ok(())
        }")?
    ])
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
        &crate_src_path.join(format!("{}.rs", entry_def.name.to_case(Case::Snake))),
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
            entry_def.name.to_case(Case::Snake),
            s
        )
    })?;

    let v: Vec<OsString> = crate_src_path
        .clone()
        .iter()
        .map(|s| s.to_os_string())
        .collect();
    map_rust_files(
        file_tree
            .path_mut(&mut v.iter())
            .ok_or(ScaffoldError::PathNotFound(crate_src_path.clone()))?,
        |file_path, mut file| {
            if file_path == PathBuf::from("lib.rs") {
                let mut first_entry_type_scaffolded = false;

                for item in &mut file.items {
                    if let syn::Item::Enum(item_enum) = item {
                        if item_enum.ident.to_string().eq(&String::from("Signal")) {
                            if !signal_has_entry_types(item_enum) {
                                first_entry_type_scaffolded = true;
                                for v in signal_entry_types_variants()? {
                                    item_enum.variants.push(v);
                                }
                            }
                        }
                    }

                    if let syn::Item::Fn(item_fn) = item {
                        if item_fn
                            .sig
                            .ident
                            .to_string()
                            .eq(&String::from("signal_action"))
                        {
                            if let None = find_ending_match_expr_in_block(&mut item_fn.block) {
                                item_fn.block = Box::new(syn::parse_str::<syn::Block>(
                                    "{ match action.hashed.content.clone() { _ => Ok(()) } }",
                                )?);
                            }

                            if let Some(expr_match) =
                                find_ending_match_expr_in_block(&mut item_fn.block)
                            {
                                if !signal_action_has_entry_types(expr_match) {
                                    for arm in signal_action_match_arms()? {
                                        expr_match.arms.insert(expr_match.arms.len() - 1, arm);
                                    }
                                }
                            }
                        }
                    }
                }

                if first_entry_type_scaffolded {
                    file.items.push(syn::parse_str::<syn::Item>("fn get_entry_for_action(action_hash: &ActionHash) -> ExternResult<Option<EntryTypes>> {
    let record = match get_details(action_hash.clone(), GetOptions::default())? {
        Some(Details::Record(record_details)) => record_details.record,
        _ => { return Ok(None); }
    };
    let entry = match record.entry().as_option() {
        Some(entry) => entry,
        None => { return Ok(None); }
    };

    let (zome_index, entry_index) = match record.action().entry_type() {
        Some(EntryType::App(AppEntryDef {
            zome_index,
            entry_index,
            ..
        })) => (zome_index, entry_index),
        _ => { return Ok(None); }
    };

    Ok(EntryTypes::deserialize_from_type(zome_index.clone(), entry_index.clone(), entry)?)
}")?);
                }
            }
            Ok(file)
        },
    )?;

    let dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;
    let zome_file_tree = ZomeFileTree::from_zome_manifest(dna_file_tree, zome_manifest)?;

    Ok(zome_file_tree)
}
