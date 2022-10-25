use std::{collections::BTreeMap, ffi::OsString};

use build_fs_tree::file;
use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Select};
use holochain_types::prelude::{DnaManifest, ZomeManifest};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{map_rust_files, FileTree},
    generators::zome::{
        coordinator::find_all_extern_functions,
        utils::{get_coordinator_zomes_for_integrity, zome_manifest_path},
    },
};

use super::IndexType;

fn global_index_getter(
    integrity_zome_name: &String,
    index_name: &String,
    link_type_name: &String,
    link_to_entry_hash: bool,
) -> String {
    let to_hash_type = match link_to_entry_hash {
        true => String::from("EntryHash"),
        false => String::from("ActionHash"),
    };
    let snake_index_name = index_name.to_case(Case::Snake);

    format!(
        r#"use hdk::prelude::*;
use {}::*;

#[hdk_extern]
pub fn get_{}(_: ()) -> ExternResult<Vec<Record>> {{
    let path = Path::from("{}");
        
    let links = get_links(path.path_entry_hash()?, LinkTypes::{}, None)?;
    
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new({}::from(link.target).into(), GetOptions::default()))
        .collect();

    let maybe_records = HDK.with(|hdk| hdk.borrow().get(get_input))?;

    let record: Vec<Record> = maybe_records.into_iter().filter_map(|r| r).collect();

    Ok(record)
}}
"#,
        integrity_zome_name, snake_index_name, snake_index_name, link_type_name, to_hash_type,
    )
}

fn by_author_index_getter(
    integrity_zome_name: &String,
    index_name: &String,
    link_type_name: &String,
    link_to_entry_hash: bool,
) -> String {
    let to_hash_type = match link_to_entry_hash {
        true => String::from("EntryHash"),
        false => String::from("ActionHash"),
    };

    let snake_index_name = index_name.to_case(Case::Snake);

    format!(
        r#"use hdk::prelude::*;
use {}::*;

#[hdk_extern]
pub fn get_{}(author: AgentPubKey) -> ExternResult<Vec<Record>> {{
    let links = get_links(author, LinkTypes::{}, None)?;
    
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new({}::from(link.target).into(), GetOptions::default()))
        .collect();

    let maybe_records = HDK.with(|hdk| hdk.borrow().get(get_input))?;

    let record: Vec<Record> = maybe_records.into_iter().filter_map(|r| r).collect();

    Ok(record)
}}
"#,
        integrity_zome_name, snake_index_name, link_type_name, to_hash_type,
    )
}

fn choose_entry_type_create_function(
    functions_by_zome: &BTreeMap<String, Vec<String>>,
    entry_type: &String,
) -> ScaffoldResult<(String, String)> {
    let all_functions: Vec<(String, String)> = functions_by_zome
        .iter()
        .map(|(z, fns)| {
            fns.iter()
                .map(|f| (z.clone(), f.clone()))
                .collect::<Vec<(String, String)>>()
        })
        .flatten()
        .collect();
    let all_fns_str: Vec<String> = all_functions
        .iter()
        .map(|(z, f)| format!(r#""{}", in zome "{}""#, f, z))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "At the end of which function should the {} entries be indexed?",
            entry_type
        ))
        .default(0)
        .items(&all_fns_str[..])
        .interact()?;

    Ok(all_functions[selection].clone())
}

fn get_or_choose_entry_type_create_function(
    app_file_tree: &FileTree,
    dna_manifest: &DnaManifest,
    coordinator_zomes_for_integrity: &Vec<ZomeManifest>,
    entry_type: &String,
) -> ScaffoldResult<(ZomeManifest, String)> {
    let default_fn_name = format!("create_{}", entry_type.to_case(Case::Snake));

    let mut functions_by_zome: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for coordinator_zome in coordinator_zomes_for_integrity {
        let all_extern_functions =
            find_all_extern_functions(&app_file_tree, dna_manifest, coordinator_zome)?;

        if all_extern_functions.contains(&default_fn_name) {
            return Ok((coordinator_zome.clone(), default_fn_name));
        }

        functions_by_zome.insert(coordinator_zome.name.to_string(), all_extern_functions);
    }

    let (zome_name, fn_name) = choose_entry_type_create_function(&functions_by_zome, &entry_type)?;

    let chosen_zome = coordinator_zomes_for_integrity
        .iter()
        .find(|z| z.name.to_string().eq(&zome_name));

    match chosen_zome {
        Some(z) => Ok((z.clone(), fn_name)),
        None => Err(ScaffoldError::CoordinatorZomeNotFound(
            zome_name.clone(),
            dna_manifest.name(),
        )),
    }
}

fn add_create_link_in_create_function(
    mut app_file_tree: FileTree,
    dna_manifest: &DnaManifest,
    coordinator_zomes_for_integrity: &Vec<ZomeManifest>,
    index_name: &String,
    link_type_name: &String,
    index_type: &IndexType,
    entry_type: &String,
    link_to_entry_hash: bool,
) -> ScaffoldResult<FileTree> {
    let (chosen_coordinator_zome, fn_name) = get_or_choose_entry_type_create_function(
        &app_file_tree,
        dna_manifest,
        coordinator_zomes_for_integrity,
        entry_type,
    )?;

    let mut manifest_path = zome_manifest_path(&app_file_tree, &chosen_coordinator_zome)?.ok_or(
        ScaffoldError::CoordinatorZomeNotFound(
            chosen_coordinator_zome.name.0.to_string(),
            dna_manifest.name(),
        ),
    )?;

    let snake_case_entry_type = entry_type.to_case(Case::Snake);

    let mut create_link_stmts = match link_to_entry_hash {
        true => vec![format!(
            "let {}_entry_hash = hash_entry(&{})?;",
            snake_case_entry_type, snake_case_entry_type
        )],
        false => vec![],
    };

    let link_to_variable = match link_to_entry_hash {
        true => format!("{}_entry_hash", snake_case_entry_type),
        false => format!("{}_hash", snake_case_entry_type),
    };

    match index_type {
        IndexType::Global => {
            create_link_stmts.push(format!(r#"let path = Path::from("{}");"#, index_name));
            create_link_stmts.push(format!(
                r#"create_link(path.path_entry_hash()?, {}.clone(), LinkTypes::{}, ())?;"#,
                link_to_variable, link_type_name
            ));
        }
        IndexType::ByAuthor => {
            create_link_stmts.push(format!(
                r#"let my_agent_pub_key = agent_info()?.agent_latest_pubkey;"#,
            ));
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

    manifest_path.pop();

    let crate_src_path = manifest_path.join("src");
    let crate_src_path_iter: Vec<OsString> =
        crate_src_path.iter().map(|s| s.to_os_string()).collect();

    map_rust_files(
        app_file_tree
            .path_mut(&mut crate_src_path_iter.iter())
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
                            .any(|a| a.path.segments.iter().any(|s| s.ident.eq("hdk_extern")))
                            && item_fn.sig.ident.eq(fn_name.as_str())
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
            ScaffoldError::MalformedFile(crate_src_path.join(&path), error)
        }
        _ => e,
    })?;

    Ok(app_file_tree)
}

pub fn add_index_to_coordinators(
    mut app_file_tree: FileTree,
    dna_manifest: &DnaManifest,
    integrity_zome_name: &String,
    index_name: &String,
    link_type_name: &String,
    index_type: &IndexType,
    entry_types: &Vec<String>,
    link_to_entry_hash: bool,
) -> ScaffoldResult<FileTree> {
    let coordinator_zomes_for_integrity =
        get_coordinator_zomes_for_integrity(dna_manifest, integrity_zome_name);

    let coordinator_zome = match coordinator_zomes_for_integrity.len() {
        0 => Err(ScaffoldError::NoCoordinatorZomesFoundForIntegrityZome(
            dna_manifest.name(),
            integrity_zome_name.clone(),
        )),
        1 => Ok(coordinator_zomes_for_integrity[0].clone()),
        _ => {
            let names: Vec<String> = coordinator_zomes_for_integrity
                .iter()
                .map(|z| z.name.0.to_string())
                .collect();
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(
                    "Which coordinator zome should the index getter functions be scaffolded in?",
                )
                .default(0)
                .items(&names[..])
                .interact()?;

            Ok(coordinator_zomes_for_integrity[selection].clone())
        }
    }?;

    // 1. Create an INDEX_NAME.rs in "src/", with the appropriate zome functions
    let mut manifest_path = zome_manifest_path(&app_file_tree, &coordinator_zome)?.ok_or(
        ScaffoldError::CoordinatorZomeNotFound(
            coordinator_zome.name.0.to_string(),
            dna_manifest.name(),
        ),
    )?;

    manifest_path.pop();

    let snake_link_type_name = index_name.to_case(Case::Snake);

    let getter = match index_type {
        IndexType::Global => global_index_getter(
            integrity_zome_name,
            index_name,
            link_type_name,
            link_to_entry_hash,
        ),
        IndexType::ByAuthor => by_author_index_getter(
            integrity_zome_name,
            index_name,
            link_type_name,
            link_to_entry_hash,
        ),
    };

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
            file!(getter),
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
                snake_link_type_name,
            )
            .as_str(),
        );

    for entry_type in entry_types {
        app_file_tree = add_create_link_in_create_function(
            app_file_tree,
            dna_manifest,
            &coordinator_zomes_for_integrity,
            index_name,
            link_type_name,
            index_type,
            entry_type,
            link_to_entry_hash,
        )?;
    }

    Ok(app_file_tree)
}
