import { IntegrityZomeDefinition, DnaDefinition, HappDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';
import { snakeCase } from 'lodash-es';
import { mergeStrings } from '../utils';
import { getCoordinatorCrateName } from '../zomes';

export const tryoramaIndexTs = (happ: HappDefinition): ScFile => ({
  type: ScNodeType.File,
  content: `
${mergeStrings(
  happ.dnas.map((dna: DnaDefinition, dnaIndex: number) =>
    dna.integrityZomes.map((zome: IntegrityZomeDefinition, zomeIndex: number) =>
      zome.entry_defs.map(
        entryDef =>
          `import ${getCoordinatorCrateName(happ, dnaIndex, zomeIndex)}_${snakeCase(entryDef.typeDefinition.name)} from './${
            dna.name
          }/${zome.name}/${entryDef.typeDefinition.name}';
${getCoordinatorCrateName(happ, dnaIndex, zomeIndex)}_${snakeCase(entryDef.typeDefinition.name)}();

`,
      ),
    ),
  ),
)}
`,
});
