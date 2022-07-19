import { EntryDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';
import { snakeCase } from 'lodash-es';
import { titleCase } from '../../utils';

export const entryHandlers = (entryDef: EntryDefinition, integrityCrateName: string): ScFile => ({
  type: ScNodeType.File,
  content: `use hdk::prelude::*;
use ${integrityCrateName}::${titleCase(entryDef.typeDefinition.name)};
use ${integrityCrateName}::EntryTypes;

${entryDef.read ? readHandler(entryDef.typeDefinition.name) : ''}
${entryDef.create ? createHandler(entryDef.typeDefinition.name) : ''}
${entryDef.update ? updateHandler(entryDef.typeDefinition.name) : ''}
${entryDef.delete ? deleteHandler(entryDef.typeDefinition.name) : ''}`,
});

export const readHandlerFnName = (entryDefId: string) => `get_${snakeCase(entryDefId)}`;

export const readHandler = (entryDefId: string) => `#[hdk_extern]
pub fn ${readHandlerFnName(entryDefId)}(action_hash: ActionHash) -> ExternResult<Option<Record>> {
  get(action_hash, GetOptions::default())
}

`;

export const createHandlerFnName = (entryDefId: string) => `create_${snakeCase(entryDefId)}`;

export const createHandler = (entryDefId: string) => `#[hdk_extern]
pub fn ${createHandlerFnName(entryDefId)}(${snakeCase(entryDefId)}: ${titleCase(
  entryDefId,
)}) -> ExternResult<ActionHash> {
  create_entry(&EntryTypes::${titleCase(entryDefId)}(${snakeCase(entryDefId)}.clone()))
}

`;

export const updateHandlerFnName = (entryDefId: string) => `update_${snakeCase(entryDefId)}`;

export const updateHandler = (entryDefId: string) => `#[derive(Serialize, Deserialize, Debug)]
pub struct Update${titleCase(entryDefId)}Input {
  original_action_hash: ActionHash,
  updated_${snakeCase(entryDefId)}: ${titleCase(entryDefId)}
}

#[hdk_extern]
pub fn ${updateHandlerFnName(entryDefId)}(input: Update${titleCase(entryDefId)}Input) -> ExternResult<ActionHash> {
  update_entry(input.original_action_hash, &input.updated_${snakeCase(entryDefId)})
}

`;

export const deleteHandlerFnName = (entryDefId: string) => `delete_${snakeCase(entryDefId)}`;

export const deleteHandler = (entryDefId: string) => `#[hdk_extern]
pub fn ${deleteHandlerFnName(entryDefId)}(action_hash: ActionHash) -> ExternResult<ActionHash> {
  delete_entry(action_hash)
}

`;
