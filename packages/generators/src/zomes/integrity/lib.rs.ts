import { IntegrityZomeDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';
import { mergeStrings, titleCase } from '../../utils';
import { snakeCase } from 'lodash-es';

export const libRs = (integrityZomeDefinition: IntegrityZomeDefinition): ScFile => ({
  type: ScNodeType.File,
  content: `use hdi::prelude::*;
${mergeStrings(
  integrityZomeDefinition.entry_defs.map(
    entry_def => `
mod ${snakeCase(entry_def.typeDefinition.name)};
pub use ${snakeCase(entry_def.typeDefinition.name)}::${titleCase(entry_def.typeDefinition.name)};
`,
  ),
)}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
${mergeStrings(
  integrityZomeDefinition.entry_defs.map(
    entry_def => `#[entry_def()]
${titleCase(entry_def.typeDefinition.name)}(${titleCase(entry_def.typeDefinition.name)}),
`,
  )
)}
}

#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
  Ok(ValidateCallbackResult::Valid)
}
`,
});
