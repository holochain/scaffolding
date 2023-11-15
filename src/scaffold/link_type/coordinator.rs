use convert_case::{Case, Casing};

use crate::{
    error::ScaffoldResult,
    file_tree::{insert_file, map_file},
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
    let plural_snake_link_type_name =
        pluralizer::pluralize(&link_type_name.to_case(Case::Snake), 2, false);
    let pascal_link_type_name = link_type_name.to_case(Case::Pascal);

    format!(
        r#"use hdk::prelude::*;
use {integrity_zome_name}::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Add{pascal_link_type_name}For{pascal_from}Input {{
    pub {snake_from_arg}: {from_arg_type},
    pub {snake_link_type_name}: String,
}}
#[hdk_extern]
pub fn add_{snake_link_type_name}_for_{snake_from}(input: Add{pascal_link_type_name}For{pascal_from}Input) -> ExternResult<()> {{
    create_link(input.{snake_from_arg}.clone(), input.{snake_from_arg}, LinkTypes::{pascal_link_type_name}, input.{snake_link_type_name})?;

    Ok(())    
}}

#[hdk_extern]
pub fn get_{plural_snake_link_type_name}_for_{snake_from}({snake_from_arg}: {from_arg_type}) -> ExternResult<Vec<String>> {{
    let links = get_links({snake_from_arg}, LinkTypes::{pascal_link_type_name}, None)?;
    
    let {snake_link_type_name}: Vec<String> = links
        .into_iter()
        .map(|link| 
          String::from_utf8(link.tag.into_inner())
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
    let singular_pascal_from_entry_type = from_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Pascal);
    let singular_snake_to_entry_type = to_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Snake);
    let singular_pascal_to_entry_type = to_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Pascal);

    let bidireccional_create = match bidireccional {
        true => format!(
            r#"create_link(input.target_{to_arg_name}, input.base_{from_arg_name}, LinkTypes::{inverse_link_type_name}, ())?;"#
        ),
        false => format!(""),
    };

    format!(
        r#"#[derive(Serialize, Deserialize, Debug)]
pub struct Add{singular_pascal_to_entry_type}For{singular_pascal_from_entry_type}Input {{
    pub base_{from_arg_name}: {from_hash_type},
    pub target_{to_arg_name}: {to_hash_type},
}}
#[hdk_extern]
pub fn add_{singular_snake_to_entry_type}_for_{singular_snake_from_entry_type}(input: Add{singular_pascal_to_entry_type}For{singular_pascal_from_entry_type}Input) -> ExternResult<()> {{
    create_link(input.base_{from_arg_name}.clone(), input.target_{to_arg_name}.clone(), LinkTypes::{normal_link_type_name}, ())?;
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
        Referenceable::Agent { .. } => {
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
    let plural_snake_to_entry_type = to_referenceable
        .to_string(&Cardinality::Vector)
        .to_case(Case::Snake);

    format!(
        r#"#[hdk_extern]
pub fn get_{plural_snake_to_entry_type}_for_{singular_snake_from_entry_type}({from_arg_name}: {from_hash_type}) -> ExternResult<Vec<Link>> {{
    get_links({from_arg_name}, LinkTypes::{pascal_link_type_name}, None)
}}"#,
    )
}

fn get_links_handler_to_entry(
    from_referenceable: &Referenceable,
    to_entry_type: &EntryTypeReference,
) -> String {
    let from_hash_type = from_referenceable.hash_type().to_string();
    let from_arg_name = from_referenceable.field_name(&Cardinality::Single);

    let pascal_link_type_name = link_type_name(
        from_referenceable,
        &Referenceable::EntryType(to_entry_type.clone()),
    );
    let singular_snake_from_entry_type = from_referenceable
        .to_string(&Cardinality::Single)
        .to_case(Case::Snake);
    let plural_snake_to_entry_type = to_entry_type
        .to_string(&Cardinality::Vector)
        .to_case(Case::Snake);

    format!(
        r#"#[hdk_extern]
pub fn get_{plural_snake_to_entry_type}_for_{singular_snake_from_entry_type}({from_arg_name}: {from_hash_type}) -> ExternResult<Vec<Link>> {{
    get_links({from_arg_name}, LinkTypes::{pascal_link_type_name}, None)
}}"#,
    )
}

fn from_link_hash_type(hash_type: &String) -> String {
    let snake_hash_type = hash_type.to_case(Case::Snake);
    let lower_hash_type = hash_type.to_case(Case::Lower);

    match hash_type.as_str() {
        "AgentPubKey" => format!("AgentPubKey::from(link.target.clone().into_entry_hash().ok_or(wasm_error!(WasmErrorInner::Guest(String::from(\"No entry_hash associated with link\"))))?)"),
        _ => format!("link.target.clone().into_{}().ok_or(wasm_error!(WasmErrorInner::Guest(String::from(\"No {} associated with link\"))))?", snake_hash_type, lower_hash_type),
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
    let links = get_links(input.target_{to_arg_name}.clone(), LinkTypes::{inverse_link_type_name}, None)?;

    for link in links {{
        if {from_inverse}.eq(&input.base_{from_arg_name}) {{
            delete_link(link.create_link_hash)?;
        }}
    }}"#
        ),
        false => format!(""),
    };
    let bidireccional_get_deleted = match bidireccional {
        true => format!(
            r#"
#[hdk_extern]
pub fn get_deleted_{singular_snake_from_entry_type}_for_{singular_snake_to_entry_type}(
    {to_arg_name}: {to_hash_type},
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {{
    let details = get_link_details({to_arg_name}, LinkTypes::{pascal_link_type_name}, None)?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| deletes.len() > 0)
        .collect())
}}"#
        ),
        false => format!(""),
    };
    format!(
        r#"#[derive(Serialize, Deserialize, Debug)]
pub struct Remove{singular_pascal_to_entry_type}For{singular_pascal_from_entry_type}Input {{
    pub base_{from_arg_name}: {from_hash_type},
    pub target_{to_arg_name}: {to_hash_type},
}}
#[hdk_extern]
pub fn remove_{singular_snake_to_entry_type}_for_{singular_snake_from_entry_type}(input: Remove{singular_pascal_to_entry_type}For{singular_pascal_from_entry_type}Input ) -> ExternResult<()> {{
    let links = get_links(input.base_{from_arg_name}.clone(), LinkTypes::{pascal_link_type_name}, None)?;
    
    for link in links {{
        if {from_link}.eq(&input.target_{to_arg_name}) {{
            delete_link(link.create_link_hash)?;
        }}
    }}
    {bidireccional_remove}

    Ok(())        
}}

#[hdk_extern]
pub fn get_deleted_{singular_snake_to_entry_type}_for_{singular_snake_from_entry_type}(
    {from_arg_name}: {from_hash_type},
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {{
    let details = get_link_details({from_arg_name}, LinkTypes::{pascal_link_type_name}, None)?;
    Ok(details
        .into_inner()
        .into_iter()
        .filter(|(_link, deletes)| deletes.len() > 0)
        .collect())
}}
{bidireccional_get_deleted}"#
    )
}

fn normal_handlers(
    integrity_zome_name: &String,
    from_referenceable: &Referenceable,
    to_referenceable: &Referenceable,
    delete: bool,
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

    let delete_link_handler = match delete {
        true => remove_link_handlers(from_referenceable, to_referenceable, bidireccional),
        false => String::from(""),
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
        delete_link_handler
    )
}

pub fn add_link_type_functions_to_coordinator(
    coordinator_zome_file_tree: ZomeFileTree,
    integrity_zome_name: &String,
    link_type_name: &String,
    from_referenceable: &Referenceable,
    to_referenceable: &Option<Referenceable>,
    delete: bool,
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
    let crate_src_path = coordinator_zome_file_tree.zome_crate_path.join("src");

    let lib_rs_path = crate_src_path.join("lib.rs");

    let mut file_tree = coordinator_zome_file_tree.dna_file_tree.file_tree();

    let handlers = match to_referenceable {
        None => metadata_handlers(integrity_zome_name, link_type_name, from_referenceable),
        Some(r) => normal_handlers(
            integrity_zome_name,
            from_referenceable,
            r,
            delete,
            bidireccional,
        ),
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
