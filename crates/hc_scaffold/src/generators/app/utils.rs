use dialoguer::{theme::ColorfulTheme, Select};
use holochain_scaffolding_utils::{find_files_by_name, FileTree};
use holochain_types::prelude::AppManifest;
use mr_bundle::{Location, Manifest};
use path_clean::PathClean;
use std::{collections::BTreeMap, path::PathBuf};

use crate::error::{ScaffoldError, ScaffoldResult};

/// Returns the path to the app manifest in the given project structure
pub fn find_happ_manifests(
    app_file_tree: &FileTree,
) -> ScaffoldResult<BTreeMap<PathBuf, AppManifest>> {
    let files = find_files_by_name(app_file_tree, &AppManifest::path());

    let manifests: BTreeMap<PathBuf, AppManifest> = files
        .into_iter()
        .map(|(key, manifest_str)| {
            let manifest: AppManifest = serde_yaml::from_str(manifest_str.as_str())?;
            Ok((key, manifest))
        })
        .collect::<serde_yaml::Result<Vec<(PathBuf, AppManifest)>>>()?
        .into_iter()
        .collect();

    Ok(manifests)
}

fn choose_app(
    app_manifests: BTreeMap<PathBuf, AppManifest>,
) -> ScaffoldResult<(PathBuf, AppManifest)> {
    let manifest_vec: Vec<(PathBuf, AppManifest)> = app_manifests.into_iter().collect();
    let app_names: Vec<String> = manifest_vec
        .iter()
        .map(|(_, m)| m.app_name().to_string())
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Multiple apps were found in this repository, choose one:")
        .default(0)
        .items(&app_names[..])
        .interact()?;

    Ok(manifest_vec[selection].clone())
}

pub fn get_or_choose_app_manifest(
    app_file_tree: &FileTree,
    app_name: &Option<String>,
) -> ScaffoldResult<(PathBuf, AppManifest)> {
    let app_manifests = find_happ_manifests(&app_file_tree)?;

    match (app_manifests.len(), app_name) {
        (0, _) => Err(ScaffoldError::AppManifestNotFound),
        (1, None) => app_manifests
            .into_iter()
            .last()
            .ok_or(ScaffoldError::AppManifestNotFound),
        (_, None) => choose_app(app_manifests),
        (_, Some(name)) => app_manifests
            .into_iter()
            .find(|(_, m)| m.app_name().to_string().eq(name))
            .ok_or_else(|| ScaffoldError::AppManifestNotFound),
    }
}

pub fn bundled_dnas_locations(
    app_manifest_location: &PathBuf,
    app_manifest: &AppManifest,
) -> Vec<PathBuf> {
    let mut dna_locations: Vec<PathBuf> = vec![];

    let mut app_workdir_location = app_manifest_location.clone();
    app_workdir_location.pop();

    for app_role in app_manifest.app_roles() {
        if let Some(Location::Bundled(mut bundled_location)) = app_role.dna.location {
            bundled_location.pop();
            bundled_location = PathBuf::new()
                .join(&app_workdir_location)
                .join(bundled_location);

            dna_locations.push(bundled_location.clean());
        }
    }

    dna_locations
}
