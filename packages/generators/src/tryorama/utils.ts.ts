import { HappDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';

import { getDnaBundlePath, mergeStrings } from '../utils';

export const tryoramaUtilsTs = (happ: HappDefinition): ScFile => ({
  type: ScNodeType.File,
  content: `import { Config, InstallAgentsHapps } from '@holochain/tryorama';
import path from 'path'
import { fileURLToPath } from "url";
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

${mergeStrings(
  happ.dnas.map(
    dna => `export const ${camelCase(dna.name)}Dna = path.join(__dirname, "../../${getDnaBundlePath(happ, dna.name)}");
`,
  ),
)}

export const config = Config.gen();

`,
});
