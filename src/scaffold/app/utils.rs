use crate::{
    file_tree::{file_content, FileTree},
    scaffold::dna::read_dna_manifest,
};
use holochain_types::prelude::AppManifest;
use mr_bundle::Location;
use path_clean::PathClean;
use std::path::{Path, PathBuf};
use std::collections::BTreeMap;

use crate::error::{ScaffoldError, ScaffoldResult};

use super::{choose_app, find_app_manifests};

pub fn read_app_manifest(
    app_file_tree: &FileTree,
    app_manifest_path: &Path,
) -> ScaffoldResult<AppManifest> {
    let content = file_content(app_file_tree, app_manifest_path)?;
    let manifest: AppManifest = serde_yaml::from_str(content.as_str())?;
    Ok(manifest)
}

pub fn get_or_choose_app_manifest_path_for_dna_manifest(
    app_file_tree: &FileTree,
    dna_manifest_path: &Path,
) -> ScaffoldResult<PathBuf> {
    let dna_manifest = read_dna_manifest(app_file_tree, dna_manifest_path)?;

    let app_manifests = find_app_manifests(app_file_tree)?;

    let apps_for_dna: BTreeMap<PathBuf, AppManifest> = app_manifests
        .clone()
        .into_iter()
        .filter(|(app_manifest_path, _manifest)| {
            match bundled_dnas_paths(app_file_tree, app_manifest_path) {
                Ok(paths) => paths.contains(&dna_manifest_path.to_path_buf()),
                _ => false,
            }
        })
        .collect();

    let (path, _manifest) = match apps_for_dna.len() {
        0 => Err(ScaffoldError::NoAppsFoundForDna(dna_manifest.name())),
        1 => app_manifests
            .into_iter()
            .last()
            .ok_or(ScaffoldError::NoAppsFoundForDna(dna_manifest.name())),
        _ => choose_app(apps_for_dna),
    }?;

    Ok(path)
}

pub fn bundled_dnas_paths(
    app_file_tree: &FileTree,
    app_manifest_path: &Path,
) -> ScaffoldResult<Vec<PathBuf>> {
    let app_manifest = read_app_manifest(app_file_tree, app_manifest_path)?;

    let mut dna_paths: Vec<PathBuf> = vec![];

    let mut app_workdir_location = app_manifest_path.to_path_buf();
    app_workdir_location.pop();

    for app_role in app_manifest.app_roles() {
        if let Some(Location::Bundled(mut bundled_location)) = app_role.dna.location {
            bundled_location.pop();
            bundled_location = PathBuf::new()
                .join(&app_workdir_location)
                .join(bundled_location);

            dna_paths.push(bundled_location.clean());
        }
    }

    Ok(dna_paths)
}
