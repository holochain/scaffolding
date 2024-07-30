use std::{collections::BTreeMap, path::PathBuf};

use dialoguer::{theme::ColorfulTheme, Select};
use holochain::prelude::AppManifest;
use mr_bundle::Manifest;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{find_files_by_name, FileTree},
};

pub mod cargo;
pub mod git;
pub mod manifests;
pub mod nix;
pub mod utils;

pub struct AppFileTree {
    file_tree: FileTree,
    pub app_manifest_path: PathBuf,
    pub app_manifest: AppManifest,
}

impl AppFileTree {
    pub fn file_tree(self) -> FileTree {
        self.file_tree
    }

    pub fn file_tree_ref(&self) -> &FileTree {
        &self.file_tree
    }

    pub fn get_or_choose(
        file_tree: FileTree,
        app_name: Option<&str>,
    ) -> ScaffoldResult<AppFileTree> {
        let app_manifests = find_app_manifests(&file_tree)?;

        let (app_manifest_path, app_manifest) = match (app_manifests.len(), app_name) {
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
        }?;

        Ok(AppFileTree {
            file_tree,
            app_manifest_path,
            app_manifest,
        })
    }
}

pub fn choose_app(
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

/// Returns the path to the existing app manifests in the given project structure
pub fn find_app_manifests(
    app_file_tree: &FileTree,
) -> ScaffoldResult<BTreeMap<PathBuf, AppManifest>> {
    let files = find_files_by_name(app_file_tree, &AppManifest::path());

    let manifests: BTreeMap<PathBuf, AppManifest> = files
        .into_iter()
        .map(|(key, manifest_str)| {
            let manifest: AppManifest = serde_yml::from_str(manifest_str.as_str())?;
            Ok((key, manifest))
        })
        .collect::<serde_yml::Result<Vec<(PathBuf, AppManifest)>>>()?
        .into_iter()
        .collect();

    Ok(manifests)
}
