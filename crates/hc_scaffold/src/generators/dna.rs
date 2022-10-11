use std::path::PathBuf;

use anyhow::anyhow;
use build_fs_tree::{dir, file, MergeableFileSystemTree};
use holochain_scaffolding_utils::{insert_tree_in_path, override_file_contents, FileTree};
use holochain_types::prelude::{
    AppManifest, AppManifestCurrentBuilder, AppRoleDnaManifest, AppRoleManifest, CellProvisioning,
    DnaModifiers, DnaModifiersOpt, SerializedBytes, Timestamp, UnsafeBytes,
};
use mr_bundle::Location;

pub mod manifest;

use super::app::utils::find_happ_manifests;
use manifest::empty_dna_manifest;

pub fn scaffold_dna(mut app_file_tree: FileTree, dna_name: String) -> anyhow::Result<FileTree> {
    let manifest_paths = find_happ_manifests(&app_file_tree);

    if manifest_paths.len() > 1 {
        todo!();
    }
    let (app_manifest_path, contents) = match manifest_paths.into_iter().last() {
        Some(contents) => Ok(contents),
        None => Err(anyhow!(
            "No happ.yaml manifests found in this directory tree."
        )),
    }?;

    let new_dna_file_tree: FileTree = dir! {
        dna_name.clone() => dir!{
          "integrity_zomes" => dir! {},
          "coordinator_zomes"=> dir! {},
          "workdir" => dir! {
            "dna.yaml" => file!(empty_dna_manifest(dna_name.clone())?)
          }
      }
    };

    insert_tree_in_path(
        &mut app_file_tree,
        new_dna_file_tree,
        &PathBuf::new().join("dnas"),
    )
    .map_err(|e| anyhow!(e))?;

    let mut dna_location = PathBuf::new();

    for _path_segment in app_manifest_path.components() {
        dna_location = dna_location.join("..");
    }
    dna_location = dna_location
        .join("dnas")
        .join(dna_name.clone())
        .join("workdir")
        .join(format!("{}.happ", dna_name));

    let app_manifest: AppManifest = serde_yaml::from_str(contents.as_str())?;
    let mut roles = app_manifest.app_roles();

    roles.push(AppRoleManifest {
        id: dna_name,
        dna: AppRoleDnaManifest {
            location: Some(Location::Bundled(dna_location)),
            modifiers: DnaModifiersOpt {
                network_seed: None,
                origin_time: None,
                properties: None,
            },
            version: None,
            clone_limit: 0,
        },
        provisioning: Some(CellProvisioning::Create { deferred: false }),
    });

    let new_manifest = AppManifestCurrentBuilder::default()
        .name(app_manifest.app_name().to_string().clone())
        .roles(roles)
        .build()
        .map_err(|e| anyhow!(e))?;

    override_file_contents(
        &mut app_file_tree,
        &app_manifest_path,
        &serde_yaml::to_string(&new_manifest)?,
    )
    .map_err(|e| anyhow!(e))?;

    Ok(app_file_tree)
}
