use std::ffi::OsString;

use build_fs_tree::file;
use convert_case::{Case, Casing};
use holochain_types::prelude::{DnaManifest, ZomeManifest};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{insert_file, map_file, FileTree},
    scaffold::{
        dna::DnaFileTree,
        entry_type::definitions::{Cardinality, Referenceable},
        zome::ZomeFileTree,
    },
};

fn metadata_handlers(
    integrity_zome_name: &String,
    link_type_name: &String,
    from_referenceable: &Referenceable,
) -> String {
    let snake_from_arg = from_referenceable
        .field_name(&Cardinality::Single)
        .to_case(Case::Snake);
    let from_arg_type = from_referenceable.hash_type().to_string();

    let snake_from = from_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Snake);
    let pascal_from = from_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Pascal);
    let snake_link_type_name = link_type_name.to_case(Case::Snake);
    let pascal_link_type_name = link_type_name.to_case(Case::Pascal);

    format!(
        r#"use hdk::prelude::*;
use {integrity_zome_name}::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Add{pascal_link_type_name}For{pascal_from}Input {{
    {snake_from_arg}: {from_arg_type},
    {snake_link_type_name}: String,
}}
#[hdk_extern]
pub fn add_{snake_link_type_name}_for_{snake_from}(input: Add{pascal_link_type_name}For{pascal_from}Input) -> ExternResult<()> {{
    create_link(input.{snake_from_arg}.clone(), input.{snake_from_arg}, LinkTypes::{pascal_link_type_name}, input.{snake_link_type_name})?;

    Ok(())    
}}

#[hdk_extern]
pub fn get_{snake_link_type_name}_for_{snake_from}({snake_from_arg}: {from_arg_type}) -> ExternResult<Vec<String>> {{
    let links = get_links({snake_from_arg}, LinkTypes::{pascal_link_type_name}, None)?;
    
    let {snake_link_type_name}: Vec<String> = links
        .into_iter()
        .map(|link| 
          String::from_utf8(link.target.into_inner())
            .map_err(|e| wasm_error!(WasmErrorInner::Guest(format!("Error converting link tag to string: {{:?}}", e))))
        )
        .collect::<ExternResult<Vec<String>>>()?;

    Ok({snake_link_type_name})
}}"#
    )
}

// Add invitee to event
fn to_agent_handlers(
    integrity_zome_name: &String,
    link_type_name: &String,
    from_entry_type: &Referenceable,
    link_from_entry_hash: bool,
) -> String {
    let pascal_link_type_name = link_type_name.to_case(Case::Pascal);
    let snake_link_type_name = link_type_name.to_case(Case::Snake);
    let pascal_from_entry_type = from_entry_type.to_string().to_case(Case::Pascal);
    let snake_from_entry_type = from_entry_type.to_string().to_case(Case::Snake);

    let from_arg = match from_entry_type {
        Referenceable::Agent => String::from("agent"),
        Referenceable::EntryType(et) => format!("{}_hash", et.to_case(Case::Snake)),
    };
    let from_arg_type = match from_entry_type {
        Referenceable::Agent => String::from("AgentPubKey"),
        Referenceable::EntryType(et) => match link_from_entry_hash {
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
    from_entry_type: &Referenceable,
    to_entry_type: &String,
    link_from_entry_hash: bool,
    link_to_entry_hash: bool,
) -> String {
    let from_hash_type = match (from_entry_type, link_from_entry_hash) {
        (Referenceable::Agent, _) => String::from("AgentPubKey"),
        (_, true) => String::from("EntryHash"),
        (_, false) => String::from("ActionHash"),
    };
    let from_arg_name = match from_entry_type {
        Referenceable::Agent => String::from("from_agent"),
        Referenceable::EntryType(et) => format!("{}_hash", et),
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
    from_referenceable: &Referenceable,
    to_referenceable: &Option<Referenceable>,
) -> String {
    match to_referenceable {
        None => metadata_handlers(integrity_zome_name, link_type_name, from_referenceable),
        Some(Referenceable::Agent) => {
            to_agent_handlers(integrity_zome_name, link_type_name, from_referenceable)
        }
        Some(Referenceable::EntryType(entry_type)) => to_entry_link_type_handlers(
            integrity_zome_name,
            link_type_name,
            from_referenceable,
            entry_type,
        ),
    }
}

pub fn add_link_type_functions_to_coordinator(
    mut coordinator_zome_file_tree: ZomeFileTree,
    integrity_zome_name: &String,
    link_type_name: &String,
    from_referenceable: &Referenceable,
    to_referenceable: &Option<Referenceable>,
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
            from_referenceable,
            to_referenceable,
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
