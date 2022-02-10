import { EntryDefinition } from '@holochain/rad-definitions';
import { PatcherFile, PatcherNodeType } from '@patcher/types';
import { toTitleCase } from '../../utils';

export const entryHandlers = (entryDef: EntryDefinition): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `use hdk::prelude::*;
use hdk::prelude::holo_hash::*;
use super::${toTitleCase(entryDef.name)};

${entryDef.read ? readHandler(entryDef.name) : ''}
${
  entryDef.create || entryDef.update
    ? `#[derive(Serialize, Deserialize, Debug)]
pub struct ${newEntryOutput(entryDef.name)} {
  header_hash: HeaderHashB64,
  entry_hash: EntryHashB64,
}
`
    : ''
}
${entryDef.create ? createHandler(entryDef.name) : ''}
${entryDef.update ? updateHandler(entryDef.name) : ''}
${entryDef.delete ? deleteHandler(entryDef.name) : ''}`,
});

export const readHandlerFnName = (entryDefId: string) => `get_${entryDefId}`;

export const readHandler = (entryDefId: string) => `#[hdk_extern]
pub fn ${readHandlerFnName(entryDefId)}(entry_hash: EntryHashB64) -> ExternResult<Option<${toTitleCase(entryDefId)}>> {
  let maybe_element = get(EntryHash::from(entry_hash), GetOptions::default())?;

  match maybe_element {
    None => Ok(None),
    Some(element) => {
      let ${entryDefId}: ${toTitleCase(entryDefId)} = element.entry()
        .to_app_option()?
        .ok_or(WasmError::Guest("Could not deserialize element to ${toTitleCase(entryDefId)}.".into()))?;
    
      Ok(Some(${entryDefId}))
    }
  }
}

`;

export const createHandlerFnName = (entryDefId: string) => `create_${entryDefId}`;

export const createHandler = (entryDefId: string) => `#[hdk_extern]
pub fn ${createHandlerFnName(entryDefId)}(${entryDefId}: ${toTitleCase(entryDefId)}) -> ExternResult<${newEntryOutput(
  entryDefId,
)}> {
  let header_hash = create_entry(&${entryDefId})?;

  let entry_hash = hash_entry(&${entryDefId})?;

  let output = ${newEntryOutput(entryDefId)} {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}

`;

export const updateHandlerFnName = (entryDefId: string) => `update_${entryDefId}`;

export const updateHandler = (entryDefId: string) => `#[derive(Serialize, Deserialize, Debug)]
pub struct Update${toTitleCase(entryDefId)}Input {
  original_header_hash: HeaderHashB64,
  updated_${entryDefId}: ${toTitleCase(entryDefId)}
}

#[hdk_extern]
pub fn ${updateHandlerFnName(entryDefId)}(input: Update${toTitleCase(
  entryDefId,
)}Input) -> ExternResult<${newEntryOutput(entryDefId)}> {
  let header_hash = update_entry(HeaderHash::from(input.original_header_hash), &input.updated_${entryDefId})?;

  let entry_hash = hash_entry(&input.updated_${entryDefId})?;

  let output = ${newEntryOutput(entryDefId)} {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}

`;

export const deleteHandlerFnName = (entryDefId: string) => `delete_${entryDefId}`;

export const deleteHandler = (entryDefId: string) => `#[hdk_extern]
pub fn ${deleteHandlerFnName(entryDefId)}(header_hash: HeaderHashB64) -> ExternResult<HeaderHash> {
  delete_entry(HeaderHash::from(header_hash))
}

`;

const newEntryOutput = (entryDefId: string) => `New${toTitleCase(entryDefId)}Output`;
