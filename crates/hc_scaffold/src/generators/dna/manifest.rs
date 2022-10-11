use holochain_types::prelude::{
    CoordinatorManifest, DnaManifest, DnaManifestCurrentBuilder, HumanTimestamp, IntegrityManifest,
    Timestamp,
};

use crate::error::ScaffoldResult;

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
