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
pub fn genesis_self_check(data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {{
    Ok(ValidateCallbackResult::Valid)
}}

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {{
    match op.to_type::<(), ()>()? {{
        OpType::StoreRecord(store_record) => Ok(ValidateCallbackResult::Valid),
        OpType::StoreEntry(store_entry) => match store_entry {{
            OpEntry::CreateEntry {{
                entry_hash,
                entry_type,
            }} | OpEntry::UpdateEntry {{
                entry_hash,
                entry_type,
                ..
            }} => Ok(ValidateCallbackResult::Invalid(
                "There are no entry types in this integrity zome yet".to_string(),
            )),

            OpEntry::CreateAgent(_) | OpEntry::UpdateAgent {{ .. }} => {{
                Ok(ValidateCallbackResult::Valid)
            }}
        }},

        OpType::RegisterUpdate(update_entry) => match update_entry {{
            OpUpdate::Entry {{
                entry_hash,
                original_action_hash,
                original_entry_hash,
                original_entry_type,
                new_entry_type,
            }} => Ok(ValidateCallbackResult::Invalid(
                "There are no entry types in this integrity zome yet".to_string(),
            )),
            _ => Ok(ValidateCallbackResult::Valid)
        }},
        OpType::RegisterDelete(delete_entry) => match delete_entry {{
            OpDelete::Entry {{
                original_action_hash,
                original_entry_hash,
                original_entry_type,
            }} => Ok(ValidateCallbackResult::Invalid(
                "There are no entry types in this integrity zome yet".to_string(),
            )),
            _ => Ok(ValidateCallbackResult::Valid),
        }},
        OpType::RegisterCreateLink {{
            link_type,
            base_address,
            target_address,
            tag,
        }} => Ok(ValidateCallbackResult::Invalid(String::from(
            "There are no link types in this integrity zome yet",
        ))),

        OpType::RegisterDeleteLink {{
            link_type,
            original_link_hash,
            base_address,
            target_address,
            tag,
        }} => Ok(ValidateCallbackResult::Invalid(String::from(
            "There are no link types in this integrity zome yet",
        ))),
        OpType::RegisterAgentActivity(_agent_activity) => Ok(ValidateCallbackResult::Valid),
    }}
}}
"#
    )
}
