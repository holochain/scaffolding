import { ZomeDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';
import { mergeStrings, titleCase } from '../utils';

export const libRs = (zomeDefinition: ZomeDefinition): ScFile => ({
  type: ScNodeType.File,
  content: `use hdk::prelude::*;
${mergeStrings(
  zomeDefinition.entry_defs.map(
    entry_def => `
mod ${entry_def.typeDefinition.name};`,
  ),
)}
${mergeStrings(
  zomeDefinition.entry_defs.map(
    entry_def => `
use ${entry_def.typeDefinition.name}::${titleCase(entry_def.typeDefinition.name)};`,
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

`,
});
