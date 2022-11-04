use holochain_types::prelude::{
    CoordinatorManifest, DnaManifest, DnaManifestCurrentBuilder, HumanTimestamp, IntegrityManifest,
    Timestamp, ZomeManifest,
};

use crate::error::{ScaffoldError, ScaffoldResult};

use super::DnaFileTree;

pub fn empty_dna_manifest(dna_name: String) -> ScaffoldResult<String> {
    let manifest: DnaManifest = DnaManifestCurrentBuilder::default()
        .name(dna_name.clone())
        .integrity(IntegrityManifest {
            network_seed: None,
            origin_time: HumanTimestamp::Micros(Timestamp::now()),
            properties: None,
            zomes: vec![],
        })
        .coordinator(CoordinatorManifest { zomes: vec![] })
        .build()
        .unwrap()
        .into();

    let s = serde_yaml::to_string(&manifest)?;
    Ok(s)
}

pub fn check_zome_doesnt_exist(
    dna_manifest: &DnaManifest,
    zome_manifest: &ZomeManifest,
) -> ScaffoldResult<()> {
    let integrity_manifest = match dna_manifest.clone() {
        DnaManifest::V1(m) => m.integrity,
    };
    let mut coordinator_manifest = match dna_manifest.clone() {
        DnaManifest::V1(m) => m.coordinator,
    };

    if let Some(_) = coordinator_manifest
        .zomes
        .iter()
        .find(|z| z.name.to_string().eq(&zome_manifest.name.0.to_string()))
    {
        return Err(ScaffoldError::ZomeAlreadyExists(
            zome_manifest.name.0.to_string(),
            dna_manifest.name(),
        ));
    }
    Ok(())
}
