use std::{ffi::OsString, path::PathBuf};

use build_fs_tree::{dir, file};
use holochain_scaffolding_utils::FileTree;
use holochain_types::prelude::{
    AppManifest, AppManifestCurrentBuilder, AppRoleDnaManifest, AppRoleManifest, CellProvisioning,
    DnaModifiersOpt,
};
use mr_bundle::Location;

pub mod manifest;
pub mod utils;

use crate::error::{ScaffoldError, ScaffoldResult};

use super::app::utils::get_or_choose_app_manifest;
use manifest::empty_dna_manifest;

pub fn scaffold_dna(
    mut app_file_tree: FileTree,
    app_name: &Option<String>,
    dna_name: &String,
) -> ScaffoldResult<FileTree> {
    let (app_manifest_path, app_manifest) = get_or_choose_app_manifest(&app_file_tree, app_name)?;

    let new_dna_file_tree: FileTree = dir! {
        "zomes" => dir! {},
        "workdir" => dir! {
            "dna.yaml" => file!(empty_dna_manifest(dna_name.clone())?)
        }
    };

    let dnas_path = PathBuf::new().join("dnas");

    let v: Vec<OsString> = dnas_path.iter().map(|s| s.to_os_string()).collect();

    app_file_tree
        .path_mut(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(dnas_path.clone()))?
        .dir_content_mut()
        .ok_or(ScaffoldError::PathNotFound(dnas_path))?
        .insert(OsString::from(dna_name.clone()), new_dna_file_tree);

    let mut dna_location = PathBuf::new();

    let app_workdir_path = app_manifest_path.parent();

    if let Some(path) = app_workdir_path {
        for _path_segment in path.components() {
            dna_location = dna_location.join("..");
        }
    }

    dna_location = dna_location
        .join("dnas")
        .join(dna_name.clone())
        .join("workdir")
        .join(format!("{}.dna", dna_name));

    let mut roles = app_manifest.app_roles();

    if let Some(_) = roles.iter().find(|r| r.id.eq(dna_name)) {
        return Err(ScaffoldError::DnaAlreadyExists(
            dna_name.clone(),
            app_manifest.app_name().to_string(),
        ));
    }

    roles.push(AppRoleManifest {
        id: dna_name.clone(),
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

    let new_manifest: AppManifest = AppManifestCurrentBuilder::default()
        .name(app_manifest.app_name().to_string().clone())
        .description(None)
        .roles(roles)
        .build()
        .unwrap()
        .into();

    let v: Vec<OsString> = app_manifest_path.iter().map(|s| s.to_os_string()).collect();

    *app_file_tree
        .path_mut(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(app_manifest_path.clone()))?
        .file_content_mut()
        .unwrap() = serde_yaml::to_string(&new_manifest)?;

    Ok(app_file_tree)
}
