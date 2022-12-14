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

/// Validation you perform during the genesis process. Nobody else on the network performs it, only you.
/// There *is no* access to network calls in this callback
#[hdk_extern]
pub fn genesis_self_check(data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {{
    Ok(ValidateCallbackResult::Valid)
}}

/// Validation the network performs when you try to join, you can't perform this validation yourself as you are not a member yet.
/// There *is* access to network calls in this function
pub fn validate_agent_joining(agent_pub_key: AgentPubKey, membrane_proof: &Option<MembraneProof>) -> ExternResult<ValidateCallbackResult> {{
    Ok(ValidateCallbackResult::Valid)
}}

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {{
    match op.to_type::<(), ()>()? {{
        OpType::StoreRecord(store_record) => match store_record {{
            OpRecord::CreateEntry {{
                app_entry,
                action
            }} => Ok(ValidateCallbackResult::Invalid("There are no entry types in this integrity zome".to_string())),
            OpRecord::UpdateEntry {{
                app_entry_type,
                action
            }} => Ok(ValidateCallbackResult::Invalid("There are no entry types in this integrity zome".to_string())),
            OpRecord::DeleteEntry {{
                original_action,
                original_app_entry_type,
                action
            }} => Ok(ValidateCallbackResult::Invalid("There are no entry types in this integrity zome".to_string())),
            OpRecord::CreateLink {{
                base_address,
                target_address,
                tag,
                link_type,
                action
            }} => Ok(ValidateCallbackResult::Invalid("There are no link types in this integrity zome".to_string())),
            OpRecord::DeleteLink {{
                original_action,
                base_address,
                target_address,
                tag,
                link_type,
                action
            }} => Ok(ValidateCallbackResult::Invalid("There are no link types in this integrity zome".to_string())),
            OpRecord::CreatePrivateEntry {{
                app_entry_type,
                action
            }}=> Ok(ValidateCallbackResult::Valid),
            OpRecord::UpdatePrivateEntry {{
                original_action,
                app_entry_type,
                action
            }}=> Ok(ValidateCallbackResult::Valid),
            OpRecord::CreateCapClaim {{
                action
            }} => Ok(ValidateCallbackResult::Valid),
            OpRecord::CreateCapGrant {{
                action
            }} => Ok(ValidateCallbackResult::Valid),
            OpRecord::UpdateCapClaim {{
                original_action,
                action
            }} => Ok(ValidateCallbackResult::Valid),
            OpRecord::UpdateCapGrant {{
                original_action,
                action
            }} => Ok(ValidateCallbackResult::Valid),
            OpRecord::Dna {{
                dna_hash,
                action
            }} => Ok(ValidateCallbackResult::Valid),
            OpRecord::OpenChain {{
                previous_dna_hash,
                action
            }} => Ok(ValidateCallbackResult::Valid),
            OpRecord::CloseChain {{
                new_dna_hash,
                action
            }} => Ok(ValidateCallbackResult::Valid),
            OpRecord::InitZomesComplete {{
                action
            }} => Ok(ValidateCallbackResult::Valid),
            _ => Ok(ValidateCallbackResult::Valid)
        }},
        OpType::StoreEntry(store_entry) => match store_entry {{
            OpEntry::CreateEntry {{
                entry_hash,
                entry_type,
            }} | OpEntry::UpdateEntry {{
                entry_hash,
                entry_type,
                ..
            }} => Ok(ValidateCallbackResult::Invalid(
                "There are no entry types in this integrity zome".to_string(),
            )),
            _ => Ok(ValidateCallbackResult::Valid)
        }},

        OpType::RegisterUpdate(update_entry) => match update_entry {{
            OpUpdate::Entry {{
                entry_hash,
                original_action_hash,
                original_entry_hash,
                original_entry_type,
                new_entry_type,
            }} => Ok(ValidateCallbackResult::Invalid(
                "There are no entry types in this integrity zome".to_string(),
            )),
            _ => Ok(ValidateCallbackResult::Valid)
        }},
        OpType::RegisterDelete(delete_entry) => match delete_entry {{
            OpDelete::Entry {{
                original_action_hash,
                original_entry_hash,
                original_entry_type,
            }} => Ok(ValidateCallbackResult::Invalid(
                "There are no entry types in this integrity zome".to_string(),
            )),
            _ => Ok(ValidateCallbackResult::Valid),
        }},
        OpType::RegisterCreateLink {{
            link_type,
            base_address,
            target_address,
            tag,
        }} => Ok(ValidateCallbackResult::Invalid(String::from(
            "There are no link types in this integrity zome",
        ))),

        OpType::RegisterDeleteLink {{
            link_type,
            original_link_hash,
            base_address,
            target_address,
            tag,
        }} => Ok(ValidateCallbackResult::Invalid(String::from(
            "There are no link types in this integrity zome",
        ))),
        OpType::RegisterAgentActivity(agent_activity) => match agent_activity {{
            OpActivity::CreateAgent {{
                agent,
                action
            }} => {{
                let previous_action = must_get_action(action.hashed.content.prev_action())?;
                match previous_action.action() {{
                    Action::AgentValidationPkg(AgentValidationPkg {{ membrane_proof, .. }}) => validate_agent_joining(agent, membrane_proof),
                    _ => Ok(ValidateCallbackResult::Invalid("The previous action for a `CreateAgent` action must be an `AgentValidationPkg`".to_string()))
                }}
            }},
            _ => Ok(ValidateCallbackResult::Valid)
        }},
    }}
}}
"#
    )
}
