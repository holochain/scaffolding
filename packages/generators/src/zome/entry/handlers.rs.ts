import { EntryDefinition } from '@holochain/rad-definitions';
import { toTitleCase } from '../../utils';

export default (entryDef: EntryDefinition) =>
  `use hdk::prelude::*;
use super::${toTitleCase(entryDef.name)};

${entryDef.read ? readHandler(entryDef.name) : ''}
${entryDef.create || entryDef.update ? `#[derive(Serialize, Deserialize, Debug)]
pub struct ${newEntryOutput(entryDef.name)} {
  header_hash: HeaderHash,
  entry_hash: EntryHash,
}
` : ''}
${entryDef.create ? createHandler(entryDef.name) : ''}
${entryDef.update ? updateHandler(entryDef.name) : ''}
${entryDef.delete ? deleteHandler(entryDef.name) : ''}`;

export const readHandler = (entryDefId: string) => `#[hdk_extern]
pub fn get_${entryDefId}(entry_hash: EntryHash) -> ExternResult<Option<${toTitleCase(entryDefId)}>> {
  let maybe_element = get(entry_hash, GetOptions::default())?;

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


export const createHandler = (entryDefId: string) => `#[hdk_extern]
pub fn create_${entryDefId}(${entryDefId}: ${toTitleCase(entryDefId)}) -> ExternResult<${newEntryOutput(entryDefId)}> {
  let header_hash = create_entry(&${entryDefId})?;

  let entry_hash = hash_entry(&${entryDefId})?;

  let output = ${newEntryOutput(entryDefId)} {
    header_hash,
    entry_hash
  };

  Ok(output)
}

`;

export const updateHandler = (entryDefId: string) => `#[derive(Serialize, Deserialize, Debug)]
pub struct Update${toTitleCase(entryDefId)}Input {
  original_header_hash: HeaderHash,
  updated_${entryDefId}: ${toTitleCase(entryDefId)}
}

#[hdk_extern]
pub fn update_${entryDefId}(input: Update${toTitleCase(entryDefId)}Input) -> ExternResult<${newEntryOutput(entryDefId)}> {
  let header_hash = update_entry(input.original_header_hash, &input.updated_${entryDefId})?;

  let entry_hash = hash_entry(&input.updated_${entryDefId})?;

  let output = ${newEntryOutput(entryDefId)} {
    header_hash,
    entry_hash
  };

  Ok(output)
}

`;

export const deleteHandler = (entryDefId: string) => `#[hdk_extern]
pub fn delete_${entryDefId}(header_hash: HeaderHash) -> ExternResult<HeaderHash> {
  delete_entry(header_hash)
}

`;

const newEntryOutput = (entryDefId: string) => `New${toTitleCase(entryDefId)}Output`