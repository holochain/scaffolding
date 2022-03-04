import { ZomeDefinition, DnaDefinition, HappDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';
import { snakeCase } from 'lodash-es';
import { mergeStrings } from '../utils';
import { getCrateName } from '../zome';

export const tryoramaIndexTs = (happ: HappDefinition): ScFile => ({
  type: ScNodeType.File,
  content: `import { Orchestrator } from "@holochain/tryorama";

${mergeStrings(
  happ.dnas.map((dna: DnaDefinition, dnaIndex: number) =>
    dna.zomes.map((zome: ZomeDefinition, zomeIndex: number) =>
      zome.entry_defs.map(
        entryDef =>
          `import ${getCrateName(happ, dnaIndex, zomeIndex)}_${snakeCase(entryDef.typeDefinition.name)} from './${
            dna.name
          }/${zome.name}/${entryDef.typeDefinition.name}';
`,
      ),
    ),
  ),
)}
let orchestrator: Orchestrator<any>;

${mergeStrings(
  happ.dnas.map((dna: DnaDefinition, dnaIndex: number) =>
    dna.zomes.map((zome: ZomeDefinition, zomeIndex: number) =>
      zome.entry_defs.map(
        entryDef =>
          `orchestrator = new Orchestrator();
${getCrateName(happ, dnaIndex, zomeIndex)}_${snakeCase(entryDef.typeDefinition.name)}(orchestrator);
orchestrator.run();

`,
      ),
    ),
  ),
)}

`,
});
