use std::{ffi::OsString, path::PathBuf};

use convert_case::{Case, Casing};
use holochain::test_utils::itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{find_map_rust_files, map_rust_files, FileTree},
    scaffold::{
        dna::DnaFileTree,
        entry_type::{
            definitions::Referenceable,
            integrity::{find_ending_match_expr, find_ending_match_expr_in_block},
        },
        zome::{utils::get_coordinator_zomes_for_integrity, ZomeFileTree},
    },
};

fn validate_referenceable(
    referenceable: &Referenceable,
    address_ident: &syn::Ident,
) -> TokenStream {
    match referenceable {
        Referenceable::EntryType(entry_type) => {
            let entry_type_snake = format_ident!("_{}", entry_type.entry_type.to_case(Case::Snake));
            let entry_type_pascal =
                format_ident!("{}", entry_type.entry_type.to_case(Case::Pascal));
            match entry_type.reference_entry_hash {
                true => quote! {
                    /// Check the entry type for the given entry hash
                    let entry_hash = EntryHash::from(#address_ident);
                    let entry = must_get_entry(entry_hash)?.content;

                    let #entry_type_snake = crate::#entry_type_pascal::try_from(entry)?;
                },
                false => quote! {
                    /// Check the entry type for the given action hash
                    let action_hash = ActionHash::from(#address_ident);
                    let record = must_get_valid_record(action_hash)?;

                    let #entry_type_snake: crate::#entry_type_pascal = record.entry().to_app_option()
                      .map_err(|e| wasm_error!(e))?.ok_or(wasm_error!(WasmErrorInner::Guest(String::from("Linked action must reference an entry"))))?;
                },
            }
        }
        _ => quote! {},
    }
}

pub fn add_link_type_to_integrity_zome(
    zome_file_tree: ZomeFileTree,
    link_type_name: &String,
    from_referenceable: &Option<Referenceable>,
    to_referenceable: &Option<Referenceable>,
    delete: bool,
    file_to_add_validation_to: &PathBuf,
) -> ScaffoldResult<ZomeFileTree> {
    let crate_src_path = zome_file_tree.zome_crate_path.join("src");

    let v: Vec<OsString> = crate_src_path
        .clone()
        .iter()
        .map(|s| s.to_os_string())
        .collect();
    let hdk_link_types_instances = find_map_rust_files(
        zome_file_tree
            .dna_file_tree
            .file_tree_ref()
            .path(&mut v.iter())
            .ok_or(ScaffoldError::PathNotFound(crate_src_path.clone()))?,
        &|_path, file| {
            file.items.clone().into_iter().find(|i| {
                if let syn::Item::Enum(item_enum) = i.clone() {
                    if item_enum
                        .attrs
                        .iter()
                        .any(|a| a.path.segments.iter().any(|s| s.ident.eq("hdk_link_types")))
                    {
                        return true;
                    }
                }
                false
            })
        },
    );

    let pascal_case_link_type_name = link_type_name.to_case(Case::Pascal);

    let dna_manifest = zome_file_tree.dna_file_tree.dna_manifest.clone();
    let dna_manifest_path = zome_file_tree.dna_file_tree.dna_manifest_path.clone();
    let zome_manifest = zome_file_tree.zome_manifest.clone();

    let mut file_tree = zome_file_tree.dna_file_tree.file_tree();

    let v: Vec<OsString> = crate_src_path
        .clone()
        .iter()
        .map(|s| s.to_os_string())
        .collect();

    // Find the #[hdk_link_types] macro and add the new link type to it
    map_rust_files(
        file_tree
            .path_mut(&mut v.iter())
            .ok_or(ScaffoldError::PathNotFound(crate_src_path.clone()))?,
        |file_path, mut file| {
            // If there are no link types in this zome, first add the empty enum
            if hdk_link_types_instances.len() == 0 && file_path == PathBuf::from("lib.rs") {
                let link_types_item = syn::parse_str::<syn::Item>(
                    "#[derive(Serialize, Deserialize)]
                     #[hdk_link_types]
                     pub enum LinkTypes {}",
                )?;

                // Insert the link types just before the first function
                match file.items.iter().find_position(|i| {
                    if let syn::Item::Fn(_) = i {
                        true
                    } else {
                        false
                    }
                }) {
                    Some((i, _)) => {
                        file.items.insert(i, link_types_item);
                    }
                    None => file.items.push(link_types_item),
                }

                for item in &mut file.items {
                    if let syn::Item::Fn(item_fn) = item {
                        if item_fn.sig.ident.to_string().eq(&String::from("validate")) {
                            for stmt in &mut item_fn.block.stmts {
                                if let syn::Stmt::Expr(syn::Expr::Match(match_expr)) = stmt {
                                    if let syn::Expr::Try(try_expr) = &mut *match_expr.expr {
                                        if let syn::Expr::MethodCall(call) = &mut *try_expr.expr {
                                            if call.method.to_string().eq(&String::from("to_type"))
                                            {
                                                if let Some(turbofish) = &mut call.turbofish {
                                                    if let Some(last_arg) =
                                                        turbofish.args.last_mut()
                                                    {
                                                        *last_arg =
                                                            syn::GenericMethodArgument::Type(
                                                                syn::parse_str::<syn::Type>(
                                                                    "LinkTypes",
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

            file.items =
                file.items
                    .into_iter()
                    .map(|mut i| {
                        if let syn::Item::Enum(mut item_enum) = i.clone() {
                            if item_enum.attrs.iter().any(|a| {
                                a.path.segments.iter().any(|s| s.ident.eq("hdk_link_types"))
                            }) {
                                if item_enum
                                    .variants
                                    .iter()
                                    .any(|v| v.ident.to_string().eq(&pascal_case_link_type_name))
                                {
                                    return Err(ScaffoldError::LinkTypeAlreadyExists(
                                        link_type_name.clone(),
                                        dna_manifest.name(),
                                        zome_manifest.name.0.to_string(),
                                    ));
                                }

                                let new_variant = syn::parse_str::<syn::Variant>(
                                    format!("{}", pascal_case_link_type_name).as_str(),
                                )?;
                                item_enum.variants.push(new_variant);
                                return Ok(syn::Item::Enum(item_enum));
                            }
                        }

                        add_link_type_to_validation_arms(&mut i, &link_type_name)?;

                        Ok(i)
                    })
                    .collect::<ScaffoldResult<Vec<syn::Item>>>()?;

            Ok(file)
        },
    )
    .map_err(|e| match e {
        ScaffoldError::MalformedFile(path, error) => {
            ScaffoldError::MalformedFile(crate_src_path.join(&path), error)
        }
        _ => e,
    })?;

    // Add validation function to appropriate file
    map_rust_files(
        file_tree
            .path_mut(&mut v.iter())
            .ok_or(ScaffoldError::PathNotFound(crate_src_path.clone()))?,
        |file_path, mut file| {
            if file_path.eq(file_to_add_validation_to) {
                let validate_create_fn = format_ident!(
                    "validate_create_link_{}",
                    link_type_name.to_case(Case::Snake)
                );

                let validate_delete_fn = format_ident!(
                    "validate_delete_link_{}",
                    link_type_name.to_case(Case::Snake)
                );

                let deleted_invalid_reason = format!(
                    "{} links cannot be deleted",
                    link_type_name.to_case(Case::Pascal)
                );

                let validate_delete_result: TokenStream = match delete {
                    true => quote! {
                        /// TODO: add the appropriate validation rules
                        Ok(ValidateCallbackResult::Valid)
                    },
                    false => quote! {
                        Ok(ValidateCallbackResult::Invalid(String::from(#deleted_invalid_reason)))
                    },
                };

                let base_address_ident = match from_referenceable {
                    Some(Referenceable::EntryType(_)) => format_ident!("base_address"),
                    _ => format_ident!("_base_address"),
                };

                let validate_create_from = match from_referenceable {
                    Some(r) => Some(validate_referenceable(r, &base_address_ident)),
                    _ => None,
                };
                let target_address_ident = match to_referenceable {
                    Some(Referenceable::EntryType(_)) => format_ident!("target_address"),
                    _ => format_ident!("_target_address"),
                };

                let validate_create_to = match to_referenceable {
                    Some(r) => Some(validate_referenceable(r, &target_address_ident)),
                    _ => None,
                };

                let create_token_stream = quote! {
                    pub fn #validate_create_fn(
                        _action: CreateLink,
                        #base_address_ident: AnyLinkableHash,
                        #target_address_ident: AnyLinkableHash,
                        _tag: LinkTag,
                    ) -> ExternResult<ValidateCallbackResult> {
                        #validate_create_from

                        #validate_create_to

                        /// TODO: add the appropriate validation rules
                        Ok(ValidateCallbackResult::Valid)
                  }
                };

                let delete_token_stream = quote! {
                    pub fn #validate_delete_fn(
                        _action: DeleteLink,
                        _original_action: CreateLink,
                        _base: AnyLinkableHash,
                        _target: AnyLinkableHash,
                        _tag: LinkTag
                    ) -> ExternResult<ValidateCallbackResult> {
                        #validate_delete_result
                  }
                };

                let item: syn::Item = syn::parse_str(create_token_stream.to_string().as_str())?;
                file.items.push(item);
                let item: syn::Item = syn::parse_str(delete_token_stream.to_string().as_str())?;
                file.items.push(item);
            }

            Ok(file)
        },
    )
    .map_err(|e| match e {
        ScaffoldError::MalformedFile(path, error) => {
            ScaffoldError::MalformedFile(crate_src_path.join(&path), error)
        }
        _ => e,
    })?;

    let coordinator_zomes_for_integrity = get_coordinator_zomes_for_integrity(
        &dna_manifest,
        &zome_file_tree.zome_manifest.name.0.to_string(),
    );

    for coordinator_zome in coordinator_zomes_for_integrity {
        let dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;
        let zome_file_tree =
            ZomeFileTree::from_zome_manifest(dna_file_tree, coordinator_zome.clone())?;
        file_tree = add_link_type_signals(
            zome_file_tree.dna_file_tree.file_tree(),
            &zome_file_tree.zome_crate_path,
        )?;
    }
    let dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;
    let zome_file_tree = ZomeFileTree::from_zome_manifest(dna_file_tree, zome_manifest)?;

    Ok(zome_file_tree)
}

fn add_link_type_signals(
    mut file_tree: FileTree,
    zome_crate_path: &PathBuf,
) -> ScaffoldResult<FileTree> {
    let crate_src_path = zome_crate_path.join("src");
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
                for item in &mut file.items {
                    if let syn::Item::Enum(item_enum) = item {
                        if item_enum.ident.to_string().eq(&String::from("Signal")) {
                            if !signal_has_link_types(item_enum) {
                                for v in signal_link_types_variants()? {
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
                                if !signal_action_has_link_types(expr_match) {
                                    for arm in signal_action_match_arms()? {
                                        expr_match.arms.insert(expr_match.arms.len() - 1, arm);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(file)
        },
    )?;
    Ok(file_tree)
}

fn signal_has_link_types(signal_enum: &syn::ItemEnum) -> bool {
    signal_enum
        .variants
        .iter()
        .find(|v| v.ident.to_string().eq(&String::from("LinkCreated")))
        .is_some()
}

fn signal_action_has_link_types(expr_match: &syn::ExprMatch) -> bool {
    expr_match
        .arms
        .iter()
        .find(|arm| {
            if let syn::Pat::TupleStruct(tuple_struct_pat) = &arm.pat {
                if let Some(first_segment) = tuple_struct_pat.path.segments.last() {
                    if first_segment
                        .ident
                        .to_string()
                        .eq(&String::from("CreateLink"))
                    {
                        return true;
                    }
                }
            }
            false
        })
        .is_some()
}

fn signal_link_types_variants() -> ScaffoldResult<Vec<syn::Variant>> {
    Ok(vec![
        syn::parse_str::<syn::Variant>(
            "LinkCreated {
        action: SignedActionHashed,
        link_type: LinkTypes,
    }",
        )?,
        syn::parse_str::<syn::Variant>(
            "LinkDeleted {
        action: SignedActionHashed,
        link_type: LinkTypes,
    }",
        )?,
    ])
}

fn signal_action_match_arms() -> ScaffoldResult<Vec<syn::Arm>> {
    Ok(vec![
        syn::parse_str::<syn::Arm>("Action::CreateLink(create_link) => {
            let link_type = LinkTypes::from_type(create_link.zome_index, create_link.link_type)?.ok_or(wasm_error!(WasmErrorInner::Guest(\"Link type should be exist\".to_string())))?;
            emit_signal(Signal::LinkCreated {
                action,
                link_type
            })?;
            Ok(())
        }")?,
        syn::parse_str::<syn::Arm>("Action::DeleteLink(delete_link) => {
            let record = get(delete_link.link_add_address.clone(), GetOptions::default())?
                .ok_or(wasm_error!(WasmErrorInner::Guest(\"Create Link should exist\".to_string())))?;
            match record.action() {
                Action::CreateLink(create_link) => {
                    let link_type = LinkTypes::from_type(create_link.zome_index, create_link.link_type)?.ok_or(wasm_error!(WasmErrorInner::Guest(\"Link type should be exist\".to_string())))?;
                    emit_signal(Signal::LinkDeleted {
                        action,
                        link_type
                    })?;
                    Ok(())
                },
                _ => {
                    return Err(wasm_error!(WasmErrorInner::Guest(\"Create Link should exist\".to_string())));
                }
            }
        }")?
    ])
}

fn is_create_link(pat: &syn::Pat) -> bool {
    if let syn::Pat::Struct(pat_struct) = pat {
        if let Some(ps) = pat_struct.path.segments.last() {
            if ps.ident.to_string().eq(&String::from("CreateLink")) {
                return true;
            }
        }
    }
    return false;
}
fn is_delete_link(pat: &syn::Pat) -> bool {
    if let syn::Pat::Struct(pat_struct) = pat {
        if let Some(ps) = pat_struct.path.segments.last() {
            if ps.ident.to_string().eq(&String::from("DeleteLink")) {
                return true;
            }
        }
    }
    return false;
}

fn add_link_type_to_validation_arms(
    item: &mut syn::Item,
    link_type_name: &String,
) -> ScaffoldResult<()> {
    if let syn::Item::Fn(item_fn) = item {
        if item_fn.sig.ident.to_string().eq(&String::from("validate")) {
            for stmt in &mut item_fn.block.stmts {
                if let syn::Stmt::Expr(syn::Expr::Match(match_expr)) = stmt {
                    if let syn::Expr::Try(try_expr) = &mut *match_expr.expr {
                        if let syn::Expr::MethodCall(call) = &mut *try_expr.expr {
                            if call.method.to_string().eq(&String::from("to_type")) {
                                for arm in &mut match_expr.arms {
                                    if let syn::Pat::TupleStruct(tuple_struct) = &mut arm.pat {
                                        if let Some(path_segment) =
                                            tuple_struct.path.segments.last()
                                        {
                                            let path_segment_str = path_segment.ident.to_string();
                                            if path_segment_str.eq(&String::from("StoreRecord")) {
                                                if let Some(op_entry_match_expr) =
                                                    find_ending_match_expr(&mut *arm.body)
                                                {
                                                    for op_record_arm in
                                                        &mut op_entry_match_expr.arms
                                                    {
                                                        if is_create_link(&op_record_arm.pat) {
                                                            // Add new link type to match arm
                                                            if let Some(_) = find_ending_match_expr(
                                                                &mut *op_record_arm.body,
                                                            ) {
                                                            } else {
                                                                // Change empty invalid to match on link_type
                                                                *op_record_arm.body =
                                                                    syn::parse_str::<syn::Expr>(
                                                                        "match link_type {}",
                                                                    )?;
                                                            }

                                                            // Add new link type to match arm
                                                            if let Some(link_type_match) =
                                                                find_ending_match_expr(
                                                                    &mut *op_record_arm.body,
                                                                )
                                                            {
                                                                let new_arm: syn::Arm = syn::parse_str(
                                                                    format!(
    "LinkTypes::{} => validate_create_link_{}(action, base_address, target_address, tag),", 
                                                                        link_type_name.to_case(Case::Pascal),
                                                                        link_type_name.to_case(Case::Snake)
                                                                    ).as_str()
                                                                )?;
                                                                link_type_match.arms.push(new_arm);
                                                            }
                                                        } else if is_delete_link(&op_record_arm.pat)
                                                        {
                                                            // Add new link type to match arm
                                                            if let Some(_) = find_ending_match_expr(
                                                                &mut *op_record_arm.body,
                                                            ) {
                                                            } else {
                                                                // Change empty invalid to match on link_type
                                                                *op_record_arm.body =
                                                                    syn::parse_str::<syn::Expr>(
                                                                        r#"{
        let record = must_get_valid_record(original_action_hash)?;
        let create_link = match record.action() {
            Action::CreateLink(create_link) => create_link.clone(),
            _ => {
                return Ok(ValidateCallbackResult::Invalid("The action that a DeleteLink deletes must be a CreateLink".to_string()));
            }
        };
        let link_type = match LinkTypes::from_type(create_link.zome_index.clone(), create_link.link_type.clone())? {
            Some(lt) => lt,
            None => {
                return Ok(ValidateCallbackResult::Valid);
            }
        };
        match link_type {}
    }"#,
                                                                    )?;
                                                            }

                                                            // Add new entry type to match arm
                                                            if let Some(link_type_match) =
                                                                find_ending_match_expr(
                                                                    &mut *op_record_arm.body,
                                                                )
                                                            {
                                                                let new_arm: syn::Arm =
                                                                    syn::parse_str(
                                                                        format!(
"LinkTypes::{} => validate_delete_link_{}(action, create_link.clone(), base_address, create_link.target_address, create_link.tag),", 
                                                                            link_type_name.to_case(Case::Pascal),
                                                                            link_type_name.to_case(Case::Snake),
                                                                        ).as_str()
                                                                    )?;
                                                                link_type_match.arms.push(new_arm);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    if let syn::Pat::Struct(pat_struct) = &mut arm.pat {
                                        if let Some(path_segment) = pat_struct.path.segments.last()
                                        {
                                            let path_segment_str = path_segment.ident.to_string();

                                            if path_segment_str
                                                .eq(&String::from("RegisterCreateLink"))
                                            {
                                                // Add new link type to match arm
                                                if let Some(_) =
                                                    find_ending_match_expr(&mut *arm.body)
                                                {
                                                } else {
                                                    // Change empty invalid to match on link_type
                                                    *arm.body = syn::parse_str::<syn::Expr>(
                                                        "match link_type {}",
                                                    )?;
                                                }

                                                // Add new link type to match arm
                                                if let Some(link_type_match) =
                                                    find_ending_match_expr(&mut *arm.body)
                                                {
                                                    let new_arm: syn::Arm = syn::parse_str(
                                                        format!(
    "LinkTypes::{} => validate_create_link_{}(action, base_address, target_address, tag),", 
                                                            link_type_name.to_case(Case::Pascal),
                                                            link_type_name.to_case(Case::Snake)
                                                        )
                                                        .as_str(),
                                                    )?;
                                                    link_type_match.arms.push(new_arm);
                                                }
                                            } else if path_segment_str
                                                .eq(&String::from("RegisterDeleteLink"))
                                            {
                                                // Add new link type to match arm
                                                if let Some(_) =
                                                    find_ending_match_expr(&mut *arm.body)
                                                {
                                                } else {
                                                    // Change empty invalid to match on link_type
                                                    *arm.body = syn::parse_str::<syn::Expr>(
                                                        "match link_type {}",
                                                    )?;
                                                }

                                                // Add new link type to match arm
                                                if let Some(link_type_match) =
                                                    find_ending_match_expr(&mut *arm.body)
                                                {
                                                    let new_arm: syn::Arm = syn::parse_str(
                                                        format!(
        "LinkTypes::{} => validate_delete_link_{}(action, original_action, base_address, target_address, tag),", 
                                                            link_type_name.to_case(Case::Pascal),
                                                            link_type_name.to_case(Case::Snake)
                                                        ).as_str()
                                                    )?;
                                                    link_type_match.arms.push(new_arm);
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
    }
    Ok(())
}
