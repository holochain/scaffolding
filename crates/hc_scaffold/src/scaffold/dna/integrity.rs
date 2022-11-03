use holochain_types::prelude::DnaManifest;

use crate::file_tree::insert_file;

use super::manifest::check_zome_doesnt_exist;

pub fn add_integrity_zome_to_manifest(
    dna_file_tree: DnaFileTree,
    name: &String,
) -> ScaffoldResult<DnaFileTree> {
    let location = zome_wasm_location(&dna_file_tree, &name);
    let zome_manifest = ZomeManifest {
        name: name.clone().into(),
        hash: None,
        location,
        dependencies: None,
    };

    check_zome_doesnt_exist(&dna_file_tree.dna_manifest, &zome_manifest)?;

    let (mut integrity_manifest, mut coordinator_manifest) =
        match dna_file_tree.dna_manifest.clone() {
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

    insert_file(
        &mut dna_file_tree.file_tree,
        &dna_file_tree.dna_manifest_path,
        &serde_yaml::to_string(&new_manifest)?,
    )?;

    Ok(dna_file_tree)
}
