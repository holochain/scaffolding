import { DnaDefinition } from '../../types/dna';
import { mergeStrings } from '../utils';

export default (dnas: DnaDefinition[]) => `
import { Orchestrator } from "@holochain/tryorama";

${mergeStrings(
  dnas.map(dna =>
    dna.zomes.map(
      zome => `import ${zome.name} from './${dna.name}/${zome.name}';
`,
    ),
  ),
)}
let orchestrator: Orchestrator<any>;

${mergeStrings(
  dnas.map(dna =>
    dna.zomes.map(
      zome => `orchestrator = new Orchestrator();
${zome.name}(orchestrator);
orchestrator.run();

`,
    ),
  ),
)}

`;
