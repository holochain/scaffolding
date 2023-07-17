use holochain_types::prelude::{DnaManifest, DnaManifestCurrentBuilder, ZomeManifest};

use crate::{error::ScaffoldResult, file_tree::insert_file};

use super::{manifest::check_zome_doesnt_exist, zome_wasm_location, DnaFileTree};

pub fn new_integrity_zome_manifest(
    dna_file_tree: &DnaFileTree,
    name: &String,
) -> ScaffoldResult<ZomeManifest> {
    let location = zome_wasm_location(&dna_file_tree, &name);
    let zome_manifest = ZomeManifest {
        name: name.clone().into(),
        hash: None,
        location,
        dependencies: None,
        dylib: None
    };

    Ok(zome_manifest)
}

pub fn add_integrity_zome_to_manifest(
    dna_file_tree: DnaFileTree,
    zome_manifest: ZomeManifest,
) -> ScaffoldResult<DnaFileTree> {
    check_zome_doesnt_exist(&dna_file_tree.dna_manifest, &zome_manifest)?;

    let (mut integrity_manifest, coordinator_manifest) = match dna_file_tree.dna_manifest.clone() {
        DnaManifest::V1(m) => (m.integrity, m.coordinator),
    };
    integrity_manifest.zomes.push(zome_manifest);

    let new_manifest: DnaManifest = DnaManifestCurrentBuilder::default()
        .coordinator(coordinator_manifest)
        .integrity(integrity_manifest)
        .name(dna_file_tree.dna_manifest.name())
        .build()
        .unwrap()
        .into();

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
