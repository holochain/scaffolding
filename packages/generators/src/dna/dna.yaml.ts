import { mergeStrings } from '../utils';
import { ZomeDefinition, HappDefinition } from '@holochain/rad-definitions';
import { getCrateName } from '../zome';

export default (happ: HappDefinition, dnaIndex: number, pathToBase: string) => {
  const dna = happ.dnas[dnaIndex];
  return `---
manifest_version: "1"
name: ${dna.name}
uuid: 00000000-0000-0000-0000-000000000000
properties: ~
zomes: 
${mergeStrings(
  dna.zomes.map(
    (zome: ZomeDefinition, zomeIndex: number) =>
      `  - name: ${zome.name}
    bundled: ${pathToBase}target/wasm32-unknown-unknown/release/${getCrateName(happ, dnaIndex, zomeIndex)}.wasm
`,
  ),
)}
`;
};
