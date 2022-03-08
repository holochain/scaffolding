import { ZomeDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';
import { mergeStrings, titleCase } from '../utils';
import { snakeCase } from 'lodash-es';

export const libRs = (zomeDefinition: ZomeDefinition): ScFile => ({
  type: ScNodeType.File,
  content: `use hdk::prelude::*;
${mergeStrings(
  zomeDefinition.entry_defs.map(
    entry_def => `
mod ${snakeCase(entry_def.typeDefinition.name)};`,
  ),
)}
${mergeStrings(
  zomeDefinition.entry_defs.map(
    entry_def => `
use ${snakeCase(entry_def.typeDefinition.name)}::${titleCase(entry_def.typeDefinition.name)};`,
  ),
)}

entry_defs![${mergeStrings(
    zomeDefinition.entry_defs.map(
      entry_def => `
  ${titleCase(entry_def.typeDefinition.name)}::entry_def()`,
    ),
    ',',
  )}
];

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
  Ok(ValidateCallbackResult::Valid)
}
`,
});
