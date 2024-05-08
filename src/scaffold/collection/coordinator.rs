use std::ffi::OsString;

use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Select};
use holochain_types::prelude::ZomeManifest;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{insert_file, map_file, map_rust_files},
    scaffold::{
        dna::DnaFileTree,
        entry_type::definitions::EntryTypeReference,
        zome::{
            coordinator::{find_extern_function_in_zomes, find_extern_function_or_choose},
            utils::get_coordinator_zomes_for_integrity,
            ZomeFileTree,
        },
    },
};

use super::CollectionType;

fn global_collection_getter(
    integrity_zome_name: &str,
    collection_name: &str,
    link_type_name: &str,
) -> String {
    let snake_collection_name = collection_name.to_case(Case::Snake);

    format!(
        r#"use hdk::prelude::*;
use {integrity_zome_name}::*;

#[hdk_extern]
pub fn get_{snake_collection_name}() -> ExternResult<Vec<Link>> {{
    let path = Path::from("{snake_collection_name}");
    get_links(GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::{link_type_name})?.build())
}}
"#,
    )
}

fn by_author_collection_getter(
    integrity_zome_name: &str,
    collection_name: &str,
    link_type_name: &str,
) -> String {
    format!(
        r#"use hdk::prelude::*;
use {integrity_zome_name}::*;

#[hdk_extern]
pub fn get_{collection_name}(author: AgentPubKey) -> ExternResult<Vec<Link>> {{
    get_links(GetLinksInputBuilder::try_new(author, LinkTypes::{link_type_name})?.build())
}}
"#,
    )
}

fn add_create_link_in_create_function(
    dna_file_tree: DnaFileTree,
    coordinator_zomes_for_integrity: &Vec<ZomeManifest>,
    collection_name: &str,
    link_type_name: &str,
    collection_type: &CollectionType,
    entry_type_reference: &EntryTypeReference,
) -> ScaffoldResult<DnaFileTree> {
    let dna_manifest_path = dna_file_tree.dna_manifest_path.clone();

    let (chosen_coordinator_zome, fn_name) = find_extern_function_or_choose(
        &dna_file_tree,
        coordinator_zomes_for_integrity,
        &format!(
            "create_{}",
            entry_type_reference.entry_type.to_case(Case::Snake)
        ),
        &format!(
            "At the end of which function should the {} entries be collected?",
            entry_type_reference.entry_type.to_case(Case::Pascal)
        ),
    )?;

    let zome_file_tree = ZomeFileTree::from_zome_manifest(dna_file_tree, chosen_coordinator_zome)?;

    let snake_case_entry_type = entry_type_reference.entry_type.to_case(Case::Snake);

    let mut create_link_stmts = match entry_type_reference.reference_entry_hash {
        true => vec![format!(
            "let {}_entry_hash = hash_entry(&{})?;",
            snake_case_entry_type, snake_case_entry_type
        )],
        false => vec![],
    };

    let link_to_variable = match entry_type_reference.reference_entry_hash {
        true => format!("{}_entry_hash", snake_case_entry_type),
        false => format!("{}_hash", snake_case_entry_type),
    };

    match collection_type {
        CollectionType::Global => {
            create_link_stmts.push(format!(r#"let path = Path::from("{}");"#, collection_name));
            create_link_stmts.push(format!(
                r#"create_link(path.path_entry_hash()?, {}.clone(), LinkTypes::{}, ())?;"#,
                link_to_variable, link_type_name
            ));
        }
        CollectionType::ByAuthor => {
            create_link_stmts
                .push("let my_agent_pub_key = agent_info()?.agent_latest_pubkey;".to_string());
            create_link_stmts.push(format!(
                r#"create_link(my_agent_pub_key, {}.clone(), LinkTypes::{}, ())?;"#,
                link_to_variable, link_type_name
            ));
        }
    };

    let stmts = create_link_stmts
        .into_iter()
        .map(|s| syn::parse_str::<syn::Stmt>(s.as_str()))
        .collect::<Result<Vec<syn::Stmt>, syn::Error>>()?;

    let crate_src_path = zome_file_tree.zome_crate_path.join("src");

    let mut file_tree = zome_file_tree.dna_file_tree.file_tree();

    let v: Vec<OsString> = crate_src_path
        .clone()
        .iter()
        .map(|s| s.to_os_string())
        .collect();
    map_rust_files(
        file_tree
            .path_mut(&mut v.iter())
            .ok_or(ScaffoldError::PathNotFound(crate_src_path.clone()))?,
        |_file_path, mut file| {
            file.items = file
                .items
                .into_iter()
                .map(|i| {
                    if let syn::Item::Fn(mut item_fn) = i.clone() {
                        if item_fn
                            .attrs
                            .iter()
                            .any(|a| a.path().segments.iter().any(|s| s.ident.eq("hdk_extern")))
                            && item_fn.sig.ident.eq(&fn_name.sig.ident)
                        {
                            for new_stmt in stmts.clone() {
                                item_fn
                                    .block
                                    .stmts
                                    .insert(item_fn.block.stmts.len() - 1, new_stmt);
                            }
                            return syn::Item::Fn(item_fn);
                        }
                    }

                    i
                })
                .collect();

            Ok(file)
        },
    )
    .map_err(|e| match e {
        ScaffoldError::MalformedFile(path, error) => {
            ScaffoldError::MalformedFile(crate_src_path.join(path), error)
        }
        _ => e,
    })?;

    let dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;

    Ok(dna_file_tree)
}

fn add_delete_link_in_delete_function(
    dna_file_tree: DnaFileTree,
    coordinator_zomes_for_integrity: &Vec<ZomeManifest>,
    collection_name: &str,
    link_type_name: &str,
    collection_type: &CollectionType,
    entry_type_reference: &EntryTypeReference,
) -> ScaffoldResult<(DnaFileTree, bool)> {
    let dna_manifest_path = dna_file_tree.dna_manifest_path.clone();

    let Some((chosen_coordinator_zome, fn_name)) = find_extern_function_in_zomes(
        &dna_file_tree,
        coordinator_zomes_for_integrity,
        &format!(
            "delete_{}",
            entry_type_reference.entry_type.to_case(Case::Snake)
        ),
    )?
    else {
        return Ok((dna_file_tree, false));
    };

    let zome_file_tree = ZomeFileTree::from_zome_manifest(dna_file_tree, chosen_coordinator_zome)?;

    let snake_case_entry_type = entry_type_reference.entry_type.to_case(Case::Snake);

    let target_hash_variable = match entry_type_reference.reference_entry_hash {
        true =>
            r#"record.action().entry_hash().ok_or(wasm_error!(WasmErrorInner::Guest("Record does not have an entry".to_string())))?"#.to_string()
        ,
        false => format!("&original_{snake_case_entry_type}_hash"),
    };
    let into_hash_fn = match entry_type_reference.reference_entry_hash {
        true => "into_entry_hash()".to_string(),
        false => "into_action_hash()".to_string(),
    };

    let mut delete_link_stmts: Vec<String> = vec![];
    match collection_type {
        CollectionType::Global => {
            delete_link_stmts.push(format!(r#"let path = Path::from("{}");"#, collection_name));
            delete_link_stmts.push(format!(
                r#"let links = get_links(
                    GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::{link_type_name})?.build(),
                )?;"#,
            ));
            delete_link_stmts.push(format!(
                r#"for link in links {{
                    if let Some(hash) = link.target.{into_hash_fn} {{
                       if hash.eq({target_hash_variable}) {{
                            delete_link(link.create_link_hash)?;
                        }}
                    }}
                }}"#,
            ));
        }
        CollectionType::ByAuthor => {
            delete_link_stmts.insert(
                0,
                format!(
                    r#"
                let record = match details {{
                    Details::Record(details) => Ok(details.record),
                    _ => Err(wasm_error!(WasmErrorInner::Guest(String::from(
                        "Malformed get details response"
                    )))),
                }}?;
            "#
                ),
            );
            delete_link_stmts.insert(0, format!(r#"
                let details = get_details(original_{snake_case_entry_type}_hash.clone(), GetOptions::default())?
                    .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("{{pascal_entry_def_name}} not found"))))?;
            "#));
            delete_link_stmts.push(format!(
                r#"let links = get_links(
                    GetLinksInputBuilder::try_new(record.action().author().clone(), LinkTypes::{link_type_name})?.build()
                )?;"#,
            ));
            delete_link_stmts.push(format!(
                r#"for link in links {{
                    if let Some(hash) = link.target.{into_hash_fn} {{
                       if hash.eq({target_hash_variable}) {{
                            delete_link(link.create_link_hash)?;
                        }}
                    }}
                }}"#,
            ));
        }
    };

    let stmts = delete_link_stmts
        .iter()
        .map(|s| syn::parse_str::<syn::Stmt>(s))
        .collect::<Result<Vec<syn::Stmt>, syn::Error>>()?;

    let crate_src_path = zome_file_tree.zome_crate_path.join("src");

    let mut file_tree = zome_file_tree.dna_file_tree.file_tree();

    let v: Vec<OsString> = crate_src_path
        .clone()
        .iter()
        .map(|s| s.to_os_string())
        .collect();
    map_rust_files(
        file_tree
            .path_mut(&mut v.iter())
            .ok_or(ScaffoldError::PathNotFound(crate_src_path.clone()))?,
        |_file_path, mut file| {
            file.items = file
                .items
                .into_iter()
                .map(|item| {
                    if let syn::Item::Fn(mut item_fn) = item.clone() {
                        if item_fn
                            .attrs
                            .iter()
                            .any(|a| a.path().segments.iter().any(|s| s.ident.eq("hdk_extern")))
                            && item_fn.sig.ident.eq(&fn_name.sig.ident)
                        {
                            for new_stmt in stmts.clone() {
                                item_fn
                                    .block
                                    .stmts
                                    .insert(item_fn.block.stmts.len() - 1, new_stmt);
                            }
                            return syn::Item::Fn(item_fn);
                        }
                    }

                    item
                })
                .collect();

            Ok(file)
        },
    )
    .map_err(|e| match e {
        ScaffoldError::MalformedFile(path, error) => {
            ScaffoldError::MalformedFile(crate_src_path.join(path), error)
        }
        _ => e,
    })?;

    let dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;

    Ok((dna_file_tree, true))
}

pub fn add_collection_to_coordinators(
    integrity_zome_file_tree: ZomeFileTree,
    collection_name: &str,
    link_type_name: &str,
    collection_type: &CollectionType,
    entry_type: &EntryTypeReference,
) -> ScaffoldResult<(DnaFileTree, ZomeManifest, bool)> {
    let integrity_zome_name = integrity_zome_file_tree.zome_manifest.name.0.to_string();
    let dna_manifest_path = integrity_zome_file_tree
        .dna_file_tree
        .dna_manifest_path
        .clone();

    let coordinator_zomes_for_integrity = get_coordinator_zomes_for_integrity(
        &integrity_zome_file_tree.dna_file_tree.dna_manifest,
        &integrity_zome_name,
    );

    let coordinator_zome = match coordinator_zomes_for_integrity.len() {
        0 => Err(ScaffoldError::NoCoordinatorZomesFoundForIntegrityZome(
            integrity_zome_file_tree.dna_file_tree.dna_manifest.name(),
            integrity_zome_file_tree.zome_manifest.name.0.to_string(),
        )),
        1 => Ok(coordinator_zomes_for_integrity[0].clone()),
        _ => {
            let names: Vec<String> = coordinator_zomes_for_integrity
                .iter()
                .map(|z| z.name.0.to_string())
                .collect();
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(
                    "Which coordinator zome should the collection getter functions be scaffolded in?",
                )
                .default(0)
                .items(&names[..])
                .interact()?;

            Ok(coordinator_zomes_for_integrity[selection].clone())
        }
    }?;

    // 1. Create an INDEX_NAME.rs in "src/", with the appropriate zome functions

    let zome_file_tree = ZomeFileTree::from_zome_manifest(
        integrity_zome_file_tree.dna_file_tree,
        coordinator_zome.clone(),
    )?;

    let snake_link_type_name = collection_name.to_case(Case::Snake);

    let getter = match collection_type {
        CollectionType::Global => {
            global_collection_getter(&integrity_zome_name, collection_name, link_type_name)
        }
        CollectionType::ByAuthor => {
            by_author_collection_getter(&integrity_zome_name, collection_name, link_type_name)
        }
    };

    let mut file_tree = zome_file_tree.dna_file_tree.file_tree();

    let crate_src_path = zome_file_tree.zome_crate_path.join("src");
    let collection_path = crate_src_path.join(format!("{}.rs", snake_link_type_name.clone()));
    insert_file(&mut file_tree, &collection_path, &getter)?;

    // 2. Add this file as a module in the entry point for the crate

    let lib_rs_path = crate_src_path.join("lib.rs");

    map_file(&mut file_tree, &lib_rs_path, |s| {
        Ok(format!(
            r#"pub mod {};

{}"#,
            snake_link_type_name, s
        ))
    })?;

    let mut dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;

    dna_file_tree = add_create_link_in_create_function(
        dna_file_tree,
        &coordinator_zomes_for_integrity,
        collection_name,
        link_type_name,
        collection_type,
        entry_type,
    )?;

    let (dna_file_tree, deletable) = add_delete_link_in_delete_function(
        dna_file_tree,
        &coordinator_zomes_for_integrity,
        collection_name,
        link_type_name,
        collection_type,
        entry_type,
    )?;

    Ok((dna_file_tree, coordinator_zome, deletable))
}
