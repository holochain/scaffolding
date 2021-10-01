import { camelize, getDnaBundlePath, mergeStrings } from "../utils";
import { HappDefinition } from "../../types";


export default (happ: HappDefinition) => 
`import { Config, InstallAgentsHapps } from '@holochain/tryorama';
import path from 'path'
import { fileURLToPath } from "url";
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

${mergeStrings(happ.dnas.map(dna => `export const ${camelize(dna.name)}Dna = path.join(__dirname, "../../${getDnaBundlePath(happ, dna.name)}");
`))}

export const config = Config.gen();

export const installation: InstallAgentsHapps = [
  // one agent
  [
    [${mergeStrings(happ.dnas.map(dna => `
      ${camelize(dna.name)}Dna, // contains this dna`))}
    ]
  ]
];

export const sleep = (ms: number) => new Promise(resolve => setTimeout(() => resolve(null), ms));
`