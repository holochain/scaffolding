use dialoguer::{theme::ColorfulTheme, Select};
use holochain_scaffolding_utils::FileTree;
use holochain_types::prelude::{AppManifest, DnaManifest, ValidatedDnaManifest};
use mr_bundle::Manifest;
use std::{ffi::OsString, path::PathBuf};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    generators::app::utils::bundled_dnas_locations,
};

fn choose_dna(
    dna_manifests: Vec<(PathBuf, DnaManifest)>,
) -> ScaffoldResult<(PathBuf, DnaManifest)> {
    let dna_names: Vec<String> = dna_manifests
        .iter()
        .map(|(_, m)| m.name().to_string())
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Multiple DNAs were found in this repository, choose one:")
        .default(0)
        .items(&dna_names[..])
        .interact()?;

    Ok(dna_manifests[selection].clone())
}

fn read_dna_manifest(
    app_file_tree: &FileTree,
    dna_manifest_location: PathBuf,
) -> ScaffoldResult<DnaManifest> {
    let v: Vec<OsString> = dna_manifest_location
        .iter()
        .map(|s| s.to_os_string())
        .collect();

    let contents = app_file_tree
        .path(&mut v.iter())
        .ok_or(ScaffoldError::PathNotFound(dna_manifest_location.clone()))?
        .file_content()
        .ok_or(ScaffoldError::PathNotFound(dna_manifest_location.clone()))?
        .clone();

    let manifest: DnaManifest = serde_yaml::from_str(contents.as_str())?;

    Ok(manifest)
}

pub fn get_or_choose_dna_manifest(
    app_file_tree: &FileTree,
    app_manifest: &(PathBuf, AppManifest),
    dna_name: Option<String>,
) -> ScaffoldResult<(PathBuf, DnaManifest)> {
    let dna_locations = bundled_dnas_locations(&app_manifest.0, &app_manifest.1);

    let dna_manifests = dna_locations
        .into_iter()
        .map(|dna_location| {
            let manifest = read_dna_manifest(
                app_file_tree,
                dna_location.join(ValidatedDnaManifest::path()),
            )?;

            Ok((dna_location, manifest))
        })
        .collect::<ScaffoldResult<Vec<(PathBuf, DnaManifest)>>>()?;

    match (dna_manifests.len(), dna_name) {
        (0, None) => Err(ScaffoldError::DnaManifestNotFound),
        (1, None) => dna_manifests
            .into_iter()
            .last()
            .ok_or(ScaffoldError::DnaManifestNotFound),
        (_, None) => choose_dna(dna_manifests),
        (_, Some(name)) => dna_manifests
            .into_iter()
            .find(|(_, m)| m.name().to_string().eq(&name))
            .ok_or(ScaffoldError::AppManifestNotFound),
    }
}
