use holochain_types::dna::DnaManifestV0;
use holochain_types::prelude::{DnaManifest, ZomeManifest};

use crate::{error::ScaffoldResult, file_tree::insert_file};

use super::{manifest::check_zome_doesnt_exist, zome_wasm_path, DnaFileTree};

pub fn add_integrity_zome_to_manifest(
    dna_file_tree: DnaFileTree,
    zome_manifest: ZomeManifest,
) -> ScaffoldResult<DnaFileTree> {
    check_zome_doesnt_exist(&dna_file_tree.dna_manifest, &zome_manifest)?;

    let (mut integrity_manifest, coordinator_manifest) = match dna_file_tree.dna_manifest.clone() {
        DnaManifest::V0(m) => (m.integrity, m.coordinator),
    };
    integrity_manifest.zomes.push(zome_manifest);

    let new_manifest = DnaManifest::V0(DnaManifestV0 {
        integrity: integrity_manifest,
        coordinator: coordinator_manifest,
        name: dna_file_tree.dna_manifest.name(),
    });

    let dna_manifest_path = dna_file_tree.dna_manifest_path.clone();
    let mut file_tree = dna_file_tree.file_tree();

    insert_file(
        &mut file_tree,
        &dna_manifest_path,
        &serde_yaml::to_string(&new_manifest)?,
    )?;

    let dna_file_tree = DnaFileTree::from_dna_manifest_path(file_tree, &dna_manifest_path)?;

    Ok(dna_file_tree)
}

pub fn new_integrity_zome_manifest(
    dna_file_tree: &DnaFileTree,
    name: &str,
) -> ScaffoldResult<ZomeManifest> {
    let path = zome_wasm_path(dna_file_tree, name);
    let zome_manifest = ZomeManifest {
        name: name.into(),
        hash: None,
        path: path
            .into_os_string()
            .into_string()
            .map_err(|str| anyhow::anyhow!("Invalid zome wasm path: {str:?}"))?,
        dependencies: None,
    };

    Ok(zome_manifest)
}
