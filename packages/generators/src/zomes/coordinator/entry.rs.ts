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
${
  entryDef.create || entryDef.update
    ? `#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ${newEntryOutput(entryDef.typeDefinition.name)} {
  action_hash: ActionHash,
  entry_hash: EntryHash,
}
`
    : ''
}
${entryDef.create ? createHandler(entryDef.typeDefinition.name) : ''}
${entryDef.update ? updateHandler(entryDef.typeDefinition.name) : ''}
${entryDef.delete ? deleteHandler(entryDef.typeDefinition.name) : ''}`,
});

export const readHandlerFnName = (entryDefId: string) => `get_${snakeCase(entryDefId)}`;

export const readHandler = (entryDefId: string) => `#[hdk_extern]
pub fn ${readHandlerFnName(entryDefId)}(entry_hash: EntryHash) -> ExternResult<Option<${titleCase(entryDefId)}>> {
  let maybe_element = get(entry_hash, GetOptions::default())?;

  match maybe_element {
    None => Ok(None),
    Some(record) => {
      let ${snakeCase(entryDefId)}: ${titleCase(entryDefId)} = record.entry()
        .to_app_option()
        .map_err(|error| wasm_error!(WasmErrorInner::Guest(format!("Could not deserialize Record to ${titleCase(
          entryDefId,
        )}: {}", error))))?
        .ok_or(wasm_error!(WasmErrorInner::Guest("No ${titleCase(entryDefId)} found for the given hash.".into())))?;

      Ok(Some(${snakeCase(entryDefId)}))
    }
  }
}

`;

export const createHandlerFnName = (entryDefId: string) => `create_${snakeCase(entryDefId)}`;

export const createHandler = (entryDefId: string) => `#[hdk_extern]
pub fn ${createHandlerFnName(entryDefId)}(${snakeCase(entryDefId)}: ${titleCase(
  entryDefId,
)}) -> ExternResult<${newEntryOutput(entryDefId)}> {
  let action_hash = create_entry(&EntryTypes::${titleCase(entryDefId)}(${snakeCase(entryDefId)}.clone()))?;

  let entry_hash = hash_entry(&EntryTypes::${titleCase(entryDefId)}(${snakeCase(entryDefId)}))?;

  let output = ${newEntryOutput(entryDefId)} {
    action_hash,
    entry_hash
  };

  Ok(output)
}

`;

export const updateHandlerFnName = (entryDefId: string) => `update_${snakeCase(entryDefId)}`;

export const updateHandler = (entryDefId: string) => `#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Update${titleCase(entryDefId)}Input {
  original_action_hash: ActionHash,
  updated_${snakeCase(entryDefId)}: ${titleCase(entryDefId)}
}

#[hdk_extern]
pub fn ${updateHandlerFnName(entryDefId)}(input: Update${titleCase(entryDefId)}Input) -> ExternResult<${newEntryOutput(
  entryDefId,
)}> {
  let action_hash = update_entry(input.original_action_hash, &input.updated_${snakeCase(entryDefId)})?;

  let entry_hash = hash_entry(&input.updated_${snakeCase(entryDefId)})?;

  let output = ${newEntryOutput(entryDefId)} {
    action_hash,
    entry_hash
  };

  Ok(output)
}

`;

export const deleteHandlerFnName = (entryDefId: string) => `delete_${snakeCase(entryDefId)}`;

export const deleteHandler = (entryDefId: string) => `#[hdk_extern]
pub fn ${deleteHandlerFnName(entryDefId)}(action_hash: ActionHash) -> ExternResult<ActionHash> {
  delete_entry(action_hash)
}

`;

const newEntryOutput = (entryDefId: string) => `New${titleCase(entryDefId)}Output`;
