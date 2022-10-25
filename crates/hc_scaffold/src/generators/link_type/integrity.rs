use std::{ffi::OsString, path::PathBuf};

use holochain_types::prelude::{AppManifest, DnaManifest};
use quote::quote;

use crate::{
    definitions::EntryDefinition,
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{find_map_rust_files, map_rust_files, FileTree},
    generators::zome::utils::zome_manifest_path,
};

pub fn add_link_type_to_integrity_zome(
    mut app_file_tree: FileTree,
    dna_manifest: &DnaManifest,
    integrity_zome_name: &String,
    link_type_name: &String,
) -> ScaffoldResult<FileTree> {
    let integrity_zome = match dna_manifest {
        DnaManifest::V1(v1) => v1
            .integrity
            .zomes
            .clone()
            .into_iter()
            .find(|z| z.name.0.eq(integrity_zome_name)),
    }
    .ok_or(ScaffoldError::IntegrityZomeNotFound(
        integrity_zome_name.clone(),
        dna_manifest.name(),
    ))?;

    let mut manifest_path = zome_manifest_path(&app_file_tree, &integrity_zome)?.ok_or(
        ScaffoldError::IntegrityZomeNotFound(integrity_zome_name.clone(), dna_manifest.name()),
    )?;

    manifest_path.pop();

    let crate_src_path = manifest_path.join("src");
    let crate_src_path_iter: Vec<OsString> =
        crate_src_path.iter().map(|s| s.to_os_string()).collect();

    let hdk_link_types_instances = find_map_rust_files(
        app_file_tree
            .path(&mut crate_src_path_iter.iter())
            .ok_or(ScaffoldError::PathNotFound(crate_src_path.clone()))?,
        &|path, file| {
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

    // Find the #[hdk_link_types] macro and add the new link type to it
    map_rust_files(
        app_file_tree
            .path_mut(&mut crate_src_path_iter.iter())
            .ok_or(ScaffoldError::PathNotFound(crate_src_path.clone()))?,
        |file_path, mut file| {
            // If there are no link types in this zome, first add the empty enum
            if hdk_link_types_instances.len() == 0 && file_path == PathBuf::from("lib.rs") {
                file.items.push(syn::parse_str::<syn::Item>(
                    "#[hdk_link_types]
                      pub enum LinkTypes {}
                        ",
                )?);
            }

            file.items =
                file.items
                    .into_iter()
                    .map(|i| {
                        if let syn::Item::Enum(mut item_enum) = i.clone() {
                            if item_enum.attrs.iter().any(|a| {
                                a.path.segments.iter().any(|s| s.ident.eq("hdk_link_types"))
                            }) {
                                if item_enum
                                    .variants
                                    .iter()
                                    .any(|v| v.ident.to_string().eq(link_type_name))
                                {
                                    return Err(ScaffoldError::LinkTypeAlreadyExists(
                                        link_type_name.clone(),
                                        dna_manifest.name(),
                                        integrity_zome_name.clone(),
                                    ));
                                }

                                let new_variant = syn::parse_str::<syn::Variant>(
                                    format!("{}", link_type_name).as_str(),
                                )
                                .unwrap();
                                item_enum.variants.push(new_variant);
                                return Ok(syn::Item::Enum(item_enum));
                            }
                        }

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

    Ok(app_file_tree)
}
