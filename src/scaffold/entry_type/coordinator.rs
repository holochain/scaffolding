use std::{ffi::OsString, path::PathBuf};

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{insert_file, map_file, map_rust_files},
    scaffold::{
        dna::DnaFileTree,
        entry_type::definitions::FieldDefinition,
        link_type::{coordinator::get_links_handler, link_type_name},
        zome::ZomeFileTree,
    },
    utils::unparse,
};

use super::{
    crud::Crud,
    definitions::{Cardinality, EntryDefinition, FieldType},
    integrity::find_ending_match_expr_in_block,
};

pub fn no_update_read_handler(entry_def: &EntryDefinition) -> TokenStream {
    let hash_type = entry_def.referenceable().hash_type().to_string();
    let snake_entry_def_name = entry_def.name.to_case(Case::Snake);

    match entry_def.referenceable().hash_type() {
        FieldType::ActionHash => {
            let get_entry_def_function_name = format_ident!("get_{snake_entry_def_name}");
            let entry_hash_param = format_ident!("{snake_entry_def_name}_hash");
            let entry_hash_param_type = format_ident!("{hash_type}");

            quote! {
                #[hdk_extern]
                pub fn #get_entry_def_function_name(#entry_hash_param: #entry_hash_param_type) -> ExternResult<Option<Record>> {
                    let Some(details) = get_details(#entry_hash_param, GetOptions::default())? else {
                        return Ok(None);
                    };
                    match details {
                        Details::Record(details) => Ok(Some(details.record)),
                        _ => Err(wasm_error!(WasmErrorInner::Guest("Malformed get details response".to_string()))),
                    }
                }
            }
        }
        FieldType::EntryHash => {
            let get_entry_def_function_name = format_ident!("get_{snake_entry_def_name}");
            let entry_hash_param = format_ident!("{snake_entry_def_name}_hash");
            let entry_hash_param_type = format_ident!("{hash_type}");

            quote! {
                #[hdk_extern]
                pub fn #get_entry_def_function_name(#entry_hash_param: #entry_hash_param_type) -> ExternResult<Option<Record>> {
                    let Some(details) = get_details(#entry_hash_param, GetOptions::default())? else {
                        return Ok(None);
                    };
                    match details {
                        Details::Entry(details) => Ok(Some(Record::new(details.actions[0].clone(), Some(details.entry)))),
                        _ => Err(wasm_error!(WasmErrorInner::Guest("Malformed get details response".to_string()))),
                    }
                }
            }
        }
        _ => quote! {},
    }
}

pub fn read_handler_without_linking_to_updates(entry_def: &EntryDefinition) -> TokenStream {
    let snake_entry_def_name = entry_def.name.clone();
    let original_hash_param_name = format_ident!("original_{snake_entry_def_name}_hash");

    let get_original_function_name = format_ident!("get_original_{snake_entry_def_name}");

    let get_original_function = quote! {
        #[hdk_extern]
        pub fn #get_original_function_name(#original_hash_param_name: ActionHash) -> ExternResult<Option<Record>> {
            let Some(details) = get_details(#original_hash_param_name, GetOptions::default())? else {
                return Ok(None);
            };
            match details {
                Details::Record(details) => Ok(Some(details.record)),
                _ => Err(wasm_error!(WasmErrorInner::Guest("Malformed get details response".to_string()))),
            }
        }
    };

    let get_latest_function_name = format_ident!("get_latest_{snake_entry_def_name}");

    let get_latest_function = quote! {
        #[hdk_extern]
        pub fn #get_latest_function_name(#original_hash_param_name: ActionHash) -> ExternResult<Option<Record>> {
            let Some(details) = get_details(#original_hash_param_name, GetOptions::default())? else {
                return Ok(None);
            };

            let record_details = match details {
                Details::Entry(_) => Err(wasm_error!(WasmErrorInner::Guest("Malformed details".into()))),
                Details::Record(record_details) => Ok(record_details)
            }?;

            match record_details.updates.last() {
                Some(update) => #get_latest_function_name(update.action_address().clone()),
                None => Ok(Some(record_details.record)),
            }
        }
    };

    let get_all_revisions_function_name =
        format_ident!("get_all_revisions_for_{snake_entry_def_name}");

    let get_all_revisions = quote! {
        #[hdk_extern]
        pub fn #get_all_revisions_function_name(#original_hash_param_name: ActionHash) -> ExternResult<Vec<Record>> {
            let Some(Details::Record(details)) = get_details(#original_hash_param_name, GetOptions::default())? else {
                return Ok(vec![]);
            };

            let mut records = vec![details.record];

            for update in details.updates {
                let mut update_records = #get_all_revisions_function_name(update.action_address().clone())?;
                records.append(&mut update_records);
            }

            Ok(records)
        }
    };

    quote! {
        #get_original_function

        #get_latest_function

        #get_all_revisions
    }
}

pub fn updates_link_name(entry_def_name: &str) -> String {
    format!("{}Updates", entry_def_name.to_case(Case::Pascal))
}

pub fn read_handler_with_linking_to_updates(entry_def: &EntryDefinition) -> TokenStream {
    let snake_entry_def_name = entry_def.snake_case_name();

    let updates_link_name = format_ident!("{}", updates_link_name(&entry_def.name));
    let original_hash_param_name = format_ident!("original_{snake_entry_def_name}_hash");

    let get_latest_function_name = format_ident!("get_latest_{snake_entry_def_name}");
    let latest_hash_variable_name = format_ident!("latest_{snake_entry_def_name}_hash");

    let get_latest_function = quote! {
        #[hdk_extern]
        pub fn #get_latest_function_name(#original_hash_param_name: ActionHash) -> ExternResult<Option<Record>> {
            let links = get_links(
                GetLinksInputBuilder::try_new(#original_hash_param_name.clone(), LinkTypes::#updates_link_name)?.build(),
            )?;

            let latest_link = links.into_iter().max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));

            let #latest_hash_variable_name = match latest_link {
                Some(link) => link.target.clone().into_action_hash().ok_or(wasm_error!(
                    WasmErrorInner::Guest("No action hash associated with link".to_string())
                ))?,
                None => #original_hash_param_name.clone()
            };

            get(#latest_hash_variable_name, GetOptions::default())
        }
    };

    let get_original_function_name = format_ident!("get_original_{snake_entry_def_name}");

    let get_original_function = quote! {
        #[hdk_extern]
        pub fn #get_original_function_name(#original_hash_param_name: ActionHash) -> ExternResult<Option<Record>> {
            let Some(details) = get_details(#original_hash_param_name, GetOptions::default())? else {
                return Ok(None);
            };
            match details {
                Details::Record(details) => Ok(Some(details.record)),
                _ => Err(wasm_error!(WasmErrorInner::Guest("Malformed get details response".to_string()))),
            }
        }
    };

    let get_all_revisions_function_name =
        format_ident!("get_all_revisions_for_{snake_entry_def_name}");

    let get_all_revisions_function_name = quote! {
        #[hdk_extern]
        pub fn #get_all_revisions_function_name(#original_hash_param_name: ActionHash) -> ExternResult<Vec<Record>> {
            let Some(original_record) = #get_original_function_name(#original_hash_param_name.clone())? else {
                return Ok(vec![]);
            };

            let links = get_links(
                GetLinksInputBuilder::try_new(#original_hash_param_name.clone(), LinkTypes::#updates_link_name)?.build(),
            )?;

            let get_input: Vec<GetInput> = links
                .into_iter()
                .map(|link| Ok(GetInput::new(
                    link.target.into_action_hash().ok_or(wasm_error!(WasmErrorInner::Guest("No action hash associated with link".to_string())))?.into(),
                    GetOptions::default(),
                )))
                .collect::<ExternResult<Vec<GetInput>>>()?;

            // load the records for all the links
            let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
            let mut records: Vec<Record> = records.into_iter().flatten().collect();
            records.insert(0, original_record);

            Ok(records)
        }
    };

    quote! {
        #get_latest_function

        #get_original_function

        #get_all_revisions_function_name
    }
}

pub fn create_link_for_cardinality(
    entry_def: &EntryDefinition,
    field_name: &str,
    link_type_name: &str,
    cardinality: &Cardinality,
) -> TokenStream {
    let snake_entry_def_name = entry_def.snake_case_name();
    let link_target = if entry_def.reference_entry_hash {
        format_ident!("{snake_entry_def_name}_entry_hash")
    } else {
        format_ident!("{snake_entry_def_name}_hash")
    };

    let link_type_name = format_ident!("{link_type_name}");
    let field_name = format_ident!("{field_name}");
    let snake_entry_def_name = format_ident!("{snake_entry_def_name}");

    match cardinality {
        Cardinality::Single => quote! {
            create_link(
                #snake_entry_def_name.#field_name.clone(),
                #link_target.clone(),
                LinkTypes::#link_type_name,
                (),
            )?;
        },
        Cardinality::Option => quote! {
            if let Some(base) = #snake_entry_def_name.#field_name.clone() {
                create_link(base, #link_target.clone(), LinkTypes::#link_type_name, ())?;
            }
        },
        Cardinality::Vector => quote! {
            for base in #snake_entry_def_name.#field_name.clone() {
                create_link(base, #link_target.clone(), LinkTypes::#link_type_name, ())?;
            }
        },
    }
}

pub fn create_handler(entry_def: &EntryDefinition) -> TokenStream {
    let pascal_entry_def_name = format_ident!("{}", entry_def.pascal_case_name());
    let snake_entry_def_name = format_ident!("{}", entry_def.snake_case_name());

    let linked_from_count = entry_def
        .fields
        .iter()
        .filter(|f| f.linked_from.is_some())
        .count();

    let entry_hash_variable_name = format_ident!("{}_entry_hash", entry_def.snake_case_name());
    let mut create_links = match entry_def.reference_entry_hash {
        true if linked_from_count > 0 => vec![quote! {
            let #entry_hash_variable_name = hash_entry(&#snake_entry_def_name)?;
        }],
        _ => vec![],
    };

    for f in &entry_def.fields {
        if let Some(linked_from) = &f.linked_from {
            create_links.push(create_link_for_cardinality(
                entry_def,
                &f.field_name,
                &link_type_name(linked_from, &entry_def.referenceable()),
                &f.cardinality,
            ));
        }
    }

    let create_entry_function_name = format_ident!("create_{snake_entry_def_name}");
    let entry_hash_variable_name = format_ident!("{snake_entry_def_name}_hash");

    quote! {
        #[hdk_extern]
        pub fn #create_entry_function_name(#snake_entry_def_name: #pascal_entry_def_name) -> ExternResult<Record> {
            let #entry_hash_variable_name = create_entry(
                &EntryTypes::#pascal_entry_def_name(#snake_entry_def_name.clone())
            )?;
            #(#create_links)*

            let record = get(#entry_hash_variable_name.clone(), GetOptions::default())?
                .ok_or(wasm_error!(
                    WasmErrorInner::Guest("Could not find the newly created pascal_entry_def_name".to_string())
                ))?;
            Ok(record)
        }
    }
}

pub fn update_handler(
    entry_def: &EntryDefinition,
    link_from_original_to_each_update: bool,
) -> TokenStream {
    if link_from_original_to_each_update {
        update_handler_linking_on_each_update(entry_def)
    } else {
        update_handler_without_linking_on_each_update(entry_def)
    }
}

pub fn update_handler_without_linking_on_each_update(entry_def: &EntryDefinition) -> TokenStream {
    let snake_entry_def_name = format_ident!("{}", entry_def.snake_case_name());
    let pascal_entry_def_name = format_ident!("{}", entry_def.pascal_case_name());

    let update_input_struct = format_ident!("Update{pascal_entry_def_name}Input");
    let previous_entry_def_hash = format_ident!("previous_{snake_entry_def_name}_hash");
    let updated_entry_def = format_ident!("updated_{snake_entry_def_name}");

    let update_entry_def_function_name = format_ident!("update_{snake_entry_def_name}");
    let updated_entry_hash_variable_name = format_ident!("updated_{snake_entry_def_name}_hash");

    quote! {
        #[derive(Serialize, Deserialize, Debug)]
        pub struct #update_input_struct {
            pub #previous_entry_def_hash: ActionHash,
            pub #updated_entry_def: #pascal_entry_def_name
        }

        #[hdk_extern]
        pub fn #update_entry_def_function_name(input: #update_input_struct) -> ExternResult<Record> {
            let #updated_entry_hash_variable_name = update_entry(
                input.#previous_entry_def_hash, &input.#updated_entry_def
            )?;

            let record = get(#updated_entry_hash_variable_name.clone(), GetOptions::default())?
                .ok_or(wasm_error!(
                    WasmErrorInner::Guest("Could not find the newly updated #pascal_entry_def_name".to_string())
                ))?;

            Ok(record)
        }
    }
}

pub fn update_handler_linking_on_each_update(entry_def: &EntryDefinition) -> TokenStream {
    let snake_entry_def_name = entry_def.snake_case_name();
    let pascal_entry_def_name = entry_def.pascal_case_name();
    let link_type_variant_name = updates_link_name(&entry_def.name);

    let update_entry_def_function_name = format_ident!("update_{snake_entry_def_name}");

    let entry_def_struct = format_ident!("{pascal_entry_def_name}");
    let update_input_type_struct = format_ident!("Update{pascal_entry_def_name}Input");

    let link_type_variant_name = format_ident!("{link_type_variant_name}");
    let updated_entry_def_name = format_ident!("updated_{snake_entry_def_name}");

    let original_entry_def_hash = format_ident!("original_{snake_entry_def_name}_hash");
    let previous_entry_def_hash = format_ident!("previous_{snake_entry_def_name}_hash");
    let updated_entry_def_hash = format_ident!("updated_{snake_entry_def_name}_hash");

    quote! {
        #[derive(Serialize, Deserialize, Debug)]
        pub struct #update_input_type_struct {
            pub #original_entry_def_hash: ActionHash,
            pub #previous_entry_def_hash: ActionHash,
            pub #updated_entry_def_name: #entry_def_struct
        }

        #[hdk_extern]
        pub fn #update_entry_def_function_name(input: #update_input_type_struct) -> ExternResult<Record> {
            let #updated_entry_def_hash = update_entry(
                input.#previous_entry_def_hash.clone(),
                &input.#updated_entry_def_name,
            )?;

            create_link(
                input.#original_entry_def_hash.clone(),
                #updated_entry_def_hash.clone(),
                LinkTypes::#link_type_variant_name,
                (),
            )?;

            let record = get(#updated_entry_def_hash.clone(), GetOptions::default())?
                .ok_or(wasm_error!(
                    WasmErrorInner::Guest("Could not find the newly updated #pascal_entry_def_name".to_string())
                ))?;

            Ok(record)
        }
    }
}

pub fn delete_handler(entry_def: &EntryDefinition) -> TokenStream {
    let pascal_entry_def_name = format_ident!("{}", entry_def.name.to_case(Case::Pascal));
    let snake_entry_def_name = format_ident!("{}", entry_def.name.to_case(Case::Snake));

    let original_entry_hash = format_ident!("original_{}_hash", snake_entry_def_name);

    let linked_from_fields: Vec<FieldDefinition> = entry_def
        .fields
        .iter()
        .filter(|field| field.linked_from.is_some())
        .cloned()
        .collect();

    let delete_depending_links = if !linked_from_fields.is_empty() {
        let mut delete_links = Vec::new();
        for linked_from_field in linked_from_fields {
            let linked_from = linked_from_field
                .linked_from
                .expect("Linked from is none after we filtered for some");
            let field_name = format_ident!("{}", linked_from_field.field_name);
            let link_type = format_ident!(
                "{}",
                link_type_name(&linked_from, &entry_def.referenceable())
            );
            let delete_this_link = match linked_from_field.cardinality {
                Cardinality::Single => quote! {
                    let links = get_links(
                        GetLinksInputBuilder::try_new(#snake_entry_def_name.#field_name.clone(), LinkTypes::#link_type)?.build(),
                    )?;
                    for link in links {
                        if let Some(action_hash) = link.target.into_action_hash() {
                            if action_hash == #original_entry_hash {
                                delete_link(link.create_link_hash)?;
                            }
                        }
                    }
                },
                Cardinality::Option => quote! {
                    if let Some(base_address) = #snake_entry_def_name.#field_name.clone() {
                        let links = get_links(
                            GetLinksInputBuilder::try_new(base_address, LinkTypes::#link_type)?.build(),
                        )?;
                        for link in links {
                            if let Some(action_hash) = link.target.into_action_hash() {
                                if action_hash == #original_entry_hash {
                                    delete_link(link.create_link_hash)?;
                                }
                            }
                        }
                    }
                },
                Cardinality::Vector => quote! {
                    for base_address in #snake_entry_def_name.#field_name {
                        let links = get_links(
                            GetLinksInputBuilder::try_new(base_address.clone(), LinkTypes::#link_type)?.build(),
                        )?;
                        for link in links {
                            if let Some(action_hash) = link.target.into_action_hash() {
                                if action_hash == #original_entry_hash {
                                    delete_link(link.create_link_hash)?;
                                }
                            }
                        }
                    }
                },
            };
            delete_links.push(delete_this_link);
        }

        quote! {
            let details = get_details(#original_entry_hash.clone(), GetOptions::default())?
                .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("#pascal_entry_def_name not found"))))?;
            let record = match details {
                Details::Record(details) => Ok(details.record),
                _ => Err(wasm_error!(WasmErrorInner::Guest(String::from(
                    "Malformed get details response"
                )))),
            }?;
            let entry = record
                .entry()
                .as_option()
                .ok_or(wasm_error!(
                    WasmErrorInner::Guest("#pascal_entry_def_name record has no entry".to_string())
                ))?;
            let #snake_entry_def_name = <#pascal_entry_def_name>::try_from(entry)?;
            #(#delete_links)*
        }
    } else {
        quote! {}
    };

    let delete_function_name = format_ident!("delete_{}", snake_entry_def_name);
    let get_all_deletes_function_name =
        format_ident!("get_all_deletes_for_{}", snake_entry_def_name);
    let get_oldest_delete_function_name =
        format_ident!("get_oldest_delete_for_{}", snake_entry_def_name);

    quote! {
        #[hdk_extern]
        pub fn #delete_function_name(#original_entry_hash: ActionHash) -> ExternResult<ActionHash> {
            #delete_depending_links
            delete_entry(#original_entry_hash)
        }

        #[hdk_extern]
        pub fn #get_all_deletes_function_name(
            #original_entry_hash: ActionHash,
        ) -> ExternResult<Option<Vec<SignedActionHashed>>> {
            let Some(details) = get_details(#original_entry_hash, GetOptions::default())? else {
                return Ok(None);
            };
            match details {
                Details::Entry(_) => Err(wasm_error!(WasmErrorInner::Guest("Malformed details".into()))),
                Details::Record(record_details) => Ok(Some(record_details.deletes)),
            }
        }

        #[hdk_extern]
        pub fn #get_oldest_delete_function_name(
            #original_entry_hash: ActionHash,
        ) -> ExternResult<Option<SignedActionHashed>> {
            let Some(mut deletes) = #get_all_deletes_function_name(#original_entry_hash)? else {
                return Ok(None);
            };
            deletes.sort_by(|delete_a, delete_b| delete_a.action().timestamp().cmp(&delete_b.action().timestamp()));
            Ok(deletes.first().cloned())
        }
    }
}

fn initial_crud_handlers(
    integrity_zome_name: &str,
    entry_def: &EntryDefinition,
    crud: &Crud,
    link_from_original_to_each_update: bool,
) -> syn::File {
    let integrity_zome_name = format_ident!("{integrity_zome_name}");

    let create_handler = create_handler(entry_def);
    let mut update_delete_and_read_handlers = Vec::new();

    if !crud.update {
        update_delete_and_read_handlers.push(no_update_read_handler(entry_def));
    } else if link_from_original_to_each_update {
        update_delete_and_read_handlers.push(read_handler_with_linking_to_updates(entry_def));
    } else {
        update_delete_and_read_handlers.push(read_handler_without_linking_to_updates(entry_def));
    }

    if crud.update {
        update_delete_and_read_handlers
            .push(update_handler(entry_def, link_from_original_to_each_update));
    }

    if crud.delete {
        update_delete_and_read_handlers.push(delete_handler(entry_def));
    }

    for f in &entry_def.fields {
        if let Some(linked_from) = &f.linked_from {
            update_delete_and_read_handlers.push(get_links_handler(
                linked_from,
                &entry_def.referenceable(),
                crud.delete,
            ));
        }
    }

    syn::parse_quote! {
        use hdk::prelude::*;
        use #integrity_zome_name::*;

        #create_handler

        #(#update_delete_and_read_handlers)*
    }
}

fn signal_has_entry_types(signal_enum: &syn::ItemEnum) -> bool {
    signal_enum
        .variants
        .iter()
        .any(|v| v.ident == "EntryCreated")
}

fn signal_action_has_entry_types(expr_match: &syn::ExprMatch) -> bool {
    expr_match.arms.iter().any(|arm| {
        if let syn::Pat::TupleStruct(tuple_struct_pat) = &arm.pat {
            if let Some(first_segment) = tuple_struct_pat.path.segments.last() {
                return first_segment.ident == "Create";
            }
        }
        false
    })
}

fn signal_entry_types_variants() -> ScaffoldResult<Vec<syn::Variant>> {
    Ok(vec![
        syn::parse_quote! {
            EntryCreated {
                action: SignedActionHashed,
                app_entry: EntryTypes,
            }
        },
        syn::parse_quote! {
            EntryUpdated {
                action: SignedActionHashed,
                app_entry: EntryTypes,
                original_app_entry: EntryTypes,
            }
        },
        syn::parse_quote! {
            EntryDeleted {
                action: SignedActionHashed,
                original_app_entry: EntryTypes,
            }
        },
    ])
}

fn signal_action_match_arms() -> ScaffoldResult<Vec<syn::Arm>> {
    Ok(vec![
        syn::parse_quote! {
            Action::Create(_create) => {
                if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                    emit_signal(Signal::EntryCreated { action, app_entry })?;
                }
                Ok(())
            }
        },
        syn::parse_quote! {
            Action::Update(update) => {
                if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                    if let Ok(Some(original_app_entry)) =
                        get_entry_for_action(&update.original_action_address)
                    {
                        emit_signal(Signal::EntryUpdated {
                            action,
                            app_entry,
                            original_app_entry,
                        })?;
                    }
                }
                Ok(())
            }
        },
        syn::parse_quote! {
            Action::Delete(delete) => {
                if let Ok(Some(original_app_entry)) = get_entry_for_action(&delete.deletes_address) {
                    emit_signal(Signal::EntryDeleted {
                        action,
                        original_app_entry,
                    })?;
                }
                Ok(())
            }
        },
    ])
}

pub fn add_crud_functions_to_coordinator(
    zome_file_tree: ZomeFileTree,
    integrity_zome_name: &str,
    entry_def: &EntryDefinition,
    crud: &Crud,
    link_from_original_to_each_update: bool,
) -> ScaffoldResult<ZomeFileTree> {
    let dna_manifest_path = zome_file_tree.dna_file_tree.dna_manifest_path.clone();
    let zome_manifest = zome_file_tree.zome_manifest.clone();
    let entry_def_snake_case_name = entry_def.snake_case_name();

    // 1. Create an ENTRY_DEF_NAME.rs in "src/", with the appropriate crud functions
    let crate_src_path = zome_file_tree.zome_crate_path.join("src");

    let mut file_tree = zome_file_tree.dna_file_tree.file_tree();

    let file = unparse(&initial_crud_handlers(
        integrity_zome_name,
        entry_def,
        crud,
        link_from_original_to_each_update,
    ));

    insert_file(
        &mut file_tree,
        &crate_src_path.join(format!("{}.rs", &entry_def_snake_case_name)),
        &file,
    )?;

    // 2. Add this file as a module in the entry point for the crate
    let lib_rs_path = crate_src_path.join("lib.rs");

    map_file(&mut file_tree, &lib_rs_path, |contents| {
        Ok(format!(
            r#"pub mod {};
{contents}"#,
            &entry_def_snake_case_name
        ))
    })?;

    let v = crate_src_path
        .iter()
        .map(|s| s.to_os_string())
        .collect::<Vec<OsString>>();

    map_rust_files(
        file_tree
            .path_mut(&mut v.iter())
            .ok_or(ScaffoldError::PathNotFound(crate_src_path.clone()))?,
        |file_path, mut file| {
            if file_path == PathBuf::from("lib.rs") {
                let mut first_entry_type_scaffolded = false;

                for item in &mut file.items {
                    if let syn::Item::Enum(item_enum) = item {
                        if item_enum.ident == "Signal" && !signal_has_entry_types(item_enum) {
                            first_entry_type_scaffolded = true;
                            for v in signal_entry_types_variants()? {
                                item_enum.variants.push(v);
                            }
                        }
                    }

                    if let syn::Item::Fn(item_fn) = item {
                        if item_fn.sig.ident == "signal_action" {
                            if find_ending_match_expr_in_block(&mut item_fn.block).is_none() {
                                item_fn.block = Box::new(syn::parse_quote! {
                                    {
                                        match action.hashed.content.clone() {
                                            _ => Ok(())
                                        }
                                    }
                                });
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
                    file.items.push(syn::parse_quote! {
                        fn get_entry_for_action(action_hash: &ActionHash) -> ExternResult<Option<EntryTypes>> {
                            let record = match get_details(action_hash.clone(), GetOptions::default())? {
                                Some(Details::Record(record_details)) => record_details.record,
                                _ => return Ok(None),
                            };
                            let entry = match record.entry().as_option() {
                                Some(entry) => entry,
                                None => return Ok(None),
                            };
                            let (zome_index, entry_index) = match record.action().entry_type() {
                                Some(EntryType::App(AppEntryDef {
                                    zome_index,
                                    entry_index,
                                    ..
                                })) => (zome_index, entry_index),
                                _ => return Ok(None),
                            };
                            EntryTypes::deserialize_from_type(*zome_index, *entry_index, entry)
                        }
                    });
                }
            }
            Ok(file)
        },
    )?;

    let dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;
    let zome_file_tree = ZomeFileTree::from_zome_manifest(dna_file_tree, zome_manifest)?;

    Ok(zome_file_tree)
}
