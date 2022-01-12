import { ZomeDefinition, DnaDefinition, HappDefinition } from '@holochain/rad-definitions';
import { mergeStrings } from '../utils';
import { getCrateName } from '../zome';

export default (happ: HappDefinition) => `
import { Orchestrator } from "@holochain/tryorama";

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

`;
