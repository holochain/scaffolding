use std::path::PathBuf;

use dialoguer::{theme::ColorfulTheme, Select};
use holochain_scaffolding_utils::FileTree;
use holochain_types::prelude::{DnaManifest, ZomeManifest};
use mr_bundle::Location;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    generators::app::cargo::workspace_package_path,
};

pub fn zome_wasm_location(dna_manifest_path: &PathBuf, zome_name: &String) -> Location {
    let mut zome_wasm_location = PathBuf::new();

    let mut dna_workdir_path = dna_manifest_path.clone();
    dna_workdir_path.pop();

    for _c in dna_workdir_path.components() {
        zome_wasm_location = zome_wasm_location.join("..");
    }
    zome_wasm_location = zome_wasm_location
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("release")
        .join(format!("{}.wasm", zome_name));

    Location::Bundled(zome_wasm_location)
}

pub fn zome_manifest_path(
    app_file_tree: &FileTree,
    zome_manifest: &ZomeManifest,
) -> ScaffoldResult<Option<PathBuf>> {
    match zome_manifest.location.clone() {
        Location::Bundled(bundled_path) => {
            let file_name_os_str = bundled_path.file_name().unwrap();
            let file_name = file_name_os_str
                .to_os_string()
                .to_str()
                .unwrap()
                .to_string();

            let crate_name = file_name.split(".wasm").next().unwrap().to_string();

            workspace_package_path(&app_file_tree, &crate_name)
        }
        _ => Ok(None),
    }
}

fn choose_integrity_zome(
    dna_name: &String,
    integrity_zome_names: &Vec<String>,
) -> ScaffoldResult<String> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "Multiple integrity zomes were found in DNA {}, choose one:",
            dna_name
        ))
        .default(0)
        .items(&integrity_zome_names[..])
        .interact()?;

    Ok(integrity_zome_names[selection].clone())
}

pub fn get_or_choose_integrity_zome(
    dna_manifest: &DnaManifest,
    integrity_zome_name: &Option<String>,
) -> ScaffoldResult<String> {
    let integrity_zomes: Vec<String> = match dna_manifest {
        DnaManifest::V1(v1) => v1
            .integrity
            .zomes
            .clone()
            .into_iter()
            .map(|z| z.name.0.to_string())
            .collect(),
    };

    match (integrity_zomes.len(), integrity_zome_name) {
        (0, None) => Err(ScaffoldError::NoIntegrityZomesFound(dna_manifest.name())),
        (1, None) => integrity_zomes
            .into_iter()
            .last()
            .ok_or(ScaffoldError::NoIntegrityZomesFound(dna_manifest.name())),
        (_, None) => choose_integrity_zome(&dna_manifest.name(), &integrity_zomes),
        (_, Some(name)) => integrity_zomes
            .into_iter()
            .find(|zome_name| zome_name.eq(name))
            .ok_or(ScaffoldError::IntegrityZomeNotFound(
                name.clone(),
                dna_manifest.name(),
            )),
    }
}
