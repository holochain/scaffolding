import { ZomeDefinition, HappDefinition } from '@holochain-scaffolding/definitions';
import { PatcherFile, PatcherNodeType } from '@patcher/types';
import { mergeStrings } from '../utils';
import { getCrateName } from '../zome';

export const dnaYaml = (happ: HappDefinition, dnaIndex: number, pathToBase: string): PatcherFile => {
  const dna = happ.dnas[dnaIndex];
  return {
    type: PatcherNodeType.File,
    content: `---
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
`,
  };
};
