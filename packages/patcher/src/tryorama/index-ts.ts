import { ZomeDefinition, DnaDefinition, HappDefinition } from '@holochain-scaffolding/definitions';
import { PatcherFile, PatcherNodeType } from '@patcher/types';
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
          `import ${getCrateName(happ, dnaIndex, zomeIndex)}_${entryDef.name} from './${dna.name}/${zome.name}/${
            entryDef.name
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
${getCrateName(happ, dnaIndex, zomeIndex)}_${entryDef.name}(orchestrator);
orchestrator.run();

`,
      ),
    ),
  ),
)}

`,
});
