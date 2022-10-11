use std::{path::PathBuf, collections::BTreeMap};
use holochain_types::prelude::AppManifest;
use holochain_scaffolding_utils::{find_files_by_name, FileTree};
use mr_bundle::Manifest;

/// Returns the path to the app manifest in the given project structure
pub fn find_happ_manifests(app_file_tree: &FileTree) -> BTreeMap<PathBuf, String> {
  find_files_by_name(app_file_tree, &AppManifest::path())
}