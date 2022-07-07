import { IntegrityZomeDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';
import { mergeStrings } from '../../utils';
import { snakeCase } from 'lodash-es';

export const libRs = (integrityZomeDefinition: IntegrityZomeDefinition): ScFile => ({
  type: ScNodeType.File,
  content: `${mergeStrings(
    integrityZomeDefinition.entry_defs.map(
      entry_def => `
mod ${snakeCase(entry_def.typeDefinition.name)};
pub use ${snakeCase(entry_def.typeDefinition.name)}::*;
`,
    ),
  )}`,
});
