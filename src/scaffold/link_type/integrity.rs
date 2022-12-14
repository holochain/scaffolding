use std::{ffi::OsString, path::PathBuf};

use convert_case::{Case, Casing};
use holochain::test_utils::itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{find_map_rust_files, map_rust_files},
    scaffold::{
        dna::DnaFileTree, entry_type::integrity::find_ending_match_expr, zome::ZomeFileTree,
    },
};

pub fn add_link_type_to_integrity_zome(
    zome_file_tree: ZomeFileTree,
    link_type_name: &String,
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
                    "#[hdk_link_types]
                      pub enum LinkTypes {}
                        ",
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
                        // TODO: add the appropriate validation rules
                        Ok(ValidateCallbackResult::Valid)
                    },
                    false => quote! {
                        Ok(ValidateCallbackResult::Invalid(String::from(#deleted_invalid_reason)))
                    },
                };

                let create_token_stream = quote! {
                    pub fn #validate_create_fn(
                        base: AnyLinkableHash,
                        target: AnyLinkableHash
                    ) -> ExternResult<ValidateCallbackResult> {
                        // TODO: add the appropriate validation rules
                        Ok(ValidateCallbackResult::Valid)
                  }
                };

                let delete_token_stream = quote! {
                    pub fn #validate_delete_fn(
                        base: AnyLinkableHash,
                        target: AnyLinkableHash
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

    let dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;
    let zome_file_tree = ZomeFileTree::from_zome_manifest(dna_file_tree, zome_manifest)?;

    Ok(zome_file_tree)
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
                                                        format!("LinkTypes::{} => validate_create_link_{}(base_address, target_address),", 
                                                            link_type_name.to_case(Case::Pascal),
                                                            link_type_name.to_case(Case::Snake)
                                                        ).as_str()
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
                                                                            format!("LinkTypes::{} => validate_delete_link_{}(base_address, target_address),", 
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
