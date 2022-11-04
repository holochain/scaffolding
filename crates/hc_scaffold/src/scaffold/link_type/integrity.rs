use std::{ffi::OsString, path::PathBuf};

use convert_case::{Case, Casing};
use holochain_types::prelude::DnaManifest;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{find_map_rust_files, map_rust_files, path, path_mut, FileTree},
    scaffold::{dna::DnaFileTree, zome::ZomeFileTree},
};

pub fn add_link_type_to_integrity_zome(
    mut zome_file_tree: ZomeFileTree,
    link_type_name: &String,
) -> ScaffoldResult<ZomeFileTree> {
    let crate_src_path = zome_file_tree.zome_crate_path.join("src");

    let hdk_link_types_instances = find_map_rust_files(
        path(
            zome_file_tree.dna_file_tree.file_tree_ref(),
            &crate_src_path,
        )?,
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

    // Find the #[hdk_link_types] macro and add the new link type to it
    map_rust_files(
        path_mut(&mut file_tree, &crate_src_path)?,
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

    let dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;
    let zome_file_tree = ZomeFileTree::from_zome_manifest(dna_file_tree, zome_manifest)?;

    Ok(zome_file_tree)
}
