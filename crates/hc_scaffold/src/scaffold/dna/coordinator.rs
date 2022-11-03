use holochain_types::prelude::{
    DnaManifest, DnaManifestCurrentBuilder, ZomeDependency, ZomeManifest,
};

use crate::error::{ScaffoldError, ScaffoldResult};

use super::{manifest::check_zome_doesnt_exist, DnaFileTree};

pub fn add_coordinator_zome_to_manifest(
    dna_file_tree: DnaFileTree,
    name: &String,
    maybe_dependencies: &Option<Vec<String>>,
) -> ScaffoldResult<(DnaFileTree, ZomeManifest)> {
    let location = zome_wasm_location(&dna_file_tree, &name);
    let zome_manifest = ZomeManifest {
        name: name.clone().into(),
        hash: None,
        location,
        dependencies: maybe_dependencies.map(|dz| {
            dz.into_iter()
                .map(|d| ZomeDependency { name: d.into() })
                .collect()
        }),
    };
    check_zome_doesnt_exist(&dna_file_tree.dna_manifest, &zome_manifest)?;

    let (mut integrity_manifest, mut coordinator_manifest) =
        match dna_file_tree.dna_manifest.clone() {
            DnaManifest::V1(m) => (m.integrity, m.coordinator),
        };
    if let Some(dependencies) = maybe_dependencies {
        for d in dependencies {
            if !integrity_manifest
                .zomes
                .iter()
                .any(|z| z.name.0.to_string().eq(d))
            {
                return Err(ScaffoldError::IntegrityZomeNotFound(
                    d.clone(),
                    self.dna_manifest.name(),
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

    Ok((dna_file_tree, zome_manifest))
}
