use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::{
    file_tree::{
        dir_exists, file_content, find_files_by_name, insert_file, insert_file_tree_in_dir,
        FileTree,
    },
    reserved_words::check_for_reserved_words,
    templates::{dna::scaffold_dna_templates, ScaffoldedTemplate},
    utils::choose_directory_path,
};
use build_fs_tree::{dir, file};
use dialoguer::{theme::ColorfulTheme, Select};
use holochain_types::prelude::{
    AppManifest, AppManifestCurrentBuilder, AppRoleDnaManifest, AppRoleManifest, CellProvisioning,
    DnaManifest, DnaModifiersOpt, ValidatedDnaManifest,
};
use mr_bundle::{Location, Manifest};

pub mod coordinator;
pub mod integrity;
pub mod manifest;

use crate::error::{ScaffoldError, ScaffoldResult};

use manifest::empty_dna_manifest;

use super::app::AppFileTree;

#[derive(Clone)]
pub struct DnaFileTree {
    file_tree: FileTree,
    pub dna_manifest_path: PathBuf,
    pub dna_manifest: DnaManifest,
}

impl DnaFileTree {
    pub fn get_or_choose(
        file_tree: FileTree,
        dna_name: &Option<String>,
    ) -> ScaffoldResult<DnaFileTree> {
        let dna_manifests = find_dna_manifests(&file_tree)?;

        let (dna_manifest_path, dna_manifest) = match (dna_manifests.len(), dna_name) {
            (0, None) => Err(ScaffoldError::NoDnasFound),
            (1, None) => dna_manifests
                .into_iter()
                .last()
                .ok_or(ScaffoldError::NoDnasFound),
            (_, None) => choose_dna(dna_manifests.into_iter().collect()),
            (_, Some(name)) => dna_manifests
                .into_iter()
                .find(|(_, m)| m.name().to_string().eq(name))
                .ok_or(ScaffoldError::DnaNotFound(name.clone())),
        }?;

        Ok(DnaFileTree {
            file_tree,
            dna_manifest_path,
            dna_manifest,
        })
    }

    pub fn from_dna_manifest_path(
        file_tree: FileTree,
        dna_manifest_path: &Path,
    ) -> ScaffoldResult<DnaFileTree> {
        let dna_manifest = read_dna_manifest(&file_tree, dna_manifest_path)?;

        Ok(DnaFileTree {
            file_tree,
            dna_manifest_path: dna_manifest_path.to_path_buf(),
            dna_manifest,
        })
    }

    pub fn file_tree(self) -> FileTree {
        self.file_tree
    }

    pub fn file_tree_ref(&self) -> &FileTree {
        &self.file_tree
    }
}

fn default_dnas_dir_path() -> PathBuf {
    PathBuf::new().join("dnas")
}

fn zome_wasm_location(dna_file_tree: &DnaFileTree, zome_name: &str) -> Location {
    let mut zome_wasm_location = PathBuf::new();

    let mut dna_workdir_path = dna_file_tree.dna_manifest_path.clone();
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

/// Returns the path to the existing app manifests in the given project structure
pub fn find_dna_manifests(
    app_file_tree: &FileTree,
) -> ScaffoldResult<BTreeMap<PathBuf, DnaManifest>> {
    let files = find_files_by_name(app_file_tree, &ValidatedDnaManifest::path());

    let manifests: BTreeMap<PathBuf, DnaManifest> = files
        .into_iter()
        .map(|(key, manifest_str)| {
            let manifest: DnaManifest = serde_yaml::from_str(manifest_str.as_str())?;
            Ok((key, manifest))
        })
        .collect::<serde_yaml::Result<Vec<(PathBuf, DnaManifest)>>>()?
        .into_iter()
        .collect();

    Ok(manifests)
}

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

pub fn read_dna_manifest(
    app_file_tree: &FileTree,
    dna_manifest_path: &Path,
) -> ScaffoldResult<DnaManifest> {
    let contents = file_content(app_file_tree, dna_manifest_path)?;
    let manifest: DnaManifest = serde_yaml::from_str(contents.as_str())?;
    Ok(manifest)
}

pub fn get_or_choose_dnas_dir_path(app_file_tree: &FileTree) -> ScaffoldResult<PathBuf> {
    let default_path = default_dnas_dir_path();
    if dir_exists(app_file_tree, &default_path) {
        Ok(default_path.clone())
    } else {
        choose_directory_path(
            &String::from("Which directory should the DNA be scaffolded in?"),
            app_file_tree,
        )
    }
}

pub fn scaffold_dna(
    app_file_tree: AppFileTree,
    template_file_tree: &FileTree,
    dna_name: &str,
) -> ScaffoldResult<ScaffoldedTemplate> {
    check_for_reserved_words(dna_name)?;

    let new_dna_file_tree: FileTree = dir! {
        "zomes" => dir! {
            "coordinator" => dir! {},
            "integrity" => dir! {},
        },
        "workdir" => dir! {
            "dna.yaml" => file!(empty_dna_manifest(dna_name)?)
        }
    };

    let dnas_path = get_or_choose_dnas_dir_path(app_file_tree.file_tree_ref())?;

    let dna_workdir_path = PathBuf::new()
        .join(&dnas_path)
        .join(dna_name)
        .join("workdir");
    let mut dna_workdir_relative_to_app_manifest = PathBuf::new();

    let app_workdir_path = app_file_tree.app_manifest_path.parent();

    if let Some(path) = app_workdir_path {
        for _path_segment in path.components() {
            dna_workdir_relative_to_app_manifest = dna_workdir_relative_to_app_manifest.join("..");
        }
    }

    dna_workdir_relative_to_app_manifest =
        dna_workdir_relative_to_app_manifest.join(dna_workdir_path);

    let dna_bundle_path = dna_workdir_relative_to_app_manifest.join(format!("{}.dna", dna_name));

    let mut roles = app_file_tree.app_manifest.app_roles();

    if roles.iter().any(|r| r.name.eq(dna_name)) {
        return Err(ScaffoldError::DnaAlreadyExists(dna_name.to_owned()));
    }

    roles.push(AppRoleManifest {
        name: dna_name.to_owned(),
        dna: AppRoleDnaManifest {
            location: Some(Location::Bundled(dna_bundle_path)),
            modifiers: DnaModifiersOpt {
                network_seed: None,
                origin_time: None,
                properties: None,
                quantum_time: None,
            },
            installed_hash: None,
            clone_limit: 0,
        },
        provisioning: Some(CellProvisioning::Create { deferred: false }),
    });

    let new_manifest: AppManifest = AppManifestCurrentBuilder::default()
        .name(app_file_tree.app_manifest.app_name().to_string().clone())
        .description(None)
        .roles(roles)
        .build()
        .unwrap()
        .into();

    let app_name = app_file_tree.app_manifest.app_name().to_string();

    let app_manifest_path = app_file_tree.app_manifest_path.clone();

    let mut file_tree = app_file_tree.file_tree();

    insert_file(
        &mut file_tree,
        &app_manifest_path,
        &serde_yaml::to_string(&new_manifest)?,
    )?;

    insert_file_tree_in_dir(
        &mut file_tree,
        &dnas_path,
        (dna_name.into(), new_dna_file_tree),
    )?;

    scaffold_dna_templates(
        file_tree,
        template_file_tree,
        &app_name.to_string(),
        dna_name,
    )
}
