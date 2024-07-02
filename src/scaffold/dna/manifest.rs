use anyhow::Context;
use holochain_types::prelude::{
    CoordinatorManifest, DnaManifest, DnaManifestCurrentBuilder, HumanTimestamp, IntegrityManifest,
    Timestamp, ZomeManifest,
};

use crate::error::{ScaffoldError, ScaffoldResult};

pub fn empty_dna_manifest(dna_name: &str) -> ScaffoldResult<String> {
    let manifest: DnaManifest = DnaManifestCurrentBuilder::default()
        .name(dna_name.to_owned())
        .integrity(IntegrityManifest {
            network_seed: None,
            origin_time: HumanTimestamp::Micros(Timestamp::now()),
            properties: None,
            zomes: vec![],
        })
        .coordinator(CoordinatorManifest { zomes: vec![] })
        .lineage(vec![])
        .build()
        .context("Failed to build DnaManifest")?
        .into();

    let s = serde_yml::to_string(&manifest)?;
    Ok(s)
}

pub fn check_zome_doesnt_exist(
    dna_manifest: &DnaManifest,
    zome_manifest: &ZomeManifest,
) -> ScaffoldResult<()> {
    let integrity_manifest = match dna_manifest.clone() {
        DnaManifest::V1(m) => m.integrity,
    };
    let coordinator_manifest = match dna_manifest.clone() {
        DnaManifest::V1(m) => m.coordinator,
    };

    if coordinator_manifest
        .zomes
        .iter()
        .chain(integrity_manifest.zomes.iter())
        .any(|z| z.name.to_string().eq(&zome_manifest.name.0.to_string()))
    {
        return Err(ScaffoldError::ZomeAlreadyExists(
            zome_manifest.name.0.to_string(),
            dna_manifest.name(),
        ));
    }
    Ok(())
}
