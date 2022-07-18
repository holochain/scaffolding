import { CoordinatorZomeDefinition, DnaDefinition, HappDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';
import { snakeCase } from 'lodash-es';
import { mergeStrings } from '../utils';
import { getCoordinatorCrateName } from '../zomes/utils';

export const tryoramaIndexTs = (happ: HappDefinition): ScFile => ({
  type: ScNodeType.File,
  content: `
${mergeStrings(
  happ.dnas.map((dna: DnaDefinition, dnaIndex: number) =>
    dna.coordinator_zomes.map((coordinatorZome: CoordinatorZomeDefinition, coordinatorZomeIndex: number) => {
      const integrityZome = dna.integrity_zomes.find(iz => coordinatorZome.dependencies.includes(iz.name));
      return integrityZome.entry_defs.map(
        entryDef =>
          `import ${getCoordinatorCrateName(happ, dnaIndex, coordinatorZomeIndex)}_${snakeCase(
            entryDef.typeDefinition.name,
          )} from './${dna.name}/${coordinatorZome.name}/${entryDef.typeDefinition.name}';
${getCoordinatorCrateName(happ, dnaIndex, coordinatorZomeIndex)}_${snakeCase(entryDef.typeDefinition.name)}();

`,
      );
    }),
  ),
)}
`,
});
