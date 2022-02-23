import { ZomeDefinition, DnaDefinition, HappDefinition } from '@holochain-scaffolding/definitions';
import { PatcherFile, PatcherNodeType } from '@source-craft/types';
import { mergeStrings } from '../utils';
import { getCrateName } from '../zome';

export const tryoramaIndexTs = (happ: HappDefinition): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `import { Orchestrator } from "@holochain/tryorama";

${mergeStrings(
  happ.dnas.map((dna: DnaDefinition, dnaIndex: number) =>
    dna.zomes.map((zome: ZomeDefinition, zomeIndex: number) =>
      zome.entry_defs.map(
        entryDef =>
          `import ${getCrateName(happ, dnaIndex, zomeIndex)}_${entryDef.typeDefinition.name} from './${dna.name}/${zome.name}/${
            entryDef.typeDefinition.name
          }';
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
${getCrateName(happ, dnaIndex, zomeIndex)}_${entryDef.typeDefinition.name}(orchestrator);
orchestrator.run();

`,
      ),
    ),
  ),
)}

`,
});
