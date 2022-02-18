import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const libRs = ({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNameTitleCase, moduleName}: {moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; moduleNameTitleCase: string; moduleName: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `use std::collections::BTreeMap;

use hdk::prelude::holo_hash::AgentPubKeyB64;
use hdk::prelude::*;

/// ${moduleNameTitleCase} entry definition.
///
/// The ${snakeCase(moduleName)} must include at a minimum the nickname of the agent
/// in order to be able to search for agents by nickname.
#[hdk_entry(id = "${snakeCase(moduleName)}", visibility = "public")]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct ${moduleNameTitleCase} {
    pub nickname: String,
    pub fields: BTreeMap<String, String>,
}

/// Used as a return type of all functions.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Agent${moduleNameTitleCase} {
    pub agent_pub_key: AgentPubKeyB64,
    pub ${snakeCase(moduleName)}: ${moduleNameTitleCase},
}

/// Input for the \`search${moduleNameSnakeCase}s\` zome function.
/// 
/// The nickname prefix must be of at least 3 characters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Search${moduleNamePluralTitleCase}Input {
    pub nickname_prefix: String,
}
`
});
    