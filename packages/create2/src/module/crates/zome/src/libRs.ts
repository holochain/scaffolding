import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const libRs = ({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNameTitleCase, moduleNamePlural, moduleName}: {moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; moduleNameTitleCase: string; moduleNamePlural: string; moduleName: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `//! ## hc_zome${moduleNameSnakeCase}s
//! 
//! ${moduleNamePluralTitleCase} zome for any Holochain app.
//! 
//! If you need to manage ${snakeCase(moduleNamePlural)} (nickname, name, avatar, age and other useful personal information)
//! you can directly include this zome in your DNA.
//! 
//! Read about how to include both this zome and its frontend module in your application [here](https://holochain-open-dev.github.io/${snakeCase(moduleNamePlural)}).

use hdk::prelude::holo_hash::AgentPubKeyB64;
use hdk::prelude::*;

mod handlers;
mod utils;

use hc_zome${moduleNameSnakeCase}s_types::*;

entry_defs![PathEntry::entry_def(), ${moduleNameTitleCase}::entry_def()];

/// Creates the ${snakeCase(moduleName)} for the agent executing this call.
#[hdk_extern]
pub fn create${moduleNameSnakeCase}(${snakeCase(moduleName)}: ${moduleNameTitleCase}) -> ExternResult<Agent${moduleNameTitleCase}> {
    handlers::create${moduleNameSnakeCase}(${snakeCase(moduleName)})
}

/// Updates the ${snakeCase(moduleName)} for the agent executing this call.
#[hdk_extern]
pub fn update${moduleNameSnakeCase}(${snakeCase(moduleName)}: ${moduleNameTitleCase}) -> ExternResult<Agent${moduleNameTitleCase}> {
    handlers::update${moduleNameSnakeCase}(${snakeCase(moduleName)})
}

/// From a search input of at least 3 characters, returns all the agents whose nickname starts with that prefix.
#[hdk_extern]
pub fn search${moduleNameSnakeCase}s(
    search${moduleNameSnakeCase}s_input: Search${moduleNamePluralTitleCase}Input,
) -> ExternResult<Vec<Agent${moduleNameTitleCase}>> {
    let agent${moduleNameSnakeCase}s = handlers::search${moduleNameSnakeCase}s(search${moduleNameSnakeCase}s_input.nickname_prefix)?;

    Ok(agent${moduleNameSnakeCase}s)
}

/// Returns the ${snakeCase(moduleName)} for the given agent, if they have created it.
#[hdk_extern]
pub fn get_agent${moduleNameSnakeCase}(agent_pub_key: AgentPubKeyB64) -> ExternResult<Option<Agent${moduleNameTitleCase}>> {
    let agent${moduleNameSnakeCase} = handlers::get_agent${moduleNameSnakeCase}(agent_pub_key)?;

    Ok(agent${moduleNameSnakeCase})
}

/// Returns the ${snakeCase(moduleNamePlural)} for the given agents if they have created them.
///
/// Use this function if you need to get the ${snakeCase(moduleName)} for multiple agents at the same time,
/// as it will be more performant than doing multiple \`get_agent${moduleNameSnakeCase}\`.
#[hdk_extern]
pub fn get_agents${moduleNameSnakeCase}(
    agent_pub_keys_b64: Vec<AgentPubKeyB64>,
) -> ExternResult<Vec<Agent${moduleNameTitleCase}>> {
    let agent${moduleNameSnakeCase}s = handlers::get_agents${moduleNameSnakeCase}(agent_pub_keys_b64)?;

    Ok(agent${moduleNameSnakeCase}s)
}

/// Gets the ${snakeCase(moduleName)} for the agent calling this function, if they have created it.
#[hdk_extern]
pub fn get_my${moduleNameSnakeCase}(_: ()) -> ExternResult<Option<Agent${moduleNameTitleCase}>> {
    let agent_info = agent_info()?;

    let agent${moduleNameSnakeCase} =
        handlers::get_agent${moduleNameSnakeCase}(AgentPubKeyB64::from(agent_info.agent_initial_pubkey))?;

    Ok(agent${moduleNameSnakeCase})
}

/// Gets all the ${snakeCase(moduleNamePlural)} that have been created in the network.
///
/// Careful! This will not be very performant in large networks.
/// In the future a cursor type functionality will be added to make this function performant.
#[hdk_extern]
pub fn get_all${moduleNameSnakeCase}s(_: ()) -> ExternResult<Vec<Agent${moduleNameTitleCase}>> {
    let agent${moduleNameSnakeCase}s = handlers::get_all${moduleNameSnakeCase}s()?;

    Ok(agent${moduleNameSnakeCase}s)
}
`
});
    