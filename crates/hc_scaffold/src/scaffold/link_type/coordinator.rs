use std::ffi::OsString;

use build_fs_tree::file;
use convert_case::{Case, Casing};
use holochain_types::prelude::{DnaManifest, ZomeManifest};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{insert_file, map_file, FileTree},
    scaffold::{
        dna::DnaFileTree,
        entry_type::definitions::{Cardinality, EntryTypeReference, Referenceable},
        zome::ZomeFileTree,
    },
};

use super::link_type_name;

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

pub fn add_link_handler(
    from_referenceable: &Referenceable,
    to_referenceable: &Referenceable,
    bidireccional: bool,
) -> String {
    let from_hash_type = from_referenceable.hash_type().to_string();
    let from_arg_name = from_referenceable.field_name(&Cardinality::Single);
    let to_hash_type = to_referenceable.hash_type().to_string();
    let to_arg_name = to_referenceable.field_name(&Cardinality::Single);

    let normal_link_type_name = link_type_name(from_referenceable, to_referenceable);
    let inverse_link_type_name = link_type_name(to_referenceable, from_referenceable);
    let singular_snake_from_entry_type = from_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Snake);
    let plural_snake_from_entry_type = from_referenceable
        .to_string(&Cardinality::Vector)
        .to_case(Case::Snake);
    let singular_pascal_from_entry_type = from_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Pascal);
    let plural_snake_to_entry_type = to_referenceable
        .to_string(&Cardinality::Vector)
        .to_case(Case::Snake);
    let singular_snake_to_entry_type = to_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Snake);
    let singular_pascal_to_entry_type = to_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Pascal);

    let bidireccional_create = match bidireccional {
        true => format!(
            r#"create_link(input.{to_arg_name}, input.{from_arg_name}, LinkTypes::{inverse_link_type_name}, ())?;"#
        ),
        false => format!(""),
    };

    format!(
        r#"#[derive(Serialize, Deserialize, Debug)]
pub struct Add{singular_pascal_to_entry_type}For{singular_pascal_from_entry_type}Input {{
    {from_arg_name}: {from_hash_type},
    {to_arg_name}: {to_hash_type},
}}
#[hdk_extern]
pub fn add_{singular_snake_to_entry_type}_for_{singular_snake_from_entry_type}(input: Add{singular_pascal_to_entry_type}For{singular_pascal_from_entry_type}Input) -> ExternResult<()> {{
    create_link(input.{from_arg_name}.clone(), input.{to_arg_name}.clone(), LinkTypes::{normal_link_type_name}, ())?;
    {bidireccional_create}

    Ok(())    
}}"#
    )
}

pub fn get_links_handler(
    from_referenceable: &Referenceable,
    to_referenceable: &Referenceable,
) -> String {
    match to_referenceable {
        Referenceable::Agent { role } => {
            get_links_handler_to_agent(from_referenceable, to_referenceable)
        }
        Referenceable::EntryType(e) => get_links_handler_to_entry(from_referenceable, e),
    }
}

fn get_links_handler_to_agent(
    from_referenceable: &Referenceable,
    to_referenceable: &Referenceable,
) -> String {
    let from_hash_type = from_referenceable.hash_type().to_string();
    let from_arg_name = from_referenceable.field_name(&Cardinality::Single);

    let pascal_link_type_name = link_type_name(from_referenceable, to_referenceable);
    let singular_snake_from_entry_type = from_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Snake);
    let plural_snake_from_entry_type = from_referenceable
        .to_string(&Cardinality::Vector)
        .to_case(Case::Snake);
    let pascal_from_entry_type = from_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Pascal);
    let plural_snake_to_entry_type = to_referenceable
        .to_string(&Cardinality::Vector)
        .to_case(Case::Snake);
    let singular_snake_to_entry_type = to_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Snake);

    format!(
        r#"#[hdk_extern]
pub fn get_{plural_snake_to_entry_type}_for_{singular_snake_from_entry_type}({from_arg_name}: {from_hash_type}) -> ExternResult<Vec<AgentPubKey>> {{
    let links = get_links({from_arg_name}, LinkTypes::{pascal_link_type_name}, None)?;
    
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(EntryHash::from(link.target)))
        .collect();

    Ok(agents)
}}"#,
    )
}

fn get_links_handler_to_entry(
    from_referenceable: &Referenceable,
    to_entry_type: &EntryTypeReference,
) -> String {
    let from_hash_type = from_referenceable.hash_type().to_string();
    let from_arg_name = from_referenceable.field_name(&Cardinality::Single);
    let to_hash_type = to_entry_type.hash_type().to_string();
    let to_arg_name = to_entry_type.field_name(&Cardinality::Single);

    let pascal_link_type_name = link_type_name(
        from_referenceable,
        &Referenceable::EntryType(to_entry_type.clone()),
    );
    let pascal_to_entry_type = to_entry_type
        .to_string(&Cardinality::Single)
        .to_case(Case::Pascal);
    let singular_snake_from_entry_type = from_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Snake);
    let plural_snake_from_entry_type = from_referenceable
        .to_string(&Cardinality::Vector)
        .to_case(Case::Snake);
    let pascal_from_entry_type = from_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Pascal);
    let plural_snake_to_entry_type = to_entry_type
        .to_string(&Cardinality::Vector)
        .to_case(Case::Snake);
    let singular_snake_to_entry_type = to_entry_type
        .to_string(&Cardinality::Single)
        .to_case(Case::Snake);

    let map_line = match to_entry_type.reference_entry_hash {
        true => String::from(".filter_map(|r| r.action().entry_hash().cloned())"),
        false => String::from(".map(|r| r.action_address().clone())"),
    };
    format!(
        r#"#[hdk_extern]
pub fn get_{plural_snake_to_entry_type}_for_{singular_snake_from_entry_type}({from_arg_name}: {from_hash_type}) -> ExternResult<Vec<{to_hash_type}>> {{
    let links = get_links({from_arg_name}, LinkTypes::{pascal_link_type_name}, None)?;
    
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new({to_hash_type}::from(link.target).into(), GetOptions::default()))
        .collect();

    // Get the records to filter out the deleted ones
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;

    let hashes: Vec<{to_hash_type}> = records
        .into_iter()
        .filter_map(|r| r)
        {map_line}
        .collect();

    Ok(hashes)
}}"#,
    )
}

fn from_link_hash_type(hash_type: &String) -> String {
    match hash_type.as_str() {
        "AgentPubKey" => format!("AgentPubKey::from(EntryHash::from(link.target.clone()))"),
        _ => format!("{}::from(link.target.clone())", hash_type),
    }
}

// Event to calendar
fn remove_link_handlers(
    from_referenceable: &Referenceable,
    to_referenceable: &Referenceable,
    bidireccional: bool,
) -> String {
    let from_hash_type = from_referenceable.hash_type().to_string();
    let from_arg_name = from_referenceable.field_name(&Cardinality::Single);
    let to_hash_type = to_referenceable.hash_type().to_string();
    let to_arg_name = to_referenceable.field_name(&Cardinality::Single);

    let pascal_link_type_name = link_type_name(from_referenceable, to_referenceable);
    let inverse_link_type_name = link_type_name(to_referenceable, from_referenceable);
    let singular_pascal_to_entry_type = to_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Pascal);
    let singular_snake_from_entry_type = from_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Snake);
    let plural_snake_from_entry_type = from_referenceable
        .to_string(&Cardinality::Vector)
        .to_case(Case::Snake);
    let singular_pascal_from_entry_type = from_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Pascal);
    let singular_snake_to_entry_type = to_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Snake);

    let from_link = from_link_hash_type(&to_hash_type);
    let from_inverse = from_link_hash_type(&from_hash_type);

    let bidireccional_remove = match bidireccional {
        true => format!(
            r#"
    let links = get_links(input.{to_arg_name}.clone(), LinkTypes::{inverse_link_type_name}, None)?;

    for link in links {{
        if {from_inverse}.eq(&input.{from_arg_name}) {{
            delete_link(link.create_link_hash)?;
        }}
    }}"#
        ),
        false => format!(""),
    };
    format!(
        r#"#[derive(Serialize, Deserialize, Debug)]
pub struct Remove{singular_pascal_to_entry_type}For{singular_pascal_from_entry_type}Input {{
    {from_arg_name}: {from_hash_type},
    {to_arg_name}: {to_hash_type},
}}
#[hdk_extern]
pub fn remove_{singular_snake_to_entry_type}_for_{singular_snake_from_entry_type}(input: Remove{singular_pascal_to_entry_type}For{singular_pascal_from_entry_type}Input ) -> ExternResult<()> {{
    let links = get_links(input.{from_arg_name}.clone(), LinkTypes::{pascal_link_type_name}, None)?;
    
    for link in links {{
        if {from_link}.eq(&input.{to_arg_name}) {{
            delete_link(link.create_link_hash)?;
        }}
    }}
    {bidireccional_remove}

    Ok(())        
}}"#
    )
}

fn normal_handlers(
    integrity_zome_name: &String,
    from_referenceable: &Referenceable,
    to_referenceable: &Referenceable,
    bidireccional: bool,
) -> String {
    let inverse_get = match bidireccional {
        true => format!(
            r#"

{}"#,
            get_links_handler(to_referenceable, from_referenceable)
        ),
        false => format!(""),
    };

    format!(
        r#"use hdk::prelude::*;
use {integrity_zome_name}::*;

{}

{}
{}
        
{}"#,
        add_link_handler(from_referenceable, to_referenceable, bidireccional),
        get_links_handler(from_referenceable, to_referenceable),
        inverse_get,
        remove_link_handlers(from_referenceable, to_referenceable, bidireccional)
    )
}

pub fn add_link_type_functions_to_coordinator(
    mut coordinator_zome_file_tree: ZomeFileTree,
    integrity_zome_name: &String,
    link_type_name: &String,
    from_referenceable: &Referenceable,
    to_referenceable: &Option<Referenceable>,
    bidireccional: bool,
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

    let handlers = match to_referenceable {
        None => metadata_handlers(integrity_zome_name, link_type_name, from_referenceable),
        Some(r) => normal_handlers(integrity_zome_name, from_referenceable, r, bidireccional),
    };

    insert_file(&mut file_tree, &new_file_path, &handlers)?;

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
