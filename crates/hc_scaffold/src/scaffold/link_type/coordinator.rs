use std::ffi::OsString;

use build_fs_tree::file;
use convert_case::{Case, Casing};
use holochain_types::prelude::{DnaManifest, ZomeManifest};

use crate::{
    definitions::EntryType,
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{insert_file, map_file, FileTree},
    scaffold::{dna::DnaFileTree, zome::ZomeFileTree},
};

fn metadata_handlers(
    integrity_zome_name: &String,
    link_type_name: &String,
    from_entry_type: &EntryType,
    link_from_entry_hash: bool,
) -> String {
    let from_arg = match from_entry_type {
        EntryType::Agent => String::from("agent"),
        EntryType::App(et) => format!("{}_hash", et.to_case(Case::Snake)),
    };
    let from_arg_type = match from_entry_type {
        EntryType::Agent => String::from("AgentPubKey"),
        EntryType::App(et) => match link_from_entry_hash {
            true => String::from("EntryHash"),
            false => String::from("ActionHash"),
        },
    };

    format!(
        r#"use hdk::prelude::*;
use {}::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Add{}For{}Input {{
    {}: {},
    {}: String,
}}
#[hdk_extern]
pub fn add_{}_for_{}(input: Add{}For{}Input) -> ExternResult<()> {{
    create_link(input.{}.clone(), input.{}, LinkTypes::{}, input.{})?;

    Ok(())    
}}

#[hdk_extern]
pub fn get_{}_for_{}({}: {}) -> ExternResult<Vec<String>> {{
    let links = get_links({}, LinkTypes::{}, None)?;
    
    let {}: Vec<String> = links
        .into_iter()
        .map(|link| 
          String::from_utf8(link.target.into_inner())
            .map_err(|e| wasm_error!(WasmErrorInner::Guest(format!("Error converting link tag to string: {{:?}}", e))))
        )
        .collect::<ExternResult<Vec<String>>>()?;

    Ok({})
}}"#,
        integrity_zome_name,
        link_type_name.to_case(Case::Pascal),
        from_entry_type.to_string().to_case(Case::Pascal),
        from_arg,
        from_arg_type,
        link_type_name.to_case(Case::Snake),
        link_type_name.to_case(Case::Snake),
        from_entry_type.to_string().to_case(Case::Snake),
        link_type_name.to_case(Case::Pascal),
        from_entry_type.to_string().to_case(Case::Pascal),
        from_arg,
        from_arg,
        link_type_name.to_case(Case::Pascal),
        link_type_name.to_case(Case::Snake),
        link_type_name.to_case(Case::Snake),
        from_entry_type.to_string().to_case(Case::Snake),
        from_arg,
        from_arg_type,
        from_arg,
        link_type_name.to_case(Case::Pascal),
        link_type_name.to_case(Case::Snake),
        link_type_name.to_case(Case::Snake),
    )
}

// Add invitee to event
fn to_agent_handlers(
    integrity_zome_name: &String,
    link_type_name: &String,
    from_entry_type: &EntryType,
    link_from_entry_hash: bool,
) -> String {
    let pascal_link_type_name = link_type_name.to_case(Case::Pascal);
    let snake_link_type_name = link_type_name.to_case(Case::Snake);
    let pascal_from_entry_type = from_entry_type.to_string().to_case(Case::Pascal);
    let snake_from_entry_type = from_entry_type.to_string().to_case(Case::Snake);

    let from_arg = match from_entry_type {
        EntryType::Agent => String::from("agent"),
        EntryType::App(et) => format!("{}_hash", et.to_case(Case::Snake)),
    };
    let from_arg_type = match from_entry_type {
        EntryType::Agent => String::from("AgentPubKey"),
        EntryType::App(et) => match link_from_entry_hash {
            true => String::from("EntryHash"),
            false => String::from("ActionHash"),
        },
    };

    format!(
        r#"use hdk::prelude::*;
use {integrity_zome_name}::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Add{pascal_link_type_name}For{pascal_from_entry_type}Input {{
    {from_arg}: {from_arg_type},
    {snake_link_type_name}: AgentPubKey,
}}
#[hdk_extern]
pub fn add_{snake_link_type_name}_for_{snake_from_entry_type}(input: Add{pascal_link_type_name}For{pascal_from_entry_type}Input) -> ExternResult<()> {{
    create_link(input.{from_arg}.clone(), input.{snake_link_type_name}, LinkTypes::{pascal_link_type_name}, ())?;

    Ok(())    
}}
        
#[hdk_extern]
pub fn get_{snake_link_type_name}_for_{snake_from_entry_type}({from_arg}: {from_arg_type}) -> ExternResult<Vec<AgentPubKey>> {{
    let links = get_links({from_arg}, LinkTypes::{pascal_link_type_name}, None)?;
    
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(link.target))
        .collect();

    Ok(agents)
}}"#
    )
}

// Event to agent
fn to_entry_link_type_handlers(
    integrity_zome_name: &String,
    link_type_name: &String,
    from_entry_type: &EntryType,
    to_entry_type: &String,
    link_from_entry_hash: bool,
    link_to_entry_hash: bool,
) -> String {
    let from_hash_type = match (from_entry_type, link_from_entry_hash) {
        (EntryType::Agent, _) => String::from("AgentPubKey"),
        (_, true) => String::from("EntryHash"),
        (_, false) => String::from("ActionHash"),
    };
    let from_arg_name = match from_entry_type {
        EntryType::Agent => String::from("from_agent"),
        EntryType::App(et) => format!("{}_hash", et),
    };
    let to_hash_type = match link_to_entry_hash {
        true => String::from("EntryHash"),
        false => String::from("ActionHash"),
    };
    let to_arg_name = format!("{}_hash", to_entry_type.to_case(Case::Snake));
    let pascal_link_type_name = link_type_name.to_case(Case::Pascal);
    let pascal_to_entry_type = to_entry_type.to_case(Case::Pascal);
    let snake_from_entry_type = from_entry_type.to_string().to_case(Case::Snake);
    let pascal_from_entry_type = from_entry_type.to_string().to_case(Case::Pascal);
    let snake_to_entry_type = to_entry_type.to_case(Case::Snake);

    format!(
        r#"use hdk::prelude::*;
use {integrity_zome_name}::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Add{pascal_to_entry_type}For{pascal_from_entry_type}Input {{
    {from_arg_name}: {from_hash_type},
    {to_arg_name}: {to_hash_type},
}}
#[hdk_extern]
pub fn add_{snake_to_entry_type}_for_{snake_from_entry_type}(input: Add{pascal_to_entry_type}For{pascal_from_entry_type}Input) -> ExternResult<()> {{
    create_link(input.{from_arg_name}, input.{to_arg_name}, LinkTypes::{pascal_link_type_name}, ())?;

    Ok(())    
}}

#[hdk_extern]
pub fn get_{snake_to_entry_type}_for_{snake_from_entry_type}({from_arg_name}: {from_hash_type}) -> ExternResult<Vec<ActionHash>> {{
    let links = get_links({from_arg_name}, LinkTypes::{pascal_link_type_name}, None)?;
    
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new({to_hash_type}::from(link.target).into(), GetOptions::default()))
        .collect();

    // Get the records to filter out the deleted ones
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;

    let action_hashes: Vec<ActionHash> = records
        .into_iter()
        .filter_map(|r| r)
        .map(|r| r.action_address().clone())
        .collect();

    Ok(action_hashes)
}}
"#
    )
}

fn initial_link_type_handlers(
    integrity_zome_name: &String,
    link_type_name: &String,
    from_entry_type: &EntryType,
    to_entry_type: &Option<EntryType>,
    link_from_entry_hash: bool,
    link_to_entry_hash: bool,
) -> String {
    match to_entry_type {
        None => metadata_handlers(
            integrity_zome_name,
            link_type_name,
            from_entry_type,
            link_from_entry_hash,
        ),
        Some(EntryType::Agent) => to_agent_handlers(
            integrity_zome_name,
            link_type_name,
            from_entry_type,
            link_from_entry_hash,
        ),
        Some(EntryType::App(entry_type)) => to_entry_link_type_handlers(
            integrity_zome_name,
            link_type_name,
            from_entry_type,
            entry_type,
            link_from_entry_hash,
            link_to_entry_hash,
        ),
    }
}

pub fn add_link_type_functions_to_coordinator(
    mut coordinator_zome_file_tree: ZomeFileTree,
    integrity_zome_name: &String,
    link_type_name: &String,
    from_entry_type: &EntryType,
    to_entry_type: &Option<EntryType>,
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
