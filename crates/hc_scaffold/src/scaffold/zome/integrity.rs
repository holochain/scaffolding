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
tsify = {{ workspace = true }}
wasm-bindgen = {{ workspace = true }}
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
