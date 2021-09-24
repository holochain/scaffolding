import { Config, InstallAgentsHapps } from '@holochain/tryorama';
import path from 'path'
import { fileURLToPath } from "url";
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const my-dnaDna = path.join(__dirname, "../../dna/workdir/my-dna.dna");


export const config = Config.gen();

export const installation: InstallAgentsHapps = [
  // one agent
  [
    [
      my-dnaDna, // contains this dna
    ]
  ]
];

export const sleep = (ms: number) => new Promise(resolve => setTimeout(() => resolve(null), ms));
