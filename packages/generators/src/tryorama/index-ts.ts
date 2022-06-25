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
          `import './${
            dna.name
          }/${zome.name}/${entryDef.typeDefinition.name}';
`,
      ),
    ),
  ),
)}
`,
});
