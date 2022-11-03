use std::{ffi::OsString, path::PathBuf};

use crate::{
    file_tree::{dna_file_tree::DnaFileTree, insert_file, FileTree},
    scaffold::dna::manifest::check_zome_doesnt_exist,
};
use holochain_types::prelude::{DnaManifest, DnaManifestCurrentBuilder, ZomeManifest};

use crate::error::{ScaffoldError, ScaffoldResult};

use super::utils::zome_wasm_location;

pub fn initial_cargo_toml(zome_name: &String) -> String {
    format!(
        r#"[package]
name = "{}"
version = "0.0.1"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]
name = "{}"

[dependencies]
hdi = {{ workspace = true }}
serde = {{ workspace = true }}
"#,
        zome_name, zome_name,
    )
}

pub fn initial_lib_rs() -> String {
    format!(
        r#"use hdi::prelude::*;

#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {{
  Ok(ValidateCallbackResult::Valid)
}}
"#
    )
}
