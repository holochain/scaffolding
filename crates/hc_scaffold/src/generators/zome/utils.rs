use std::path::PathBuf;

use mr_bundle::Location;

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
