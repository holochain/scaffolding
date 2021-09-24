import { mergeStrings } from '../utils';
import { DnaDefinition } from '../../types';

export default (dna: DnaDefinition, pathToBase: string) =>
  `---
manifest_version: "1"
name: ${dna.name}
uuid: 00000000-0000-0000-0000-000000000000
properties: ~
zomes: 
${mergeStrings(
  dna.zomes.map(
    zome =>
      `  - name: ${zome.name}
    bundled: ${pathToBase}target/wasm32-unknown-unknown/release/${zome.name}.wasm
`,
  ),
)}
`;
