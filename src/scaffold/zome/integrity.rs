use proc_macro2::TokenStream;
use quote::quote;

pub fn initial_cargo_toml(zome_name: &str) -> String {
    format!(
        r#"[package]
name = "{zome_name}"
version = "0.0.1"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]
name = "{zome_name}"

[dependencies]
hdi = {{ workspace = true }}
serde = {{ workspace = true }}
holochain_serialized_bytes = {{ workspace = true }}
"#,
    )
}

pub fn initial_lib_rs() -> TokenStream {
    quote! {
        use hdi::prelude::*;

        /// Validation you perform during the genesis process. Nobody else on the network performs it, only you.
        /// There *is no* access to network calls in this callback
        #[hdk_extern]
        pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
            Ok(ValidateCallbackResult::Valid)
        }

        /// Validation the network performs when you try to join, you can't perform this validation yourself as you are not a member yet.
        /// There *is* access to network calls in this function
        pub fn validate_agent_joining(_agent_pub_key: AgentPubKey, _membrane_proof: &Option<MembraneProof>) -> ExternResult<ValidateCallbackResult> {
            Ok(ValidateCallbackResult::Valid)
        }

        /// This is the unified validation callback for all entries and link types in this integrity zome
        /// Below is a match template for all of the variants of `DHT Ops` and entry and link types
        ///
        /// Holochain has already performed the following validation for you:
        /// - The action signature matches on the hash of its content and is signed by its author
        /// - The previous action exists, has a lower timestamp than the new action, and incremented sequence number
        /// - The previous action author is the same as the new action author
        /// - The timestamp of each action is after the DNA's origin time
        /// - AgentActivity authorities check that the agent hasn't forked their chain
        /// - The entry hash in the action matches the entry content
        /// - The entry type in the action matches the entry content
        /// - The entry size doesn't exceed the maximum entry size (currently 4MB)
        /// - Private entry types are not included in the Op content, and public entry types are
        /// - If the `Op` is an update or a delete, the original action exists and is a `Create` or `Update` action
        /// - If the `Op` is an update, the original entry exists and is of the same type as the new one
        /// - If the `Op` is a delete link, the original action exists and is a `CreateLink` action
        /// - Link tags don't exceed the maximum tag size (currently 1KB)
        /// - Countersigned entries include an action from each required signer
        ///
        /// You can read more about validation here: https://docs.rs/hdi/latest/hdi/index.html#data-validation
        #[hdk_extern]
        pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
            match op.flattened::<(), ()>()? {
                FlatOp::CreateEntry(create_entry) => match create_entry {
                    OpEntry::CreateEntry {
                        app_entry,
                        action,
                    } => Ok(ValidateCallbackResult::Invalid(
                        "There are no entry types in this integrity zome".to_string(),
                    )),
                    OpEntry::UpdateEntry {
                        app_entry,
                        action,
                        ..
                    } => Ok(ValidateCallbackResult::Invalid(
                        "There are no entry types in this integrity zome".to_string(),
                    )),
                    _ => Ok(ValidateCallbackResult::Valid)
                },
                FlatOp::Update(update_entry) => match update_entry {
                    OpUpdate::Entry { app_entry, action } => {
                        Ok(ValidateCallbackResult::Invalid(
                            "There are no entry types in this integrity zome".to_string(),
                        ))
                    },
                    _ => Ok(ValidateCallbackResult::Valid)
                },
                FlatOp::Delete(OpDelete { action }) => Ok(ValidateCallbackResult::Invalid(
                    "There are no entry types in this integrity zome".to_string(),
                )),
                FlatOp::Link(OpLink::CreateLink {
                    link_type,
                    action
                }) => Ok(ValidateCallbackResult::Invalid(
                    "There are no link types in this integrity zome".to_string()
                )),
                FlatOp::Link(OpLink::DeleteLink {
                    link_type,
                    original_action,
                    action
                }) => Ok(ValidateCallbackResult::Invalid("There are no link types in this integrity zome".to_string())),
                FlatOp::CreateRecord(store_record) => match store_record {
                    /// Complementary validation to the `CreateEntry` Op, in which the record itself is validated
                    /// If you want to optimize performance, you can remove the validation for an entry type here and keep it in `CreateEntry`
                    /// Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `CreateEntry` validation failed
                    OpRecord::CreateEntry {
                        app_entry,
                        action
                    } => Ok(ValidateCallbackResult::Invalid("There are no entry types in this integrity zome".to_string())),
                    /// Complementary validation to the `Update` Op, in which the record itself is validated
                    /// If you want to optimize performance, you can remove the validation for an entry type here and keep it in `CreateEntry` and in `Update`
                    /// Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the other validations failed
                    OpRecord::UpdateEntry {
                        app_entry,
                        action,
                        ..
                    } => Ok(ValidateCallbackResult::Invalid("There are no entry types in this integrity zome".to_string())),
                    /// Complementary validation to the `Delete` Op, in which the record itself is validated
                    /// If you want to optimize performance, you can remove the validation for an entry type here and keep it in `Delete`
                    /// Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `Delete` validation failed
                    OpRecord::DeleteEntry {
                        action,
                        ..
                    } => Ok(ValidateCallbackResult::Invalid("There are no entry types in this integrity zome".to_string())),
                    /// Complementary validation to the `Link(OpLink::CreateLink { .. })` Op, in which the record itself is validated
                    /// If you want to optimize performance, you can remove the validation for a link type here and keep it in `Link`
                    /// Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `Link` validation failed
                    OpRecord::CreateLink {
                        link_type,
                        action
                    } => Ok(ValidateCallbackResult::Invalid("There are no link types in this integrity zome".to_string())),
                    /// Complementary validation to the `Link(OpLink::DeleteLink { .. })` Op, in which the record itself is validated
                    /// If you want to optimize performance, you can remove the validation for a link type here and keep it in `Link`
                    /// Notice that doing so will cause `must_get_valid_record` for this record to return a valid record even if the `Link` validation failed
                    OpRecord::DeleteLink {
                        action
                    } => Ok(ValidateCallbackResult::Invalid("There are no link types in this integrity zome".to_string())),
                    OpRecord::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                    OpRecord::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                    OpRecord::CreateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                    OpRecord::CreateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                    OpRecord::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                    OpRecord::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                    OpRecord::Dna { .. } => Ok(ValidateCallbackResult::Valid),
                    OpRecord::OpenChain { .. } => Ok(ValidateCallbackResult::Valid),
                    OpRecord::CloseChain { .. } => Ok(ValidateCallbackResult::Valid),
                    OpRecord::InitZomesComplete { .. } => Ok(ValidateCallbackResult::Valid),
                    _ => Ok(ValidateCallbackResult::Valid)
                },
                FlatOp::AgentActivity(agent_activity) => match agent_activity {
                    OpActivity::CreateAgent { agent, action } => {
                        let prev = action
                            .prev_action()
                            .ok_or_else(|| {
                                wasm_error!(WasmErrorInner::Guest("expected a prior action".into()))
                            })?
                            .clone();

                        let previous_action = must_get_action(prev)?;
                        match &previous_action.action().data {
                            ActionData::AgentValidationPkg(AgentValidationPkgData { membrane_proof, .. }) => validate_agent_joining(agent, membrane_proof),
                            _ => Ok(ValidateCallbackResult::Invalid("The previous action for a `CreateAgent` action must be an `AgentValidationPkg`".to_string()))
                        }
                    },
                    _ => Ok(ValidateCallbackResult::Valid)
                },
            }
        }
    }
}
