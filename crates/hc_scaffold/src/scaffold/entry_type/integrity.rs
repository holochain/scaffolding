use build_fs_tree::file;
use convert_case::{Case, Casing};
use holochain_types::prelude::DnaManifest;
use prettyplease::unparse;
use proc_macro2::TokenStream;
use quote::quote;
use std::{ffi::OsString, path::PathBuf};

use crate::error::{ScaffoldError, ScaffoldResult};
use crate::file_tree::insert_file;
use crate::scaffold::dna::DnaFileTree;
use crate::{
    definitions::EntryDefinition,
    file_tree::{find_map_rust_files, map_file, map_rust_files, FileTree},
    scaffold::zome::ZomeFileTree,
};

pub fn render_entry_definition_struct(entry_def: &EntryDefinition) -> ScaffoldResult<TokenStream> {
    let name: syn::Expr = syn::parse_str(entry_def.singular_name.to_case(Case::Pascal).as_str())?;

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

pub fn render_entry_definition_file(entry_def: &EntryDefinition) -> ScaffoldResult<syn::File> {
    let entry_def_token_stream = render_entry_definition_struct(entry_def)?;

    let type_definitions: Vec<TokenStream> = entry_def
        .fields
        .iter()
        .filter_map(|field_def| field_def.field_type.rust_type_definition())
        .collect();

    let token_stream = quote! {
      use hdi::prelude::*;

      #(#type_definitions)*

      #[hdk_entry_helper]
      #[derive(Clone)]
      #entry_def_token_stream
    };

    let file = syn::parse_file(token_stream.to_string().as_str())?;

    Ok(file)
}

pub fn add_entry_type_to_integrity_zome(
    zome_file_tree: ZomeFileTree,
    entry_def: &EntryDefinition,
) -> ScaffoldResult<ZomeFileTree> {
    let dna_manifest_path = zome_file_tree.dna_file_tree.dna_manifest_path.clone();
    let dna_manifest = zome_file_tree.dna_file_tree.dna_manifest.clone();
    let zome_manifest = zome_file_tree.zome_manifest.clone();

    let snake_entry_def_name = entry_def.singular_name.to_case(Case::Snake);
    let entry_def_file = render_entry_definition_file(entry_def)?;

    let entry_types = get_all_entry_types(&zome_file_tree)?;

    // 1. Create an ENTRY_DEF_NAME.rs in "src/", with the entry definition struct
    let crate_src_path = zome_file_tree.zome_crate_path.join("src");

    let entry_def_path = crate_src_path.join(format!("{}.rs", snake_entry_def_name));

    let mut file_tree = zome_file_tree.dna_file_tree.file_tree();

    insert_file(&mut file_tree, &entry_def_path, &unparse(&entry_def_file))?;

    // 2. Add this file as a module in the entry point for the crate

    let lib_rs_path = crate_src_path.join("lib.rs");

    map_file(&mut file_tree, &lib_rs_path, |s| {
        format!(
            r#"pub mod {};
pub use {}::*;

{}"#,
            snake_entry_def_name, snake_entry_def_name, s
        )
    })?;

    let pascal_entry_def_name = entry_def.singular_name.to_case(Case::Pascal);

    let v: Vec<OsString> = crate_src_path
        .clone()
        .iter()
        .map(|s| s.to_os_string())
        .collect();
    // 3. Find the #[hdk_entry_defs] macro
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
                file.items.push(syn::parse_str::<syn::Item>(
                    "#[hdk_entry_defs]
                     #[unit_enum(UnitEntryTypes)]
                      pub enum EntryTypes {}
                        ",
                )?);
            }

            file.items =
                file.items
                    .into_iter()
                    .map(|i| {
                        if let syn::Item::Enum(mut item_enum) = i.clone() {
                            if item_enum.attrs.iter().any(|a| {
                                a.path.segments.iter().any(|s| s.ident.eq("hdk_entry_defs"))
                            }) {
                                if item_enum
                                    .variants
                                    .iter()
                                    .any(|v| v.ident.to_string().eq(&pascal_entry_def_name))
                                {
                                    return Err(ScaffoldError::EntryTypeAlreadyExists(
                                        pascal_entry_def_name.clone(),
                                        dna_manifest.name(),
                                        zome_file_tree.zome_manifest.name.0.to_string(),
                                    ));
                                }

                                found = true;
                                let new_variant = syn::parse_str::<syn::Variant>(
                                    format!("{}({})", pascal_entry_def_name, pascal_entry_def_name)
                                        .as_str(),
                                )
                                .unwrap();
                                item_enum.variants.push(new_variant);
                                return Ok(syn::Item::Enum(item_enum));
                            }
                        }

                        Ok(i)
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
            ScaffoldError::MalformedFile(crate_src_path.join(&path), error)
        }
        _ => e,
    })?;

    let dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;
    let zome_file_tree = ZomeFileTree::from_zome_manifest(dna_file_tree, zome_manifest)?;

    Ok(zome_file_tree)
}

pub fn get_all_entry_types(zome_file_tree: &ZomeFileTree) -> ScaffoldResult<Option<Vec<String>>> {
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
                    if item_enum
                        .attrs
                        .iter()
                        .any(|a| a.path.segments.iter().any(|s| s.ident.eq("hdk_entry_defs")))
                    {
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

            Ok(Some(variants))
        }
        _ => Err(ScaffoldError::MultipleEntryTypesDefsFoundForIntegrityZome(
            zome_file_tree.dna_file_tree.dna_manifest.name(),
            zome_file_tree.zome_manifest.name.0.to_string(),
        )),
    }
}
