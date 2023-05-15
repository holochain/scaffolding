use holochain_types::prelude::{
    DnaManifest, DnaManifestCurrentBuilder, ZomeDependency, ZomeManifest,
};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::insert_file,
};

use super::{manifest::check_zome_doesnt_exist, zome_wasm_location, DnaFileTree};

pub fn new_coordinator_zome_manifest(
    dna_file_tree: &DnaFileTree,
    name: &String,
    maybe_dependencies: &Option<Vec<String>>,
) -> ScaffoldResult<ZomeManifest> {
    let location = zome_wasm_location(&dna_file_tree, &name);
    let zome_manifest = ZomeManifest {
        name: name.clone().into(),
        hash: None,        
        location,
        dependencies: maybe_dependencies.clone().map(|dz| {
            dz.into_iter()
                .map(|d| ZomeDependency { name: d.into() })
                .collect()
        }),
        dylib: None,
    };

    Ok(zome_manifest)
}

pub fn add_coordinator_zome_to_manifest(
    mut dna_file_tree: DnaFileTree,
    zome_manifest: ZomeManifest,
) -> ScaffoldResult<DnaFileTree> {
    check_zome_doesnt_exist(&dna_file_tree.dna_manifest, &zome_manifest)?;

    let (integrity_manifest, mut coordinator_manifest) = match dna_file_tree.dna_manifest.clone() {
        DnaManifest::V1(m) => (m.integrity, m.coordinator),
    };
    if let Some(dependencies) = zome_manifest.dependencies.clone() {
        for d in dependencies {
            if !integrity_manifest
                .zomes
                .iter()
                .any(|z| z.name.0.to_string().eq(&d.name.0.to_string()))
            {
                return Err(ScaffoldError::IntegrityZomeNotFound(
                    d.name.0.to_string(),
                    dna_file_tree.dna_manifest.name(),
                ));
            }
        }
    }
    coordinator_manifest.zomes.push(zome_manifest);

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
