use convert_case::{Case, Casing};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{ffi::OsString, path::PathBuf};

use crate::error::{ScaffoldError, ScaffoldResult};
use crate::file_tree::insert_file;
use crate::scaffold::dna::DnaFileTree;
use crate::scaffold::zome::coordinator::find_extern_function_in_zomes;
use crate::scaffold::zome::utils::get_coordinator_zomes_for_integrity;
use crate::utils::unparse;
use crate::{
    file_tree::{find_map_rust_files, map_file, map_rust_files},
    scaffold::zome::ZomeFileTree,
};

use super::crud::Crud;
use super::definitions::{
    Cardinality, EntryDefinition, EntryTypeReference, FieldDefinition, Referenceable,
};

pub fn render_entry_definition_struct(entry_def: &EntryDefinition) -> ScaffoldResult<TokenStream> {
    let name: syn::Expr = syn::parse_str(entry_def.name.to_case(Case::Pascal).as_str())?;

    let fields: Vec<TokenStream> = entry_def
        .fields
        .iter()
        .map(|field_def| {
            let name: syn::Expr =
                syn::parse_str(field_def.field_name.to_case(Case::Snake).as_str())?;
            let rust_type = field_def.rust_type();
            Ok(quote! {  #name: #rust_type })
        })
        .collect::<ScaffoldResult<Vec<TokenStream>>>()?;
    Ok(quote! {

      pub struct #name {
        #(pub #fields),*
      }

    })
}

pub fn render_entry_definition_file(
    entry_def: &EntryDefinition,
    crud: &Crud,
) -> ScaffoldResult<syn::File> {
    let entry_def_token_stream = render_entry_definition_struct(entry_def)?;
    let name_pascal: syn::Expr = syn::parse_str(entry_def.name.to_case(Case::Pascal).as_str())?;
    let plural_name_title =
        pluralizer::pluralize(entry_def.name.as_str(), 2, false).to_case(Case::Title);

    let type_definitions: Vec<TokenStream> = entry_def
        .fields
        .iter()
        .filter_map(|field_def| field_def.field_type.rust_type_definition())
        .collect();

    let validate_update_fn =
        format_ident!("validate_update_{}", entry_def.name.to_case(Case::Snake));
    let new_entry_arg = format_ident!("_{}", entry_def.name.to_case(Case::Snake));
    let original_entry_arg = format_ident!("_original_{}", entry_def.name.to_case(Case::Snake));

    let updated_invalid_reason = format!("{} cannot be updated", plural_name_title);

    let validate_update_result: TokenStream = match crud.update {
        true => quote! {
            /// TODO: add the appropriate validation rules
            Ok(ValidateCallbackResult::Valid)
        },
        false => quote! {
             Ok(ValidateCallbackResult::Invalid(String::from(#updated_invalid_reason)))
        },
    };
    let validate_update: TokenStream = quote! {
        pub fn #validate_update_fn(
            _action: Update,
            #new_entry_arg: #name_pascal,
            _original_action: EntryCreationAction,
            #original_entry_arg: #name_pascal
        ) -> ExternResult<ValidateCallbackResult> {
            #validate_update_result
        }
    };

    let validate_delete_fn =
        format_ident!("validate_delete_{}", entry_def.name.to_case(Case::Snake));
    let deleted_post_arg = format_ident!("_original_{}", entry_def.name.to_case(Case::Snake));

    let deleted_invalid_reason = format!("{} cannot be deleted", plural_name_title);

    let validate_delete_result: TokenStream = match crud.delete {
        true => quote! {
            /// TODO: add the appropriate validation rules
            Ok(ValidateCallbackResult::Valid)
        },
        false => quote! {
             Ok(ValidateCallbackResult::Invalid(String::from(#deleted_invalid_reason)))
        },
    };
    let validate_delete: TokenStream = quote! {
        pub fn #validate_delete_fn(
            _action: Delete,
            _original_action: EntryCreationAction,
            #deleted_post_arg: #name_pascal
        ) -> ExternResult<ValidateCallbackResult> {
            #validate_delete_result
        }
    };

    let validate_create_fn =
        format_ident!("validate_create_{}", entry_def.name.to_case(Case::Snake));

    let deps: Vec<(FieldDefinition, EntryTypeReference)> = entry_def
        .fields
        .iter()
        .filter_map(|f| match &f.linked_from {
            Some(Referenceable::EntryType(entry_type_reference)) => {
                Some((f.clone(), entry_type_reference.clone()))
            }
            _ => None,
        })
        .collect();

    let create_new_entry_arg = match deps.len() {
        0 => format_ident!("_{}", entry_def.name.to_case(Case::Snake)),
        _ => format_ident!("{}", entry_def.name.to_case(Case::Snake)),
    };
    let deps_validation: Vec<TokenStream> = deps
        .into_iter()
        .map(|(field_def, reference)| {
            let field_name = format_ident!("{}",field_def.field_name);
            let dependant_entry_type_snake = format_ident!("_{}", reference.entry_type.to_case(Case::Snake));
            let dependant_entry_type_pascal = format_ident!("{}", reference.entry_type.to_case(Case::Pascal));
            match (field_def.cardinality, reference.reference_entry_hash) {
                (Cardinality::Single, false) => quote! {
                    let record = must_get_valid_record(#create_new_entry_arg.#field_name.clone())?;

                    let #dependant_entry_type_snake: crate::#dependant_entry_type_pascal = record.entry().to_app_option()
                        .map_err(|e| wasm_error!(e))?
                        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("Dependant action must be accompanied by an entry"))))?;
                },
                (Cardinality::Option, false) => quote! {
                    if let Some(action_hash) = #create_new_entry_arg.#field_name.clone() {
                        let record = must_get_valid_record(action_hash)?;

                        let #dependant_entry_type_snake: crate::#dependant_entry_type_pascal = record.entry().to_app_option()
                            .map_err(|e| wasm_error!(e))?
                            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("Dependant action must be accompanied by an entry"))))?;
                    }
                },
                (Cardinality::Vector, false) => quote! {
                    for action_hash in #create_new_entry_arg.#field_name.clone() {
                        let record = must_get_valid_record(action_hash)?;

                        let #dependant_entry_type_snake: crate::#dependant_entry_type_pascal = record.entry().to_app_option()
                            .map_err(|e| wasm_error!(e))?
                            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("Dependant action must be accompanied by an entry"))))?;
                    }
                },
                (Cardinality::Single,true) => quote! {
                    let entry = must_get_entry(#create_new_entry_arg.#field_name.clone())?;

                    let #dependant_entry_type_snake = crate::#dependant_entry_type_pascal::try_from(entry)?;
                },
                (Cardinality::Option, true) => quote! {
                    if let Some(entry_hash) = #create_new_entry_arg.#field_name.clone() {
                        let entry = must_get_entry(entry_hash)?;

                        let #dependant_entry_type_snake = crate::#dependant_entry_type_pascal::try_from(entry)?;
                    }
                },
                (Cardinality::Vector, true) => quote! {
                    for entry_hash in #create_new_entry_arg.#field_name.clone() {
                        let entry = must_get_entry(entry_hash)?;

                        let #dependant_entry_type_snake = crate::#dependant_entry_type_pascal::try_from(entry)?;
                    }
                },
            }
        })
        .collect();

    let token_stream = quote! {
      use hdi::prelude::*;

      #(#type_definitions)*

      #[hdk_entry_helper]
      #[derive(Clone, PartialEq)]
      #entry_def_token_stream

      pub fn #validate_create_fn(
          _action: EntryCreationAction,
          #create_new_entry_arg: #name_pascal
      ) -> ExternResult<ValidateCallbackResult> {
          #(#deps_validation)*

          /// TODO: add the appropriate validation rules
          Ok(ValidateCallbackResult::Valid)
      }

      #validate_update

      #validate_delete
    };

    let file = syn::parse_file(token_stream.to_string().as_str())?;

    Ok(file)
}

pub fn add_entry_type_to_integrity_zome(
    zome_file_tree: ZomeFileTree,
    entry_def: &EntryDefinition,
    crud: &Crud,
) -> ScaffoldResult<ZomeFileTree> {
    let dna_manifest_path = zome_file_tree.dna_file_tree.dna_manifest_path.clone();
    let dna_manifest = zome_file_tree.dna_file_tree.dna_manifest.clone();
    let zome_manifest = zome_file_tree.zome_manifest.clone();

    let snake_entry_def_name = entry_def.name.to_case(Case::Snake);
    let entry_def_file = render_entry_definition_file(entry_def, crud)?;

    let entry_types = get_all_entry_types(&zome_file_tree)?;

    // 1. Create an ENTRY_DEF_NAME.rs in "src/", with the entry definition struct
    let crate_src_path = zome_file_tree.zome_crate_path.join("src");

    let entry_def_path = crate_src_path.join(format!("{}.rs", snake_entry_def_name));

    let mut file_tree = zome_file_tree.dna_file_tree.file_tree();

    insert_file(&mut file_tree, &entry_def_path, &unparse(&entry_def_file))?;

    // 2. Add this file as a module in the entry point for the crate

    let lib_rs_path = crate_src_path.join("lib.rs");

    map_file(&mut file_tree, &lib_rs_path, |s| {
        Ok(format!(
            r#"pub mod {};
pub use {}::*;

{}"#,
            snake_entry_def_name, snake_entry_def_name, s
        ))
    })?;

    let pascal_entry_def_name = entry_def.name.to_case(Case::Pascal);

    let v: Vec<OsString> = crate_src_path
        .clone()
        .iter()
        .map(|s| s.to_os_string())
        .collect();
    // 3. Find the #[hdk_entry_types] macro
    // 3.1 Import the new struct
    // 3.2 Add a variant for the new entry def with the struct as its payload
    map_rust_files(
        file_tree
            .path_mut(&mut v.iter())
            .ok_or(ScaffoldError::PathNotFound(crate_src_path.clone()))?,
        |file_path, mut file| {
            let mut found = false;

            // If there are no entry types definitions in this zome, first add the empty enum
            if entry_types.is_none() && file_path == PathBuf::from("lib.rs") {
                let entry_types_item = syn::parse_quote! {
                    #[derive(Serialize, Deserialize)]
                    #[serde(tag = "type")]
                    #[hdk_entry_types]
                    #[unit_enum(UnitEntryTypes)]
                    pub enum EntryTypes {}
                };

                // Insert the entry types just before LinkTypes or before the first function if LinkTypes doesn't exist
                match file.items.iter().find_position(|item| {
                    if let syn::Item::Enum(item_enum) = item {
                        return item_enum.ident == "LinkTypes";
                    }
                    matches!(item, syn::Item::Fn(_))
                }) {
                    Some((i, _)) => {
                        file.items.insert(i, entry_types_item);
                    }
                    None => file.items.push(entry_types_item),
                }

                // update generic parameters
                for item in &mut file.items {
                    if let syn::Item::Fn(item_fn) = item {
                        if item_fn.sig.ident == "validate" {
                            for stmt in &mut item_fn.block.stmts {
                                if let syn::Stmt::Expr(syn::Expr::Match(match_expr), _) = stmt {
                                    if let syn::Expr::Try(try_expr) = &mut *match_expr.expr {
                                        if let syn::Expr::MethodCall(call) = &mut *try_expr.expr {
                                            if call.method == "flattened" {
                                                if let Some(turbofish) = &mut call.turbofish {
                                                    if let Some(first_arg) =
                                                        turbofish.args.first_mut()
                                                    {
                                                        *first_arg = syn::GenericArgument::Type(
                                                            syn::parse_str::<syn::Type>(
                                                                "EntryTypes",
                                                            )?,
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            file.items = file
                .items
                .into_iter()
                .map(|mut item| {
                    if let syn::Item::Enum(mut item_enum) = item.clone() {
                        if item_enum.attrs.iter().any(|attr| {
                            attr.path()
                                .segments
                                .iter()
                                .any(|s| s.ident == "hdk_entry_types")
                        }) {
                            if item_enum
                                .variants
                                .iter()
                                .any(|v| v.ident == pascal_entry_def_name)
                            {
                                return Err(ScaffoldError::EntryTypeAlreadyExists(
                                    pascal_entry_def_name.clone(),
                                    dna_manifest.name(),
                                    zome_file_tree.zome_manifest.name.to_string(),
                                ));
                            }
                            found = true;
                            let new_variant = syn::parse_str::<syn::Variant>(&format!(
                                "{pascal_entry_def_name}({pascal_entry_def_name})"
                            ))
                            .unwrap();
                            item_enum.variants.push(new_variant);
                            return Ok(syn::Item::Enum(item_enum));
                        }
                    }

                    add_entry_type_to_validation_arms(&mut item, entry_def)?;

                    Ok(item)
                })
                .collect::<ScaffoldResult<Vec<syn::Item>>>()?;

            // If the file is lib.rs, we already have the entry struct imported so no need to import it again
            if found && file_path.file_name() != Some(OsString::from("lib.rs").as_os_str()) {
                file.items
                    .insert(0, syn::parse_str::<syn::Item>("use crate::*;").unwrap());
            }

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
    let zome_file_tree = ZomeFileTree::from_zome_manifest(dna_file_tree, zome_manifest)?;

    Ok(zome_file_tree)
}

pub fn get_all_entry_types(
    zome_file_tree: &ZomeFileTree,
) -> ScaffoldResult<Option<Vec<EntryTypeReference>>> {
    let crate_src_path = zome_file_tree.zome_crate_path.join("src");
    let crate_src_path_iter: Vec<OsString> =
        crate_src_path.iter().map(|s| s.to_os_string()).collect();
    let entry_defs_instances = find_map_rust_files(
        zome_file_tree
            .dna_file_tree
            .file_tree_ref()
            .path(&mut crate_src_path_iter.iter())
            .ok_or(ScaffoldError::PathNotFound(crate_src_path.clone()))?,
        &|_file_path, rust_file| {
            rust_file.items.iter().find_map(|i| {
                if let syn::Item::Enum(item_enum) = i.clone() {
                    if item_enum.attrs.iter().any(|a| {
                        a.path()
                            .segments
                            .iter()
                            .any(|s| s.ident == "hdk_entry_types")
                    }) {
                        return Some(item_enum.clone());
                    }
                }

                None
            })
        },
    );

    match entry_defs_instances.len() {
        0 => Ok(None),
        1 => {
            let entry_def_enum = entry_defs_instances.values().next().unwrap();

            let variants: Vec<String> = entry_def_enum
                .clone()
                .variants
                .into_iter()
                .map(|v| v.ident.to_string())
                .collect();

            let coordinators_for_zome = get_coordinator_zomes_for_integrity(
                &zome_file_tree.dna_file_tree.dna_manifest,
                zome_file_tree.zome_manifest.name.0.as_ref(),
            );

            let mut entry_types: Vec<EntryTypeReference> = Vec::new();

            let entry_hash_type: syn::Type = syn::parse_str("EntryHash")?;

            for v in variants {
                let referenced_by_entry_hash = match find_extern_function_in_zomes(
                    &zome_file_tree.dna_file_tree,
                    &coordinators_for_zome,
                    &format!("get_{}", v.to_case(Case::Snake)),
                )? {
                    Some((_z, item_fn)) => {
                        match item_fn
                            .sig
                            .inputs
                            .first()
                            .expect("Extern function must have one argument")
                        {
                            syn::FnArg::Typed(typed) => entry_hash_type.eq(&(*typed.ty)),
                            _ => false,
                        }
                    }
                    None => false,
                };
                entry_types.push(EntryTypeReference {
                    entry_type: v,
                    reference_entry_hash: referenced_by_entry_hash,
                });
            }

            Ok(Some(entry_types))
        }
        _ => Err(ScaffoldError::MultipleEntryTypesDefsFoundForIntegrityZome(
            zome_file_tree.dna_file_tree.dna_manifest.name(),
            zome_file_tree.zome_manifest.name.0.to_string(),
        )),
    }
}

fn add_entry_type_to_validation_arms(
    item: &mut syn::Item,
    entry_def: &EntryDefinition,
) -> ScaffoldResult<()> {
    let pascal_entry_def_name = entry_def.name.to_case(Case::Pascal);
    let snake_entry_def_name = entry_def.name.to_case(Case::Snake);
    if let syn::Item::Fn(item_fn) = item {
        if item_fn.sig.ident == "validate" {
            for stmt in &mut item_fn.block.stmts {
                if let syn::Stmt::Expr(syn::Expr::Match(match_expr), _) = stmt {
                    if let syn::Expr::Try(try_expr) = &mut *match_expr.expr {
                        if let syn::Expr::MethodCall(call) = &mut *try_expr.expr {
                            if call.method == "flattened" {
                                for arm in &mut match_expr.arms {
                                    if let syn::Pat::TupleStruct(pat_tuple_struct) = &mut arm.pat {
                                        if let Some(path_segment) =
                                            pat_tuple_struct.path.segments.last()
                                        {
                                            let path_segment_str = path_segment.ident.to_string();
                                            if path_segment_str == "StoreRecord" {
                                                handle_store_record_arm(
                                                    arm,
                                                    &pascal_entry_def_name,
                                                    &snake_entry_def_name,
                                                )?;
                                            } else if path_segment_str == "StoreEntry" {
                                                handle_store_entry_arm(
                                                    arm,
                                                    &pascal_entry_def_name,
                                                    &snake_entry_def_name,
                                                )?;
                                            } else if path_segment_str == "RegisterUpdate" {
                                                handle_register_update_arm(
                                                    arm,
                                                    &pascal_entry_def_name,
                                                    &snake_entry_def_name,
                                                )?;
                                            } else if path_segment_str == "RegisterDelete" {
                                                handle_register_delete_arm(
                                                    arm,
                                                    &pascal_entry_def_name,
                                                    &snake_entry_def_name,
                                                )?;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn handle_store_record_arm(
    arm: &mut syn::Arm,
    pascal_entry_def_name: &str,
    snake_entry_def_name: &str,
) -> ScaffoldResult<()> {
    if let Some(op_entry_match_expr) = find_ending_match_expr(&mut arm.body) {
        for op_record_arm in &mut op_entry_match_expr.arms {
            if is_entry(&op_record_arm.pat, "CreateEntry") {
                // Add new entry type to match arm
                if find_ending_match_expr(&mut op_record_arm.body).is_none() {
                    // Change empty invalid to match on entry_type
                    *op_record_arm.body = syn::parse_quote! {match app_entry {}};
                }

                // Add new entry type to match arm
                if let Some(entry_type_match) = find_ending_match_expr(&mut op_record_arm.body) {
                    let new_arm = syn::parse_str(&format!(
                        r#"EntryTypes::{pascal_entry_def_name}({snake_entry_def_name}) => {{
                            validate_create_{snake_entry_def_name}(
                                EntryCreationAction::Create(action),
                                {snake_entry_def_name},
                            )
                        }}"#
                    ))?;
                    entry_type_match.arms.push(new_arm);
                }
            } else if is_entry(&op_record_arm.pat, "UpdateEntry") {
                // Add new entry type to match arm
                if find_ending_match_expr(&mut op_record_arm.body).is_none() {
                    // Change empty invalid to match on entry_type
                    *op_record_arm.body = syn::parse_quote! {
                        {
                            let original_record = must_get_valid_record(original_action_hash)?;
                            let original_action = original_record.action().clone();
                            let original_action = match original_action {
                                Action::Create(create) => EntryCreationAction::Create(create),
                                Action::Update(update) => EntryCreationAction::Update(update),
                                _ => {
                                    return Ok(ValidateCallbackResult::Invalid(
                                        "Original action for an update must be a Create or Update action".to_string()
                                    ));
                                }
                            };
                            match app_entry {}
                        }
                    };
                }

                // Add new entry type to match arm
                if let Some(entry_type_match) = find_ending_match_expr(&mut op_record_arm.body) {
                    let new_arm = syn::parse_str(&format!(
                        r#"EntryTypes::{pascal_entry_def_name}({snake_entry_def_name}) => {{
                            let result = validate_create_{snake_entry_def_name}(EntryCreationAction::Update(action.clone()), {snake_entry_def_name}.clone())?;
                            if let ValidateCallbackResult::Valid = result {{
                                let original_{snake_entry_def_name}: Option<{pascal_entry_def_name}> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_{snake_entry_def_name} = match original_{snake_entry_def_name} {{
                                    Some({snake_entry_def_name}) => {snake_entry_def_name},
                                    None => {{
                                        return Ok(ValidateCallbackResult::Invalid(
                                            "The updated entry type must be the same as the original entry type".to_string()
                                        ));
                                    }}
                                }};
                                validate_update_{snake_entry_def_name}(
                                    action,
                                    {snake_entry_def_name},
                                    original_action,
                                    original_{snake_entry_def_name},
                                )
                            }} else {{
                                Ok(result)
                            }}
                        }},"#,
                    ))?;
                    entry_type_match.arms.push(new_arm);
                }
            } else if is_entry(&op_record_arm.pat, "DeleteEntry") {
                // Add new entry type to match arm
                if find_ending_match_expr(&mut op_record_arm.body).is_none() {
                    // Change empty invalid to match on entry_type
                    *op_record_arm.body = syn::parse_quote! {
                        {
                            let original_record = must_get_valid_record(original_action_hash)?;
                            let original_action = original_record.action().clone();
                            let original_action = match original_action {
                                Action::Create(create) => EntryCreationAction::Create(create),
                                Action::Update(update) => EntryCreationAction::Update(update),
                                _ => {
                                    return Ok(ValidateCallbackResult::Invalid(
                                        "Original action for a delete must be a Create or Update action".to_string()
                                    ));
                                }
                            };
                            let app_entry_type = match original_action.entry_type() {
                                EntryType::App(app_entry_type) => app_entry_type,
                                _ => {
                                    return Ok(ValidateCallbackResult::Valid);
                                }
                            };
                            let entry = match original_record.entry().as_option() {
                                Some(entry) => entry,
                                None => {
                                    return Ok(ValidateCallbackResult::Invalid(
                                        "Original record for a delete must contain an entry".to_string(),
                                    ));
                                }
                            };
                            let original_app_entry = match EntryTypes::deserialize_from_type(
                                app_entry_type.zome_index,
                                app_entry_type.entry_index,
                                entry,
                            )? {
                                Some(app_entry) => app_entry,
                                None => {
                                    return Ok(ValidateCallbackResult::Invalid(
                                        "Original app entry must be one of the defined entry types for this zome"
                                            .to_string(),
                                    ));
                                }
                            };
                            match original_app_entry {}
                        }
                    };
                }

                // Add new entry type to match arm
                if let Some(entry_type_match) = find_ending_match_expr(&mut op_record_arm.body) {
                    let new_arm = syn::parse_str(&format!(
                        r#"EntryTypes::{pascal_entry_def_name}(original_{snake_entry_def_name}) => {{
                            validate_delete_{snake_entry_def_name}(
                                action, 
                                original_action, 
                                original_{snake_entry_def_name},
                            )
                        }}"#
                    ))?;
                    entry_type_match.arms.push(new_arm);
                }
            }
        }
    }
    Ok(())
}

fn handle_store_entry_arm(
    arm: &mut syn::Arm,
    pascal_entry_def_name: &str,
    snake_entry_def_name: &str,
) -> ScaffoldResult<()> {
    if let Some(op_entry_match_expr) = find_ending_match_expr(&mut arm.body) {
        for op_entry_arm in &mut op_entry_match_expr.arms {
            if is_entry(&op_entry_arm.pat, "CreateEntry") {
                // Add new entry type to match arm
                if find_ending_match_expr(&mut op_entry_arm.body).is_none() {
                    // Change empty invalid to match on entry_type
                    *op_entry_arm.body = syn::parse_quote! {match app_entry {}};
                }

                // Add new entry type to match arm
                if let Some(entry_type_match) = find_ending_match_expr(&mut op_entry_arm.body) {
                    let new_arm = syn::parse_str(&format!(
                        r#"EntryTypes::{pascal_entry_def_name}({snake_entry_def_name}) => {{
                                validate_create_{snake_entry_def_name}(
                                    EntryCreationAction::Create(action),
                                    {snake_entry_def_name},
                                )
                        }},"#
                    ))?;
                    entry_type_match.arms.push(new_arm);
                }
            } else if is_entry(&op_entry_arm.pat, "UpdateEntry") {
                // Add new entry type to match arm
                if find_ending_match_expr(&mut op_entry_arm.body).is_none() {
                    // Change empty invalid to match on entry_type
                    *op_entry_arm.body = syn::parse_str::<syn::Expr>("match app_entry {}")?;
                }

                // Add new entry type to match arm
                if let Some(entry_type_match) = find_ending_match_expr(&mut op_entry_arm.body) {
                    let new_arm = syn::parse_str(&format!(
                        r#"EntryTypes::{pascal_entry_def_name}({snake_entry_def_name}) => {{
                                validate_create_{snake_entry_def_name}(
                                    EntryCreationAction::Update(action),
                                    {snake_entry_def_name},
                                )
                        }}"#
                    ))?;
                    entry_type_match.arms.push(new_arm);
                }
            }
        }
    }
    Ok(())
}

fn handle_register_update_arm(
    arm: &mut syn::Arm,
    pascal_entry_def_name: &str,
    snake_entry_def_name: &str,
) -> ScaffoldResult<()> {
    if let Some(op_entry_match_expr) = find_ending_match_expr(&mut arm.body) {
        for op_entry_arm in &mut op_entry_match_expr.arms {
            if let syn::Pat::Struct(pat_struct) = &mut op_entry_arm.pat {
                if let Some(ps) = pat_struct.path.segments.last() {
                    if ps.ident == "Entry" {
                        // Add new entry type to match arm
                        if find_ending_match_expr(&mut op_entry_arm.body).is_none() {
                            // Change empty invalid to match on entry_type
                            *op_entry_arm.body = syn::parse_quote! {
                                {
                                    let original_action = must_get_action(action.clone().original_action_address)?
                                        .action()
                                        .to_owned();
                                    let original_create_action = match EntryCreationAction::try_from(original_action) {
                                        Ok(action) => action,
                                        Err(e) => {
                                            return Ok(ValidateCallbackResult::Invalid(
                                                format!("Expected to get EntryCreationAction from Action: {e:?}")
                                            ));
                                        }
                                    };
                                    match app_entry {}
                                }
                            };
                        }

                        // Add new entry type to match arm
                        if let Some(entry_type_match) =
                            find_ending_match_expr(&mut op_entry_arm.body)
                        {
                            let new_arm = syn::parse_str(&format!(
                                r#"EntryTypes::{pascal_entry_def_name}({snake_entry_def_name}) => {{
                                    let original_app_entry = must_get_valid_record(action.clone().original_action_address)?;
                                    let original_{snake_entry_def_name} = match {pascal_entry_def_name}::try_from(original_app_entry) {{
                                        Ok(entry) => entry,
                                        Err(e) => {{
                                            return Ok(ValidateCallbackResult::Invalid(
                                                format!("Expected to get {pascal_entry_def_name} from Record: {{e:?}}")
                                            ));
                                        }}
                                    }};
                                    validate_update_{snake_entry_def_name}(
                                        action,
                                        {snake_entry_def_name},
                                        original_create_action,
                                        original_{snake_entry_def_name},
                                    )
                                }}"#,
                            ))?;
                            entry_type_match.arms.insert(0, new_arm);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn handle_register_delete_arm(
    arm: &mut syn::Arm,
    pascal_entry_def_name: &str,
    snake_entry_def_name: &str,
) -> ScaffoldResult<()> {
    if find_ending_match_expr(&mut arm.body).is_none() {
        *arm.body = syn::parse_quote! {
            {
                let original_action_hash = delete_entry.clone().action.deletes_address;
                let original_record = must_get_valid_record(original_action_hash)?;
                let original_record_action = original_record.action().clone();
                let original_action = match EntryCreationAction::try_from(original_record_action) {
                    Ok(action) => action,
                    Err(e) => return Ok(ValidateCallbackResult::Invalid(format!(
                        "Expected to get EntryCreationAction from Action: {e:?}"
                    ))),
                };
                let app_entry_type = match original_action.entry_type() {
                    EntryType::App(app_entry_type) => app_entry_type,
                    _ => {
                        return Ok(ValidateCallbackResult::Valid);
                    }
                };
                let entry = match original_record.entry().as_option() {
                    Some(entry) => entry,
                    None => {
                        return Ok(ValidateCallbackResult::Invalid(
                            "Original record for a delete must contain an entry".to_string(),
                        ));
                    }
                };
                let original_app_entry = match EntryTypes::deserialize_from_type(
                    app_entry_type.zome_index,
                    app_entry_type.entry_index,
                    entry,
                )? {
                    Some(app_entry) => app_entry,
                    None => {
                        return Ok(ValidateCallbackResult::Invalid(
                            "Original app entry must be one of the defined entry types for this zome"
                                .to_string(),
                        ));
                    }
                };
                match original_app_entry {}
            }
        };
    }

    // Add new entry type to match arm
    if let Some(match_expr) = find_ending_match_expr(&mut arm.body) {
        let new_arm = syn::parse_str(&format!(
            r#"EntryTypes::{pascal_entry_def_name}(original_{snake_entry_def_name}) => {{
                validate_delete_{snake_entry_def_name}(
                    delete_entry.clone().action, 
                    original_action, 
                    original_{snake_entry_def_name}
                )
            }}"#,
        ))?;
        match_expr.arms.insert(0, new_arm);
    }

    Ok(())
}

pub fn find_ending_match_expr_in_block(block: &mut syn::Block) -> Option<&mut syn::ExprMatch> {
    if let Some(e) = block.stmts.last_mut() {
        match e {
            syn::Stmt::Expr(syn::Expr::Match(e_m), _) => Some(e_m),
            _ => None,
        }
    } else {
        None
    }
}

pub fn find_ending_match_expr(e: &mut syn::Expr) -> Option<&mut syn::ExprMatch> {
    match e {
        syn::Expr::Match(expr_match) => Some(expr_match),
        syn::Expr::Block(expr_block) => find_ending_match_expr_in_block(&mut expr_block.block),
        _ => None,
    }
}

fn is_entry(pat: &syn::Pat, entry: &str) -> bool {
    if let syn::Pat::Struct(pat_struct) = pat {
        if let Some(ps) = pat_struct.path.segments.last() {
            return ps.ident == entry;
        }
    }
    false
}
