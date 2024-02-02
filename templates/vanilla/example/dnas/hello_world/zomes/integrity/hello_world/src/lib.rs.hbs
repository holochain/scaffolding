use hdi::prelude::*;

/// this struct defines the content of the Hello entry
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Hello {
    pub message: String,
}

/// Definition of the Hello entry type itself using the entry-helper struct as its content
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Hello(Hello), 
}

/// Definition of a link type to be used for linking from an anchor to all created entries
#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    AllHellos,
}

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
/// In this example app we leave validation aside, please look at the Forum example for validation samples
///
/// You can read more about validation here: https://docs.rs/hdi/latest/hdi/index.html#data-validation
/// 
/// 
#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

