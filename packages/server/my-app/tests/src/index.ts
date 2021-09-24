
import { Orchestrator } from "@holochain/tryorama";

import my-dna_my-zome from './my-dna/my-zome';

let orchestrator: Orchestrator<any>;

orchestrator = new Orchestrator();
my-dna_my-zome(orchestrator);
orchestrator.run();



