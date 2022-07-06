import { IntegrityZomeDefinition, DnaDefinition, HappDefinition, ZomeBundleDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';
import { snakeCase } from 'lodash-es';
import { mergeStrings } from '../utils';
import { getCoordinatorCrateName } from '../zomes';

export const tryoramaIndexTs = (happ: HappDefinition): ScFile => ({
  type: ScNodeType.File,
  content: `
${mergeStrings(
  happ.dnas.map((dna: DnaDefinition, dnaIndex: number) =>
    dna.zomeBundles.map((zomeBundle: ZomeBundleDefinition, zomeBundleIndex: number) =>
      zomeBundle.integrityZome.entry_defs.map(
        entryDef =>
          `import ${getCoordinatorCrateName(happ, dnaIndex, zomeBundleIndex)}_${snakeCase(entryDef.typeDefinition.name)} from './${
            dna.name
          }/${zomeBundle.name}/${entryDef.typeDefinition.name}';
${getCoordinatorCrateName(happ, dnaIndex, zomeBundleIndex)}_${snakeCase(entryDef.typeDefinition.name)}();

`,
      ),
    ),
  ),
)}
`,
});
