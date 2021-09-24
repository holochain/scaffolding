import { DnaDefinition } from '../../types';
import { mergeStrings } from '../utils';

export default (dnas: DnaDefinition[]) => `
import { Orchestrator } from "@holochain/tryorama";

${mergeStrings(
  dnas.map(dna =>
    dna.zomes.map(
      zome => `import ${dna.name}_${zome.name} from './${dna.name}/${zome.name}';
`,
    ),
  ),
)}
let orchestrator: Orchestrator<any>;

${mergeStrings(
  dnas.map(dna =>
    dna.zomes.map(
      zome => `orchestrator = new Orchestrator();
${dna.name}_${zome.name}(orchestrator);
orchestrator.run();

`,
    ),
  ),
)}

`;
